use std::sync::Arc;

pub trait IMessagable<TMessage, TSender> {
	fn send_message(&self, msg: TMessage);
	fn set_sender(&self, sender: TSender);
}

pub trait ICooperative {
	fn tick(&self);
}

pub trait IPreemptive {
	fn run(arc: &Arc<Self>);
	fn start(arc: &Arc<Self>);
}
