use std::fs;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::PathBuf;
use anyhow::Context;
use clap::Parser;
use log::info;
use super::merger;
use crate::{error_messages, util};

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

pub fn run_cli() -> Result<(), anyhow::Error> {
    let args = Args::parse();
    run_cli_with_args(args)
}

pub fn run_cli_with_args(args: Args) -> Result<(), anyhow::Error> {
    util::setup_logging(args.quiet);

    let master_path = args.master;
    let mut output_path = args.output;

    if output_path.is_dir() {
        let file_name = master_path.file_name()
            .with_context(|| error_messages::INPUT_PATH_MISSING_FILE_NAME)?;
        output_path = output_path.join(file_name);
    }

    info!("Loading input file...");

    let input_file_contents = fs::read(master_path)
        .with_context(|| error_messages::INPUT_FILE_READ_ERROR)?;

    info!("Merging files...");
    let mut master = gpx::read(input_file_contents.as_slice())
        .with_context(|| error_messages::GPX_PARSE_ERROR)?;

    for other_path in args.files {
        info!("  {}", other_path.display());
        let other_file_contents = fs::read(other_path)
            .with_context(|| error_messages::INPUT_FILE_READ_ERROR)?;
        let other_gpx = gpx::read(other_file_contents.as_slice())
            .with_context(|| error_messages::GPX_PARSE_ERROR)?;

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

    info!("Writing output to {}...", output_path.display());
    let output_file = File::create(output_path.as_path())
        .with_context(|| error_messages::OUTPUT_FILE_CREATION_ERROR)?;
    let mut output_writer = BufWriter::new(output_file);

    gpx::write(&master, &mut output_writer).with_context(|| error_messages::GPX_SERIALIZE_ERROR)?;

    output_writer.flush().with_context(|| error_messages::OUTPUT_FILE_WRITE_ERROR)?;

    Ok(())
}
