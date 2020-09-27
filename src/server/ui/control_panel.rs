use cursive::{View, Printer, Cursive};
use cursive::event::{EventResult, Event};
use cursive::views::{Panel, ResizedView, StackView, LinearLayout, TextView};
use cursive::view::SizeConstraint;
use crate::server::ServerV3::Server;

pub fn control_panel(s: &mut Cursive) -> Box<dyn View> {
    Box::new(
        ResizedView::new(
            SizeConstraint::Fixed(s.screen_size().x-8),
            SizeConstraint::Fixed(s.screen_size().y-8),
            Panel::new(
                LinearLayout::horizontal()
                    .child(
                        LinearLayout::vertical()
                            .child(
                                TextView::new("  ═════╡ Server ╞═════  ")
                            )
                            .child(
                                TextView::new(
                                    format!("Server name: {}", s.user_data::<Server>().unwrap().get_name())
                                )
                            )
                            .child(
                                TextView::new(
                                    format!("Server host: {}", s.user_data::<Server>().unwrap().get_address())
                                )
                            )
                            .child(
                                TextView::new(
                                    format!("Server owner: {}", s.user_data::<Server>().unwrap().get_owner())
                                )
                            )
                            .child(
                                TextView::new(
                                    format!("  ═════╡ metrics ╞═════  ")
                                )
                            )
                            .child(
                                TextView::new(
                                    format!("Server o2s_rqst: {}", s.user_data::<Server>().unwrap().o2s_rqst)
                                )
                            )
                            .child(
                                TextView::new(
                                    format!("Server c2s_msgs: {}", s.user_data::<Server>().unwrap().c2s_msgs)
                                )
                            )
                            .child(
                                TextView::new(
                                    format!("Server s2s_msgs: {}", s.user_data::<Server>().unwrap().s2s_msgs)
                                )
                            )
                            .child(
                                TextView::new(
                                    format!("Server s2c_msgs: {}", s.user_data::<Server>().unwrap().s2c_msgs)
                                )
                            )
                    )
                    .child()
            )
        )
    )
}