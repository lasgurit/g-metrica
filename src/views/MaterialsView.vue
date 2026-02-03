<template>
  <AppLayout title="Gestión de Materiales">
    <!-- Barra de búsqueda y botón agregar -->
    <div class="flex flex-wrap items-center justify-between gap-4 mb-8">
      <div class="flex items-center gap-3 flex-1 max-w-2xl">
        <SearchBar
          v-model="searchQuery"
          placeholder="Buscar material..."
          class="flex-1"
        />

        <BaseButton
          variant="secondary"
          icon="filter_list"
          @click="showFilters = !showFilters"
        >
          Filtrar
        </BaseButton>
      </div>

      <BaseButton
        variant="primary"
        icon="add_circle_outline"
        @click="openNewMaterialModal"
      >
        Añadir Nuevo Material
      </BaseButton>
    </div>

    <!-- Filtro de categorías (si showFilters está activo) -->
    <div
      v-if="showFilters"
      class="bg-white dark:bg-surface-dark border border-border-light dark:border-border-dark rounded-xl p-4 mb-6"
    >
      <div class="flex items-center gap-3 flex-wrap">
        <span class="text-sm font-semibold text-slate-700 dark:text-slate-300">
          Filtrar por:
        </span>
        <button
          @click="selectedCategory = ''"
          class="px-3 py-1.5 rounded-lg text-sm font-medium transition-colors"
          :class="
            selectedCategory === ''
              ? 'bg-primary text-white'
              : 'bg-slate-100 dark:bg-slate-800 text-slate-600 dark:text-slate-400 hover:bg-slate-200 dark:hover:bg-slate-700'
          "
        >
          Todos
        </button>
        <button
          @click="selectedCategory = 'sistema'"
          class="px-3 py-1.5 rounded-lg text-sm font-medium transition-colors"
          :class="
            selectedCategory === 'sistema'
              ? 'bg-primary text-white'
              : 'bg-slate-100 dark:bg-slate-800 text-slate-600 dark:text-slate-400 hover:bg-slate-200 dark:hover:bg-slate-700'
          "
        >
          Sistema ({{ materialesSistema.length }})
        </button>
        <button
          @click="selectedCategory = 'usuario'"
          class="px-3 py-1.5 rounded-lg text-sm font-medium transition-colors"
          :class="
            selectedCategory === 'usuario'
              ? 'bg-primary text-white'
              : 'bg-slate-100 dark:bg-slate-800 text-slate-600 dark:text-slate-400 hover:bg-slate-200 dark:hover:bg-slate-700'
          "
        >
          Mis Materiales ({{ materialesUsuario.length }})
        </button>
      </div>
    </div>

    <!-- Estado de carga -->
    <div
      v-if="materialStore.loading && materiales.length === 0"
      class="flex items-center justify-center py-20"
    >
      <div class="text-center">
        <div
          class="w-16 h-16 border-4 border-primary border-t-transparent rounded-full animate-spin mx-auto mb-4"
        ></div>
        <p class="text-slate-500 dark:text-slate-400">Cargando materiales...</p>
      </div>
    </div>

    <!-- Estado vacío (solo si no hay materiales en absoluto) -->
    <div
      v-else-if="!materialStore.loading && materiales.length === 0"
      class="flex items-center justify-center py-20"
    >
      <div class="text-center max-w-md">
        <div
          class="w-20 h-20 bg-slate-100 dark:bg-slate-800 rounded-full flex items-center justify-center mx-auto mb-6"
        >
          <span class="material-icons-round text-4xl text-slate-400">
            inventory_2
          </span>
        </div>
        <h3 class="text-xl font-bold text-slate-800 dark:text-white mb-2">
          No hay materiales disponibles
        </h3>
        <p class="text-slate-500 dark:text-slate-400 mb-8">
          Crea tu primer material personalizado para empezar
        </p>
        <BaseButton
          variant="primary"
          icon="add_circle_outline"
          @click="openNewMaterialModal"
        >
          Crear Primer Material
        </BaseButton>
      </div>
    </div>

    <!-- Tabla de materiales -->
    <div
      v-else
      class="bg-white dark:bg-surface-dark border border-border-light dark:border-border-dark rounded-2xl overflow-hidden shadow-sm"
    >
      <!-- Resumen de materiales -->
      <div
        class="px-6 py-4 bg-slate-50 dark:bg-slate-900/50 border-b border-slate-100 dark:border-slate-800"
      >
        <div class="flex items-center justify-between">
          <div class="flex items-center gap-6">
            <div class="flex items-center gap-2">
              <span
                class="w-2 h-2 rounded-full bg-blue-500 dark:bg-blue-400"
              ></span>
              <span class="text-sm text-slate-600 dark:text-slate-400">
                Sistema: <strong>{{ materialesSistema.length }}</strong>
              </span>
            </div>
            <div class="flex items-center gap-2">
              <span class="w-2 h-2 rounded-full bg-primary"></span>
              <span class="text-sm text-slate-600 dark:text-slate-400">
                Personalizados: <strong>{{ materialesUsuario.length }}</strong>
              </span>
            </div>
          </div>
          <span class="text-sm font-bold text-slate-700 dark:text-slate-300">
            Total: {{ materiales.length }}
          </span>
        </div>
      </div>

      <div class="overflow-x-auto">
        <table class="w-full text-left">
          <thead>
            <tr
              class="bg-slate-50 dark:bg-slate-900/50 text-slate-500 dark:text-slate-400 text-xs font-bold uppercase tracking-wider"
            >
              <th class="px-6 py-4">Nombre del Material</th>
              <th class="px-6 py-4">Categoría</th>
              <th class="px-6 py-4">Unidad</th>
              <th class="px-6 py-4">Actualizado</th>
              <th class="px-6 py-4 text-right">Acciones</th>
            </tr>
          </thead>
          <tbody class="divide-y divide-slate-100 dark:divide-slate-800">
            <!-- Mensaje si no hay resultados después de filtrar -->
            <tr
              v-if="paginatedMaterials.length === 0"
              class="hover:bg-slate-50 dark:hover:bg-slate-800/40"
            >
              <td colspan="5" class="px-6 py-12 text-center">
                <div class="flex flex-col items-center gap-3">
                  <span
                    class="material-icons-round text-4xl text-slate-300 dark:text-slate-600"
                  >
                    search_off
                  </span>
                  <p class="text-slate-500 dark:text-slate-400">
                    No se encontraron materiales con los filtros aplicados
                  </p>
                </div>
              </td>
            </tr>

            <MaterialRow
              v-for="material in paginatedMaterials"
              :key="material.id"
              :material="material"
              @edit="editMaterial"
              @delete="confirmDeleteMaterial"
            />
          </tbody>
        </table>
      </div>

      <!-- Paginación -->
      <div
        v-if="paginatedMaterials.length > 0"
        class="px-6 py-4 flex items-center justify-between border-t border-slate-100 dark:border-border-dark"
      >
        <p class="text-sm text-slate-500 dark:text-slate-400 font-medium">
          Mostrando
          <span class="text-slate-700 dark:text-slate-200"
            >{{ startIndex + 1 }}-{{ endIndex }}</span
          >
          de {{ totalMaterials }} materiales
        </p>

        <div class="flex items-center gap-2">
          <button
            @click="previousPage"
            :disabled="currentPage === 1"
            class="w-10 h-10 flex items-center justify-center rounded-xl bg-slate-100 dark:bg-border-dark text-slate-500 dark:text-slate-400 hover:bg-slate-200 dark:hover:bg-slate-700 transition-colors disabled:opacity-50 disabled:cursor-not-allowed"
          >
            <span class="material-icons-round">chevron_left</span>
          </button>

          <button
            v-for="page in visiblePages"
            :key="page"
            @click="currentPage = page"
            class="w-10 h-10 flex items-center justify-center rounded-xl font-bold transition-colors"
            :class="
              page === currentPage
                ? 'bg-primary text-white shadow-md shadow-primary/20'
                : 'bg-slate-100 dark:bg-border-dark text-slate-600 dark:text-slate-400 hover:bg-slate-200 dark:hover:bg-slate-700'
            "
          >
            {{ page }}
          </button>

          <button
            @click="nextPage"
            :disabled="currentPage === totalPages"
            class="w-10 h-10 flex items-center justify-center rounded-xl bg-slate-100 dark:bg-border-dark text-slate-500 dark:text-slate-400 hover:bg-slate-200 dark:hover:bg-slate-700 transition-colors disabled:opacity-50 disabled:cursor-not-allowed"
          >
            <span class="material-icons-round">chevron_right</span>
          </button>
        </div>

        <p class="text-sm text-slate-500 dark:text-slate-400 font-medium">
          Página
          <span class="text-slate-700 dark:text-slate-200"
            >{{ currentPage }} de {{ totalPages }}</span
          >
        </p>
      </div>
    </div>

    <!-- Modal de nuevo/editar material -->
    <BaseModal
      v-model="showMaterialModal"
      :title="editingMaterial ? 'Editar Material' : 'Nuevo Material'"
      description="Complete los datos del material"
      icon="inventory_2"
    >
      <div class="space-y-4">
        <BaseInput
          v-model="materialForm.nombre"
          label="Nombre del Material"
          placeholder="Ej: Cemento Portland"
          required
          :error="materialFormErrors.nombre"
        />

        <BaseInput
          v-model="materialForm.codigo"
          label="Código (Opcional)"
          placeholder="Ej: CEM-001"
        />

        <div>
          <label
            class="block text-sm font-semibold text-slate-700 dark:text-slate-300 mb-2"
          >
            Categoría
            <span class="text-red-500">*</span>
          </label>
          <div class="relative">
            <select
              v-model="materialForm.categoria"
              class="w-full px-4 py-3 rounded-xl border border-border-light dark:border-border-dark bg-white dark:bg-surface-dark text-slate-900 dark:text-white focus:ring-2 focus:ring-primary focus:border-transparent outline-none appearance-none"
              :class="
                materialFormErrors.categoria
                  ? 'border-red-500 focus:ring-red-500'
                  : ''
              "
            >
              <option value="">Seleccionar categoría</option>
              <option value="aglomerantes">Aglomerantes</option>
              <option value="aridos">Áridos</option>
              <option value="hierros">Hierros y Refuerzos</option>
              <option value="bloques">Bloques</option>
              <option value="aislantes">Aislantes</option>
              <option value="perfiles">Perfiles</option>
              <option value="chapas">Chapas</option>
              <option value="terminaciones">Terminaciones</option>
              <option value="aditivos">Aditivos</option>
              <option value="soldadura">Soldadura</option>
              <option value="varios">Varios</option>
            </select>
            <span
              class="material-icons-round absolute right-4 top-1/2 -translate-y-1/2 text-slate-400 pointer-events-none"
            >
              expand_more
            </span>
          </div>
          <p
            v-if="materialFormErrors.categoria"
            class="mt-1 text-sm text-red-500"
          >
            {{ materialFormErrors.categoria }}
          </p>
        </div>

        <div>
          <label
            class="block text-sm font-semibold text-slate-700 dark:text-slate-300 mb-2"
          >
            Unidad de Medida
            <span class="text-red-500">*</span>
          </label>
          <div class="relative">
            <select
              v-model="materialForm.unidad"
              class="w-full px-4 py-3 rounded-xl border border-border-light dark:border-border-dark bg-white dark:bg-surface-dark text-slate-900 dark:text-white focus:ring-2 focus:ring-primary focus:border-transparent outline-none appearance-none"
              :class="
                materialFormErrors.unidad
                  ? 'border-red-500 focus:ring-red-500'
                  : ''
              "
            >
              <option value="">Seleccionar unidad</option>
              <option value="m³">m³ (metro cúbico)</option>
              <option value="m²">m² (metro cuadrado)</option>
              <option value="m">m (metro)</option>
              <option value="metro lineal">metro lineal</option>
              <option value="kg">kg (kilogramo)</option>
              <option value="bolsa 25kg">Bolsa 25kg</option>
              <option value="bolsa 30kg">Bolsa 30kg</option>
              <option value="bolsa 50kg">Bolsa 50kg</option>
              <option value="barra 12m">Barra 12m</option>
              <option value="unidad">Unidad</option>
              <option value="litro">Litro</option>
            </select>
            <span
              class="material-icons-round absolute right-4 top-1/2 -translate-y-1/2 text-slate-400 pointer-events-none"
            >
              expand_more
            </span>
          </div>
          <p v-if="materialFormErrors.unidad" class="mt-1 text-sm text-red-500">
            {{ materialFormErrors.unidad }}
          </p>
        </div>

        <BaseInput
          v-model="materialForm.descripcion"
          label="Descripción (Opcional)"
          placeholder="Ej: Material para construcción"
        />

        <BaseInput
          v-model="materialForm.precio_referencia"
          type="number"
          label="Precio de Referencia (Opcional)"
          placeholder="0.00"
          icon="attach_money"
        />
      </div>

      <template #footer>
        <div class="flex items-center gap-4">
          <BaseButton variant="ghost" full-width @click="closeMaterialModal">
            Cancelar
          </BaseButton>
          <BaseButton
            variant="primary"
            full-width
            :loading="saving"
            @click="saveMaterial"
          >
            {{ editingMaterial ? "Actualizar" : "Guardar" }}
          </BaseButton>
        </div>
      </template>
    </BaseModal>

    <!-- Modal de confirmación de eliminación -->
    <BaseModal
      v-model="showDeleteModal"
      title="Eliminar Material"
      description="Esta acción no se puede deshacer"
      icon="delete_forever"
    >
      <p class="text-slate-600 dark:text-slate-400">
        ¿Está seguro que desea eliminar este material? Esta acción no se puede
        revertir.
      </p>

      <template #footer>
        <div class="flex items-center gap-4">
          <BaseButton
            variant="ghost"
            full-width
            @click="showDeleteModal = false"
          >
            Cancelar
          </BaseButton>
          <BaseButton
            variant="danger"
            full-width
            :loading="deleting"
            @click="deleteMaterial"
          >
            Eliminar Material
          </BaseButton>
        </div>
      </template>
    </BaseModal>
  </AppLayout>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from "vue";
