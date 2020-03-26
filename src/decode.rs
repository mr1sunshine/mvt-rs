use protobuf::{parse_from_bytes};

use crate::protos::vector_tile::Tile as ProtoTile;
use super::tile::Tile;
use super::feature::Feature;

pub fn decode<T>(bytes: &[u8]) -> Result<Tile<T>, protobuf::ProtobufError>
where T: Feature + std::default::Default {
    let tile = parse_from_bytes::<ProtoTile>(&bytes)?;

    Ok(Tile::new(&tile))
}
