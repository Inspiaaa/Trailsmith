# CLI GPX Tools

Trailsmith is a collection of lightweight, fast, and easy-to-use command-line utilities for processing and transforming GPX files, built with Rust. Whether you're cleaning up GPS data, converting formats, or optimizing tracks, Trailsmith helps you streamline your GPX workflows with minimal effort.

All tools are available through a unified command-line interface via subcommands. Alternatively, you can build and use each tool independently by compiling from source.

## Usage

```
Usage: trailsmith.exe <COMMAND>

Commands:
  clean             Fix encoding errors, remove metadata and features, change track names
  reduce-points     Reduce the number of points in tracks
  gpx-to-kml        Convert a GPX file to KML format
  kml-to-gpx        Convert a KML file to GPX format
  reverse-tracks    Reverse the order of track points in all tracks
  routes-to-tracks  Convert GPX routes into tracks
  minify            Minify a GPX file by removing whitespace to reduce the file size
  merge-files       Merge multiple GPX files into a single file
  merge-tracks      Merge all tracks within a GPX file
  split-file        Split waypoints, tracks, and routes from a GPX file into separate files
  help              Print this message or the help of the given subcommand(s)

Options:
  -h, --help  Print help
```

### Building from Source

To build Trailsmith from source, ensure you have [Rust and Cargo](https://www.rust-lang.org/tools/install) installed.

```bash
cargo build --release
```

This will generate:
- A single CLI binary (`trailsmith`) with all subcommands
- Individual executables for each tool, e.g. `gpx_reduce_points.exe`

## Examples

### Reducing Track Point Count

**Example:** Reduce the point count of each track to 500 points:

```
mkdir simplified
./trailsmith.exe my_gpx_file.gpx -o ./simplified -n 500
```

You can specify either an output folder or an exact path (including filename) for the output file.

**Help:**

```
Reduce the number of points in tracks

Usage: trailsmith.exe reduce-points [OPTIONS] --output <OUTPUT> --points <MAX_POINTS> <INPUT>

Arguments:
  <INPUT>
          Input GPX file

Options:
  -o, --output <OUTPUT>
          Output GPX file path

  -n, --points <MAX_POINTS>
          Max point count per track

  -i, --iterations <MAX_ITERATIONS>
          Max solver iterations

          [default: 20]

  -a, --algorithm <ALGORITHM>
          Simplification algorithm

          Possible values:
          - rdp: Ramer-Douglas-Peucker
          - vw:  Visvalingam-Whyatt

          [default: rdp]

  -e, --epsilon <EPSILON>
          Initial epsilon value for simplification

  -q, --quiet
          Quiet: Disable logging

  -h, --help
          Print help (see a summary with '-h')
```

### Converting GPX --> KML

**Example:**

```
./trailsmith.exe gpx-to-kml my_gpx_file.gpx
```

You can specify either an output folder or an exact path (including filename) for the output file.

**Help:**

```
Convert a GPX file to KML format

Usage: trailsmith.exe gpx-to-kml [OPTIONS] <INPUT>

Arguments:
  <INPUT>  Input GPX file

Options:
  -o, --output <OUTPUT>     Output KML file / directory path
  -q, --quiet               Quite: Disable logging
  -c, --color <LINE_COLOR>  Line color. Remember to include the alpha value at the end [default: #FF4136FF]
  -w, --width <LINE_WIDTH>  Line width [default: 1.0]
  -h, --help                Print help            Print help
```

### Cleaning GPX Files

**Features:**
- Remove non-ASCII chars
- Remove metadata and unwanted fields
- Easily rename tracks

**Example:** Convert to ASCII, but keep all information:

```
./trailsmith.exe clean my_gpx_file.gpx -o clean.gpx
```

**Example:** Removing all metadata:

```
./trailsmith.exe clean my_gpx_file.gpx -o clean.gpx --set-creator "Inspiaaa" --set-version 1.1 --remove-metadata --remove-track-metadata --remove-track-point-metadata --remove-route-metadata --remove-route-point-metadata
```

**Example:** Removing routes and waypoints:

```
./trailsmith.exe clean my_gpx_file.gpx -o clean.gpx --remove-routes --remove-waypoints
```

**Example:** Removing elevation data:

```
./trailsmith.exe clean my_gpx_file.gpx -o clean.gpx --remove-track-elevation --remove-route-elevation
```

**Example:** Renaming all tracks: Interactively asks you for a new name for each track:

```
./trailsmith.exe clean my_gpx_file.gpx -o clean.gpx --rename-tracks
```

Example console output during rename operation:

```
Rename tracks:
[Track 1 of 3]:
  Current name: 'TET-ALBANIA-Section 1-20191214'
  Enter new name (or press Enter to keep original):
  >> Section 1
[Track 2 of 3]:
  Current name: 'TET-ALBANIA-Section 2-20191214'
  Enter new name (or press Enter to keep original):
  >> Section 2
[Track 3 of 3]:
  Current name: 'TET-ALBANIA-Wet detour-20191214'
  Enter new name (or press Enter to keep original):
  >> Section 3
```

**Help:**

```
Fix encoding errors, remove metadata and features, change track names

Usage: trailsmith.exe clean [OPTIONS] --output <OUTPUT> <INPUT>

Arguments:
  <INPUT>
          Input GPX file path

Options:
  -o, --output <OUTPUT>
          Output GPX file path

  -q, --quiet
          Quiet: Disable logging

  -e, --encoding <ENCODING>
          Output file encoding

          [default: ascii]
          [possible values: utf8, ascii]

  -s, --strategy <STRATEGY>
          Strategy for dealing with non-ASCII characters

          Possible values:
          - ignore:  Ignores non-ASCII characters
          - replace: Converts non-ASCII characters to '?'

          [default: ignore]

      --set-creator <SET_CREATOR>
          Set the "creator" field (software / person who made the GPX file)

      --set-version <SET_VERSION>
          Set the GPX file format version

          Possible values:
          - 1.1: Version 1.1
          - 1.0: Version 1.0

          [default: 1.1]

      --rename-tracks
          Interactively rename each track

      --remove-waypoints
          Remove all waypoints

      --remove-tracks
          Remove all tracks

      --remove-routes
          Remove all routes

      --remove-metadata
          Remove all "general" GPX metadata

      --remove-track-metadata
          Remove all general metadata (except for the name) from each track

      --remove-route-metadata
          Remove all general metadata (except for the name) from each route

      --remove-track-point-metadata
          Remove the metadata from each point of each track, only keeping lon, lat, and elevation

      --remove-route-point-metadata
          Remove the metadata from each point of each route, only keeping lon, lat, and elevation

      --remove-track-elevation
          Remove the elevation data from each track point

      --remove-route-elevation
          Remove the elevation data from each route point

  -h, --help
          Print help (see a summary with '-h')
```

### Additional Tools

For more information on the additional tools provided by the CLI suite, please consult the `--help`.
