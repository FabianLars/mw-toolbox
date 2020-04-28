use clap::Clap;

use core::{ commands::{ delete::*, list::*, rename::*, upload::* }, util::props::* };
#[cfg(feature = "league")]
use core::commands::league::*;
#[cfg(feature = "skylords")]
use core::commands::skylords::*;

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
    League {
        #[clap(arg_enum)]
        league_type: LeagueType,

        #[clap(short, long, parse(from_os_str))]
        path: Option<std::path::PathBuf>,
    },
    Skylords {
        #[clap(arg_enum)]
        skylords_type: SkylordsType,

        #[clap(short, long, parse(from_os_str))]
        path: Option<std::path::PathBuf>,
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
    Random,
    Rotation,
    Rotations,
    Set,
}

#[derive(Clap, Debug, PartialEq)]
enum SkylordsType {
    Carddata,
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
    command: Option<Subcommand>,

    #[clap(short = "n", long, env = "FANDOM_BOT_NAME")]
    loginname: String,
    #[clap(short = "p", long, env = "FANDOM_BOT_PASSWORD")]
    loginpassword: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    match cli.command {
        None => (),
        Some(x) => match x {
            Subcommand::Delete { input } => {
                delete_pages(Props::new(Some(input), None, cli.loginname, cli.loginpassword)).await?
            }
            Subcommand::List { list_type, parameter, output } => match list_type {
                ListType::Allimages =>
                    allimages(Props::new(output, parameter, cli.loginname, cli.loginpassword)).await?,
                ListType::Allpages =>
                    allpages(Props::new(output, parameter, cli.loginname, cli.loginpassword)).await?,
                ListType::Alllinks =>
                    alllinks(Props::new(output, parameter, cli.loginname, cli.loginpassword)).await?,
                ListType::Allcategories =>
                    allcategories(Props::new(output, parameter, cli.loginname, cli.loginpassword)).await?,
                ListType::Backlinks =>
                    backlinks(Props::new(output, parameter, cli.loginname, cli.loginpassword)).await?,
                ListType::Categorymembers =>
                    categorymembers(Props::new(output, parameter, cli.loginname, cli.loginpassword)).await?,
                ListType::Embeddedin =>
                    embeddedin(Props::new(output, parameter, cli.loginname, cli.loginpassword)).await?,
                ListType::Imageusage =>
                    imageusage(Props::new(output, parameter, cli.loginname, cli.loginpassword)).await?,
                ListType::Iwbacklinks =>
                    iwbacklinks(Props::new(output, parameter, cli.loginname, cli.loginpassword)).await?,
                ListType::Langbacklinks =>
                    langbacklinks(Props::new(output, parameter, cli.loginname, cli.loginpassword)).await?,
                ListType::Search =>
                    search(Props::new(output, parameter, cli.loginname, cli.loginpassword)).await?,
                ListType::Exturlusage =>
                    exturlusage(Props::new(output, parameter, cli.loginname, cli.loginpassword)).await?,
                ListType::Protectedtitles =>
                    protectedtitles(Props::new(output, parameter, cli.loginname, cli.loginpassword)).await?,
                ListType::Querypage =>
                    querypage(Props::new(output, parameter, cli.loginname, cli.loginpassword)).await?,
                ListType::Wkpoppages =>
                    wkpoppages(Props::new(output, parameter, cli.loginname, cli.loginpassword)).await?,
                ListType::Unconvertedinfoboxes =>
                    unconvertedinfoboxes(Props::new(output, parameter, cli.loginname, cli.loginpassword)).await?,
                ListType::Allinfoboxes =>
                    allinfoboxes(Props::new(output, parameter, cli.loginname, cli.loginpassword)).await?,
            }
            Subcommand::Move { input } => {
                move_pages(Props::new(Some(input), None, cli.loginname, cli.loginpassword)).await?
            }
            Subcommand::Upload { input } => {
                upload(Props::new(Some(input), None, cli.loginname, cli.loginpassword)).await?
            }
            #[cfg(not(feature = "league"))]
            Subcommand::League { .. } => panic!("Did you forget to set the league feature flag?"),
            #[cfg(feature = "league")]
            Subcommand::League { league_type, path } => match league_type {
                LeagueType::Champs | LeagueType::Champions => champs().await?,
                LeagueType::Discount | LeagueType::Discounts => discounts(Props::new(path, None, cli.loginname, cli.loginpassword)).await?,
                LeagueType::Random => random(Props::new(path, None, cli.loginname, cli.loginpassword)).await?,
                LeagueType::Rotation | LeagueType::Rotations => {
                    #[cfg(not(feature = "riot-api"))]
                    panic!("Did you forget to set the riot-api feature flag?");
                    #[cfg(feature = "riot-api")]
                        rotation(Props::new(path, None, cli.loginname, cli.loginpassword)).await?
                }
                LeagueType::Set => set(Props::new(path, None, cli.loginname, cli.loginpassword)).await?
            }
            #[cfg(not(feature = "skylords"))]
            Subcommand::Skylords { .. } => panic!("Did you forget to set the skylords feature flag?"),
            #[cfg(feature = "skylords")]
            Subcommand::Skylords { skylords_type, path } => match skylords_type {
                SkylordsType::Carddata => carddata(Props::new(path, None, cli.loginname, cli.loginpassword)).await?
            }
        },
    }
    Ok(())
}