import { useMaterialStore, type Material } from "@/stores/materials";
import AppLayout from "@/components/layout/AppLayout.vue";
import BaseButton from "@/components/ui/BaseButton.vue";
import BaseInput from "@/components/ui/BaseInput.vue";
import BaseModal from "@/components/ui/BaseModal.vue";
import SearchBar from "@/components/ui/SearchBar.vue";
import MaterialRow from "@/components/materials/MaterialRow.vue";

const materialStore = useMaterialStore();

// Estado
const searchQuery = ref("");
const showFilters = ref(false);
const selectedCategory = ref(""); // '', 'sistema', 'usuario'
const showMaterialModal = ref(false);
const showDeleteModal = ref(false);
const editingMaterial = ref<Material | null>(null);
const materialToDelete = ref<string | null>(null);
const saving = ref(false);
const deleting = ref(false);

// Formulario
const materialForm = ref({
  nombre: "",
  codigo: "",
  categoria: "",
  unidad: "",
  descripcion: "",
  precio_referencia: "",
});

const materialFormErrors = ref({
  nombre: "",
  categoria: "",
  unidad: "",
});

// Paginación
const currentPage = ref(1);
const itemsPerPage = 10;

// Computed
const materiales = computed(() => materialStore.materialesActivos);
const materialesSistema = computed(() => materialStore.materialesSistema);
const materialesUsuario = computed(() => materialStore.materialesUsuario);

