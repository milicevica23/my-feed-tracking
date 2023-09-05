mod dev_db;
use tokio::sync::OnceCell;
use tracing::info;

use crate::{
    ctx::Ctx,
    model::{
        ingredient::{Ingredient, IngredientBmc, IngredientForCreate},
        ModelManager,
    },
};

pub async fn init_dev() {
    static INIT: OnceCell<()> = OnceCell::const_new();

    INIT.get_or_init(|| async {
        info!("{:<12} - init_dev_all()", "FOR-DEV-ONLY");

        dev_db::init_dev_db().await.unwrap();
    })
    .await;
}

/// Initialize test environment.
pub async fn init_test() -> ModelManager {
    static INIT: OnceCell<ModelManager> = OnceCell::const_new();

    let mm = INIT
        .get_or_init(|| async {
            init_dev().await;
            ModelManager::new().await.unwrap()
        })
        .await;

    mm.clone()
}

// pub async fn seed_tasks(
//     ctx: &Ctx,
//     mm: &ModelManager,
//     titles: &[&str],
// ) -> crate::model::Result<Vec<Ingredient>> {
//     let mut names = Vec::new();

//     for name in names {
//         let id = IngredientBmc::create(
//             ctx,
//             mm,
//             IngredientForCreate {
//                 name: name.to_string(),
//             },
//         )
//         .await?;
//         let ing = IngredientBmc::get(ctx, mm, id).await?;

//         names.push(ing);
//     }

//     Ok(names)
// }
