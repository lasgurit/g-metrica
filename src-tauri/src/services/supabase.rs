use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION, CONTENT_TYPE};
 
const SUPABASE_URL: &str = env!("SUPABASE_URL");
const SUPABASE_ANON_KEY: &str = env!("SUPABASE_ANON_KEY");

pub struct SupabaseClient {
    pub client: reqwest::Client,
}
 
impl SupabaseClient {
    pub fn new() -> Self {
        SupabaseClient {
            client: reqwest::Client::new(),
        }
    }
 
    /// Construye los headers HTTP para una request a Supabase.
    /// Si se pasa un token JWT de usuario, se usa como Bearer.
    /// Si no, se usa la anon key como Bearer (para endpoints públicos).
    pub fn headers(&self, auth_token: Option<&str>) -> HeaderMap {
        let mut headers = HeaderMap::new();
 
        headers.insert("apikey", HeaderValue::from_static(SUPABASE_ANON_KEY));
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
 
        let bearer = match auth_token {
            Some(token) => format!("Bearer {}", token),
            None => format!("Bearer {}", SUPABASE_ANON_KEY),
        };
        headers.insert(AUTHORIZATION, HeaderValue::from_str(&bearer).unwrap());
 
        headers
    }
 
    pub fn rest_url(&self, table: &str) -> String {
        format!("{}/rest/v1/{}", SUPABASE_URL, table)
    }
 
    pub fn auth_url(&self, endpoint: &str) -> String {
        format!("{}/auth/v1/{}", SUPABASE_URL, endpoint)
    }
}
