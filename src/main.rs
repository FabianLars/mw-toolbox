mod commands;
mod gui;
mod helpers;

use clap::{Clap, arg_enum};

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
        output: Option<std::path::PathBuf>,
    },
}

arg_enum! {
    #[derive(Debug)]
    enum UpdateType {
        Champs,
        Champions,
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
        None => gui::iced::start(),
        Some(x) => match x {
            Subcommand::Delete { .. } => commands::delete::delete_pages(DeleteProps::new(Cli::parse())).await?,
            Subcommand::List { list_type, .. } => match list_type {
                ListType::Allimages => commands::list::allimages(ListProps::new(Cli::parse())).await?,
                ListType::Allpages => commands::list::allpages(ListProps::new(Cli::parse())).await?,
                ListType::Alllinks => commands::list::alllinks(ListProps::new(Cli::parse())).await?,
                ListType::Allcategories => commands::list::allcategories(ListProps::new(Cli::parse())).await?,
                ListType::Backlinks => commands::list::backlinks(ListProps::new(Cli::parse())).await?,
                ListType::Categorymembers => commands::list::categorymembers(ListProps::new(Cli::parse())).await?,
                ListType::Embeddedin => commands::list::embeddedin(ListProps::new(Cli::parse())).await?,
                ListType::Imageusage => commands::list::imageusage(ListProps::new(Cli::parse())).await?,
                ListType::Iwbacklinks => commands::list::iwbacklinks(ListProps::new(Cli::parse())).await?,
                ListType::Langbacklinks => commands::list::langbacklinks(ListProps::new(Cli::parse())).await?,
                ListType::Search => commands::list::search(ListProps::new(Cli::parse())).await?,
                ListType::Exturlusage => commands::list::exturlusage(ListProps::new(Cli::parse())).await?,
                ListType::Protectedtitles => commands::list::protectedtitles(ListProps::new(Cli::parse())).await?,
                ListType::Querypage => commands::list::querypage(ListProps::new(Cli::parse())).await?,
                ListType::Wkpoppages => commands::list::wkpoppages(ListProps::new(Cli::parse())).await?,
                ListType::Unconvertedinfoboxes => commands::list::unconvertedinfoboxes(ListProps::new(Cli::parse())).await?,
                ListType::Allinfoboxes => commands::list::allinfoboxes(ListProps::new(Cli::parse())).await?,
            },
            Subcommand::Move { .. } => commands::rename::move_pages(MoveProps::new(Cli::parse())).await?,
            Subcommand::Update { update_type, .. } => match update_type {
                UpdateType::Champs | UpdateType::Champions => commands::update::champs().await?,
                #[cfg(feature = "riot-api")]
                UpdateType::Rotation | UpdateType::Rotations => commands::update::rotation(UpdateProps::new(Cli::parse())).await?,
                _ => panic!("did you use update rotation without feature flag 'riot-api' being set?"),
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