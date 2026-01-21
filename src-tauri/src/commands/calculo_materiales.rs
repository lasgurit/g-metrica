use crate::models::{ApiResponse, CalculoMaterial, CalculoMaterialInput, CalculoMaterialUpdate};
use crate::services::supabase::SupabaseClient;
use serde_json::json;
use tauri::State;
use uuid::Uuid;

// ============================================================================
// COMANDOS DE CÁLCULO-MATERIALES
// ============================================================================

/// Obtener todos los materiales de un cálculo
#[tauri::command]
pub async fn get_materiales_calculo(
    supabase: State<'_, SupabaseClient>,
    calculo_id: String,
) -> Result<ApiResponse<Vec<CalculoMaterial>>, String> {
    let uuid = Uuid::parse_str(&calculo_id)
        .map_err(|e| format!("UUID inválido: {}", e))?;

    let client = supabase.table("calculo_materiales")
        .map_err(|e| e.to_string())?;

    let response = client
        .select("*")
        .eq("calculo_id", uuid.to_string())
        .execute()
        .await
        .map_err(|e| e.to_string())?;

    let materiales: Vec<CalculoMaterial> = response
        .json()
        .await
        .map_err(|e| e.to_string())?;

    Ok(ApiResponse::success(materiales))
}

/// Agregar un material a un cálculo
#[tauri::command]
pub async fn add_material_to_calculo(
    supabase: State<'_, SupabaseClient>,
    calculo_material_input: CalculoMaterialInput,
) -> Result<ApiResponse<CalculoMaterial>, String> {
    let client = supabase.table("calculo_materiales")
        .map_err(|e| e.to_string())?;

    let response = client
        .insert(json!({
            "calculo_id": calculo_material_input.calculo_id,
            "material_id": calculo_material_input.material_id,
            "cantidad": calculo_material_input.cantidad,
        }).to_string())
        .execute()
        .await
        .map_err(|e| e.to_string())?;

    let calculo_material: CalculoMaterial = response
        .json()
        .await
        .map_err(|e| e.to_string())?;

    Ok(ApiResponse::success(calculo_material))
}

/// Actualizar la cantidad de un material en un cálculo
#[tauri::command]
pub async fn update_cantidad_material(
    supabase: State<'_, SupabaseClient>,
    calculo_material_id: String,
    cantidad_editada: f64,
) -> Result<ApiResponse<CalculoMaterial>, String> {
    let uuid = Uuid::parse_str(&calculo_material_id)
        .map_err(|e| format!("UUID inválido: {}", e))?;

    let client = supabase.table("calculo_materiales")
        .map_err(|e| e.to_string())?;

    let response = client
        .update(json!({
            "cantidad_editada": cantidad_editada,
            "fue_editado": true,
        }).to_string())
        .eq("id", uuid.to_string())
        .execute()
        .await
        .map_err(|e| e.to_string())?;

    let calculo_material: CalculoMaterial = response
        .json()
        .await
        .map_err(|e| e.to_string())?;

    Ok(ApiResponse::success(calculo_material))
}

/// Restablecer la cantidad original de un material
#[tauri::command]
pub async fn reset_cantidad_material(
    supabase: State<'_, SupabaseClient>,
    calculo_material_id: String,
) -> Result<ApiResponse<CalculoMaterial>, String> {
    let uuid = Uuid::parse_str(&calculo_material_id)
        .map_err(|e| format!("UUID inválido: {}", e))?;

    let client = supabase.table("calculo_materiales")
        .map_err(|e| e.to_string())?;

    let response = client
        .update(json!({
            "cantidad_editada": null,
            "fue_editado": false,
        }).to_string())
        .eq("id", uuid.to_string())
        .execute()
        .await
        .map_err(|e| e.to_string())?;

    let calculo_material: CalculoMaterial = response
        .json()
        .await
        .map_err(|e| e.to_string())?;

    Ok(ApiResponse::success(calculo_material))
}

/// Eliminar un material de un cálculo
#[tauri::command]
pub async fn remove_material_from_calculo(
    supabase: State<'_, SupabaseClient>,
    calculo_material_id: String,
) -> Result<ApiResponse<bool>, String> {
    let uuid = Uuid::parse_str(&calculo_material_id)
        .map_err(|e| format!("UUID inválido: {}", e))?;

    let client = supabase.table("calculo_materiales")
        .map_err(|e| e.to_string())?;

    client
        .delete()
        .eq("id", uuid.to_string())
        .execute()
        .await
        .map_err(|e| e.to_string())?;

    Ok(ApiResponse::success(true))
}

/// Agregar múltiples materiales a un cálculo de una vez
#[tauri::command]
pub async fn add_materiales_batch(
    supabase: State<'_, SupabaseClient>,
    calculo_id: String,
    materiales: Vec<(String, f64)>, // (material_id, cantidad)
) -> Result<ApiResponse<Vec<CalculoMaterial>>, String> {
    let calculo_uuid = Uuid::parse_str(&calculo_id)
        .map_err(|e| format!("UUID de cálculo inválido: {}", e))?;

    let client = supabase.table("calculo_materiales")
        .map_err(|e| e.to_string())?;

    // Crear array de objetos para inserción batch
    let mut batch_data = Vec::new();
    for (material_id, cantidad) in materiales {
        let material_uuid = Uuid::parse_str(&material_id)
            .map_err(|e| format!("UUID de material inválido: {}", e))?;

        batch_data.push(json!({
            "calculo_id": calculo_uuid,
            "material_id": material_uuid,
            "cantidad": cantidad,
        }));
    }

    let response = client
        .insert(json!(batch_data).to_string())
        .execute()
        .await
        .map_err(|e| e.to_string())?;

    let calculo_materiales: Vec<CalculoMaterial> = response
        .json()
        .await
        .map_err(|e| e.to_string())?;

    Ok(ApiResponse::success(calculo_materiales))
}