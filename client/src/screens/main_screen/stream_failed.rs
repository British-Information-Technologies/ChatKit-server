use cursive::{
	views::{Dialog, TextView},
	View,
};

pub fn stream_failed_dialogue() -> impl View {
	Dialog::new().content(TextView::new("stream failed to open?"))
}
