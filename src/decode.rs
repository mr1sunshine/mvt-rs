use protobuf::{parse_from_bytes};

use crate::protos::vector_tile::Tile as ProtoTile;
use super::tile::Tile;
use super::feature::{FeatureWithJson, FeatureWithCommands};

pub fn decode_with_json(bytes: &[u8]) -> Result<Tile<FeatureWithJson>, protobuf::ProtobufError> {
    let tile = parse_from_bytes::<ProtoTile>(&bytes)?;

    Ok(Tile::new(&tile))
}

pub fn decode_with_commands(bytes: &[u8]) -> Result<Tile<FeatureWithCommands>, protobuf::ProtobufError> {
    let tile = parse_from_bytes::<ProtoTile>(&bytes)?;

    Ok(Tile::new(&tile))
}