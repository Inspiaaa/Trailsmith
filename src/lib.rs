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
    pub mod convert;
}

pub mod gpx_merge {
    pub mod cli;
    pub mod merger;
}

pub(crate) mod util;
