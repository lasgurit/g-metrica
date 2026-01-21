use postgrest::Postgrest;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum SupabaseError {
    #[error("Error de red: {0}")]
    NetworkError(#[from] reqwest::Error),
    
    #[error("Error de autenticación: {0}")]
    AuthError(String),
    
    #[error("Error de base de datos: {0}")]
    DatabaseError(String),
    
    #[error("Usuario no autenticado")]
    Unauthorized,
    
    #[error("Error al parsear JSON: {0}")]
    JsonError(#[from] serde_json::Error),
}

pub type Result<T> = std::result::Result<T, SupabaseError>;

// ============================================================================
// ESTRUCTURAS DE AUTENTICACIÓN
// ============================================================================
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthResponse {
    pub access_token: String,
    pub token_type: String,
    pub expires_in: u64,
    pub refresh_token: String,
    pub user: User,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: String,
    pub email: String,
    pub email_confirmed_at: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoginCredentials {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignupCredentials {
    pub email: String,
    pub password: String,
    pub nombre: String,
    pub empresa: Option<String>,
}

// ============================================================================
// CLIENTE DE SUPABASE
// ============================================================================
#[derive(Clone)]
pub struct SupabaseClient {
    pub url: String,
    pub anon_key: String,
    client: Client,
    access_token: Arc<Mutex<Option<String>>>,
}

impl SupabaseClient {
    pub fn new(url: String, anon_key: String) -> Self {
        Self {
            url,
            anon_key,
            client: Client::new(),
            access_token: Arc::new(Mutex::new(None)),
        }
    }

    // ========================================================================
    // MÉTODOS DE AUTENTICACIÓN
    // ========================================================================
    
    /// Iniciar sesión con email y contraseña
    pub async fn login(&self, credentials: LoginCredentials) -> Result<AuthResponse> {
        let auth_url = format!("{}/auth/v1/token?grant_type=password", self.url);
        
        let response = self.client
            .post(&auth_url)
            .header("apikey", &self.anon_key)
            .header("Content-Type", "application/json")
            .json(&credentials)
            .send()
            .await?;

        if !response.status().is_success() {
            let error_text = response.text().await?;
            return Err(SupabaseError::AuthError(error_text));
        }

        let auth_response: AuthResponse = response.json().await?;
        
        // Guardar el token
        let mut token = self.access_token.lock().unwrap();
        *token = Some(auth_response.access_token.clone());

        Ok(auth_response)
    }

    /// Registrar nuevo usuario
    pub async fn signup(&self, credentials: SignupCredentials) -> Result<AuthResponse> {
        let auth_url = format!("{}/auth/v1/signup", self.url);
        
        let payload = serde_json::json!({
            "email": credentials.email,
            "password": credentials.password,
            "data": {
                "nombre": credentials.nombre,
                "empresa": credentials.empresa
            }
        });

        let response = self.client
            .post(&auth_url)
            .header("apikey", &self.anon_key)
            .header("Content-Type", "application/json")
            .json(&payload)
            .send()
            .await?;

        if !response.status().is_success() {
            let error_text = response.text().await?;
            return Err(SupabaseError::AuthError(error_text));
        }

        let auth_response: AuthResponse = response.json().await?;
        
        // Guardar el token
        let mut token = self.access_token.lock().unwrap();
        *token = Some(auth_response.access_token.clone());

        Ok(auth_response)
    }

    /// Cerrar sesión
    pub async fn logout(&self) -> Result<()> {
        let mut token = self.access_token.lock().unwrap();
        *token = None;
        Ok(())
    }

    /// Obtener el token actual
    pub fn get_token(&self) -> Option<String> {
        let token = self.access_token.lock().unwrap();
        token.clone()
    }

    /// Establecer token manualmente (útil para restaurar sesiones)
    pub fn set_token(&self, token: String) {
        let mut current_token = self.access_token.lock().unwrap();
        *current_token = Some(token);
    }

    // ========================================================================
    // MÉTODOS PARA INTERACTUAR CON LA BASE DE DATOS
    // ========================================================================
    
    /// Obtener cliente de Postgrest para una tabla específica
    pub fn table(&self, table_name: &str) -> Result<Postgrest> {
        let token = self.get_token()
            .ok_or(SupabaseError::Unauthorized)?;

        let rest_url = format!("{}/rest/v1/{}", self.url, table_name);
        
        let client = Postgrest::new(rest_url)
            .insert_header("apikey", &self.anon_key)
            .insert_header("Authorization", format!("Bearer {}", token));

        Ok(client)
    }

    /// Ejecutar una función RPC de Supabase
    pub async fn rpc(&self, function_name: &str, params: serde_json::Value) -> Result<String> {
        let token = self.get_token()
            .ok_or(SupabaseError::Unauthorized)?;

        let rpc_url = format!("{}/rest/v1/rpc/{}", self.url, function_name);
        
        let response = self.client
            .post(&rpc_url)
            .header("apikey", &self.anon_key)
            .header("Authorization", format!("Bearer {}", token))
            .header("Content-Type", "application/json")
            .json(&params)
            .send()
            .await?;

        if !response.status().is_success() {
            let error_text = response.text().await?;
            return Err(SupabaseError::DatabaseError(error_text));
        }

        Ok(response.text().await?)
    }

    /// Verificar si el usuario está autenticado
    pub fn is_authenticated(&self) -> bool {
        self.get_token().is_some()
    }
}

// ============================================================================
// BUILDER PARA CONFIGURACIÓN
// ============================================================================
pub struct SupabaseClientBuilder {
    url: Option<String>,
    anon_key: Option<String>,
}

impl SupabaseClientBuilder {
    pub fn new() -> Self {
        Self {
            url: None,
            anon_key: None,
        }
    }

    pub fn url(mut self, url: String) -> Self {
        self.url = Some(url);
        self
    }

    pub fn anon_key(mut self, anon_key: String) -> Self {
        self.anon_key = Some(anon_key);
        self
    }

    pub fn build(self) -> std::result::Result<SupabaseClient, String> {
        let url = self.url.ok_or("URL de Supabase no proporcionada")?;
        let anon_key = self.anon_key.ok_or("Anon key de Supabase no proporcionada")?;

        Ok(SupabaseClient::new(url, anon_key))
    }
}

impl Default for SupabaseClientBuilder {
    fn default() -> Self {
        Self::new()
    }
}