const filteredMaterials = computed(() => {
  let filtered = materiales.value;

  // Filtrar por tipo (sistema/usuario)
  if (selectedCategory.value === "sistema") {
    filtered = materialesSistema.value;
  } else if (selectedCategory.value === "usuario") {
    filtered = materialesUsuario.value;
  }

  // Filtrar por búsqueda
  if (searchQuery.value) {
    filtered = filtered.filter(
      (m) =>
        m.nombre.toLowerCase().includes(searchQuery.value.toLowerCase()) ||
        m.categoria?.toLowerCase().includes(searchQuery.value.toLowerCase()) ||
        m.codigo?.toLowerCase().includes(searchQuery.value.toLowerCase()),
    );
  }

  return filtered;
});

const paginatedMaterials = computed(() => {
  const start = (currentPage.value - 1) * itemsPerPage;
  const end = start + itemsPerPage;
  return filteredMaterials.value.slice(start, end);
});

const totalMaterials = computed(() => filteredMaterials.value.length);
const totalPages = computed(() =>
  Math.ceil(totalMaterials.value / itemsPerPage),
);
const startIndex = computed(() => (currentPage.value - 1) * itemsPerPage);
const endIndex = computed(() =>
  Math.min(startIndex.value + itemsPerPage, totalMaterials.value),
);

