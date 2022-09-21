//! # config_manager
//! This module contains all the code that deals with server configuration.
//! It tries to implement a singleton actor, that will be fetchable globaly.

pub mod arg_parser;
mod builder;
mod config_manager;
mod messages;
mod types;

pub(crate) use config_manager::ConfigManager;
pub(crate) use messages::{ConfigManagerDataMessage, ConfigManagerDataResponse};
pub(crate) use types::ConfigValue;
