// Prevents additional console window on Windows in release
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod models;
mod services;
mod commands;
mod utils;

use services::supabase::SupabaseClientBuilder;
use commands::{auth, proyectos, calculos, materiales, calculo_materiales, tipos_calculo};

fn main() {
    // Cargar variables de entorno desde .env
    dotenv::dotenv().ok();

    // Obtener credenciales de Supabase desde variables de entorno
    let supabase_url = std::env::var("SUPABASE_URL")
        .expect("SUPABASE_URL debe estar definida en .env");
    
    let supabase_anon_key = std::env::var("SUPABASE_ANON_KEY")
        .expect("SUPABASE_ANON_KEY debe estar definida en .env");

    // Crear cliente de Supabase
    let supabase_client = SupabaseClientBuilder::new()
        .url(supabase_url)
        .anon_key(supabase_anon_key)
        .build()
        .expect("Error al crear cliente de Supabase");

    tauri::Builder::default()
        .manage(supabase_client)
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            // Comandos de autenticación
            auth::login,
            auth::signup,
            auth::logout,
            auth::is_authenticated,
            auth::set_auth_token,
            auth::get_auth_token,
            
            // Comandos de proyectos
            proyectos::get_proyectos,
            proyectos::get_proyecto,
            proyectos::create_proyecto,
            proyectos::update_proyecto,
            proyectos::delete_proyecto,
            proyectos::hard_delete_proyecto,
            proyectos::get_resumen_proyecto,
            
            // Comandos de cálculos
            calculos::get_calculos,
            calculos::get_calculo,
            calculos::create_calculo,
            calculos::update_calculo,
            calculos::delete_calculo,
            calculos::calcular_y_guardar,
            calculos::recalcular,
            
            // Comandos de materiales
            materiales::get_materiales,
            materiales::get_materiales_sistema,
            materiales::get_materiales_usuario,
            materiales::get_material,
            materiales::get_material_by_codigo,
            materiales::create_material,
            materiales::update_material,
            materiales::delete_material,
            materiales::get_materiales_by_categoria,
            
            // Comandos de cálculo-materiales
            calculo_materiales::get_materiales_calculo,
            calculo_materiales::add_material_to_calculo,
            calculo_materiales::update_cantidad_material,
            calculo_materiales::reset_cantidad_material,
            calculo_materiales::remove_material_from_calculo,
            calculo_materiales::add_materiales_batch,
            
            // Comandos de tipos de cálculo
            tipos_calculo::get_tipos_calculo,
            tipos_calculo::get_tipo_calculo,
            tipos_calculo::get_tipo_calculo_by_codigo,
            tipos_calculo::get_plantillas_calculo,
            tipos_calculo::get_tipos_by_categoria,
        ])
        .run(tauri::generate_context!())
        .expect("error al ejecutar la aplicación Tauri");
}