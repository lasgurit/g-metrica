import { defineStore } from "pinia";
import { ref, computed } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { useAuthStore } from "./auth";

// Interfaces
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

export interface MaterialInput {
  codigo?: string;
  nombre: string;
  unidad: string;
  categoria?: string;
  descripcion?: string;
  precio_referencia?: number;
}

export const useMaterialStore = defineStore("materials", () => {
  // Estado
  const materiales = ref<Material[]>([]);
  const loading = ref(false);
  const error = ref<string | null>(null);

  // Computed
  const materialesActivos = computed(() =>
    materiales.value.filter((m) => m.activo),
  );

  const materialesSistema = computed(() =>
    materialesActivos.value.filter((m) => m.es_sistema),
  );

  const materialesUsuario = computed(() =>
    materialesActivos.value.filter((m) => !m.es_sistema),
  );

  const totalMateriales = computed(() => materialesActivos.value.length);

  // Acciones

  /**
   * Cargar todos los materiales (sistema + usuario)
   */
  const cargarMateriales = async () => {
    const authStore = useAuthStore();

    if (!authStore.token) {
      error.value = "Usuario no autenticado";
      return;
    }

    loading.value = true;
    error.value = null;

    try {
      const result = await invoke<Material[]>("get_materiales", {
        token: authStore.token,
      });

      materiales.value = result;
      console.log("Materiales cargados:", result);
    } catch (err: any) {
      error.value = err.toString();
      console.error("Error al cargar materiales:", err);
      throw err;
    } finally {
      loading.value = false;
    }
  };

  /**
   * Cargar solo materiales del sistema
   */
  const cargarMaterialesSistema = async () => {
    const authStore = useAuthStore();

    if (!authStore.token) {
      error.value = "Usuario no autenticado";
      return;
    }

    loading.value = true;
    error.value = null;

    try {
      const result = await invoke<Material[]>("get_materiales_sistema", {
        token: authStore.token,
      });

      console.log("Materiales del sistema cargados:", result);
      return result;
    } catch (err: any) {
      error.value = err.toString();
      console.error("Error al cargar materiales del sistema:", err);
      throw err;
    } finally {
      loading.value = false;
    }
  };

  /**
   * Crear un nuevo material
   */
  const crearMaterial = async (input: MaterialInput): Promise<Material> => {
    const authStore = useAuthStore();

    if (!authStore.token || !authStore.currentUser) {
      throw new Error("Usuario no autenticado");
    }

    loading.value = true;
    error.value = null;

    try {
      const result = await invoke<Material>("crear_material", {
        token: authStore.token,
        usuarioId: authStore.currentUser.id,
        input: input,
      });

      // Agregar el nuevo material al array
      materiales.value.unshift(result);

      console.log("Material creado:", result);
      return result;
    } catch (err: any) {
      error.value = err.toString();
      console.error("Error al crear material:", err);
      throw err;
    } finally {
      loading.value = false;
    }
  };

  /**
   * Actualizar un material existente
   */
  const actualizarMaterial = async (
    materialId: string,
    input: MaterialInput,
  ): Promise<Material> => {
    const authStore = useAuthStore();

    if (!authStore.token) {
      throw new Error("Usuario no autenticado");
    }

    loading.value = true;
    error.value = null;

    try {
      const result = await invoke<Material>("actualizar_material", {
        token: authStore.token,
        materialId: materialId,
        input: input,
      });

      // Actualizar el material en el array
      const index = materiales.value.findIndex((m) => m.id === materialId);
      if (index !== -1) {
        materiales.value[index] = result;
      }

      console.log("Material actualizado:", result);
      return result;
    } catch (err: any) {
      error.value = err.toString();
      console.error("Error al actualizar material:", err);
      throw err;
    } finally {
      loading.value = false;
    }
  };

  /**
   * Eliminar un material (soft delete)
   */
  const eliminarMaterial = async (materialId: string): Promise<void> => {
    const authStore = useAuthStore();

    if (!authStore.token) {
      throw new Error("Usuario no autenticado");
    }

    loading.value = true;
    error.value = null;

    try {
      await invoke<void>("eliminar_material", {
        token: authStore.token,
        materialId: materialId,
      });

      // Remover el material del array
      const index = materiales.value.findIndex((m) => m.id === materialId);
      if (index !== -1) {
        materiales.value.splice(index, 1);
      }

      console.log("Material eliminado:", materialId);
    } catch (err: any) {
      error.value = err.toString();
      console.error("Error al eliminar material:", err);
      throw err;
    } finally {
      loading.value = false;
    }
  };

  /**
   * Limpiar el estado
   */
  const limpiarEstado = () => {
    materiales.value = [];
    loading.value = false;
    error.value = null;
  };

  return {
    // Estado
    materiales,
    loading,
    error,

    // Computed
    materialesActivos,
    materialesSistema,
    materialesUsuario,
    totalMateriales,

    // Acciones
    cargarMateriales,
    cargarMaterialesSistema,
    crearMaterial,
    actualizarMaterial,
    eliminarMaterial,
    limpiarEstado,
  };
});
