#![allow(unused)] // For beginning only.

use anyhow::Result;
use serde_json::json;

#[tokio::main]
async fn main() -> Result<()> {
    let hc = httpc_test::new_client("http://localhost:8080")?;

    let req_login = hc.do_post(
        "/api/login",
        json!({
            "username": "demo1",
            "pwd": "welcome"
        }),
    );
    req_login.await?.print().await?;

    let req_create_task = hc.do_post(
        "/api/rpc",
        json!({
            "id": 1,
            "method": "create_ingredient",
            "params": {
                "data": {
                    "name": "aleks",
                    "allergy_information": "alergy"
                }
            }
        }),
    );
    req_create_task.await?.print().await?;

    Ok(())
}
