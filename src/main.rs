use clap::{Parser, Subcommand};
use std::path::PathBuf;

mod error;
mod library;
mod scraper;

pub use self::error::{Error, Result};
use library::Library;

#[derive(Parser, Debug)]
#[clap(version)]
#[command(about = "A command line reference manager", long_about = None)]
struct Args {
    #[command(subcommand)]
    cmd: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Addentry,
    Addfile {
        filepath: String,

        #[clap(short, long)]
        delete_after: bool,
    },
    Searchkey,
}

fn main() -> Result<()> {
    let args = Args::parse();
    //dbg!(&args);
    let mut library = Library::new();
    library.load()?;
    dbg!(&library);
    match args.cmd {
        Commands::Addentry => library.add_entry(None)?,
        Commands::Addfile {
            filepath,
            delete_after,
        } => {
            let filepath = PathBuf::from(&filepath);
            library.add_file(&filepath, delete_after)?
        }
        Commands::Searchkey => todo!("Missing implementation for searchkey"),
    };
    dbg!(library);
    Ok(())
}
