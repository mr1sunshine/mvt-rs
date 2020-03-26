use serde::{Serialize, Deserialize};

use crate::protos::vector_tile::Tile_Feature;

use super::geometry_type::GeometryType;
use super::value::Value;

use std::collections::HashMap;

pub trait Feature {
    fn new(feature: &Tile_Feature) -> Self;
    fn default() -> Self;
}

#[derive(Default, Debug, Serialize, Deserialize, PartialEq)]
pub struct FeatureJson {
    #[serde(default)]
    pub id: u64,
    pub tags: Vec<u32>,
    #[serde(default)]
    pub gtype: GeometryType,
    #[serde(default)]
    pub geometry: Vec<u32>
}

impl Feature for FeatureJson {
    fn new(feature: &Tile_Feature) -> Self {
        Self {
            id: feature.get_id(),
            tags: feature.get_tags().to_vec(),
            gtype: GeometryType::new(feature.get_field_type()),
            geometry: feature.get_geometry().to_vec()
        }
    }

    fn default() -> Self {
        Default::default()
    }
}

#[derive(Debug)]
pub struct FeatureWithCommands {
    id: u64,
    metadata: HashMap<String, Value>,
    commands: Vec<Command>,
    gtype: GeometryType
}

#[derive(Debug, Clone, Copy)]
pub enum Command {
    MoveTo(i64, i64),
    LineTo(i64, i64),
    ClosePath
}

#[derive(Debug)]
pub struct FeatureWithCoordinates {
    id: u64,
    metadata: HashMap<String, Value>,
    geometry: Vec<Vec<[i64; 2]>>,
    gtype: GeometryType
}