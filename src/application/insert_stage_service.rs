use anyhow::Result;

use crate::domain::models::stage::stage_repository::StageRepository;

pub struct InsertStageService<T: StageRepository> {
    stage_repository: T,
}

impl<T: StageRepository> InsertStageService<T> {
    pub fn new(stage_repository: T) -> Self {
        Self { stage_repository }
    }

    pub fn handle(&self, difficulty: i32) -> Result<()> {
        match self.stage_repository.insert(difficulty) {
            Ok(_) => Ok(()),
            Err(_) => Err(anyhow::anyhow!("failed insert stage")),
        }
    }
}

#[cfg(test)]
mod test {}
