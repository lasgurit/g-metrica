<template>
  <AppLayout title="Resumen Total de Materiales" :show-back-button="true">
    <!-- Breadcrumb y header del proyecto -->
    <div
      class="bg-primary rounded-xl p-8 text-white relative overflow-hidden shadow-lg shadow-primary/20 mb-8"
    >
      <div class="relative z-10">
        <p class="text-sm font-medium opacity-80 mb-1 uppercase tracking-wider">
          {{ proyectoNombre }}
        </p>
        <h1 class="text-4xl font-bold">Resumen Total de Materiales</h1>
      </div>

      <button
        @click="exportPDF"
        class="absolute top-8 right-8 bg-white/20 hover:bg-white/30 backdrop-blur-md text-white border border-white/30 px-5 py-2 rounded-lg flex items-center gap-2 transition-all"
        :disabled="!hasData"
      >
        <span class="material-icons-round text-lg">download</span>
        Exportar Resumen PDF
      </button>

      <div
        class="absolute -right-16 -bottom-16 w-64 h-64 bg-white/10 rounded-full blur-3xl"
      ></div>
    </div>

    <!-- Estado de carga -->
    <div v-if="loading" class="flex items-center justify-center py-20">
      <div class="text-center">
        <div
          class="w-16 h-16 border-4 border-primary border-t-transparent rounded-full animate-spin mx-auto mb-4"
        ></div>
        <p class="text-slate-500 dark:text-slate-400">
          Cargando resumen de materiales...
        </p>
      </div>
    </div>

    <!-- Estado vacío -->
    <div
      v-else-if="!loading && materialesPorCategoria.length === 0"
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
          No hay cálculos en este proyecto
        </h3>
        <p class="text-slate-500 dark:text-slate-400 mb-8">
          Crea tu primer cálculo para ver el resumen de materiales
        </p>
        <BaseButton variant="primary" @click="irACalculos">
          Ir a Cálculos
        </BaseButton>
      </div>
    </div>

    <!-- Tabla de materiales -->
    <div
      v-else
      class="bg-white dark:bg-surface-dark rounded-xl shadow-sm border border-border-light dark:border-border-dark overflow-hidden"
    >
      <!-- Header de tabla -->
      <div
        class="grid grid-cols-12 gap-4 px-8 py-4 border-b border-border-light dark:border-border-dark text-xs font-bold uppercase tracking-widest text-slate-400 dark:text-slate-500"
      >
        <div class="col-span-4">Material</div>
        <div class="col-span-4">Especificación</div>
        <div class="col-span-2 text-right">Cantidad Total</div>
        <div class="col-span-2 text-right">Unidad</div>
      </div>

      <!-- Contenido de tabla -->
      <div class="divide-y divide-slate-100 dark:divide-slate-800/50">
        <!-- Iterar por categorías -->
        <template
          v-for="categoria in materialesPorCategoria"
          :key="categoria.nombre"
        >
          <!-- Categoría Header -->
          <MaterialCategory :title="categoria.titulo" :icon="categoria.icono" />

          <!-- Materiales de esta categoría -->
          <MaterialRow
            v-for="material in categoria.materiales"
            :key="material.material_id"
            :material="material.material_nombre"
            :specification="getEspecificacion(material)"
            :quantity="material.cantidad_total"
            :unit="material.material_unidad"
          />
        </template>
      </div>
    </div>

    <!-- Nota de aclaración -->
    <div
      v-if="hasData"
      class="bg-amber-50 dark:bg-amber-900/20 border border-amber-200 dark:border-amber-800/50 rounded-xl p-6 flex gap-4 mt-8"
    >
      <span class="material-icons-round text-amber-500 shrink-0"
        >warning_amber</span
      >
      <div>
        <h3 class="font-bold text-amber-900 dark:text-amber-400 mb-1">
          Aclaración Importante
        </h3>
        <p
          class="text-sm text-amber-800 dark:text-amber-500/80 leading-relaxed"
        >
          Los resultados presentados son estimaciones en base a las dimensiones
          y criterios de diseño proporcionado. Las cantidades reales pueden
          variar debido a desperdicios de obra, condiciones del terreno, y
          técnicas de ejecución. Se recomienda una verificación profesional
          antes de realizar la compra final de materiales.
        </p>
      </div>
    </div>

    <!-- Footer -->
    <div class="text-center pt-8 pb-12">
      <p class="text-xs text-slate-400 dark:text-slate-600">
        © 2026 G-Métrica by GurIT. Todos los derechos reservados
      </p>
    </div>
  </AppLayout>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from "vue";