const visiblePages = computed(() => {
  const pages = [];
  const maxVisible = 3;
  let start = Math.max(1, currentPage.value - Math.floor(maxVisible / 2));
  let end = Math.min(totalPages.value, start + maxVisible - 1);

  if (end - start + 1 < maxVisible) {
    start = Math.max(1, end - maxVisible + 1);
  }

  for (let i = start; i <= end; i++) {
    pages.push(i);
  }
  return pages;
});

// Métodos
const openNewMaterialModal = () => {
  editingMaterial.value = null;
  materialForm.value = {
    nombre: "",
    codigo: "",
    categoria: "",
    unidad: "",
    descripcion: "",
    precio_referencia: "",
  };
  materialFormErrors.value = {
    nombre: "",
    categoria: "",
    unidad: "",
  };
  showMaterialModal.value = true;
};

const editMaterial = (material: Material) => {
  editingMaterial.value = material;
  materialForm.value = {
    nombre: material.nombre,
    codigo: material.codigo || "",
    categoria: material.categoria || "",
    unidad: material.unidad,
    descripcion: material.descripcion || "",
    precio_referencia: material.precio_referencia?.toString() || "",
  };
  materialFormErrors.value = {
    nombre: "",
    categoria: "",
    unidad: "",
  };
  showMaterialModal.value = true;
};

