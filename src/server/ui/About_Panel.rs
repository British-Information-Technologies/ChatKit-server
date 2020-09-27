use cursive::views::{Dialog, TextView};
use cursive::view::ViewWrapper;
use cursive::{Printer, View};

pub fn About() -> Box<dyn View> {
    Box::new(
        Dialog::new()
            .content("rust chat server written by Mitchel Hardie & Michael Bailey (c) 2020")
            .button("Close", |s| {s.pop_layer();})
    )
}