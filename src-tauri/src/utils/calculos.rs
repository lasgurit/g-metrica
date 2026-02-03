// src-tauri/src/utils/calculos.rs

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VigaInputs {
    pub longitud: f64,                    // metros
    pub ancho: f64,                       // metros
    pub alto: f64,                        // metros
    pub diametro_longitudinal: u8,        // mm (6, 8, 10, 12)
    pub diametro_estribos: u8,           // mm (6, 8, 10, 12)
    pub separacion_estribos: f64,        // metros (0.15, 0.20, 0.25, etc.)
    pub tipo_hormigon: String,           // "H21", "H30", "H38"
    pub tipo_bolsa_cemento: u8,          // 25 o 50 kg
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
    pub kg_cemento_por_m3: f64,          // Información adicional
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
    
    // Validar diámetros permitidos
    if ![6, 8, 10, 12].contains(&inputs.diametro_longitudinal) {
        return Err("El diámetro longitudinal debe ser 6, 8, 10 o 12 mm".to_string());
    }
    
    if ![6, 8, 10, 12].contains(&inputs.diametro_estribos) {
        return Err("El diámetro de estribos debe ser 6, 8, 10 o 12 mm".to_string());
    }
    
    // Validar separación de estribos
    if inputs.separacion_estribos < 0.10 || inputs.separacion_estribos > 0.30 {
        return Err("La separación de estribos debe estar entre 0.10m y 0.30m".to_string());
    }
    
    // Validar tipo de hormigón
    if !["H21", "H30", "H38"].contains(&inputs.tipo_hormigon.as_str()) {
        return Err("El tipo de hormigón debe ser H21, H30 o H38".to_string());
    }
    
    // Validar tipo de bolsa
    if ![25, 50].contains(&inputs.tipo_bolsa_cemento) {
        return Err("El tipo de bolsa debe ser 25kg o 50kg".to_string());
    }
    
    Ok(())
}

