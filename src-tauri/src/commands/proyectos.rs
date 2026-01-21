use crate::models::{ApiResponse, Proyecto, ProyectoInput, ProyectoUpdate, ResumenMaterial};
use crate::services::supabase::SupabaseClient;
use serde_json::json;
use tauri::State;
use uuid::Uuid;

// ============================================================================
// COMANDOS DE PROYECTOS
// ============================================================================

/// Obtener todos los proyectos del usuario
#[tauri::command]
pub async fn get_proyectos(
    supabase: State<'_, SupabaseClient>,
) -> Result<ApiResponse<Vec<Proyecto>>, String> {
    let client = supabase.table("proyectos")
        .map_err(|e| e.to_string())?;

    let response = client
        .select("*")
        .order("created_at.desc")
        .execute()
        .await
        .map_err(|e| e.to_string())?;

    let proyectos: Vec<Proyecto> = response
        .json()
        .await
        .map_err(|e| e.to_string())?;

    Ok(ApiResponse::success(proyectos))
}

/// Obtener un proyecto específico por ID
#[tauri::command]
pub async fn get_proyecto(
    supabase: State<'_, SupabaseClient>,
    proyecto_id: String,
) -> Result<ApiResponse<Proyecto>, String> {
    let uuid = Uuid::parse_str(&proyecto_id)
        .map_err(|e| format!("UUID inválido: {}", e))?;

    let client = supabase.table("proyectos")
        .map_err(|e| e.to_string())?;

    let response = client
        .select("*")
        .eq("id", uuid.to_string())
        .single()
        .execute()
        .await
        .map_err(|e| e.to_string())?;

    let proyecto: Proyecto = response
        .json()
        .await
        .map_err(|e| e.to_string())?;

    Ok(ApiResponse::success(proyecto))
}

/// Crear un nuevo proyecto
#[tauri::command]
pub async fn create_proyecto(
    supabase: State<'_, SupabaseClient>,
    proyecto_input: ProyectoInput,
) -> Result<ApiResponse<Proyecto>, String> {
    let client = supabase.table("proyectos")
        .map_err(|e| e.to_string())?;

    let response = client
        .insert(json!({
            "nombre": proyecto_input.nombre,
            "descripcion": proyecto_input.descripcion,
            "ubicacion": proyecto_input.ubicacion,
        }).to_string())
        .execute()
        .await
        .map_err(|e| e.to_string())?;

    let proyecto: Proyecto = response
        .json()
        .await
        .map_err(|e| e.to_string())?;

    Ok(ApiResponse::success(proyecto))
}

/// Actualizar un proyecto existente
#[tauri::command]
pub async fn update_proyecto(
    supabase: State<'_, SupabaseClient>,
    proyecto_id: String,
    proyecto_update: ProyectoUpdate,
) -> Result<ApiResponse<Proyecto>, String> {
    let uuid = Uuid::parse_str(&proyecto_id)
        .map_err(|e| format!("UUID inválido: {}", e))?;

    let mut update_data = json!({});
    
    if let Some(nombre) = proyecto_update.nombre {
        update_data["nombre"] = json!(nombre);
    }
    if let Some(descripcion) = proyecto_update.descripcion {
        update_data["descripcion"] = json!(descripcion);
    }
    if let Some(ubicacion) = proyecto_update.ubicacion {
        update_data["ubicacion"] = json!(ubicacion);
    }
    if let Some(activo) = proyecto_update.activo {
        update_data["activo"] = json!(activo);
    }

    let client = supabase.table("proyectos")
        .map_err(|e| e.to_string())?;

    let response = client
        .update(update_data.to_string())
        .eq("id", uuid.to_string())
        .execute()
        .await
        .map_err(|e| e.to_string())?;

    let proyecto: Proyecto = response
        .json()
        .await
        .map_err(|e| e.to_string())?;

    Ok(ApiResponse::success(proyecto))
}

/// Eliminar un proyecto (soft delete)
#[tauri::command]
pub async fn delete_proyecto(
    supabase: State<'_, SupabaseClient>,
    proyecto_id: String,
) -> Result<ApiResponse<bool>, String> {
    let uuid = Uuid::parse_str(&proyecto_id)
        .map_err(|e| format!("UUID inválido: {}", e))?;

    let client = supabase.table("proyectos")
        .map_err(|e| e.to_string())?;

    client
        .update(json!({"activo": false}).to_string())
        .eq("id", uuid.to_string())
        .execute()
        .await
        .map_err(|e| e.to_string())?;

    Ok(ApiResponse::success(true))
}

/// Eliminar permanentemente un proyecto
#[tauri::command]
pub async fn hard_delete_proyecto(
    supabase: State<'_, SupabaseClient>,
    proyecto_id: String,
) -> Result<ApiResponse<bool>, String> {
    let uuid = Uuid::parse_str(&proyecto_id)
        .map_err(|e| format!("UUID inválido: {}", e))?;

    let client = supabase.table("proyectos")
        .map_err(|e| e.to_string())?;

    client
        .delete()
        .eq("id", uuid.to_string())
        .execute()
        .await
        .map_err(|e| e.to_string())?;

    Ok(ApiResponse::success(true))
}

/// Obtener resumen de materiales de un proyecto usando la función RPC
#[tauri::command]
pub async fn get_resumen_proyecto(
    supabase: State<'_, SupabaseClient>,
    proyecto_id: String,
) -> Result<ApiResponse<Vec<ResumenMaterial>>, String> {
    let uuid = Uuid::parse_str(&proyecto_id)
        .map_err(|e| format!("UUID inválido: {}", e))?;

    let response = supabase.rpc(
        "obtener_resumen_proyecto",
        json!({ "proyecto_uuid": uuid })
    )
    .await
    .map_err(|e| e.to_string())?;

    let materiales: Vec<ResumenMaterial> = serde_json::from_str(&response)
        .map_err(|e| e.to_string())?;

    Ok(ApiResponse::success(materiales))
}