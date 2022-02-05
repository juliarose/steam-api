
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct AuthenticateUser {
    pub token: String,
    pub tokensecure: String,
}