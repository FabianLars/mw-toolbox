#![forbid(unsafe_code)]

use std::path::PathBuf;

use clap::Clap;
use tokio::{fs, prelude::*};

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
    Unconvertedinfoboxes,
    Allinfoboxes,
}

#[derive(Clap, Debug, PartialEq)]
struct Cli {
    #[clap(subcommand)]
    command: Subcommand,

    #[clap(short, long, env = "FANDOM_BOT_NAME")]
    name: String,
    #[clap(short, long, env = "FANDOM_BOT_PASSWORD")]
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
                    ListType::Unconvertedinfoboxes => {
                        api::list::unconvertedinfoboxes(&client).await?
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
        Subcommand::Move { input } => {
            api::rename::move_pages(&client, PathType::from(input)?).await?
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
