<template>
  <!-- Contenedor principal del item de resumen -->
  <div
    class="p-4 bg-slate-50 dark:bg-slate-800 border border-slate-200 dark:border-slate-700 rounded-xl flex items-center justify-between"
  >
    <!-- Sección izquierda: etiqueta + acción -->
    <div class="flex-1">
      <!-- Texto descriptivo del valor -->
      <p class="text-sm font-semibold text-slate-900 dark:text-white">
        {{ label }}
      </p>

      <!-- Acción secundaria (placeholder para futuras funcionalidades) -->
      <button
        class="text-[10px] text-slate-400 dark:text-slate-500 uppercase hover:text-primary dark:hover:text-primary transition-colors"
      >
        Ver Detalles
      </button>
    </div>

    <!-- Campo editable (solo si editable = true) -->
    <input
      v-if="editable"
      v-model="editableValue"
      type="text"
      class="text-lg font-bold text-slate-900 dark:text-white bg-transparent border-none text-right w-24 focus:outline-none focus:ring-2 focus:ring-primary/20 rounded px-2"
      @blur="updateValue"
    />
    <!-- Cuando el input pierde el foco, se emite el nuevo valor -->

    <!-- Valor solo lectura (cuando editable = false) -->
    <span v-else class="text-lg font-bold text-slate-900 dark:text-white">
      {{ value }}
    </span>
  </div>
</template>

<script setup lang="ts">
import { ref, watch } from "vue";

/**
 * Props del componente
 *
 * label: texto descriptivo del valor (ej: "Superficie total")
 * value: valor a mostrar o editar
 * editable: define si el valor puede modificarse desde el componente
 */
interface Props {
  label: string;
  value: string | number;
  editable?: boolean;
}

/**
 * Valores por defecto de las props
 * Si editable no se pasa, el valor se muestra en modo solo lectura
 */
const props = withDefaults(defineProps<Props>(), {
  editable: false,
});

/**
 * Evento emitido al modificar el valor
 * Sigue el patrón v-model: update:value
 */
const emit = defineEmits<{
  "update:value": [value: string | number];
}>();

/**
 * Copia local del valor para edición
 * Evita modificar directamente la prop (anti-pattern en Vue)
 */
const editableValue = ref(props.value);

/* Sincroniza el valor interno cuando cambia la prop desde el padre */
watch(
  () => props.value,
  (newValue) => {
    editableValue.value = newValue;
  },
);

/**
 * Emite el nuevo valor al perder el foco del input
 * Permite al componente padre actualizar su estado
 */
const updateValue = () => {
  emit("update:value", editableValue.value);
};
</script>
