import { defineStore } from "pinia";
import { ref, computed } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { useAuthStore } from "./auth";

// ============================================================================
// INTERFACES
// ============================================================================

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

// Inputs para viga de fundación
export interface VigaInputs {
  longitud: number; // metros
  ancho: number; // metros
  alto: number; // metros
  diametro_longitudinal: number; // mm (6, 8, 10, 12)
  diametro_estribos: number; // mm (6, 8, 10, 12)
  separacion_estribos: number; // metros (0.15, 0.20, 0.25)
  tipo_hormigon: string; // "H21", "H30", "H38"
  tipo_bolsa_cemento: number; // 25 o 50 kg
}

// Resultados
export interface VigaResultados {
  volumen_hormigon: number;
  cemento_bolsas: number;
  arena_m3: number;
  piedra_m3: number;
  hierro_longitudinal_barras: number;
  hierro_estribos_barras: number;
  alambre_kg: number;
  agua_litros: number;
  kg_cemento_por_m3: number; //
}

export interface MaterialCalculado {
  codigo_material: string;
  cantidad: number;
}

// ============================================================================
// STORE
// ============================================================================

export const useCalculoStore = defineStore("calculos", () => {
  // Estado
  const tiposCalculo = ref<TipoCalculo[]>([]);
  const calculos = ref<Calculo[]>([]);
  const calculoActual = ref<Calculo | null>(null);
  const loading = ref(false);
  const error = ref<string | null>(null);

  // Computed
  const calculosActivos = computed(() =>
    calculos.value.filter((c) => c.activo),
  );

  const totalCalculos = computed(() => calculosActivos.value.length);

  const tiposCalculoActivos = computed(() =>
    tiposCalculo.value.filter((t) => t.activo),
  );

  // ============================================================================
  // ACCIONES - TIPOS DE CÁLCULO
  // ============================================================================

  const cargarTiposCalculo = async () => {
    const authStore = useAuthStore();

    if (!authStore.token) {
      error.value = "Usuario no autenticado";
      return;
    }

    loading.value = true;
    error.value = null;

    try {
      const result = await invoke<TipoCalculo[]>("get_tipos_calculo", {
        token: authStore.token,
      });

      tiposCalculo.value = result;
      console.log("Tipos de cálculo cargados:", result);
    } catch (err: any) {
      error.value = err.toString();
      console.error("Error al cargar tipos de cálculo:", err);
      throw err;
    } finally {
      loading.value = false;
    }
  };

  // ============================================================================
  // ACCIONES - CÁLCULOS
  // ============================================================================

  const cargarCalculosProyecto = async (proyectoId: string) => {
    const authStore = useAuthStore();

    if (!authStore.token) {
      error.value = "Usuario no autenticado";
      return;
    }

    loading.value = true;
    error.value = null;

    try {
      const result = await invoke<Calculo[]>("get_calculos_proyecto", {
        token: authStore.token,
        proyectoId: proyectoId,
      });

      calculos.value = result;
      console.log("Cálculos del proyecto cargados:", result);
    } catch (err: any) {
      error.value = err.toString();
      console.error("Error al cargar cálculos del proyecto:", err);
      throw err;
    } finally {
      loading.value = false;
    }
  };

  const obtenerCalculo = async (calculoId: string): Promise<Calculo> => {
    const authStore = useAuthStore();

    if (!authStore.token) {
      throw new Error("Usuario no autenticado");
    }

    loading.value = true;
    error.value = null;

    try {
      const result = await invoke<Calculo>("get_calculo", {
        token: authStore.token,
        calculoId: calculoId,
      });

      calculoActual.value = result;
      console.log("Cálculo obtenido:", result);
      return result;
    } catch (err: any) {
      error.value = err.toString();
      console.error("Error al obtener cálculo:", err);
      throw err;
    } finally {
      loading.value = false;
    }
  };

  const crearCalculo = async (input: CalculoInput): Promise<Calculo> => {
    const authStore = useAuthStore();

    if (!authStore.token) {
      throw new Error("Usuario no autenticado");
    }

    loading.value = true;
    error.value = null;

    try {
      const result = await invoke<Calculo>("crear_calculo", {
        token: authStore.token,
        input: input,
      });

      calculos.value.unshift(result);
      console.log("Cálculo creado:", result);
      return result;
    } catch (err: any) {
      error.value = err.toString();
      console.error("Error al crear cálculo:", err);
      throw err;
    } finally {
      loading.value = false;
    }
  };

  const actualizarCalculo = async (
    calculoId: string,
    input: CalculoInput,
  ): Promise<Calculo> => {
    const authStore = useAuthStore();

    if (!authStore.token) {
      throw new Error("Usuario no autenticado");
    }

    loading.value = true;
    error.value = null;

    try {
      const result = await invoke<Calculo>("actualizar_calculo", {
        token: authStore.token,
        calculoId: calculoId,
        input: input,
      });

      const index = calculos.value.findIndex((c) => c.id === calculoId);
      if (index !== -1) {
        calculos.value[index] = result;
      }

      console.log("Cálculo actualizado:", result);
      return result;
    } catch (err: any) {
      error.value = err.toString();
      console.error("Error al actualizar cálculo:", err);
      throw err;
    } finally {
      loading.value = false;
    }
  };

  const eliminarCalculo = async (calculoId: string): Promise<void> => {
    const authStore = useAuthStore();

    if (!authStore.token) {
      throw new Error("Usuario no autenticado");
    }

    loading.value = true;
    error.value = null;

    try {
      await invoke<void>("eliminar_calculo", {
        token: authStore.token,
        calculoId: calculoId,
      });

      const index = calculos.value.findIndex((c) => c.id === calculoId);
      if (index !== -1) {
        calculos.value.splice(index, 1);
      }

      console.log("Cálculo eliminado:", calculoId);
    } catch (err: any) {
      error.value = err.toString();
      console.error("Error al eliminar cálculo:", err);
      throw err;
    } finally {
      loading.value = false;
    }
  };

  // ============================================================================
  // ACCIONES - VIGA DE FUNDACIÓN
  // ============================================================================

  /**
   * Ejecutar el cálculo de viga de fundación (solo cálculo, no guarda)
   */
  const calcularVigaFundacion = async (
    inputs: VigaInputs,
  ): Promise<VigaResultados> => {
    loading.value = true;
    error.value = null;

    try {
      const result = await invoke<VigaResultados>("calcular_viga_fundacion", {
        inputs: inputs,
      });

      console.log("Resultados de viga:", result);
      return result;
    } catch (err: any) {
      error.value = err.toString();
      console.error("Error al calcular viga:", err);
      throw err;
    } finally {
      loading.value = false;
    }
  };

  /**
   * Crear cálculo completo: calcula + guarda + asocia materiales
   */
  const crearCalculoCompleto = async (
    proyectoId: string,
    nombre: string | undefined,
    inputs: VigaInputs,
  ): Promise<Calculo> => {
    const authStore = useAuthStore();

    if (!authStore.token) {
      throw new Error("Usuario no autenticado");
    }

    loading.value = true;
    error.value = null;

    try {
      const result = await invoke<Calculo>("crear_calculo_completo", {
        token: authStore.token,
        proyectoId: proyectoId,
        nombre: nombre,
        inputs: inputs,
      });

      calculos.value.unshift(result);
      console.log("Cálculo completo creado:", result);
      return result;
    } catch (err: any) {
      error.value = err.toString();
      console.error("Error al crear cálculo completo:", err);
      throw err;
    } finally {
      loading.value = false;
    }
  };

  /**
   * Limpiar el estado
   */
  const limpiarEstado = () => {
    tiposCalculo.value = [];
    calculos.value = [];
    calculoActual.value = null;
    loading.value = false;
    error.value = null;
  };

  return {
    // Estado
    tiposCalculo,
    calculos,
    calculoActual,
    loading,
    error,

    // Computed
    calculosActivos,
    totalCalculos,
    tiposCalculoActivos,

    // Acciones - Tipos de Cálculo
    cargarTiposCalculo,

    // Acciones - Cálculos
    cargarCalculosProyecto,
    obtenerCalculo,
    crearCalculo,
    actualizarCalculo,
    eliminarCalculo,

    // Acciones - Viga de Fundación
    calcularVigaFundacion,
    crearCalculoCompleto,

    // Utilidades
    limpiarEstado,
  };
});
