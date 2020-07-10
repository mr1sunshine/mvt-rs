use serde::{Serialize, Deserialize};

use crate::protos::vector_tile::Tile_Feature;
use crate::utils::{decode_zigzag, signed_area};

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
        let tags = feature.get_tags().to_vec();

        Self {
            id: feature.get_id(),
            tags: tags,
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
    properties: HashMap<String, Value>,
    commands: Vec<Command>,
    r#type: GeometryType
}

impl Feature for FeatureWithCommands {
    fn new(feature: &Tile_Feature, keys: &Vec<String>, values: &Vec<Value>) -> Self {
        let feature_with_json = FeatureWithJson::new(feature, keys, values);
        let mut hm = HashMap::new();
        for i in (0..feature_with_json.tags.len()).step_by(2) {
            hm.insert(keys[feature_with_json.tags[i] as usize].clone(), values[feature_with_json.tags[i + 1] as usize].clone());
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
            properties: hm,
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

    pub fn properties(&self) -> &HashMap<String, Value> {
        &self.properties
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
    properties: HashMap<String, Value>,
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
            properties: feature_with_commands.properties().clone(),
            geometry: geometry
        }
    }

    fn default() -> Self {
        Default::default()
    }
}

impl FeatureWithCoordinates {
    pub fn id(&self) -> u64 {
        self.id
    }

    pub fn properties(&self) -> &HashMap<String, Value> {
        &self.properties
    }

    pub fn geometry(&self) -> &Geometry {
        &self.geometry
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
    let mut linestrings = Vec::new();
    let mut current_x = 0;
    let mut current_y = 0;
    for command in commands {
        match command {
            Command::MoveTo(x, y) => {
                linestrings.push(Vec::new());

                current_x += x;
                current_y += y;

                if let Some(last) = linestrings.last_mut() {
                    last.push([current_x, current_y]);
                }
            },
            Command::LineTo(x, y) => {
                current_x += x;
                current_y += y;

                if let Some(last) = linestrings.last_mut() {
                    last.push([current_x, current_y]);
                }
            },
            Command::ClosePath => {
                assert!(false, "Unexpected ClosePath for LINESTRING");
            }
        }
    }

    if linestrings.len() == 0 {
        assert!(false, "Geometry LINESTRING failed to parsed");
        unreachable!();
    } else if linestrings.len() == 1 {
        Geometry::LineString(linestrings[0].clone())
    } else {
        Geometry::MultyLineString(linestrings)
    }
}

fn get_geometry_for_polygon(commands : &Vec<Command>) -> Geometry {
    let mut geometry : Vec<Polygon> = Vec::new();
    let mut current_x = 0;
    let mut current_y = 0;
    let mut polygon = Vec::new();
    for command in commands {
        match command {
            Command::MoveTo(x, y) => {
                current_x += x;
                current_y += y;

                polygon.push([current_x, current_y]);
            },
            Command::LineTo(x, y) => {
                current_x += x;
                current_y += y;

                polygon.push([current_x, current_y]);
            },
            Command::ClosePath => {
                if signed_area(&polygon) > 0.0 {
                    geometry.push(Polygon::new(polygon));
                } else {
                    if let Some(last) = geometry.last_mut() {
                        last.add_ring(polygon);
                    }
                }
                polygon = Vec::new();
            }
        }
    }

    if geometry.len() == 0 {
        assert!(false, "Geometry POLYGON failed to parsed");
        unreachable!();
    } else if geometry.len() == 1 {
        Geometry::Polygon(geometry[0].clone())
    } else {
        Geometry::MultyPolygon(geometry)
    }
}

#[derive(Debug, Clone)]
pub enum Geometry {
    Point([i64; 2]),
    MultyPoint(Vec<[i64; 2]>),
    LineString(Vec<[i64; 2]>),
    MultyLineString(Vec<Vec<[i64; 2]>>),
    Polygon(Polygon),
    MultyPolygon(Vec<Polygon>),
}

impl Default for Geometry {
    fn default() -> Self { Geometry::Point([0, 0]) }
}

type Ring = Vec<[i64; 2]>;

#[derive(Debug, Clone)]
pub struct Polygon {
    exterior_ring: Ring,
    interior_rings: Vec<Ring>
}

impl Polygon {
    pub fn new(ring: Ring) -> Self {
        Self {
            exterior_ring: ring,
            interior_rings: Vec::new()
        }
    }

    pub fn add_ring(&mut self, ring: Ring) {
        self.interior_rings.push(ring)
    }

    pub fn exterior_ring(&self) -> &Ring {
        &self.exterior_ring
    }

    pub fn interior_rings(&self) -> &Vec<Ring> {
        &self.interior_rings
    }
}