use gpx_tools::gpx_routes_to_tracks;

fn main() -> anyhow::Result<()> {
    gpx_routes_to_tracks::cli::run_cli()
}
