use crate::domain::models::stage::stage::Stage;
use crate::domain::models::stage::stage_repository::StageRepository;
use anyhow::{Context, Result};
use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};

use super::schema::stage;
use super::schema::stage::dsl::*;

#[derive(Debug, Queryable, Clone)]
pub struct StageEntity {
    pub id: i32,
    pub difficulty: i32,
}

#[derive(Debug, Insertable)]
#[table_name = "stage"]
pub struct NewStage {
    pub difficulty: i32,
}

impl From<StageEntity> for Stage {
    fn from(entity: StageEntity) -> Stage {
        Stage {
            id: entity.id,
            difficulty: entity.difficulty,
        }
    }
}

pub struct StageRepositoryImpl {
    pub pool: Box<Pool<ConnectionManager<PgConnection>>>,
}

impl StageRepository for StageRepositoryImpl {
    fn list(&self) -> Result<Vec<Stage>> {
        let conn = self.pool.get().context("failed to get connection")?;
        match stage.load::<StageEntity>(&conn) {
            Ok(result) => Ok(result
                .iter()
                .map(|c| Stage::from(c.clone()))
                .collect::<Vec<Stage>>()),
            Err(e) => Err(anyhow::anyhow!(e)),
        }
    }

    fn insert(&self, num: i32) -> Result<()> {
        let conn = self.pool.get().context("failed to get connection")?;
        let new_stage = NewStage { difficulty: num };
        diesel::insert_into(stage::table)
            .values(&new_stage)
            .execute(&conn)
            .expect("Error insert new stage");

        Ok(())
    }
}
