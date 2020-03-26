use serde::{Serialize, Deserialize};

use crate::protos::vector_tile::Tile_Layer;

use super::feature::Feature;
use super::value::Value;

#[derive(Default, Debug, Serialize, Deserialize, PartialEq)]
pub struct Layer<F>
where F: Feature + std::default::Default {
    pub version: u32,
    pub name: String,
    pub features: Vec<F>,
    pub keys: Vec<String>,
    pub values: Vec<Value>,
    #[serde(default = "default_extent")]
    pub extent: u32
}

impl<F> Layer<F>
where F: Feature + std::default::Default {
    pub fn new(layer: &Tile_Layer) -> Self {
        decode_layer(layer)
    }
}

fn default_extent() -> u32 {
    4096
}

fn decode_layer<F>(layer: &Tile_Layer) -> Layer<F>
where F: Feature + std::default::Default {
    let mut out : Layer<F> = Default::default();

    out.version = layer.get_version();
    out.name = layer.get_name().to_string();
    out.keys = layer.get_keys().to_vec();
    for v in layer.get_values() {
        out.values.push(Value::new(v));
    }
    for f in layer.get_features() {
        out.features.push(F::new(f, &out.keys, &out.values));
    }
    out.extent = layer.get_extent();

    out
}