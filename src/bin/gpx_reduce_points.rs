use gpx_tools::gpx_reduce_points;

fn main() -> Result<(), anyhow::Error> {
    gpx_reduce_points::cli::run_cli()
}
