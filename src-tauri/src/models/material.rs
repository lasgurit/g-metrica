use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Material {
    pub id: String,
    pub usuario_id: Option<String>,
    pub codigo: Option<String>,
    pub nombre: String,
    pub unidad: String,
    pub categoria: Option<String>,
    pub descripcion: Option<String>,
    pub precio_referencia: Option<f64>,
    pub es_sistema: bool,
    pub activo: bool,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MaterialInput {
    pub codigo: Option<String>,
    pub nombre: String,
    pub unidad: String,
    pub categoria: Option<String>,
    pub descripcion: Option<String>,
    pub precio_referencia: Option<f64>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CalculoMaterial {
    pub id: String,
    pub calculo_id: String,
    pub material_id: String,
    pub cantidad: f64,
    pub cantidad_editada: Option<f64>,
    pub fue_editado: bool,
    pub created_at: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CalculoMaterialInput {
    pub calculo_id: String,
    pub material_id: String,
    pub cantidad: f64,
}
