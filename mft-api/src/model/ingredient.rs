use crate::ctx::Ctx;
use crate::model::base::{self, DbBmc};
use crate::model::ModelManager;
use crate::model::{Error, Result};
use serde::{Deserialize, Serialize};
use sqlb::Fields;
use sqlx::FromRow;

#[derive(Debug, Clone, FromRow, Serialize)]
pub struct Ingredient {
    pub id: i64,
    pub name: String,
}

pub struct TaskForCreate {}
