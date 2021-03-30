pub trait IMessagable<M> {
	fn send_message(&self, msg: M);
}

pub trait ICooperative {
	fn tick(&self);
}