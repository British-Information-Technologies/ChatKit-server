use cursive::{
	views::{Dialog, LinearLayout, PaddedView, TextView},
	View,
};

pub fn info_error_dialogue(message: &str) -> impl View {
	Dialog::new()
		.title("Info Fetch Error")
		.content(PaddedView::lrtb(
			2,
			2,
			2,
			2,
			LinearLayout::vertical()
				.child(TextView::new("Error fetching the Info from the server"))
				.child(TextView::new(message)),
		))
		.dismiss_button("Big Oof")
}
