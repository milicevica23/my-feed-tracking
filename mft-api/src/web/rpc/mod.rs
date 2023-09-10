// region:    --- Modules

mod feeling_rpc;
mod ingredient_rpc;

use crate::ctx::Ctx;
use crate::model::ModelManager;
use crate::web::rpc::feeling_rpc::{
    add_feeling_entry_to_user, add_feeling_to_user, create_feeling, list_feeling_by_user,
};
use crate::web::rpc::ingredient_rpc::{
    add_ingredient_entry_to_user, add_ingredient_to_user, create_ingredient,
    list_ingredient_by_user,
};

use crate::web::{Error, Result};
use axum::extract::State;
use axum::response::{IntoResponse, Response};
use axum::routing::post;
use axum::{Json, Router};
use serde::Deserialize;
use serde_json::{from_value, json, to_value, Value};
use tracing::debug;

// endregion: --- Modules

// region:    --- RPC Types

/// JSON-RPC Request Body.
#[derive(Deserialize)]
struct RpcRequest {
    id: Option<Value>,
    method: String,
    params: Option<Value>,
}

#[derive(Deserialize)]
pub struct ParamsForCreate<D> {
    data: D,
}

#[derive(Deserialize)]
pub struct ParamsForUpdate<D> {
    id: i64,
    data: D,
}

#[derive(Deserialize)]
pub struct ParamsIded {
    id: i64,
}

// endregion: --- RPC Types

pub fn routes(mm: ModelManager) -> Router {
    Router::new()
        .route("/rpc", post(rpc_handler))
        .with_state(mm)
}

async fn rpc_handler(
    State(mm): State<ModelManager>,
    ctx: Ctx,
    Json(rpc_req): Json<RpcRequest>,
) -> Response {
    // -- Create the RPC Info to be set to the response.extensions.
    let rpc_info = RpcInfo {
        id: rpc_req.id.clone(),
        method: rpc_req.method.clone(),
    };

    // -- Exec & Store RpcInfo in response.
    let mut res = _rpc_handler(ctx, mm, rpc_req).await.into_response();
    res.extensions_mut().insert(rpc_info);

    res
}

/// RPC basic information holding the id and method for further logging.
#[derive(Debug)]
pub struct RpcInfo {
    pub id: Option<Value>,
    pub method: String,
}

macro_rules! exec_rpc_fn {
    // With Params
    ($rpc_fn:expr, $ctx:expr, $mm:expr, $rpc_params:expr) => {{
        let rpc_fn_name = stringify!($rpc_fn);
        let params = $rpc_params.ok_or(Error::RpcMissingParams {
            rpc_method: rpc_fn_name.to_string(),
        })?;
        let params = from_value(params).map_err(|_| Error::RpcFailJsonParams {
            rpc_method: rpc_fn_name.to_string(),
        })?;
        $rpc_fn($ctx, $mm, params).await.map(to_value)??
    }};

    // Without Params
    ($rpc_fn:expr, $ctx:expr, $mm:expr) => {
        $rpc_fn($ctx, $mm).await.map(to_value)??
    };
}

async fn _rpc_handler(ctx: Ctx, mm: ModelManager, rpc_req: RpcRequest) -> Result<Json<Value>> {
    let RpcRequest {
        id: rpc_id,
        method: rpc_method,
        params: rpc_params,
    } = rpc_req;

    debug!("{:<12} - _rpc_handler - method: {rpc_method}", "HANDLER");

    let result_json: Value = match rpc_method.as_str() {
        // -- Task RPC methods.
        "create_ingredient" => exec_rpc_fn!(create_ingredient, ctx, mm, rpc_params),
        "create_feeling" => exec_rpc_fn!(create_feeling, ctx, mm, rpc_params),
        "list_ingredient_by_user" => exec_rpc_fn!(list_ingredient_by_user, ctx, mm),
        "list_feelig_by_user" => exec_rpc_fn!(list_feeling_by_user, ctx, mm),
        "add_feeling_to_user" => exec_rpc_fn!(add_feeling_to_user, ctx, mm, rpc_params),
        "add_ingredient_to_user" => exec_rpc_fn!(add_ingredient_to_user, ctx, mm, rpc_params),
        "add_feeling_entry_to_user" => exec_rpc_fn!(add_feeling_entry_to_user, ctx, mm, rpc_params),
        "add_ingredient_entry_to_user" => {
            exec_rpc_fn!(add_ingredient_entry_to_user, ctx, mm, rpc_params)
        }

        // -- Fallback as Err.
        _ => return Err(Error::RpcMethodUnknown(rpc_method)),
    };

    let body_response = json!({
        "id": rpc_id,
        "result": result_json
    });

    Ok(Json(body_response))
}
