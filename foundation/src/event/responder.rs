use crate::event::Event;
use std::sync::Weak;

pub trait IResponder {
	fn post_event(&self, event: Event) {
		if let Some(next) = self.get_next() {
			if let Some(next) = next.upgrade() {
				next.post_event(event);
				return;
			}
		}
		self.r#final(event);
	}
	fn get_next(&self) -> Option<Weak<dyn IResponder>>;
	fn on_event(&self, event: Event);
	fn r#final(&self, _event: Event) {}
}
