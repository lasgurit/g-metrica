use crate::models::material::{Material, MaterialInput, CalculoMaterial, CalculoMaterialInput};
use crate::services::supabase::SupabaseClient;
use serde_json::json;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ResumenMaterial {
    pub material_id: String,
    pub material_nombre: String,
    pub material_unidad: String,
    pub material_categoria: Option<String>,
    pub cantidad_total: f64,
}

#[tauri::command]
pub async fn get_materiales(token: String) -> Result<Vec<Material>, String> {
    let client = SupabaseClient::new();
    
    let response = client.client
        .get(&format!(
            "{}?activo=eq.true&order=categoria.asc,nombre.asc",
            client.rest_url("materiales")
        ))
        .headers(client.headers(Some(&token)))
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if !response.status().is_success() {
        let error_text = response.text().await.unwrap_or_default();
        return Err(format!("Error al obtener materiales: {}", error_text));
    }

    let materiales: Vec<Material> = response
        .json()
        .await
        .map_err(|e| e.to_string())?;

    Ok(materiales)
}

#[tauri::command]
pub async fn get_materiales_sistema(token: String) -> Result<Vec<Material>, String> {
    let client = SupabaseClient::new();
    
    let response = client.client
        .get(&format!(
            "{}?es_sistema=eq.true&activo=eq.true&order=categoria.asc,nombre.asc",
            client.rest_url("materiales")
        ))
        .headers(client.headers(Some(&token)))
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if !response.status().is_success() {
        let error_text = response.text().await.unwrap_or_default();
        return Err(format!("Error al obtener materiales del sistema: {}", error_text));
    }

    let materiales: Vec<Material> = response
        .json()
        .await
        .map_err(|e| e.to_string())?;

    Ok(materiales)
}

#[tauri::command]
pub async fn crear_material(
    token: String,
    usuario_id: String,
    input: MaterialInput
) -> Result<Material, String> {
    let client = SupabaseClient::new();
    
    let response = client.client
        .post(&client.rest_url("materiales"))
        .headers(client.headers(Some(&token)))
        .header("Prefer", "return=representation")
        .json(&json!({
            "usuario_id": usuario_id,
            "codigo": input.codigo,
            "nombre": input.nombre,
            "unidad": input.unidad,
            "categoria": input.categoria,
            "descripcion": input.descripcion,
            "precio_referencia": input.precio_referencia,
            "es_sistema": false
        }))
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if !response.status().is_success() {
        let error_text = response.text().await.unwrap_or_default();
        return Err(format!("Error al crear material: {}", error_text));
    }

    let materiales: Vec<Material> = response
        .json()
        .await
        .map_err(|e| e.to_string())?;

    materiales.into_iter().next()
        .ok_or_else(|| "Error al crear material".to_string())
}

#[tauri::command]
pub async fn actualizar_material(
    token: String,
    material_id: String,
    input: MaterialInput
) -> Result<Material, String> {
    let client = SupabaseClient::new();
    
    let response = client.client
        .patch(&format!(
            "{}?id=eq.{}",
            client.rest_url("materiales"),
            material_id
        ))
        .headers(client.headers(Some(&token)))
        .header("Prefer", "return=representation")
        .json(&json!({
            "codigo": input.codigo,
            "nombre": input.nombre,
            "unidad": input.unidad,
            "categoria": input.categoria,
            "descripcion": input.descripcion,
            "precio_referencia": input.precio_referencia
        }))
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if !response.status().is_success() {
        let error_text = response.text().await.unwrap_or_default();
        return Err(format!("Error al actualizar material: {}", error_text));
    }

    let materiales: Vec<Material> = response
        .json()
        .await
        .map_err(|e| e.to_string())?;

    materiales.into_iter().next()
        .ok_or_else(|| "Error al actualizar material".to_string())
}

#[tauri::command]
pub async fn eliminar_material(token: String, material_id: String) -> Result<(), String> {
    let client = SupabaseClient::new();
    
    let response = client.client
        .patch(&format!(
            "{}?id=eq.{}",
            client.rest_url("materiales"),
            material_id
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
        return Err(format!("Error al eliminar material: {}", error_text));
    }

    Ok(())
}

#[tauri::command]
pub async fn get_materiales_calculo(
    token: String,
    calculo_id: String
) -> Result<Vec<CalculoMaterial>, String> {
    let client = SupabaseClient::new();
    
    let response = client.client
        .get(&format!(
            "{}?calculo_id=eq.{}",
            client.rest_url("calculo_materiales"),
            calculo_id
        ))
        .headers(client.headers(Some(&token)))
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if !response.status().is_success() {
        let error_text = response.text().await.unwrap_or_default();
        return Err(format!("Error al obtener materiales del cálculo: {}", error_text));
    }

    let materiales: Vec<CalculoMaterial> = response
        .json()
        .await
        .map_err(|e| e.to_string())?;

    Ok(materiales)
}

#[tauri::command]
pub async fn agregar_material_calculo(
    token: String,
    input: CalculoMaterialInput
) -> Result<CalculoMaterial, String> {
    let client = SupabaseClient::new();
    
    let response = client.client
        .post(&client.rest_url("calculo_materiales"))
        .headers(client.headers(Some(&token)))
        .header("Prefer", "return=representation")
        .json(&json!({
            "calculo_id": input.calculo_id,
            "material_id": input.material_id,
            "cantidad": input.cantidad
        }))
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if !response.status().is_success() {
        let error_text = response.text().await.unwrap_or_default();
        return Err(format!("Error al agregar material al cálculo: {}", error_text));
    }

    let materiales: Vec<CalculoMaterial> = response
        .json()
        .await
        .map_err(|e| e.to_string())?;

    materiales.into_iter().next()
        .ok_or_else(|| "Error al agregar material".to_string())
}

#[tauri::command]
pub async fn editar_cantidad_material(
    token: String,
    calculo_material_id: String,
    cantidad_editada: f64
) -> Result<CalculoMaterial, String> {
    let client = SupabaseClient::new();
    
    let response = client.client
        .patch(&format!(
            "{}?id=eq.{}",
            client.rest_url("calculo_materiales"),
            calculo_material_id
        ))
        .headers(client.headers(Some(&token)))
        .header("Prefer", "return=representation")
        .json(&json!({
            "cantidad_editada": cantidad_editada,
            "fue_editado": true
        }))
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if !response.status().is_success() {
        let error_text = response.text().await.unwrap_or_default();
        return Err(format!("Error al editar cantidad: {}", error_text));
    }

    let materiales: Vec<CalculoMaterial> = response
        .json()
        .await
        .map_err(|e| e.to_string())?;

    materiales.into_iter().next()
        .ok_or_else(|| "Error al editar cantidad".to_string())
}

#[tauri::command]
pub async fn get_resumen_proyecto(
    token: String,
    proyecto_id: String
) -> Result<Vec<ResumenMaterial>, String> {
    let client = SupabaseClient::new();
    
    let response = client.client
        .post(&format!("{}/rpc/obtener_resumen_proyecto", client.base_url))
        .headers(client.headers(Some(&token)))
        .json(&json!({
            "proyecto_uuid": proyecto_id
        }))
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if !response.status().is_success() {
        let error_text = response.text().await.unwrap_or_default();
        return Err(format!("Error al obtener resumen: {}", error_text));
    }

    let resumen: Vec<ResumenMaterial> = response
        .json()
        .await
        .map_err(|e| e.to_string())?;

    Ok(resumen)
}
