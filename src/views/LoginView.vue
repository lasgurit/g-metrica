<template>
  <div
    class="min-h-screen flex flex-col bg-background-light dark:bg-background-dark blueprint-bg transition-colors duration-300"
  >
    <!-- ===== HEADER SUPERIOR ===== -->
    <header
      class="fixed top-0 inset-x-0 z-50 h-16 flex items-center justify-between px-8 bg-white/80 dark:bg-background-dark/80 backdrop-blur-md border-b border-gray-200 dark:border-gray-800"
    >
      <BrandLogo size="md" :show-background="true" />

      <button
        @click="toggleDarkMode"
        class="flex items-center gap-2 px-3 py-1.5 rounded-full bg-white dark:bg-card-dark border border-gray-200 dark:border-gray-800 text-gray-600 dark:text-gray-400 text-xs font-semibold hover:bg-gray-50 dark:hover:bg-gray-800 transition-colors"
      >
        <span class="material-icons-round text-sm">
          {{ isDark ? "light_mode" : "dark_mode" }}
        </span>
        Modo
      </button>
    </header>

    <!-- ===== MAIN ===== -->
    <main class="flex-1 flex items-center justify-center px-4 pt-24 pb-10">
      <div class="w-full max-w-[500px]">
        <!-- Card -->
        <div
          class="bg-white dark:bg-surface-dark border border-border-light dark:border-border-dark shadow-xl rounded-2xl overflow-hidden"
        >
          <div class="p-8 md:p-12">
            <!-- Header card -->
            <div class="text-center mb-10">
              <div class="flex justify-center mb-4">
                <div
                  class="w-20 h-20 rounded-full bg-primary flex items-center justify-center shadow-lg shadow-primary/20"
                >
                  <img
                    src="../assets/logo.png"
                    alt="G-Métrica Logo"
                    class="w-12 h-12 object-contain"
                  />
                </div>
              </div>
              <p
                class="text-sm font-medium text-gray-500 dark:text-gray-400 uppercase tracking-widest"
              >
                Programa de Cálculos de Materiales
              </p>
            </div>

            <!-- Form -->
            <form @submit.prevent="handleLogin" class="space-y-6">
              <!-- Email -->
              <BaseInput
                v-model="loginForm.email"
                type="email"
                label="Usuario / Email"
                placeholder="usuario@gmail.com"
                icon="person"
                :error="formErrors.email"
                required
              />

              <!-- Password -->
              <div class="space-y-2">
                <label
                  class="block text-sm font-semibold text-slate-700 dark:text-slate-300 mb-2"
                >
                  Contraseña
                  <span class="text-red-500">*</span>
                </label>
                <div class="relative">
                  <span
                    class="material-icons-round absolute left-4 top-1/2 -translate-y-1/2 text-slate-400 dark:text-slate-500"
                  >
                    lock
                  </span>
                  <input
                    v-model="loginForm.password"
                    :type="showPassword ? 'text' : 'password'"
                    placeholder="••••••••"
                    required
                    class="w-full pl-12 pr-12 py-3 rounded-xl border border-border-light dark:border-border-dark bg-white dark:bg-surface-dark text-slate-900 dark:text-white placeholder:text-slate-400 dark:placeholder:text-slate-500 focus:ring-2 focus:ring-primary focus:border-transparent outline-none transition-all password-input"
                    :class="
                      formErrors.password
                        ? 'border-red-500 focus:ring-red-500'
                        : ''
                    "
                  />
                  <button
                    type="button"
                    @click="showPassword = !showPassword"
                    class="absolute right-4 top-1/2 -translate-y-1/2 text-slate-400 dark:text-slate-500 hover:text-primary transition-colors"
                  >
                    <span class="material-icons-round">
                      {{ showPassword ? "visibility_off" : "visibility" }}
                    </span>
                  </button>
                </div>
                <p v-if="formErrors.password" class="mt-1 text-sm text-red-500">
                  {{ formErrors.password }}
                </p>
              </div>

              <!-- Error general -->
              <div
                v-if="loginError"
                class="bg-red-50 dark:bg-red-900/20 border border-red-200 dark:border-red-800/50 rounded-xl p-4 flex items-start gap-3"
              >
                <span class="material-icons-round text-red-500 text-sm"
                  >error</span
                >
                <p class="text-sm text-red-800 dark:text-red-400">
                  {{ loginError }}
                </p>
              </div>

              <!-- Submit -->
              <div class="pt-4">
                <BaseButton
                  type="submit"
                  variant="primary"
                  size="lg"
                  full-width
                  :loading="loading"
                  class="shadow-lg shadow-primary/20"
                >
                  Iniciar Sesión
                </BaseButton>
              </div>
            </form>

            <div class="mt-8 text-center">
              <p class="text-sm font-medium text-gray-600 dark:text-gray-400">
                ¿Necesitás una cuenta?
                <router-link
                  to="/support"
                  class="text-primary hover:underline ml-1 font-semibold"
                >
                  Contactar a soporte
                </router-link>
              </p>
            </div>
          </div>

          <!-- Footer card -->
          <div
            class="border-t border-gray-100 dark:border-gray-800 bg-gray-50/50 dark:bg-black/20 py-6 text-center"
          >
            <p
              class="text-xs font-bold text-gray-400 dark:text-gray-500 uppercase tracking-widest"
            >
              © 2026 GURIT GESTIÓN
            </p>
          </div>
        </div>
      </div>
    </main>
  </div>
