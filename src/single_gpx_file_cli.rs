use crate::{error_messages, util};
use anyhow::Context;
use gpx::Gpx;
use log::info;
use std::fs;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::PathBuf;

pub fn read_and_write_gpx_file<F>(
    input_path: PathBuf,
    output_path: PathBuf,
    process: F,
) -> anyhow::Result<()>
where
    F: FnOnce(&mut Gpx) -> anyhow::Result<()>,
{
    let output_path = util::process_output_path(output_path, &input_path)?;

    info!("Loading input file...");
    let input_file_contents =
        fs::read(input_path).with_context(|| error_messages::INPUT_FILE_READ_ERROR)?;

    info!("Parsing GPX file...");
    let mut gpx = gpx::read(input_file_contents.as_slice())
        .with_context(|| error_messages::GPX_PARSE_ERROR)?;

    process(&mut gpx)?;

    info!("Writing output to {}...", output_path.display());
    let output_file = File::create(output_path.as_path())
        .with_context(|| error_messages::OUTPUT_FILE_CREATION_ERROR)?;
    let mut output_writer = BufWriter::new(output_file);

    gpx::write(&gpx, &mut output_writer).with_context(|| error_messages::GPX_SERIALIZE_ERROR)?;

    output_writer
        .flush()
        .with_context(|| error_messages::OUTPUT_FILE_WRITE_ERROR)?;

    Ok(())
}
