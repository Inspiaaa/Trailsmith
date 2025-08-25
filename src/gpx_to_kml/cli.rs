use super::convert;
use super::convert::LineStyleConfig;
use crate::{error_messages, single_gpx_file_cli, util};
use clap::Parser;
use log::info;
use std::fs;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::PathBuf;
use anyhow::Context;
use kml::KmlWriter;
use crate::util::process_output_path;
// Src for the GPX-->KML code: https://github.com/vilaureu/gpx_kml_convert/tree/master

#[derive(Parser)]
pub struct Args {
    /// Input GPX file
    input: PathBuf,

    /// Output KML file / directory path
    #[arg(short = 'o', long = "output")]
    output: Option<PathBuf>,

    /// Quite: Disable logging
    #[arg(short = 'q', long = "quiet")]
    quiet: bool,

    /// Line color. Remember to include the alpha value at the end.
    #[arg(short = 'c', long = "color", default_value = "#FF4136FF")]
    line_color: String,

    /// Line width
    #[arg(short = 'w', long = "width", default_value = "1.0")]
    line_width: f64,
}

pub fn run_cli() -> anyhow::Result<()> {
    let args = Args::parse();
    run_cli_with_args(args)
}

pub fn run_cli_with_args(args: Args) -> anyhow::Result<()> {
    util::setup_logging(args.quiet);

    let input_path = args.input;

    let output_path = match args.output {
        None => input_path.with_extension("kml"),
        Some(path) => process_output_path(path, &input_path)?.with_extension("kml"),
    };

    let gpx = single_gpx_file_cli::read_gpx_file(&input_path)?;

    info!("Converting to KML...");
    let line_style = LineStyleConfig {
        color: args.line_color,
        width: args.line_width,
    };
    let kml = convert::convert(gpx, &line_style);

    info!("Writing output to {}...", output_path.display());
    let output_file = File::create(output_path.as_path())
        .with_context(|| error_messages::OUTPUT_FILE_CREATION_ERROR)?;
    let mut output_writer = BufWriter::new(output_file);

    convert::serialize_kml(&kml, &mut output_writer)
        .with_context(|| error_messages::KML_SERIALIZE_ERROR)?;

    output_writer
        .flush()
        .with_context(|| error_messages::OUTPUT_FILE_WRITE_ERROR)?;

    Ok(())
}
