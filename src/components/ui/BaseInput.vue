<template>
  <!-- Contenedor principal del input -->
  <div class="w-full">
    <!-- Label del input (opcional) -->
    <label
      v-if="label"
      :for="inputId"
      class="block text-sm font-semibold text-slate-700 dark:text-slate-300 mb-2"
    >
      {{ label }}
      <!-- Indicador de campo obligatorio -->
      <span v-if="required" class="text-red-500">*</span>
    </label>

    <!-- Wrapper del input para posicionar el icono -->
    <div class="relative">
      <!-- Ícono dentro del input (opcional) -->
      <span
        v-if="icon"
        class="material-icons-round absolute left-4 top-1/2 -translate-y-1/2 text-slate-400 dark:text-slate-500"
      >
        {{ icon }}
      </span>

      <!-- Campo de entrada -->
      <input
        :id="inputId"
        :type="type"
        :value="modelValue"
        :placeholder="placeholder"
        :disabled="disabled"
        :required="required"
        @input="onInput"
        class="w-full px-4 py-3 rounded-xl border border-border-light dark:border-border-dark bg-white dark:bg-surface-dark text-slate-900 dark:text-white placeholder:text-slate-400 dark:placeholder:text-slate-500 focus:ring-2 focus:ring-primary focus:border-transparent outline-none transition-all disabled:opacity-50 disabled:cursor-not-allowed"
        :class="[
          icon ? 'pl-12' : '', // Padding extra si hay ícono
          error ? 'border-red-500 focus:ring-red-500' : '', // Estilos de error
        ]"
      />
    </div>

    <!-- Mensaje de error -->
    <p v-if="error" class="mt-1 text-sm text-red-500">
      {{ error }}
    </p>

    <!-- Texto de ayuda (hint) -->
    <p v-else-if="hint" class="mt-1 text-sm text-slate-500 dark:text-slate-400">
      {{ hint }}
    </p>
  </div>
</template>

<script setup lang="ts">
import { computed } from "vue";

/**
 * Props del input
 *
 * modelValue: valor del input (v-model)
 * label: texto del label
 * type: tipo de input (text, email, number, etc.)
 * placeholder: texto de ayuda
 * icon: ícono a mostrar dentro del input
 * disabled: deshabilita el input
 * required: marca el campo como obligatorio
 * error: mensaje de error
 * hint: texto de ayuda debajo del input
 */
interface Props {
  modelValue: string | number;
  label?: string;
  type?: string;
  placeholder?: string;
  icon?: string;
  disabled?: boolean;
  required?: boolean;
  error?: string;
  hint?: string;
}

/**
 * Valores por defecto de las props
 */
const props = withDefaults(defineProps<Props>(), {
  type: "text",
});

/**
 * Emit para v-model
 */
const emit = defineEmits<{
  "update:modelValue": [value: string | number];
}>();

/**
 * ID único para asociar el label con el input
 * Evita conflictos cuando hay múltiples inputs en pantalla
 */
const inputId = computed(() => {
  return `input-${Math.random().toString(36).substr(2, 9)}`;
});

/**
 * Maneja el evento input
 * Emite el nuevo valor al componente padre
 */
const onInput = (event: Event) => {
  const target = event.target as HTMLInputElement;
  emit("update:modelValue", target.value);
};
</script>
