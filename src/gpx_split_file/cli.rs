use super::splitter;
use super::splitter::NamingStyle;
use crate::{error_messages, gpx_cli_util, util};
use anyhow::{Context, anyhow};
use clap::Parser;
use std::path::PathBuf;

#[derive(Parser)]
pub struct Args {
    /// Input GPX file path.
    input: PathBuf,

    /// Output folder to place new GPX files.
    #[arg(short = 'o', long = "output")]
    output_folder: PathBuf,

    /// Quiet: Disable logging.
    #[arg(short = 'q', long = "quiet")]
    quiet: bool,

    /// Copy the general file metadata to each resultant file.
    #[arg(long = "keep-metadata")]
    keep_metadata: bool,

    /// Naming style for automatically generated files.
    #[arg(long = "style", default_value = "spaces")]
    naming_style: NamingStyle,

    /// Base file name for the resulting files. E.g. "<Base Name> Track 1.gpx"
    #[arg(long = "name")]
    base_file_name: Option<String>,
}

pub fn run_cli() -> anyhow::Result<()> {
    let args = Args::parse();
    run_cli_with_args(args)
}

pub fn run_cli_with_args(args: Args) -> anyhow::Result<()> {
    util::setup_logging(args.quiet);

    if !args.output_folder.is_dir() {
        return Err(anyhow!(error_messages::OUTPUT_PATH_IS_NOT_FOLDER));
    }

    let input_path = args.input;

    let base_file_name = match args.base_file_name {
        Some(name) => name,
        None => {
            let file_stem = input_path
                .file_stem()
                .with_context(|| error_messages::INPUT_PATH_MISSING_FILE_NAME)?
                .to_str()
                .ok_or_else(|| anyhow::anyhow!("Filename is not valid UTF-8."))?
                .to_string();
            file_stem
        }
    };

    let gpx = gpx_cli_util::read_input_gpx_file(&input_path)?;

    splitter::split_gpx_file_automatically(
        &gpx,
        &base_file_name,
        &args.output_folder,
        args.keep_metadata,
        args.naming_style,
    )
}
