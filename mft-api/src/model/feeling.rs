use crate::ctx::Ctx;
use crate::model::base::{self, DbBmc};
use crate::model::ModelManager;
use crate::model::{Error, Result};
use serde::{Deserialize, Serialize};
use sqlb::Fields;
use sqlx::FromRow;

#[derive(Debug, Fields, Clone, FromRow, Serialize)]
pub struct Feeling {
    pub id: i64,
    pub name: String,
}

#[derive(Deserialize, Fields)]
pub struct FeelingForCreate {
    pub name: String,
}

#[derive(Fields, Deserialize)]
pub struct FeelingForUpdate {
    pub name: String,
}

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct FeelingEntry {
    id: i64,
    value: i64,
    timestamp: chrono::DateTime<chrono::Utc>,
}

pub struct FeelingBmc;

impl DbBmc for FeelingBmc {
    const TABLE: &'static str = "feeling";
    const TABLE2USER: &'static str = "user_2_feeling";
}

impl FeelingBmc {
    pub async fn create(ctx: &Ctx, mm: &ModelManager, task_c: FeelingForCreate) -> Result<i64> {
        base::create::<Self, _>(ctx, mm, task_c).await
    }

    pub async fn get(ctx: &Ctx, mm: &ModelManager, id: i64) -> Result<Feeling> {
        base::get::<Self, _>(ctx, mm, id).await
    }

    pub async fn list(ctx: &Ctx, mm: &ModelManager) -> Result<Vec<Feeling>> {
        base::list::<Self, _>(ctx, mm).await
    }

    pub async fn update(
        ctx: &Ctx,
        mm: &ModelManager,
        id: i64,
        task_u: FeelingForUpdate,
    ) -> Result<()> {
        base::update::<Self, _>(ctx, mm, id, task_u).await
    }

    pub async fn delete(ctx: &Ctx, mm: &ModelManager, id: i64) -> Result<()> {
        base::delete::<Self>(ctx, mm, id).await
    }

    pub async fn list_by_user(ctx: &Ctx, mm: &ModelManager) -> Result<Vec<Feeling>> {
        base::list_by_user::<Self, _>(ctx, mm).await
    }

    pub async fn add_feeling_to_user(ctx: &Ctx, mm: &ModelManager, id: i64) -> Result<i64> {
        base::add_to_user::<Self>(ctx, mm, id).await
    }

    pub async fn add_feeling_entry_to_user(
        ctx: &Ctx,
        mm: &ModelManager,
        entry: FeelingEntry,
    ) -> Result<()> {
        sqlx::query("INSERT INTO user_has_feeling values($1,$2,$3,$4)")
            .bind(ctx.user_id())
            .bind(entry.id)
            .bind(entry.value)
            .bind(entry.timestamp)
            .execute(mm.db())
            .await?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {}
