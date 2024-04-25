use cursive::{
	views::{Dialog, TextView},
	View,
};

pub fn invlaid_address_dialogue(address: String) -> impl View {
	Dialog::new()
		.title("error")
		.content(TextView::new(format!(
			"'{}' is an invalid address",
			address
		)))
		.button("Dismiss", |s| {
			s.pop_layer();
		})
}
