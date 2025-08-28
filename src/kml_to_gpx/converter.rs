use geo::{Coord, Point};
use gpx::{Gpx, GpxVersion, Track, TrackSegment, Waypoint};
use kml::types::Placemark;
use kml::Kml;
use log::warn;

use kml::types as KmlTypes;
use kml::types::Geometry as KmlGeometry;

pub fn convert(kml: &Kml) -> Gpx {
    // TODO: Copy metadata
    let mut gpx = Gpx::default();
    gpx.version = GpxVersion::Gpx11;
    convert_kml_element(kml, &mut gpx);
    gpx
}

fn convert_kml_element(kml: &Kml<f64>, gpx: &mut Gpx) {
    match kml {
        Kml::KmlDocument(doc) => {
            for element in &doc.elements {
                convert_kml_element(&element, gpx);
            }
        }
        Kml::Document{ elements, .. } => {
            for entry in elements {
                convert_kml_element(entry, gpx);
            }
        }
        Kml::Folder(folder) => {
            for entry in &folder.elements {
                convert_kml_element(entry, gpx);
            }
        }
        Kml::Placemark(placemark) => {
            convert_placemark(placemark, gpx);
        }
        _ => {
            // TODO: Handle other KML types?
        }
    }
}

fn convert_placemark(placemark: &Placemark<f64>, gpx: &mut Gpx) {
    let Some(geometry) = &placemark.geometry else {
        return;
    };

    // let collection: GeometryCollection<f64> = Kml::Placemark(placemark.clone()).try_into().unwrap();

    let name = placemark.name.clone();
    let description = placemark.description.clone();

    match geometry {
        KmlGeometry::Point(point) => {
            let mut waypoint = make_waypoint(point);
            waypoint.name = name;
            waypoint.description = description;

            gpx.waypoints.push(waypoint);
        }
        geometry => {
            let mut track = Track::default();
            track.name = name;
            track.description = description;

            convert_geometry_for_track(geometry, &mut track);

            gpx.tracks.push(track);
        }
    }
}

fn convert_geometry_for_track(geometry: &KmlGeometry, track: &mut Track) {
    match geometry {
        KmlGeometry::Point(point) => {
            let waypoint = make_waypoint(point);
            track.segments.push(TrackSegment {
                points: vec![waypoint],
            });
        }
        KmlGeometry::LineString(line) => {
            let segment = make_track_segment(line);
            track.segments.push(segment);
        },
        KmlGeometry::MultiGeometry(geometry) => {
            for segment_geometry in &geometry.geometries {
                convert_geometry_for_track(segment_geometry, track);
            }
        }
        _ => {
            warn!("  Unsupported geometry type: {:?}", geometry);
        }
    }
}

fn make_waypoint(point: &KmlTypes::Point) -> Waypoint {
    let point = Point::from(point.clone());
    Waypoint::new(point)
}

fn make_track_segment(line: &KmlTypes::LineString) -> TrackSegment {
    let points: Vec<Point> = line
        .coords
        .iter()
        .map(|c| Point::from(Coord::from(c.clone())))
        .collect();

    let waypoints: Vec<Waypoint> = points
        .iter()
        .map(|point| Waypoint::new(point.clone()))
        .collect();

    TrackSegment {
        points: waypoints,
    }
}