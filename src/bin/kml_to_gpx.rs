use gpx_tools::kml_to_gpx;

fn main() -> anyhow::Result<()> {
    kml_to_gpx::cli::run_cli()
}
