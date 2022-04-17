use crate::event::Event;
use crate::event::EventResult;
use std::fmt::Debug;
use std::sync::Arc;
use std::sync::Weak;

use futures::channel::oneshot::Receiver;

pub type WeakPluginInterface = Weak<dyn IPluginInterface>;
pub(crate) type PluginInterface = Arc<dyn IPluginInterface>;

pub trait IPluginInterface: Send + Sync + Debug {
	fn send_event(&self, event: Event) -> Receiver<EventResult>;
}
