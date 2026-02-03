<template>
  <!-- Botón base reutilizable -->
  <button
    :type="type"
    :disabled="disabled || loading"
    class="inline-flex items-center justify-center gap-2 font-semibold transition-all active:scale-95 disabled:opacity-50 disabled:cursor-not-allowed"
    :class="[
      sizeClasses, // Clases según el tamaño (sm, md, lg)
      variantClasses, // Clases según la variante visual
      rounded ? 'rounded-xl' : '',
      fullWidth ? 'w-full' : '',
    ]"
  >
    <!-- Ícono de carga (spinner) -->
    <span v-if="loading" class="material-icons-round animate-spin">
      refresh
    </span>

    <!-- Ícono opcional (si no está cargando) -->
    <span
      v-if="icon && !loading"
      class="material-icons-round"
      :class="iconSize"
    >
      {{ icon }}
    </span>

    <!-- Contenido del botón -->
    <slot />
  </button>
</template>

<script setup lang="ts">
import { computed } from "vue";

// Props configurables del botón
interface Props {
  variant?: "primary" | "secondary" | "danger" | "ghost";
  size?: "sm" | "md" | "lg";
  type?: "button" | "submit" | "reset";
  icon?: string;
  disabled?: boolean;
  loading?: boolean;
  rounded?: boolean;
  fullWidth?: boolean;
}

// Valores por defecto
const props = withDefaults(defineProps<Props>(), {
  variant: "primary",
  size: "md",
  type: "button",
  rounded: true,
  fullWidth: false,
});

// Clases según el tamaño del botón
const sizeClasses = computed(() => {
  const sizes = {
    sm: "px-4 py-2 text-sm",
    md: "px-6 py-3 text-base",
    lg: "px-8 py-4 text-lg",
  };
  return sizes[props.size];
});

// Tamaño del ícono según el tamaño del botón
const iconSize = computed(() => {
  const sizes = {
    sm: "text-base",
    md: "text-lg",
    lg: "text-xl",
  };
  return sizes[props.size];
});

// Clases visuales según la variante
const variantClasses = computed(() => {
  const variants = {
    primary:
      "bg-primary text-white shadow-lg shadow-primary/30 hover:bg-primary/90",
    secondary:
      "bg-slate-100 dark:bg-slate-800 text-slate-600 dark:text-slate-300 hover:bg-slate-200 dark:hover:bg-slate-700",
    danger:
      "bg-red-500 text-white shadow-lg shadow-red-500/30 hover:bg-red-600",
    ghost:
      "text-slate-600 dark:text-slate-400 hover:bg-slate-100 dark:hover:bg-slate-800",
  };
  return variants[props.variant];
});
</script>
