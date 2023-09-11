use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct UserForCreate {
    pub username: String,
    pub pwd_clear: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct LoginPayload {
    pub username: String,
    pub pwd: String,
}
