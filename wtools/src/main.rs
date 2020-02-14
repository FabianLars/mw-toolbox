use structopt::StructOpt;

mod commands;
mod helpers;

#[derive(StructOpt, Debug)]
enum Cli {
    Delete {
        #[structopt(parse(from_os_str))]
        path: std::path::PathBuf
    },
    Rotation
}

fn main() {
    let args = Cli::from_args();
    //let content = std::fs::read_to_string(&args.delete.path)
    //    .expect("could not read file");

    match args {
        Cli::Delete { path } => commands::delete::delete_pages(std::fs::read_to_string(&path).expect("could not read file")),
        Cli::Rotation => commands::rotation::update_rotation(),
    }

}


