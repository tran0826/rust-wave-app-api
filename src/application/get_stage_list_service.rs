use anyhow::Result;
use crate::domain::models::stage::{stage_repository::StageRepository};
use super::stage_data::StageData;
pub struct GetStageListService<T>
where
    T: StageRepository,
{
    stage_repository: T,
}

impl<T: StageRepository> GetStageListService<T> {
    pub fn new(stage_repository: T) -> Self {
        Self {
            stage_repository,
        }
    }

    pub fn handle(&self) -> Result<Vec<StageData>> {
        match self.stage_repository.list() {
            Ok(value) => Ok(value.iter().map(|c| StageData::new(c.clone())).collect::<Vec<StageData>>()),
            Err(_) => Err(anyhow::anyhow!("can't get stage list.")),
        }
    }
}

#[cfg(test)]
mod tests{

}