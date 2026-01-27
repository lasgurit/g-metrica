<script setup lang="ts">
import { ref } from 'vue';
import { useAuth } from '../composables/useAuth';

const emit = defineEmits<{
  (e: 'login-success'): void;
}>();

const { login, register, loading, error } = useAuth();

const isRegistro = ref(false);
const email = ref('');
const password = ref('');
const nombre = ref('');
const empresa = ref('');
const mostrarPassword = ref(false);

async function handleSubmit() {
  if (isRegistro.value) {
    const success = await register(email.value, password.value, nombre.value, empresa.value);
    if (success) {
      emit('login-success');
    }
  } else {
    const success = await login(email.value, password.value);
    if (success) {
      emit('login-success');
    }
  }
}

function toggleMode() {
  isRegistro.value = !isRegistro.value;
  error.value = null;
}
</script>

<template>
  <div class="min-h-screen flex items-center justify-center bg-beige-100 p-5">
    <div class="bg-white rounded-2xl shadow-soft-lg overflow-hidden w-full max-w-md">
      <!-- Header -->
      <div class="bg-gradient-to-br from-brand-500 to-brown-700 text-white p-8 text-center">
        <img 
          src="../assets/logo.png" 
          alt="G-Métrica" 
          class="w-24 h-24 mx-auto mb-4 object-contain"
        />
        <h1 class="text-3xl font-bold">G-Métrica</h1>
        <p class="mt-2 text-sm opacity-90">Sistema de Cálculo de Materiales</p>
      </div>

      <!-- Form -->
      <form @submit.prevent="handleSubmit" class="p-8">
        <h2 class="text-2xl font-semibold text-brown-700 text-center mb-6">
          {{ isRegistro ? 'Crear cuenta' : 'Iniciar sesión' }}
        </h2>

        <!-- Nombre (solo registro) -->
        <div v-if="isRegistro" class="mb-5">
          <label for="nombre" class="label-field">
            <i class="ri-user-line mr-1"></i>
            Nombre
          </label>
          <div class="relative">
            <input
              id="nombre"
              v-model="nombre"
              type="text"
              placeholder="Tu nombre"
              class="input-field pl-10"
              required
            />
            <i class="ri-user-line absolute left-3 top-1/2 -translate-y-1/2 text-neutral-400"></i>
          </div>
        </div>

        <!-- Empresa (solo registro) -->
        <div v-if="isRegistro" class="mb-5">
          <label for="empresa" class="label-field">
            <i class="ri-building-line mr-1"></i>
            Empresa (opcional)
          </label>
          <div class="relative">
            <input
              id="empresa"
              v-model="empresa"
              type="text"
              placeholder="Nombre de tu empresa"
              class="input-field pl-10"
            />
            <i class="ri-building-line absolute left-3 top-1/2 -translate-y-1/2 text-neutral-400"></i>
          </div>
        </div>

        <!-- Email -->
        <div class="mb-5">
          <label for="email" class="label-field">
            <i class="ri-mail-line mr-1"></i>
            Email
          </label>
          <div class="relative">
            <input
              id="email"
              v-model="email"
              type="email"
              placeholder="tu@email.com"
              class="input-field pl-10"
              required
            />
            <i class="ri-mail-line absolute left-3 top-1/2 -translate-y-1/2 text-neutral-400"></i>
          </div>
        </div>

        <!-- Contraseña -->
        <div class="mb-5">
          <label for="password" class="label-field">
            <i class="ri-lock-line mr-1"></i>
            Contraseña
          </label>
          <div class="relative">
            <input
              id="password"
              v-model="password"
              :type="mostrarPassword ? 'text' : 'password'"
              placeholder="••••••••"
              class="input-field pr-10 password-input"
              required
              minlength="6"
            />
            <button
              type="button"
              @click="mostrarPassword = !mostrarPassword"
              class="absolute right-3 top-1/2 -translate-y-1/2 text-neutral-400 hover:text-neutral-600 transition-colors"
            >
              <i :class="mostrarPassword ? 'ri-eye-off-line' : 'ri-eye-line'"></i>
            </button>
          </div>
        </div>

        <!-- Error Message -->
        <div v-if="error" class="alert-error mb-5">
          <i class="ri-error-warning-line mr-2"></i>
          {{ error }}
        </div>

        <!-- Submit Button -->
        <button 
          type="submit" 
          class="btn-primary w-full"
          :disabled="loading"
        >
          <i v-if="loading" class="ri-loader-4-line animate-spin mr-2"></i>
          <i v-else-if="isRegistro" class="ri-user-add-line mr-2"></i>
          <i v-else class="ri-login-box-line mr-2"></i>
          {{ loading ? 'Cargando...' : (isRegistro ? 'Registrarse' : 'Ingresar') }}
        </button>

        <!-- Toggle Mode -->
        <p class="text-center mt-5 text-neutral-600 text-sm">
          {{ isRegistro ? '¿Ya tenés cuenta?' : '¿No tenés cuenta?' }}
          <a 
            href="#" 
            @click.prevent="toggleMode"
            class="text-brand-600 font-medium hover:text-brand-700 hover:underline ml-1"
          >
            {{ isRegistro ? 'Iniciar sesión' : 'Registrate' }}
          </a>
        </p>
      </form>
    </div>
  </div>
</template>

<style scoped>
/* Ocultar el botón nativo de mostrar/ocultar contraseña del navegador */
.password-input::-ms-reveal,
.password-input::-ms-clear {
  display: none;
}

.password-input::-webkit-credentials-auto-fill-button,
.password-input::-webkit-caps-lock-indicator {
  display: none;
}

/* Para Edge y otros navegadores basados en Chromium */
input[type="password"]::-webkit-textfield-decoration-container {
  display: none;
}
</style>