use anyhow::Result;
use super::score::Score;

pub trait ScoreRepository{
  fn find_by_uuid(&self,uuid:&String) -> Result<Vec<Score>>;
  fn insert(&self,score:&Score) -> Result<()>;
  fn list(&self,stage_id:i32) -> Result<Vec<Score>>;
  fn is_exist(&self,score:&Score) -> bool {
    match self.find_by_uuid(&score.uuid){
      Ok(_) => true,
      Err(_) => false,
    }
  }
}