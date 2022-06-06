#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Stage{
  pub id:i32,
  pub difficulty:i32,
}


impl Stage{
  pub fn new(id:i32,difficulty:i32)->Self {
    Self {
      id,
      difficulty,
    }
  }
}