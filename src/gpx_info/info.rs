use crate::error_messages;
use anyhow::Context;
use geo::{Distance, Haversine};
use gpx::{Gpx, Route, Track, Waypoint};
use std::fs;
use std::path::Path;

pub fn print_gpx_file_info(path: &Path, verbose: bool) -> anyhow::Result<()> {
    let input_file_contents =
        fs::read(path).with_context(|| error_messages::INPUT_FILE_READ_ERROR)?;

    let file_name = path
        .file_name()
        .with_context(|| error_messages::INPUT_PATH_MISSING_FILE_NAME)?;

    println!("File: {}", file_name.display());

    if verbose {
        let size_in_kb = input_file_contents.len() as f64 / 1000.0;
        println!("Size: {:.1} KB", size_in_kb);
    }

    let gpx = gpx::read(input_file_contents.as_slice())
        .with_context(|| error_messages::GPX_PARSE_ERROR)?;

    println!();

    print_gpx_info(&gpx, verbose);

    Ok(())
}

pub fn print_gpx_info(gpx: &Gpx, verbose: bool) {
    print_all_waypoints_info(&gpx.waypoints, verbose);

    print_all_tracks_info(&gpx.tracks, verbose);

    print_all_routes_info(&gpx.routes, verbose);
}

pub fn print_all_tracks_info(tracks: &Vec<Track>, verbose: bool) {
    if tracks.is_empty() {
        return;
    }

    println!("Tracks: {}", tracks.len());

    for track in tracks {
        print_track_info(track, verbose);
    }
}

pub fn print_track_info(track: &Track, verbose: bool) {
    print!("  Track: ");

    if let Some(name) = &track.name {
        println!("'{name}'");
    }

    if verbose && let Some(description) = &track.description {
        println!("  Description:");
        print_intended(description, "    ")
    }

    println!("    Segments: {}", track.segments.len());
    println!("    Points: {}", count_points_in_track(track));

    let distance_in_m = compute_distance_of_track_in_metres(track);
    let distance_in_km = distance_in_m / 1000.0;

    println!("    Distance: {:.2} km", distance_in_km);
    println!();
}

pub fn print_all_waypoints_info(waypoints: &Vec<Waypoint>, verbose: bool) {
    if waypoints.is_empty() {
        return;
    }

    println!("Waypoints: {}", waypoints.len());

    for waypoint in waypoints {
        match &waypoint.name {
            Some(name) => println!("- '{name}'"),
            None => println!("-  no name"),
        }

        if verbose && let Some(description) = &waypoint.description {
            println!("   Description:");
            print_intended(description, "     ");
        }
    }

    println!();
}

pub fn print_all_routes_info(routes: &Vec<Route>, verbose: bool) {
    if routes.is_empty() {
        return;
    }

    println!("Routes: {}", routes.len());
    for route in routes {
        print_route_info(route, verbose);
    }
}

pub fn print_route_info(route: &Route, verbose: bool) {
    print!("  Route: ");

    if let Some(name) = &route.name {
        println!("'{name}'");
    }

    if verbose && let Some(description) = &route.description {
        println!("  Description:");
        print_intended(description, "    ")
    }

    println!("    Points: {}", route.points.len());

    let distance_in_m = compute_distance_of_polyline_in_metres(&route.points);
    let distance_in_km = distance_in_m / 1000.0;

    println!("    Distance: {:.2} km", distance_in_km);
    println!();
}

fn print_intended(text: &str, prefix: &str) {
    for line in text.lines() {
        print!("{prefix}");
        println!("{line}");
    }
}

fn count_points_in_track(track: &Track) -> usize {
    track
        .segments
        .iter()
        .map(|segment| segment.points.len())
        .sum()
}

fn compute_distance_of_polyline_in_metres(points: &Vec<Waypoint>) -> f64 {
    let mut distance: f64 = 0.0;

    for i in 1..points.len() {
        let last_point = &points[i - 1];
        let next_point = &points[i];

        distance += Haversine.distance(last_point.point(), next_point.point());
    }

    distance
}

fn compute_distance_of_track_in_metres(track: &Track) -> f64 {
    track
        .segments
        .iter()
        .map(|segment| compute_distance_of_polyline_in_metres(&segment.points))
        .sum()
}
