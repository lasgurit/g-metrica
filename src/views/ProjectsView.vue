<template>
  <AppLayout title="Gestión de Proyectos">
    <!-- Barra de búsqueda y acciones -->
    <div class="flex items-center justify-between mb-6">
      <div class="flex items-center gap-4 flex-1 max-w-2xl">
        <div class="relative flex-1">
          <span
            class="material-icons-round absolute left-4 top-1/2 -translate-y-1/2 text-slate-400"
          >
            search
          </span>
          <input
            v-model="searchQuery"
            type="text"
            placeholder="Buscar proyecto por nombre..."
            class="w-full pl-12 pr-4 py-3 rounded-xl border border-border-light dark:border-border-dark bg-white dark:bg-surface-dark text-slate-900 dark:text-white focus:ring-2 focus:ring-primary focus:border-transparent outline-none transition-all"
          />
        </div>

        <BaseButton
          variant="secondary"
          icon="filter_list"
          @click="showFilterOptions = !showFilterOptions"
        >
          Filtrar
        </BaseButton>
      </div>

      <BaseButton
        variant="primary"
        icon="add_circle_outline"
        @click="openNewProjectModal"
      >
        Crear Nuevo Proyecto
      </BaseButton>
    </div>

    <!-- Estado de carga -->
    <div
      v-if="projectStore.loading && proyectos.length === 0"
      class="flex items-center justify-center py-20"
    >
      <div class="text-center">
        <div
          class="w-16 h-16 border-4 border-primary border-t-transparent rounded-full animate-spin mx-auto mb-4"
        ></div>
        <p class="text-slate-500 dark:text-slate-400">Cargando proyectos...</p>
      </div>
    </div>

    <!-- Estado vacío -->
    <div
      v-else-if="!projectStore.loading && proyectos.length === 0"
      class="flex items-center justify-center py-20"
    >
      <div class="text-center max-w-md">
        <div
          class="w-20 h-20 bg-slate-100 dark:bg-slate-800 rounded-full flex items-center justify-center mx-auto mb-6"
        >
          <span class="material-icons-round text-4xl text-slate-400">
            folder_off
          </span>
        </div>
        <h3 class="text-xl font-bold text-slate-800 dark:text-white mb-2">
          No hay proyectos aún
        </h3>
        <p class="text-slate-500 dark:text-slate-400 mb-8">
          Comienza creando tu primer proyecto para gestionar tus cálculos de
          materiales
        </p>
        <BaseButton
          variant="primary"
          icon="add_circle_outline"
          @click="openNewProjectModal"
        >
          Crear Primer Proyecto
        </BaseButton>
      </div>
    </div>

    <!-- Tabla de proyectos -->
    <div
      v-else
      class="flex-1 overflow-hidden rounded-2xl border border-border-light dark:border-border-dark bg-white dark:bg-surface-dark shadow-sm"
    >
      <table class="w-full text-left border-collapse">
        <thead>
          <tr
            class="bg-slate-50 dark:bg-slate-900/50 text-slate-500 dark:text-slate-400 text-xs font-bold uppercase tracking-wider"
          >
            <th class="px-6 py-4">Nombre del Proyecto</th>
            <th class="px-6 py-4">Ubicación</th>
            <th class="px-6 py-4">Fecha</th>
            <th class="px-6 py-4 text-right">Acciones</th>
          </tr>
        </thead>
        <tbody class="divide-y divide-slate-100 dark:divide-slate-800">
          <tr
            v-for="project in paginatedProjects"
            :key="project.id"
            class="hover:bg-slate-50 dark:hover:bg-slate-800/40 transition-colors cursor-pointer"
            @click="openProject(project.id)"
          >
            <td class="px-6 py-4">
              <div class="flex items-center gap-4">
                <div class="p-2 rounded-lg bg-primary/10 text-primary">
                  <span class="material-icons-round">folder</span>
                </div>
                <div>
                  <span class="font-medium text-slate-800 dark:text-slate-200">
                    {{ project.nombre }}
                  </span>
                  <p
                    v-if="project.descripcion"
                    class="text-xs text-slate-500 dark:text-slate-400 mt-0.5"
                  >
                    {{ project.descripcion }}
                  </p>
                </div>
              </div>
            </td>
            <td class="px-6 py-4 text-slate-600 dark:text-slate-400 text-sm">
              {{ project.ubicacion || "-" }}
            </td>
            <td class="px-6 py-4 text-slate-600 dark:text-slate-400 text-sm">
              {{ formatDate(project.created_at) }}
            </td>
            <td class="px-6 py-4 text-right">
              <div class="flex items-center justify-end gap-2">
                <button
                  @click.stop="editProject(project)"
                  class="p-2 text-slate-400 dark:text-slate-500 hover:text-primary transition-colors rounded-lg hover:bg-primary/5"
                  title="Editar proyecto"
                >
                  <span class="material-icons-round text-xl">edit</span>
                </button>
                <button
                  @click.stop="confirmDeleteProject(project.id)"
                  class="p-2 text-slate-400 dark:text-slate-500 hover:text-red-500 transition-colors rounded-lg hover:bg-red-50 dark:hover:bg-red-900/10"
                  title="Eliminar proyecto"
                >
                  <span class="material-icons-round text-xl">delete</span>
                </button>
              </div>
            </td>
          </tr>
        </tbody>
      </table>
    </div>

    <!-- Paginación -->
    <footer
      v-if="proyectos.length > 0"
      class="flex items-center justify-between mt-6 pt-4 border-t border-border-light dark:border-border-dark"
    >
      <p class="text-sm text-slate-500 dark:text-slate-400">
        Mostrando {{ startIndex + 1 }}-{{ endIndex }} de
        {{ totalProjects }} proyectos
      </p>

      <div class="flex items-center gap-2">
        <button
          @click="previousPage"
          :disabled="currentPage === 1"
          class="w-8 h-8 flex items-center justify-center rounded-lg transition-colors"
          :class="
            currentPage === 1
              ? 'bg-slate-100 dark:bg-slate-800 text-slate-400 dark:text-slate-500 cursor-not-allowed'
              : 'bg-white dark:bg-slate-800 text-slate-600 dark:text-slate-300 border border-border-light dark:border-border-dark hover:bg-slate-50 dark:hover:bg-slate-700'
          "
        >
          <span class="material-icons-round text-sm">chevron_left</span>
        </button>

        <button
          v-for="page in visiblePages"
          :key="page"
          @click="currentPage = page"
          class="w-8 h-8 flex items-center justify-center rounded-lg text-sm font-bold transition-colors"
          :class="
            page === currentPage
              ? 'bg-primary text-white shadow-md shadow-primary/20'
              : 'bg-white dark:bg-slate-800 text-slate-600 dark:text-slate-300 border border-border-light dark:border-border-dark hover:bg-slate-50 dark:hover:bg-slate-700'
          "
        >
          {{ page }}
        </button>

        <button
          @click="nextPage"
          :disabled="currentPage === totalPages"
          class="w-8 h-8 flex items-center justify-center rounded-lg transition-colors"
          :class="
            currentPage === totalPages
              ? 'bg-slate-100 dark:bg-slate-800 text-slate-400 dark:text-slate-500 cursor-not-allowed'
              : 'bg-white dark:bg-slate-800 text-slate-600 dark:text-slate-300 border border-border-light dark:border-border-dark hover:bg-slate-50 dark:hover:bg-slate-700'
          "
        >
          <span class="material-icons-round text-sm">chevron_right</span>
        </button>
      </div>

      <p class="text-sm text-slate-500 dark:text-slate-400 italic">
        Página {{ currentPage }} de {{ totalPages }}
      </p>
    </footer>

    <!-- Modal de nuevo/editar proyecto -->
    <BaseModal
      v-model="showProjectModal"
      :title="editingProject ? 'Editar Proyecto' : 'Nuevo Proyecto'"
      description="Complete los datos para agregar un nuevo proyecto"
      icon="folder_shared"
    >
      <div class="space-y-4">
        <BaseInput
          v-model="projectForm.nombre"
          label="Nombre del Proyecto"
          placeholder="Ej: Casa de Campo"
          required
          :error="projectFormErrors.nombre"
        />

        <BaseInput
          v-model="projectForm.ubicacion"
          label="Ubicación (Opcional)"
          placeholder="Ej: Buenos Aires, Argentina"
          icon="location_on"
        />

        <div>
          <label
            class="block text-sm font-semibold text-slate-700 dark:text-slate-300 mb-2"
          >
            Descripción (Opcional)
          </label>
          <textarea
            v-model="projectForm.descripcion"
            rows="3"
            placeholder="Breve descripción del proyecto..."
            class="w-full px-4 py-3 rounded-xl border border-border-light dark:border-border-dark bg-white dark:bg-surface-dark text-slate-900 dark:text-white placeholder:text-slate-400 dark:placeholder:text-slate-500 focus:ring-2 focus:ring-primary focus:border-transparent outline-none transition-all resize-none"
          ></textarea>
        </div>
      </div>

      <template #footer>
        <div class="flex items-center gap-4">
          <BaseButton variant="ghost" full-width @click="closeProjectModal">
            Cancelar
          </BaseButton>
          <BaseButton
            variant="primary"
            full-width
            :loading="saving"
            @click="saveProject"
          >
            {{ editingProject ? "Actualizar" : "Guardar" }} Proyecto
          </BaseButton>
        </div>
      </template>
    </BaseModal>

    <!-- Modal de confirmación de eliminación -->
    <BaseModal
      v-model="showDeleteModal"
      title="Eliminar Proyecto"
      description="Esta acción no se puede deshacer"
      icon="delete_forever"
    >
      <p class="text-slate-600 dark:text-slate-400">
        ¿Está seguro que desea eliminar este proyecto? Todos los cálculos
        asociados también serán eliminados.
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
            @click="deleteProject"
          >
            Eliminar Proyecto
          </BaseButton>
        </div>
      </template>
    </BaseModal>
  </AppLayout>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from "vue";
