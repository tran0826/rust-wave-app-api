use anyhow::Result;
use crate::domain::models::score::{score_repository::ScoreRepository};
use super::score_data::ScoreData;
pub struct GetScoreListService<T>
where
    T: ScoreRepository,
{
    score_repository: T,
}

impl<T: ScoreRepository> GetScoreListService<T> {
    pub fn new(score_repository: T) -> Self {
        Self {
            score_repository,
        }
    }

    pub fn handle(&self,stage_id:i32) -> Result<Vec<ScoreData>> {
        match self.score_repository.list(stage_id) {
            Ok(value) => Ok(value.iter().map(|c| ScoreData::new(c.clone())).collect::<Vec<ScoreData>>()),
            Err(_) => Err(anyhow::anyhow!("can't get score list.")),
        }
    }
}

#[cfg(test)]
mod tests{

}