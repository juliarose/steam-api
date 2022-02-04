
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct AuthenticateUser {
    token: String,
    tokensecure: String,
}