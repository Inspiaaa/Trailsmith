use gpx::Gpx;

pub fn merge_waypoints(master: &mut Gpx, other: &Gpx) {
    for waypoint in &other.waypoints {
        master.waypoints.push(waypoint.clone());
    }
}

pub fn merge_tracks(master: &mut Gpx, other: &Gpx) {
    for track in &other.tracks {
        master.tracks.push(track.clone());
    }
}

pub fn merge_routes(master: &mut Gpx, other: &Gpx) {
    for route in &other.routes {
        master.routes.push(route.clone());
    }
}
