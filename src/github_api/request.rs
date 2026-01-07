
use reqwest::{Client, Method, RequestBuilder, header::{self, HeaderMap, HeaderValue}};

#[derive(Default)]
pub struct GitHubAPI {
    pub url: String,
    pub client: Client
}

impl GitHubAPI {
    pub fn init(github_token: &str) -> Self {
        let user_agent = "git-tools";
        let accept = "application/vnd.github+json";
        let url = "https://api.github.com";
        
        let bearer = format!("Bearer {}", github_token);
        
        let mut default_header = HeaderMap::new();
        default_header.insert(header::ACCEPT, HeaderValue::from_str(accept).unwrap());
        default_header.insert(header::AUTHORIZATION, HeaderValue::from_str(&bearer).unwrap());
        default_header.insert(header::USER_AGENT, HeaderValue::from_str(user_agent).unwrap());
        default_header.insert("X-GitHub-Api-Version", HeaderValue::from_str("2022-11-28").unwrap());

        let client_builder = Client::builder();
        let client = client_builder
            .default_headers(default_header)
            .build()
            .unwrap();
        
        Self { url: url.to_owned(), client: client }
    }

    pub fn request(&self, end_point: &str, method: Method) -> RequestBuilder {
        let url = format!("{}{}", &self.url, end_point);
        match method {
            Method::GET => self.client.get(url),
            _ => self.client.get(url)
        }
    }
}