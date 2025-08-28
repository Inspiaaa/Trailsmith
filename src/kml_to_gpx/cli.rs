use super::converter;
use crate::util::process_output_path;
use crate::{error_messages, gpx_cli_util, util};
use anyhow::Context;
use clap::Parser;
use kml::Kml;
use std::fs;
use std::path::PathBuf;
use log::info;

#[derive(Parser)]
pub struct Args {
    /// Input KML file
    input: PathBuf,

    /// Output GPX file / directory path
    #[arg(short = 'o', long = "output")]
    output: Option<PathBuf>,

    /// Quite: Disable logging
    #[arg(short = 'q', long = "quiet")]
    quiet: bool,
}

pub fn run_cli() -> anyhow::Result<()> {
    let args = Args::parse();
    run_cli_with_args(args)
}

pub fn run_cli_with_args(args: Args) -> anyhow::Result<()> {
    util::setup_logging(args.quiet);

    let input_path = args.input;

    let output_path = match args.output {
        None => input_path.with_extension("gpx"),
        Some(path) => process_output_path(path, &input_path)?.with_extension("gpx"),
    };

    info!("Loading input file...");
    let file_contents =
        fs::read_to_string(&input_path).with_context(|| error_messages::INPUT_FILE_READ_ERROR)?;

    info!("Parsing KML file...");
    let kml: Kml = file_contents
        .parse()
        .with_context(|| error_messages::KML_PARSE_ERROR)?;

    info!("Converting to GPX...");
    let gpx = converter::convert(&kml);

    info!("Writing output to {}...", output_path.display());
    gpx_cli_util::write_gpx_file(&gpx, &output_path)
}
