use gpx_tools::gpx_merge_files;

fn main() -> anyhow::Result<()> {
    gpx_merge_files::cli::run_cli()
}