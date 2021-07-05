#![forbid(unsafe_code)]

use std::path::PathBuf;

use anyhow::{anyhow, Result};
use clap::Clap;
use tokio::{fs, io::AsyncWriteExt};

use api::rename::Destination;
use mw_tools::{api, WikiClient};

#[derive(Clap, Debug, PartialEq)]
enum Subcommand {
    Delete {
        /// uses newline separation
        #[clap(parse(from_os_str))]
        input: PathBuf,
    },
    List {
        #[clap(arg_enum)]
        list_type: ListType,

        parameter: Option<String>,

        #[clap(short, long, parse(from_os_str))]
        output: Option<PathBuf>,
    },
    Move {
        /// uses newline separation
        #[clap(parse(from_os_str))]
        input: PathBuf,
        #[clap(long)]
        append: Option<String>,
        #[clap(long)]
        prepend: Option<String>,
        #[clap(long)]
        replace: Option<Vec<String>>,
    },
    Nulledit {
        /// uses newline separation
        #[clap(parse(from_os_str))]
        input: PathBuf,
    },
    Purge {
        #[clap(short, long)]
        recursive: bool,

        #[clap(parse(from_os_str))]
        input: PathBuf,
    },
    Upload {
        #[clap(parse(from_os_str))]
        input: PathBuf,

        #[clap(short, long)]
        text: Option<String>,
    },

    #[cfg(feature = "league-wiki")]
    League {
        #[clap(arg_enum)]
        league_type: LeagueType,

        #[clap(short, long, parse(from_os_str))]
        path: Option<PathBuf>,
    },
}

#[derive(Clap, Debug, PartialEq)]
enum LeagueType {
    Champs,
    Champions,
    Discount,
    Discounts,
    Positions,
    Rotation,
    Rotations,
    Set,
}

#[derive(Clap, Debug, PartialEq)]
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

#[derive(Clap, Debug, PartialEq)]
struct Cli {
    #[clap(subcommand)]
    command: Subcommand,

    #[clap(short, long, env = "FANDOM_BOT_NAME", hide_env_values = true)]
    name: String,
    #[clap(short, long, env = "FANDOM_BOT_PASSWORD", hide_env_values = true)]
    password: String,
    #[clap(
        short,
        long,
        default_value = "https://leagueoflegends.fandom.com/de/api.php"
    )]
    url: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();

    let cli = Cli::parse();
    let client = WikiClient::new_logged_in(&cli.url, &cli.name, &cli.password).await?;

    match cli.command {
        Subcommand::Delete { input } => {
            let contents = fs::read_to_string(input).await?;
            let titles: Vec<&str> = contents.lines().collect();
            api::delete::delete(&client, &titles, None).await?
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
                    ListType::Allpages => {
                        api::list::allpages(&client, parameter.as_deref()).await?
                    }
                    ListType::Alllinks => api::list::alllinks(&client).await?,
                    ListType::Allcategories => api::list::allcategories(&client).await?,
                    ListType::Backlinks => {
                        api::list::backlinks(&client, &parameter.ok_or_else(|| anyhow!("parameter 'bltitle' required. Visit https://www.mediawiki.org/wiki/Special:MyLanguage/API:Backlinks for help."))?).await?
                    }
                    ListType::Categorymembers => {
                        api::list::categorymembers(&client, &parameter.ok_or_else(|| anyhow!("parameter 'cmtitle' required. Visit https://www.mediawiki.org/wiki/Special:MyLanguage/API:Categorymembers for help."))?).await?
                    }
                    ListType::Embeddedin => {
                        api::list::embeddedin(&client, &parameter.ok_or_else(|| anyhow!("parameter 'eititle' required. Visit https://www.mediawiki.org/wiki/Special:MyLanguage/API:Embeddedin for help."))?).await?
                    }
                    ListType::Imageusage => {
                        api::list::imageusage(&client, &parameter.ok_or_else(|| anyhow!("parameter 'iutitle' required. Visit https://www.mediawiki.org/wiki/Special:MyLanguage/API:Imageusage for help."))?).await?
                    }
                    ListType::Search => api::list::search(&client, &parameter.ok_or_else(|| anyhow!("parameter 'srsearch' required. Visit https://www.mediawiki.org/wiki/Special:MyLanguage/API:Search for help."))?).await?,
                    ListType::Protectedtitles => api::list::protectedtitles(&client).await?,
                    ListType::Querypage => {
                        api::list::querypage(&client, &parameter.ok_or_else(|| anyhow!("parameter 'qppage' required. Visit https://www.mediawiki.org/wiki/Special:MyLanguage/API:Querypage for help."))?).await?
                    }
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
            api::rename::rename(&client, from, to, prepend.as_deref(), append.as_deref()).await?
        }
        Subcommand::Nulledit { input } => {
            let contents = fs::read_to_string(input).await?;
            let titles: Vec<&str> = contents.lines().collect();
            api::edit::nulledit(&client, &titles).await?
        }
        Subcommand::Purge { input, recursive } => {
            let contents = fs::read_to_string(input).await?;
            let titles: Vec<&str> = contents.lines().collect();
            api::purge::purge(&client, &titles, recursive).await?
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
                    }
                }
            } else {
                return Err(anyhow!("Invalid path given!"));
            }
            api::upload::upload_multiple(&client, &files, text.as_deref()).await?
        }
        #[cfg(feature = "league-wiki")]
        Subcommand::League { league_type, path } => match league_type {
            LeagueType::Champs | LeagueType::Champions => league::champs().await?,
            LeagueType::Discount | LeagueType::Discounts => {
                let path = match path {
                    Some(p) => p,
                    None => get_client_path()?,
                };

                league::discounts(&client, path).await?
            }
            LeagueType::Positions => league::positions(&client).await?,
            LeagueType::Rotation | LeagueType::Rotations => {
                #[cfg(not(feature = "riot-api"))]
                return Err(anyhow!("Did you forget to set the riot-api feature flag?"));
                #[cfg(feature = "riot-api")]
                league::rotation(&client).await?
            }
            LeagueType::Set => league::set(&client).await?,
        },
    }
    Ok(())
}

#[cfg(feature = "league-wiki")]
fn get_client_path() -> Result<PathBuf> {
    use sysinfo::{ProcessExt, RefreshKind, System, SystemExt};

    let system = System::new_with_specifics(RefreshKind::new().with_processes());

    let process = system.process_by_name("LeagueClient.exe");

    if let Some(p) = process.get(0) {
        if let Some(path) = p.exe().parent() {
            let mut path = path.to_path_buf();
            path.push("lockfile");
            return Ok(path);
        }
    }

    Err(anyhow!("Can't find lockfile. Make sure that the League Client is running. If it still doesn't work, try specifying the path to the lockfile yourself."))
}
