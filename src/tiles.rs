#[derive(Debug)]
pub enum GeometryType {
    UNKNOWN,
    POINT,
    LINESTRING,
    POLYGON
}

impl Default for GeometryType {
    fn default() -> Self { GeometryType::UNKNOWN }
}

#[derive(Debug)]
pub enum Value {
    StringValue(String),
    FloatValue(f32),
    DoubleValue(f64),
    IntValue(i64),
    UintValue(u64),
    SintValue(i64),
    BoolValue(bool)
}

#[derive(Default,Debug)]
pub struct Feature {
    pub id: u64,
    pub tags: Vec<u32>,
    pub r#type: GeometryType,
    pub geometry: Vec<u32>
}

#[derive(Default,Debug)]
pub struct Layer {
    pub version: u32,
    pub name: String,
    pub features: Vec<Feature>,
    pub keys: Vec<String>,
    pub values: Vec<Value>,
    pub extent: u32
}

#[derive(Default,Debug)]
pub struct Tile {
    pub layers: Vec<Layer>
}