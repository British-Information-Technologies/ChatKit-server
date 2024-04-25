use cursive::Cursive;

pub type CursiveCB = Box<dyn FnOnce(&mut Cursive) + Send>;

pub fn segue_pop_layer() -> CursiveCB {
	Box::new(|s| {
		s.pop_layer();
	})
}
