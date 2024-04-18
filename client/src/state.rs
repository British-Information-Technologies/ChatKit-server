use tokio::runtime::Runtime;

pub struct State {
	runtime: Runtime,
	host: String,
}

impl State {
	pub fn new() -> Self {
		Self {
			runtime: Runtime::new().unwrap(),
			host: "localhost:6500".into(),
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
}
