use crate::models::{ApiResponse, Calculo, CalculoInput, CalculoUpdate};
use crate::services::supabase::SupabaseClient;
use crate::utils::calculos::ejecutar_calculo;
use serde_json::json;
use tauri::State;
use uuid::Uuid;

// ============================================================================
// COMANDOS DE CÁLCULOS
// ============================================================================

/// Obtener todos los cálculos de un proyecto
#[tauri::command]
pub async fn get_calculos(
    supabase: State<'_, SupabaseClient>,
    proyecto_id: String,
) -> Result<ApiResponse<Vec<Calculo>>, String> {
    let uuid = Uuid::parse_str(&proyecto_id)
        .map_err(|e| format!("UUID inválido: {}", e))?;

    let client = supabase.table("calculos")
        .map_err(|e| e.to_string())?;

    let response = client
        .select("*")
        .eq("proyecto_id", uuid.to_string())
        .eq("activo", "true")
        .order("created_at.desc")
        .execute()
        .await
        .map_err(|e| e.to_string())?;

    let calculos: Vec<Calculo> = response
        .json()
        .await
        .map_err(|e| e.to_string())?;

    Ok(ApiResponse::success(calculos))
}

/// Obtener un cálculo específico por ID
#[tauri::command]
pub async fn get_calculo(
    supabase: State<'_, SupabaseClient>,
    calculo_id: String,
) -> Result<ApiResponse<Calculo>, String> {
    let uuid = Uuid::parse_str(&calculo_id)
        .map_err(|e| format!("UUID inválido: {}", e))?;

    let client = supabase.table("calculos")
        .map_err(|e| e.to_string())?;

    let response = client
        .select("*")
        .eq("id", uuid.to_string())
        .single()
        .execute()
        .await
        .map_err(|e| e.to_string())?;

    let calculo: Calculo = response
        .json()
        .await
        .map_err(|e| e.to_string())?;

    Ok(ApiResponse::success(calculo))
}

/// Crear un nuevo cálculo
#[tauri::command]
pub async fn create_calculo(
    supabase: State<'_, SupabaseClient>,
    calculo_input: CalculoInput,
) -> Result<ApiResponse<Calculo>, String> {
    let client = supabase.table("calculos")
        .map_err(|e| e.to_string())?;

    let response = client
        .insert(json!({
            "proyecto_id": calculo_input.proyecto_id,
            "tipo_calculo_id": calculo_input.tipo_calculo_id,
            "nombre": calculo_input.nombre,
            "inputs": calculo_input.inputs,
            "resultados": calculo_input.resultados,
            "notas": calculo_input.notas,
        }).to_string())
        .execute()
        .await
        .map_err(|e| e.to_string())?;

    let calculo: Calculo = response
        .json()
        .await
        .map_err(|e| e.to_string())?;

    Ok(ApiResponse::success(calculo))
}

/// Actualizar un cálculo existente
#[tauri::command]
pub async fn update_calculo(
    supabase: State<'_, SupabaseClient>,
    calculo_id: String,
    calculo_update: CalculoUpdate,
) -> Result<ApiResponse<Calculo>, String> {
    let uuid = Uuid::parse_str(&calculo_id)
        .map_err(|e| format!("UUID inválido: {}", e))?;

    let mut update_data = json!({});
    
    if let Some(nombre) = calculo_update.nombre {
        update_data["nombre"] = json!(nombre);
    }
    if let Some(inputs) = calculo_update.inputs {
        update_data["inputs"] = inputs;
    }
    if let Some(resultados) = calculo_update.resultados {
        update_data["resultados"] = resultados;
    }
    if let Some(notas) = calculo_update.notas {
        update_data["notas"] = json!(notas);
    }
    if let Some(activo) = calculo_update.activo {
        update_data["activo"] = json!(activo);
    }

    let client = supabase.table("calculos")
        .map_err(|e| e.to_string())?;

    let response = client
        .update(update_data.to_string())
        .eq("id", uuid.to_string())
        .execute()
        .await
        .map_err(|e| e.to_string())?;

    let calculo: Calculo = response
        .json()
        .await
        .map_err(|e| e.to_string())?;

    Ok(ApiResponse::success(calculo))
}

/// Eliminar un cálculo (soft delete)
#[tauri::command]
pub async fn delete_calculo(
    supabase: State<'_, SupabaseClient>,
    calculo_id: String,
) -> Result<ApiResponse<bool>, String> {
    let uuid = Uuid::parse_str(&calculo_id)
        .map_err(|e| format!("UUID inválido: {}", e))?;

    let client = supabase.table("calculos")
        .map_err(|e| e.to_string())?;

    client
        .update(json!({"activo": false}).to_string())
        .eq("id", uuid.to_string())
        .execute()
        .await
        .map_err(|e| e.to_string())?;

    Ok(ApiResponse::success(true))
}

/// Ejecutar cálculo de materiales y guardarlo
#[tauri::command]
pub async fn calcular_y_guardar(
    supabase: State<'_, SupabaseClient>,
    proyecto_id: String,
    tipo_calculo_codigo: String,
    tipo_calculo_id: i32,
    nombre: Option<String>,
    inputs: serde_json::Value,
    notas: Option<String>,
) -> Result<ApiResponse<Calculo>, String> {
    // 1. Ejecutar el cálculo
    let resultados = ejecutar_calculo(&tipo_calculo_codigo, inputs.clone())
        .map_err(|e| format!("Error al calcular: {}", e))?;

    // 2. Crear el cálculo en la BD
    let proyecto_uuid = Uuid::parse_str(&proyecto_id)
        .map_err(|e| format!("UUID inválido: {}", e))?;

    let calculo_input = CalculoInput {
        proyecto_id: proyecto_uuid,
        tipo_calculo_id,
        nombre,
        inputs,
        resultados,
        notas,
    };

    create_calculo(supabase, calculo_input).await
}

/// Recalcular un cálculo existente
#[tauri::command]
pub async fn recalcular(
    supabase: State<'_, SupabaseClient>,
    calculo_id: String,
    tipo_calculo_codigo: String,
    inputs: serde_json::Value,
) -> Result<ApiResponse<Calculo>, String> {
    // 1. Ejecutar el cálculo
    let resultados = ejecutar_calculo(&tipo_calculo_codigo, inputs.clone())
        .map_err(|e| format!("Error al calcular: {}", e))?;

    // 2. Actualizar el cálculo
    let calculo_update = CalculoUpdate {
        nombre: None,
        inputs: Some(inputs),
        resultados: Some(resultados),
        notas: None,
        activo: None,
    };

    update_calculo(supabase, calculo_id, calculo_update).await
}