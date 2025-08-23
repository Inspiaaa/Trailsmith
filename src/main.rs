use crate::simplifier::SimplificationMethod;
use clap::{Parser, ValueEnum};
use std::fs;
use std::fs::File;
use std::path::PathBuf;
use env_logger::Target;
use log::info;

mod simplifier;

#[derive(ValueEnum, Clone, Debug)]
enum Algorithm {
    /// Ramer-Douglas-Peucker
    RDP,
    /// Visvalingam-Whyatt
    VW,
}

#[derive(Parser)]
struct Cli {
    /// Input GPX file
    input: PathBuf,

    /// Output GPX file path
    #[arg(short = 'o', long = "output")]
    output: PathBuf,

    /// Max point count per track
    #[arg(short = 'n', long = "points")]
    max_points: u32,

    /// Max solver iterations
    #[arg(short = 'i', long = "iterations", default_value = "20")]
    max_iterations: u32,

    /// Simplification algorithm
    #[arg(short = 'a', long = "algorithm", default_value = "rdp")]
    algorithm: Algorithm,

    /// Quiet: Disable logging
    #[arg(short = 'q', long = "quiet")]
    quiet: bool,
}

fn main() {
    let args = Cli::parse();

    let logging_level = if args.quiet {
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

    info!("Processing paths...");

    let input_path = args.input;
    let mut output_path = args.output;

    if output_path.is_dir() {
        output_path = output_path.join(input_path.file_name().unwrap());
    }

    info!("Loading input file...");

    let input_file_contents = fs::read(input_path).expect("Could not read input file.");
    let output_file = File::create(output_path.as_path()).expect("Unable to create output file.");

    info!("Simplifying...");

    let method = match args.algorithm {
        Algorithm::RDP => SimplificationMethod::RamerDouglasPeucker,
        Algorithm::VW => SimplificationMethod::VisvalingamWhyatt,
    };

    simplifier::simplify_all_tracks_in_file(
        input_file_contents.as_slice(),
        output_file,
        args.max_points,
        args.max_iterations,
        method,
    );

    info!("Finished simplification. Wrote output to '{}'.", output_path.display())
}
