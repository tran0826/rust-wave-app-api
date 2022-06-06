use anyhow::Result;
use crate::domain::models::stage::{stage::Stage};

pub trait StageRepository{
  fn list(&self) -> Result<Vec<Stage>>;
  fn insert(&self,difficulty:i32) -> Result<()>;
}