use crate::ctx::Ctx;
use crate::model::base::{self, DbBmc};
use crate::model::ModelManager;
use crate::model::{Error, Result};
use serde::{Deserialize, Serialize};
use sqlb::Fields;
use sqlx::FromRow;

#[derive(Debug, Fields, Clone, FromRow, Serialize)]
pub struct Ingredient {
    pub id: i64,
    pub name: String,
    pub allergy_information: String,
}

#[derive(Deserialize, Fields)]
pub struct IngredientForCreate {
    pub name: String,
    pub allergy_information: String,
}

#[derive(Fields, Deserialize)]
pub struct IngredientForUpdate {
    pub name: String,
}

pub struct IngredientBmc;

impl DbBmc for IngredientBmc {
    const TABLE: &'static str = "ingredient";
}

impl IngredientBmc {
    pub async fn create(ctx: &Ctx, mm: &ModelManager, task_c: IngredientForCreate) -> Result<i64> {
        base::create::<Self, _>(ctx, mm, task_c).await
    }

    pub async fn get(ctx: &Ctx, mm: &ModelManager, id: i64) -> Result<Ingredient> {
        base::get::<Self, _>(ctx, mm, id).await
    }

    pub async fn list(ctx: &Ctx, mm: &ModelManager) -> Result<Vec<Ingredient>> {
        base::list::<Self, _>(ctx, mm).await
    }

    pub async fn update(
        ctx: &Ctx,
        mm: &ModelManager,
        id: i64,
        task_u: IngredientForUpdate,
    ) -> Result<()> {
        base::update::<Self, _>(ctx, mm, id, task_u).await
    }

    pub async fn delete(ctx: &Ctx, mm: &ModelManager, id: i64) -> Result<()> {
        base::delete::<Self>(ctx, mm, id).await
    }
}

#[cfg(test)]
mod tests {
    #![allow(unused)]
    use serial_test::serial;

    use crate::_dev_utils;

    use super::*;

    #[serial]
    #[tokio::test]
    async fn test_create_ok() -> Result<()> {
        let mm = _dev_utils::init_test().await;
        let ctx = Ctx::root_ctx();

        let fx_name = "test_create_ok ing name";

        let ing_c = IngredientForCreate {
            name: fx_name.to_string(),
        };
        let id = IngredientBmc::create(&ctx, &mm, ing_c).await?;
        let (title,): (String,) = sqlx::query_as("SELECT name from tasks where id = $1")
            .bind(id)
            .fetch_one(mm.db())
            .await?;

        assert_eq!(title, fx_name);

        let count = sqlx::query("DELETE FROM ingredient where id = $1")
            .bind(id)
            .execute(mm.db())
            .await?
            .rows_affected();
        assert_eq!(count, 1, "Did not delete 1 row");
        Ok(())
    }
}
