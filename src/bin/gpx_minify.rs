use gpx_tools::gpx_minify;

fn main() -> anyhow::Result<()> {
    gpx_minify::cli::run_cli()
}
