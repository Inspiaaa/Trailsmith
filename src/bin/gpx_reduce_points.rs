use gpx_tools::gpx_reduce_points;

fn main() -> anyhow::Result<()> {
    gpx_reduce_points::cli::run_cli()
}
