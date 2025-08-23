use geo;
use geo::{LineString, Point, SimplifyIdx, SimplifyVwIdx};
use std::io;
use log::info;

#[derive(Copy, Clone)]
pub enum SimplificationMethod {
    RamerDouglasPeucker,
    VisvalingamWhyatt,
}

pub fn simplify_all_tracks_in_file(
    input_file: impl io::Read,
    output_file: impl io::Write,
    max_points: u32,
    max_iterations: u32,
    method: SimplificationMethod,
) {
    info!("Parsing GPX...");
    let mut gpx = gpx::read(input_file).unwrap();
    simplify_all_tracks_in_gpx_mut(&mut gpx, max_points, max_iterations, method);
    gpx::write(&gpx, output_file).unwrap();
}

pub fn simplify_all_tracks_in_gpx(
    gpx: gpx::Gpx,
    max_points: u32,
    max_iterations: u32,
    method: SimplificationMethod,
) -> gpx::Gpx {
    let mut result = gpx.clone();

    simplify_all_tracks_in_gpx_mut(&mut result, max_points, max_iterations, method);

    result
}

pub fn simplify_all_tracks_in_gpx_mut(
    gpx: &mut gpx::Gpx,
    max_points: u32,
    max_iterations: u32,
    method: SimplificationMethod,
) {
    info!("Found {} track(s)", gpx.tracks.len());

    for track in gpx.tracks.iter_mut() {
        *track = simplify_track(track, max_points, max_iterations, method);
    }
}

fn simplify_track(
    track: &gpx::Track,
    max_points: u32,
    max_iterations: u32,
    method: SimplificationMethod,
) -> gpx::Track {
    let point_count = count_points_in_track(track);

    if point_count <= max_points {
        info!("  Track already has {point_count} <= {max_points} points.");
        return track.clone();
    }

    info!("  Simplifying track...");

    let mut min_epsilon: f64 = 0.0;
    let mut max_epsilon: f64 = 0.01;

    let mut iteration_count: u32 = 0;

    // Binary search to find the epsilon that maximises the number of points.
    loop {
        iteration_count += 1;

        let epsilon = (max_epsilon + min_epsilon) / 2.0;

        let simplified_indices = simplify_segments(&track, epsilon, method);
        let new_point_count = count_points_in_simplified_segment_indices(&simplified_indices);

        info!("    [{iteration_count}] {new_point_count} points for epsilon={epsilon}");

        // TODO: Handle the max iterations case differently (return best value, not the current
        // value which may be too large!)
        // TODO: Add option for min and max epsilon
        if new_point_count == max_points || iteration_count >= max_iterations {
            let segments: Vec<gpx::TrackSegment> = track
                .segments
                .iter()
                .zip(simplified_indices.iter())
                .map(|(segment, indices)| create_segment_from_indices(segment, indices))
                .collect();

            return gpx::Track {
                segments,
                ..track.clone()
            };
        }

        if new_point_count < max_points {
            max_epsilon = epsilon;
        } else {
            min_epsilon = epsilon;
        }
    }
}

/// Returns for each segment the indices of the points that were kept.
fn simplify_segments(
    track: &gpx::Track,
    epsilon: f64,
    method: SimplificationMethod,
) -> Vec<Vec<usize>> {
    track
        .segments
        .iter()
        .map(|segment| {
            let line = get_line_string_from_segment(segment);
            let simplified_indices = simplify_line_string(&line, epsilon, method);
            simplified_indices
        })
        .collect()
}

fn get_line_string_from_segment(segment: &gpx::TrackSegment) -> LineString {
    let xy_points: Vec<Point> = segment
        .points
        .iter()
        .map(|waypoint| waypoint.point())
        .collect();

    LineString::from(xy_points)
}

/// Returns the indices of the original points.
fn simplify_line_string(
    line: &LineString,
    epsilon: f64,
    method: SimplificationMethod,
) -> Vec<usize> {
    match method {
        SimplificationMethod::RamerDouglasPeucker => line.simplify_idx(&epsilon),
        SimplificationMethod::VisvalingamWhyatt => line.simplify_vw_idx(&epsilon),
    }
}

fn count_points_in_simplified_segment_indices(indices_per_segment: &Vec<Vec<usize>>) -> u32 {
    indices_per_segment
        .iter()
        .map(|indices| indices.len() as u32)
        .sum()
}

fn create_segment_from_indices(
    original: &gpx::TrackSegment,
    indices: &Vec<usize>,
) -> gpx::TrackSegment {
    let points = indices
        .iter()
        .map(|i| original.points[*i].clone())
        .collect();
    gpx::TrackSegment { points }
}

fn count_points_in_track(track: &gpx::Track) -> u32 {
    track
        .segments
        .iter()
        .map(|segment| segment.points.len() as u32)
        .sum()
}
