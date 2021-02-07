use url::Url

pub trait TBundle {
  fn main() -> Result<Self>;
  
  fn initWithURL(url: Url) -> Result<Self>;
  fn initWithPath(path: String) -> Result<Self>;

  fn urlForResource(name: String, extention: String, subDirectory: Option<Strign>) -> Result<[u8]>;


}