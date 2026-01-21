use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// ============================================================================
// ESTRUCTURAS PARA INPUTS Y RESULTADOS
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VigaFundacionInput {
    pub longitud: f64,           // metros
    pub ancho: f64,              // metros
    pub alto: f64,               // metros
    pub diametro_hierro: u8,     // mm (6, 8, 10, 12)
    #[serde(default = "default_barras_longitudinales")]
    pub cantidad_barras_longitudinales: u8,  // default: 4
    #[serde(default = "default_separacion_estribos")]
    pub separacion_estribos: f64,            // metros, default: 0.20
    #[serde(default = "default_dosificacion_cemento")]
    pub dosificacion_cemento: f64,           // kg/m³, default: 300
}

fn default_barras_longitudinales() -> u8 { 4 }
fn default_separacion_estribos() -> f64 { 0.20 }
fn default_dosificacion_cemento() -> f64 { 300.0 }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VigaFundacionResultado {
    pub volumen_hormigon: f64,
    pub hierro_longitudinal_barras: u32,
    pub hierro_estribos_barras: u32,
    pub alambre_atar_kg: f64,
    pub cemento_bolsas: u32,
    pub arena_m3: f64,
    pub piedra_m3: f64,
    pub agua_litros: f64,
    pub materiales: HashMap<String, f64>,  // Para guardar en la BD
}

// ============================================================================
// CONSTANTES
// ============================================================================

const LONGITUD_BARRA_COMERCIAL: f64 = 12.0; // metros
const PESO_BOLSA_CEMENTO: f64 = 25.0;       // kg
const SOLAPE_ESTRIBO: f64 = 0.20;           // metros
const ALAMBRE_POR_UNION: f64 = 0.12;        // metros
const METROS_ALAMBRE_POR_KG: f64 = 50.0;    // metros/kg (aproximado)
const AGUA_POR_M3: f64 = 190.0;             // litros/m³

// ============================================================================
// CÁLCULO DE VIGA DE FUNDACIÓN
// ============================================================================

pub fn calcular_viga_fundacion(input: VigaFundacionInput) -> VigaFundacionResultado {
    // ------------------------------------------------------------------------
    // 1. VOLUMEN DE HORMIGÓN
    // ------------------------------------------------------------------------
    let volumen = input.longitud * input.ancho * input.alto;
    
    // ------------------------------------------------------------------------
    // 2. HIERRO LONGITUDINAL
    // ------------------------------------------------------------------------
    let longitud_total_longitudinal = input.longitud * input.cantidad_barras_longitudinales as f64;
    let barras_longitudinales = (longitud_total_longitudinal / LONGITUD_BARRA_COMERCIAL).ceil() as u32;
    
    // ------------------------------------------------------------------------
    // 3. ESTRIBOS
    // ------------------------------------------------------------------------
    let perimetro_estribo = 2.0 * (input.ancho + input.alto);
    let longitud_por_estribo = perimetro_estribo + SOLAPE_ESTRIBO;
    
    let cantidad_estribos = (input.longitud / input.separacion_estribos).ceil() as u32;
    let longitud_total_estribos = cantidad_estribos as f64 * longitud_por_estribo;
    let barras_estribos = (longitud_total_estribos / LONGITUD_BARRA_COMERCIAL).ceil() as u32;
    
    // ------------------------------------------------------------------------
    // 4. ALAMBRE DE ATAR
    // ------------------------------------------------------------------------
    let puntos_atadura = 4; // 4 puntos por estribo
    let total_uniones = cantidad_estribos * puntos_atadura;
    let metros_alambre = total_uniones as f64 * ALAMBRE_POR_UNION;
    let kg_alambre = (metros_alambre / METROS_ALAMBRE_POR_KG).ceil();
    
    // ------------------------------------------------------------------------
    // 5. MATERIALES PARA HORMIGÓN
    // ------------------------------------------------------------------------
    
    // Arena (50% del volumen + 10% desperdicio)
    let arena = (volumen * 0.50 * 1.10).ceil() * 100.0 / 100.0; // Redondear a 2 decimales
    
    // Piedra (50% del volumen + 10% desperdicio)
    let piedra = (volumen * 0.50 * 1.10).ceil() * 100.0 / 100.0;
    
    // Cemento
    let kg_cemento = volumen * input.dosificacion_cemento;
    let bolsas_cemento = (kg_cemento / PESO_BOLSA_CEMENTO).ceil() as u32;
    
    // Agua
    let litros_agua = (volumen * AGUA_POR_M3).round();
    
    // ------------------------------------------------------------------------
    // 6. CREAR MAPA DE MATERIALES
    // ------------------------------------------------------------------------
    let mut materiales = HashMap::new();
    
    // Códigos deben coincidir con los de la tabla 'materiales' en Supabase
    materiales.insert(
        format!("hierro_{}mm", input.diametro_hierro), 
        (barras_longitudinales + barras_estribos) as f64
    );
    materiales.insert("alambre_atar".to_string(), kg_alambre);
    materiales.insert("cemento_25kg".to_string(), bolsas_cemento as f64);
    materiales.insert("arena".to_string(), arena);
    materiales.insert("piedra".to_string(), piedra);
    materiales.insert("agua".to_string(), litros_agua);
    
    // ------------------------------------------------------------------------
    // 7. RETORNAR RESULTADO
    // ------------------------------------------------------------------------
    VigaFundacionResultado {
        volumen_hormigon: (volumen * 1000.0).round() / 1000.0, // 3 decimales
        hierro_longitudinal_barras: barras_longitudinales,
        hierro_estribos_barras: barras_estribos,
        alambre_atar_kg: kg_alambre,
        cemento_bolsas: bolsas_cemento,
        arena_m3: arena,
        piedra_m3: piedra,
        agua_litros: litros_agua,
        materiales,
    }
}

