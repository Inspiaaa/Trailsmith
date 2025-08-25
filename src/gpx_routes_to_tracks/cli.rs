use std::path::PathBuf;
use clap::Parser;
use log::info;
use crate::{single_gpx_file_cli, util};
use super::converter;

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

    /// Keep original routes in the output along with converted tracks.
    #[arg(long = "keep-routes")]
    keep_routes: bool,
}

pub fn run_cli() -> anyhow::Result<()> {
    let args = Args::parse();
    run_cli_with_args(args)
}

pub fn run_cli_with_args(args: Args) -> anyhow::Result<()> {
    util::setup_logging(args.quiet);

    single_gpx_file_cli::read_and_write_gpx_file(args.input, args.output, |gpx| {
        info!("Converting routes...");

        converter::convert_all_routes_to_tracks(gpx);

        if !args.keep_routes {
            gpx.routes.clear();
        }

        Ok(())
    })
}