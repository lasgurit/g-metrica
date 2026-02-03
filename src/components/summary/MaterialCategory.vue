<template>
  <!-- Contenedor del encabezado de sección -->
  <div class="bg-slate-50/50 dark:bg-slate-900/30 px-8 py-4">
    <!-- Ícono + título de la sección -->
    <div
      class="flex items-center gap-2 text-primary font-bold text-sm uppercase"
    >
      <!-- Ícono representativo de la sección -->
      <span class="material-icons-round text-lg">{{ icon }}</span>

      <!-- Título de la sección -->
      {{ title }}
    </div>
  </div>
</template>

<script setup lang="ts">
/**
 * Props del componente
 *
 * title: título de la sección
 * icon: nombre del ícono de Material Icons
 */
defineProps<{
  title: string;
  icon: string;
}>();
</script>
