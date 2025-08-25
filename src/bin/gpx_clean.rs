use gpx_tools::gpx_clean;

fn main() -> anyhow::Result<()> {
    gpx_clean::cli::run_cli()
}
