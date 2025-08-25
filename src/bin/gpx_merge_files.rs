use gpx_tools::gpx_merge_files;

fn main() -> Result<(), anyhow::Error> {
    gpx_merge_files::cli::run_cli()
}