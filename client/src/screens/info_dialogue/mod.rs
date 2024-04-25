use cursive::Cursive;

use crate::{
	network::network_connection::NetworkConnection,
	screens::{
		info_dialogue::segues::{
			segue_to_info_dialgue,
			segue_to_info_loading_panel,
			segue_to_load_error_dialogue,
		},
		main_screen::segues::segue_open_invalid_address_dialogue,
	},
	state::StateObject,
};

pub mod info_dialogue;
pub mod info_error_dialogue;
pub mod info_loading_panel;
pub mod segues;

pub fn get_info(s: &mut Cursive) {
	let sink = s.cb_sink().clone();
	let state = s.state();
	let address = state.get_host().parse();

	let Ok(address) = address else {
		_ = sink.send(segue_open_invalid_address_dialogue(state.get_host()));
		return;
	};

	state.spawn(async move {
		_ = sink.send(segue_to_info_loading_panel());
		let conn = NetworkConnection::connect(address).await;

		let Ok(conn) = conn else {
			_ = sink.send(segue_to_load_error_dialogue(
				"
				failed to connect to the server
			"
				.into(),
			));
			return;
		};

		let res = conn.send_get_info().await;

		let Ok(info) = res else {
			_ = sink.send(segue_to_load_error_dialogue(
				"
				Failed to retrieve info
			"
				.into(),
			));
			return;
		};

		_ = sink.send(segue_to_info_dialgue(info.server_name, info.owner));
	})
}
