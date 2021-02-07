use uuid::Uuid;

pub trait TClient<TClientMessage> {
  fn new(uuid: Uuid, name: String, addr: String);

  fn send(&self, bytes: Vec<u8>) -> Result<(), &str>;
  fn recv(&self) -> Option<Vec<u8>>;

  fn sendMsg(&self, msg: TClientMessage) -> Result<(), &str>;
  fn recvMsg(&self) -> Option<TClientMessage>;

  fn tick(&self);
}