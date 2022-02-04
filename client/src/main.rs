mod worker;
mod managers;

use worker::Worker;
use cursive::{Cursive, CursiveExt};
use cursive::traits::Nameable;
use cursive::views::{Dialog, TextView};

fn main() {
	let mut app = Cursive::default();
	let workerStream = Worker::new(app.cb_sink().clone()).start();
	
	app.set_user_data(workerStream);
	app.add_layer(Dialog::new()
		.content(TextView::new("Hello world").with_name("TextView"))
		.button("close", |s| s.quit()));
	app.set_fps(30);
	app.run();
}
