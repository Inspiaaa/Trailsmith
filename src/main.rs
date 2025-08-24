use clap::{Parser, Subcommand};
use gpx_tools::{gpx_clean, gpx_merge, gpx_reduce_points, gpx_to_kml};

#[derive(Subcommand)]
enum Command {
    Clean(gpx_clean::cli::Args),
    ReducePoints(gpx_reduce_points::cli::Args),
    GpxToKml(gpx_to_kml::cli::Args),
    Merge(gpx_merge::cli::Args),
}

#[derive(Parser)]
struct Cli {
    #[clap(subcommand)]
    command: Command,
}

fn main() {
    let args = Cli::parse();
    match args.command {
        Command::Clean(args) => gpx_clean::cli::run_cli_with_args(args),
        Command::ReducePoints(args) => gpx_reduce_points::cli::run_cli_with_args(args),
        Command::GpxToKml(args) => gpx_to_kml::cli::run_cli_with_args(args),
        Command::Merge(args) => gpx_merge::cli::run_cli_with_args(args),
    }
}
