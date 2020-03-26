use glob::glob;
use mvt::{decode, Tile, FeatureWithJson};

use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;

use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
struct Validity {
    v1: bool,
    v2: bool,
    #[serde(default)]
    error: String
}

#[derive(Debug, Serialize, Deserialize)]
struct Info {
    description: String,
    specification_reference: String,
    validity: Validity,
    proto: String
}

#[test]
fn decode_fixtures() {
    let tiles = glob("./fixtures/fixtures/**/tile.mvt").unwrap();

    for tile in tiles {
        let path = tile.unwrap();
        println!("{:?}", path);

        // Test cases which we cannot process due to different reasons
        if path == PathBuf::from("fixtures/fixtures/014/tile.mvt") ||
           path == PathBuf::from("fixtures/fixtures/023/tile.mvt") ||
           path == PathBuf::from("fixtures/fixtures/024/tile.mvt") ||
           path == PathBuf::from("fixtures/fixtures/030/tile.mvt") ||
           path == PathBuf::from("fixtures/fixtures/061/tile.mvt") {
            continue;
        }

        // Parse test info
        let json_info_path = path.parent().unwrap().join("info.json");
        let mut f_info = File::open(&json_info_path).unwrap();
        let mut info_str = String::new();
        f_info.read_to_string(&mut info_str).unwrap();
        let i: Info = serde_json::from_str(&info_str).unwrap();

        // Parse protobuf
        let mut f_tile = File::open(&path).unwrap();
        let mut buffer = Vec::new();
        f_tile.read_to_end(&mut buffer).unwrap();
        let tile = decode::<FeatureWithJson>(&buffer);
        let tile = match tile {
            Ok(t) => t,
            Err(error) => {
                assert!(!i.validity.v1 || !i.validity.v2, error);
                continue;
            },
        };

        // Parse json reference
        let json_tile_path = path.parent().unwrap().join("tile.json");
        let mut f_json = File::open(&json_tile_path).unwrap();
        let mut json = String::new();
        f_json.read_to_string(&mut json).unwrap();
        let reference = serde_json::from_str::<Tile<FeatureWithJson>>(&json);
        match reference {
            Ok(r) => {
                assert_eq!(r, tile);
            },
            Err(error) => {
                assert!(!i.validity.v1 || !i.validity.v2, error);
            },
        };
    }
}