use crate::{
	screens::connect_setup::user_details_form::user_details_form,
	segues::CursiveCB,
};

pub fn segue_to_user_details_form() -> CursiveCB {
	Box::new(|s| {
		s.pop_layer();
		s.add_layer(user_details_form())
	})
}
