mod worker;
mod managers;
mod worker_message;

use worker::Worker;
use cursive::{Cursive, CursiveExt};
use cursive::menu::{Item, Tree};
use cursive::traits::Nameable;
use cursive::views::{Dialog, TextView};

fn main() {
	let mut app = Cursive::default();
	let worker_stream =
		Worker::new(app.cb_sink().clone()).start();
	
	
	
	app.set_user_data(worker_stream);
	app.add_layer(Dialog::new()
		.content(TextView::new("Hello world").with_name("TextView"))
		.button("close", |s| s.quit()));
	app.menubar().autohide = false;
	app.menubar().add_subtree(
		"Application",
		Tree::new()
			.item(
				Item::leaf("About", |s| s.quit())
			).delimiter().item(
				Item::leaf("Quit",|s| s.quit())
			)
	);
	app.set_fps(30);
	app.run();
}
