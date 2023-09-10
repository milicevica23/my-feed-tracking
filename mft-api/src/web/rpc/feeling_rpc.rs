use crate::ctx::Ctx;
use crate::model::feeling::{Feeling, FeelingBmc, FeelingEntry, FeelingForCreate};
use crate::model::ModelManager;
use crate::web::rpc::{ParamsForCreate, ParamsForUpdate, ParamsIded};
use crate::web::Result;

pub async fn create_feeling(
    ctx: Ctx,
    mm: ModelManager,
    params: ParamsForCreate<FeelingForCreate>,
) -> Result<Feeling> {
    let ParamsForCreate { data } = params;

    let id = FeelingBmc::create(&ctx, &mm, data).await?;
    let task = FeelingBmc::get(&ctx, &mm, id).await?;

    Ok(task)
}

pub async fn list_feeling_by_user(ctx: Ctx, mm: ModelManager) -> Result<Vec<Feeling>> {
    let feelings = FeelingBmc::list_by_user(&ctx, &mm).await?;

    Ok(feelings)
}

pub async fn add_feeling_to_user(ctx: Ctx, mm: ModelManager, params: ParamsIded) -> Result<()> {
    let ParamsIded { id } = params;

    FeelingBmc::add_feeling_to_user(&ctx, &mm, id).await?;

    Ok(())
}

pub async fn add_feeling_entry_to_user(
    ctx: Ctx,
    mm: ModelManager,
    params: ParamsForCreate<FeelingEntry>,
) -> Result<()> {
    let ParamsForCreate { data } = params;

    let id = FeelingBmc::add_feeling_entry_to_user(&ctx, &mm, data).await?;

    Ok(())
}
