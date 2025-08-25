use std::error::Error;
use gpx_tools::gpx_reverse_tracks;

fn main() -> anyhow::Result<()> {
    gpx_reverse_tracks::cli::run_cli()
}
