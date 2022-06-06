use serde::{Deserialize, Serialize};

use crate::domain::models::score::score::Score;


#[derive(Debug, Clone, Eq, PartialEq, Hash, Default, Deserialize, Serialize)]
pub struct StageRequest{
  pub difficulty: i32,
}

impl StageRequest{
  pub fn of(&self) -> i32 {
    self.difficulty
  }
}

#[derive(Debug, Clone,  Default, Deserialize, Serialize)]
pub struct ScoreRequest{
  pub uuid: String,
  pub stage_id: i32,
  pub clear_time: f32,
  pub user_name: String,
}

impl ScoreRequest{
  pub fn of(&self) -> Score{
    Score::new(
      self.uuid.clone(),
      self.stage_id,
      self.clear_time,
      self.user_name.clone()
    )
  }
}