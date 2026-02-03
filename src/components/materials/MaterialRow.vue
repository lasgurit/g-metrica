<template>
  <tr class="hover:bg-slate-50 dark:hover:bg-slate-800/40 transition-colors">
    <td class="px-6 py-4">
      <div class="flex items-center gap-4">
        <div
          class="w-10 h-10 rounded-lg flex items-center justify-center"
          :class="getCategoryBgClass(material.categoria)"
        >
          <span
            class="material-icons-round"
            :class="getCategoryIconColor(material.categoria)"
          >
            {{ getCategoryIcon(material.categoria) }}
          </span>
        </div>
        <div>
          <div class="flex items-center gap-2">
            <span class="font-medium text-slate-800 dark:text-slate-200">
              {{ material.nombre }}
            </span>
            <span
              v-if="material.es_sistema"
              class="text-[10px] uppercase font-bold px-2 py-0.5 bg-blue-100 dark:bg-blue-900/30 text-blue-600 dark:text-blue-400 rounded-full"
            >
              Sistema
            </span>
          </div>
          <p
            v-if="material.descripcion"
            class="text-xs text-slate-500 dark:text-slate-400 mt-0.5"
          >
            {{ material.descripcion }}
          </p>
        </div>
      </div>
    </td>
    <td class="px-6 py-4">
      <span
        class="inline-flex items-center px-2.5 py-1 rounded-full text-xs font-medium"
        :class="getCategoryBadgeClass(material.categoria)"
      >
        {{ getCategoryLabel(material.categoria) }}
      </span>
    </td>
    <td class="px-6 py-4 text-slate-600 dark:text-slate-400">
      {{ material.unidad }}
    </td>
    <td class="px-6 py-4 text-slate-600 dark:text-slate-400">
      {{ formatDate(material.updated_at) }}
    </td>
    <td class="px-6 py-4 text-right">
      <div class="flex items-center justify-end gap-2">
        <button
          v-if="!material.es_sistema"
          @click.stop="$emit('edit', material)"
          class="p-2 text-slate-400 dark:text-slate-500 hover:text-primary transition-colors rounded-lg hover:bg-primary/5"
          title="Editar material"
        >
          <span class="material-icons-round text-xl">edit</span>
        </button>
        <button
          v-if="!material.es_sistema"
          @click.stop="$emit('delete', material.id)"
          class="p-2 text-slate-400 dark:text-slate-500 hover:text-red-500 transition-colors rounded-lg hover:bg-red-50 dark:hover:bg-red-900/10"
          title="Eliminar material"
        >
          <span class="material-icons-round text-xl">delete</span>
        </button>
        <span
          v-if="material.es_sistema"
          class="text-xs text-slate-400 dark:text-slate-600 italic"
        >
          No editable
        </span>
      </div>
    </td>
  </tr>
</template>

<script setup lang="ts">
import type { Material } from "@/stores/materials";

defineProps<{
  material: Material;
}>();

defineEmits<{
  edit: [material: Material];
  delete: [id: string];
}>();

const formatDate = (date?: string): string => {
  if (!date) return "-";

  return new Intl.DateTimeFormat("es-AR", {
    day: "2-digit",
    month: "2-digit",
    year: "numeric",
  }).format(new Date(date));
};

const getCategoryIcon = (category?: string): string => {
  const icons: Record<string, string> = {
    aglomerantes: "architecture",
    aridos: "layers",
    hierros: "reorder",
    bloques: "grid_on",
    aislantes: "shield",
    perfiles: "straighten",
    chapas: "view_module",
    terminaciones: "format_paint",
    aditivos: "science",
    varios: "category",
    soldadura: "flare",
  };
  return icons[category || ""] || "folder";
};

const getCategoryLabel = (category?: string): string => {
  const labels: Record<string, string> = {
    aglomerantes: "Aglomerantes",
    aridos: "Áridos",
    hierros: "Hierros",
    bloques: "Bloques",
    aislantes: "Aislantes",
    perfiles: "Perfiles",
    chapas: "Chapas",
    terminaciones: "Terminaciones",
    aditivos: "Aditivos",
    varios: "Varios",
    soldadura: "Soldadura",
  };
  return labels[category || ""] || category || "Sin categoría";
};

const getCategoryBgClass = (category?: string): string => {
  const classes: Record<string, string> = {
    aglomerantes: "bg-blue-100 dark:bg-blue-900/20",
    aridos: "bg-amber-100 dark:bg-amber-900/20",
    hierros: "bg-slate-100 dark:bg-slate-800",
    bloques: "bg-orange-100 dark:bg-orange-900/20",
    aislantes: "bg-cyan-100 dark:bg-cyan-900/20",
    perfiles: "bg-purple-100 dark:bg-purple-900/20",
    chapas: "bg-gray-100 dark:bg-gray-800",
    terminaciones: "bg-pink-100 dark:bg-pink-900/20",
    aditivos: "bg-emerald-100 dark:bg-emerald-900/20",
    varios: "bg-indigo-100 dark:bg-indigo-900/20",
    soldadura: "bg-red-100 dark:bg-red-900/20",
  };
  return classes[category || ""] || "bg-gray-100 dark:bg-gray-800";
};

const getCategoryIconColor = (category?: string): string => {
  const colors: Record<string, string> = {
    aglomerantes: "text-blue-600 dark:text-blue-400",
    aridos: "text-amber-600 dark:text-amber-400",
    hierros: "text-slate-600 dark:text-slate-400",
    bloques: "text-orange-600 dark:text-orange-400",
    aislantes: "text-cyan-600 dark:text-cyan-400",
    perfiles: "text-purple-600 dark:text-purple-400",
    chapas: "text-gray-600 dark:text-gray-400",
    terminaciones: "text-pink-600 dark:text-pink-400",
    aditivos: "text-emerald-600 dark:text-emerald-400",
    varios: "text-indigo-600 dark:text-indigo-400",
    soldadura: "text-red-600 dark:text-red-400",
  };
  return colors[category || ""] || "text-gray-600 dark:text-gray-400";
};

const getCategoryBadgeClass = (category?: string): string => {
  const classes: Record<string, string> = {
    aglomerantes:
      "bg-blue-100 dark:bg-blue-900/30 text-blue-700 dark:text-blue-400",
    aridos:
      "bg-amber-100 dark:bg-amber-900/30 text-amber-700 dark:text-amber-400",
    hierros:
      "bg-slate-100 dark:bg-slate-800 text-slate-700 dark:text-slate-400",
    bloques:
      "bg-orange-100 dark:bg-orange-900/30 text-orange-700 dark:text-orange-400",
    aislantes:
      "bg-cyan-100 dark:bg-cyan-900/30 text-cyan-700 dark:text-cyan-400",
    perfiles:
      "bg-purple-100 dark:bg-purple-900/30 text-purple-700 dark:text-purple-400",
    chapas: "bg-gray-100 dark:bg-gray-800 text-gray-700 dark:text-gray-400",
    terminaciones:
      "bg-pink-100 dark:bg-pink-900/30 text-pink-700 dark:text-pink-400",
    aditivos:
      "bg-emerald-100 dark:bg-emerald-900/30 text-emerald-700 dark:text-emerald-400",
    varios:
      "bg-indigo-100 dark:bg-indigo-900/30 text-indigo-700 dark:text-indigo-400",
    soldadura: "bg-red-100 dark:bg-red-900/30 text-red-700 dark:text-red-400",
  };
  return (
    classes[category || ""] ||
    "bg-gray-100 dark:bg-gray-800 text-gray-700 dark:text-gray-400"
  );
};
</script>
