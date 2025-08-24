use std::fs;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::PathBuf;
use clap::Parser;
use log::info;
use super::merger;
use crate::util;

#[derive(Parser, Debug)]
pub struct Args {
    /// The base GPX file to merge others into (retains metadata).
    master: PathBuf,

    /// Additional GPX files to be merged into the base file.
    files: Vec<PathBuf>,

    /// Path to save the merged GPX output.
    #[arg(short = 'o', long = "output")]
    output: PathBuf,

    /// Quiet: Disable logging.
    #[arg(short = 'q', long = "quiet")]
    quiet: bool,

    /// Don't merge routes.
    #[arg(long = "no-routes")]
    no_routes: bool,

    /// Don't merge tracks.
    #[arg(long = "no-tracks")]
    no_tracks: bool,

    /// Don't merge waypoints.
    #[arg(long = "no-waypoints")]
    no_waypoints: bool,
}

pub fn run_cli() {
    let args = Args::parse();
    run_cli_with_args(args);
}

pub fn run_cli_with_args(args: Args) {
    util::setup_logging(args.quiet);

    let master_path = args.master;
    let mut output_path = args.output;

    if output_path.is_dir() {
        output_path = output_path.join(master_path.file_name().expect("Input path malformed."));
    }

    info!("Loading input file...");

    let input_file_contents = fs::read(master_path).expect("Could not read input file.");
    let output_file = File::create(output_path.as_path()).expect("Unable to create output file.");
    let mut output_writer = BufWriter::new(output_file);

    info!("Merging files...");
    let mut master = gpx::read(input_file_contents.as_slice()).expect("Could not parse master GPX file.");

    for other_path in args.files {
        info!("  {}", other_path.display());
        let other_file_contents = fs::read(other_path).expect("Could not read file.");
        let other_gpx = gpx::read(other_file_contents.as_slice()).expect("Could not parse GPX file.");

        if !args.no_tracks {
            merger::merge_tracks(&mut master, &other_gpx);
        }
        if !args.no_routes {
            merger::merge_routes(&mut master, &other_gpx);
        }
        if !args.no_waypoints {
            merger::merge_waypoints(&mut master, &other_gpx);
        }
    }

    info!("Writing output...");
    gpx::write(&master, &mut output_writer).expect("Failed to write GPX file.");
    output_writer.flush().expect("Error writing to output file");

    info!("Finished merge. Wrote output to '{}'.", output_path.display())
}
