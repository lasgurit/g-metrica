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

export interface CreateMaterialDTO {
  codigo?: string;
  nombre: string;
  unidad: string;
  categoria?: string;
  descripcion?: string;
  precio_referencia?: number;
}

export interface UpdateMaterialDTO {
  codigo?: string;
  nombre?: string;
  unidad?: string;
  categoria?: string;
  descripcion?: string;
  precio_referencia?: number;
}
