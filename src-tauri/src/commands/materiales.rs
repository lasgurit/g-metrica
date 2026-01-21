use crate::models::{ApiResponse, Material, MaterialInput};
use crate::services::supabase::SupabaseClient;
use serde_json::json;
use tauri::State;
use uuid::Uuid;

// ============================================================================
// COMANDOS DE MATERIALES
// ============================================================================

/// Obtener todos los materiales (sistema + del usuario)
#[tauri::command]
pub async fn get_materiales(
    supabase: State<'_, SupabaseClient>,
) -> Result<ApiResponse<Vec<Material>>, String> {
    let client = supabase.table("materiales")
        .map_err(|e| e.to_string())?;

    let response = client
        .select("*")
        .eq("activo", "true")
        .order("categoria,nombre")
        .execute()
        .await
        .map_err(|e| e.to_string())?;

    let materiales: Vec<Material> = response
        .json()
        .await
        .map_err(|e| e.to_string())?;

    Ok(ApiResponse::success(materiales))
}

/// Obtener solo materiales del sistema
#[tauri::command]
pub async fn get_materiales_sistema(
    supabase: State<'_, SupabaseClient>,
) -> Result<ApiResponse<Vec<Material>>, String> {
    let client = supabase.table("materiales")
        .map_err(|e| e.to_string())?;

    let response = client
        .select("*")
        .eq("es_sistema", "true")
        .eq("activo", "true")
        .order("categoria,nombre")
        .execute()
        .await
        .map_err(|e| e.to_string())?;

    let materiales: Vec<Material> = response
        .json()
        .await
        .map_err(|e| e.to_string())?;

    Ok(ApiResponse::success(materiales))
}

/// Obtener solo materiales del usuario
#[tauri::command]
pub async fn get_materiales_usuario(
    supabase: State<'_, SupabaseClient>,
) -> Result<ApiResponse<Vec<Material>>, String> {
    let client = supabase.table("materiales")
        .map_err(|e| e.to_string())?;

    let response = client
        .select("*")
        .eq("es_sistema", "false")
        .eq("activo", "true")
        .order("categoria,nombre")
        .execute()
        .await
        .map_err(|e| e.to_string())?;

    let materiales: Vec<Material> = response
        .json()
        .await
        .map_err(|e| e.to_string())?;

    Ok(ApiResponse::success(materiales))
}

/// Obtener un material específico por ID
#[tauri::command]
pub async fn get_material(
    supabase: State<'_, SupabaseClient>,
    material_id: String,
) -> Result<ApiResponse<Material>, String> {
    let uuid = Uuid::parse_str(&material_id)
        .map_err(|e| format!("UUID inválido: {}", e))?;

    let client = supabase.table("materiales")
        .map_err(|e| e.to_string())?;

    let response = client
        .select("*")
        .eq("id", uuid.to_string())
        .single()
        .execute()
        .await
        .map_err(|e| e.to_string())?;

    let material: Material = response
        .json()
        .await
        .map_err(|e| e.to_string())?;

    Ok(ApiResponse::success(material))
}

/// Buscar material por código
#[tauri::command]
pub async fn get_material_by_codigo(
    supabase: State<'_, SupabaseClient>,
    codigo: String,
) -> Result<ApiResponse<Option<Material>>, String> {
    let client = supabase.table("materiales")
        .map_err(|e| e.to_string())?;

    let response = client
        .select("*")
        .eq("codigo", codigo)
        .eq("activo", "true")
        .limit(1)
        .execute()
        .await
        .map_err(|e| e.to_string())?;

    let materiales: Vec<Material> = response
        .json()
        .await
        .map_err(|e| e.to_string())?;

    Ok(ApiResponse::success(materiales.into_iter().next()))
}

/// Crear un nuevo material personalizado
#[tauri::command]
pub async fn create_material(
    supabase: State<'_, SupabaseClient>,
    material_input: MaterialInput,
) -> Result<ApiResponse<Material>, String> {
    let client = supabase.table("materiales")
        .map_err(|e| e.to_string())?;

    let response = client
        .insert(json!({
            "codigo": material_input.codigo,
            "nombre": material_input.nombre,
            "unidad": material_input.unidad,
            "categoria": material_input.categoria,
            "descripcion": material_input.descripcion,
            "precio_referencia": material_input.precio_referencia,
            "es_sistema": false,
        }).to_string())
        .execute()
        .await
        .map_err(|e| e.to_string())?;

    let material: Material = response
        .json()
        .await
        .map_err(|e| e.to_string())?;

    Ok(ApiResponse::success(material))
}

/// Actualizar un material personalizado
#[tauri::command]
pub async fn update_material(
    supabase: State<'_, SupabaseClient>,
    material_id: String,
    material_input: MaterialInput,
) -> Result<ApiResponse<Material>, String> {
    let uuid = Uuid::parse_str(&material_id)
        .map_err(|e| format!("UUID inválido: {}", e))?;

    let client = supabase.table("materiales")
        .map_err(|e| e.to_string())?;

    let response = client
        .update(json!({
            "codigo": material_input.codigo,
            "nombre": material_input.nombre,
            "unidad": material_input.unidad,
            "categoria": material_input.categoria,
            "descripcion": material_input.descripcion,
            "precio_referencia": material_input.precio_referencia,
        }).to_string())
        .eq("id", uuid.to_string())
        .execute()
        .await
        .map_err(|e| e.to_string())?;

    let material: Material = response
        .json()
        .await
        .map_err(|e| e.to_string())?;

    Ok(ApiResponse::success(material))
}

/// Eliminar un material personalizado (soft delete)
#[tauri::command]
pub async fn delete_material(
    supabase: State<'_, SupabaseClient>,
    material_id: String,
) -> Result<ApiResponse<bool>, String> {
    let uuid = Uuid::parse_str(&material_id)
        .map_err(|e| format!("UUID inválido: {}", e))?;

    let client = supabase.table("materiales")
        .map_err(|e| e.to_string())?;

    client
        .update(json!({"activo": false}).to_string())
        .eq("id", uuid.to_string())
        .execute()
        .await
        .map_err(|e| e.to_string())?;

    Ok(ApiResponse::success(true))
}

/// Buscar materiales por categoría
#[tauri::command]
pub async fn get_materiales_by_categoria(
    supabase: State<'_, SupabaseClient>,
    categoria: String,
) -> Result<ApiResponse<Vec<Material>>, String> {
    let client = supabase.table("materiales")
        .map_err(|e| e.to_string())?;

    let response = client
        .select("*")
        .eq("categoria", categoria)
        .eq("activo", "true")
        .order("nombre")
        .execute()
        .await
        .map_err(|e| e.to_string())?;

    let materiales: Vec<Material> = response
        .json()
        .await
        .map_err(|e| e.to_string())?;

    Ok(ApiResponse::success(materiales))
}