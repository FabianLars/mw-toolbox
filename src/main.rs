mod commands;
mod helpers;
#[cfg(feature = "gui")]
mod gui;

use clap::{arg_enum, Clap};
use crate::helpers::props::*;

#[derive(Clap, Debug)]
enum Subcommand {
    Delete {
        /// uses newline seperation
        #[clap(parse(from_os_str))]
        input: std::path::PathBuf,
    },
    List {
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
    Update {
        update_type: UpdateType,

        #[clap(short, long, parse(from_os_str))]
        path: Option<std::path::PathBuf>,
    },
    Upload {
        #[clap(parse(from_os_str))]
        input: std::path::PathBuf,
    },
}

arg_enum! {
    #[derive(Debug)]
    enum UpdateType {
        Champs,
        Champions,
        Discount,
        Discounts,
        Rotation,
        Rotations
    }
}

arg_enum! {
    #[derive(Debug)]
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
        Allinfoboxes
    }
}

#[derive(Clap, Debug)]
struct Cli {
    #[clap(subcommand)]
    command: Option<Subcommand>,

    #[clap(short = "n", long, env = "FANDOM_BOT_NAME")]
    loginname: String,
    #[clap(short = "p", long, env = "FANDOM_BOT_PASSWORD")]
    loginpassword: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    match Cli::parse().command {
        #[cfg(not(feature = "gui"))]
        None => (),
        #[cfg(feature = "gui")]
        None => gui::app::start(),
        Some(x) => match x {
            Subcommand::Delete { .. } => {
                commands::delete::delete_pages(Props::from_delete(Cli::parse())).await?
            }
            Subcommand::List { list_type, .. } => match list_type {
                ListType::Allimages => {
                    commands::list::allimages(Props::from_list(Cli::parse())).await?
                }
                ListType::Allpages => {
                    commands::list::allpages(Props::from_list(Cli::parse())).await?
                }
                ListType::Alllinks => {
                    commands::list::alllinks(Props::from_list(Cli::parse())).await?
                }
                ListType::Allcategories => {
                    commands::list::allcategories(Props::from_list(Cli::parse())).await?
                }
                ListType::Backlinks => {
                    commands::list::backlinks(Props::from_list(Cli::parse())).await?
                }
                ListType::Categorymembers => {
                    commands::list::categorymembers(Props::from_list(Cli::parse())).await?
                }
                ListType::Embeddedin => {
                    commands::list::embeddedin(Props::from_list(Cli::parse())).await?
                }
                ListType::Imageusage => {
                    commands::list::imageusage(Props::from_list(Cli::parse())).await?
                }
                ListType::Iwbacklinks => {
                    commands::list::iwbacklinks(Props::from_list(Cli::parse())).await?
                }
                ListType::Langbacklinks => {
                    commands::list::langbacklinks(Props::from_list(Cli::parse())).await?
                }
                ListType::Search => commands::list::search(Props::from_list(Cli::parse())).await?,
                ListType::Exturlusage => {
                    commands::list::exturlusage(Props::from_list(Cli::parse())).await?
                }
                ListType::Protectedtitles => {
                    commands::list::protectedtitles(Props::from_list(Cli::parse())).await?
                }
                ListType::Querypage => {
                    commands::list::querypage(Props::from_list(Cli::parse())).await?
                }
                ListType::Wkpoppages => {
                    commands::list::wkpoppages(Props::from_list(Cli::parse())).await?
                }
                ListType::Unconvertedinfoboxes => {
                    commands::list::unconvertedinfoboxes(Props::from_list(Cli::parse())).await?
                }
                ListType::Allinfoboxes => {
                    commands::list::allinfoboxes(Props::from_list(Cli::parse())).await?
                }
            },
            Subcommand::Move { .. } => {
                commands::rename::move_pages(Props::from_move(Cli::parse())).await?
            }
            Subcommand::Update { update_type, .. } => match update_type {
                UpdateType::Champs | UpdateType::Champions => commands::update::champs().await?,
                UpdateType::Discount | UpdateType::Discounts => commands::update::discounts(Props::from_update(Cli::parse())).await?,
                UpdateType::Rotation | UpdateType::Rotations => {
                    #[cfg(not(feature = "riot-api"))]
                    panic!("Did you forget the riot-api feature flag?");
                    #[cfg(feature = "riot-api")]
                    commands::update::rotation(Props::from_update(Cli::parse())).await?
                }
            },
            Subcommand::Upload { .. } => {
                commands::upload::upload(Props::from_upload(Cli::parse())).await?
            }
        },
    }
    Ok(())
}