const closeMaterialModal = () => {
  showMaterialModal.value = false;
  editingMaterial.value = null;
};

const validateForm = () => {
  let isValid = true;

  if (!materialForm.value.nombre.trim()) {
    materialFormErrors.value.nombre = "El nombre es requerido";
    isValid = false;
  } else {
    materialFormErrors.value.nombre = "";
  }

  if (!materialForm.value.categoria) {
    materialFormErrors.value.categoria = "La categoría es requerida";
    isValid = false;
  } else {
    materialFormErrors.value.categoria = "";
  }

  if (!materialForm.value.unidad) {
    materialFormErrors.value.unidad = "La unidad es requerida";
    isValid = false;
  } else {
    materialFormErrors.value.unidad = "";
  }

  return isValid;
};

const saveMaterial = async () => {
  if (!validateForm()) return;

  saving.value = true;

  try {
    const input = {
      nombre: materialForm.value.nombre,
      codigo: materialForm.value.codigo || undefined,
      categoria: materialForm.value.categoria || undefined,
      unidad: materialForm.value.unidad,
      descripcion: materialForm.value.descripcion || undefined,
      precio_referencia: materialForm.value.precio_referencia
        ? parseFloat(materialForm.value.precio_referencia)
        : undefined,
    };

    if (editingMaterial.value) {
      // Actualizar material existente
      await materialStore.actualizarMaterial(editingMaterial.value.id, input);
    } else {
      // Crear nuevo material
      await materialStore.crearMaterial(input);
    }

    closeMaterialModal();
  } catch (error: any) {
    console.error("Error al guardar material:", error);
    alert(
      `Error al guardar el material: ${error.message || "Error desconocido"}`,
    );
  } finally {
    saving.value = false;
  }
};

const confirmDeleteMaterial = (id: string) => {
  materialToDelete.value = id;
  showDeleteModal.value = true;
};

const deleteMaterial = async () => {
  if (!materialToDelete.value) return;

  deleting.value = true;

  try {
    await materialStore.eliminarMaterial(materialToDelete.value);
    showDeleteModal.value = false;
    materialToDelete.value = null;
  } catch (error: any) {
    console.error("Error al eliminar material:", error);
    alert(
      `Error al eliminar el material: ${error.message || "Error desconocido"}`,
    );
  } finally {
    deleting.value = false;
  }
};

const previousPage = () => {
  if (currentPage.value > 1) {
    currentPage.value--;
  }
};

const nextPage = () => {
  if (currentPage.value < totalPages.value) {
    currentPage.value++;
  }
};

// Lifecycle
onMounted(async () => {
  // Cargar materiales al montar el componente
  try {
    await materialStore.cargarMateriales();
  } catch (error) {
    console.error("Error al cargar materiales:", error);
  }
});
</script>
