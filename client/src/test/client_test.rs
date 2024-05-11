#[cfg(test)]
mod test {
	use foundation::{
		client::network_connection::NetworkConnection,
		prelude::Info,
	};
	use uuid::Uuid;

	#[tokio::test]
	async fn get_info() {
		let client = NetworkConnection::connect(
			"127.0.0.1:6500"
				.parse()
				.expect("failed to parse address string"),
		)
		.await
		.expect("failed to connect to test server");

		let info: Info = client.send_get_info().await.unwrap();

		println!("info: {:?}", info)
	}

	#[tokio::test]
	async fn connect_and_disconnect() {
		let client = NetworkConnection::connect(
			"127.0.0.1:6500"
				.parse()
				.expect("failed to parse address string"),
		)
		.await
		.expect("failed to connect to test server");

		let (w, r) = client
			.send_connect(Uuid::new_v4(), "test user".into())
			.await
			.unwrap();

		drop(w);
		drop(r);

		println!("finished")
	}
}
