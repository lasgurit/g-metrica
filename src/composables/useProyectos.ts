import { ref } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import type { Proyecto, ProyectoInput } from '../types';
import { useAuth } from './useAuth';

export function useProyectos() {
  const proyectos = ref<Proyecto[]>([]);
  const proyectoActual = ref<Proyecto | null>(null);
  const loading = ref(false);
  const error = ref<string | null>(null);

  const { token, userId } = useAuth();

  async function cargarProyectos(): Promise<void> {
    loading.value = true;
    error.value = null;
    
    try {
      const response = await invoke<Proyecto[]>('get_proyectos', {
        token: token.value,
        usuarioId: userId.value
      });
      proyectos.value = response;
    } catch (e) {
      error.value = e as string;
    } finally {
      loading.value = false;
    }
  }

  async function obtenerProyecto(proyectoId: string): Promise<Proyecto | null> {
    loading.value = true;
    error.value = null;
    
    try {
      const response = await invoke<Proyecto>('get_proyecto', {
        token: token.value,
        proyectoId
      });
      proyectoActual.value = response;
      return response;
    } catch (e) {
      error.value = e as string;
      return null;
    } finally {
      loading.value = false;
    }
  }

  async function crearProyecto(input: ProyectoInput): Promise<Proyecto | null> {
    loading.value = true;
    error.value = null;
    
    try {
      const response = await invoke<Proyecto>('crear_proyecto', {
        token: token.value,
        usuarioId: userId.value,
        input
      });
      proyectos.value.unshift(response);
      return response;
    } catch (e) {
      error.value = e as string;
      return null;
    } finally {
      loading.value = false;
    }
  }

  async function actualizarProyecto(
    proyectoId: string,
    input: ProyectoInput
  ): Promise<Proyecto | null> {
    loading.value = true;
    error.value = null;
    
    try {
      const response = await invoke<Proyecto>('actualizar_proyecto', {
        token: token.value,
        proyectoId,
        input
      });
      
      const index = proyectos.value.findIndex(p => p.id === proyectoId);
      if (index !== -1) {
        proyectos.value[index] = response;
      }
      
      return response;
    } catch (e) {
      error.value = e as string;
      return null;
    } finally {
      loading.value = false;
    }
  }

  async function eliminarProyecto(proyectoId: string): Promise<boolean> {
    loading.value = true;
    error.value = null;
    
    try {
      await invoke('eliminar_proyecto', {
        token: token.value,
        proyectoId
      });
      
      proyectos.value = proyectos.value.filter(p => p.id !== proyectoId);
      return true;
    } catch (e) {
      error.value = e as string;
      return false;
    } finally {
      loading.value = false;
    }
  }

  async function buscarProyectos(termino: string): Promise<void> {
    loading.value = true;
    error.value = null;
    
    try {
      const response = await invoke<Proyecto[]>('buscar_proyectos', {
        token: token.value,
        usuarioId: userId.value,
        termino
      });
      proyectos.value = response;
    } catch (e) {
      error.value = e as string;
    } finally {
      loading.value = false;
    }
  }

  return {
    proyectos,
    proyectoActual,
    loading,
    error,
    cargarProyectos,
    obtenerProyecto,
    crearProyecto,
    actualizarProyecto,
    eliminarProyecto,
    buscarProyectos
  };
}
