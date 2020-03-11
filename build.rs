extern crate protoc_rust;

use protoc_rust::Customize;
use git2::Repository;
use std::path::Path;

fn main() {
    if !Path::new("fixtures").exists() {
        let url = "https://github.com/mapbox/mvt-fixtures";
        match Repository::clone(url, "fixtures") {
            Err(e) => panic!("failed to clone: {}", e),
            _ => ()
        };
    }

    protoc_rust::run(protoc_rust::Args {
        out_dir: "src/protos",
        input: &["src/protos/vector_tile.proto"],
        includes: &["src/protos"],
        customize: Customize {
            lite_runtime: Some(true),
            serde_derive: Some(true),
            ..Default::default()
        },
    })
    .expect("protoc");
}