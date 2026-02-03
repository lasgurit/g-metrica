<template>
  <!-- Item de lista / feature dentro de un menú o sección -->
  <div
    class="flex items-center justify-between p-3 rounded-xl hover:bg-slate-50 dark:hover:bg-slate-800 transition-colors"
  >
    <!-- Sección izquierda: ícono + etiqueta -->
    <div class="flex items-center gap-3">
      <!-- Ícono representativo (Material Icons) -->
      <span class="material-icons-round text-primary">
        {{ icon }}
      </span>

      <!-- Nombre de la funcionalidad -->
      <span class="font-medium dark:text-slate-300">
        {{ label }}
      </span>
    </div>

    <!-- Estado ACTIVO -->
    <span
      v-if="active"
      class="text-[10px] uppercase font-bold px-2 py-0.5 bg-emerald-100 dark:bg-emerald-900/30 text-emerald-600 dark:text-emerald-400 rounded-full"
    >
      Activo
    </span>

    <!-- Estado INACTIVO / PRÓXIMAMENTE -->
    <span
      v-else
      class="text-[10px] uppercase font-bold px-2 py-0.5 bg-slate-100 dark:bg-slate-800 text-slate-500 dark:text-slate-400 rounded-full"
    >
      Próximamente
    </span>
  </div>
</template>

<script setup lang="ts">
/**
 * Props del componente
 *
 * icon: nombre del ícono de Material Icons
 * label: texto descriptivo del item
 * active: define si la funcionalidad está disponible o no
 */
defineProps<{
  icon: string;
  label: string;
  active?: boolean;
}>();
</script>
