#![forbid(unsafe_code)]
use std::fs;
use std::path::PathBuf;

use clap::Clap;

#[cfg(feature = "league-wiki")]
use league::*;
use wtools::{Api, PathType};

#[derive(Clap, Debug, PartialEq)]
enum Subcommand {
    Delete {
        /// uses newline seperation
        #[clap(parse(from_os_str))]
        input: std::path::PathBuf,
    },
    List {
        #[clap(arg_enum)]
        list_type: ListType,

        parameter: Option<String>,

        #[clap(short, long, parse(from_os_str))]
        output: Option<std::path::PathBuf>,
    },
    Move {
        /// uses newline seperation
        #[clap(parse(from_os_str))]
        input: std::path::PathBuf,
    },
    #[cfg(feature = "league-wiki")]
    League {
        #[clap(arg_enum)]
        league_type: LeagueType,

        #[clap(short, long, parse(from_os_str))]
        path: Option<std::path::PathBuf>,
    },
    Purge {
        #[clap(long)]
        forcelinkupdate: bool,

        pages: Option<String>,
        #[clap(parse(from_os_str))]
        file: Option<std::path::PathBuf>,
    },
    Upload {
        #[clap(parse(from_os_str))]
        input: std::path::PathBuf,
    },
}

#[derive(Clap, Debug, PartialEq)]
enum LeagueType {
    Champs,
    Champions,
    Discount,
    Discounts,
    Positions,
    Random,
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
    let api = Api::from(&cli.url)
        .credentials(&cli.name, &cli.password)
        .login()
        .await?;

    match cli.command {
        Subcommand::Delete { input } => {
            let contents = fs::read_to_string(input)?;
            let titles: Vec<&str> = contents.lines().collect();
            api.delete_pages(&titles).await?
        }
        Subcommand::List {
            list_type,
            parameter,
            output,
        } => {
            if list_type == ListType::Exturlusage {
                ::serde_json::to_writer_pretty(
                    &fs::File::create(
                        output.unwrap_or_else(|| PathBuf::from("./wtools_output.json")),
                    )?,
                    &api.exturlusage().await?,
                )?;
            } else {
                ::serde_json::to_writer_pretty(
                    &fs::File::create(
                        output.unwrap_or_else(|| PathBuf::from("./wtools_output.json")),
                    )?,
                    &match list_type {
                        ListType::Allimages => api.allimages().await?,
                        ListType::Allpages => api.allpages(parameter.as_deref()).await?,
                        ListType::Alllinks => api.alllinks().await?,
                        ListType::Allcategories => api.allcategories().await?,
                        ListType::Backlinks => api.backlinks(parameter.as_deref()).await?,
                        ListType::Categorymembers => {
                            api.categorymembers(parameter.as_deref()).await?
                        }
                        ListType::Embeddedin => api.embeddedin(parameter.as_deref()).await?,
                        ListType::Imageusage => api.imageusage(parameter.as_deref()).await?,
                        ListType::Iwbacklinks => api.iwbacklinks(parameter.as_deref()).await?,
                        ListType::Langbacklinks => api.langbacklinks(parameter.as_deref()).await?,
                        ListType::Search => api.search(parameter.as_deref()).await?,
                        ListType::Protectedtitles => api.protectedtitles().await?,
                        ListType::Querypage => api.querypage(parameter.as_deref()).await?,
                        ListType::Wkpoppages => api.wkpoppages().await?,
                        ListType::Unconvertedinfoboxes => api.unconvertedinfoboxes().await?,
                        ListType::Allinfoboxes => api.allinfoboxes().await?,
                        _ => vec![String::new()],
                    },
                )?;
            }
        }
        Subcommand::Move { input } => api.move_pages(PathType::new(input)).await?,
        Subcommand::Purge { pages, .. } => {
            println!("{:?}", pages.unwrap());
            api.purge().await
        }
        Subcommand::Upload { input } => api.upload(PathType::new(input)).await?,
        #[cfg(feature = "league-wiki")]
        Subcommand::League { league_type, path } => match league_type {
            LeagueType::Champs | LeagueType::Champions => champs().await?,
            LeagueType::Discount | LeagueType::Discounts => {
                api.discounts(PathType::new(input.unwrap_or(PathBuf::from(
                    "E:/Spiele/Riot Games/League of Legends/lockfile",
                ))))
                .await?
            }
            LeagueType::Positions => positions(&api).await?,
            LeagueType::Random => random(&api).await?,
            LeagueType::Rotation | LeagueType::Rotations => {
                #[cfg(not(feature = "riot-api"))]
                panic!("Did you forget to set the riot-api feature flag?");
                #[cfg(feature = "riot-api")]
                rotation(&api).await?
            }
            LeagueType::Set => set(&api).await?,
        },
    }
    Ok(())
}
