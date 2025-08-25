use std::fs;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::PathBuf;
use clap::Parser;
use log::info;
use super::reverser;
use super::reverser::RenameStrategy;
use crate::util;

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

    /// Strategy for renaming track names in the output (e.g. to mark reversed tracks).
    #[arg(long = "rename", default_value = "prefix")]
    rename_strategy: RenameStrategy,

    /// Keep the original tracks alongside the reversed ones.
    #[arg(long = "keep-original")]
    keep_original: bool,
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
    let output_file = File::create(output_path.as_path()).expect("Unable to create output file.");
    let mut output_writer = BufWriter::new(output_file);

    let mut gpx = gpx::read(input_file_contents.as_slice()).expect("Could not parse GPX file.");

    info!("Reversing tracks...");
    reverser::reverse_all_tracks(&mut gpx, args.rename_strategy, args.keep_original);

    gpx::write(&gpx, &mut output_writer).expect("Failed to write GPX file.");
    output_writer.flush().expect("Error writing to output file");

    info!("Finished reversing tracks. Wrote output to '{}'.", output_path.display())
}