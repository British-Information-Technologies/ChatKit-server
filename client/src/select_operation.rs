use cursive::{
	view::Nameable,
	views::{Button, LinearLayout, PaddedView, Panel, SelectView, TextView},
	CbSink,
	Cursive,
	View,
};
use foundation::{
	networking::{read_message, write_message},
	prelude::{
		network_client_message,
		network_server_message,
		GetInfo,
		Info,
		NetworkClientMessage,
		NetworkServerMessage,
	},
};
use tokio::{net::TcpStream, process::Command};

use crate::{info_dialogue::info_dialogue, state::State, MethodSelection};

pub fn methods_view() -> impl View {
	let horizontal = LinearLayout::horizontal();
	Panel::new(PaddedView::lrtb(
		2,
		2,
		2,
		2,
		LinearLayout::vertical()
			.child(TextView::new("Select option"))
			.child(
				SelectView::new()
					.item("Get Info", MethodSelection::GetInfo)
					.on_submit(execute)
					.with_name("method_selector"),
			)
			.child(horizontal.child(Button::new("Cancel", exit))),
	))
	.title("Select method")
}

fn exit(s: &mut Cursive) {
	s.quit();
}

fn execute(s: &mut Cursive, item: &MethodSelection) {
	let _sink = s.cb_sink().clone();

	match item {
		MethodSelection::GetInfo => run_get_info(s),
	}

	let rt = &s.user_data::<State>().unwrap().get_rt();

	rt.spawn(async {});
}

fn run_get_info(s: &mut Cursive) {
	let host = s.user_data::<State>().unwrap().get_host();
	let sink = s.cb_sink().clone();
	let rt = &s.user_data::<State>().unwrap().get_rt();

	// _ = sink.send(Box::new(|s| s.add_layer(Dialog::new())));

	rt.spawn(async move {
		let stream_res = TcpStream::connect(host).await;
		match stream_res {
			Ok(stream) => {
				get_request(stream, sink).await;
			}
			Err(_e) => {}
		}
	});
}

async fn get_request(mut stream: TcpStream, sink: CbSink) {
	let message = read_message::<NetworkServerMessage>(&mut stream).await;

	if let Ok(NetworkServerMessage {
		message:
			Some(network_server_message::Message::Request(
				foundation::prelude::Request { a: true },
			)),
	}) = message
	{
		perform_get_info(stream, sink.clone()).await;
	}
}

async fn perform_get_info(mut stream: TcpStream, sink: CbSink) {
	let message = NetworkClientMessage {
		message: Some(network_client_message::Message::GetInfo(GetInfo {})),
	};

	write_message(&mut stream, message).await.unwrap();

	let message = read_message::<NetworkServerMessage>(&mut stream)
		.await
		.unwrap();

	if let NetworkServerMessage {
		message:
			Some(network_server_message::Message::GotInfo(Info { owner, server_name })),
	} = message
	{
		sink
			.send(segue_to_info_dialgue(server_name, owner))
			.unwrap();
	}
}

fn segue_to_info_dialgue(
	name: String,
	owner: String,
) -> Box<dyn FnOnce(&mut Cursive) + Send> {
	Box::new(|s| {
		s.add_layer(info_dialogue(name, owner));
	})
}
