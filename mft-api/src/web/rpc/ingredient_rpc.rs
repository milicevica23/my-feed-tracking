use crate::ctx::Ctx;
use crate::model::feeling::Feeling;
use crate::model::ingredient::{
    self, Ingredient, IngredientBmc, IngredientEntry, IngredientForCreate,
};
use crate::model::ModelManager;
use crate::web::rpc::{ParamsForCreate, ParamsForUpdate, ParamsIded};
use crate::web::Result;

pub async fn create_ingredient(
    ctx: Ctx,
    mm: ModelManager,
    params: ParamsForCreate<IngredientForCreate>,
) -> Result<Ingredient> {
    let ParamsForCreate { data } = params;

    let id = IngredientBmc::create(&ctx, &mm, data).await?;
    let task = IngredientBmc::get(&ctx, &mm, id).await?;

    Ok(task)
}
pub async fn list_ingredient_by_user(ctx: Ctx, mm: ModelManager) -> Result<Vec<Ingredient>> {
    let ingredient = IngredientBmc::list_by_user(&ctx, &mm).await?;

    Ok(ingredient)
}

pub async fn add_ingredient_to_user(ctx: Ctx, mm: ModelManager, params: ParamsIded) -> Result<()> {
    let ParamsIded { id } = params;

    IngredientBmc::add_ingredient_to_user(&ctx, &mm, id).await?;

    Ok(())
}

pub async fn add_ingredient_entry_to_user(
    ctx: Ctx,
    mm: ModelManager,
    params: ParamsForCreate<IngredientEntry>,
) -> Result<()> {
    let ParamsForCreate { data } = params;

    let id = IngredientBmc::add_ingredient_entry_to_user(&ctx, &mm, data).await?;

    Ok(())
}

// pub async fn list_tasks(ctx: Ctx, mm: ModelManager) -> Result<Vec<Task>> {
// 	let tasks = TaskBmc::list(&ctx, &mm).await?;

// 	Ok(tasks)
// }

// pub async fn update_task(
// 	ctx: Ctx,
// 	mm: ModelManager,
// 	params: ParamsForUpdate<TaskForUpdate>,
// ) -> Result<Task> {
// 	let ParamsForUpdate { id, data } = params;

// 	TaskBmc::update(&ctx, &mm, id, data).await?;

// 	let task = TaskBmc::get(&ctx, &mm, id).await?;

// 	Ok(task)
// }

// pub async fn delete_task(
// 	ctx: Ctx,
// 	mm: ModelManager,
// 	params: ParamsIded,
// ) -> Result<Task> {
// 	let ParamsIded { id } = params;

// 	let task = TaskBmc::get(&ctx, &mm, id).await?;
// 	TaskBmc::delete(&ctx, &mm, id).await?;

// 	Ok(task)
// }
