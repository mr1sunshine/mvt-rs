use super::*;

use std::collections::HashMap;

#[derive(Debug)]
pub struct Tile {
    layers: Vec<Layer>
}

impl Tile {
    pub fn new(tile: &tiles::Tile) -> Self {
        let mut layers = Vec::new();
        for layer in &tile.layers {
            layers.push(Layer::new(layer));
        }

        Self {
            layers
        }
    }

    pub fn layers(&self) -> &Vec<Layer> {
        &self.layers
    }
}

#[derive(Debug)]
pub struct Layer {
    name: String,
    features: Vec<Feature>,
    extent: u32
}

impl Layer {
    pub fn new(layer: &tiles::Layer) -> Self {
        let mut features = Vec::new();
        for f in &layer.features {
            features.push(Feature::new(&f, layer));
        }

        Self {
            name: layer.name.clone(),
            features: features,
            extent: layer.extent
        }
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn features(&self) -> &Vec<Feature> {
        &self.features
    }

    pub fn extent(&self) -> u32 {
        self.extent
    }
}

#[derive(Debug)]
pub struct Feature {
    id: u64,
    metadata: HashMap<String, tiles::Value>,
    commands: Vec<Command>,
    r#type: tiles::GeometryType
}

impl Feature {
    pub fn new(feature: &tiles::Feature, layer: &tiles::Layer) -> Self {
        let mut hm = HashMap::new();
        for i in (0..feature.tags.len()).step_by(2) {
            hm.insert(layer.keys[feature.tags[i] as usize].clone(), layer.values[feature.tags[i + 1] as usize].clone());
        }
        let mut commands = Vec::new();
        let mut i = 0;

        while i < feature.geometry.len() {
            let command_id = feature.geometry[i] & 0x7;
            let count = feature.geometry[i] >> 3;
            i += 1;
            if command_id == 1 {
                for _ in 0..count {
                    let x = feature.geometry[i] as i64;
                    i += 1;
                    let y = feature.geometry[i] as i64;
                    i += 1;
                    commands.push(Command::MoveTo(((x >> 1) ^ (-(x & 1))) as f32, ((y >> 1) ^ (-(y & 1))) as f32))
                }
            } else if command_id == 2 {
                for _ in 0..count {
                    let x = feature.geometry[i] as i64;
                    i += 1;
                    let y = feature.geometry[i] as i64;
                    i += 1;
                    commands.push(Command::LineTo(((x >> 1) ^ (-(x & 1))) as f32, ((y >> 1) ^ (-(y & 1))) as f32))
                }
            } else if command_id == 7 {
                commands.push(Command::ClosePath);
            } else {
                assert!(false);
            }
        }

        Self {
            id: feature.id,
            metadata: hm,
            r#type: feature.r#type,
            commands: commands
        }
    }

    pub fn id(&self) -> u64 {
        self.id
    }

    pub fn metadata(&self) -> &HashMap<String, tiles::Value> {
        &self.metadata
    }

    pub fn commands(&self) -> &Vec<Command> {
        &self.commands
    }

    pub fn r#type(&self) -> tiles::GeometryType {
        self.r#type
    }
}

#[derive(Debug)]
pub enum Command {
    MoveTo(f32, f32),
    LineTo(f32, f32),
    ClosePath
}