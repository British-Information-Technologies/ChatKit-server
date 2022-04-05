use std::sync::Arc;
use crate::client::Client;
use crate::client_manager::ClientMgrMessage;

pub struct PluginInterface {
	new_connection_callback: Box<dyn FnMut(&mut PluginInterface) -> ()>,
}