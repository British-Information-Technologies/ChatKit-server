use crate::{
	screens::main_screen::{
		invalid_address_dialogue::invlaid_address_dialogue,
		select_operation::methods_view,
	},
	segues::CursiveCB,
};

pub fn segue_to_select_operation() -> CursiveCB {
	Box::new(|s| {
		s.pop_layer();
		s.add_layer(methods_view("127.0.0.1:6500".into()))
	})
}

pub fn segue_open_invalid_address_dialogue(address: String) -> CursiveCB {
	Box::new(|s| s.add_layer(invlaid_address_dialogue(address)))
}
