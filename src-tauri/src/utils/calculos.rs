// src-tauri/src/utils/calculos.rs

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VigaInputs {
    pub longitud: f64,           // metros
    pub ancho: f64,              // metros
    pub alto: f64,               // metros
    pub diametro_hierro: u8,     // mm (6, 8, 10, 12)
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VigaResultados {
    pub volumen_hormigon: f64,
    pub cemento_bolsas: f64,
    pub arena_m3: f64,
    pub piedra_m3: f64,
    pub hierro_longitudinal_barras: f64,
    pub hierro_estribos_barras: f64,
    pub alambre_kg: f64,
    pub agua_litros: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MaterialCalculado {
    pub codigo_material: String,
    pub cantidad: f64,
}

/// Valida los inputs antes de calcular
fn validar_inputs(inputs: &VigaInputs) -> Result<(), String> {
    // Validar que sean valores positivos
    if inputs.longitud <= 0.0 {
        return Err("La longitud debe ser mayor a cero".to_string());
    }
    if inputs.ancho <= 0.0 {
        return Err("El ancho debe ser mayor a cero".to_string());
    }
    if inputs.alto <= 0.0 {
        return Err("El alto debe ser mayor a cero".to_string());
    }
    
    // Validar rangos razonables para vigas de fundación
    if inputs.longitud > 50.0 {
        return Err("La longitud máxima soportada es 50 metros".to_string());
    }
    
    if inputs.ancho < 0.10 || inputs.ancho > 1.0 {
        return Err("El ancho debe estar entre 0.10m y 1.0m".to_string());
    }
    
    if inputs.alto < 0.15 || inputs.alto > 1.5 {
        return Err("El alto debe estar entre 0.15m y 1.5m".to_string());
    }
    
    // Validar diámetro permitido
    if ![6, 8, 10, 12].contains(&inputs.diametro_hierro) {
        return Err("El diámetro de hierro debe ser 6, 8, 10 o 12 mm".to_string());
    }
    
    Ok(())
}

pub fn calcular_viga_fundacion(inputs: VigaInputs) -> (VigaResultados, Vec<MaterialCalculado>) {
    // Validar inputs - si falla, devolver resultados vacíos
    if let Err(_) = validar_inputs(&inputs) {
        return (
            VigaResultados {
                volumen_hormigon: 0.0,
                cemento_bolsas: 0.0,
                arena_m3: 0.0,
                piedra_m3: 0.0,
                hierro_longitudinal_barras: 0.0,
                hierro_estribos_barras: 0.0,
                alambre_kg: 0.0,
                agua_litros: 0.0,
            },
            vec![]
        );
    }
    
    // ========================================================================
    // 1. VOLUMEN DE HORMIGÓN
    // ========================================================================
    let volumen_hormigon = inputs.longitud * inputs.ancho * inputs.alto;
    
    // ========================================================================
    // 2. DOSIFICACIÓN SEGÚN ESPECIFICACIONES DE LA ARQUITECTA
    // ========================================================================
    // Cemento: 300 kg por m³ = 12 bolsas de 25kg por m³
    let cemento_bolsas = volumen_hormigon * 12.0;
    
    // Arena: 50% del volumen (según especificación)
    let arena_m3 = volumen_hormigon * 0.5;
    
    // Piedra: 50% del volumen (según especificación)
    let piedra_m3 = volumen_hormigon * 0.5;
    
    // ========================================================================
    // 3. HIERRO LONGITUDINAL (4 barras corridas)
    // ========================================================================
    // Longitud total necesaria = longitud × 4 barras
    let longitud_total_longitudinal = inputs.longitud * 4.0;
    
    // Cada barra comercial tiene 12 metros
    let hierro_longitudinal_barras = longitud_total_longitudinal / 12.0;
    
    // ========================================================================
    // 4. ESTRIBOS (hierro transversal)
    // ========================================================================
    // Configuración: un estribo cada 20 cm (0.20 m)
    let cantidad_estribos = inputs.longitud / 0.2;
    
    // Perímetro del estribo = 2 × (ancho + alto)
    let perimetro_base = 2.0 * (inputs.ancho + inputs.alto);
    
    // Agregar 20cm de solape por estribo (según especificación)
    let longitud_por_estribo = perimetro_base + 0.20;
    
    // Longitud total de hierro para estribos
    let longitud_total_estribos = cantidad_estribos * longitud_por_estribo;
    
    // Cantidad de barras comerciales de 12m
    let hierro_estribos_barras = longitud_total_estribos / 12.0;
    
    // ========================================================================
    // 5. ALAMBRE DE ATAR
    // ========================================================================
    // Total de uniones = cantidad de estribos × 4 puntos de atadura
    let total_uniones = cantidad_estribos * 4.0;
    
    // Alambre por unión ≈ 0.12 m (12 cm)
    let alambre_metros = total_uniones * 0.12;
    
    // 1 kg de alambre ≈ 50 metros lineales
    let alambre_kg = alambre_metros / 50.0;
    
    // ========================================================================
    // 6. AGUA
    // ========================================================================
    // 180-200 litros por m³, usamos 190 litros como promedio
    let agua_litros = volumen_hormigon * 190.0;
    
    // ========================================================================
    // 7. CREAR ESTRUCTURA DE RESULTADOS
    // ========================================================================
    let resultados = VigaResultados {
        volumen_hormigon,
        cemento_bolsas,
        arena_m3,
        piedra_m3,
        hierro_longitudinal_barras,
        hierro_estribos_barras,
        alambre_kg,
        agua_litros,
    };
    
    // ========================================================================
    // 8. CREAR LISTA DE MATERIALES CON CÓDIGOS
    // ========================================================================
    let codigo_hierro = match inputs.diametro_hierro {
        6 => "hierro_6mm",
        8 => "hierro_8mm",
        10 => "hierro_10mm",
        12 => "hierro_12mm",
        _ => "hierro_8mm", // Por defecto
    };
    
    // Total de barras de hierro = longitudinales + estribos
    let total_hierro_barras = hierro_longitudinal_barras + hierro_estribos_barras;
    
    let materiales = vec![
        MaterialCalculado {
            codigo_material: "cemento_25kg".to_string(),
            cantidad: cemento_bolsas,
        },
        MaterialCalculado {
            codigo_material: "arena".to_string(),
            cantidad: arena_m3,
        },
        MaterialCalculado {
            codigo_material: "piedra".to_string(),
            cantidad: piedra_m3,
        },
        MaterialCalculado {
            codigo_material: codigo_hierro.to_string(),
            cantidad: total_hierro_barras,
        },
        MaterialCalculado {
            codigo_material: "alambre_atar".to_string(),
            cantidad: alambre_kg,
        },
        MaterialCalculado {
            codigo_material: "agua".to_string(),
            cantidad: agua_litros,
        },
    ];
    
    (resultados, materiales)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_viga_20_metros_ejemplo_arquitecta() {
        let inputs = VigaInputs {
            longitud: 20.0,
            ancho: 0.15,
            alto: 0.25,
            diametro_hierro: 12,
        };

        let (resultados, materiales) = calcular_viga_fundacion(inputs);

        // Volumen = 20 × 0.15 × 0.25 = 0.75 m³
        assert!((resultados.volumen_hormigon - 0.75).abs() < 0.01);
        
        // Cemento = 0.75 × 12 = 9 bolsas
        assert!((resultados.cemento_bolsas - 9.0).abs() < 0.01);
        
        // Arena = 0.75 × 0.5 = 0.375 m³
        assert!((resultados.arena_m3 - 0.375).abs() < 0.01);
        
        // Piedra = 0.75 × 0.5 = 0.375 m³
        assert!((resultados.piedra_m3 - 0.375).abs() < 0.01);
        
        // Hierro longitudinal = (20 × 4) / 12 = 6.67 barras
        assert!((resultados.hierro_longitudinal_barras - 6.666666).abs() < 0.01);
        
        // Estribos: 100 estribos × 1m c/u = 100m / 12 = 8.33 barras
        assert!((resultados.hierro_estribos_barras - 8.333333).abs() < 0.1);
        
        // Agua = 0.75 × 190 = 142.5 litros
        assert!((resultados.agua_litros - 142.5).abs() < 0.5);
        
        // Debe tener 6 materiales
        assert_eq!(materiales.len(), 6);
    }
    
    #[test]
    fn test_validacion_longitud_negativa() {
        let inputs = VigaInputs {
            longitud: -5.0,
            ancho: 0.15,
            alto: 0.25,
            diametro_hierro: 12,
        };
        
        assert!(validar_inputs(&inputs).is_err());
    }
    
    #[test]
    fn test_validacion_dimensiones_muy_grandes() {
        let inputs = VigaInputs {
            longitud: 100.0, // Mayor a 50m
            ancho: 0.15,
            alto: 0.25,
            diametro_hierro: 12,
        };
        
        assert!(validar_inputs(&inputs).is_err());
    }
}