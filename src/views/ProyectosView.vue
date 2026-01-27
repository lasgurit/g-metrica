<script setup lang="ts">
import { ref, onMounted } from 'vue';
import { useProyectos } from '../composables/useProyectos';
import { useAuth } from '../composables/useAuth';
import type { Proyecto } from '../types';

const emit = defineEmits<{
  (e: 'select-proyecto', proyecto: Proyecto): void;
  (e: 'logout'): void;
}>();

const { proyectos, loading, error, cargarProyectos, crearProyecto, eliminarProyecto, buscarProyectos } = useProyectos();
const { perfil, logout } = useAuth();

const mostrarModal = ref(false);
const busqueda = ref('');
const nuevoProyecto = ref({
  nombre: '',
  descripcion: '',
  ubicacion: ''
});

onMounted(() => {
  cargarProyectos();
});

async function handleCrearProyecto() {
  if (!nuevoProyecto.value.nombre.trim()) return;
  
  const proyecto = await crearProyecto({
    nombre: nuevoProyecto.value.nombre,
    descripcion: nuevoProyecto.value.descripcion || undefined,
    ubicacion: nuevoProyecto.value.ubicacion || undefined
  });
  
  if (proyecto) {
    mostrarModal.value = false;
    nuevoProyecto.value = { nombre: '', descripcion: '', ubicacion: '' };
  }
}

async function handleEliminar(proyecto: Proyecto) {
  if (confirm(`¿Eliminar el proyecto "${proyecto.nombre}"?`)) {
    await eliminarProyecto(proyecto.id);
  }
}

async function handleBuscar() {
  if (busqueda.value.trim()) {
    await buscarProyectos(busqueda.value);
  } else {
    await cargarProyectos();
  }
}

function handleLogout() {
  logout();
  emit('logout');
}

function seleccionarProyecto(proyecto: Proyecto) {
  emit('select-proyecto', proyecto);
}
</script>

