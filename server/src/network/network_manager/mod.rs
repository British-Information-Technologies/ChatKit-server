//! # network_manager
//! This module contains the network manager actor
//! it's role involves handling new oncomming network connections

mod actor;
mod builder;
mod messages;

pub(crate) use actor::NetworkManager;
pub(crate) use builder::*;
pub(crate) use messages::{
	NetworkDataMessage,
	NetworkDataOutput,
	NetworkMessage,
	NetworkOutput,
};
