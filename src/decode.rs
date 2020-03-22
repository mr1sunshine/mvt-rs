use protobuf::{parse_from_bytes};

use super::protos::vector_tile::{*};
use super::tiles;
use super::decoded::Tile as DecodeTile;

pub fn decode(bytes: &[u8]) -> Result<DecodeTile, protobuf::ProtobufError> {
    let mut out : tiles::Tile = Default::default();
    let tile = parse_from_bytes::<Tile>(&bytes)?;

    for layer in tile.get_layers() {
        out.layers.push(decode_layer(layer));
    }

    Ok(DecodeTile::new(&out))
}

fn decode_value(value: &Tile_Value) -> tiles::Value {
    if value.has_string_value() {
        return tiles::Value::StringValue(value.get_string_value().to_string());
    }
    else if value.has_float_value() {
        return tiles::Value::FloatValue(value.get_float_value());
    }
    else if value.has_double_value() {
        return tiles::Value::DoubleValue(value.get_double_value());
    }
    else if value.has_int_value() {
        return tiles::Value::IntValue(value.get_int_value());
    }
    else if value.has_uint_value() {
        return tiles::Value::UintValue(value.get_uint_value());
    }
    else if value.has_sint_value() {
        return tiles::Value::SintValue(value.get_sint_value());
    }
    else if value.has_bool_value() {
        return tiles::Value::BoolValue(value.get_bool_value());
    }

    tiles::Value::StringValue("".to_string())
}

fn decode_geom_type(r#type: Tile_GeomType) -> tiles::GeometryType {
    match r#type {
        Tile_GeomType::UNKNOWN => tiles::GeometryType::UNKNOWN,
        Tile_GeomType::POINT => tiles::GeometryType::POINT,
        Tile_GeomType::LINESTRING => tiles::GeometryType::LINESTRING,
        Tile_GeomType::POLYGON => tiles::GeometryType::POLYGON,
    }
}

fn decode_feature(feature: &Tile_Feature) -> tiles::Feature {
    tiles::Feature {
        id: feature.get_id(),
        tags: feature.get_tags().to_vec(),
        r#type: decode_geom_type(feature.get_field_type()),
        geometry: feature.get_geometry().to_vec()
    }
}

fn decode_layer(layer: &Tile_Layer) -> tiles::Layer {
    let mut out : tiles::Layer = Default::default();

    out.version = layer.get_version();
    out.name = layer.get_name().to_string();
    for f in layer.get_features() {
        out.features.push(decode_feature(f));
    }
    out.keys = layer.get_keys().to_vec();
    for v in layer.get_values() {
        out.values.push(decode_value(v));
    }
    out.extent = layer.get_extent();

    out
}