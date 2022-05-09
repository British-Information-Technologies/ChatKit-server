use foundation::event::Event;
use foundation::event::EventResult;
use foundation::event::IResponder;
use std::fmt::Debug;
use std::sync::Arc;
use std::sync::Weak;

use futures::channel::oneshot::Receiver;

pub type WeakPluginInterface<T>
where
	T: Sync + Send,
= Weak<dyn IPluginInterface<T>>;

pub(crate) type PluginInterface<T>
where
	T: Sync + Send,
= Arc<dyn IPluginInterface<T>>;

pub trait IPluginInterface<T>: IResponder<T> + Send + Sync + Debug
where
	T: Sync + Send,
{
	fn send_event(&self, event: Event<T>) -> Receiver<EventResult>;
}
