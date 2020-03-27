extern crate protoc_rust;

use protoc_rust::Customize;

fn main() {
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