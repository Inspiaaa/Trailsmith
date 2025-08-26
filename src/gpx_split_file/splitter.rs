use crate::gpx_cli_util;
use clap::ValueEnum;
use gpx::{Gpx, GpxVersion, Route, Track, Waypoint};
use log::info;
use std::path::{Path, PathBuf};

#[derive(Debug, Copy, Clone, PartialEq, Eq, ValueEnum)]
pub enum NamingStyle {
    /// E.g. "File Track 1.gpx"
    Spaces,
    /// E.g. "File_track_1.gpx"
    SnakeCase,
    /// E.g. "FileTrack1.gpx"
    CamelCase,
}

pub fn split_gpx_file_automatically(
    gpx: &Gpx,
    base_file_name: &str,
    output_folder: &Path,
    keep_metadata: bool,
    naming_style: NamingStyle,
) -> anyhow::Result<()> {
    let metadata_source = if keep_metadata { Some(gpx) } else { None };

    if gpx.waypoints.len() > 0 {
        let waypoints_file =
            get_auto_path_for_waypoints(base_file_name, output_folder, naming_style);
        info!("Saving waypoints to '{}'...", waypoints_file.display());
        save_waypoints_to_file(&gpx.waypoints, &waypoints_file, metadata_source)?;
    }

    for (i, track) in gpx.tracks.iter().enumerate() {
        let track_number = i + 1;

        let track_file =
            get_auto_path_for_track(track_number, base_file_name, output_folder, naming_style);

        let track_name = track
            .name
            .clone()
            .unwrap_or_else(|| track_number.to_string());
        info!(
            "Saving track '{track_name}' to '{}'...",
            track_file.display()
        );
        save_track_to_file(track, &track_file, metadata_source)?;
    }

    for (i, route) in gpx.routes.iter().enumerate() {
        let route_number = i + 1;

        let route_file =
            get_auto_path_for_route(route_number, base_file_name, output_folder, naming_style);

        let route_name = route
            .name
            .clone()
            .unwrap_or_else(|| route_number.to_string());
        info!(
            "Saving route '{route_name}' to '{}'...",
            route_file.display()
        );
        save_route_to_file(route, &route_file, metadata_source)?;
    }

    Ok(())
}

fn get_auto_path_for_track(
    track_number: usize,
    base_file_name: &str,
    output_folder: &Path,
    naming_style: NamingStyle,
) -> PathBuf {
    let file_name = match naming_style {
        NamingStyle::Spaces => format!("{base_file_name} Track {track_number}.gpx"),
        NamingStyle::SnakeCase => format!("{base_file_name}_track_{track_number}.gpx"),
        NamingStyle::CamelCase => format!("{base_file_name}Track{track_number}.gpx"),
    };
    output_folder.join(file_name)
}

fn get_auto_path_for_route(
    route_number: usize,
    base_file_name: &str,
    output_folder: &Path,
    naming_style: NamingStyle,
) -> PathBuf {
    let file_name = match naming_style {
        NamingStyle::Spaces => format!("{base_file_name} Route {route_number}.gpx"),
        NamingStyle::SnakeCase => format!("{base_file_name}_route_{route_number}.gpx"),
        NamingStyle::CamelCase => format!("{base_file_name}Route{route_number}.gpx"),
    };
    output_folder.join(file_name)
}

fn get_auto_path_for_waypoints(
    base_file_name: &str,
    output_folder: &Path,
    naming_style: NamingStyle,
) -> PathBuf {
    let file_name = match naming_style {
        NamingStyle::Spaces => format!("{base_file_name} Waypoints.gpx"),
        NamingStyle::SnakeCase => format!("{base_file_name}_waypoints.gpx"),
        NamingStyle::CamelCase => format!("{base_file_name}Waypoints.gpx"),
    };
    output_folder.join(file_name)
}

pub fn save_track_to_file(
    track: &Track,
    output_path: &Path,
    metadata_source: Option<&Gpx>,
) -> anyhow::Result<()> {
    let mut track_gpx = new_gpx_file_with_optional_metadata(metadata_source);
    track_gpx.tracks.push(track.clone());
    gpx_cli_util::write_gpx_file(&track_gpx, output_path)
}

pub fn save_route_to_file(
    route: &Route,
    output_path: &Path,
    metadata_source: Option<&Gpx>,
) -> anyhow::Result<()> {
    let mut route_gpx = new_gpx_file_with_optional_metadata(metadata_source);
    route_gpx.routes.push(route.clone());
    gpx_cli_util::write_gpx_file(&route_gpx, output_path)
}

pub fn save_waypoints_to_file(
    waypoints: &Vec<Waypoint>,
    output_path: &Path,
    metadata_source: Option<&Gpx>,
) -> anyhow::Result<()> {
    let mut waypoints_gpx = new_gpx_file_with_optional_metadata(metadata_source);
    waypoints_gpx.waypoints.extend(waypoints.clone());
    gpx_cli_util::write_gpx_file(&waypoints_gpx, output_path)
}

fn new_gpx_file_with_optional_metadata(metadata_source: Option<&Gpx>) -> Gpx {
    let mut gpx = Gpx::default();
    gpx.version = GpxVersion::Gpx11;
    copy_optional_metadata(&mut gpx, metadata_source);
    gpx
}

fn copy_optional_metadata(target: &mut Gpx, source: Option<&Gpx>) {
    if let Some(source) = source {
        copy_metadata(target, source);
    }
}

fn copy_metadata(target: &mut Gpx, source: &Gpx) {
    target.metadata = source.metadata.clone();
    target.creator = source.creator.clone();
}
