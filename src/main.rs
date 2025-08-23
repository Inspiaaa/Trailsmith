use std::fs;
use std::io::Write;
use std::path::PathBuf;
use clap::{Parser, ValueEnum};
use env_logger::Target;
use gpx::{Gpx, GpxVersion, Waypoint};
use log::info;

#[derive(ValueEnum, Clone, Debug, Eq, PartialEq)]
enum EncodingOption {
    Utf8,
    Ascii
}

#[derive(ValueEnum, Clone, Debug, Eq, PartialEq)]
enum ErrorStrategy {
    /// Ignores non-ASCII characters.
    Ignore,
    /// Converts non-ASCII characters to '?'
    Replace
}

#[derive(ValueEnum, Clone, Debug, Eq, PartialEq)]
enum VersionOption {
    /// Version 1.1
    #[value(name = "1.1")]
    V11,
    /// Version 1.0
    #[value(name = "1.0")]
    V10
}

#[derive(Parser)]
struct Cli {
    /// Input GPX file.
    input: PathBuf,

    /// Output GPX file path.
    #[arg(short = 'o', long = "output")]
    output: PathBuf,

    /// Output file encoding.
    #[arg(short = 'e', long = "encoding", default_value = "ascii")]
    encoding: EncodingOption,

    /// Strategy for dealing with non-ASCII characters.
    #[arg(short = 's', long = "strategy", default_value = "ignore")]
    strategy: ErrorStrategy,

    /// Quiet: Disable logging.
    #[arg(short = 'q', long = "quiet")]
    quiet: bool,

    /// Sets the "creator" field (software / person who made the GPX file).
    #[arg(long = "set-creator")]
    set_creator: Option<String>,

    /// Removes all metadata.
    #[arg(long = "remove-metadata")]
    remove_metadata: bool,

    /// Removes all waypoints.
    #[arg(long = "remove-waypoints")]
    remove_waypoints: bool,

    /// Removes all tracks.
    #[arg(long = "remove-tracks")]
    remove_tracks: bool,

    /// Removes all routes.
    #[arg(long = "remove-routes")]
    remove_routes: bool,

    /// Sets the GPX file format version.
    #[arg(long = "set-version", default_value = "1.1")]
    set_version: VersionOption,

    /// Interactively rename each track.
    #[arg(long = "rename-tracks")]
    rename_tracks: bool,

    /// Removes all metadata (except for the name) from each track.
    #[arg(long = "remove-track-metadata")]
    remove_track_metadata: bool,

    /// Remove metadata (except for the name) from each route.
    #[arg(long = "remove-route-metadata")]
    remove_route_metadata: bool,

    /// Remove the metadata from each point of each track, only keeping lon, lat, and elevation.
    #[arg(long = "remove-track-point-metadata")]
    remove_track_point_metadata: bool,

    /// Remove the metadata from each point of each route, only keeping lon, lat, and elevation.
    #[arg(long = "remove-route-point-metadata")]
    remove_route_point_metadata: bool,
}

fn main() {
    let args = Cli::parse();

    setup_logging(args.quiet);

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

    if args.rename_tracks { rename_tracks(&mut gpx); }

    info!("Serializing GPX file...");
    let mut output = Vec::new();
    gpx::write(&gpx, &mut output).expect("Could not write GPX file.");

    if args.encoding == EncodingOption::Ascii {
        info!("Converting to ASCII...");
        remove_non_ascii_chars(&mut output, args.strategy);
    }

    info!("Writing output...");
    fs::write(output_path.as_path(), output).expect("Could not write output file.");

    info!("Finished clean. Wrote output to '{}'.", output_path.display())
}

fn setup_logging(quiet: bool) {
    let logging_level = if quiet {
        log::LevelFilter::Off
    } else {
        log::LevelFilter::Trace
    };

    env_logger::builder()
        .target(Target::Stdout)
        .format_timestamp(None)
        .format_target(false)
        .format_level(false)
        .filter_level(logging_level)
        .init();
}

fn set_creator(gpx: &mut Gpx, creator: String) {
    gpx.creator = Some(creator);
}

fn remove_metadata(gpx: &mut Gpx) {
    gpx.metadata = None;
}

fn remove_waypoints(gpx: &mut Gpx) {
    gpx.waypoints.clear();
}

fn remove_tracks(gpx: &mut Gpx) {
    gpx.tracks.clear();
}

fn remove_routes(gpx: &mut Gpx) {
    gpx.routes.clear();
}

fn set_version(gpx: &mut Gpx, version: VersionOption) {
    gpx.version = match version {
        VersionOption::V11 => GpxVersion::Gpx11,
        VersionOption::V10 => GpxVersion::Gpx10,
    }
}

fn rename_tracks(gpx: &mut Gpx) {
    let count = gpx.tracks.len();
    println!("Rename tracks:");

    for (i, track) in gpx.tracks.iter_mut().enumerate() {
        let number = i + 1;

        println!("[Track {number} of {count}]:");

        match &track.name {
            Some(name) => {
                println!("  Current name: '{name}'");
            },
            None => {
                println!("  No name available.");
            }
        }

        println!("  Enter new name (or press Enter to keep original):");
        print!("  >> ");
        std::io::stdout().flush().expect("Could not flush stdout.");

        let mut new_name: String = String::new();
        std::io::stdin().read_line(&mut new_name).expect("Could not read from stdin.");
        let new_name = new_name.trim();

        if !new_name.is_empty() {
            track.name = Some(new_name.to_string());
        }
    }
}

fn remove_track_metadata(gpx: &mut Gpx) {
    for track in gpx.tracks.iter_mut() {
        track.comment = None;
        track.description = None;
        track.source = None;
        track.links.clear();
        track.type_ = None;
        track.number = None;
    }
}

fn remove_route_metadata(gpx: &mut Gpx) {
    for route in gpx.routes.iter_mut() {
        route.comment = None;
        route.description = None;
        route.source = None;
        route.links.clear();
        route.type_ = None;
    }
}

fn remove_track_point_metadata(gpx: &mut Gpx) {
    for track in gpx.tracks.iter_mut() {
        for segment in track.segments.iter_mut() {
            for point in segment.points.iter_mut() {
                remove_waypoint_metadata(point);
            }
        }
    }
}

fn remove_route_point_metadata(gpx: &mut Gpx) {
    for route in gpx.routes.iter_mut() {
        for point in route.points.iter_mut() {
            remove_waypoint_metadata(point);
        }
    }
}

fn remove_waypoint_metadata(point: &mut Waypoint) {
    point.speed = None;
    point.time = None;
    point.name = None;
    point.comment = None;
    point.description = None;
    point.source = None;
    point.links.clear();
    point.symbol = None;
    point.type_ = None;
    point.geoidheight = None;
    point.fix = None;
    point.sat = None;
    point.hdop = None;
    point.vdop = None;
    point.pdop = None;
    point.dgps_age = None;
    point.dgpsid = None;
}

fn remove_non_ascii_chars(text: &mut Vec<u8>, strategy: ErrorStrategy) {
    match strategy {
        ErrorStrategy::Ignore => {
            text.retain(|c| c.is_ascii());
        },
        ErrorStrategy::Replace => {
            for char in text.iter_mut() {
                if !char.is_ascii() {
                    *char = b'?';
                }
            }
        }
    }
}