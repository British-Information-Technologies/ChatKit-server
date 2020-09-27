use cursive::menu::MenuTree;

pub fn main_Menu() -> MenuTree {
    MenuTree::new()
        .leaf("About ^+A", |s| s.add_layer(About::new()))
        .delimiter()
        .leaf("Quit  ^+Q", |s| s.quit())
}