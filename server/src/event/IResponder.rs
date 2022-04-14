use crate::event::Event;

pub(crate) trait IResponder {
	fn accepts_event<'a, 'b>(&'a self, event: Event<'b>) -> bool;
}