</template>

<script setup lang="ts">
import { ref } from "vue";
import { useRouter } from "vue-router";
import { useDarkMode } from "@/composables/useDarkMode";
import { useAuthStore } from "@/stores/auth";
import BrandLogo from "@/components/ui/BrandLogo.vue";
import BaseInput from "@/components/ui/BaseInput.vue";
import BaseButton from "@/components/ui/BaseButton.vue";

const router = useRouter();
const authStore = useAuthStore();
const { isDark, toggleDarkMode } = useDarkMode();

// Estado del formulario
const loginForm = ref({
  email: "",
  password: "",
});

const formErrors = ref({
  email: "",
  password: "",
});

const showPassword = ref(false);
const loading = ref(false);
const loginError = ref("");

// Validación del formulario
const validateForm = (): boolean => {
  let isValid = true;

  // Validar email
  if (!loginForm.value.email.trim()) {
    formErrors.value.email = "El email es requerido";
    isValid = false;
  } else if (!/^[^\s@]+@[^\s@]+\.[^\s@]+$/.test(loginForm.value.email)) {
    formErrors.value.email = "Email inválido";
    isValid = false;
  } else {
    formErrors.value.email = "";
  }

  // Validar password
  if (!loginForm.value.password.trim()) {
    formErrors.value.password = "La contraseña es requerida";
    isValid = false;
  } else if (loginForm.value.password.length < 4) {
    formErrors.value.password =
      "La contraseña debe tener al menos 6 caracteres";
    isValid = false;
  } else {
    formErrors.value.password = "";
  }

  return isValid;
};

// Manejar el login
const handleLogin = async () => {
  // Limpiar error previo
  loginError.value = "";

  // Validar formulario
  if (!validateForm()) {
    return;
  }

  loading.value = true;

  try {
    // Llamar al store de auth para hacer login
    await authStore.login(loginForm.value.email, loginForm.value.password);

    // Redirigir a la página de proyectos
    router.push("/projects");
  } catch (error: any) {
    loginError.value =
      error.message || "Error al iniciar sesión. Verifique sus credenciales.";
    console.error("Error de login:", error);
  } finally {
    loading.value = false;
  }
};
</script>

<style scoped>
/* Blueprint background */
.blueprint-bg {
  background-color: #f3f4f6;
  background-image:
    linear-gradient(#e5e7eb 1px, transparent 1px),
    linear-gradient(90deg, #e5e7eb 1px, transparent 1px);
  background-size: 40px 40px;
}

.dark .blueprint-bg {
  background-color: #0f1115;
  background-image:
    linear-gradient(#1f2937 1px, transparent 1px),
    linear-gradient(90deg, #1f2937 1px, transparent 1px);
}

/* Eliminar el ojo nativo del navegador en campos de contraseña */
.password-input::-ms-reveal,
.password-input::-ms-clear {
  display: none;
}

.password-input::-webkit-contacts-auto-fill-button,
.password-input::-webkit-credentials-auto-fill-button {
  visibility: hidden;
  position: absolute;
  right: 0;
}
</style>
