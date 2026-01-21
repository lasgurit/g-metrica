// src-tauri/src/commands/tipos_calculo.rs

use crate::models::{ApiResponse, TipoCalculo};
use crate::services::supabase::SupabaseClient;
use serde::{Deserialize, Serialize};
use tauri::State;

// ============================================================================
// MODELOS ADICIONALES
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlantillaCalculo {
    pub id: i32,
    pub tipo_calculo_id: i32,
    pub campo_nombre: String,
    pub campo_label: String,
    pub campo_tipo: String,
    pub campo_unidad: Option<String>,
    pub campo_min: Option<f64>,
    pub campo_max: Option<f64>,
    pub campo_default: Option<f64>,
    pub campo_requerido: bool,
    pub campo_orden: i32,
    pub campo_opciones: Option<serde_json::Value>,
}

// ============================================================================
// COMANDOS DE TIPOS DE CÁLCULO
// ============================================================================

/// Obtener todos los tipos de cálculo disponibles
#[tauri::command]
pub async fn get_tipos_calculo(
    supabase: State<'_, SupabaseClient>,
) -> Result<ApiResponse<Vec<TipoCalculo>>, String> {
    let client = supabase.table("tipo_calculos")
        .map_err(|e| e.to_string())?;

    let response = client
        .select("*")
        .eq("activo", "true")
        .order("orden")
        .execute()
        .await
        .map_err(|e| e.to_string())?;

    let tipos: Vec<TipoCalculo> = response
        .json()
        .await
        .map_err(|e| e.to_string())?;

    Ok(ApiResponse::success(tipos))
}

/// Obtener un tipo de cálculo por ID
#[tauri::command]
pub async fn get_tipo_calculo(
    supabase: State<'_, SupabaseClient>,
    tipo_id: i32,
) -> Result<ApiResponse<TipoCalculo>, String> {
    let client = supabase.table("tipo_calculos")
        .map_err(|e| e.to_string())?;

    let response = client
        .select("*")
        .eq("id", tipo_id.to_string())
        .single()
        .execute()
        .await
        .map_err(|e| e.to_string())?;

    let tipo: TipoCalculo = response
        .json()
        .await
        .map_err(|e| e.to_string())?;

    Ok(ApiResponse::success(tipo))
}

/// Obtener un tipo de cálculo por código
#[tauri::command]
pub async fn get_tipo_calculo_by_codigo(
    supabase: State<'_, SupabaseClient>,
    codigo: String,
) -> Result<ApiResponse<TipoCalculo>, String> {
    let client = supabase.table("tipo_calculos")
        .map_err(|e| e.to_string())?;

    let response = client
        .select("*")
        .eq("codigo", codigo)
        .single()
        .execute()
        .await
        .map_err(|e| e.to_string())?;

    let tipo: TipoCalculo = response
        .json()
        .await
        .map_err(|e| e.to_string())?;

    Ok(ApiResponse::success(tipo))
}

/// Obtener las plantillas (campos) para un tipo de cálculo
#[tauri::command]
pub async fn get_plantillas_calculo(
    supabase: State<'_, SupabaseClient>,
    tipo_calculo_id: i32,
) -> Result<ApiResponse<Vec<PlantillaCalculo>>, String> {
    let client = supabase.table("plantillas_calculo")
        .map_err(|e| e.to_string())?;

    let response = client
        .select("*")
        .eq("tipo_calculo_id", tipo_calculo_id.to_string())
        .order("campo_orden")
        .execute()
        .await
        .map_err(|e| e.to_string())?;

    let plantillas: Vec<PlantillaCalculo> = response
        .json()
        .await
        .map_err(|e| e.to_string())?;

    Ok(ApiResponse::success(plantillas))
}

/// Obtener tipos de cálculo por categoría
#[tauri::command]
pub async fn get_tipos_by_categoria(
    supabase: State<'_, SupabaseClient>,
    categoria: String,
) -> Result<ApiResponse<Vec<TipoCalculo>>, String> {
    let client = supabase.table("tipo_calculos")
        .map_err(|e| e.to_string())?;

    let response = client
        .select("*")
        .eq("categoria", categoria)
        .eq("activo", "true")
        .order("orden")
        .execute()
        .await
        .map_err(|e| e.to_string())?;

    let tipos: Vec<TipoCalculo> = response
        .json()
        .await
        .map_err(|e| e.to_string())?;

    Ok(ApiResponse::success(tipos))
}