use crate::models::usuario::{AuthResponse, LoginCredentials, Perfil, RegisterInput};
use crate::services::supabase::SupabaseClient;
use serde_json::json;

#[tauri::command]
pub async fn login(email: String, password: String) -> Result<AuthResponse, String> {
    let client = SupabaseClient::new();
    
    let response = client.client
        .post(&client.auth_url("token?grant_type=password"))
        .headers(client.headers(None))
        .json(&LoginCredentials { email, password })
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if !response.status().is_success() {
        let error_text = response.text().await.unwrap_or_default();
        return Err(format!("Error de autenticación: {}", error_text));
    }

    let auth_response: AuthResponse = response
        .json()
        .await
        .map_err(|e| e.to_string())?;

    Ok(auth_response)
}

#[tauri::command]
pub async fn register(
    email: String,
    password: String,
    nombre: String,
    empresa: Option<String>
) -> Result<AuthResponse, String> {
    let client = SupabaseClient::new();
    
    // 1. Registrar usuario en auth
    let auth_response = client.client
        .post(&client.auth_url("signup"))
        .headers(client.headers(None))
        .json(&json!({
            "email": email,
            "password": password
        }))
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if !auth_response.status().is_success() {
        let error_text = auth_response.text().await.unwrap_or_default();
        return Err(format!("Error al registrar: {}", error_text));
    }

    let auth_data: AuthResponse = auth_response
        .json()
        .await
        .map_err(|e| e.to_string())?;

    // 2. Crear perfil en tabla perfiles
    let _perfil_response = client.client
        .post(&client.rest_url("perfiles"))
        .headers(client.headers(Some(&auth_data.access_token)))
        .json(&json!({
            "id": auth_data.user.id,
            "nombre": nombre,
            "empresa": empresa
        }))
        .send()
        .await
        .map_err(|e| e.to_string())?;

    Ok(auth_data)
}

#[tauri::command]
pub async fn logout() -> Result<(), String> {
    // En el frontend se limpia el token guardado
    Ok(())
}

#[tauri::command]
pub async fn get_perfil(token: String, user_id: String) -> Result<Perfil, String> {
    let client = SupabaseClient::new();
    
    let response = client.client
        .get(&format!("{}?id=eq.{}", client.rest_url("perfiles"), user_id))
        .headers(client.headers(Some(&token)))
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if !response.status().is_success() {
        let error_text = response.text().await.unwrap_or_default();
        return Err(format!("Error al obtener perfil: {}", error_text));
    }

    let perfiles: Vec<Perfil> = response
        .json()
        .await
        .map_err(|e| e.to_string())?;

    perfiles.into_iter().next()
        .ok_or_else(|| "Perfil no encontrado".to_string())
}
