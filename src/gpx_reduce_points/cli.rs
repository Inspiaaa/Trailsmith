use super::simplifier;
use super::simplifier::{SimplificationMethod, SolverConfig};
use crate::{error_messages, util};
use clap::{Parser, ValueEnum};
use log::info;
use std::fs;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::PathBuf;
use anyhow::Context;

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

pub fn run_cli() -> Result<(), anyhow::Error> {
    let args = Args::parse();
    run_cli_with_args(args)
}

pub fn run_cli_with_args(args: Args) -> Result<(), anyhow::Error> {
    util::setup_logging(args.quiet);

    let input_path = args.input;
    let output_path = util::process_output_path(args.output, &input_path)?;

    info!("Loading input file...");
    let input_file_contents = fs::read(input_path)
        .with_context(|| error_messages::INPUT_FILE_READ_ERROR)?;

    info!("Parsing GPX file...");
    let mut gpx = gpx::read(input_file_contents.as_slice())
        .with_context(|| error_messages::GPX_PARSE_ERROR)?;

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
        initial_epsilon,
    };
    
    simplifier::simplify_all_tracks_in_gpx(&mut gpx, &solver_config);

    info!("Writing output to {}...", output_path.display());
    let output_file = File::create(output_path.as_path())
        .with_context(|| error_messages::OUTPUT_FILE_CREATION_ERROR)?;
    let mut output_writer = BufWriter::new(output_file);

    gpx::write(&gpx, &mut output_writer).with_context(|| error_messages::GPX_SERIALIZE_ERROR)?;

    output_writer.flush().with_context(|| error_messages::OUTPUT_FILE_WRITE_ERROR)?;

    Ok(())
}
