use std::fs;
use std::path::PathBuf;
use clap::{Parser, ValueEnum};
use env_logger::Target;
use log::info;

#[derive(ValueEnum, Clone, Debug, Eq, PartialEq)]
enum EncodingOption {
    Utf8,
    Ascii
}

#[derive(ValueEnum, Clone, Debug, Eq, PartialEq)]
enum ErrorStrategy {
    /// Ignores non-ASCII characters.
    Ignore,
    /// Converts non-ASCII characters to '?'
    Replace
}

#[derive(Parser)]
struct Cli {
    /// Input GPX file
    input: PathBuf,

    /// Output GPX file path
    #[arg(short = 'o', long = "output")]
    output: PathBuf,

    /// Output file encoding
    #[arg(short = 'e', long = "encoding", default_value = "ascii")]
    encoding: EncodingOption,

    /// Strategy for dealing with non-ASCII characters
    #[arg(short = 's', long = "strategy", default_value = "ignore")]
    strategy: ErrorStrategy,

    /// Quiet: Disable logging
    #[arg(short = 'q', long = "quiet")]
    quiet: bool,
}

fn main() {
    let args = Cli::parse();

    let logging_level = if args.quiet {
        log::LevelFilter::Off
    } else {
        log::LevelFilter::Trace
    };

    env_logger::builder()
        .target(Target::Stdout)
        .format_timestamp(None)
        .format_target(false)
        .format_level(false)
        .filter_level(logging_level)
        .init();

    let input_path = args.input;
    let mut output_path = args.output;

    if output_path.is_dir() {
        output_path = output_path.join(input_path.file_name().expect("Input path malformed."));
    }

    info!("Loading input file...");
    let input_file_contents = fs::read(input_path).expect("Could not read input file.");

    info!("Parsing GPX file...");
    let gpx = gpx::read(input_file_contents.as_slice()).expect("Could not parse GPX file.");

    info!("Serializing GPX file...");
    let mut output = Vec::new();
    gpx::write(&gpx, &mut output).expect("Could not write GPX file.");

    if args.encoding == EncodingOption::Ascii {
        info!("Converting to ASCII...");

        match args.strategy {
            ErrorStrategy::Ignore => {
                output.retain(|c| c.is_ascii());
            },
            ErrorStrategy::Replace => {
                for char in output.iter_mut() {
                    if !char.is_ascii() {
                        *char = b'?';
                    }
                }
            }
        }
    }

    info!("Writing output...");
    fs::write(output_path.as_path(), output).expect("Could not write output file.");

    info!("Finished clean. Wrote output to '{}'.", output_path.display())
}
