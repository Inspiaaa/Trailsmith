use gpx_tools::gpx_merge_tracks;

fn main() -> anyhow::Result<()> {
    gpx_merge_tracks::cli::run_cli()
}
