use crate::crypt::{pwd, EncryptContent};
use crate::ctx::Ctx;
use crate::model::user::{UserBmc, UserForLogin};
use crate::model::ModelManager;
use crate::web::mw_auth::CtxExtError;
use crate::web::{self, remove_token_cookie, Error, Result, AUTH_TOKEN};
use axum::extract::State;
use axum::routing::post;
use axum::{Json, Router};
use mft_domain::user::LoginPayload;
use mft_domain::user::UserForCreate;
use serde::Deserialize;
use serde_json::{json, Value};
use tower_cookies::{Cookie, Cookies};
use tower_http::cors::{Any, CorsLayer};
use tracing::debug;

pub fn routes(mm: ModelManager) -> Router {
    Router::new()
        .route("/api/login", post(api_login_handler))
        .route("/api/register", post(api_register_handler))
        .route("/api/logoff", post(api_logoff_handler))
        .layer(CorsLayer::permissive())
        .with_state(mm)
}

// region:    --- Login
async fn api_register_handler(
    State(mm): State<ModelManager>,
    Json(user_c): Json<UserForCreate>,
) -> Result<Json<Value>> {
    debug!("{:<12} - api_register_handler", "HANDLER");
    debug!("{:#?} - api_register_handler", user_c);
    let root_ctx = Ctx::root_ctx();
    let res = UserBmc::register_user(&root_ctx, &mm, user_c).await;

    // Create the success body.
    let body = Json(json!({
        "result": {
            "success": true
        }
    }));

    Ok(body)
}

// region:    --- Login
async fn api_login_handler(
    State(mm): State<ModelManager>,
    cookies: Cookies,
    Json(payload): Json<LoginPayload>,
) -> Result<Json<Value>> {
    debug!("{:<12} - api_login_handler", "HANDLER");

    let LoginPayload {
        username,
        pwd: pwd_clear,
    } = payload;
    let root_ctx = Ctx::root_ctx();

    // -- Get the user.
    let user: UserForLogin = UserBmc::first_by_username(&root_ctx, &mm, &username)
        .await?
        .ok_or(Error::LoginFailUsernameNotFound)?;
    let user_id = user.id;

    // -- Validate the password.
    let Some(pwd) = user.pwd else {
		return Err(Error::LoginFailUserHasNoPwd{ user_id });
	};

    pwd::validate_pwd(
        &EncryptContent {
            salt: user.pwd_salt.to_string(),
            content: pwd_clear.clone(),
        },
        &pwd,
    )
    .map_err(|_| Error::LoginFailPwdNotMatching { user_id })?;

    // -- Set web token.
    web::set_token_cookie(&cookies, &user.username, &user.token_salt.to_string())?;

    // -- Get Token String
    let token = cookies
        .get(AUTH_TOKEN)
        .map(|c| c.value().to_string())
        .ok_or(CtxExtError::TokenNotInCookie)
        .unwrap();

    // Create the success body.
    let body = Json(json!({
        "result": {
            "success": true,
            "access_token": token
        }
    }));

    Ok(body)
}

// endregion: --- Login

// region:    --- Logoff
async fn api_logoff_handler(
    cookies: Cookies,
    Json(payload): Json<LogoffPayload>,
) -> Result<Json<Value>> {
    debug!("{:<12} - api_logoff_handler", "HANDLER");
    let should_logoff = payload.logoff;

    if should_logoff {
        remove_token_cookie(&cookies)?;
    }

    // Create the success body.
    let body = Json(json!({
        "result": {
            "logged_off": should_logoff
        }
    }));

    Ok(body)
}

#[derive(Debug, Deserialize)]
struct LogoffPayload {
    logoff: bool,
}
// endregion: --- Logoff
