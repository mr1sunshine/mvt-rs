use serde::{Deserialize, Serialize};

use crate::protos::vector_tile::Tile as ProtoTile;

use super::feature::Feature;
use super::layer::Layer;

#[derive(Default, Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct Tile<F>
where
    F: Feature + std::default::Default,
{
    #[serde(default)]
    pub layers: Vec<Layer<F>>,
}

impl<F> Tile<F>
where
    F: Feature + std::default::Default,
{
    pub fn new(tile: &ProtoTile) -> Self {
        let mut out: Tile<F> = Default::default();

        for layer in tile.get_layers() {
            out.layers.push(Layer::new(layer));
        }

        out
    }

    pub fn get_layer(&self, name: &str) -> Option<&Layer<F>> {
        for layer in &self.layers {
            if name == layer.name {
                return Some(layer);
            }
        }
        None
    }
}
