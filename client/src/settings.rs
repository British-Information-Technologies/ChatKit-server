use cursive::{
	view::{Margins, Nameable, Resizable},
	views::{Button, EditView, LinearLayout, PaddedView, Panel},
	Cursive,
	View,
	XY,
};

use crate::state::State;

pub fn settings_panel(host: Option<String>) -> impl View {
	Panel::new(PaddedView::new(
		Margins::lrtb(2, 2, 2, 2),
		LinearLayout::vertical()
			.child(
				EditView::new()
					.content(host.unwrap_or("localhost:6500".into()))
					.on_edit(set_host)
					.with_name("host_input"),
			)
			.child(Button::new("Close", |s| {
				s.pop_layer();
			}))
			.min_size(XY { x: 30, y: 8 }),
	))
	.title("Settings")
}

fn set_host(s: &mut Cursive, host: &str, _: usize) {
	s.user_data::<State>().unwrap().set_host(host);
}
