use gpx_tools::gpx_split_file;

fn main() -> anyhow::Result<()> {
    gpx_split_file::cli::run_cli()
}
