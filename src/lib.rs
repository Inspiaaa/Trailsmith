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

pub mod gpx_merge_tracks {
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

pub mod gpx_minify {
    pub mod cli;
    pub mod minifier;
}

pub mod gpx_split_file {
    pub mod cli;
    pub mod splitter;
}

pub mod kml_to_gpx {
    pub mod cli;
    pub mod converter;
}

pub mod gpx_info {
    pub mod cli;
    pub mod info;
}

pub mod util;

pub mod error_messages;
mod gpx_cli_util;
