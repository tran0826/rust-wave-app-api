use crate::domain::models::score::score::Score;
use getset::Getters;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Getters, Debug)]
pub struct ScoreData{
  #[getset(get = "pub with_prefix")]
  stage_id: i32,
  #[getset(get = "pub with_prefix")]
  clear_time: f32,
  #[getset(get = "pub with_prefix")]
  user_name: String,
  #[getset(get = "pub with_prefix")]
  uuid: String,
}

impl ScoreData {
  pub fn new(source: &Score) -> Self{
    Self{
      uuid:source.uuid.clone(),
      stage_id:source.stage_id,
      clear_time:source.clear_time,
      user_name:source.user_name.clone(),
    }
  }
}