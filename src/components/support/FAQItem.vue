<template>
  <!-- Contenedor principal del acordeón -->
  <div
    class="border border-slate-100 dark:border-slate-700/50 rounded-xl overflow-hidden bg-slate-50 dark:bg-slate-800/40"
  >
    <!-- Botón que abre/cierra el contenido -->
    <button
      @click="isOpen = !isOpen"
      class="w-full flex items-center justify-between p-4 text-left font-semibold text-slate-700 dark:text-slate-200 hover:bg-slate-100 dark:hover:bg-slate-800 transition-colors"
    >
      <!-- Pregunta -->
      <span>{{ question }}</span>

      <!-- Ícono de expansión -->
      <span
        class="material-icons-round text-sm transition-transform"
        :class="{ 'rotate-180': isOpen }"
      >
        expand_more
      </span>
    </button>

    <!-- Animación de apertura/cierre -->
    <Transition
      enter-active-class="transition-all duration-200 ease-out"
      enter-from-class="opacity-0 max-h-0"
      enter-to-class="opacity-100 max-h-96"
      leave-active-class="transition-all duration-200 ease-in"
      leave-from-class="opacity-100 max-h-96"
      leave-to-class="opacity-0 max-h-0"
    >
      <!-- Respuesta visible solo cuando está abierto -->
      <div
        v-if="isOpen"
        class="px-4 pb-4 text-sm text-slate-600 dark:text-slate-400 leading-relaxed"
      >
        {{ answer }}
      </div>
    </Transition>
  </div>
</template>

<script setup lang="ts">
import { ref } from "vue";

/**
 * Props del componente
 *
 * question: texto de la pregunta
 * answer: contenido de la respuesta
 * open: define si el item inicia abierto
 */
interface Props {
  question: string;
  answer: string;
  open?: boolean;
}

/**
 * Valores por defecto de las props
 */
const props = withDefaults(defineProps<Props>(), {
  open: false,
});

/**
 * Estado interno del acordeón
 * Controla si el contenido está visible o no
 */
const isOpen = ref(props.open);
</script>
