use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Proyecto {
    pub id: String,
    pub usuario_id: String,
    pub nombre: String,
    pub descripcion: Option<String>,
    pub ubicacion: Option<String>,
    pub activo: bool,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProyectoInput {
    pub nombre: String,
    pub descripcion: Option<String>,
    pub ubicacion: Option<String>,
}
