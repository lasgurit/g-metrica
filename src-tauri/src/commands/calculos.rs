use crate::models::calculo::{Calculo, CalculoInput, TipoCalculo};
use crate::services::supabase::SupabaseClient;
use crate::utils::calculos::{VigaInputs, VigaResultados, calcular_viga_fundacion as calcular_viga};
use serde_json::json;

// ========================================================================
// COMANDOS EXISTENTES (mantener como están)
// ========================================================================

#[tauri::command]
pub async fn get_tipos_calculo(token: String) -> Result<Vec<TipoCalculo>, String> {
    let client = SupabaseClient::new();
    
    let response = client.client
        .get(&format!(
            "{}?activo=eq.true&order=orden.asc",
            client.rest_url("tipo_calculos")
        ))
        .headers(client.headers(Some(&token)))
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if !response.status().is_success() {
        let error_text = response.text().await.unwrap_or_default();
        return Err(format!("Error al obtener tipos de cálculo: {}", error_text));
    }

    let tipos: Vec<TipoCalculo> = response
        .json()
        .await
        .map_err(|e| e.to_string())?;

    Ok(tipos)
}

#[tauri::command]
pub async fn get_calculos_proyecto(
    token: String,
    proyecto_id: String
) -> Result<Vec<Calculo>, String> {
    let client = SupabaseClient::new();
    
    let response = client.client
        .get(&format!(
            "{}?proyecto_id=eq.{}&activo=eq.true&order=created_at.desc",
            client.rest_url("calculos"),
            proyecto_id
        ))
        .headers(client.headers(Some(&token)))
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if !response.status().is_success() {
        let error_text = response.text().await.unwrap_or_default();
        return Err(format!("Error al obtener cálculos: {}", error_text));
    }

    let calculos: Vec<Calculo> = response
        .json()
        .await
        .map_err(|e| e.to_string())?;

    Ok(calculos)
}

#[tauri::command]
pub async fn get_calculo(token: String, calculo_id: String) -> Result<Calculo, String> {
    let client = SupabaseClient::new();
    
    let response = client.client
        .get(&format!(
            "{}?id=eq.{}",
            client.rest_url("calculos"),
            calculo_id
        ))
        .headers(client.headers(Some(&token)))
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if !response.status().is_success() {
        let error_text = response.text().await.unwrap_or_default();
        return Err(format!("Error al obtener cálculo: {}", error_text));
    }

    let calculos: Vec<Calculo> = response
        .json()
        .await
        .map_err(|e| e.to_string())?;

    calculos.into_iter().next()
        .ok_or_else(|| "Cálculo no encontrado".to_string())
}

#[tauri::command]
pub async fn crear_calculo(token: String, input: CalculoInput) -> Result<Calculo, String> {
    let client = SupabaseClient::new();
    
    let response = client.client
        .post(&client.rest_url("calculos"))
        .headers(client.headers(Some(&token)))
        .header("Prefer", "return=representation")
        .json(&json!({
            "proyecto_id": input.proyecto_id,
            "tipo_calculo_id": input.tipo_calculo_id,
            "nombre": input.nombre,
            "inputs": input.inputs,
            "resultados": input.resultados,
            "notas": input.notas
        }))
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if !response.status().is_success() {
        let error_text = response.text().await.unwrap_or_default();
        return Err(format!("Error al crear cálculo: {}", error_text));
    }

    let calculos: Vec<Calculo> = response
        .json()
        .await
        .map_err(|e| e.to_string())?;

    calculos.into_iter().next()
        .ok_or_else(|| "Error al crear cálculo".to_string())
}

#[tauri::command]
pub async fn actualizar_calculo(
    token: String,
    calculo_id: String,
    input: CalculoInput
) -> Result<Calculo, String> {
    let client = SupabaseClient::new();
    
    let response = client.client
        .patch(&format!(
            "{}?id=eq.{}",
            client.rest_url("calculos"),
            calculo_id
        ))
        .headers(client.headers(Some(&token)))
        .header("Prefer", "return=representation")
        .json(&json!({
            "nombre": input.nombre,
            "inputs": input.inputs,
            "resultados": input.resultados,
            "notas": input.notas
        }))
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if !response.status().is_success() {
        let error_text = response.text().await.unwrap_or_default();
        return Err(format!("Error al actualizar cálculo: {}", error_text));
    }

    let calculos: Vec<Calculo> = response
        .json()
        .await
        .map_err(|e| e.to_string())?;

    calculos.into_iter().next()
        .ok_or_else(|| "Error al actualizar cálculo".to_string())
}

