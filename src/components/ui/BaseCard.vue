<template>
  <!-- Contenedor principal tipo card -->
  <div
    class="bg-white dark:bg-surface-dark border border-border-light dark:border-border-dark rounded-xl overflow-hidden transition-all"
    :class="[
      // Efectos hover opcionales
      hoverable &&
        'hover:border-primary hover:shadow-xl hover:shadow-primary/5 cursor-pointer',
      padding && paddingClasses, // Padding interno
      shadow && 'shadow-sm', // Sombra base
    ]"
  >
    <!-- Header (opcional) -->
    <div
      v-if="$slots.header || title"
      class="px-6 py-4 border-b border-border-light dark:border-border-dark"
      :class="headerClass"
    >
      <!-- Slot personalizado de header -->
      <slot name="header">
        <!-- Título por defecto si no se usa el slot -->
        <h3 class="text-lg font-bold text-slate-800 dark:text-white">
          {{ title }}
        </h3>
      </slot>
    </div>

    <!-- Body principal -->
    <div :class="!$slots.header && !title ? paddingClasses : ''">
      <slot />
    </div>

    <!-- Footer (opcional) -->
    <div
      v-if="$slots.footer"
      class="px-6 py-4 border-t border-border-light dark:border-border-dark bg-slate-50 dark:bg-slate-900/50"
    >
      <slot name="footer" />
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed } from "vue";

// Props configurables de la card
interface Props {
  title?: string;
  hoverable?: boolean;
  padding?: boolean;
  shadow?: boolean;
  headerClass?: string;
}

// Valores por defecto
const props = withDefaults(defineProps<Props>(), {
  hoverable: false,
  padding: true,
  shadow: true,
});

// Clases de padding interno
const paddingClasses = computed(() => {
  return props.padding ? "p-6" : "";
});
</script>
