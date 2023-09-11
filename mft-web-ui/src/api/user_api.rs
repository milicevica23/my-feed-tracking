use super::types::{ErrorResponse, UserLoginResponse};
use reqwasm::http;
use mft_domain::user::LoginPayload;
use mft_domain::user::UserForCreate;

const BACKEND_URL: &'static str= "http://localhost:8080/";

pub async fn api_register_user(username: String, pwd_clear: String) -> Result<(), String> {
    let user = UserForCreate{
        username: username.to_string(),
        pwd_clear: pwd_clear.to_string(),
    };

    let response = match http::Request::post(&format!("{}{}",BACKEND_URL, "api/register"))
        .header("Content-Type", "application/json")
        .body(serde_json::to_string(&user).unwrap())
        .send()
        .await
    {
        Ok(res) => res,
        Err(_) => return Err("Failed to make request".to_string()),
    };

    if response.status() != 200 {
        let error_response = response.json::<ErrorResponse>().await;
        if let Ok(error_response) = error_response {
            return Err(error_response.message);
        } else {
            return Err(format!("API error: {}", response.status()));
        }
    }
    Ok(())
    // let res_json = response.json::<UserResponse>().await;
    // match res_json {
    //     Ok(data) => Ok(data.data.user),
    //     Err(_) => Err("Failed to parse response".to_string()),
    // }
}

pub async fn api_login_user(username: String, pwd_clear: String) -> Result<UserLoginResponse, String> {
    let login_payload = LoginPayload {
        username: username,
        pwd: pwd_clear,
    }; 
    let response = match http::Request::post(&format!("{}{}",BACKEND_URL, "api/auth/login"))
        .header("Content-Type", "application/json")
        .body(serde_json::to_string(&login_payload).unwrap())
        .send()
        .await
    {
        Ok(res) => res,
        Err(_) => return Err("Failed to make request".to_string()),
    };

    if response.status() != 200 {
        let error_response = response.json::<ErrorResponse>().await;
        if let Ok(error_response) = error_response {
            return Err(error_response.message);
        } else {
            return Err(format!("API error: {}", response.status()));
        }
    }

    let res_json = response.json::<UserLoginResponse>().await;
    match res_json {
        Ok(data) => Ok(data),
        Err(_) => Err("Failed to parse response".to_string()),
    }
}

// pub async fn api_refresh_access_token() -> Result<UserLoginResponse, String> {
//     let response = match http::Request::get("http://localhost:8000/api/auth/refresh")
//         .header("Content-Type", "application/json")
//         .credentials(http::RequestCredentials::Include)
//         .send()
//         .await
//     {
//         Ok(res) => res,
//         Err(_) => return Err("Failed to make request".to_string()),
//     };

//     if response.status() != 200 {
//         let error_response = response.json::<ErrorResponse>().await;
//         if let Ok(error_response) = error_response {
//             return Err(error_response.message);
//         } else {
//             return Err(format!("API error: {}", response.status()));
//         }
//     }

//     let res_json = response.json::<UserLoginResponse>().await;
//     match res_json {
//         Ok(data) => Ok(data),
//         Err(_) => Err("Failed to parse response".to_string()),
//     }
// }

// pub async fn api_user_info() -> Result<User, String> {
//     let response = match http::Request::get("http://localhost:8000/api/users/me")
//         .credentials(http::RequestCredentials::Include)
//         .send()
//         .await
//     {
//         Ok(res) => res,
//         Err(_) => return Err("Failed to make request".to_string()),
//     };

//     if response.status() != 200 {
//         let error_response = response.json::<ErrorResponse>().await;
//         if let Ok(error_response) = error_response {
//             return Err(error_response.message);
//         } else {
//             return Err(format!("API error: {}", response.status()));
//         }
//     }

//     let res_json = response.json::<UserResponse>().await;
//     match res_json {
//         Ok(data) => Ok(data.data.user),
//         Err(_) => Err("Failed to parse response".to_string()),
//     }
// }

// pub async fn api_logout_user() -> Result<(), String> {
//     let response = match http::Request::get("http://localhost:8000/api/auth/logout")
//         .credentials(http::RequestCredentials::Include)
//         .send()
//         .await
//     {
//         Ok(res) => res,
//         Err(_) => return Err("Failed to make request".to_string()),
//     };

//     if response.status() != 200 {
//         let error_response = response.json::<ErrorResponse>().await;
//         if let Ok(error_response) = error_response {
//             return Err(error_response.message);
//         } else {
//             return Err(format!("API error: {}", response.status()));
//         }
//     }

//     Ok(())
// }
