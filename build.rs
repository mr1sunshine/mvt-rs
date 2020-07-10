fn main() {
    protoc_rust::Codegen::new()
        .out_dir("src/protos")
        .inputs(&["src/protos/vector_tile.proto"])
        .include("src/protos")
        .run()
        .expect("protoc");
}