use geo;
use geo::{LineString, Point, SimplifyIdx, SimplifyVwIdx};
use log::info;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum SimplificationMethod {
    RamerDouglasPeucker,
    VisvalingamWhyatt,
}

#[derive(Clone, Debug)]
pub struct SolverConfig {
    pub max_points: u32,
    pub max_iterations: u32,
    pub method: SimplificationMethod,
    pub initial_epsilon: f64,
}

pub fn simplify_all_tracks_in_gpx(gpx: &mut gpx::Gpx, solver_config: &SolverConfig) {
    info!("Found {} track(s):", gpx.tracks.len());

    for track in gpx.tracks.iter_mut() {
        *track = simplify_track(track, solver_config);
    }
}

fn simplify_track(track: &gpx::Track, config: &SolverConfig) -> gpx::Track {
    let point_count = count_points_in_track(track);

    let track_name: &str = track.name.as_ref().map_or("", |name| name.as_str());

    if point_count <= config.max_points {
        info!(
            "Track '{track_name}' already has {point_count} <= {} points.",
            config.max_points
        );
        return track.clone();
    }

    info!("Simplifying track '{track_name}' ({point_count} points)...");

    // We perform a binary search to find an optimal epsilon value for simplification.

    // initial_epsilon = middle between min and max = (min + max) / 2
    // => Solve for max_epsilon.
    let mut min_epsilon: f64 = 0.0;
    let mut max_epsilon: f64 = config.initial_epsilon * 2.0 - min_epsilon;

    let mut iteration_count: u32 = 0;

    let segments_as_lines: Vec<LineString> = track
        .segments
        .iter()
        .map(get_line_string_from_segment)
        .collect();

    let mut best_segment_indices: Vec<Vec<usize>>;
    let mut best_point_count: u32;

    // Find a suitable upper bound by increasing the max_epsilon parameter.
    info!("  Finding upper bound for epsilon...");
    loop {
        iteration_count += 1;

        best_segment_indices =
            simplify_segment_lines(&segments_as_lines, max_epsilon, config.method);
        best_point_count = count_points_in_simplified_segment_indices(&best_segment_indices);

        info!("    [{iteration_count}] {best_point_count} points for epsilon={max_epsilon}");

        if best_point_count <= config.max_points || iteration_count >= config.max_iterations {
            break;
        }

        max_epsilon *= 2.0;
    }

    info!("  Finding optimal epsilon...");
    // Perform binary search to find the best epsilon parameter to maximise the point count
    // within the given constraints.
    while iteration_count < config.max_iterations && best_point_count < config.max_points {
        iteration_count += 1;

        let epsilon = (max_epsilon + min_epsilon) / 2.0;

        let simplified_indices = simplify_segment_lines(&segments_as_lines, epsilon, config.method);
        let new_point_count = count_points_in_simplified_segment_indices(&simplified_indices);

        info!("    [{iteration_count}] {new_point_count} points for epsilon={epsilon}.");

        if (best_point_count > config.max_points && new_point_count < best_point_count)
            || (new_point_count > best_point_count && new_point_count <= config.max_points)
        {
            best_segment_indices = simplified_indices;
            best_point_count = new_point_count;
        }

        if new_point_count < config.max_points {
            max_epsilon = epsilon;
        } else {
            min_epsilon = epsilon;
        }
    }

    if best_point_count <= config.max_points {
        info!("  Reduced track to {best_point_count} points.");
    } else {
        info!(
            "  Failed to reduce the point count sufficiently. Consider increasing the number of iterations."
        )
    }

    let segments: Vec<gpx::TrackSegment> = track
        .segments
        .iter()
        .zip(best_segment_indices.iter())
        .map(|(segment, indices)| create_segment_from_indices(segment, indices))
        .collect();

    gpx::Track {
        segments,
        ..track.clone()
    }
}

/// Returns for each segment the indices of the points that were kept.
fn simplify_segment_lines(
    segments_as_lines: &Vec<LineString>,
    epsilon: f64,
    method: SimplificationMethod,
) -> Vec<Vec<usize>> {
    segments_as_lines
        .iter()
        .map(|line| simplify_line_string(line, epsilon, method))
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
