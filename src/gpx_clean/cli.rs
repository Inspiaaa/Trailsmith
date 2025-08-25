use super::cleaner::*;
use crate::util;
use clap::Parser;
use log::info;
use std::fs;
use std::path::PathBuf;

#[derive(Parser)]
pub struct Args {
    /// Input GPX file path.
    input: PathBuf,

    /// Output GPX file path.
    #[arg(short = 'o', long = "output")]
    output: PathBuf,

    /// Quiet: Disable logging.
    #[arg(short = 'q', long = "quiet")]
    quiet: bool,

    /// Output file encoding.
    #[arg(short = 'e', long = "encoding", default_value = "ascii")]
    encoding: EncodingOption,

    /// Strategy for dealing with non-ASCII characters.
    #[arg(short = 's', long = "strategy", default_value = "ignore")]
    strategy: AsciiErrorStrategy,

    /// Set the "creator" field (software / person who made the GPX file).
    #[arg(long = "set-creator")]
    set_creator: Option<String>,

    /// Set the GPX file format version.
    #[arg(long = "set-version", default_value = "1.1")]
    set_version: VersionOption,

    /// Interactively rename each track.
    #[arg(long = "rename-tracks")]
    rename_tracks: bool,

    /// Remove all waypoints.
    #[arg(long = "remove-waypoints")]
    remove_waypoints: bool,

    /// Remove all tracks.
    #[arg(long = "remove-tracks")]
    remove_tracks: bool,

    /// Remove all routes.
    #[arg(long = "remove-routes")]
    remove_routes: bool,

    /// Remove all "general" GPX metadata.
    #[arg(long = "remove-metadata")]
    remove_metadata: bool,

    /// Remove all general metadata (except for the name) from each track.
    #[arg(long = "remove-track-metadata")]
    remove_track_metadata: bool,

    /// Remove all general metadata (except for the name) from each route.
    #[arg(long = "remove-route-metadata")]
    remove_route_metadata: bool,

    /// Remove the metadata from each point of each track, only keeping lon, lat, and elevation.
    #[arg(long = "remove-track-point-metadata")]
    remove_track_point_metadata: bool,

    /// Remove the metadata from each point of each route, only keeping lon, lat, and elevation.
    #[arg(long = "remove-route-point-metadata")]
    remove_route_point_metadata: bool,

    /// Remove the elevation data from each track point.
    #[arg(long = "remove-track-elevation")]
    remove_track_elevation: bool,

    /// Remove the elevation data from each route point.
    #[arg(long = "remove-route-elevation")]
    remove_route_elevation: bool,
}

pub fn run_cli() {
    let args = Args::parse();
    run_cli_with_args(args);
}

pub fn run_cli_with_args(args: Args) {
    util::setup_logging(args.quiet);

    let input_path = args.input;
    let mut output_path = args.output;

    if output_path.is_dir() {
        output_path = output_path.join(input_path.file_name().expect("Input path malformed."));
    }

    info!("Loading input file...");
    let input_file_contents = fs::read(input_path).expect("Could not read input file.");

    info!("Parsing GPX file...");
    let mut gpx = gpx::read(input_file_contents.as_slice()).expect("Could not parse GPX file.");

    info!("Processing...");
    set_version(&mut gpx, args.set_version);

    if let Some(creator) = args.set_creator {
        set_creator(&mut gpx, creator);
    }

    if args.remove_metadata { remove_metadata(&mut gpx); }
    if args.remove_waypoints { remove_waypoints(&mut gpx); }
    if args.remove_tracks { remove_tracks(&mut gpx); }
    if args.remove_routes { remove_routes(&mut gpx); }
    if args.remove_track_metadata { remove_track_metadata(&mut gpx); }
    if args.remove_route_metadata { remove_route_metadata(&mut gpx); }
    if args.remove_track_point_metadata { remove_track_point_metadata(&mut gpx); }
    if args.remove_route_point_metadata { remove_route_point_metadata(&mut gpx); }
    if args.remove_track_elevation { remove_track_elevation(&mut gpx); }
    if args.remove_route_elevation { remove_route_elevation(&mut gpx); }

    if args.rename_tracks { rename_tracks_interactively(&mut gpx); }

    info!("Serializing GPX file...");
    let mut output = Vec::new();
    gpx::write(&gpx, &mut output).expect("Could not write GPX file.");

    if args.encoding == EncodingOption::Ascii {
        info!("Converting to ASCII...");
        remove_non_ascii_chars(&mut output, args.strategy);
    }

    info!("Writing output...");
    fs::write(output_path.as_path(), output).expect("Could not write output file.");

    info!(
        "Finished clean. Wrote output to '{}'.",
        output_path.display()
    )
}
