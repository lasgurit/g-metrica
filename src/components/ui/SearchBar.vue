<template>
  <!-- Contenedor del input -->
  <div class="relative flex-1" :class="containerClass">
    <!-- Ícono de búsqueda -->
    <span
      class="material-icons-round absolute left-4 top-1/2 -translate-y-1/2 text-slate-400 dark:text-slate-500"
    >
      search
    </span>

    <!-- Input -->
    <input
      :value="modelValue"
      @input="onInput"
      type="text"
      :placeholder="placeholder"
      class="w-full pl-12 pr-4 py-3 rounded-xl border border-border-light dark:border-border-dark bg-white dark:bg-surface-dark text-slate-900 dark:text-white placeholder:text-slate-400 focus:ring-2 focus:ring-primary focus:border-transparent outline-none transition-all"
    />

    <!-- Botón para limpiar el input -->
    <button
      v-if="modelValue && clearable"
      @click="clear"
      class="absolute right-4 top-1/2 -translate-y-1/2 text-slate-400 hover:text-slate-600 dark:hover:text-slate-300 transition-colors"
    >
      <span class="material-icons-round text-lg">close</span>
    </button>
  </div>
</template>

<script setup lang="ts">
interface Props {
  modelValue: string; // Valor del input (v-model)
  placeholder?: string;
  clearable?: boolean; // Muestra botón de limpiar
  containerClass?: string;
}

// Valores por defecto
const props = withDefaults(defineProps<Props>(), {
  placeholder: "Buscar...",
  clearable: true,
});

// Emit para v-model
const emit = defineEmits<{
  "update:modelValue": [value: string];
}>();

// Maneja escritura en el input
const onInput = (event: Event) => {
  const target = event.target as HTMLInputElement;
  emit("update:modelValue", target.value);
};

// Limpia el input
const clear = () => {
  emit("update:modelValue", "");
};
</script>
