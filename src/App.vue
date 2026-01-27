<script setup lang="ts">
import { ref, onMounted } from 'vue';
import { useAuth } from './composables/useAuth';
import LoginView from './views/LoginView.vue';
import ProyectosView from './views/ProyectosView.vue';
import ProyectoView from './views/ProyectoView.vue';
import type { Proyecto } from './types';

const { isAuthenticated, restoreSession } = useAuth();

const vistaActual = ref<'login' | 'proyectos' | 'proyecto'>('login');
const proyectoSeleccionado = ref<Proyecto | null>(null);

onMounted(() => {
  restoreSession();
  if (isAuthenticated.value) {
    vistaActual.value = 'proyectos';
  }
});

function onLoginSuccess() {
  vistaActual.value = 'proyectos';
}

function onLogout() {
  vistaActual.value = 'login';
  proyectoSeleccionado.value = null;
}

function onSelectProyecto(proyecto: Proyecto) {
  proyectoSeleccionado.value = proyecto;
  vistaActual.value = 'proyecto';
}

function onVolverAProyectos() {
  vistaActual.value = 'proyectos';
  proyectoSeleccionado.value = null;
}
</script>

<template>
  <LoginView 
    v-if="vistaActual === 'login'" 
    @login-success="onLoginSuccess" 
  />
  
  <ProyectosView 
    v-else-if="vistaActual === 'proyectos'" 
    @select-proyecto="onSelectProyecto"
    @logout="onLogout"
  />
  
  <ProyectoView 
    v-else-if="vistaActual === 'proyecto' && proyectoSeleccionado"
    :proyecto="proyectoSeleccionado"
    @volver="onVolverAProyectos"
  />
</template>