// ============================================================================
// FUNCIÓN GENÉRICA PARA EJECUTAR CÁLCULOS
// ============================================================================

pub fn ejecutar_calculo(tipo_calculo: &str, inputs: serde_json::Value) -> Result<serde_json::Value, String> {
    match tipo_calculo {
        "viga_fundacion" => {
            let input: VigaFundacionInput = serde_json::from_value(inputs)
                .map_err(|e| format!("Error al parsear inputs: {}", e))?;
            
            let resultado = calcular_viga_fundacion(input);
            
            serde_json::to_value(resultado)
                .map_err(|e| format!("Error al serializar resultado: {}", e))
        },
        _ => Err(format!("Tipo de cálculo '{}' no implementado", tipo_calculo))
    }
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_viga_fundacion_basico() {
        let input = VigaFundacionInput {
            longitud: 20.0,
            ancho: 0.15,
            alto: 0.25,
            diametro_hierro: 12,
            cantidad_barras_longitudinales: 4,
            separacion_estribos: 0.20,
            dosificacion_cemento: 300.0,
        };

        let resultado = calcular_viga_fundacion(input);

        // Verificar volumen
        assert!((resultado.volumen_hormigon - 0.75).abs() < 0.01);
        
        // Verificar hierro longitudinal
        assert_eq!(resultado.hierro_longitudinal_barras, 7);
        
        // Verificar cemento
        assert_eq!(resultado.cemento_bolsas, 9);
        
        // Verificar arena y piedra
        assert!(resultado.arena_m3 >= 0.37 && resultado.arena_m3 <= 0.45);
        assert!(resultado.piedra_m3 >= 0.37 && resultado.piedra_m3 <= 0.45);
    }

    #[test]
    fn test_materiales_map() {
        let input = VigaFundacionInput {
            longitud: 10.0,
            ancho: 0.15,
            alto: 0.25,
            diametro_hierro: 10,
            cantidad_barras_longitudinales: 4,
            separacion_estribos: 0.20,
            dosificacion_cemento: 300.0,
        };

        let resultado = calcular_viga_fundacion(input);

        assert!(resultado.materiales.contains_key("hierro_10mm"));
        assert!(resultado.materiales.contains_key("cemento_25kg"));
        assert!(resultado.materiales.contains_key("arena"));
        assert!(resultado.materiales.contains_key("piedra"));
    }
}