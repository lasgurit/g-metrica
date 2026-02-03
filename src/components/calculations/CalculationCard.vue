<template>
  <!-- Botón principal que representa una tarjeta de cálculo -->
  <button
    class="group bg-white dark:bg-surface-dark border transition-all hover:border-primary hover:shadow-xl hover:shadow-primary/5 dark:hover:bg-slate-800/50 p-6 rounded-2xl text-left"
    :class="[
      // Si el cálculo es personalizado, usamos borde punteado
      isCustom
        ? 'border-dashed border-2 border-border-light dark:border-border-dark'
        : 'border-border-light dark:border-border-dark',
    ]"
  >
    <!-- Header de la tarjeta: etiqueta + ícono -->
    <div class="flex justify-between items-start mb-4">
      <!-- Etiqueta superior (tipo de contenido) -->
      <span
        class="text-xs font-semibold text-slate-400 uppercase tracking-wider"
      >
        Cálculo
      </span>

      <!-- Ícono del cálculo (Material Icons) -->
      <span
        class="material-icons-round text-primary group-hover:scale-110 transition-transform"
      >
        {{ icon }}
      </span>
    </div>

    <!-- Título principal del cálculo -->
    <h5 class="text-xl font-bold text-primary">
      {{ title }}
    </h5>
  </button>
</template>

<script setup lang="ts">
/**
 * Props del componente
 *
 * title: nombre del cálculo que se muestra en la tarjeta
 * icon: nombre del ícono de Material Icons (ej: "calculate", "home", etc.)
 * isCustom: indica si el cálculo es personalizado (opcional)
 *           → cambia el estilo del borde a punteado
 */
defineProps<{
  title: string;
  icon: string;
  isCustom?: boolean;
}>();
</script>
