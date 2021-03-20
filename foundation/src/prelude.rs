pub trait IMessagable<TMessage, TSender> {
	fn send_message(&self, msg: TMessage);
  fn set_sender(&self, sender: TSender);
}

pub trait ICooperative {
	fn tick(&self);
}