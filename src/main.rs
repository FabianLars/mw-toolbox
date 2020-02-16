use structopt::StructOpt;

mod commands;
mod helpers;

#[derive(StructOpt, Debug)]
enum Cli {
    Delete {
        #[structopt(parse(from_os_str))]
        path: std::path::PathBuf,
    },
    Rotation,
    List {
        list_type: String,
        #[structopt(parse(from_os_str))]
        destination: std::path::PathBuf,
    },
}

fn main() {
    let args = Cli::from_args();

    match args {
        Cli::Delete { path } => commands::delete::delete_pages(
            std::fs::read_to_string(&path).expect("could not read file path"),
        ),
        Cli::Rotation => commands::rotation::update_rotation(),
        Cli::List {
            list_type,
            destination,
        } => commands::list::distributor(list_type, destination),
    }
}
