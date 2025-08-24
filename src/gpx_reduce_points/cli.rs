use clap::{Parser, ValueEnum};
use std::fs;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::PathBuf;
use log::info;
use super::simplifier;
use super::simplifier::{SimplificationMethod, SolverConfig};
use crate::util;

const DEFAULT_VW_EPSILON: f64 = 0.0001;
const DEFAULT_RDP_EPSILON: f64 = 0.001;

#[derive(ValueEnum, Clone, Copy, Eq, PartialEq, Debug)]
enum AlgorithmOption {
    /// Ramer-Douglas-Peucker
    RDP,
    /// Visvalingam-Whyatt
    VW,
}

#[derive(Parser)]
pub struct Args {
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
    algorithm: AlgorithmOption,

    /// Initial epsilon value for simplification
    #[arg(short = 'e', long = "epsilon")]
    epsilon: Option<f64>,

    /// Quiet: Disable logging
    #[arg(short = 'q', long = "quiet")]
    quiet: bool,
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

    info!("Simplifying...");

    let method = match args.algorithm {
        AlgorithmOption::RDP => SimplificationMethod::RamerDouglasPeucker,
        AlgorithmOption::VW => SimplificationMethod::VisvalingamWhyatt,
    };

    let initial_epsilon = args.epsilon.unwrap_or(match args.algorithm {
        AlgorithmOption::RDP => DEFAULT_RDP_EPSILON,
        AlgorithmOption::VW => DEFAULT_VW_EPSILON,
    });

    let solver_config = SolverConfig {
        max_points: args.max_points,
        max_iterations: args.max_iterations,
        method,
        initial_epsilon
    };

    simplifier::simplify_all_tracks_in_file(
        input_file_contents.as_slice(),
        &mut output_writer,
        &solver_config
    );

    output_writer.flush().expect("Error writing to output file");   

    info!("Finished simplification. Wrote output to '{}'.", output_path.display())
}
