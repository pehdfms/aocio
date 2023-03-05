use std::{error::Error, path::PathBuf, str::FromStr};

use aocinput::{
    common::{day::AocDay, session::Session, year::AocYear},
    domain::fetcher::{cache::FileCache, InputFetcher},
};
use clap::{command, Parser, Subcommand, ValueHint::FilePath};

#[derive(Debug, Parser)]
#[command(name = "aocinput", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// Fetch Advent of Code input file
    #[command(arg_required_else_help = true)]
    Fetch {
        /// AoC Session Token
        #[arg(short, long, value_parser = Session::from_str)]
        session: Session,

        /// Year you want to fetch
        #[arg(short, long, value_parser = AocYear::from_str)]
        year: AocYear,

        /// Day you want to fetch
        #[arg(short, long, value_parser = AocDay::from_str)]
        day: AocDay,

        /// File to download the input file to, including path and filename
        /// (default: ./day{day}.txt)
        #[arg(short = 'l', long, value_hint = FilePath)]
        download_location: Option<PathBuf>,

        /// If unset then the download will fail in case a file we want to create already exists, otherwise we overwrite it
        #[arg(short, long)]
        overwrite_file: bool,
    },
}

fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Fetch {
            session,
            year,
            day,
            download_location,
            overwrite_file,
        } => {
            let download_location = download_location
                .unwrap_or(PathBuf::from(format!("./day{}.txt", usize::from(day))));

            println!(
                "Downloading AoC input for {year} Day {day} to {}",
                download_location
                    .to_str()
                    .expect("download location should be a valid path")
            );

            let mut fetcher = InputFetcher::with_cache(
                session,
                FileCache::new(|year, day| PathBuf::from(format!("day{day}.txt"))),
            );

            dbg!(fetcher.get_input(year, day));

            Ok(())
        }
    }
}
