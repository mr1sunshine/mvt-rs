use serde::{Serialize, Deserialize};
use serde_repr::*;

#[derive(Debug, Serialize_repr, Deserialize_repr, PartialEq, Clone, Copy)]
#[repr(u8)]
pub enum GeometryType {
    UNKNOWN = 0,
    POINT = 1,
    LINESTRING = 2,
    POLYGON = 3
}

impl Default for GeometryType {
    fn default() -> Self { GeometryType::UNKNOWN }
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum Value {
    #[serde(rename = "string_value")]
    StringValue(String),
    #[serde(rename = "float_value")]
    FloatValue(f32),
    #[serde(rename = "double_value")]
    DoubleValue(f64),
    #[serde(rename = "int_value")]
    IntValue(i64),
    #[serde(rename = "uint_value")]
    UintValue(u64),
    #[serde(rename = "sint_value")]
    SintValue(i64),
    #[serde(rename = "bool_value")]
    BoolValue(bool)
}

impl Clone for Value {
    fn clone(&self) -> Value {
        match self {
            Value::StringValue(v) => Value::StringValue(v.clone()),
            Value::FloatValue(v) => Value::FloatValue(*v),
            Value::DoubleValue(v) => Value::DoubleValue(*v),
            Value::IntValue(v) => Value::IntValue(*v),
            Value::UintValue(v) => Value::UintValue(*v),
            Value::SintValue(v) => Value::SintValue(*v),
            Value::BoolValue(v) => Value::BoolValue(*v),
        }
    }
}

#[derive(Default, Debug, Serialize, Deserialize, PartialEq)]
pub struct Feature {
    #[serde(default)]
    pub id: u64,
    pub tags: Vec<u32>,
    #[serde(default)]
    pub r#type: GeometryType,
    #[serde(default)]
    pub geometry: Vec<u32>
}

#[derive(Default, Debug, Serialize, Deserialize, PartialEq)]
pub struct Layer {
    pub version: u32,
    pub name: String,
    pub features: Vec<Feature>,
    pub keys: Vec<String>,
    pub values: Vec<Value>,
    #[serde(default = "default_extent")]
    pub extent: u32
}

fn default_extent() -> u32 {
    4096
}

#[derive(Default,Debug, Serialize, Deserialize, PartialEq)]
pub struct Tile {
    #[serde(default)]
    pub layers: Vec<Layer>
}