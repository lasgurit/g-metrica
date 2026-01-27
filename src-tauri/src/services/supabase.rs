use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION, CONTENT_TYPE};
use std::env;

pub struct SupabaseClient {
    pub client: reqwest::Client,
    pub base_url: String,
    pub api_key: String,
}

impl SupabaseClient {
    pub fn new() -> Self {
        dotenv::dotenv().ok();
        
        let base_url = env::var("SUPABASE_URL")
            .expect("SUPABASE_URL debe estar configurada");
        let api_key = env::var("SUPABASE_ANON_KEY")
            .expect("SUPABASE_ANON_KEY debe estar configurada");

        let client = reqwest::Client::new();

        SupabaseClient {
            client,
            base_url,
            api_key,
        }
    }

    pub fn headers(&self, auth_token: Option<&str>) -> HeaderMap {
        let mut headers = HeaderMap::new();
        
        headers.insert(
            "apikey",
            HeaderValue::from_str(&self.api_key).unwrap()
        );
        
        headers.insert(
            CONTENT_TYPE,
            HeaderValue::from_static("application/json")
        );

        // Si hay token de usuario autenticado, lo usamos
        // Si no, usamos la api_key como Bearer
        let bearer = match auth_token {
            Some(token) => format!("Bearer {}", token),
            None => format!("Bearer {}", self.api_key),
        };
        
        headers.insert(
            AUTHORIZATION,
            HeaderValue::from_str(&bearer).unwrap()
        );

        headers
    }

    pub fn rest_url(&self, table: &str) -> String {
        format!("{}/rest/v1/{}", self.base_url, table)
    }

    pub fn auth_url(&self, endpoint: &str) -> String {
        format!("{}/auth/v1/{}", self.base_url, endpoint)
    }
}
