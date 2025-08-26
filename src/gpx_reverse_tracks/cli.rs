use std::path::PathBuf;
use clap::Parser;
use log::info;
use super::reverser;
use super::reverser::RenameStrategy;
use crate::{gpx_cli_util, util};

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

pub fn run_cli() -> anyhow::Result<()> {
    let args = Args::parse();
    run_cli_with_args(args)
}

pub fn run_cli_with_args(args: Args) -> anyhow::Result<()> {
    util::setup_logging(args.quiet);

    gpx_cli_util::read_and_write_gpx_file(args.input, args.output, |gpx| {
        info!("Reversing tracks...");
        reverser::reverse_all_tracks(gpx, args.rename_strategy, args.keep_original);
        Ok(())
    })
}