use structopt::StructOpt;

mod commands;
#[cfg(any(feature = "gui-iced", feature = "gui-azul"))]
mod gui;
mod helpers;

#[derive(StructOpt, Debug)]
enum Subcommands {
    Delete {
        #[structopt(parse(from_os_str))]
        input: std::path::PathBuf,
        #[structopt(short = "n", long, env = "FANDOM_BOT_NAME")]
        loginname: String,
        #[structopt(short = "p", long, env = "FANDOM_BOT_PASSWORD")]
        loginpassword: String,
    },
    List {
        list_type: String,
        #[structopt(parse(from_os_str))]
        output: std::path::PathBuf,
        #[structopt(short = "n", long, env = "FANDOM_BOT_NAME")]
        loginname: String,
        #[structopt(short = "p", long, env = "FANDOM_BOT_PASSWORD")]
        loginpassword: String,
    },
    Update {
        update_type: String,
        #[structopt(short = "n", long, env = "FANDOM_BOT_NAME")]
        loginname: String,
        #[structopt(short = "p", long, env = "FANDOM_BOT_PASSWORD")]
        loginpassword: String,
    },
}

#[cfg(any(feature = "gui-iced", feature = "gui-azul"))]
#[derive(StructOpt, Debug)]
struct Cli {
    #[structopt(subcommand)]
    subcommands: Option<Subcommands>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    #[cfg(any(feature = "gui-iced", feature = "gui-azul"))]
    let args = Cli::from_args();
    #[cfg(any(feature = "gui-iced", feature = "gui-azul"))]
    match args.subcommands {
        #[cfg(feature = "gui-iced")]
        None => gui::iced::start(),
        #[cfg(feature = "gui-azul")]
        None => gui::azul::start(),
        Some(x) => match x {
            Subcommands::Delete { input, loginname, loginpassword } => {
                commands::delete::delete_pages(std::fs::read_to_string(&input)?, loginname, loginpassword).await?
            }
            Subcommands::List {
                list_type,
                output,
                loginname,
                loginpassword,

            } => match list_type.as_str() {
                "images" => commands::list::images(output, loginname, loginpassword).await?,
                _ => panic!("Invalid list_type"),
            },
            Subcommands::Update { update_type, loginname, loginpassword } => match update_type.as_str() {
                #[cfg(feature = "riot-api")]
                "rotation" | "rotations" => commands::update::rotation(loginname, loginpassword).await?,
                "champs" | "champions" => commands::update::champs().await?,
                _ => panic!("Invalid update_type"),
            },
        },
    }

    #[cfg(not(any(feature = "gui-iced", feature = "gui-azul")))]
    let args = Subcommands::from_args();
    #[cfg(not(any(feature = "gui-iced", feature = "gui-azul")))]
    match args {
        Subcommands::Delete { input, loginname, loginpassword } => {
            commands::delete::delete_pages(std::fs::read_to_string(&input)?, loginname, loginpassword).await?
        }
        Subcommands::List {
            list_type,
            output,
            loginname,
            loginpassword
        } => match list_type.as_str() {
            "images" => commands::list::images(output, loginname, loginpassword).await?,
            _ => panic!("Invalid list_type"),
        },
        Subcommands::Update { update_type, loginname, loginpassword } => match update_type.as_str() {
            #[cfg(feature = "riot-api")]
            "rotation" | "rotations" => commands::update::rotation(loginname, loginpassword).await?,
            "champs" | "champions" => commands::update::champs().await?,
            _ => panic!("Invalid update_type"),
        },
    }

    Ok(())
}
