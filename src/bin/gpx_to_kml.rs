use gpx_tools::gpx_to_kml;

fn main() -> anyhow::Result<()> {
    gpx_to_kml::cli::run_cli()
}
