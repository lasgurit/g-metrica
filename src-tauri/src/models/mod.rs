use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use uuid::Uuid;
use serde_json::Value as JsonValue;

// ============================================================================
// PERFIL
// ============================================================================
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Perfil {
    pub id: Uuid,
    pub nombre: String,
    pub empresa: Option<String>,
    pub fecha_registro: DateTime<Utc>,
    pub ultima_sesion: Option<DateTime<Utc>>,
    pub activo: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerfilInput {
    pub nombre: String,
    pub empresa: Option<String>,
}

// ============================================================================
// PROYECTO
// ============================================================================
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Proyecto {
    pub id: Uuid,
    pub usuario_id: Uuid,
    pub nombre: String,
    pub descripcion: Option<String>,
    pub ubicacion: Option<String>,
    pub activo: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProyectoInput {
    pub nombre: String,
    pub descripcion: Option<String>,
    pub ubicacion: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProyectoUpdate {
    pub nombre: Option<String>,
    pub descripcion: Option<String>,
    pub ubicacion: Option<String>,
    pub activo: Option<bool>,
}

// ============================================================================
// TIPO DE CÁLCULO
// ============================================================================
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TipoCalculo {
    pub id: i32,
    pub codigo: String,
    pub nombre: String,
    pub descripcion: Option<String>,
    pub categoria: Option<String>,
    pub icono: Option<String>,
    pub orden: i32,
    pub activo: bool,
    pub es_personalizado: bool,
    pub created_at: DateTime<Utc>,
}

// ============================================================================
// CÁLCULO
// ============================================================================
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Calculo {
    pub id: Uuid,
    pub proyecto_id: Uuid,
    pub tipo_calculo_id: i32,
    pub nombre: Option<String>,
    pub inputs: JsonValue,
    pub resultados: JsonValue,
    pub notas: Option<String>,
    pub activo: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CalculoInput {
    pub proyecto_id: Uuid,
    pub tipo_calculo_id: i32,
    pub nombre: Option<String>,
    pub inputs: JsonValue,
    pub resultados: JsonValue,
    pub notas: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CalculoUpdate {
    pub nombre: Option<String>,
    pub inputs: Option<JsonValue>,
    pub resultados: Option<JsonValue>,
    pub notas: Option<String>,
    pub activo: Option<bool>,
}

// ============================================================================
// MATERIAL
// ============================================================================
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Material {
    pub id: Uuid,
    pub usuario_id: Option<Uuid>,
    pub codigo: Option<String>,
    pub nombre: String,
    pub unidad: String,
    pub categoria: Option<String>,
    pub descripcion: Option<String>,
    pub precio_referencia: Option<f64>,
    pub es_sistema: bool,
    pub activo: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MaterialInput {
    pub codigo: Option<String>,
    pub nombre: String,
    pub unidad: String,
    pub categoria: Option<String>,
    pub descripcion: Option<String>,
    pub precio_referencia: Option<f64>,
}

// ============================================================================
// CÁLCULO MATERIALES
// ============================================================================
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CalculoMaterial {
    pub id: Uuid,
    pub calculo_id: Uuid,
    pub material_id: Uuid,
    pub cantidad: f64,
    pub cantidad_editada: Option<f64>,
    pub fue_editado: bool,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CalculoMaterialInput {
    pub calculo_id: Uuid,
    pub material_id: Uuid,
    pub cantidad: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CalculoMaterialUpdate {
    pub cantidad_editada: f64,
    pub fue_editado: bool,
}

// ============================================================================
// RESUMEN DE MATERIALES
// ============================================================================
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResumenMaterial {
    pub material_id: Uuid,
    pub material_nombre: String,
    pub material_unidad: String,
    pub material_categoria: Option<String>,
    pub cantidad_total: f64,
}

// ============================================================================
// RESPUESTAS GENÉRICAS
// ============================================================================
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub error: Option<String>,
}

impl<T> ApiResponse<T> {
    pub fn success(data: T) -> Self {
        Self {
            success: true,
            data: Some(data),
            error: None,
        }
    }

    pub fn error(message: String) -> Self {
        Self {
            success: false,
            data: None,
            error: Some(message),
        }
    }
}