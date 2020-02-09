use protobuf::{parse_from_bytes};

use super::protos::vector_tile::{*};

pub fn decode(bytes: &[u8]) {
    println!("Hello! {}", bytes.len());
}