import { useRoute, useRouter } from "vue-router";
import { invoke } from "@tauri-apps/api/core";
import { useAuthStore } from "@/stores/auth";
import { useProjectStore } from "@/stores/projects";
import AppLayout from "@/components/layout/AppLayout.vue";
import BaseButton from "@/components/ui/BaseButton.vue";
import MaterialCategory from "@/components/summary/MaterialCategory.vue";
import MaterialRow from "@/components/summary/MaterialRow.vue";

const route = useRoute();
const router = useRouter();
const authStore = useAuthStore();
const projectStore = useProjectStore();

const projectId = computed(() => route.params.id as string);

// Estado
const loading = ref(false);
const proyectoNombre = ref("Cargando...");
const resumenMateriales = ref<ResumenMaterial[]>([]);

// Interface para el resumen (debe coincidir con ResumenMaterial del backend)
interface ResumenMaterial {
  material_id: string;
  material_nombre: string;
  material_unidad: string;
  material_categoria: string;
  cantidad_total: number;
}

// Computed
const hasData = computed(() => resumenMateriales.value.length > 0);

// Mapeo de categorías a títulos e iconos
const categoriasConfig: Record<string, { titulo: string; icono: string }> = {
  aglomerantes: { titulo: "CEMENTOS Y AGLOMERADOS", icono: "architecture" },
  aridos: { titulo: "ÁRIDOS", icono: "layers" },
  hierros: { titulo: "HIERROS Y REFUERZOS", icono: "reorder" },
  bloques: { titulo: "LADRILLOS Y BLOQUES", icono: "grid_on" },
  aislantes: { titulo: "AISLANTES", icono: "shield" },
  perfiles: { titulo: "PERFILES METÁLICOS", icono: "straighten" },
  chapas: { titulo: "CHAPAS", icono: "view_module" },
  terminaciones: { titulo: "TERMINACIONES", icono: "format_paint" },
  aditivos: { titulo: "ADITIVOS", icono: "science" },
  soldadura: { titulo: "SOLDADURA", icono: "flare" },
  varios: { titulo: "VARIOS", icono: "category" },
};

// Agrupar materiales por categoría
const materialesPorCategoria = computed(() => {
  const grupos: Record<string, ResumenMaterial[]> = {};

  // Agrupar
  resumenMateriales.value.forEach((material) => {
    const categoria = material.material_categoria || "varios";
    if (!grupos[categoria]) {
      grupos[categoria] = [];
    }
    grupos[categoria].push(material);
  });

  // Convertir a array con configuración
  return Object.entries(grupos)
    .map(([categoria, materiales]) => ({
      nombre: categoria,
      titulo: categoriasConfig[categoria]?.titulo || categoria.toUpperCase(),
      icono: categoriasConfig[categoria]?.icono || "category",
      materiales: materiales,
    }))
    .sort((a, b) => {
      // Orden personalizado de categorías
      const orden = [
        "aglomerantes",
        "aridos",
        "hierros",
        "bloques",
        "aislantes",
        "perfiles",
        "chapas",
        "terminaciones",
        "aditivos",
        "soldadura",
        "varios",
      ];
      const indexA = orden.indexOf(a.nombre);
      const indexB = orden.indexOf(b.nombre);
      return (indexA === -1 ? 999 : indexA) - (indexB === -1 ? 999 : indexB);
    });
});

// Obtener especificación del material (simplificado para el diseño original)
const getEspecificacion = (material: ResumenMaterial): string => {
  // Para el diseño original, la especificación es simplemente la unidad
  // Esto se puede extender en el futuro
  return material.material_unidad;
};

// Cargar resumen desde el backend
const cargarResumen = async () => {
  loading.value = true;

  try {
    // Obtener información del proyecto
    const proyecto = await projectStore.obtenerProyecto(projectId.value);
    proyectoNombre.value = proyecto.nombre;

    // Llamar al comando de Tauri que invoca la función RPC de Supabase
    const result = await invoke<ResumenMaterial[]>("get_resumen_proyecto", {
      token: authStore.token,
      proyectoId: projectId.value,
    });

    resumenMateriales.value = result;
    console.log("Resumen cargado:", result);
  } catch (error: any) {
    console.error("ERROR RESUMEN RAW:", error);
    alert(typeof error === "string" ? error : JSON.stringify(error, null, 2));
  } finally {
    loading.value = false;
  }
};

const exportPDF = () => {
  if (!hasData.value) return;

  // TODO: Implementar exportación a PDF
  console.log("Exportando resumen a PDF...");
  console.log("Datos a exportar:", {
    proyecto: proyectoNombre.value,
    materiales: resumenMateriales.value,
  });
  alert("Funcionalidad de exportación a PDF en desarrollo");
};

const irACalculos = () => {
  router.push(`/projects/${projectId.value}/calculations`);
};

// Lifecycle
onMounted(() => {
  cargarResumen();
});
</script>
