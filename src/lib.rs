pub mod gpx_clean {
    pub mod cleaner;
    pub mod cli;
}

pub mod gpx_reduce_points {
    pub mod cli;
    pub mod simplifier;
}

pub mod gpx_to_kml {
    pub mod cli;
    pub mod converter;
}

pub mod gpx_merge_files {
    pub mod cli;
    pub mod merger;
}

pub mod gpx_reverse_tracks {
    pub mod cli;
    pub mod reverser;
}

pub mod gpx_routes_to_tracks {
    pub mod cli;
    pub mod converter;
}

pub mod util;

pub mod error_messages;
mod single_gpx_file_cli;
