import { ref, computed } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import type { AuthResponse, Perfil } from '../types';

const authData = ref<AuthResponse | null>(null);
const perfil = ref<Perfil | null>(null);
const loading = ref(false);
const error = ref<string | null>(null);

export function useAuth() {
  const isAuthenticated = computed(() => !!authData.value?.access_token);
  const token = computed(() => authData.value?.access_token || '');
  const userId = computed(() => authData.value?.user?.id || '');

  async function login(email: string, password: string): Promise<boolean> {
    loading.value = true;
    error.value = null;
    
    try {
      const response = await invoke<AuthResponse>('login', { email, password });
      authData.value = response;
      
      // Obtener perfil del usuario
      const perfilData = await invoke<Perfil>('get_perfil', {
        token: response.access_token,
        userId: response.user.id
      });
      perfil.value = perfilData;
      
      // Guardar en localStorage para persistencia
      localStorage.setItem('auth', JSON.stringify(response));
      localStorage.setItem('perfil', JSON.stringify(perfilData));
      
      return true;
    } catch (e) {
      error.value = e as string;
      return false;
    } finally {
      loading.value = false;
    }
  }

  async function register(
    email: string,
    password: string,
    nombre: string,
    empresa?: string
  ): Promise<boolean> {
    loading.value = true;
    error.value = null;
    
    try {
      const response = await invoke<AuthResponse>('register', {
        email,
        password,
        nombre,
        empresa
      });
      authData.value = response;
      
      perfil.value = {
        id: response.user.id,
        nombre,
        empresa,
        activo: true
      };
      
      localStorage.setItem('auth', JSON.stringify(response));
      localStorage.setItem('perfil', JSON.stringify(perfil.value));
      
      return true;
    } catch (e) {
      error.value = e as string;
      return false;
    } finally {
      loading.value = false;
    }
  }

  function logout() {
    authData.value = null;
    perfil.value = null;
    localStorage.removeItem('auth');
    localStorage.removeItem('perfil');
  }

  function restoreSession() {
    const savedAuth = localStorage.getItem('auth');
    const savedPerfil = localStorage.getItem('perfil');
    
    if (savedAuth) {
      authData.value = JSON.parse(savedAuth);
    }
    if (savedPerfil) {
      perfil.value = JSON.parse(savedPerfil);
    }
  }

  return {
    authData,
    perfil,
    loading,
    error,
    isAuthenticated,
    token,
    userId,
    login,
    register,
    logout,
    restoreSession
  };
}
