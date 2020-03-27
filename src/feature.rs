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


impl FeatureWithCommands {
    pub fn id(&self) -> u64 {
        self.id
    }

    pub fn metadata(&self) -> &HashMap<String, Value> {
        &self.metadata
    }

    pub fn commands(&self) -> &Vec<Command> {
        &self.commands
    }

    pub fn r#type(&self) -> GeometryType {
        self.r#type
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Command {
    MoveTo(i64, i64),
    LineTo(i64, i64),
    ClosePath
}

#[derive(Default, Debug)]
pub struct FeatureWithCoordinates {
    id: u64,
    metadata: HashMap<String, Value>,
    geometry: Geometry
}

impl Feature for FeatureWithCoordinates {
    fn new(feature: &Tile_Feature, keys: &Vec<String>, values: &Vec<Value>) -> Self {
        let feature_with_commands = FeatureWithCommands::new(feature, keys, values);
        let commands = feature_with_commands.commands();
        let geometry = match feature_with_commands.r#type {
            GeometryType::POINT => get_geometry_for_point(commands),
            GeometryType::LINESTRING => get_geometry_for_linestring(commands),
            GeometryType::POLYGON => get_geometry_for_polygon(commands),
            GeometryType::UNKNOWN => unreachable!()
        };

        Self {
            id: feature_with_commands.id(),
            metadata: feature_with_commands.metadata().clone(),
            geometry: geometry
        }
    }

    fn default() -> Self {
        Default::default()
    }
}

fn get_geometry_for_point(commands : &Vec<Command>) -> Geometry {
    if commands.is_empty() {
        assert!(false, "Geometry POINT should contain at least one point");
    }

    if commands.len() == 1 {
        return assert_matches!(commands[0], Command::MoveTo(x, y) => Geometry::Point([x.clone(), y.clone()]));
    }

    let mut points = Vec::new();

    for command in commands {
        match command {
            Command::MoveTo(x, y) => points.push([x.clone(), y.clone()]),
            _ => assert!(false, "Geometry MULTYPOINT should contain only MoveTo commands"),
        }
    }

    Geometry::MultyPoint(points)
}

fn get_geometry_for_linestring(commands : &Vec<Command>) -> Geometry {
    let mut geometry = Vec::new();
    let mut current_x = 0;
    let mut current_y = 0;
    let mut element = Vec::new();
    for command in commands {
        match command {
            Command::MoveTo(x, y) => {
                element = Vec::new();

                current_x += x;
                current_y += y;

                element.push([current_x, current_y]);
            },
            Command::LineTo(x, y) => {
                current_x += x;
                current_y += y;

                element.push([current_x, current_y]);
            },
            Command::ClosePath => {
                geometry.push(element.clone());
            }
        }
    }

    if geometry.len() == 0 {
        assert!(false, "Geometry LINESTRING failed to parsed");
        unreachable!();
    } else if geometry.len() == 1 {
        Geometry::LineString(geometry[0].clone())
    } else {
        Geometry::MultyLineString(geometry)
    }
}

fn get_geometry_for_polygon(commands : &Vec<Command>) -> Geometry {
    let mut geometry = Vec::new();
    let mut current_x = 0;
    let mut current_y = 0;
    let mut element = Vec::new();
    for command in commands {
        match command {
            Command::MoveTo(x, y) => {
                element = Vec::new();

                current_x += x;
                current_y += y;

                element.push([current_x, current_y]);
            },
            Command::LineTo(x, y) => {
                current_x += x;
                current_y += y;

                element.push([current_x, current_y]);
            },
            Command::ClosePath => {
                geometry.push(element.clone());
            }
        }
    }

    // TODO: The following block to calculate is wrong. We should parse polygons based on winding rules
    if geometry.len() == 0 {
        assert!(false, "Geometry POLYGON failed to parsed");
        unreachable!();
    } else if geometry.len() == 1 {
        Geometry::Polygon(geometry)
    } else {
        Geometry::MultyPolygon(vec![geometry])
    }
}

#[derive(Debug, Clone)]
enum Geometry {
    Point([i64; 2]),
    MultyPoint(Vec<[i64; 2]>),
    LineString(Vec<[i64; 2]>),
    MultyLineString(Vec<Vec<[i64; 2]>>),
    Polygon(Vec<Vec<[i64; 2]>>),
    MultyPolygon(Vec<Vec<Vec<[i64; 2]>>>),
}

impl Default for Geometry {
    fn default() -> Self { Geometry::Point([0, 0]) }
}