use cursive::Cursive;

use crate::{
	screens::info_dialogue::{
		info_dialogue::info_dialogue,
		info_error_dialogue::info_error_dialogue,
		info_loading_panel::info_loading_panel,
	},
	segues::CursiveCB,
};

pub fn segue_to_info_loading_panel() -> CursiveCB {
	Box::new(|s: &mut Cursive| {
		s.add_layer(info_loading_panel());
	})
}

pub fn segue_to_load_error_dialogue(reason: String) -> CursiveCB {
	Box::new(move |s| {
		s.pop_layer();
		s.add_layer(info_error_dialogue(&reason));
	})
}

pub fn segue_to_info_dialgue(
	name: String,
	owner: String,
) -> Box<dyn FnOnce(&mut Cursive) + Send> {
	Box::new(|s| {
		s.pop_layer();
		s.add_layer(info_dialogue(name, owner));
	})
}