import { useRouter } from "vue-router";
import { useProjectStore, type Proyecto } from "@/stores/projects";
import AppLayout from "@/components/layout/AppLayout.vue";
import BaseButton from "@/components/ui/BaseButton.vue";
import BaseInput from "@/components/ui/BaseInput.vue";
import BaseModal from "@/components/ui/BaseModal.vue";

const router = useRouter();
const projectStore = useProjectStore();

// Estado
const searchQuery = ref("");
const showFilterOptions = ref(false);
const showProjectModal = ref(false);
const showDeleteModal = ref(false);
const editingProject = ref<Proyecto | null>(null);
const projectToDelete = ref<string | null>(null);
const saving = ref(false);
const deleting = ref(false);

// Formulario
const projectForm = ref({
  nombre: "",
  descripcion: "",
  ubicacion: "",
});

const projectFormErrors = ref({
  nombre: "",
});

// Paginación
const currentPage = ref(1);
const itemsPerPage = 10;

// Computed
const proyectos = computed(() => projectStore.proyectosActivos);

const filteredProjects = computed(() => {
  let filtered = proyectos.value;

  if (searchQuery.value) {
    filtered = filtered.filter((p) =>
      p.nombre.toLowerCase().includes(searchQuery.value.toLowerCase()),
    );
  }

  return filtered;
});

