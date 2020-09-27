use cursive::menu::MenuTree;

use crate::server::ui::about_panel::about;

pub fn main_menu() -> MenuTree {
    MenuTree::new()
        .leaf("About ^+A", |s| s.add_layer(about()))
        .delimiter()
        .leaf("Quit  ^+Q", |s| s.quit())
}