use std::sync::Arc;
use reqwest::{cookie::Jar, Url, header};
use reqwest_middleware::ClientWithMiddleware;
use crate::{
    APIError,
    request,
    response,
    api_helpers::{
        get_default_middleware,
        parses_response
    }
};
use std::fmt::{self, Write};
use lazy_regex::regex_captures;
use steamid_ng::SteamID;
use serde::{Deserialize, Serialize};
use rand::Rng;

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
    
    pub async fn authenticate_user<'a>(&self, steamid: &'a SteamID, sessionkey: &'a [u8], encrypted_loginkey: &'a [u8]) -> Result<(String, Vec<String>), APIError> {
        #[derive(Deserialize, Debug)]
        struct Response {
            authenticateuser: response::AuthenticateUser,
        }
        
        fn bytes_to_string(bytes: &[u8]) -> Result<String, fmt::Error> {
            let mut s = String::with_capacity(bytes.len() * 3);
            
            for &b in bytes {
                write!(&mut s, "%{:02x}", b)?;
            }
            
            Ok(s)
        }
        
        fn generate_sessionid() -> Result<String, fmt::Error> {
            fn bytes_to_string(bytes: &[u8]) -> Result<String, fmt::Error> {
                let mut s = String::with_capacity(bytes.len() * 2);
                
                for &b in bytes {
                    write!(&mut s, "{:02x}", b)?;
                }
                
                Ok(s)
            }
            
            bytes_to_string(&rand::thread_rng().gen::<[u8; 12]>())
        }
        
        let query = vec![
            ("steamid", u64::from(steamid.clone()).to_string()),
            ("sessionkey", bytes_to_string(sessionkey)?),
            ("encrypted_loginkey", bytes_to_string(encrypted_loginkey)?),
        ];
        let body = query
            .iter()
            .map(|(a, b)| format!("{}={}", a, b))
            .collect::<Vec<String>>()
            .join("&");
        let uri = self.get_api_url("ISteamUserAuth", "AuthenticateUser", 1);
        let response = self.client.post(uri)
            .header(header::CONTENT_TYPE, "application/x-www-form-urlencoded")
            .header(header::CONTENT_LENGTH, body.len())
            .body(body)
            .send()
            .await?;
        let body: Response = parses_response(response).await?;
        let sessionid = generate_sessionid()?;
        let cookies: Vec<_> = vec![
            format!("sessionid={}", sessionid),
            format!("steamLogin={}", body.authenticateuser.token),
            format!("steamLoginSecure={}", body.authenticateuser.tokensecure),
        ];
        
        Ok((sessionid, cookies))
    }
}