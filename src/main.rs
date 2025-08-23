use std::fs;
use std::fs::File;
use std::path::PathBuf;
use clap::Parser;
use crate::convert::LineStyleConfig;

mod convert;

// Src for the GPX-->KML code: https://github.com/vilaureu/gpx_kml_convert/tree/master

#[derive(Parser)]
struct Cli {
    /// Input GPX file
    input: PathBuf,

    /// Output KML file / directory path
    #[arg(short = 'o', long = "output")]
    output: Option<PathBuf>,

    /// Quite: Disable logging
    #[arg(short = 'q', long = "quiet")]
    quiet: bool,

    /// Line color.
    /// Remember to include the alpha value at the end, e.g. "#ff0000ff" for red.
    #[arg(short = 'c', long = "color", default_value = "#FF4136FF")]
    line_color: String,

    /// Line width
    #[arg(short = 'w', long = "width", default_value = "1.0")]
    line_width: f64
}

fn main() {
    let args = Cli::parse();

    if !args.quiet { println!("Processing paths..."); }

    let input_path = args.input;
    let mut output_path = args.output.unwrap_or_else(|| {
        input_path.with_extension("kml")
    });

    if output_path.is_dir() {
        output_path = output_path.join(input_path.file_name().unwrap()).with_extension("kml");
    }

    if !args.quiet { println!("Loading input file..."); }

    let input_file_contents = fs::read(input_path).expect("Could not read input file.");
    let output_file = File::create(output_path.as_path()).expect("Unable to create output file.");

    if !args.quiet { println!("Converting..."); }
    
    let line_style = LineStyleConfig {
        color: args.line_color,
        width: args.line_width,
    };

    convert::convert(input_file_contents.as_slice(), output_file, &line_style).unwrap();

    if !args.quiet {
        println!("Finished conversion. Wrote output to '{}'.", output_path.display())
    }
}
