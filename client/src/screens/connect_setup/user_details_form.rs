use cursive::{
	view::Resizable,
	views::{Dialog, EditView, LinearLayout, TextView},
	Cursive,
	View,
};

use crate::screens::main_screen::segues::segue_to_select_operation;

pub fn user_details_form() -> impl View {
	Dialog::new()
		.title("User Setup")
		.content(
			LinearLayout::vertical()
				.child(
					LinearLayout::horizontal()
						.child(TextView::new("Username").min_width(9))
						.child(EditView::new().full_width()),
				)
				.child(
					LinearLayout::horizontal()
						.child(TextView::new("UUID").min_width(9))
						.child(EditView::new().full_width()),
				),
		)
		.button("Cancel", on_cancel)
		.button("Connect", on_connect)
		.fixed_size((40, 10))
}

fn on_connect(s: &mut Cursive) {
	println!("Attempting conneciton");
	s.add_layer(
		Dialog::new()
			.title("LOL XD")
			.content(TextView::new("Yeah this isnt iomplemented yet"))
			.dismiss_button("Dismiss"),
	)
}

fn on_cancel(s: &mut Cursive) {
	_ = s.cb_sink().send(segue_to_select_operation());
}
