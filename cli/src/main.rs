#![forbid(unsafe_code)]

use std::path::PathBuf;

use anyhow::{anyhow, Result};
use clap::{Parser, ValueEnum};
use tokio::{fs, io::AsyncWriteExt};

use api::rename::Destination;
use mw_tools::{api, Client};

#[derive(Parser, Debug, PartialEq)]
enum Subcommand {
    Delete {
        /// uses newline separation
        input: PathBuf,
    },
    List {
        #[arg(value_enum)]
        list_type: ListType,

        parameter: Option<String>,

        #[arg(short, long)]
        output: Option<PathBuf>,
    },
    Move {
        /// uses newline separation
        input: PathBuf,
        #[arg(long)]
        append: Option<String>,
        #[arg(long)]
        prepend: Option<String>,
        #[arg(long)]
        replace: Option<Vec<String>>,
    },
    Nulledit {
        /// uses newline separation
        input: PathBuf,
    },
    Purge {
        #[arg(short, long)]
        recursive: bool,

        input: PathBuf,
    },
    Upload {
        input: PathBuf,

        #[arg(short, long)]
        text: Option<String>,
    },
}

#[derive(ValueEnum, Clone, Debug, PartialEq)]
enum ListType {
    Allimages,
    Allpages,
    Alllinks,
    Allcategories,
    Backlinks,
    Categorymembers,
    Embeddedin,
    Imageusage,
    Iwbacklinks,
    Langbacklinks,
    Search,
    Exturlusage,
    Protectedtitles,
    Querypage,
    Wkpoppages,
    Allinfoboxes,
}

#[derive(Parser, Debug, PartialEq)]
struct Cli {
    #[command(subcommand)]
    command: Subcommand,

    #[arg(short, long, env = "FANDOM_BOT_NAME", hide_env_values = true)]
    name: String,
    #[arg(short, long, env = "FANDOM_BOT_PASSWORD", hide_env_values = true)]
    password: String,
    #[arg(
        short,
        long,
        default_value = "https://leagueoflegends.fandom.com/de/api.php"
    )]
    url: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    pretty_env_logger::init();

    let cli = Cli::parse();
    let mut client = Client::new(&cli.url)?;
    client.login(&cli.name, &cli.password).await?;
    let client = client;

    match cli.command {
        Subcommand::Delete { input } => {
            let contents = fs::read_to_string(input).await?;
            let titles: Vec<&str> = contents.lines().collect();
            api::delete::delete(&client, &titles, None).await?;
        }
        Subcommand::List {
            list_type,
            parameter,
            output,
        } => {
            if list_type == ListType::Exturlusage {
                let res = serde_json::to_vec_pretty(&api::list::exturlusage(&client).await?)?;
                match output {
                    Some(o) => {
                        let mut file = fs::File::create(o).await?;
                        file.write_all(&res).await?;
                    }
                    None => println!("{}", String::from_utf8_lossy(&res)),
                }
            } else {
                let res = match list_type {
                    ListType::Allimages => api::list::allimages(&client).await?,
                    ListType::Allpages => api::list::allpages(&client, parameter.as_deref()).await?,
                    ListType::Alllinks => api::list::alllinks(&client).await?,
                    ListType::Allcategories => api::list::allcategories(&client).await?,
                    ListType::Backlinks => api::list::backlinks(&client, &parameter.ok_or_else(|| anyhow!("parameter 'bltitle' required. Visit https://www.mediawiki.org/wiki/Special:MyLanguage/API:Backlinks for help."))?).await?,
                    ListType::Categorymembers => api::list::categorymembers(&client, &parameter.ok_or_else(|| anyhow!("parameter 'cmtitle' required. Visit https://www.mediawiki.org/wiki/Special:MyLanguage/API:Categorymembers for help."))?).await?,
                    ListType::Embeddedin => api::list::embeddedin(&client, &parameter.ok_or_else(|| anyhow!("parameter 'eititle' required. Visit https://www.mediawiki.org/wiki/Special:MyLanguage/API:Embeddedin for help."))?).await?,
                    ListType::Imageusage => api::list::imageusage(&client, &parameter.ok_or_else(|| anyhow!("parameter 'iutitle' required. Visit https://www.mediawiki.org/wiki/Special:MyLanguage/API:Imageusage for help."))?).await?,
                    ListType::Search => api::list::search(&client, &parameter.ok_or_else(|| anyhow!("parameter 'srsearch' required. Visit https://www.mediawiki.org/wiki/Special:MyLanguage/API:Search for help."))?).await?,
                    ListType::Protectedtitles => api::list::protectedtitles(&client).await?,
                    ListType::Querypage => api::list::querypage(&client, &parameter.ok_or_else(|| anyhow!("parameter 'qppage' required. Visit https://www.mediawiki.org/wiki/Special:MyLanguage/API:Querypage for help."))?).await?,
                    ListType::Allinfoboxes => api::list::allinfoboxes(&client).await?,
                    _ => vec![String::new()],
                };

                match output {
                    Some(o) => {
                        let mut file = fs::File::create(o).await?;
                        file.write_all(&serde_json::to_vec_pretty(&res)?).await?;
                    }
                    None => println!("{}", res.join("\n")),
                }
            }
        }
        Subcommand::Move {
            input,
            append,
            prepend,
            replace,
        } => {
            let file = fs::read_to_string(input).await?;
            let mut from: Vec<String> = Vec::new();
            let mut to: Vec<String> = Vec::new();
            file.lines().for_each(|l| {
                let parts: Vec<String> = l.split(';').map(|x| x.to_string()).collect();
                from.push(parts[0].clone());
                if parts.len() > 1 && !parts[1].is_empty() {
                    to.push(parts[1].clone());
                }
            });

            let to = if to.is_empty() {
                replace.map(|x| Destination::Replace((x[0].clone(), x[1].clone())))
            } else {
                Some(Destination::Plain(to))
            };
            api::rename::rename(&client, from, to, prepend.as_deref(), append.as_deref()).await?;
        }
        Subcommand::Nulledit { input } => {
            let contents = fs::read_to_string(input).await?;
            let titles: Vec<&str> = contents.lines().collect();
            api::edit::nulledit(&client, &titles).await?;
        }
        Subcommand::Purge { input, recursive } => {
            let contents = fs::read_to_string(input).await?;
            let titles: Vec<&str> = contents.lines().collect();
            api::purge::purge(&client, &titles, recursive).await?;
        }
        Subcommand::Upload { input, text } => {
            let mut files: Vec<PathBuf> = Vec::new();
            if input.is_file() {
                files.push(input);
            } else if input.is_dir() {
                for entry in input.read_dir()? {
                    match entry {
                        Ok(entry) => files.push(entry.path()),
                        Err(err) => println!("Invalid path in dir: {:?}\nProceeding...", err),
                    };
                }
            } else {
                return Err(anyhow!("Invalid path given!"));
            }
            api::upload::upload_multiple(&client, &files, text.as_deref()).await?;
        }
    }
    Ok(())
}

#[test]
fn verify_cli() {
    use clap::CommandFactory;
    Cli::command().debug_assert()
}
