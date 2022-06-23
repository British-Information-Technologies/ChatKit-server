/// Configuration for the server
pub(super) struct ServerConfig {
	pub(super) port: u16,
	pub(super) name: String,
	pub(super) owner: String,
}

impl Default for ServerConfig {
	fn default() -> Self {
		ServerConfig {
			owner: "john_smith@example.com".to_string(),
			name: "default server name".to_string(),
			port: 5600,
		}
	}
}