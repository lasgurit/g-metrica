use crate::models::proyecto::{Proyecto, ProyectoInput};
use crate::services::supabase::SupabaseClient;
use serde_json::json;

#[tauri::command]
pub async fn get_proyectos(token: String, usuario_id: String) -> Result<Vec<Proyecto>, String> {
    let client = SupabaseClient::new();
    
    let response = client.client
        .get(&format!(
            "{}?usuario_id=eq.{}&activo=eq.true&order=created_at.desc",
            client.rest_url("proyectos"),
            usuario_id
        ))
        .headers(client.headers(Some(&token)))
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if !response.status().is_success() {
        let error_text = response.text().await.unwrap_or_default();
        return Err(format!("Error al obtener proyectos: {}", error_text));
    }

    let proyectos: Vec<Proyecto> = response
        .json()
        .await
        .map_err(|e| e.to_string())?;

    Ok(proyectos)
}

#[tauri::command]
pub async fn get_proyecto(token: String, proyecto_id: String) -> Result<Proyecto, String> {
    let client = SupabaseClient::new();
    
    let response = client.client
        .get(&format!(
            "{}?id=eq.{}",
            client.rest_url("proyectos"),
            proyecto_id
        ))
        .headers(client.headers(Some(&token)))
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if !response.status().is_success() {
        let error_text = response.text().await.unwrap_or_default();
        return Err(format!("Error al obtener proyecto: {}", error_text));
    }

    let proyectos: Vec<Proyecto> = response
        .json()
        .await
        .map_err(|e| e.to_string())?;

    proyectos.into_iter().next()
        .ok_or_else(|| "Proyecto no encontrado".to_string())
}

#[tauri::command]
pub async fn crear_proyecto(
    token: String,
    usuario_id: String,
    input: ProyectoInput
) -> Result<Proyecto, String> {
    let client = SupabaseClient::new();
    
    let response = client.client
        .post(&client.rest_url("proyectos"))
        .headers(client.headers(Some(&token)))
        .header("Prefer", "return=representation")
        .json(&json!({
            "usuario_id": usuario_id,
            "nombre": input.nombre,
            "descripcion": input.descripcion,
            "ubicacion": input.ubicacion
        }))
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if !response.status().is_success() {
        let error_text = response.text().await.unwrap_or_default();
        return Err(format!("Error al crear proyecto: {}", error_text));
    }

    let proyectos: Vec<Proyecto> = response
        .json()
        .await
        .map_err(|e| e.to_string())?;

    proyectos.into_iter().next()
        .ok_or_else(|| "Error al crear proyecto".to_string())
}

#[tauri::command]
pub async fn actualizar_proyecto(
    token: String,
    proyecto_id: String,
    input: ProyectoInput
) -> Result<Proyecto, String> {
    let client = SupabaseClient::new();
    
    let response = client.client
        .patch(&format!(
            "{}?id=eq.{}",
            client.rest_url("proyectos"),
            proyecto_id
        ))
        .headers(client.headers(Some(&token)))
        .header("Prefer", "return=representation")
        .json(&json!({
            "nombre": input.nombre,
            "descripcion": input.descripcion,
            "ubicacion": input.ubicacion
        }))
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if !response.status().is_success() {
        let error_text = response.text().await.unwrap_or_default();
        return Err(format!("Error al actualizar proyecto: {}", error_text));
    }

    let proyectos: Vec<Proyecto> = response
        .json()
        .await
        .map_err(|e| e.to_string())?;

    proyectos.into_iter().next()
        .ok_or_else(|| "Error al actualizar proyecto".to_string())
}

#[tauri::command]
pub async fn eliminar_proyecto(token: String, proyecto_id: String) -> Result<(), String> {
    let client = SupabaseClient::new();
    
    // Soft delete: marcamos como inactivo
    let response = client.client
        .patch(&format!(
            "{}?id=eq.{}",
            client.rest_url("proyectos"),
            proyecto_id
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
        return Err(format!("Error al eliminar proyecto: {}", error_text));
    }

    Ok(())
}

#[tauri::command]
pub async fn buscar_proyectos(
    token: String,
    usuario_id: String,
    termino: String
) -> Result<Vec<Proyecto>, String> {
    let client = SupabaseClient::new();
    
    let response = client.client
        .get(&format!(
            "{}?usuario_id=eq.{}&activo=eq.true&nombre=ilike.*{}*&order=created_at.desc",
            client.rest_url("proyectos"),
            usuario_id,
            termino
        ))
        .headers(client.headers(Some(&token)))
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if !response.status().is_success() {
        let error_text = response.text().await.unwrap_or_default();
        return Err(format!("Error al buscar proyectos: {}", error_text));
    }

    let proyectos: Vec<Proyecto> = response
        .json()
        .await
        .map_err(|e| e.to_string())?;

    Ok(proyectos)
}
