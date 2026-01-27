export interface AuthResponse {
  access_token: string;
  token_type: string;
  expires_in: number;
  refresh_token: string;
  user: AuthUser;
}

export interface AuthUser {
  id: string;
  email: string;
}

export interface Perfil {
  id: string;
  nombre: string;
  empresa?: string;
  activo: boolean;
  created_at?: string;
  updated_at?: string;
}

export interface Proyecto {
  id: string;
  usuario_id: string;
  nombre: string;
  descripcion?: string;
  ubicacion?: string;
  activo: boolean;
  created_at?: string;
  updated_at?: string;
}

export interface ProyectoInput {
  nombre: string;
  descripcion?: string;
  ubicacion?: string;
}

export interface TipoCalculo {
  id: number;
  codigo: string;
  nombre: string;
  descripcion?: string;
  categoria?: string;
  icono?: string;
  orden: number;
  activo: boolean;
  es_personalizado: boolean;
}

export interface Calculo {
  id: string;
  proyecto_id: string;
  tipo_calculo_id: number;
  nombre?: string;
  inputs: Record<string, any>;
  resultados: Record<string, any>;
  notas?: string;
  activo: boolean;
  created_at?: string;
  updated_at?: string;
}

export interface CalculoInput {
  proyecto_id: string;
  tipo_calculo_id: number;
  nombre?: string;
  inputs: Record<string, any>;
  resultados: Record<string, any>;
  notas?: string;
}

export interface Material {
  id: string;
  usuario_id?: string;
  codigo?: string;
  nombre: string;
  unidad: string;
  categoria?: string;
  descripcion?: string;
  precio_referencia?: number;
  es_sistema: boolean;
  activo: boolean;
  created_at?: string;
  updated_at?: string;
}

export interface CalculoMaterial {
  id: string;
  calculo_id: string;
  material_id: string;
  cantidad: number;
  cantidad_editada?: number;
  fue_editado: boolean;
  created_at?: string;
}

export interface ResumenMaterial {
  material_id: string;
  material_nombre: string;
  material_unidad: string;
  material_categoria?: string;
  cantidad_total: number;
}

export interface VigaInputs {
  longitud: number;
  ancho: number;
  alto: number;
  diametro_hierro: number;
}

export interface VigaResultados {
  volumen_hormigon: number;
  cemento_bolsas: number;
  arena_m3: number;
  piedra_m3: number;
  hierro_longitudinal_barras: number;  // ← NUEVO: Separado
  hierro_estribos_barras: number;      // ← NUEVO: Separado
  alambre_kg: number;
  agua_litros: number;
}

// Helper para obtener el total de hierro
export function getTotalHierroBarras(resultados: VigaResultados): number {
  return resultados.hierro_longitudinal_barras + resultados.hierro_estribos_barras;
}