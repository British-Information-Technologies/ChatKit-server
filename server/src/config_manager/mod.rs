//! # config_manager
//! This module contains all the code that deals with server configuration.
//! It tries to implement a singleton actor, that will be fetchable globaly.

mod arg_fetcher;
mod config_manager;
mod messages;
mod types;

pub(crate) use arg_fetcher::get_args;
pub(crate) use config_manager::ConfigManager;
pub(crate) use messages::{
	ConfigManagerDataMessage, ConfigManagerDataResponse, ConfigManagerOutput,
};
pub(crate) use types::ConfigValue;
