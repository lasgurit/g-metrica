import { invoke } from '@tauri-apps/api/core';
import { ref, computed } from 'vue';

// ============================================================================
// TIPOS
// ============================================================================

interface ApiResponse<T> {
  success: boolean;
  data?: T;
  error?: string;
}

interface AuthResponse {
  access_token: string;
  token_type: string;
  expires_in: number;
  refresh_token: string;
  user: User;
}

interface User {
  id: string;
  email: string;
  email_confirmed_at?: string;
}

interface Proyecto {
  id: string;
  usuario_id: string;
  nombre: string;
  descripcion?: string;
  ubicacion?: string;
  activo: boolean;
  created_at: string;
  updated_at: string;
}

interface ProyectoInput {
  nombre: string;
  descripcion?: string;
  ubicacion?: string;
}

interface ProyectoUpdate {
  nombre?: string;
  descripcion?: string;
  ubicacion?: string;
  activo?: boolean;
}

interface ResumenMaterial {
  material_id: string;
  material_nombre: string;
  material_unidad: string;
  material_categoria?: string;
  cantidad_total: number;
}

// ============================================================================
// COMPOSABLE
// ============================================================================

export function useSupabase() {
  const isAuthenticated = ref(false);
  const currentUser = ref<User | null>(null);
  const authToken = ref<string | null>(null);

  // ==========================================================================
  // AUTENTICACIÓN
  // ==========================================================================

  const login = async (email: string, password: string): Promise<AuthResponse> => {
    try {
      const response = await invoke<ApiResponse<AuthResponse>>('login', {
        email,
        password
      });

      if (!response.success || !response.data) {
        throw new Error(response.error || 'Error al iniciar sesión');
      }

      isAuthenticated.value = true;
      currentUser.value = response.data.user;
      authToken.value = response.data.access_token;

      // Guardar token en localStorage para persistencia
      localStorage.setItem('auth_token', response.data.access_token);

      return response.data;
    } catch (error) {
      console.error('Error en login:', error);
      throw error;
    }
  };

  const signup = async (
    email: string,
    password: string,
    nombre: string,
    empresa?: string
  ): Promise<AuthResponse> => {
    try {
      const response = await invoke<ApiResponse<AuthResponse>>('signup', {
        email,
        password,
        nombre,
        empresa
      });

      if (!response.success || !response.data) {
        throw new Error(response.error || 'Error al registrarse');
      }

      isAuthenticated.value = true;
      currentUser.value = response.data.user;
      authToken.value = response.data.access_token;

      // Guardar token en localStorage
      localStorage.setItem('auth_token', response.data.access_token);

      return response.data;
    } catch (error) {
      console.error('Error en signup:', error);
      throw error;
    }
  };

  const logout = async (): Promise<void> => {
    try {
      await invoke<ApiResponse<boolean>>('logout');
      
      isAuthenticated.value = false;
      currentUser.value = null;
      authToken.value = null;

      // Limpiar localStorage
      localStorage.removeItem('auth_token');
    } catch (error) {
      console.error('Error en logout:', error);
      throw error;
    }
  };

  const checkAuth = async (): Promise<boolean> => {
    try {
      const response = await invoke<ApiResponse<boolean>>('is_authenticated');
      isAuthenticated.value = response.data || false;
      return isAuthenticated.value;
    } catch (error) {
      console.error('Error al verificar autenticación:', error);
      return false;
    }
  };

  const restoreSession = async (): Promise<void> => {
    const savedToken = localStorage.getItem('auth_token');
    
    if (savedToken) {
      try {
        await invoke<ApiResponse<boolean>>('set_auth_token', {
          token: savedToken
        });
        
        authToken.value = savedToken;
        isAuthenticated.value = true;
      } catch (error) {
        console.error('Error al restaurar sesión:', error);
        localStorage.removeItem('auth_token');
      }
    }
  };

  // ==========================================================================
  // PROYECTOS
  // ==========================================================================

  const getProyectos = async (): Promise<Proyecto[]> => {
    try {
      const response = await invoke<ApiResponse<Proyecto[]>>('get_proyectos');

      if (!response.success || !response.data) {
        throw new Error(response.error || 'Error al obtener proyectos');
      }

      return response.data;
    } catch (error) {
      console.error('Error al obtener proyectos:', error);
      throw error;
    }
  };

  const getProyecto = async (proyectoId: string): Promise<Proyecto> => {
    try {
      const response = await invoke<ApiResponse<Proyecto>>('get_proyecto', {
        proyectoId
      });

      if (!response.success || !response.data) {
        throw new Error(response.error || 'Error al obtener proyecto');
      }

      return response.data;
    } catch (error) {
      console.error('Error al obtener proyecto:', error);
      throw error;
    }
  };

  const createProyecto = async (input: ProyectoInput): Promise<Proyecto> => {
    try {
      const response = await invoke<ApiResponse<Proyecto>>('create_proyecto', {
        proyectoInput: input
      });

      if (!response.success || !response.data) {
        throw new Error(response.error || 'Error al crear proyecto');
      }

      return response.data;
    } catch (error) {
      console.error('Error al crear proyecto:', error);
      throw error;
    }
  };

  const updateProyecto = async (
    proyectoId: string,
    update: ProyectoUpdate
  ): Promise<Proyecto> => {
    try {
      const response = await invoke<ApiResponse<Proyecto>>('update_proyecto', {
        proyectoId,
        proyectoUpdate: update
      });

      if (!response.success || !response.data) {
        throw new Error(response.error || 'Error al actualizar proyecto');
      }

      return response.data;
    } catch (error) {
      console.error('Error al actualizar proyecto:', error);
      throw error;
    }
  };

  const deleteProyecto = async (proyectoId: string): Promise<boolean> => {
    try {
      const response = await invoke<ApiResponse<boolean>>('delete_proyecto', {
        proyectoId
      });

      if (!response.success) {
        throw new Error(response.error || 'Error al eliminar proyecto');
      }

      return true;
    } catch (error) {
      console.error('Error al eliminar proyecto:', error);
      throw error;
    }
  };

  const getResumenProyecto = async (
    proyectoId: string
  ): Promise<ResumenMaterial[]> => {
    try {
      const response = await invoke<ApiResponse<ResumenMaterial[]>>(
        'get_resumen_proyecto',
        { proyectoId }
      );

      if (!response.success || !response.data) {
        throw new Error(response.error || 'Error al obtener resumen');
      }

      return response.data;
    } catch (error) {
      console.error('Error al obtener resumen:', error);
      throw error;
    }
  };

  // ==========================================================================
  // RETURN
  // ==========================================================================

  return {
    // Estado
    isAuthenticated: computed(() => isAuthenticated.value),
    currentUser: computed(() => currentUser.value),
    authToken: computed(() => authToken.value),

    // Métodos de autenticación
    login,
    signup,
    logout,
    checkAuth,
    restoreSession,

    // Métodos de proyectos
    getProyectos,
    getProyecto,
    createProyecto,
    updateProyecto,
    deleteProyecto,
    getResumenProyecto,
  };
}