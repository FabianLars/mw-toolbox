use structopt::StructOpt;

mod commands;
mod helpers;

#[derive(StructOpt, Debug)]
enum Cli {
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
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::from_args();

    match args {
        Cli::Delete { path } => {
            commands::delete::delete_pages(std::fs::read_to_string(&path)?).await?
        }
        Cli::List {
            list_type,
            destination,
        } => match list_type.as_str() {
            "images" => commands::list::images(destination).await?,
            _ => panic!("Invalid list_type"),
        },
        Cli::Update { update_type } => match update_type.as_str() {
            "rotation" | "rotations" => commands::update::rotation().await?,
            "champs" | "champions" => commands::update::champs().await?,
            _ => panic!("Invalid update_type"),
        },
    }
    Ok(())
}
