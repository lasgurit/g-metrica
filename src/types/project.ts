export interface Project {
  id: string;
  usuario_id: string;
  nombre: string;
  descripcion?: string;
  ubicacion?: string;
  activo: boolean;
  created_at?: string;
  updated_at?: string;
}

export interface CreateProjectDTO {
  nombre: string;
  descripcion?: string;
  ubicacion?: string;
}

export interface UpdateProjectDTO {
  nombre?: string;
  descripcion?: string;
  ubicacion?: string;
}
