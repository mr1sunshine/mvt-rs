use super::*;

use std::collections::HashMap;

#[derive(Debug, Default)]
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
    geometry: Vec<Vec<[i64; 2]>>,
    // commands: Vec<Command>,
    r#type: tiles::GeometryType
}

fn decode_zigzag(input: u32) -> i64 {
    return (input as i64 >> 1) ^ (-(input as i64 & 1));
}

fn process_command(command_type: CommandType, count : u32, i: &mut usize, data: &Vec<u32>) -> Vec<Command> {
    let mut out = Vec::new();
    for _ in 0..count {
        let x = data[*i];
        *i += 1;
        let y = data[*i];
        *i += 1;
        out.push(Command{command_type: command_type, x: decode_zigzag(x), y: decode_zigzag(y)});
    }

    return out;
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
                commands.append(&mut process_command(CommandType::MoveTo, count, &mut i, &feature.geometry));
            } else if command_id == 2 {
                commands.append(&mut process_command(CommandType::LineTo, count, &mut i, &feature.geometry));
            } else if command_id == 7 {
                commands.push(Command{command_type: CommandType::ClosePath, x: 0, y: 0});
            } else {
                assert!(false);
            }
        }

        let mut geometry = Vec::new();
        let mut current_x = 0;
        let mut current_y = 0;
        let mut element = Vec::new();
        for command in &commands {
            match command.command_type {
                CommandType::MoveTo => {
                    element = Vec::new();

                    current_x += command.x;
                    current_y += command.y;

                    element.push([current_x, current_y]);
                },
                CommandType::LineTo => {
                    current_x += command.x;
                    current_y += command.y;

                    element.push([current_x, current_y]);
                },
                CommandType::ClosePath => {
                    geometry.push(element.clone());
                }
            }
        }

        Self {
            id: feature.id,
            metadata: hm,
            r#type: feature.r#type,
            // commands: commands,
            geometry: geometry
        }
    }

    pub fn id(&self) -> u64 {
        self.id
    }

    pub fn metadata(&self) -> &HashMap<String, tiles::Value> {
        &self.metadata
    }

    // pub fn commands(&self) -> &Vec<Command> {
    //     &self.commands
    // }

    pub fn r#type(&self) -> tiles::GeometryType {
        self.r#type
    }
}

#[derive(Debug)]
pub struct Command {
    command_type: CommandType,
    x: i64,
    y: i64
}

#[derive(Debug, Clone, Copy)]
pub enum CommandType {
    MoveTo,
    LineTo,
    ClosePath
}