const paginatedProjects = computed(() => {
  const start = (currentPage.value - 1) * itemsPerPage;
  const end = start + itemsPerPage;
  return filteredProjects.value.slice(start, end);
});

const totalProjects = computed(() => filteredProjects.value.length);
const totalPages = computed(() =>
  Math.ceil(totalProjects.value / itemsPerPage),
);
const startIndex = computed(() => (currentPage.value - 1) * itemsPerPage);
const endIndex = computed(() =>
  Math.min(startIndex.value + itemsPerPage, totalProjects.value),
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
const formatDate = (dateString?: string) => {
  if (!dateString) return "-";

  const date = new Date(dateString);
  return new Intl.DateTimeFormat("es-AR", {
    day: "2-digit",
    month: "2-digit",
    year: "numeric",
  }).format(date);
};

const openNewProjectModal = () => {
  editingProject.value = null;
  projectForm.value = {
    nombre: "",
    descripcion: "",
    ubicacion: "",
  };
  projectFormErrors.value = { nombre: "" };
  showProjectModal.value = true;
};

const editProject = (project: Proyecto) => {
  editingProject.value = project;
  projectForm.value = {
    nombre: project.nombre,
    descripcion: project.descripcion || "",
    ubicacion: project.ubicacion || "",
  };
  projectFormErrors.value = { nombre: "" };
  showProjectModal.value = true;
};

const closeProjectModal = () => {
  showProjectModal.value = false;
  editingProject.value = null;
};

const validateForm = () => {
  let isValid = true;

  if (!projectForm.value.nombre.trim()) {
    projectFormErrors.value.nombre = "El nombre es requerido";
    isValid = false;
  } else {
    projectFormErrors.value.nombre = "";
  }

  return isValid;
};

const saveProject = async () => {
  if (!validateForm()) return;

  saving.value = true;

  try {
    if (editingProject.value) {
      // Actualizar proyecto existente
      await projectStore.actualizarProyecto(editingProject.value.id, {
        nombre: projectForm.value.nombre,
        descripcion: projectForm.value.descripcion || undefined,
        ubicacion: projectForm.value.ubicacion || undefined,
      });
    } else {
      // Crear nuevo proyecto
      await projectStore.crearProyecto({
        nombre: projectForm.value.nombre,
        descripcion: projectForm.value.descripcion || undefined,
        ubicacion: projectForm.value.ubicacion || undefined,
      });
    }

    closeProjectModal();
  } catch (error: any) {
    console.error("Error al guardar proyecto:", error);
    alert(
      `Error al guardar el proyecto: ${error.message || "Error desconocido"}`,
    );
  } finally {
    saving.value = false;
  }
};

const confirmDeleteProject = (id: string) => {
  projectToDelete.value = id;
  showDeleteModal.value = true;
};

const deleteProject = async () => {
  if (!projectToDelete.value) return;

  deleting.value = true;

  try {
    await projectStore.eliminarProyecto(projectToDelete.value);
    showDeleteModal.value = false;
    projectToDelete.value = null;
  } catch (error: any) {
    console.error("Error al eliminar proyecto:", error);
    alert(
      `Error al eliminar el proyecto: ${error.message || "Error desconocido"}`,
    );
  } finally {
    deleting.value = false;
  }
};

const openProject = (id: string) => {
  router.push(`/projects/${id}/calculations`);
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
  // Cargar proyectos al montar el componente
  try {
    await projectStore.cargarProyectos();
  } catch (error) {
    console.error("Error al cargar proyectos:", error);
  }
});
</script>
