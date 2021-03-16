use std::sync::{Weak,Arc};

pub trait IOwned<T> {
  fn set_owner(&self, owner: Weak<T>);
}

pub trait IOwner<T> {
  fn add_child(&self, child: Arc<T>);
	fn get_ref(&self) -> Weak<Self>;
}

pub trait IMessagable<M> {
	fn send_message(&self, msg: M);
}

pub trait ICooperative {
	fn tick(&self);
}