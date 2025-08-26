use gpx::{Gpx, Track};

pub fn merge_tracks(gpx: &mut Gpx, merged_name: String) {
    let mut merged_track = Track {
        name: Some(merged_name),
        ..Default::default()
    };

    gpx.tracks.reverse();
    while let Some(mut track) = gpx.tracks.pop() {
        track.segments.reverse();
        while let Some(segment) = track.segments.pop() {
            merged_track.segments.push(segment);
        }
    }

    gpx.tracks.push(merged_track);
}
