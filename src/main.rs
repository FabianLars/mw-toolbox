mod commands;
mod gui;
mod helpers;

use clap::Clap;

#[derive(Clap, Debug)]
enum Subcommand {
    Delete {
        /// uses newline seperation
        #[clap(parse(from_os_str))]
        input: std::path::PathBuf,
    },
    List {
        #[clap(case_insensitive = true, possible_values(&[
            "allimages",
            "allpages",
            "alllinks",
            "allcategories",
            "backlinks",
            "categorymembers",
            "embeddedin",
            "imageusage",
            "iwbacklinks",
            "langbacklinks",
            "search",
            "exturlusage",
            "protectedtitles",
            "querypage",
            "wkpoppages",
            "unconvertedinfoboxes",
            "allinfoboxes",
        ]))]
        list_type: String,

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
        #[clap(subcommand)]
        update_type: UpdateType,

        #[clap(short, long, parse(from_os_str))]
        output: Option<std::path::PathBuf>,
    },
}

#[derive(Clap, Debug)]
enum UpdateType {
    #[cfg(feature = "riot-api")]
    Rotation,
    #[cfg(feature = "riot-api")]
    Rotations,

    Champs,
    Champions,
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
        None => gui::iced::start(),
        Some(x) => match x {
            Subcommand::Delete { .. } => commands::delete::delete_pages(DeleteProps::new(Cli::parse())).await?,
            Subcommand::List { list_type, .. } => match list_type.to_lowercase().as_str() {
                "allimages" => commands::list::allimages(ListProps::new(Cli::parse())).await?,
                "allpages" => commands::list::allpages(ListProps::new(Cli::parse())).await?,
                "alllinks" => commands::list::alllinks(ListProps::new(Cli::parse())).await?,
                "allcategories" => commands::list::allcategories(ListProps::new(Cli::parse())).await?,
                "backlinks" => commands::list::backlinks(ListProps::new(Cli::parse())).await?,
                "categorymembers" => commands::list::categorymembers(ListProps::new(Cli::parse())).await?,
                "embeddedin" => commands::list::embeddedin(ListProps::new(Cli::parse())).await?,
                "imageusage" => commands::list::imageusage(ListProps::new(Cli::parse())).await?,
                "iwbacklinks" => commands::list::iwbacklinks(ListProps::new(Cli::parse())).await?,
                "langbacklinks" => commands::list::langbacklinks(ListProps::new(Cli::parse())).await?,
                "search" => commands::list::search(ListProps::new(Cli::parse())).await?,
                "exturlusage" => commands::list::exturlusage(ListProps::new(Cli::parse())).await?,
                "protectedtitles" => commands::list::protectedtitles(ListProps::new(Cli::parse())).await?,
                "querypage" => commands::list::querypage(ListProps::new(Cli::parse())).await?,
                "wkpoppages" => commands::list::wkpoppages(ListProps::new(Cli::parse())).await?,
                "unconvertedinfoboxes" => commands::list::unconvertedinfoboxes(ListProps::new(Cli::parse())).await?,
                "allinfoboxes" => commands::list::allinfoboxes(ListProps::new(Cli::parse())).await?,
                _ => panic!("LOL"),
            },
            Subcommand::Move { .. } => commands::rename::move_pages(MoveProps::new(Cli::parse())).await?,
            Subcommand::Update { update_type, .. } => match update_type {
                UpdateType::Champs | UpdateType::Champions => commands::update::champs().await?,
                #[cfg(feature = "riot-api")]
                UpdateType::Rotation | UpdateType::Rotations => commands::update::rotation(UpdateProps::new(Cli::parse())).await?,
            },
        }
    }
    Ok(())
}

pub struct DeleteProps {
    input: String,
    loginname: String,
    loginpassword: String,
}

impl DeleteProps {
    fn new(args: Cli) -> Self {
        let input: String = match args.command.unwrap() {
            Subcommand::Delete { input } => std::fs::read_to_string(input).unwrap(),
            _ => panic!("weird error")
        };


        return Self {
            input,
            loginname: args.loginname,
            loginpassword: args.loginpassword,
        }
    }
}

pub struct ListProps {
    output: std::path::PathBuf,
    parameter: String,
    loginname: String,
    loginpassword: String,
}

impl ListProps {
    fn new(args: Cli) -> Self {
        let out: std::path::PathBuf;
        let param: String;

        match args.command.expect("args.command") {
            Subcommand::List { output, parameter, .. } => {
            out = match output {
                Some(x) => x,
                None => std::path::PathBuf::from("./wtools_output.json"),
            };

            param = match parameter {
                Some(x) => x,
                None => "".to_string(),
            };
        },
            _ => panic!("weird error")
        }

        return Self {
            output: out,
            parameter: param,
            loginname: args.loginname,
            loginpassword: args.loginpassword,
        }
    }
}

pub struct MoveProps {
    input: String,
    loginname: String,
    loginpassword: String,
}

impl MoveProps {
    fn new(args: Cli) -> Self {
        let input: String = match args.command.unwrap() {
            Subcommand::Move { input } => std::fs::read_to_string(input).unwrap(),
            _ => panic!("weird error")
        };

        return Self {
            input,
            loginname: args.loginname,
            loginpassword: args.loginpassword,
        }
    }
}

#[cfg(feature = "riot-api")]
pub struct UpdateProps {
    output: std::path::PathBuf,
    loginname: String,
    loginpassword: String,
}

#[cfg(feature = "riot-api")]
impl UpdateProps {
    fn new(args: Cli) -> Self {
        let output = match args.command.unwrap() {
            Subcommand::Update { output, .. } => match output {
                Some(x) => x,
                None => std::path::PathBuf::from("./wtools_output.json"),
            },
            _ => panic!("weird error")
        };

        return Self {
            output,
            loginname: args.loginname,
            loginpassword: args.loginpassword,
        }
    }
}