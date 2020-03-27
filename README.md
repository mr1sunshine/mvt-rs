# mvt-rs

A library for decoding Mapbox vector tiles according to [Mapbox Vector Tile specification](https://github.com/mapbox/vector-tile-spec).

The purpose of this library is to provide ability to get decoded information from mapbox vector tiles in three ways:

- As structure as it is presented in protobuf file. With geometry field contained all encoded CommandIntegers and ParameterIntegers.
- As structure with decoded geometry field contained decoded commands: `MoveTo`, `LineTo`, `ClosePath`.
- As structure with processed command and contained coordinates as output.

Please see examples below for more detailed information.

## Usage

As described above you can use this library in three different as follow:
```rust
    let tile_with_json = decode::<FeatureWithJson>(&buffer).unwrap();
    let tile_with_commands = decode::<FeatureWithCommands>(&buffer).unwrap();
    let tile_with_coordinates = decode::<FeatureWithCoordinates>(&buffer).unwrap();
```
## Tests

For testing purpose we use fixtures available here: [mvt-fixtures](https://github.com/mapbox/mvt-fixtures)

To run test cases:
```bash
cargo test decode_fixtures
```

## Examples

We provide the following decoding examples:

- "As it is" decoding:
```bash
cargo run --example decode_with_json fixtures/fixtures/026/tile.mvt
    Finished dev [unoptimized + debuginfo] target(s) in 0.04s
     Running `target/debug/examples/decode_with_json fixtures/fixtures/026/tile.mvt`
Tile { layers: [Layer { version: 2, name: "howdy", features: [FeatureWithJson { id: 1, tags: [], type: POINT, geometry: [9, 50, 34] }], keys: [], values: [StringValue("")], extent: 4096 }] }
```

- Encoded commands decoding:
```bash
cargo run --example decode_with_commands fixtures/fixtures/026/tile.mvt
    Finished dev [unoptimized + debuginfo] target(s) in 0.04s
     Running `target/debug/examples/decode_with_commands fixtures/fixtures/026/tile.mvt`
Tile { layers: [Layer { version: 2, name: "howdy", features: [FeatureWithCommands { id: 1, metadata: {}, commands: [MoveTo(25, 17)], type: POINT }], keys: [], values: [StringValue("")], extent: 4096 }] }
```

- Decoding with coordinates output:
```bash
cargo run --example decode_with_coordinates fixtures/fixtures/026/tile.mvt
    Finished dev [unoptimized + debuginfo] target(s) in 0.05s
     Running `target/debug/examples/decode_with_coordinates fixtures/fixtures/026/tile.mvt`
Tile { layers: [Layer { version: 2, name: "howdy", features: [FeatureWithCoordinates { id: 1, metadata: {}, geometry: Point([25, 17]) }], keys: [], values: [StringValue("")], extent: 4096 }] }
```
