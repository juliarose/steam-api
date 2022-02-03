use std::sync::Arc;
use reqwest::{cookie::Jar, Url};
use reqwest_middleware::ClientWithMiddleware;
use crate::APIError;
use crate::api_helpers::{
    get_default_middleware,
    parses_response
};
use lazy_regex::regex_captures;

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
    
    pub async fn authenticate_user(&self) -> Result<(), APIError> {
        Ok(())
    }
}