use std::fs;
use std::path::PathBuf;
use anyhow::Context;
use clap::Parser;
use log::info;
use super::merger;
use crate::{error_messages, gpx_cli_util, util};

#[derive(Parser, Debug)]
pub struct Args {
    /// The base GPX file to merge others into (retains metadata).
    master: PathBuf,

    /// Additional GPX files to be merged into the base file.
    files: Vec<PathBuf>,

    /// Path to save the merged GPX output.
    #[arg(short = 'o', long = "output")]
    output: PathBuf,

    /// Quiet: Disable logging.
    #[arg(short = 'q', long = "quiet")]
    quiet: bool,

    /// Don't merge routes.
    #[arg(long = "no-routes")]
    no_routes: bool,

    /// Don't merge tracks.
    #[arg(long = "no-tracks")]
    no_tracks: bool,

    /// Don't merge waypoints.
    #[arg(long = "no-waypoints")]
    no_waypoints: bool,
}

pub fn run_cli() -> Result<(), anyhow::Error> {
    let args = Args::parse();
    run_cli_with_args(args)
}

pub fn run_cli_with_args(args: Args) -> Result<(), anyhow::Error> {
    util::setup_logging(args.quiet);

    gpx_cli_util::read_and_write_gpx_file(args.master, args.output, |master_gpx| {
        info!("Merging files...");

        for other_path in args.files {
            info!("  {}", other_path.display());
            let other_file_contents = fs::read(other_path)
                .with_context(|| error_messages::INPUT_FILE_READ_ERROR)?;
            let other_gpx = gpx::read(other_file_contents.as_slice())
                .with_context(|| error_messages::GPX_PARSE_ERROR)?;

            if !args.no_tracks {
                merger::merge_tracks(master_gpx, &other_gpx);
            }
            if !args.no_routes {
                merger::merge_routes(master_gpx, &other_gpx);
            }
            if !args.no_waypoints {
                merger::merge_waypoints(master_gpx, &other_gpx);
            }
        }

        Ok(())
    })
}
