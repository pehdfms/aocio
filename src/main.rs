use std::{error::Error, path::PathBuf, str::FromStr};

use aocio::{
    common::{day::AocDay, session::Session, year::AocYear},
    domain::fetcher::{cache::FileCache, HandleCacheHitStrategy, InputFetcher},
};
use clap::{command, Parser, Subcommand, ValueHint::DirPath};

#[derive(Debug, Parser)]
#[command(name = "aocio", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// Fetch All Input files for an Advent of Code Year
    #[command(arg_required_else_help = true)]
    FetchYear {
        /// AoC Session Token
        #[arg(short, long, value_parser = Session::from_str)]
        session: Session,

        /// Year you want to fetch
        #[arg(short, long, value_parser = AocYear::from_str)]
        year: AocYear,

        /// Directory to download file to
        /// (default: ./)
        #[arg(short = 'l', long, value_hint = DirPath)]
        download_directory: Option<PathBuf>,

        /// If unset then the download will fail in case a file we want to create already exists, otherwise we overwrite it
        #[arg(short, long)]
        overwrite_files: bool,
    },

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

        /// Directory to download file to
        /// (default: ./)
        #[arg(short = 'l', long, value_hint = DirPath)]
        download_directory: Option<PathBuf>,

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
            download_directory,
            overwrite_file,
        } => {
            let download_directory = download_directory.unwrap_or(PathBuf::from("."));

            println!(
                "Downloading AoC input for {year} Day {day} to {}",
                download_directory
                    .to_str()
                    .expect("download directory should be a valid path")
            );

            let mut fetcher = InputFetcher::with_cache(
                session,
                FileCache::new(|_, day| {
                    download_directory
                        .join(PathBuf::from(format!("day{day}.txt")))
                        .to_path_buf()
                }),
            );

            let handle_cache_hit = if overwrite_file {
                HandleCacheHitStrategy::OverwriteCache
            } else {
                HandleCacheHitStrategy::ErrorOnCacheHit
            };

            fetcher.get_input_handle_cache(year, day, handle_cache_hit)?;

            Ok(())
        }
        Commands::FetchYear {
            session,
            year,
            download_directory,
            overwrite_files,
        } => {
            let download_directory = download_directory.unwrap_or(PathBuf::from("."));

            println!(
                "Downloading AoC inputs for {year} to {}",
                download_directory
                    .to_str()
                    .expect("download directory should be a valid path")
            );

            let mut fetcher = InputFetcher::with_cache(
                session,
                FileCache::new(|_, day| {
                    download_directory
                        .join(PathBuf::from(format!("day{day}.txt")))
                        .to_path_buf()
                }),
            );

            let handle_cache_hit = if overwrite_files {
                HandleCacheHitStrategy::OverwriteCache
            } else {
                HandleCacheHitStrategy::ErrorOnCacheHit
            };

            for day in 1..=25 {
                let day = AocDay::try_from(day)
                    .expect("Advent of Code days should range between 1 and 25, if you see this error contact the library author!");

                println!("Downloading AoC Input for Day {day}");
                fetcher.get_input_handle_cache(year, day, handle_cache_hit)?;
            }

            Ok(())
        }
    }
}
