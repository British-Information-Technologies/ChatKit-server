use cursive::{
	views::{Panel, TextView},
	View,
};

pub fn info_loading_panel() -> impl View {
	Panel::new(TextView::new("Loading"))
}
