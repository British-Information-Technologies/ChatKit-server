use std::sync::Weak;

use crate::event::Event;

pub trait IResponder<T>
where
	T: Sync + Send,
{
	fn post_event(&self, event: Event<T>) {
		if let Some(next) = self.get_next() {
			if let Some(next) = next.upgrade() {
				next.post_event(event);
				return;
			}
		}
		self.r#final(event);
	}
	fn get_next(&self) -> Option<Weak<dyn IResponder<T>>>;
	fn on_event(&self, event: Event<T>);
	fn r#final(&self, _event: Event<T>) {}
}
