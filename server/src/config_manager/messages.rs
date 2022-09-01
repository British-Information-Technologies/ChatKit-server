use crate::config_manager::types::ConfigValue;
use actix::{Message, MessageResponse};

#[derive(Message)]
#[rtype(result = "()")]
pub enum ConfigManagerOutput {
	ConfigUpdated(String, ConfigValue),
}

#[derive(Message)]
#[rtype(result = "Result<ConfigManagerDataResponse, &'static str>")]
pub enum ConfigManagerDataMessage {
	GetValue(String),
	SetValue(String, ConfigValue),
	SoftSetValue(String, ConfigValue),
}

#[derive(MessageResponse)]
pub enum ConfigManagerDataResponse {
	GotValue(ConfigValue),
	SetValue,
}
