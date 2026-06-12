// Lee variables de entorno y las inyecta como constantes de compilación.
fn main() {
    // Leer .env del directorio raíz del workspace
    // dotenv lee automáticamente el .env del directorio padre de src-tauri/
    dotenv::dotenv().ok();
    // Leer SUPABASE_URL — si no existe, el build falla con mensaje claro
    let supabase_url = std::env::var("SUPABASE_URL")
    .expect("ERROR: SUPABASE_URL no definida. Revisar archivo .env");
    let supabase_anon_key = std::env::var("SUPABASE_ANON_KEY")
    .expect("ERROR: SUPABASE_ANON_KEY no definida. Revisar
    archivo .env");
    // Exponer como constantes de compilación accesibles con env!()
    println!("cargo:rustc-env=SUPABASE_URL={}", supabase_url);
    println!("cargo:rustc-env=SUPABASE_ANON_KEY={}", supabase_anon_key);
    // Re-ejecutar este script si el .env cambia
    println!("cargo:rerun-if-changed=../.env");
    println!("cargo:rerun-if-env-changed=SUPABASE_URL");
    println!("cargo:rerun-if-env-changed=SUPABASE_ANON_KEY");
    // Script estándar de Tauri (obligatorio — no eliminar)
    tauri_build::build()
}
