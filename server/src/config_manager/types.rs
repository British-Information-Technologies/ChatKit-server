use std::collections::BTreeMap;

use toml::value::Value;

/// # ConfigValue
/// Each value type that can be used within a config file.
/// gets used when reading and writing to a config file.
#[derive(Clone, Debug)]
pub enum ConfigValue {
	Dict(BTreeMap<String, Self>),
	Array(Vec<Self>),
	String(String),
	Number(i64),
	Float(f64),
	Bool(bool),
}

impl From<ConfigValue> for Value {
	fn from(v: ConfigValue) -> Self {
		match v {
			ConfigValue::Dict(dict) => Value::Table(
				dict.into_iter().map(|(k, v)| (k, v.into())).collect(),
			),
			ConfigValue::Array(arr) => {
				Value::Array(arr.into_iter().map(|v| v.into()).collect())
			}
			ConfigValue::String(s) => Value::String(s),
			ConfigValue::Number(n) => Value::Integer(n),
			ConfigValue::Float(f) => Value::Float(f),
			ConfigValue::Bool(b) => Value::Boolean(b),
		}
	}
}

impl From<Value> for ConfigValue {
	fn from(v: Value) -> Self {
		match v {
			Value::Table(dict) => ConfigValue::Dict(
				dict.into_iter().map(|(k, v)| (k, v.into())).collect(),
			),
			Value::Array(arr) => {
				ConfigValue::Array(arr.into_iter().map(|v| v.into()).collect())
			}
			Value::String(s) => ConfigValue::String(s),
			Value::Integer(n) => ConfigValue::Number(n),
			Value::Float(f) => ConfigValue::Float(f),
			Value::Boolean(b) => ConfigValue::Bool(b),
			Value::Datetime(d) => ConfigValue::String(d.to_string()),
		}
	}
}
