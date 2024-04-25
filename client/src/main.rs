pub mod network;
pub mod screens;

pub mod segues;
mod settings;
pub mod state;

use cursive::{event::Event, menu::Tree, views::Menubar, Cursive};

use crate::{
	screens::main_screen::select_operation::methods_view,
	settings::settings_panel,
	state::State,
};

enum MethodSelection {
	GetInfo,
	Connect,
}

fn menu_bar(menu_bar: &mut Menubar) {
	menu_bar.add_subtree(
		"Chat Kit",
		Tree::new()
			.leaf("Settings", open_settings)
			.delimiter()
			.leaf("Quit", exit),
	);
}

fn main() {
	let mut scr = cursive::default();
	scr.set_fps(30);

	let state = State::new();

	scr.set_user_data(state);

	menu_bar(scr.menubar());
	scr.add_global_callback(Event::Key(cursive::event::Key::Esc), |s| {
		s.select_menubar()
	});

	scr.add_layer(methods_view("127.0.0.1:6500".into()));

	scr.run()
}

fn exit(s: &mut Cursive) {
	s.quit();
}

fn open_settings(s: &mut Cursive) {
	let host = s.user_data::<State>().map(|s| s.get_host());
	s.add_layer(settings_panel(host));
}
