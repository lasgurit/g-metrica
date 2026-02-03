<template>
  <!-- Header principal de la aplicación -->
  <header
    class="h-20 border-b border-border-light dark:border-border-dark bg-surface-light dark:bg-sidebar-dark px-8 flex items-center justify-between sticky top-0 z-20"
  >
    <!-- Sección izquierda: botón volver + título -->
    <div class="flex items-center gap-4">
      <!-- Botón para volver a la vista anterior (opcional) -->
      <button
        v-if="showBackButton"
        @click="goBack"
        class="w-8 h-8 flex items-center justify-center rounded-full hover:bg-slate-100 dark:hover:bg-slate-800 text-slate-600 dark:text-slate-300 transition-colors"
      >
        <span class="material-icons-round">arrow_back</span>
      </button>

      <!-- Título de la vista actual -->
      <h2 class="text-2xl font-bold text-slate-800 dark:text-white">
        {{ title }}
      </h2>
    </div>

    <!-- Sección derecha: información del usuario -->
    <div class="flex items-center gap-4">
      <!-- Nombre y email del usuario (oculto en mobile) -->
      <div class="text-right hidden sm:block">
        <p class="text-sm font-semibold text-slate-800 dark:text-white">
          {{ user.name }}
        </p>
        <p class="text-xs text-slate-500 dark:text-slate-400">
          {{ user.email }}
        </p>
      </div>

      <!-- Avatar del usuario -->
      <div
        class="w-10 h-10 rounded-full overflow-hidden ring-2 ring-primary/20 bg-slate-200 dark:bg-slate-700"
      >
        <!-- Imagen de avatar si existe -->
        <img
          v-if="user.avatar"
          :src="user.avatar"
          :alt="`Avatar de ${user.name}`"
          class="w-full h-full object-cover"
        />

        <!-- Iniciales como fallback si no hay avatar -->
        <div
          v-else
          class="w-full h-full flex items-center justify-center text-slate-600 dark:text-slate-300 font-bold"
        >
          {{ userInitials }}
        </div>
      </div>

      <!-- Botón de cerrar sesión -->
      <button
        @click="handleLogout"
        class="w-9 h-9 flex items-center justify-center rounded-full hover:bg-red-50 dark:hover:bg-red-900/20 text-slate-600 dark:text-slate-300 hover:text-red-600 dark:hover:text-red-400 transition-colors"
        title="Cerrar sesión"
      >
        <span class="material-icons-round text-[20px]">logout</span>
      </button>
    </div>
  </header>
</template>

<script setup lang="ts">
import { computed } from "vue";
import { useRouter } from "vue-router";
import { useAuthStore } from "../../stores/auth";

/**
 * Props del componente
 *
 * title: título que se muestra en el header
 * showBackButton: define si se muestra el botón de volver
 */
interface Props {
  title: string;
  showBackButton?: boolean;
}

/**
 * Valores por defecto de las props
 */
const props = withDefaults(defineProps<Props>(), {
  showBackButton: false,
});

/**
 * Router para navegación (volver / redirecciones)
 */
const router = useRouter();

/**
 * Store de autenticación (Pinia)
 * Maneja usuario logueado y logout
 */
const authStore = useAuthStore();

/**
 * Usuario actual
 * Si no hay usuario logueado, se usan valores de fallback
 * para evitar errores de render
 */
const user = computed(
  () =>
    authStore.currentUser || {
      name: "Usuario",
      email: "usuario@email.com",
      avatar: "",
    },
);

/**
 * Iniciales del usuario (máximo 2 letras)
 * Se usan como fallback cuando no hay avatar
 */
const userInitials = computed(() => {
  const names = user.value.name.split(" ");
  return names
    .map((n) => n[0])
    .join("")
    .toUpperCase()
    .slice(0, 2);
});

/**
 * Navega a la vista anterior
 */
const goBack = () => {
  router.back();
};

/**
 * Maneja el cierre de sesión
 * - Muestra confirmación
 * - Limpia el estado de autenticación
 * - Redirige al login
 */
const handleLogout = async () => {
  if (!confirm("¿Cerrar sesión?")) return;

  await authStore.logout();
  router.replace("/login");
};
</script>
