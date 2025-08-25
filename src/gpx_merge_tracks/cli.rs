use std::path::PathBuf;
use clap::Parser;
use log::info;
use crate::{single_gpx_file_cli, util};
use crate::gpx_merge_tracks::merger;

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

    /// Name for the resulting merged track.
    #[arg(short = 'n', long = "name")]
    name: String
}

pub fn run_cli() -> Result<(), anyhow::Error> {
    let args = Args::parse();
    run_cli_with_args(args)
}

pub fn run_cli_with_args(args: Args) -> Result<(), anyhow::Error> {
    util::setup_logging(args.quiet);

    single_gpx_file_cli::read_and_write_gpx_file(args.input, args.output, |gpx| {
        info!("Merging tracks...");
        merger::merge_tracks(gpx, args.name);
        Ok(())
    })
}