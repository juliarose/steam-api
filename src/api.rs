use std::sync::Arc;
use reqwest::{cookie::Jar, Url};
use reqwest_middleware::ClientWithMiddleware;
use crate::{
    APIError,
    request::{
        self,
        serializers::{
            steamid_as_string
        }
    },
    response,
    api_helpers::{
        get_default_middleware,
        parses_response
    }
};
use lazy_regex::regex_captures;
use steamid_ng::SteamID;
use serde::Serialize;

#[derive(Debug)]
pub struct SteamAPI {
    pub cookies: Arc<Jar>,
    client: ClientWithMiddleware,
    pub sessionid: Option<String>,
}

const HOSTNAME: &'static str = "api.steampowered.com";

impl SteamAPI {
    
    pub fn new() -> Self {
        let cookies = Arc::new(Jar::default());

        Self {
            cookies: Arc::clone(&cookies),
            client: get_default_middleware(Arc::clone(&cookies)),
            sessionid: None,
        }
    }
    
    fn get_uri(&self, pathname: &str) -> String {
        format!("https://{}{}", HOSTNAME, pathname)
    }

    fn get_api_url(&self, interface: &str, method: &str, version: usize) -> String {
        format!("https://{}/{}/{}/v{}", HOSTNAME, interface, method, version)
    }
    
    pub fn set_cookie(&mut self, cookie_str: &str) {
        if let Ok(url) = HOSTNAME.parse::<Url>() {
            self.cookies.add_cookie_str(cookie_str, &url);
            
            if let Some((_, sessionid)) = regex_captures!(r#"sessionid=([A-z0-9]+)"#, cookie_str) {
                self.sessionid = Some(String::from(sessionid));
            }
        }
    }
    
    pub fn set_cookies(&mut self, cookies: &Vec<String>) {
        for cookie_str in cookies {
            self.set_cookie(cookie_str)
        }
    }
    
    pub async fn authenticate_user<'a>(&self, steamid: &'a SteamID, sessionkey: &'a str, encrypted_loginkey: &'a str) -> Result<response::AuthenticateUser, APIError> {        
        #[derive(Serialize, Debug)]
        struct AuthicateUserParams<'a> {
            #[serde(serialize_with = "steamid_as_string")]
            steamid: &'a SteamID,
            sessionkey: &'a str,
            encrypted_loginkey: &'a str,
        }
        
        let uri = self.get_api_url("ISteamUserAuth", "AuthenticateUser", 1);
        let response = self.client.post(uri)
            .form(&AuthicateUserParams {
                steamid,
                sessionkey,
                encrypted_loginkey,
            })
            .send()
            .await?;
        let body: response::AuthenticateUser = parses_response(response).await?;
        
        Ok(body)
    }
}