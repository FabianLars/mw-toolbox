use structopt::StructOpt;

mod commands;
mod gui;
mod helpers;

#[derive(StructOpt, Debug)]
enum Subcommands {
    Delete {
        #[structopt(parse(from_os_str))]
        path: std::path::PathBuf,
    },
    List {
        list_type: String,
        #[structopt(parse(from_os_str))]
        destination: std::path::PathBuf,
    },
    Update {
        update_type: String,
    },
    Test,
}

#[derive(StructOpt, Debug)]
struct Cli {
    #[structopt(subcommand)]
    subcommands: Option<Subcommands>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::from_args();

    match args.subcommands {
        None => gui::start(),
        Some(x) => match x {
            Subcommands::Delete { path } => {
                commands::delete::delete_pages(std::fs::read_to_string(&path)?).await?
            }
            Subcommands::List {
                list_type,
                destination,
            } => match list_type.as_str() {
                "images" => commands::list::images(destination).await?,
                _ => panic!("Invalid list_type"),
            },
            Subcommands::Update { update_type } => match update_type.as_str() {
                "rotation" | "rotations" => commands::update::rotation().await?,
                "champs" | "champions" => commands::update::champs().await?,
                _ => panic!("Invalid update_type"),
            },
            Subcommands::Test {} => commands::test::test().await?,
        },
    }

    Ok(())
}