<template>
  <div class="min-h-screen bg-beige-100">
    <!-- Header -->
    <header class="header-gradient text-white py-4 px-6 shadow-soft-lg">
      <div class="flex justify-between items-center">
        <div class="flex items-center gap-3">
          <div class="w-10 h-10 bg-white/10 backdrop-blur rounded-lg flex items-center justify-center">
            <i class="ri-ruler-2-line text-2xl"></i>
          </div>
          <h1 class="text-2xl font-bold">G-Métrica</h1>
        </div>
        <div class="flex items-center gap-4">
          <div class="flex items-center gap-2">
            <i class="ri-user-line text-sm opacity-90"></i>
            <span class="text-sm opacity-90">{{ perfil?.nombre }}</span>
          </div>
          <button 
            @click="handleLogout" 
            class="px-4 py-2 bg-white/20 hover:bg-white/30 rounded-lg text-sm font-medium transition-colors flex items-center gap-2"
          >
            <i class="ri-logout-box-line"></i>
            Salir
          </button>
        </div>
      </div>
    </header>

    <!-- Main Content -->
    <main class="max-w-7xl mx-auto px-6 py-6">
      <!-- Toolbar -->
      <div class="flex justify-between items-center mb-6 gap-4 flex-wrap">
        <!-- Search Box -->
        <div class="flex gap-2 flex-1 max-w-md">
          <div class="relative flex-1">
            <input
              v-model="busqueda"
              type="text"
              placeholder="Buscar proyectos..."
              @keyup.enter="handleBuscar"
              class="input-field pl-10"
            />
            <i class="ri-search-line absolute left-3 top-1/2 -translate-y-1/2 text-neutral-400"></i>
          </div>
          <button 
            @click="handleBuscar" 
            class="px-5 py-3 bg-sky-200 hover:bg-sky-300 text-sky-900 rounded-lg font-medium transition-colors flex items-center gap-2"
          >
            <i class="ri-search-line"></i>
            Buscar
          </button>
        </div>
        
        <!-- Nuevo Proyecto Button -->
        <button 
          @click="mostrarModal = true" 
          class="btn-primary flex items-center gap-2"
        >
          <i class="ri-add-line text-xl"></i>
          Nuevo Proyecto
        </button>
      </div>

      <!-- Loading State -->
      <div v-if="loading" class="text-center py-20 text-neutral-500">
        <i class="ri-loader-4-line text-4xl animate-spin mb-4"></i>
        <p>Cargando proyectos...</p>
      </div>
      
      <!-- Error State -->
      <div v-else-if="error" class="alert-error">
        <i class="ri-error-warning-line mr-2"></i>
        {{ error }}
      </div>
      
      <!-- Empty State -->
      <div v-else-if="proyectos.length === 0" class="text-center py-20">
        <div class="w-24 h-24 mx-auto mb-6 bg-neutral-100 rounded-full flex items-center justify-center">
          <i class="ri-folder-open-line text-5xl text-neutral-400"></i>
        </div>
        <p class="text-lg mb-2 text-neutral-700">No tenés proyectos todavía</p>
        <p class="text-neutral-500">¡Creá tu primer proyecto para empezar!</p>
      </div>
      
      <!-- Projects Grid -->
      <div v-else class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-5">
        <div
          v-for="proyecto in proyectos"
          :key="proyecto.id"
          class="card-interactive p-5"
          @click="seleccionarProyecto(proyecto)"
        >
          <div class="flex justify-between items-start mb-3">
            <div class="flex items-start gap-3 flex-1">
              <div class="w-10 h-10 bg-brand-100 rounded-lg flex items-center justify-center flex-shrink-0">
                <i class="ri-folder-line text-brand-600 text-xl"></i>
              </div>
              <div class="flex-1">
                <h3 class="text-lg font-semibold text-neutral-800">{{ proyecto.nombre }}</h3>
              </div>
            </div>
            <button 
              @click.stop="handleEliminar(proyecto)" 
              class="text-neutral-400 hover:text-red-500 text-xl transition-colors p-1"
            >
              <i class="ri-delete-bin-line"></i>
            </button>
          </div>
          
          <p v-if="proyecto.descripcion" class="text-sm text-neutral-600 mb-2 line-clamp-2">
            {{ proyecto.descripcion }}
          </p>
          
          <p v-if="proyecto.ubicacion" class="text-sm text-neutral-500 flex items-center gap-1">
            <i class="ri-map-pin-line"></i>
            {{ proyecto.ubicacion }}
          </p>
        </div>
      </div>
    </main>

    <!-- Modal Nuevo Proyecto -->
    <div 
      v-if="mostrarModal" 
      class="fixed inset-0 bg-black/50 flex items-center justify-center p-5 z-50"
      @click.self="mostrarModal = false"
    >
      <div class="bg-white rounded-2xl p-8 w-full max-w-lg shadow-soft-lg">
        <div class="flex items-center justify-between mb-6">
          <h2 class="text-2xl font-semibold text-neutral-800 flex items-center gap-2">
            <i class="ri-add-circle-line text-brand-500"></i>
            Nuevo Proyecto
          </h2>
          <button @click="mostrarModal = false" class="text-neutral-400 hover:text-neutral-600">
            <i class="ri-close-line text-2xl"></i>
          </button>
        </div>
        
        <form @submit.prevent="handleCrearProyecto">
          <div class="mb-5">
            <label for="nombre" class="label-field">
              <i class="ri-folder-line mr-1"></i>
              Nombre del proyecto *
            </label>
            <input
              id="nombre"
              v-model="nuevoProyecto.nombre"
              type="text"
              placeholder="Ej: Casa familia García"
              class="input-field"
              required
            />
          </div>
          
          <div class="mb-5">
            <label for="descripcion" class="label-field">
              <i class="ri-file-text-line mr-1"></i>
              Descripción
            </label>
            <textarea
              id="descripcion"
              v-model="nuevoProyecto.descripcion"
              placeholder="Descripción del proyecto..."
              rows="3"
              class="input-field resize-none"
            ></textarea>
          </div>
          
          <div class="mb-6">
            <label for="ubicacion" class="label-field">
              <i class="ri-map-pin-line mr-1"></i>
              Ubicación
            </label>
            <input
              id="ubicacion"
              v-model="nuevoProyecto.ubicacion"
              type="text"
              placeholder="Ej: Córdoba, Argentina"
              class="input-field"
            />
          </div>
          
          <div class="flex gap-3 justify-end">
            <button 
              type="button" 
              @click="mostrarModal = false" 
              class="btn-ghost flex items-center gap-2"
            >
              <i class="ri-close-line"></i>
              Cancelar
            </button>
            <button 
              type="submit" 
              class="btn-primary flex items-center gap-2"
              :disabled="loading"
            >
              <i v-if="loading" class="ri-loader-4-line animate-spin"></i>
              <i v-else class="ri-save-line"></i>
              {{ loading ? 'Guardando...' : 'Crear proyecto' }}
            </button>
          </div>
        </form>
      </div>
    </div>
  </div>
</template>