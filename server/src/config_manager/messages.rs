use actix::{Message, MessageResponse};

use crate::config_manager::types::ConfigValue;

#[derive(Message, Debug)]
#[rtype(result = "()")]
pub enum ConfigManagerOutput {
	#[allow(dead_code)]
	ConfigUpdated(String, ConfigValue),
}

#[derive(Message, Debug)]
#[rtype(result = "ConfigManagerDataResponse")]
pub enum ConfigManagerDataMessage {
	GetValue(String),
	#[allow(dead_code)]
	SetValue(String, Option<ConfigValue>),
	#[allow(dead_code)]
	SoftSetValue(String, Option<ConfigValue>),
}

#[derive(MessageResponse, Debug)]
pub enum ConfigManagerDataResponse {
	GotValue(Option<ConfigValue>),
	SetValue(String, Option<ConfigValue>),
	SoftSetValue(String, Option<ConfigValue>),
}