#[tauri::command]
pub async fn eliminar_calculo(token: String, calculo_id: String) -> Result<(), String> {
    let client = SupabaseClient::new();
    
    let response = client.client
        .patch(&format!(
            "{}?id=eq.{}",
            client.rest_url("calculos"),
            calculo_id
        ))
        .headers(client.headers(Some(&token)))
        .json(&json!({
            "activo": false
        }))
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if !response.status().is_success() {
        let error_text = response.text().await.unwrap_or_default();
        return Err(format!("Error al eliminar cálculo: {}", error_text));
    }

    Ok(())
}

// ========================================================================
// NUEVOS COMANDOS
// ========================================================================

/// Ejecutar cálculo de viga de fundación (solo cálculo, no guarda)
#[tauri::command]
pub fn calcular_viga_fundacion(inputs: VigaInputs) -> Result<VigaResultados, String> {
    let (resultados, _materiales) = calcular_viga(inputs);
    Ok(resultados)
}

/// Crear cálculo completo: calcula, guarda el cálculo y asocia los materiales
#[tauri::command]
pub async fn crear_calculo_completo(
    token: String,
    proyecto_id: String,
    nombre: Option<String>,
    inputs: VigaInputs
) -> Result<Calculo, String> {
    let client = SupabaseClient::new();
    
    // 1. Ejecutar cálculo
    let (resultados, materiales) = calcular_viga(inputs.clone());
    
    // 2. Crear el cálculo en la BD
    let calculo_input = json!({
        "proyecto_id": proyecto_id,
        "tipo_calculo_id": 1, // Viga de fundación
        "nombre": nombre,
        "inputs": json!(inputs),
        "resultados": json!(resultados)
    });
    
    let response = client.client
        .post(&client.rest_url("calculos"))
        .headers(client.headers(Some(&token)))
        .header("Prefer", "return=representation")
        .json(&calculo_input)
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if !response.status().is_success() {
        let error_text = response.text().await.unwrap_or_default();
        return Err(format!("Error al crear cálculo: {}", error_text));
    }

    let calculos: Vec<Calculo> = response
        .json()
        .await
        .map_err(|e| e.to_string())?;

    let calculo = calculos.into_iter().next()
        .ok_or_else(|| "Error al crear cálculo".to_string())?;
    
    // 3. Obtener IDs de materiales por código
    let materiales_response = client.client
        .get(&format!(
            "{}?select=id,codigo&activo=eq.true",
            client.rest_url("materiales")
        ))
        .headers(client.headers(Some(&token)))
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if !materiales_response.status().is_success() {
        return Ok(calculo); // Retornar el cálculo aunque no se puedan agregar materiales
    }

    #[derive(serde::Deserialize)]
    struct MaterialCodigo {
        id: String,
        codigo: Option<String>,
    }

    let materiales_bd: Vec<MaterialCodigo> = materiales_response
        .json()
        .await
        .unwrap_or_default();

    // 4. Insertar cada material en calculo_materiales
    for material_calc in materiales {
        // Buscar el ID del material por código
        if let Some(material_bd) = materiales_bd.iter().find(|m| {
            m.codigo.as_ref().map(|c| c.as_str()) == Some(&material_calc.codigo_material)
        }) {
            // Insertar en calculo_materiales
            let _ = client.client
                .post(&client.rest_url("calculo_materiales"))
                .headers(client.headers(Some(&token)))
                .json(&json!({
                    "calculo_id": calculo.id,
                    "material_id": material_bd.id,
                    "cantidad": material_calc.cantidad
                }))
                .send()
                .await;
        }
    }
    
    Ok(calculo)
}