use clap::ValueEnum;
use gpx::{Gpx, Track, TrackSegment};

#[derive(ValueEnum, Clone, Copy, Eq, PartialEq, Debug)]
pub enum RenameStrategy {
    /// Keep original track names unchanged.
    No,
    /// Add "Rev " at the beginning of each track name.
    Prefix,
    /// Add "Reversed " at the beginning of each track name.
    LongPrefix,
    /// Add " Rev" at the end of each track name.
    Suffix,
    /// Add " Reversed" at the end of each track name.
    LongSuffix,
}

pub fn reverse_all_tracks(gpx: &mut Gpx, rename_strategy: RenameStrategy, keep_original: bool) {
    let mut original_tracks = Vec::new();

    if keep_original {
        original_tracks.extend(gpx.tracks.iter().cloned());
    }

    for track in gpx.tracks.iter_mut() {
        reverse_track(track, rename_strategy);
    }

    gpx.tracks.extend(original_tracks);
}

pub fn reverse_track(track: &mut Track, rename_strategy: RenameStrategy) {
    track.segments.reverse();

    for segment in track.segments.iter_mut() {
        reverse_segment(segment);
    }

    track.name = match rename_strategy {
        RenameStrategy::No => track.name.clone(),
        RenameStrategy::Prefix => Some(
            track
                .name
                .as_ref()
                .map_or_else(|| "Rev".to_string(), |n| "Rev ".to_string() + n),
        ),
        RenameStrategy::LongPrefix => Some(
            track
                .name
                .as_ref()
                .map_or_else(|| "Reversed".to_string(), |n| "Reversed ".to_string() + n),
        ),
        RenameStrategy::Suffix => Some(
            track
                .name
                .as_ref()
                .map_or_else(|| "Rev".to_string(), |n| n.to_string() + " Rev"),
        ),
        RenameStrategy::LongSuffix => Some(
            track
                .name
                .as_ref()
                .map_or_else(|| "Reversed".to_string(), |n| n.to_string() + " Reversed"),
        )
    }
}

pub fn reverse_segment(segment: &mut TrackSegment) {
    segment.points.reverse()
}
