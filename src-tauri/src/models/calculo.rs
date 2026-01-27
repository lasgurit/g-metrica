use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Calculo {
    pub id: String,
    pub proyecto_id: String,
    pub tipo_calculo_id: i32,
    pub nombre: Option<String>,
    pub inputs: Value,
    pub resultados: Value,
    pub notas: Option<String>,
    pub activo: bool,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CalculoInput {
    pub proyecto_id: String,
    pub tipo_calculo_id: i32,
    pub nombre: Option<String>,
    pub inputs: Value,
    pub resultados: Value,
    pub notas: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
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
}
