use std::path::{Path, PathBuf};
use anyhow::Context;
use env_logger::Target;
use log::LevelFilter;
use crate::error_messages;

pub fn setup_logging(quiet: bool) {
    let logging_level = if quiet {
        LevelFilter::Off
    } else {
        LevelFilter::Trace
    };

    env_logger::builder()
        .target(Target::Stdout)
        .format_timestamp(None)
        .format_target(false)
        .format_level(false)
        .filter_level(logging_level)
        .init();
}

/// If the output path is a directory, appends the input file name to it to construct the full
/// output file path. Otherwise, returns the output path as-is.
pub fn process_output_path(output_path: PathBuf, input_path: &Path) -> Result<PathBuf, anyhow::Error> {
    if output_path.is_dir() {
        let file_name = input_path.file_name()
            .with_context(|| error_messages::INPUT_PATH_MISSING_FILE_NAME)?;
        Ok(output_path.join(file_name))
    } else {
        Ok(output_path)
    }
}
