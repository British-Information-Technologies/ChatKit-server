use crate::event::Event;

#[async_trait::async_trait]
pub trait IPluginInterface {
	fn get_string<T: Into<String>>() -> T;
	fn get_next_event(&self) -> Option<Event>;
}
