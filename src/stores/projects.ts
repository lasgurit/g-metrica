import { defineStore } from "pinia";
import { ref, computed } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { useAuthStore } from "./auth";

/**
 * Modelo de proyecto
 */
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

/**
 * Datos necesarios para crear o actualizar un proyecto
 */
export interface ProyectoInput {
  nombre: string;
  descripcion?: string;
  ubicacion?: string;
}

/**
 * Store de proyectos
 * Maneja CRUD y estado asociado
 */
export const useProjectStore = defineStore("projects", () => {
  /**
   * ======================
   * Estado
   * ======================
   */

  // Lista de proyectos del usuario
  const proyectos = ref<Proyecto[]>([]);

  // Estado de carga global
  const loading = ref(false);

  // Mensaje de error
  const error = ref<string | null>(null);

  /**
   * ======================
   * Computed
   * ======================
   */

  /**
   * Proyectos activos (no eliminados)
   */
  const proyectosActivos = computed(() =>
    proyectos.value.filter((p) => p.activo),
  );

  /**
   * Total de proyectos activos
   */
  const totalProyectos = computed(() => proyectosActivos.value.length);

  /**
   * ======================
   * Acciones
   * ======================
   */

  /**
   * Cargar todos los proyectos del usuario autenticado
   */
  const cargarProyectos = async () => {
    const authStore = useAuthStore();

    if (!authStore.token || !authStore.currentUser) {
      error.value = "Usuario no autenticado";
      return;
    }

    loading.value = true;
    error.value = null;

    try {
      const result = await invoke<Proyecto[]>("get_proyectos", {
        token: authStore.token,
        usuarioId: authStore.currentUser.id,
      });

      proyectos.value = result;
      console.log("Proyectos cargados:", result);
    } catch (err: any) {
      error.value = err.toString();
      console.error("Error al cargar proyectos:", err);
      throw err;
    } finally {
      loading.value = false;
    }
  };

  /**
   * Obtener un proyecto por ID
   */
  const obtenerProyecto = async (proyectoId: string): Promise<Proyecto> => {
    const authStore = useAuthStore();

    if (!authStore.token) {
      throw new Error("Usuario no autenticado");
    }

    loading.value = true;
    error.value = null;

    try {
      const result = await invoke<Proyecto>("get_proyecto", {
        token: authStore.token,
        proyectoId: proyectoId,
      });

      console.log("Proyecto obtenido:", result);
      return result;
    } catch (err: any) {
      error.value = err.toString();
      console.error("Error al obtener proyecto:", err);
      throw err;
    } finally {
      loading.value = false;
    }
  };

  /**
   * Crear un nuevo proyecto
   */
  const crearProyecto = async (input: ProyectoInput): Promise<Proyecto> => {
    const authStore = useAuthStore();

    if (!authStore.token || !authStore.currentUser) {
      throw new Error("Usuario no autenticado");
    }

    loading.value = true;
    error.value = null;

    try {
      const result = await invoke<Proyecto>("crear_proyecto", {
        token: authStore.token,
        usuarioId: authStore.currentUser.id,
        input: input,
      });

      // Agregar proyecto al inicio de la lista
      proyectos.value.unshift(result);

      console.log("Proyecto creado:", result);
      return result;
    } catch (err: any) {
      error.value = err.toString();
      console.error("Error al crear proyecto:", err);
      throw err;
    } finally {
      loading.value = false;
    }
  };

  /**
   * Actualizar un proyecto existente
   */
  const actualizarProyecto = async (
    proyectoId: string,
    input: ProyectoInput,
  ): Promise<Proyecto> => {
    const authStore = useAuthStore();

    if (!authStore.token) {
      throw new Error("Usuario no autenticado");
    }

    loading.value = true;
    error.value = null;

    try {
      const result = await invoke<Proyecto>("actualizar_proyecto", {
        token: authStore.token,
        proyectoId: proyectoId,
        input: input,
      });

      // Reemplazar proyecto actualizado en el array
      const index = proyectos.value.findIndex((p) => p.id === proyectoId);
      if (index !== -1) {
        proyectos.value[index] = result;
      }

      console.log("Proyecto actualizado:", result);
      return result;
    } catch (err: any) {
      error.value = err.toString();
      console.error("Error al actualizar proyecto:", err);
      throw err;
    } finally {
      loading.value = false;
    }
  };

  /**
   * Eliminar un proyecto (soft delete)
   */
  const eliminarProyecto = async (proyectoId: string): Promise<void> => {
    const authStore = useAuthStore();

    if (!authStore.token) {
      throw new Error("Usuario no autenticado");
    }

    loading.value = true;
    error.value = null;

    try {
      await invoke<void>("eliminar_proyecto", {
        token: authStore.token,
        proyectoId: proyectoId,
      });

      // Eliminar proyecto del estado local
      const index = proyectos.value.findIndex((p) => p.id === proyectoId);
      if (index !== -1) {
        proyectos.value.splice(index, 1);
      }

      console.log("Proyecto eliminado:", proyectoId);
    } catch (err: any) {
      error.value = err.toString();
      console.error("Error al eliminar proyecto:", err);
      throw err;
    } finally {
      loading.value = false;
    }
  };

  /**
   * Buscar proyectos por término
   */
  const buscarProyectos = async (termino: string): Promise<Proyecto[]> => {
    const authStore = useAuthStore();

    if (!authStore.token || !authStore.currentUser) {
      throw new Error("Usuario no autenticado");
    }

    loading.value = true;
    error.value = null;

    try {
      const result = await invoke<Proyecto[]>("buscar_proyectos", {
        token: authStore.token,
        usuarioId: authStore.currentUser.id,
        termino: termino,
      });

      console.log("Proyectos encontrados:", result);
      return result;
    } catch (err: any) {
      error.value = err.toString();
      console.error("Error al buscar proyectos:", err);
      throw err;
    } finally {
      loading.value = false;
    }
  };

  /**
   * Limpia el estado del store
   * Útil al cerrar sesión
   */
  const limpiarEstado = () => {
    proyectos.value = [];
    loading.value = false;
    error.value = null;
  };

  /**
   * API pública del store
   */
  return {
    // Estado
    proyectos,
    loading,
    error,

    // Computed
    proyectosActivos,
    totalProyectos,

    // Acciones
    cargarProyectos,
    obtenerProyecto,
    crearProyecto,
    actualizarProyecto,
    eliminarProyecto,
    buscarProyectos,
    limpiarEstado,
  };
});
