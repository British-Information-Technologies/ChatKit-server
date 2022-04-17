use crate::event::Event;

pub(crate) trait IResponder {
	fn accepts_event(&self, event: Event) -> bool;
	fn on_event(&self, event: Event);
}
