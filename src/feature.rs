use serde::{Serialize, Deserialize};

use crate::protos::vector_tile::Tile_Feature;

use super::geometry_type::GeometryType;
use super::value::Value;

use std::collections::HashMap;

pub trait Feature {
    fn new(feature: &Tile_Feature, keys: &Vec<String>, values: &Vec<Value>) -> Self;
    fn default() -> Self;
}

#[derive(Default, Debug, Serialize, Deserialize, PartialEq)]
pub struct FeatureWithJson {
    #[serde(default)]
    pub id: u64,
    pub tags: Vec<u32>,
    #[serde(default)]
    pub r#type: GeometryType,
    #[serde(default)]
    pub geometry: Vec<u32>
}

impl Feature for FeatureWithJson {
    fn new(feature: &Tile_Feature, _: &Vec<String>, _: &Vec<Value>) -> Self {
        Self {
            id: feature.get_id(),
            tags: feature.get_tags().to_vec(),
            r#type: GeometryType::new(feature.get_field_type()),
            geometry: feature.get_geometry().to_vec()
        }
    }

    fn default() -> Self {
        Default::default()
    }
}

#[derive(Default, Debug)]
pub struct FeatureWithCommands {
    id: u64,
    metadata: HashMap<String, Value>,
    commands: Vec<Command>,
    r#type: GeometryType
}

fn decode_zigzag(input: u32) -> i64 {
    return (input as i64 >> 1) ^ (-(input as i64 & 1));
}

impl Feature for FeatureWithCommands {
    fn new(feature: &Tile_Feature, keys: &Vec<String>, values: &Vec<Value>) -> Self {
        let feature_with_json = FeatureWithJson::new(feature, keys, values);

        let tags = &feature_with_json.tags;
        let mut hm = HashMap::new();
        for i in (0..tags.len()).step_by(2) {
            hm.insert(keys[tags[i] as usize].clone(), values[tags[i + 1] as usize].clone());
        }

        let mut commands = Vec::new();
        let mut i = 0;

        let geometry = &feature_with_json.geometry;
        while i < geometry.len() {
            let command_id = geometry[i] & 0x7;
            let count = geometry[i] >> 3;
            i += 1;
            if command_id == 1 {
                for _ in 0..count {
                    let x = geometry[i];
                    i += 1;
                    let y = geometry[i];
                    i += 1;
                    commands.push(Command::MoveTo(decode_zigzag(x), decode_zigzag(y)));
                }
            } else if command_id == 2 {
                for _ in 0..count {
                    let x = geometry[i];
                    i += 1;
                    let y = geometry[i];
                    i += 1;
                    commands.push(Command::LineTo(decode_zigzag(x), decode_zigzag(y)));
                }
            } else if command_id == 7 {
                commands.push(Command::ClosePath);
            } else {
                assert!(false);
            }
        }

        Self {
            id: feature_with_json.id,
            metadata: hm,
            r#type: feature_with_json.r#type,
            commands: commands
        }
    }

    fn default() -> Self {
        Default::default()
    }
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
    r#type: GeometryType
}