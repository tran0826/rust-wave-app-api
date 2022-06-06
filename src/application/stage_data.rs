use crate::domain::models::stage::stage::Stage;
use getset::Getters;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Getters, PartialEq, Eq, Debug)]
pub struct StageData{
  #[getset(get = "pub with_prefix")]
  id: i32,
  #[getset(get = "pub with_prefix")]
  difficulty:i32,
}

impl StageData {
  pub fn new(source: Stage) -> Self{
    Self{
      id:source.id,
      difficulty:source.difficulty,
    }
  }
}