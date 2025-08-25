use super::simplifier;
use super::simplifier::{SimplificationMethod, SolverConfig};
use crate::{single_gpx_file_cli, util};
use clap::{Parser, ValueEnum};
use log::info;
use std::path::PathBuf;

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

    single_gpx_file_cli::read_and_write_gpx_file(args.input, args.output, |gpx| {
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

        simplifier::simplify_all_tracks_in_gpx(gpx, &solver_config);

        Ok(())
    })
}
