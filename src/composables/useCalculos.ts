import { ref } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import type { Calculo, CalculoInput, TipoCalculo, VigaInputs, VigaResultados } from '../types';
import { useAuth } from './useAuth';

export function useCalculos() {
  const calculos = ref<Calculo[]>([]);
  const calculoActual = ref<Calculo | null>(null);
  const tiposCalculo = ref<TipoCalculo[]>([]);
  const loading = ref(false);
  const error = ref<string | null>(null);

  const { token } = useAuth();

  async function cargarTiposCalculo(): Promise<void> {
    loading.value = true;
    error.value = null;
    
    try {
      const response = await invoke<TipoCalculo[]>('get_tipos_calculo', {
        token: token.value
      });
      tiposCalculo.value = response;
    } catch (e) {
      error.value = e as string;
    } finally {
      loading.value = false;
    }
  }

  async function cargarCalculosProyecto(proyectoId: string): Promise<void> {
    loading.value = true;
    error.value = null;
    
    try {
      const response = await invoke<Calculo[]>('get_calculos_proyecto', {
        token: token.value,
        proyectoId
      });
      calculos.value = response;
    } catch (e) {
      error.value = e as string;
    } finally {
      loading.value = false;
    }
  }

  async function obtenerCalculo(calculoId: string): Promise<Calculo | null> {
    loading.value = true;
    error.value = null;
    
    try {
      const response = await invoke<Calculo>('get_calculo', {
        token: token.value,
        calculoId
      });
      calculoActual.value = response;
      return response;
    } catch (e) {
      error.value = e as string;
      return null;
    } finally {
      loading.value = false;
    }
  }

  // ========================================================================
  // NUEVO: Calcular viga usando el backend
  // ========================================================================
  async function calcularViga(inputs: VigaInputs): Promise<VigaResultados | null> {
    loading.value = true;
    error.value = null;
    
    try {
      const response = await invoke<VigaResultados>('calcular_viga_fundacion', {
        inputs
      });
      return response;
    } catch (e) {
      error.value = e as string;
      return null;
    } finally {
      loading.value = false;
    }
  }

  // ========================================================================
  // NUEVO: Crear cálculo completo (cálculo + materiales)
  // ========================================================================
  async function crearCalculoCompleto(
    proyectoId: string,
    nombre: string | undefined,
    inputs: VigaInputs
  ): Promise<Calculo | null> {
    loading.value = true;
    error.value = null;
    
    try {
      const response = await invoke<Calculo>('crear_calculo_completo', {
        token: token.value,
        proyectoId,
        nombre,
        inputs
      });
      calculos.value.unshift(response);
      return response;
    } catch (e) {
      error.value = e as string;
      return null;
    } finally {
      loading.value = false;
    }
  }

  async function crearCalculo(input: CalculoInput): Promise<Calculo | null> {
    loading.value = true;
    error.value = null;
    
    try {
      const response = await invoke<Calculo>('crear_calculo', {
        token: token.value,
        input
      });
      calculos.value.unshift(response);
      return response;
    } catch (e) {
      error.value = e as string;
      return null;
    } finally {
      loading.value = false;
    }
  }

  async function actualizarCalculo(
    calculoId: string,
    input: CalculoInput
  ): Promise<Calculo | null> {
    loading.value = true;
    error.value = null;
    
    try {
      const response = await invoke<Calculo>('actualizar_calculo', {
        token: token.value,
        calculoId,
        input
      });
      
      const index = calculos.value.findIndex(c => c.id === calculoId);
      if (index !== -1) {
        calculos.value[index] = response;
      }
      
      return response;
    } catch (e) {
      error.value = e as string;
      return null;
    } finally {
      loading.value = false;
    }
  }

  async function eliminarCalculo(calculoId: string): Promise<boolean> {
    loading.value = true;
    error.value = null;
    
    try {
      await invoke('eliminar_calculo', {
        token: token.value,
        calculoId
      });
      
      calculos.value = calculos.value.filter(c => c.id !== calculoId);
      return true;
    } catch (e) {
      error.value = e as string;
      return false;
    } finally {
      loading.value = false;
    }
  }

  return {
    calculos,
    calculoActual,
    tiposCalculo,
    loading,
    error,
    cargarTiposCalculo,
    cargarCalculosProyecto,
    obtenerCalculo,
    calcularViga,           // Ahora usa el backend
    crearCalculoCompleto,   // Nuevo: crea cálculo + materiales
    crearCalculo,
    actualizarCalculo,
    eliminarCalculo
  };
}