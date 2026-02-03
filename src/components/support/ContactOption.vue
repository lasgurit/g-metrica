<template>
  <!-- Enlace externo presentado como card -->
  <a
    :href="href"
    target="_blank"
    rel="noopener noreferrer"
    class="flex items-center justify-between p-5 rounded-xl group hover:shadow-md transition-all"
    :class="colorClasses"
  >
    <!-- Sección izquierda: ícono + textos -->
    <div class="flex items-center gap-4">
      <!-- Contenedor del ícono -->
      <div
        class="w-12 h-12 rounded-lg flex items-center justify-center"
        :class="iconBgClasses"
      >
        <!-- Ícono principal -->
        <span class="material-icons-round text-2xl" :class="iconTextClasses">
          {{ icon }}
        </span>
      </div>

      <!-- Título y subtítulo -->
      <div>
        <!-- Título del enlace -->
        <h3 class="font-bold" :class="titleClasses">
          {{ title }}
        </h3>

        <!-- Subtítulo descriptivo -->
        <p class="text-xs" :class="subtitleClasses">
          {{ subtitle }}
        </p>
      </div>
    </div>

    <!-- Ícono indicador de enlace externo -->
    <span
      class="material-icons-round group-hover:translate-x-1 transition-transform"
      :class="arrowClasses"
    >
      open_in_new
    </span>
  </a>
</template>

<script setup lang="ts">
import { computed } from "vue";

/**
 * Props del componente
 *
 * icon: ícono principal (Material Icons)
 * title: título del enlace
 * subtitle: texto descriptivo
 * color: variante de color visual del card
 * href: URL del enlace externo
 */
interface Props {
  icon: string;
  title: string;
  subtitle: string;
  color: "emerald" | "blue" | "amber" | "red";
  href: string;
}

const props = defineProps<Props>();

/**
 * Clases base del card según el color elegido
 */
const colorClasses = computed(() => {
  const colors = {
    emerald:
      "bg-emerald-50 dark:bg-emerald-950/20 border border-emerald-100 dark:border-emerald-900/30",
    blue: "bg-blue-50 dark:bg-blue-950/20 border border-blue-100 dark:border-blue-900/30",
    amber:
      "bg-amber-50 dark:bg-amber-950/20 border border-amber-100 dark:border-amber-900/30",
    red: "bg-red-50 dark:bg-red-950/20 border border-red-100 dark:border-red-900/30",
  };

  return colors[props.color];
});

/**
 * Clases de fondo del contenedor del ícono
 */
const iconBgClasses = computed(() => {
  const colors = {
    emerald: "bg-emerald-500/10",
    blue: "bg-blue-500/10",
    amber: "bg-amber-500/10",
    red: "bg-red-500/10",
  };

  return colors[props.color];
});

/**
 * Clases de color del ícono
 */
const iconTextClasses = computed(() => {
  const colors = {
    emerald: "text-emerald-600 dark:text-emerald-400",
    blue: "text-blue-600 dark:text-blue-400",
    amber: "text-amber-600 dark:text-amber-400",
    red: "text-red-600 dark:text-red-400",
  };

  return colors[props.color];
});

/**
 * Clases del título según el color
 */
const titleClasses = computed(() => {
  const colors = {
    emerald: "text-emerald-800 dark:text-emerald-300",
    blue: "text-blue-800 dark:text-blue-300",
    amber: "text-amber-800 dark:text-amber-300",
    red: "text-red-800 dark:text-red-300",
  };

  return colors[props.color];
});

/**
 * Clases del subtítulo según el color
 */
const subtitleClasses = computed(() => {
  const colors = {
    emerald: "text-emerald-600 dark:text-emerald-500",
    blue: "text-blue-600 dark:text-blue-500",
    amber: "text-amber-600 dark:text-amber-500",
    red: "text-red-600 dark:text-red-500",
  };

  return colors[props.color];
});

/**
 * Clases del ícono de flecha (enlace externo)
 */
const arrowClasses = computed(() => {
  const colors = {
    emerald: "text-emerald-400",
    blue: "text-blue-400",
    amber: "text-amber-400",
    red: "text-red-400",
  };

  return colors[props.color];
});
</script>
