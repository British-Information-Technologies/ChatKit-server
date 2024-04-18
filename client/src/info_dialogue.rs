use cursive::{
	view::Margins,
	views::{Dialog, LinearLayout, TextView},
	View,
};

pub fn info_dialogue(name: String, owner: String) -> impl View {
	Dialog::new()
		.padding(Margins::lrtb(2, 2, 2, 2))
		.content(
			LinearLayout::vertical()
				.child(TextView::new("Got Info:"))
				.child(TextView::new(format!("name: {}", name)))
				.child(TextView::new(format!("owner: {}", owner))),
		)
		.button("Close", |s| {
			s.pop_layer();
		})
}
