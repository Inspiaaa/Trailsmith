use clap::{Parser, Subcommand};
use gpx_tools::{
    gpx_clean, gpx_merge_files, gpx_merge_tracks, gpx_minify, gpx_reduce_points,
    gpx_reverse_tracks, gpx_routes_to_tracks, gpx_split_file, gpx_to_kml,
};
use std::error::Error;

#[derive(Subcommand)]
enum Command {
    /// Fix encoding errors, remove metadata and features, change track names.
    Clean(gpx_clean::cli::Args),

    /// Reduce the number of points in tracks.
    ReducePoints(gpx_reduce_points::cli::Args),

    /// Convert a GPX file to KML format.
    GpxToKml(gpx_to_kml::cli::Args),

    /// Reverse the order of track points in all tracks.
    ReverseTracks(gpx_reverse_tracks::cli::Args),

    /// Convert GPX routes into tracks.
    RoutesToTracks(gpx_routes_to_tracks::cli::Args),

    /// Minify a GPX file by removing whitespace to reduce the file size.
    Minify(gpx_minify::cli::Args),

    /// Merge multiple GPX files into a single file.
    MergeFiles(gpx_merge_files::cli::Args),

    /// Merge all tracks within a GPX file.
    MergeTracks(gpx_merge_tracks::cli::Args),

    /// Split waypoints, tracks, and routes from a GPX file into separate files.
    SplitFile(gpx_split_file::cli::Args),
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
        Command::ReducePoints(args) => gpx_reduce_points::cli::run_cli_with_args(args)?,
        Command::GpxToKml(args) => gpx_to_kml::cli::run_cli_with_args(args)?,
        Command::MergeFiles(args) => gpx_merge_files::cli::run_cli_with_args(args)?,
        Command::ReverseTracks(args) => gpx_reverse_tracks::cli::run_cli_with_args(args)?,
        Command::RoutesToTracks(args) => gpx_routes_to_tracks::cli::run_cli_with_args(args)?,
        Command::MergeTracks(args) => gpx_merge_tracks::cli::run_cli_with_args(args)?,
        Command::Minify(args) => gpx_minify::cli::run_cli_with_args(args)?,
        Command::SplitFile(args) => gpx_split_file::cli::run_cli_with_args(args)?,
    }

    Ok(())
}
