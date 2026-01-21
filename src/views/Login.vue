<template>
  <div class="login-container">
    <div class="login-card">
      <h1>G-Métrica</h1>
      <p class="subtitle">Sistema de Cálculo de Materiales</p>

      <!-- Formulario de Login -->
      <form v-if="!showSignup" @submit.prevent="handleLogin">
        <div class="form-group">
          <label for="email">Email</label>
          <input
            id="email"
            v-model="email"
            type="email"
            placeholder="tu@email.com"
            required
          />
        </div>

        <div class="form-group">
          <label for="password">Contraseña</label>
          <input
            id="password"
            v-model="password"
            type="password"
            placeholder="••••••••"
            required
          />
        </div>

        <button type="submit" class="btn-primary" :disabled="loading">
          {{ loading ? 'Iniciando sesión...' : 'Iniciar Sesión' }}
        </button>

        <p class="toggle-form">
          ¿No tienes cuenta?
          <a href="#" @click.prevent="showSignup = true">Regístrate</a>
        </p>
      </form>

      <!-- Formulario de Registro -->
      <form v-else @submit.prevent="handleSignup">
        <div class="form-group">
          <label for="signup-nombre">Nombre</label>
          <input
            id="signup-nombre"
            v-model="signupForm.nombre"
            type="text"
            placeholder="Tu nombre"
            required
          />
        </div>

        <div class="form-group">
          <label for="signup-empresa">Empresa (opcional)</label>
          <input
            id="signup-empresa"
            v-model="signupForm.empresa"
            type="text"
            placeholder="Nombre de tu empresa"
          />
        </div>

        <div class="form-group">
          <label for="signup-email">Email</label>
          <input
            id="signup-email"
            v-model="signupForm.email"
            type="email"
            placeholder="tu@email.com"
            required
          />
        </div>

        <div class="form-group">
          <label for="signup-password">Contraseña</label>
          <input
            id="signup-password"
            v-model="signupForm.password"
            type="password"
            placeholder="••••••••"
            required
            minlength="6"
          />
        </div>

        <button type="submit" class="btn-primary" :disabled="loading">
          {{ loading ? 'Registrando...' : 'Registrarse' }}
        </button>

        <p class="toggle-form">
          ¿Ya tienes cuenta?
          <a href="#" @click.prevent="showSignup = false">Inicia sesión</a>
        </p>
      </form>

      <!-- Mensaje de error -->
      <div v-if="error" class="error-message">
        {{ error }}
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue';
import { useRouter } from 'vue-router';
import { useSupabase } from '@/composables/useSupabase';

const router = useRouter();
const supabase = useSupabase();

// Estado
const showSignup = ref(false);
const loading = ref(false);
const error = ref('');

// Login form
const email = ref('');
const password = ref('');

// Signup form
const signupForm = ref({
  nombre: '',
  empresa: '',
  email: '',
  password: '',
});

// Handlers
const handleLogin = async () => {
  try {
    loading.value = true;
    error.value = '';

    await supabase.login(email.value, password.value);

    // Redirigir al dashboard
    router.push('/dashboard');
  } catch (err: any) {
    error.value = err.message || 'Error al iniciar sesión';
  } finally {
    loading.value = false;
  }
};

const handleSignup = async () => {
  try {
    loading.value = true;
    error.value = '';

    await supabase.signup(
      signupForm.value.email,
      signupForm.value.password,
      signupForm.value.nombre,
      signupForm.value.empresa || undefined
    );

    // Redirigir al dashboard
    router.push('/dashboard');
  } catch (err: any) {
    error.value = err.message || 'Error al registrarse';
  } finally {
    loading.value = false;
  }
};
</script>

<style scoped>
.login-container {
  display: flex;
  justify-content: center;
  align-items: center;
  min-height: 100vh;
  background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
  padding: 20px;
}

.login-card {
  background: white;
  border-radius: 12px;
  padding: 40px;
  width: 100%;
  max-width: 400px;
  box-shadow: 0 10px 40px rgba(0, 0, 0, 0.1);
}

h1 {
  text-align: center;
  color: #333;
  margin-bottom: 8px;
  font-size: 32px;
}

.subtitle {
  text-align: center;
  color: #666;
  margin-bottom: 32px;
  font-size: 14px;
}

.form-group {
  margin-bottom: 20px;
}

label {
  display: block;
  margin-bottom: 8px;
  color: #333;
  font-weight: 500;
  font-size: 14px;
}

input {
  width: 100%;
  padding: 12px;
  border: 1px solid #ddd;
  border-radius: 6px;
  font-size: 14px;
  transition: border-color 0.3s;
}

input:focus {
  outline: none;
  border-color: #667eea;
}

.btn-primary {
  width: 100%;
  padding: 12px;
  background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
  color: white;
  border: none;
  border-radius: 6px;
  font-size: 16px;
  font-weight: 600;
  cursor: pointer;
  transition: opacity 0.3s;
}

.btn-primary:hover:not(:disabled) {
  opacity: 0.9;
}

.btn-primary:disabled {
  opacity: 0.6;
  cursor: not-allowed;
}

.toggle-form {
  text-align: center;
  margin-top: 20px;
  color: #666;
  font-size: 14px;
}

.toggle-form a {
  color: #667eea;
  text-decoration: none;
  font-weight: 600;
}

.toggle-form a:hover {
  text-decoration: underline;
}

.error-message {
  margin-top: 16px;
  padding: 12px;
  background: #fee;
  border: 1px solid #fcc;
  border-radius: 6px;
  color: #c33;
  font-size: 14px;
  text-align: center;
}
</style>