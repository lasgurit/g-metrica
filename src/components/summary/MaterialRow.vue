<template>
  <!-- Fila de información de material -->
  <div
    class="grid grid-cols-12 gap-4 px-8 py-3.5 items-center hover:bg-slate-50 dark:hover:bg-slate-800/30 transition-colors"
  >
    <!-- Nombre del material -->
    <div class="col-span-4 text-slate-700 dark:text-slate-200 font-medium">
      {{ material }}
    </div>

    <!-- Especificación o detalle adicional -->
    <div class="col-span-4 text-slate-500 dark:text-slate-400">
      {{ specification }}
    </div>

    <!-- Cantidad formateada -->
    <div class="col-span-2 text-right font-bold dark:text-white">
      {{ formatQuantity(quantity) }}
    </div>

    <!-- Unidad de medida -->
    <div class="col-span-2 text-right text-slate-500">
      {{ unit }}
    </div>
  </div>
</template>

<script setup lang="ts">
/**
 * Props del componente
 *
 * material: nombre del material
 * specification: detalle o especificación
 * quantity: cantidad numérica
 * unit: unidad de medida
 */
interface Props {
  material: string;
  specification: string;
  quantity: number;
  unit: string;
}

defineProps<Props>();

/**
 * Formatea la cantidad:
 * - Si es entero, se muestra sin decimales
 * - Si tiene decimales, se muestran hasta 2
 */
const formatQuantity = (qty: number): string => {
  // Número entero → sin decimales
  if (Number.isInteger(qty)) {
    return qty.toString();
  }

  // Número decimal → hasta 2 decimales
  return qty.toFixed(2);
};
</script>
