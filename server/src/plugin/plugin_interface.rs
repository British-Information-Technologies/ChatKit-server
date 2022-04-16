use crate::event::Event;
use std::fmt::Debug;
use std::sync::Arc;
use std::sync::Weak;

pub type WeakPluginInterface = Weak<dyn IPluginInterface>;
pub(crate) type PluginInterface = Arc<dyn IPluginInterface>;

#[async_trait::async_trait]
pub trait IPluginInterface: Send + Sync + Debug {
	fn get_next_event(&self) -> Option<Event>;
	fn get_event_count(&self) -> usize;
}
