use serde::{Serialize, Deserialize};

use crate::protos::vector_tile::Tile as ProtoTile;

use super::feature::Feature;
use super::layer::Layer;

#[derive(Default,Debug, Serialize, Deserialize, PartialEq)]
pub struct Tile<F>
where F: Feature + std::default::Default {
    #[serde(default)]
    pub layers: Vec<Layer<F>>
}

impl<F> Tile<F>
where F: Feature + std::default::Default {
    pub fn new(tile: &ProtoTile) -> Self {
        let mut out : Tile<F> = Default::default();

        for layer in tile.get_layers() {
            out.layers.push(Layer::new(layer));
        }

        out
    }
}
