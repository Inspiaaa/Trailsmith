use env_logger::Target;
use log::LevelFilter;

pub fn setup_logging(quiet: bool) {
    let logging_level = if quiet {
        LevelFilter::Off
    } else {
        LevelFilter::Trace
    };

    env_logger::builder()
        .target(Target::Stdout)
        .format_timestamp(None)
        .format_target(false)
        .format_level(false)
        .filter_level(logging_level)
        .init();
}