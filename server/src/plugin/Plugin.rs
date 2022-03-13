use std::sync::Arc;

type CreatePluginFn = dyn Fn() -> Arc<dyn Plugin>;

pub trait Plugin {
	fn name(&self) -> String;
	fn init(&self);
}