mod commands;
mod models;
mod services;
mod utils;  // ← AGREGAR ESTA LÍNEA

use commands::auth::{login, register, logout, get_perfil};
use commands::proyectos::{
    get_proyectos, get_proyecto, crear_proyecto, 
    actualizar_proyecto, eliminar_proyecto, buscar_proyectos
};
use commands::calculos::{
    get_tipos_calculo, get_calculos_proyecto, get_calculo,
    crear_calculo, actualizar_calculo, eliminar_calculo,
    calcular_viga_fundacion, crear_calculo_completo  // ← Asegurar que estén aquí
};
use commands::materiales::{
    get_materiales, get_materiales_sistema, crear_material,
    actualizar_material, eliminar_material, get_materiales_calculo,
    agregar_material_calculo, editar_cantidad_material, get_resumen_proyecto
};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            // Auth
            login,
            register,
            logout,
            get_perfil,
            // Proyectos
            get_proyectos,
            get_proyecto,
            crear_proyecto,
            actualizar_proyecto,
            eliminar_proyecto,
            buscar_proyectos,
            // Cálculos
            get_tipos_calculo,
            get_calculos_proyecto,
            get_calculo,
            crear_calculo,
            actualizar_calculo,
            eliminar_calculo,
            calcular_viga_fundacion,      // ← Ya está registrado
            crear_calculo_completo,        // ← Ya está registrado
            // Materiales
            get_materiales,
            get_materiales_sistema,
            crear_material,
            actualizar_material,
            eliminar_material,
            get_materiales_calculo,
            agregar_material_calculo,
            editar_cantidad_material,
            get_resumen_proyecto
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}