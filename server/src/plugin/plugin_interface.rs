#[async_trait::async_trait]
pub trait IPluginInterface {
	fn get_string<T: Into<String>>() -> T;
}
