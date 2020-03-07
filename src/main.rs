mod commands;
mod gui;
mod helpers;

use structopt::{ StructOpt, clap::arg_enum };

#[derive(StructOpt, Debug)]
enum Subcommand {
    Delete {
        #[structopt(parse(from_os_str))]
        input: std::path::PathBuf,
    },
    List {
        #[structopt(subcommand)]
        list_type: ListType,

        #[structopt(parse(from_os_str))]
        output: Option<std::path::PathBuf>,
    },
    Move {
        #[structopt(parse(from_os_str))]
        input: std::path::PathBuf,
    },
    Update {
        #[structopt(subcommand)]
        update_type: UpdateType,

        #[structopt(parse(from_os_str))]
        output: Option<std::path::PathBuf>,
    },
}

arg_enum!{
    #[derive(Debug)]
    enum Format {
        Json,
        Newline,
    }
}

#[derive(StructOpt, Debug)]
enum UpdateType {
    #[cfg(feature = "riot-api")]
    Rotation,
    #[cfg(feature = "riot-api")]
    Rotations,

    Champs,
    Champions,
}

#[derive(StructOpt, Debug)]
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

#[derive(StructOpt, Debug)]
struct Cli {
    #[structopt(subcommand)]
    command: Option<Subcommand>,

    #[structopt(short, long, case_insensitive = true, possible_values = &Format::variants(), default_value = "newline", about = "Format to use for input and/or output (json or newline seperation). Newline is default.")]
    format: Format,

    #[structopt(short = "n", long, env = "FANDOM_BOT_NAME")]
    loginname: String,
    #[structopt(short = "p", long, env = "FANDOM_BOT_PASSWORD")]
    loginpassword: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    match Cli::from_args().command {
        None => gui::iced::start(),
        Some(x) => match x {
            Subcommand::Delete { .. } => commands::delete::delete_pages(DeleteProps::new(Cli::from_args())).await?,
            Subcommand::List { list_type, .. } => match list_type {
                ListType::Allimages => commands::list::allimages(ListProps::new(Cli::from_args())).await?,
                ListType::Allpages => commands::list::allpages(ListProps::new(Cli::from_args())).await?,
                _ => panic!("invalid list type")
            },
            Subcommand::Move { .. } => commands::massmove::move_pages(MoveProps::new(Cli::from_args())).await?,
            Subcommand::Update { update_type, .. } => match update_type {
                UpdateType::Champs | UpdateType::Champions => commands::update::champs(UpdateProps::new(Cli::from_args())).await?,
                #[cfg(feature = "riot-api")]
                UpdateType::Rotation | UpdateType::Rotations => commands::update::rotation(UpdateProps::new(Cli::from_args())).await?,
            },
        }
    }
    Ok(())
}

pub struct DeleteProps {
    input: String,
    format: Format,
    loginname: String,
    loginpassword: String,
}

impl DeleteProps {
    fn new(args: Cli) -> Self {
        let input: String = match args.command.unwrap() {
            Subcommand::Delete { input } => std::fs::read_to_string(input).unwrap(),
            _ => panic!("weird error")
        };

        let format = match args.format {
            Format::Json => Format::Json,
            _ => Format::Newline,
        };

        return Self {
            input,
            format,
            loginname: args.loginname,
            loginpassword: args.loginpassword,
        }
    }
}

pub struct ListProps {
    output: std::path::PathBuf,
    format: Format,
    loginname: String,
    loginpassword: String,
}

impl ListProps {
    fn new(args: Cli) -> Self {
        let format = match args.format {
            Format::Json => Format::Json,
             _ => Format::Newline,
        };

        let output = match args.command.unwrap() {
            Subcommand::List { output, .. } => match output {
                Some(x) => x,
                None => match format {
                    Format::Json => std::path::PathBuf::from("./wtools_output.json"),
                    _ => std::path::PathBuf::from("./wtools_output.txt"),
                }
            },
            _ => panic!("weird error")
        };

        return Self {
            output,
            format,
            loginname: args.loginname,
            loginpassword: args.loginpassword,
        }
    }
}

pub struct MoveProps {
    input: String,
    format: Format,
    loginname: String,
    loginpassword: String,
}

impl MoveProps {
    fn new(args: Cli) -> Self {
        let format = match args.format {
            Format::Json => Format::Json,
            _ => Format::Newline,
        };

        let input: String = match args.command.unwrap() {
            Subcommand::Move { input } => std::fs::read_to_string(input).unwrap(),
            _ => panic!("weird error")
        };

        return Self {
            input,
            format,
            loginname: args.loginname,
            loginpassword: args.loginpassword,
        }
    }
}

pub struct UpdateProps {
    output: std::path::PathBuf,
    format: Format,
    loginname: String,
    loginpassword: String,
}

impl UpdateProps {
    fn new(args: Cli) -> Self {
        let format = match args.format {
            Format::Json => Format::Json,
            _ => Format::Newline,
        };

        let output = match args.command.unwrap() {
            Subcommand::Update { output, .. } => match output {
                Some(x) => x,
                None => match format {
                    Format::Json => std::path::PathBuf::from("./wtools_output.json"),
                    _ => std::path::PathBuf::from("./wtools_output.txt"),
                }
            },
            _ => panic!("weird error")
        };

        return Self {
            output,
            format,
            loginname: args.loginname,
            loginpassword: args.loginpassword,
        }
    }
}