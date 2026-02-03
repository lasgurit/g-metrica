<template>
  <!-- Sidebar lateral fijo -->
  <aside
    class="w-64 border-r border-border-light dark:border-border-dark bg-surface-light dark:bg-sidebar-dark flex flex-col fixed h-full z-20"
  >
    <!-- Logo / Branding -->
    <div class="p-6 flex items-center justify-center">
      <div class="flex items-center gap-3">
        <div
          class="w-10 h-10 rounded-full bg-primary flex items-center justify-center shadow-md shadow-primary/20"
        >
          <img
            src="../../assets/logo.png"
            alt="Logo"
            class="w-6 h-6 object-contain"
          />
        </div>

        <h1
          class="text-xl font-bold tracking-tight text-slate-800 dark:text-white"
        >
          G-Métrica
        </h1>
      </div>
    </div>

    <!-- Navegación principal -->
    <nav class="flex-1 px-4 py-4 space-y-2">
      <!-- Renderiza los items del menú dinámicamente -->
      <SidebarItem
        v-for="item in menuItems"
        :key="item.path"
        :icon="item.icon"
        :label="item.label"
        :path="item.path"
        :active="isActive(item.path)"
      />
    </nav>

    <!-- Footer del sidebar -->
    <div class="p-4 border-t border-border-light dark:border-border-dark">
      <!-- Botón para alternar tema claro / oscuro -->
      <button
        @click="toggleDarkMode"
        class="w-full flex items-center justify-center gap-2 px-4 py-2 rounded-lg bg-slate-100 dark:bg-slate-800 text-slate-600 dark:text-slate-300 hover:bg-slate-200 dark:hover:bg-slate-700 transition-colors"
      >
        <!-- Ícono dinámico según el tema -->
        <span class="material-icons-round text-sm">
          {{ isDark ? "light_mode" : "dark_mode" }}
        </span>

        <span class="text-sm font-medium"> Cambiar Tema </span>
      </button>

      <!-- Footer legal / marca -->
      <p
        class="text-[10px] text-center mt-4 text-slate-400 dark:text-slate-600 uppercase tracking-widest"
      >
        © 2026 GURIT GESTIÓN
      </p>
    </div>
  </aside>
</template>

<script setup lang="ts">
import { useRoute } from "vue-router";
import { useDarkMode } from "../../composables/useDarkMode";
import SidebarItem from "./SidebarItem.vue";

/**
 * Ruta actual
 * Se usa para determinar qué item del menú está activo
 */
const route = useRoute();

/**
 * Composable de modo oscuro
 *
 * isDark: indica si el tema actual es oscuro
 * toggleDarkMode: alterna entre modo claro y oscuro
 */
const { isDark, toggleDarkMode } = useDarkMode();

/**
 * Items del menú lateral
 *
 * Cada item define:
 * - icon: ícono de Material Icons
 * - label: texto visible
 * - path: ruta asociada
 */
const menuItems = [
  { icon: "folder", label: "Proyectos", path: "/projects" },
  { icon: "inventory_2", label: "Materiales", path: "/materials" },
  { icon: "description", label: "Documentación", path: "/documentation" },
  { icon: "support_agent", label: "Soporte", path: "/support" },
];

/**
 * Determina si un item del menú está activo
 *
 * Se considera activo si:
 * - la ruta coincide exactamente
 * - o la ruta actual comienza con el path del item
 *   (útil para subrutas)
 */
const isActive = (path: string) => {
  return route.path === path || route.path.startsWith(path);
};
</script>
