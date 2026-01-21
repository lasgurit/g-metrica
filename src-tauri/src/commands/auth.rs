use crate::models::ApiResponse;
use crate::services::supabase::{
    SupabaseClient, AuthResponse, LoginCredentials, SignupCredentials
};
use tauri::State;

// ============================================================================
// COMANDOS DE AUTENTICACIÓN
// ============================================================================

/// Iniciar sesión
#[tauri::command]
pub async fn login(
    supabase: State<'_, SupabaseClient>,
    email: String,
    password: String,
) -> Result<ApiResponse<AuthResponse>, String> {
    let credentials = LoginCredentials { email, password };
    
    let auth_response = supabase.login(credentials)
        .await
        .map_err(|e| e.to_string())?;

    // Después del login exitoso, crear o actualizar el perfil
    let _ = update_ultima_sesion(&supabase).await;

    Ok(ApiResponse::success(auth_response))
}

/// Registrar nuevo usuario
#[tauri::command]
pub async fn signup(
    supabase: State<'_, SupabaseClient>,
    email: String,
    password: String,
    nombre: String,
    empresa: Option<String>,
) -> Result<ApiResponse<AuthResponse>, String> {
    let credentials = SignupCredentials {
        email,
        password,
        nombre: nombre.clone(),
        empresa: empresa.clone(),
    };
    
    let auth_response = supabase.signup(credentials)
        .await
        .map_err(|e| e.to_string())?;

    // Crear el perfil del usuario
    let user_id = auth_response.user.id.clone();
    let _ = create_perfil(&supabase, user_id, nombre, empresa).await;

    Ok(ApiResponse::success(auth_response))
}

/// Cerrar sesión
#[tauri::command]
pub async fn logout(
    supabase: State<'_, SupabaseClient>,
) -> Result<ApiResponse<bool>, String> {
    supabase.logout()
        .await
        .map_err(|e| e.to_string())?;

    Ok(ApiResponse::success(true))
}

/// Verificar si el usuario está autenticado
#[tauri::command]
pub fn is_authenticated(
    supabase: State<'_, SupabaseClient>,
) -> Result<ApiResponse<bool>, String> {
    let is_auth = supabase.is_authenticated();
    Ok(ApiResponse::success(is_auth))
}

/// Establecer token manualmente (para restaurar sesión)
#[tauri::command]
pub fn set_auth_token(
    supabase: State<'_, SupabaseClient>,
    token: String,
) -> Result<ApiResponse<bool>, String> {
    supabase.set_token(token);
    Ok(ApiResponse::success(true))
}

/// Obtener el token actual
#[tauri::command]
pub fn get_auth_token(
    supabase: State<'_, SupabaseClient>,
) -> Result<ApiResponse<Option<String>>, String> {
    let token = supabase.get_token();
    Ok(ApiResponse::success(token))
}

// ============================================================================
// FUNCIONES AUXILIARES
// ============================================================================

/// Crear perfil de usuario después del registro
async fn create_perfil(
    supabase: &SupabaseClient,
    user_id: String,
    nombre: String,
    empresa: Option<String>,
) -> Result<(), String> {
    let client = supabase.table("perfiles")
        .map_err(|e| e.to_string())?;

    let perfil_data = serde_json::json!({
        "id": user_id,
        "nombre": nombre,
        "empresa": empresa,
        "activo": true
    });

    client
        .insert(perfil_data.to_string())
        .execute()
        .await
        .map_err(|e| e.to_string())?;

    Ok(())
}

/// Actualizar última sesión del usuario
async fn update_ultima_sesion(
    supabase: &SupabaseClient,
) -> Result<(), String> {
    let client = supabase.table("perfiles")
        .map_err(|e| e.to_string())?;

    let now = chrono::Utc::now().to_rfc3339();

    client
        .update(serde_json::json!({
            "ultima_sesion": now
        }).to_string())
        .execute()
        .await
        .map_err(|e| e.to_string())?;

    Ok(())
}