<template>
  <!-- Layout principal de la aplicación -->
  <div class="min-h-screen flex bg-background-light dark:bg-background-dark">
    <!-- Sidebar lateral (navegación principal) -->
    <AppSidebar />

    <!-- Contenedor del contenido principal -->
    <div class="flex-1 ml-64 flex flex-col min-h-screen">
      <!-- Header superior con título y navegación -->
      <AppHeader :title="pageTitle" :show-back-button="showBackButton" />

      <!-- Contenido específico de cada página -->
      <main class="flex-1 p-8 overflow-auto custom-scrollbar">
        <!-- Slot donde se renderiza la vista actual -->
        <slot />
      </main>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed } from "vue";
import { useRoute } from "vue-router";
import AppSidebar from "./AppSideBar.vue";
import AppHeader from "./AppHeader.vue";

/**
 * Props del layout
 *
 * title: permite forzar un título desde la vista
 * showBackButton: indica si se muestra el botón volver en el header
 */
interface Props {
  title?: string;
  showBackButton?: boolean;
}

/**
 * Valores por defecto de las props
 */
const props = withDefaults(defineProps<Props>(), {
  showBackButton: false,
});

/**
 * Ruta actual
 * Se usa para obtener metadatos como el título
 */
const route = useRoute();

/**
 * Título que se muestra en el header
 *
 * Prioridad:
 * 1. Prop "title" pasada al layout
 * 2. route.meta.title (definido en el router)
 * 3. Valor por defecto ("GMétrica")
 */
const pageTitle = computed(() => {
  return props.title || (route.meta.title as string) || "GMétrica";
});
</script>
