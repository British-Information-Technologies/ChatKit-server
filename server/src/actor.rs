mod network;
pub(crate) mod prelude;

use network::NetworkManager;
use network::NetworkMessage::Ping;
use network::NetworkResponse::Pong;

#[actix::main()]
async fn main() {
	let network = NetworkManager::new();

	let pong = network.send(Ping).await;
	if let Ok(Pong) = pong {
		println!("received pong");
	} else {
		println!("error occurred")
	}
}
