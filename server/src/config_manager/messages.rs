use super::types::ConfigError;
use crate::config_manager::types::ConfigValue;
use actix::{Message, MessageResponse};

#[derive(Message, Debug)]
#[rtype(result = "()")]
pub enum ConfigManagerOutput {
	ConfigUpdated(String, ConfigValue),
}

#[derive(Message, Debug)]
#[rtype(result = "Result<ConfigManagerDataResponse, ConfigError>")]
pub enum ConfigManagerDataMessage {
	GetValue(String),
	SetValue(String, ConfigValue),
	SoftSetValue(String, ConfigValue),
}

#[derive(MessageResponse, Debug)]
pub enum ConfigManagerDataResponse {
	GotValue(ConfigValue),
	SetValue(String, ConfigValue),
	SoftSetValue(String, ConfigValue),
}
