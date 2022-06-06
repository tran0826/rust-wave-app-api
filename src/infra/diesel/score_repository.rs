use crate::domain::models::score::score::Score;
use crate::domain::models::score::score_repository::ScoreRepository;

use super::schema::score;
use super::schema::score::dsl::*;
use anyhow::{Context, Result};
use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};

#[derive(Debug, Queryable, Clone)]
pub struct ScoreEntity {
    pub uuid: String,
    pub stage_id: i32,
    pub clear_time: f32,
    pub user_name: String,
}

#[derive(Debug, Insertable)]
#[table_name = "score"]
pub struct NewScore {
    pub uuid: String,
    pub stage_id: i32,
    pub clear_time: f32,
    pub user_name: String,
}

impl From<ScoreEntity> for Score {
    fn from(entity: ScoreEntity) -> Score {
        Score {
            uuid: entity.uuid,
            stage_id: entity.stage_id,
            clear_time: entity.clear_time,
            user_name: entity.user_name,
        }
    }
}

pub struct ScoreRepositoryImpl {
    pub pool: Box<Pool<ConnectionManager<PgConnection>>>,
}

impl ScoreRepository for ScoreRepositoryImpl {
    fn find_by_uuid(&self, id: &String) -> Result<Vec<Score>> {
        let conn = self.pool.get().context("failed to get connection")?;
        match score.filter(score::uuid.eq(id)).load::<ScoreEntity>(&conn) {
            Ok(result) => match result.len() {
                0 => Err(anyhow::anyhow!("Not found score")),
                _ => Ok(result.iter().map(|c| Score::from(c.clone())).collect::<Vec<Score>>()),
            },
            Err(e) => Err(anyhow::anyhow!(e)),
        }
    }

    fn insert(&self, data: &Score) -> Result<()> {
        let conn = self.pool.get().context("failed to get connection")?;
        let new_score = NewScore {
            uuid: data.uuid.clone(),
            stage_id: data.stage_id,
            clear_time: data.clear_time,
            user_name: data.user_name.clone(),
        };
        diesel::insert_into(score::table)
            .values(&new_score)
            .execute(&conn)
            .expect("Error insert new stage");

        Ok(())
    }

    fn list(&self, num: i32) -> Result<Vec<Score>> {
        let conn = self.pool.get().context("failed to get connection")?;
        match score
            .filter(score::stage_id.eq(num))
            .order(clear_time.asc())
            .limit(50)
            .load::<ScoreEntity>(&conn)
        {
            Ok(result) => Ok(result
                .iter()
                .map(|c| Score::from(c.clone()))
                .collect::<Vec<Score>>()),
            Err(e) => Err(anyhow::anyhow!(e)),
        }
    }
}
