<template>
  <!-- Enlace de navegación del sidebar -->
  <router-link
    :to="path"
    class="flex items-center gap-3 px-4 py-3 rounded-lg transition-all group"
    :class="[
      // Estilo cuando el item está activo
      active
        ? 'bg-primary text-white shadow-lg shadow-primary/20'
        : // Estilo cuando el item NO está activo
          'text-slate-600 dark:text-slate-400 hover:bg-slate-100 dark:hover:bg-slate-800',
    ]"
  >
    <!-- Ícono del item -->
    <span
      class="material-icons-round text-[20px]"
      :class="[
        // Cambia de color en hover solo si el item no está activo
        !active && 'group-hover:text-primary transition-colors',
      ]"
    >
      {{ icon }}
    </span>

    <!-- Texto del item -->
    <span class="font-medium">
      {{ label }}
    </span>
  </router-link>
</template>

<script setup lang="ts">
/**
 * Props del componente SidebarItem
 *
 * icon: nombre del ícono de Material Icons
 * label: texto visible del item
 * path: ruta a la que navega el enlace
 * active: indica si la ruta actual corresponde a este item
 */
defineProps<{
  icon: string;
  label: string;
  path: string;
  active: boolean;
}>();
</script>
