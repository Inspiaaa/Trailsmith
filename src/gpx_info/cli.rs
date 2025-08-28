use std::path::PathBuf;
use clap::Parser;
use super::info;

#[derive(Parser)]
pub struct Args {
    /// Input GPX file path.
    input: PathBuf,

    /// Display additional information.
    #[arg(short = 'v', long = "verbose")]
    verbose: bool,
}

pub fn run_cli() -> anyhow::Result<()> {
    let args = Args::parse();
    run_cli_with_args(args)
}

pub fn run_cli_with_args(args: Args) -> anyhow::Result<()> {
    info::print_gpx_file_info(&args.input, args.verbose)
}
