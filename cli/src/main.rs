#![forbid(unsafe_code)]

use std::path::PathBuf;

use api::rename::Destination;
use clap::Clap;
use tokio::{fs, io::AsyncWriteExt};

use wtools::{api, PathType, WikiClient};

#[derive(Clap, Debug, PartialEq)]
enum Subcommand {
    Delete {
        /// uses newline seperation
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
        /// uses newline seperation
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
        /// uses newline seperation
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
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    let cli = Cli::parse();
    let mut client = WikiClient::from(&cli.url)?;
    client.credentials(&cli.name, &cli.password);
    client.login().await?;

    match cli.command {
        Subcommand::Delete { input } => {
            let contents = fs::read_to_string(input).await?;
            let titles: Vec<&str> = contents.lines().collect();
            api::delete::delete_pages(&client, &titles).await?
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
                        api::list::backlinks(&client, parameter.as_deref()).await?
                    }
                    ListType::Categorymembers => {
                        api::list::categorymembers(&client, parameter.as_deref()).await?
                    }
                    ListType::Embeddedin => {
                        api::list::embeddedin(&client, parameter.as_deref()).await?
                    }
                    ListType::Imageusage => {
                        api::list::imageusage(&client, parameter.as_deref()).await?
                    }
                    ListType::Iwbacklinks => {
                        api::list::iwbacklinks(&client, parameter.as_deref()).await?
                    }
                    ListType::Langbacklinks => {
                        api::list::langbacklinks(&client, parameter.as_deref()).await?
                    }
                    ListType::Search => api::list::search(&client, parameter.as_deref()).await?,
                    ListType::Protectedtitles => api::list::protectedtitles(&client).await?,
                    ListType::Querypage => {
                        api::list::querypage(&client, parameter.as_deref()).await?
                    }
                    ListType::Wkpoppages => api::list::wkpoppages(&client).await?,
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
                let parts: Vec<String> = l.split(',').map(|x| x.to_string()).collect();
                from.push(parts[0].clone());
                if parts.len() > 1 && !parts[1].is_empty() {
                    to.push(parts[1].clone());
                }
            });
            let to = match to.is_empty() {
                true => match replace {
                    None => None,
                    Some(x) => Some(Destination::Replace((x[0].clone(), x[1].clone()))),
                },
                false => Some(Destination::Plain(to)),
            };
            api::rename::rename(&client, from, to, prepend, append).await?
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
        Subcommand::Upload { input } => {
            api::upload::upload(&client, PathType::from(input)?).await?
        }
        #[cfg(feature = "league-wiki")]
        Subcommand::League { league_type, path } => match league_type {
            LeagueType::Champs | LeagueType::Champions => league::champs().await?,
            LeagueType::Discount | LeagueType::Discounts => {
                league::discounts(
                    &client,
                    PathType::from(path.unwrap_or(PathBuf::from(
                        "E:/Spiele/Riot Games/League of Legends/lockfile",
                    )))?,
                )
                .await?
            }
            LeagueType::Positions => league::positions(&client).await?,
            LeagueType::Rotation | LeagueType::Rotations => {
                #[cfg(not(feature = "riot-api"))]
                panic!("Did you forget to set the riot-api feature flag?");
                #[cfg(feature = "riot-api")]
                league::rotation(&client).await?
            }
            LeagueType::Set => league::set(&client).await?,
        },
    }
    Ok(())
}
