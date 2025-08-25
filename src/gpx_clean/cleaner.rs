use clap::ValueEnum;
use gpx::{Gpx, GpxVersion, Waypoint};
use std::io::Write;

#[derive(ValueEnum, Copy, Clone, Debug, Eq, PartialEq)]
pub enum EncodingOption {
    Utf8,
    Ascii,
}

#[derive(ValueEnum, Copy, Clone, Debug, Eq, PartialEq)]
pub enum AsciiErrorStrategy {
    /// Ignores non-ASCII characters.
    Ignore,
    /// Converts non-ASCII characters to '?'
    Replace,
}

#[derive(ValueEnum, Copy, Clone, Debug, Eq, PartialEq)]
pub enum VersionOption {
    /// Version 1.1
    #[value(name = "1.1")]
    V11,
    /// Version 1.0
    #[value(name = "1.0")]
    V10,
}

pub fn set_creator(gpx: &mut Gpx, creator: String) {
    gpx.creator = Some(creator);
}

pub fn remove_metadata(gpx: &mut Gpx) {
    gpx.metadata = None;
}

pub fn remove_waypoints(gpx: &mut Gpx) {
    gpx.waypoints.clear();
}

pub fn remove_tracks(gpx: &mut Gpx) {
    gpx.tracks.clear();
}

pub fn remove_routes(gpx: &mut Gpx) {
    gpx.routes.clear();
}

pub fn set_version(gpx: &mut Gpx, version: VersionOption) {
    gpx.version = match version {
        VersionOption::V11 => GpxVersion::Gpx11,
        VersionOption::V10 => GpxVersion::Gpx10,
    }
}

pub fn rename_tracks_interactively(gpx: &mut Gpx) {
    let count = gpx.tracks.len();
    println!("Rename tracks:");

    for (i, track) in gpx.tracks.iter_mut().enumerate() {
        let number = i + 1;

        println!("[Track {number} of {count}]:");

        match &track.name {
            Some(name) => {
                println!("  Current name: '{name}'");
            }
            None => {
                println!("  No name available.");
            }
        }

        println!("  Enter new name (or press Enter to keep original):");
        print!("  >> ");
        std::io::stdout().flush().expect("Could not flush stdout.");

        let mut new_name: String = String::new();
        std::io::stdin()
            .read_line(&mut new_name)
            .expect("Could not read from stdin.");
        let new_name = new_name.trim();

        if !new_name.is_empty() {
            track.name = Some(new_name.to_string());
        }
    }
}

pub fn remove_track_metadata(gpx: &mut Gpx) {
    for track in gpx.tracks.iter_mut() {
        track.comment = None;
        track.description = None;
        track.source = None;
        track.links.clear();
        track.type_ = None;
        track.number = None;
    }
}

pub fn remove_route_metadata(gpx: &mut Gpx) {
    for route in gpx.routes.iter_mut() {
        route.comment = None;
        route.description = None;
        route.source = None;
        route.links.clear();
        route.type_ = None;
    }
}

pub fn remove_track_point_metadata(gpx: &mut Gpx) {
    for track in gpx.tracks.iter_mut() {
        for segment in track.segments.iter_mut() {
            for point in segment.points.iter_mut() {
                remove_waypoint_metadata(point);
            }
        }
    }
}

pub fn remove_route_point_metadata(gpx: &mut Gpx) {
    for route in gpx.routes.iter_mut() {
        for point in route.points.iter_mut() {
            remove_waypoint_metadata(point);
        }
    }
}

pub fn remove_waypoint_metadata(point: &mut Waypoint) {
    point.speed = None;
    point.time = None;
    point.name = None;
    point.comment = None;
    point.description = None;
    point.source = None;
    point.links.clear();
    point.symbol = None;
    point.type_ = None;
    point.geoidheight = None;
    point.fix = None;
    point.sat = None;
    point.hdop = None;
    point.vdop = None;
    point.pdop = None;
    point.dgps_age = None;
    point.dgpsid = None;
}

pub fn remove_non_ascii_chars(text: &mut Vec<u8>, strategy: AsciiErrorStrategy) {
    match strategy {
        AsciiErrorStrategy::Ignore => {
            text.retain(|c| c.is_ascii());
        }
        AsciiErrorStrategy::Replace => {
            for char in text.iter_mut() {
                if !char.is_ascii() {
                    *char = b'?';
                }
            }
        }
    }
}
