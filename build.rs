extern crate protoc_rust;

use protoc_rust::Customize;

fn main() {
    protoc_rust::run(protoc_rust::Args {
        out_dir: "src/protos",
        input: &["src/protos/vector_tile.proto"],
        includes: &["src/protos"],
        customize: Customize {
            ..Default::default()
        },
    })
    .expect("protoc");
}