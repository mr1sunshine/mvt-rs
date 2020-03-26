use serde::{Serialize, Deserialize};
use serde_repr::*;

use crate::protos::vector_tile::Tile_GeomType;

#[derive(Debug, Serialize_repr, Deserialize_repr, PartialEq, Clone, Copy)]
#[repr(u8)]
pub enum GeometryType {
    UNKNOWN = 0,
    POINT = 1,
    LINESTRING = 2,
    POLYGON = 3
}

impl GeometryType {
    pub fn new(gtype: Tile_GeomType) -> Self {
        decode_geom_type(gtype)
    }
}

impl Default for GeometryType {
    fn default() -> Self { GeometryType::UNKNOWN }
}

fn decode_geom_type(r#type: Tile_GeomType) -> GeometryType {
    match r#type {
        Tile_GeomType::UNKNOWN => GeometryType::UNKNOWN,
        Tile_GeomType::POINT => GeometryType::POINT,
        Tile_GeomType::LINESTRING => GeometryType::LINESTRING,
        Tile_GeomType::POLYGON => GeometryType::POLYGON,
    }
}