/// Obtiene la dosificación de cemento según el tipo de hormigón
fn obtener_dosificacion_cemento(tipo_hormigon: &str) -> f64 {
    match tipo_hormigon {
        "H21" => 300.0,  // kg/m³
        "H30" => 350.0,  // kg/m³
        "H38" => 400.0,  // kg/m³
        _ => 300.0,      // Por defecto H21
    }
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
                kg_cemento_por_m3: 0.0,
            },
            vec![]
        );
    }
    
    // ========================================================================
    // 1. VOLUMEN DE HORMIGÓN
    // ========================================================================
    let volumen_hormigon = inputs.longitud * inputs.ancho * inputs.alto;
    
    // ========================================================================
    // 2. DOSIFICACIÓN SEGÚN TIPO DE HORMIGÓN
    // ========================================================================
    let kg_cemento_por_m3 = obtener_dosificacion_cemento(&inputs.tipo_hormigon);
    let kg_cemento_total = volumen_hormigon * kg_cemento_por_m3;
    
    // Convertir a bolsas según el tipo elegido
    let cemento_bolsas = kg_cemento_total / inputs.tipo_bolsa_cemento as f64;
    
    // Arena: 50% del volumen
    let arena_m3 = volumen_hormigon * 0.5;
    
    // Piedra: 50% del volumen
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
    // Configuración: según la separación elegida por el usuario
    let cantidad_estribos = inputs.longitud / inputs.separacion_estribos;
    
    // Perímetro del estribo = 2 × (ancho + alto)
    let perimetro_base = 2.0 * (inputs.ancho + inputs.alto);
    
    // Agregar 20cm de solape por estribo
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
        kg_cemento_por_m3,
    };
    
    // ========================================================================
    // 8. CREAR LISTA DE MATERIALES CON CÓDIGOS
    // ========================================================================
    
    // Código para hierro longitudinal
    let codigo_hierro_longitudinal = match inputs.diametro_longitudinal {
        6 => "hierro_6mm",
        8 => "hierro_8mm",
        10 => "hierro_10mm",
        12 => "hierro_12mm",
        _ => "hierro_8mm", // Por defecto
    };
    
    // Código para hierro de estribos
    let codigo_hierro_estribos = match inputs.diametro_estribos {
        6 => "hierro_6mm",
        8 => "hierro_8mm",
        10 => "hierro_10mm",
        12 => "hierro_12mm",
        _ => "hierro_6mm", // Por defecto
    };
    
    // Código para cemento según tipo de bolsa
    let codigo_cemento = match inputs.tipo_bolsa_cemento {
        25 => "cemento_25kg",
        50 => "cemento_50kg",
        _ => "cemento_25kg",
    };
    
    let mut materiales = vec![
        MaterialCalculado {
            codigo_material: codigo_cemento.to_string(),
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
            codigo_material: "alambre_atar".to_string(),
            cantidad: alambre_kg,
        },
        MaterialCalculado {
            codigo_material: "agua".to_string(),
            cantidad: agua_litros,
        },
    ];
    
    // Agregar hierro longitudinal
    materiales.push(MaterialCalculado {
        codigo_material: codigo_hierro_longitudinal.to_string(),
        cantidad: hierro_longitudinal_barras,
    });
    
    // Agregar hierro para estribos (solo si es diferente al longitudinal)
    if inputs.diametro_longitudinal != inputs.diametro_estribos {
        materiales.push(MaterialCalculado {
            codigo_material: codigo_hierro_estribos.to_string(),
            cantidad: hierro_estribos_barras,
        });
    } else {
        // Si es el mismo diámetro, sumar las cantidades
        if let Some(hierro_existente) = materiales.iter_mut().find(|m| m.codigo_material == codigo_hierro_longitudinal) {
            hierro_existente.cantidad += hierro_estribos_barras;
        }
    }
    
    (resultados, materiales)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_viga_h21_bolsa_25kg() {
        let inputs = VigaInputs {
            longitud: 20.0,
            ancho: 0.15,
            alto: 0.25,
            diametro_longitudinal: 12,
            diametro_estribos: 8,
            separacion_estribos: 0.20,
            tipo_hormigon: "H21".to_string(),
            tipo_bolsa_cemento: 25,
        };

        let (resultados, materiales) = calcular_viga_fundacion(inputs);

        // Volumen = 20 × 0.15 × 0.25 = 0.75 m³
        assert!((resultados.volumen_hormigon - 0.75).abs() < 0.01);
        
        // H21 = 300 kg/m³ → 0.75 * 300 = 225 kg → 225/25 = 9 bolsas de 25kg
        assert!((resultados.cemento_bolsas - 9.0).abs() < 0.01);
        assert!((resultados.kg_cemento_por_m3 - 300.0).abs() < 0.01);
        
        // Verificar que los materiales tengan los códigos correctos
        let codigos: Vec<&str> = materiales.iter().map(|m| m.codigo_material.as_str()).collect();
        assert!(codigos.contains(&"cemento_25kg"));
        assert!(codigos.contains(&"hierro_12mm"));
        assert!(codigos.contains(&"hierro_8mm"));
    }
    
    #[test]
    fn test_viga_h30_bolsa_50kg() {
        let inputs = VigaInputs {
            longitud: 20.0,
            ancho: 0.15,
            alto: 0.25,
            diametro_longitudinal: 12,
            diametro_estribos: 8,
            separacion_estribos: 0.20,
            tipo_hormigon: "H30".to_string(),
            tipo_bolsa_cemento: 50,
        };

        let (resultados, _) = calcular_viga_fundacion(inputs);

        // H30 = 350 kg/m³ → 0.75 * 350 = 262.5 kg → 262.5/50 = 5.25 bolsas de 50kg
        assert!((resultados.cemento_bolsas - 5.25).abs() < 0.01);
        assert!((resultados.kg_cemento_por_m3 - 350.0).abs() < 0.01);
    }
    
    #[test]
    fn test_viga_h38_bolsa_50kg() {
        let inputs = VigaInputs {
            longitud: 20.0,
            ancho: 0.15,
            alto: 0.25,
            diametro_longitudinal: 12,
            diametro_estribos: 8,
            separacion_estribos: 0.20,
            tipo_hormigon: "H38".to_string(),
            tipo_bolsa_cemento: 50,
        };

        let (resultados, _) = calcular_viga_fundacion(inputs);

        // H38 = 400 kg/m³ → 0.75 * 400 = 300 kg → 300/50 = 6 bolsas de 50kg
        assert!((resultados.cemento_bolsas - 6.0).abs() < 0.01);
        assert!((resultados.kg_cemento_por_m3 - 400.0).abs() < 0.01);
    }
    
    #[test]
    fn test_separacion_estribos_personalizada() {
        let inputs_20cm = VigaInputs {
            longitud: 10.0,
            ancho: 0.15,
            alto: 0.25,
            diametro_longitudinal: 10,
            diametro_estribos: 8,
            separacion_estribos: 0.20,
            tipo_hormigon: "H21".to_string(),
            tipo_bolsa_cemento: 25,
        };
        
        let inputs_15cm = VigaInputs {
            separacion_estribos: 0.15,
            ..inputs_20cm.clone()
        };

        let (resultados_20cm, _) = calcular_viga_fundacion(inputs_20cm);
        let (resultados_15cm, _) = calcular_viga_fundacion(inputs_15cm);

        // Con menos separación, debe haber más estribos
        assert!(resultados_15cm.hierro_estribos_barras > resultados_20cm.hierro_estribos_barras);
    }
    
    #[test]
    fn test_mismo_diametro_suma_hierro() {
        let inputs = VigaInputs {
            longitud: 10.0,
            ancho: 0.15,
            alto: 0.25,
            diametro_longitudinal: 10,
            diametro_estribos: 10,  // Mismo diámetro
            separacion_estribos: 0.20,
            tipo_hormigon: "H21".to_string(),
            tipo_bolsa_cemento: 25,
        };

        let (resultados, materiales) = calcular_viga_fundacion(inputs);

        // Debe haber solo UNA entrada de hierro_10mm con la suma
        let hierros_10mm: Vec<&MaterialCalculado> = materiales.iter()
            .filter(|m| m.codigo_material == "hierro_10mm")
            .collect();
        
        assert_eq!(hierros_10mm.len(), 1);
        
        let total_esperado = resultados.hierro_longitudinal_barras + resultados.hierro_estribos_barras;
        assert!((hierros_10mm[0].cantidad - total_esperado).abs() < 0.01);
    }
    
    #[test]
    fn test_validacion_tipo_hormigon_invalido() {
        let inputs = VigaInputs {
            longitud: 10.0,
            ancho: 0.15,
            alto: 0.25,
            diametro_longitudinal: 10,
            diametro_estribos: 8,
            separacion_estribos: 0.20,
            tipo_hormigon: "H50".to_string(),  // Inválido
            tipo_bolsa_cemento: 25,
        };
        
        assert!(validar_inputs(&inputs).is_err());
    }
    
    #[test]
    fn test_validacion_bolsa_invalida() {
        let inputs = VigaInputs {
            longitud: 10.0,
            ancho: 0.15,
            alto: 0.25,
            diametro_longitudinal: 10,
            diametro_estribos: 8,
            separacion_estribos: 0.20,
            tipo_hormigon: "H21".to_string(),
            tipo_bolsa_cemento: 30,  // Inválido
        };
        
        assert!(validar_inputs(&inputs).is_err());
    }
}