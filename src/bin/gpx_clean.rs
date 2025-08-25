use gpx_tools::gpx_clean;

fn main() -> Result<(), anyhow::Error> {
    gpx_clean::cli::run_cli()
}
