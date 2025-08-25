use std::error::Error;
use clap::{Parser, Subcommand};
use gpx_tools::{gpx_clean, gpx_merge_files, gpx_reduce_points, gpx_reverse_tracks, gpx_to_kml};

#[derive(Subcommand)]
enum Command {
    Clean(gpx_clean::cli::Args),
    ReducePoints(gpx_reduce_points::cli::Args),
    GpxToKml(gpx_to_kml::cli::Args),
    MergeFiles(gpx_merge_files::cli::Args),
    ReverseTracks(gpx_reverse_tracks::cli::Args),
}

#[derive(Parser)]
struct Cli {
    #[clap(subcommand)]
    command: Command,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Cli::parse();

    match args.command {
        Command::Clean(args) => gpx_clean::cli::run_cli_with_args(args)?,
        Command::ReducePoints(args) => gpx_reduce_points::cli::run_cli_with_args(args),
        Command::GpxToKml(args) => gpx_to_kml::cli::run_cli_with_args(args),
        Command::MergeFiles(args) => gpx_merge_files::cli::run_cli_with_args(args)?,
        Command::ReverseTracks(args) => gpx_reverse_tracks::cli::run_cli_with_args(args),
    }

    Ok(())
}
