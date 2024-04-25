use cursive::{
	view::{Nameable, Resizable},
	views::{
		Button,
		EditView,
		LinearLayout,
		PaddedView,
		Panel,
		SelectView,
		TextView,
	},
	Cursive,
	View,
};

use crate::{
	exit,
	screens::{
		connect_setup::segues::segue_to_user_details_form,
		info_dialogue::get_info,
	},
	state::StateObject,
	MethodSelection,
};

pub fn methods_view(host: String) -> impl View {
	let horizontal = LinearLayout::horizontal();
	Panel::new(PaddedView::lrtb(
		2,
		2,
		2,
		2,
		LinearLayout::vertical()
			.child(
				EditView::new()
					.content(host)
					.on_edit(Cursive::set_host)
					.with_name("host_input"),
			)
			.child(TextView::new("Select option"))
			.child(
				SelectView::new()
					.item("Get Info", MethodSelection::GetInfo)
					.item("Connect...", MethodSelection::Connect)
					.on_submit(execute)
					.with_name("method_selector"),
			)
			.child(horizontal.child(Button::new("Cancel", exit))),
	))
	.fixed_size((40, 10))
}

fn execute(s: &mut Cursive, item: &MethodSelection) {
	println!("executing");

	match item {
		MethodSelection::GetInfo => run_get_info(s),
		MethodSelection::Connect => {
			_ = s.cb_sink().send(segue_to_user_details_form());
		}
	}
}

// mark: - this should be removed

fn run_get_info(s: &mut Cursive) {
	let sink = s.cb_sink().clone();
	_ = sink.send(Box::new(get_info));
}
