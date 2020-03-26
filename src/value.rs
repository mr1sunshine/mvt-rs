use serde::{Serialize, Deserialize};

use crate::protos::vector_tile::Tile_Value;

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

impl Value {
    pub fn new(value: &Tile_Value) -> Self {
        decode_value(value)
    }
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

fn decode_value(value: &Tile_Value) -> Value {
    if value.has_string_value() {
        return Value::StringValue(value.get_string_value().to_string());
    }
    else if value.has_float_value() {
        return Value::FloatValue(value.get_float_value());
    }
    else if value.has_double_value() {
        return Value::DoubleValue(value.get_double_value());
    }
    else if value.has_int_value() {
        return Value::IntValue(value.get_int_value());
    }
    else if value.has_uint_value() {
        return Value::UintValue(value.get_uint_value());
    }
    else if value.has_sint_value() {
        return Value::SintValue(value.get_sint_value());
    }
    else if value.has_bool_value() {
        return Value::BoolValue(value.get_bool_value());
    }

    Value::StringValue("".to_string())
}