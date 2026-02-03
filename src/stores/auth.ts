import { defineStore } from "pinia";
import { ref, computed } from "vue";
import { invoke } from "@tauri-apps/api/core";

/**
 * Modelo de usuario utilizado en el frontend
 */
interface User {
  id: string;
  name: string;
  email: string;
  avatar?: string;
  empresa?: string;
}

/**
 * Respuesta esperada desde el backend
 * para login / registro
 */
interface AuthResponse {
  access_token: string;
  token_type: string;
  expires_in: number;
  refresh_token: string;
  user: {
    id: string;
    email: string;
  };
}

/**
 * Store de autenticación
 * Maneja:
 * - Login / logout
 * - Registro
 * - Usuario actual
 * - Persistencia del token
 */
export const useAuthStore = defineStore("auth", () => {
  /**
   * ======================
   * Estado
   * ======================
   */

  // Token JWT (se inicializa desde localStorage si existe)
  const token = ref<string | null>(localStorage.getItem("auth_token"));

  // Usuario actualmente autenticado
  const currentUser = ref<User | null>(null);

  // Estado de carga para acciones async
  const loading = ref(false);

  /**
   * ======================
   * Computed
   * ======================
   */

  /**
   * Indica si el usuario está autenticado
   * Requiere token y usuario cargado
   */
  const isAuthenticated = computed(() => !!token.value && !!currentUser.value);

  /**
   * ======================
   * Acciones
   * ======================
   */

  /**
   * Login del usuario
   * - Llama al backend vía Tauri
   * - Guarda el token
   * - Obtiene el perfil del usuario
   */
  const login = async (email: string, password: string) => {
    loading.value = true;

    try {
      // Llamar al comando de Tauri para login
      const response = await invoke<AuthResponse>("login", {
        email,
        password,
      });

      // Guardar token en estado y localStorage
      token.value = response.access_token;
      localStorage.setItem("auth_token", response.access_token);

      // Obtener perfil completo del usuario
      await fetchUserProfile(response.user.id);

      return response;
    } catch (error) {
      console.error("Error en login:", error);
      throw error;
    } finally {
      loading.value = false;
    }
  };

  /**
   * Obtiene el perfil del usuario autenticado
   * desde el backend
   */
  const fetchUserProfile = async (userId: string) => {
    if (!token.value) return;

    try {
      const perfil = await invoke<any>("get_perfil", {
        token: token.value,
        userId: userId,
      });

      // Mapear respuesta del backend al modelo frontend
      currentUser.value = {
        id: perfil.id,
        name: perfil.nombre,
        email: perfil.email || "",
        empresa: perfil.empresa,
      };
    } catch (error) {
      console.error("Error al obtener perfil:", error);
    }
  };

  /**
   * Logout del usuario
   * - Informa al backend
   * - Limpia estado local
   */
  const logout = async () => {
    try {
      await invoke("logout");
    } catch (error) {
      console.error("Error en logout:", error);
    } finally {
      // Limpiar estado local
      token.value = null;
      currentUser.value = null;
      localStorage.removeItem("auth_token");
      localStorage.removeItem("user_id");
    }
  };

  /**
   * Verifica si existe una sesión activa
   * Se usa al iniciar la app
   */
  const checkAuth = async () => {
    const savedToken = localStorage.getItem("auth_token");

    if (savedToken) {
      token.value = savedToken;

      // Recuperar usuario si existe el ID
      const userId = localStorage.getItem("user_id");
      if (userId) {
        await fetchUserProfile(userId);
      }
    }
  };

  /**
   * Registro de nuevo usuario
   * - Crea la cuenta
   * - Autentica automáticamente
   */
  const register = async (
    email: string,
    password: string,
    nombre: string,
    empresa?: string,
  ) => {
    loading.value = true;

    try {
      const response = await invoke<AuthResponse>("register", {
        email,
        password,
        nombre,
        empresa,
      });

      // Guardar token y user_id
      token.value = response.access_token;
      localStorage.setItem("auth_token", response.access_token);
      localStorage.setItem("user_id", response.user.id);

      // Obtener perfil del usuario
      await fetchUserProfile(response.user.id);

      return response;
    } catch (error) {
      console.error("Error en registro:", error);
      throw error;
    } finally {
      loading.value = false;
    }
  };

  /**
   * API pública del store
   */
  return {
    // Estado
    token,
    currentUser,
    loading,

    // Computed
    isAuthenticated,

    // Acciones
    login,
    logout,
    checkAuth,
    register,
    fetchUserProfile,
  };
});
