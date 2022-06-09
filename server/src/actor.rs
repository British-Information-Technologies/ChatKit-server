mod actix_server;
mod client_management;
mod network;
pub(crate) mod prelude;

pub(crate) use actix_server::ServerActor;

use tokio::time::sleep;
use tokio::time::Duration;

#[actix::main()]
async fn main() {
	let _server = ServerActor::new();
	loop {
		sleep(Duration::from_millis(1000)).await;
	}
}
