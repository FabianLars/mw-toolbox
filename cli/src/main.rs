use clap::Clap;

use core::{ commands::{ delete::*, list::*, rename::*, upload::* }, util::config::* };
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
                delete_pages(Config::new(cli.loginname, cli.loginpassword).with_pathbuf(input)).await?
            }
            Subcommand::List { list_type, parameter, output } => {
                let props = Config::new(cli.loginname, cli.loginpassword).with_parameter(parameter).with_pathbuf_opt(output);
                if list_type == ListType::Exturlusage {
                    ::serde_json::to_writer_pretty(&std::fs::File::create(props.path.clone().file_path())?, &exturlusage(props).await?)?;
                } else {
                    ::serde_json::to_writer_pretty(&std::fs::File::create(props.path.clone().file_path())?, &match list_type {
                        ListType::Allimages =>
                            allimages(props).await?,
                        ListType::Allpages =>
                            allpages(props).await?,
                        ListType::Alllinks =>
                            alllinks(props).await?,
                        ListType::Allcategories =>
                            allcategories(props).await?,
                        ListType::Backlinks =>
                            backlinks(props).await?,
                        ListType::Categorymembers =>
                            categorymembers(props).await?,
                        ListType::Embeddedin =>
                            embeddedin(props).await?,
                        ListType::Imageusage =>
                            imageusage(props).await?,
                        ListType::Iwbacklinks =>
                            iwbacklinks(props).await?,
                        ListType::Langbacklinks =>
                            langbacklinks(props).await?,
                        ListType::Search =>
                            search(props).await?,
                        ListType::Protectedtitles =>
                            protectedtitles(props).await?,
                        ListType::Querypage =>
                            querypage(props).await?,
                        ListType::Wkpoppages =>
                            wkpoppages(props).await?,
                        ListType::Unconvertedinfoboxes =>
                            unconvertedinfoboxes(props).await?,
                        ListType::Allinfoboxes =>
                            allinfoboxes(props).await?,
                        _ => vec![String::new()],
                    })?;
                }
            }
            Subcommand::Move { input } => {
                move_pages(Config::new(cli.loginname, cli.loginpassword).with_pathbuf(input)).await?
            }
            Subcommand::Upload { input } => {
                upload(Config::new(cli.loginname, cli.loginpassword).with_pathbuf(input)).await?
            }
            #[cfg(not(feature = "league"))]
            Subcommand::League { .. } => panic!("Did you forget to set the league feature flag?"),
            #[cfg(feature = "league")]
            Subcommand::League { league_type, path } => match league_type {
                LeagueType::Champs | LeagueType::Champions => champs().await?,
                LeagueType::Discount | LeagueType::Discounts => discounts(Config::new(cli.loginname, cli.loginpassword).with_pathbuf_opt(path)).await?,
                LeagueType::Random => random(Config::new(cli.loginname, cli.loginpassword).with_pathbuf_opt(path)).await?,
                LeagueType::Rotation | LeagueType::Rotations => {
                    #[cfg(not(feature = "riot-api"))]
                    panic!("Did you forget to set the riot-api feature flag?");
                    #[cfg(feature = "riot-api")]
                        rotation(Config::new(cli.loginname, cli.loginpassword).with_pathbuf_opt(path)).await?
                }
                LeagueType::Set => set(Config::new(cli.loginname, cli.loginpassword).with_pathbuf_opt(path)).await?
            }
            #[cfg(not(feature = "skylords"))]
            Subcommand::Skylords { .. } => panic!("Did you forget to set the skylords feature flag?"),
            #[cfg(feature = "skylords")]
            Subcommand::Skylords { skylords_type, path } => match skylords_type {
                SkylordsType::Carddata => carddata(Config::new(cli.loginname, cli.loginpassword).with_pathbuf_opt(path)).await?
            }
        },
    }
    Ok(())
}