use gpx::{Gpx, Route, Track, TrackSegment};

pub fn convert_all_routes_to_tracks(gpx: &mut Gpx) {
    for route in &gpx.routes {
        gpx.tracks.push(convert_route_to_track(route));
    }
}

pub fn convert_route_to_track(route: &Route) -> Track {
    Track {
        name: route.name.clone(),
        comment: route.comment.clone(),
        description: route.description.clone(),
        source: route.source.clone(),
        links: route.links.clone(),
        type_: route.type_.clone(),
        number: route.number,
        segments: vec![TrackSegment{
            points: route.points.clone(),
        }]
    }
}