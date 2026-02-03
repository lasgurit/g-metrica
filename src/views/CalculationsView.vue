<template>
  <AppLayout title="Gestión de Cálculos" :show-back-button="true">
    <!-- Barra de búsqueda y filtros -->
    <div class="flex flex-col md:flex-row gap-4 mb-8">
      <SearchBar
        v-model="searchQuery"
        placeholder="Buscar cálculo..."
        class="flex-1"
      />

      <BaseButton
        variant="secondary"
        icon="tune"
        @click="showFilters = !showFilters"
      >
        Filtrar
      </BaseButton>
    </div>

    <!-- Breadcrumb y nombre del proyecto -->
    <div
      class="bg-primary rounded-2xl p-8 mb-10 text-white relative overflow-hidden shadow-lg shadow-primary/20"
    >
      <!-- Efecto de fondo -->
      <div
        class="absolute top-0 right-0 w-64 h-64 bg-white/10 rounded-full -mr-20 -mt-20 blur-3xl"
      ></div>

      <!-- Breadcrumb -->
      <nav
        aria-label="Breadcrumb"
        class="flex text-xs font-medium uppercase tracking-wider mb-2 opacity-90 relative z-10"
      >
        <ol class="flex items-center space-x-2">
          <li>
            <span class="material-icons-round text-sm">home</span>
          </li>
          <li class="flex items-center gap-2">
            <span class="material-icons-round text-xs">chevron_right</span>
            <router-link to="/projects" class="hover:underline"
              >Proyectos</router-link
            >
          </li>
          <li class="flex items-center gap-2">
            <span class="material-icons-round text-xs">chevron_right</span>
            <span>Cálculos</span>
          </li>
        </ol>
      </nav>

      <!-- Nombre del proyecto -->
      <h3 class="text-4xl font-black relative z-10">{{ proyectoNombre }}</h3>
    </div>

    <!-- Estado de carga -->
    <div
      v-if="calculoStore.loading && tiposCalculoFiltrados.length === 0"
      class="flex items-center justify-center py-20"
    >
      <div class="text-center">
        <div
          class="w-16 h-16 border-4 border-primary border-t-transparent rounded-full animate-spin mx-auto mb-4"
        ></div>
        <p class="text-slate-500 dark:text-slate-400">
          Cargando tipos de cálculo...
        </p>
      </div>
    </div>

    <!-- Título de sección -->
    <div
      v-else
      class="flex items-center gap-2 mb-6 text-slate-500 dark:text-slate-400"
    >
      <div
        class="w-8 h-8 rounded-lg bg-primary/10 flex items-center justify-center"
      >
        <span class="material-icons-round text-primary text-[20px]"
          >calculate</span
        >
      </div>
      <h4 class="font-bold uppercase tracking-widest text-sm">
        Cálculos de Materiales
      </h4>
    </div>

    <!-- Grid de cálculos -->
    <div
      v-if="!calculoStore.loading || tiposCalculoFiltrados.length > 0"
      class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-4 gap-4 mb-12"
    >
      <CalculationCard
        v-for="tipo in tiposCalculoFiltrados"
        :key="tipo.id"
        :title="tipo.nombre"
        :icon="getIconForTipo(tipo.codigo)"
        :is-custom="tipo.es_personalizado"
        @click="openCalculation(tipo)"
      />
    </div>

    <!-- Botón de resumen total -->
    <div class="flex justify-center mt-8">
      <BaseButton
        variant="primary"
        size="lg"
        icon="receipt_long"
        @click="openSummary"
        class="shadow-xl shadow-primary/30"
      >
        Resumen Total
      </BaseButton>
    </div>
  </AppLayout>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from "vue";
import { useRoute, useRouter } from "vue-router";
import { useCalculoStore } from "@/stores/calculos";
import { useProjectStore } from "@/stores/projects";
import AppLayout from "@/components/layout/AppLayout.vue";
import BaseButton from "@/components/ui/BaseButton.vue";
import SearchBar from "@/components/ui/SearchBar.vue";
import CalculationCard from "@/components/calculations/CalculationCard.vue";

const route = useRoute();
const router = useRouter();
const calculoStore = useCalculoStore();
const proyectoStore = useProjectStore();

const projectId = computed(() => route.params.id as string);
const searchQuery = ref("");
const showFilters = ref(false);
const proyectoNombre = ref("Cargando...");

// Computed
const tiposCalculoFiltrados = computed(() => {
  let tipos = calculoStore.tiposCalculoActivos;

  if (searchQuery.value) {
    tipos = tipos.filter((t) =>
      t.nombre.toLowerCase().includes(searchQuery.value.toLowerCase()),
    );
  }

  return tipos;
});

// Métodos
const getIconForTipo = (codigo: string): string => {
  const iconos: Record<string, string> = {
    viga_fundacion: "foundation",
    contrapisos: "layers",
    mamposteria: "grid_on",
    revoque_exterior: "format_paint",
    revoque_interior: "format_paint",
    techo: "roofing",
    pisos: "grid_view",
    cielorraso: "crop_landscape",
    banio_estandar: "bathtub",
    cocina_estandar: "kitchen",
    electricidad: "electrical_services",
    personalizado: "auto_awesome",
  };
  return iconos[codigo] || "calculate";
};

const openCalculation = (tipo: any) => {
  // Solo viga de fundación está implementada
  if (tipo.codigo === "viga_fundacion") {
    router.push(`/projects/${projectId.value}/calculations/${tipo.codigo}`);
  } else {
    alert(
      `El cálculo "${tipo.nombre}" estará disponible próximamente.\n\nActualmente solo está implementado el cálculo de Viga de Fundación.`,
    );
  }
};

const openSummary = () => {
  router.push(`/projects/${projectId.value}/summary`);
};

// Lifecycle
onMounted(async () => {
  try {
    // Cargar tipos de cálculo
    await calculoStore.cargarTiposCalculo();

    // Cargar nombre del proyecto
    const proyecto = await proyectoStore.obtenerProyecto(projectId.value);
    proyectoNombre.value = proyecto.nombre;
  } catch (error) {
    console.error("Error al cargar datos:", error);
    proyectoNombre.value = "Proyecto";
  }
});
</script>
