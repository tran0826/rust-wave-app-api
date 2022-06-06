use anyhow::Result;

use crate::domain::models::score::{score::Score, score_repository::ScoreRepository};

use super::score_data::ScoreData;

pub struct InsertScoreService<T: ScoreRepository> {
    score_repository: T,
}

impl<T: ScoreRepository> InsertScoreService<T> {
    pub fn new(score_repository: T) -> Self {
        Self { score_repository }
    }

    pub fn handle(&self, data: ScoreData) -> Result<()> {
      let score = Score::new(
        data.get_uuid().clone(),
        data.get_stage_id().clone(),
        data.get_clear_time().clone(),
        data.get_user_name().clone(),
      );
      if self.score_repository.is_exist(&score) {
        return Err(anyhow::anyhow!("already used uuid"));
      }else{
        self.score_repository.insert(&score).unwrap();
      }
      Ok(())
    }
}

#[cfg(test)]
mod test {}
