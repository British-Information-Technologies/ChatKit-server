use std::future::Future;

use cursive::Cursive;
use tokio::runtime::Runtime;

use crate::network::NetworkState;

pub struct State {
	runtime: Runtime,
	connection_state: NetworkState,
	host: String,
}

impl State {
	pub fn new() -> Self {
		Self {
			runtime: Runtime::new().unwrap(),
			connection_state: NetworkState::Disconnected,
			host: "127.0.0.1:6500".into(),
		}
	}

	pub fn get_host(&self) -> String {
		self.host.clone()
	}

	pub fn set_host<T: Into<String>>(&mut self, value: T) {
		self.host = value.into()
	}

	pub fn get_rt(&mut self) -> &mut Runtime {
		&mut self.runtime
	}

	pub fn spawn<F>(&mut self, future: F)
	where
		F: Future + Send + 'static,
		F::Output: Send + 'static,
	{
		self.runtime.spawn(future);
	}
}

impl Default for State {
	fn default() -> Self {
		Self::new()
	}
}

pub trait StateObject {
	fn state(&mut self) -> &mut State;
	fn set_host(&mut self, host: &str, _: usize);
}

impl StateObject for Cursive {
	fn set_host(&mut self, host: &str, _: usize) {
		self.user_data::<State>().unwrap().set_host(host);
	}

	fn state(&mut self) -> &mut State {
		self.user_data::<State>().unwrap()
	}
}
