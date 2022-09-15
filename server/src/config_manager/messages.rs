use crate::config_manager::types::ConfigValue;
use actix::{Message, MessageResponse};

#[derive(Message, Debug)]
#[rtype(result = "()")]
pub enum ConfigManagerOutput {
	ConfigUpdated(String, ConfigValue),
}

#[derive(Message, Debug)]
#[rtype(result = "ConfigManagerDataResponse")]
pub enum ConfigManagerDataMessage {
	GetValue(String),
	SetValue(String, Option<ConfigValue>),
	SoftSetValue(String, Option<ConfigValue>),
}

#[derive(MessageResponse, Debug)]
pub enum ConfigManagerDataResponse {
	GotValue(Option<ConfigValue>),
	SetValue(String, Option<ConfigValue>),
	SoftSetValue(String, Option<ConfigValue>),
}
