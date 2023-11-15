// export type TypeNames = "record" | "array" | "null" | "map" | string;

use anyhow::{anyhow, Ok};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum Type {
    Null,
    String,
    Boolean,
    Int,
    Long,
    Float,
    Double,
    Bytes,
    Record(Box<Record>),
    Enum(Enum),
    Union(Vec<Type>),
    Array(Array),
    Map(Box<Map>),
    Fixed(Fixed),
}

pub fn primitive_type_name_to_type(name: &str) -> anyhow::Result<Type> {
    match name {
        // primitives
        "string" => Ok(Type::String),
        "null" => Ok(Type::Null),
        "boolean" => Ok(Type::Boolean),
        "int" => Ok(Type::Int),
        "long" => Ok(Type::Long),
        "float" => Ok(Type::Float),
        "double" => Ok(Type::Double),
        "bytes" => Ok(Type::Bytes),
        _ => Err(anyhow!("Could not find type in type declaraiont")),
    }
}

pub fn type_name_to_type(name: &str, value: &serde_json::Value) -> anyhow::Result<Type> {
    match name {
        "record" => serde_json::from_value::<Record>(value.clone())
            .map(|r| Type::Record(Box::new(r)))
            .map_err(|e| anyhow::format_err!(e)),
        "enum" => serde_json::from_value::<Enum>(value.clone())
            .map(|e| Type::Enum(e))
            .map_err(|e| anyhow::format_err!(e)),
        "array" => serde_json::from_value::<Array>(value.clone())
            .map(|e| Type::Array(e))
            .map_err(|e| anyhow::format_err!(e)),
        "map" => serde_json::from_value::<Map>(value.clone())
            .map(|e| Type::Map(Box::new(e)))
            .map_err(|e| anyhow::format_err!(e)),
        "fixed" => serde_json::from_value::<Fixed>(value.clone())
            .map(|e| Type::Fixed(e))
            .map_err(|e| anyhow::format_err!(e)),

        // primitives
        "string" => Ok(Type::String),
        "null" => Ok(Type::Null),
        "boolean" => Ok(Type::Boolean),
        "int" => Ok(Type::Int),
        "long" => Ok(Type::Long),
        "float" => Ok(Type::Float),
        "double" => Ok(Type::Double),
        "bytes" => Ok(Type::Bytes),
        _ => Err(anyhow!("Could not find type in type declaraiont")),
    }
}

impl TryFrom<&serde_json::Value> for Type {
    type Error = anyhow::Error;

    fn try_from(value: &serde_json::Value) -> Result<Self, Self::Error> {
        match value {
            serde_json::Value::Object(obj) => match obj.get("type").and_then(|v| v.as_str()) {
                Some(vs) => {
                    info!("Matching: {}", vs);
                    type_name_to_type(vs, value)
                }
                None => Err(anyhow!("Key `type` not found.")),
            },
            serde_json::Value::Array(value) => {
                info!("Iterating over:{:?}", value);
                // This likely means it is a union
                Ok(Type::Union(
                    value
                        .iter()
                        .filter_map(|v| Type::try_from(v).ok())
                        .collect(),
                ))
            }
            serde_json::Value::String(s) => primitive_type_name_to_type(s),
            serde_json::Value::Null | serde_json::Value::Bool(_) | serde_json::Value::Number(_) => {
                Err(anyhow::anyhow!("Invalid type for a Avro Schema."))
            }
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Record {
    #[serde(alias = "type")]
    pub type_name: String,
    pub name: String,
    pub namespace: Option<String>,
    pub fields: Vec<Field>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Field {
    #[serde(alias = "type")]
    pub type_name: serde_json::Value,
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Enum {
    #[serde(alias = "type")]
    pub type_name: String,
    pub name: String,
    pub symbols: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Array {
    #[serde(alias = "type")]
    pub type_name: String,
    pub items: Vec<serde_json::Value>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Map {
    #[serde(alias = "type")]
    pub type_name: String,
    pub values: serde_json::Value,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Fixed {
    #[serde(alias = "type")]
    pub type_name: String,
    pub size: u64,
    pub name: String,
}
