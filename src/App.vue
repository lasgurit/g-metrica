<template>
  <router-view />
</template>

<script setup lang="ts">
import { onMounted } from "vue";
import { useDarkMode } from "./composables/useDarkMode";
import { useAuthStore } from "./stores/auth";

// const { initDarkMode } = useDarkMode();
const { isDark, toggleDarkMode, setDarkMode } = useDarkMode();
const authStore = useAuthStore();

onMounted(async () => {
  // Inicializar dark mode
  onMounted(() => {
    setDarkMode(true);
  });

  // Verificar autenticación
  await authStore.checkAuth();
});
</script>
