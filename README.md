# CLI GPX Tools

## Reducing Track Point Count

**Example:** Reduce the point count of each track to 500 points:

```
mkdir simplified
./gpx_reduce_points.exe my_gpx_file.gpx -o ./simplified -n 500
```

You can specify either an output folder or an exact path (including filename) for the output file.

**Help:**

```
Usage: gpx_reduce_points.exe [OPTIONS] --output <OUTPUT> --points <MAX_POINTS> <INPUT>

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

## Converting GPX --> KML

**Example:**

```
./gpx_to_kml.exe my_gpx_file.gpx
```

You can specify either an output folder or an exact path (including filename) for the output file.

**Help:**

```
Usage: gpx_to_kml.exe [OPTIONS] <INPUT>

Arguments:
  <INPUT>  Input GPX file

Options:
  -o, --output <OUTPUT>     Output KML file / directory path
  -q, --quiet               Quite: Disable logging
  -c, --color <LINE_COLOR>  Line color. Remember to include the alpha value at the end [default: #FF4136FF]
  -w, --width <LINE_WIDTH>  Line width [default: 1.0]
  -h, --help                Print help
```

## Cleaning GPX Files

**Features:**
- Remove non-ASCII chars
- Remove metadata and unwanted fields
- Easily rename tracks

**Example:** Convert to ASCII, but keep all information:

```
./gpx_clean.exe my_gpx_file.gpx -o clean.gpx
```

**Example:** Removing all metadata:

```
./gpx_clean.exe my_gpx_file.gpx -o clean.gpx --set-creator Inspiaaa --set-version 1.1 --remove-metadata --remove-track-metadata --remove-track-point-metadata --remove-route-metadata --remove-route-point-metadata
```

**Example:** Removing routes and waypoints:

```
./gpx_clean.exe my_gpx_file.gpx -o clean.gpx --remove-routes --remove-waypoints
```

**Example:** Renaming all tracks: Interactive, asks you for a new name for each track:

```
./gpx_clean.exe my_gpx_file.gpx -o clean.gpx --rename-tracks
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
Usage: gpx_clean.exe [OPTIONS] --output <OUTPUT> <INPUT>

Arguments:
  <INPUT>
          Input GPX file

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
          Sets the "creator" field (software / person who made the GPX file)

      --set-version <SET_VERSION>
          Sets the GPX file format version

          Possible values:
          - 1.1: Version 1.1
          - 1.0: Version 1.0

          [default: 1.1]

      --rename-tracks
          Interactively rename each track

      --remove-waypoints
          Removes all waypoints

      --remove-tracks
          Removes all tracks

      --remove-routes
          Removes all routes

      --remove-metadata
          Removes all "general" GPX metadata

      --remove-track-metadata
          Removes all general metadata (except for the name) from each track

      --remove-route-metadata
          Remove all general metadata (except for the name) from each route

      --remove-track-point-metadata
          Remove the metadata from each point of each track, only keeping lon, lat, and elevation

      --remove-route-point-metadata
          Remove the metadata from each point of each route, only keeping lon, lat, and elevation

  -h, --help
          Print help (see a summary with '-h')
```