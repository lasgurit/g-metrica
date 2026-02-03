<template>
  <div class="min-h-screen flex bg-background-light dark:bg-background-dark">
    <!-- Sidebar -->
    <AppSidebar />

    <!-- Main Content -->
    <div class="flex-1 ml-64 flex flex-col min-h-screen">
      <!-- Header -->
      <header
        class="h-16 border-b border-border-light dark:border-border-dark bg-white/80 dark:bg-surface-dark/80 backdrop-blur-md flex items-center justify-between px-8 sticky top-0 z-10"
      >
        <div
          class="flex items-center gap-2 text-sm text-slate-500 dark:text-slate-400"
        >
          <span class="material-icons-round text-lg">grid_view</span>
          <router-link to="/projects" class="hover:underline"
            >Proyectos</router-link
          >
          <span class="material-icons-round text-xs">chevron_right</span>
          <router-link
            :to="`/projects/${projectId}/calculations`"
            class="hover:underline"
            >Cálculos</router-link
          >
          <span class="material-icons-round text-xs">chevron_right</span>
          <span class="text-slate-900 dark:text-slate-100 font-medium">{{
            calculationName
          }}</span>
        </div>

        <div class="flex items-center gap-3">
          <div class="text-right hidden sm:block">
            <p class="text-sm font-semibold dark:text-white leading-tight">
              {{ user?.name || "Usuario" }}
            </p>
            <p class="text-xs text-slate-500 dark:text-slate-400">
              {{ user?.email || "usuario@email.com" }}
            </p>
          </div>
          <img
            v-if="user?.avatar"
            :src="user.avatar"
            alt="Avatar"
            class="w-10 h-10 rounded-full border border-slate-200 dark:border-slate-700 bg-white"
          />
          <div
            v-else
            class="w-10 h-10 rounded-full border border-slate-200 dark:border-slate-700 bg-slate-200 dark:bg-slate-700 flex items-center justify-center text-slate-600 dark:text-slate-300 font-bold"
          >
            {{ userInitials }}
          </div>
        </div>
      </header>

      <div class="flex-1 flex overflow-hidden">
        <!-- Formulario de cálculo -->
        <div class="flex-1 overflow-y-auto p-8 custom-scrollbar">
          <div class="max-w-3xl">
            <h1 class="text-3xl font-bold dark:text-white mb-2">
              {{ calculationName }}
            </h1>
            <p class="text-slate-500 dark:text-slate-400 mb-8 font-medium">
              Configure las especificaciones técnicas para la estimación precisa
              de materiales
            </p>

            <div class="space-y-6">
              <!-- Sección: Dimensiones de Viga -->
              <section
                class="bg-white dark:bg-surface-dark p-6 rounded-2xl shadow-sm border border-border-light dark:border-border-dark"
              >
                <h2
                  class="text-sm font-bold tracking-widest text-slate-500 dark:text-slate-400 mb-6 uppercase"
                >
                  Dimensiones de Viga
                </h2>
                <div class="grid grid-cols-1 md:grid-cols-3 gap-6">
                  <div>
                    <label
                      class="block text-sm font-medium text-slate-600 dark:text-slate-400 mb-2"
                    >
                      Longitud (M)
                    </label>
                    <input
                      v-model.number="formData.longitud"
                      type="number"
                      step="0.01"
                      class="w-full bg-slate-50 dark:bg-slate-800 border border-slate-200 dark:border-slate-700 rounded-xl px-4 py-3 focus:ring-2 focus:ring-primary/20 focus:border-primary outline-none transition-all font-medium text-slate-900 dark:text-white"
                    />
                  </div>
                  <div>
                    <label
                      class="block text-sm font-medium text-slate-600 dark:text-slate-400 mb-2"
                    >
                      Ancho (M)
                    </label>
                    <input
                      v-model.number="formData.ancho"
                      type="number"
                      step="0.01"
                      class="w-full bg-slate-50 dark:bg-slate-800 border border-slate-200 dark:border-slate-700 rounded-xl px-4 py-3 focus:ring-2 focus:ring-primary/20 focus:border-primary outline-none transition-all font-medium text-slate-900 dark:text-white"
                    />
                  </div>
                  <div>
                    <label
                      class="block text-sm font-medium text-slate-600 dark:text-slate-400 mb-2"
                    >
                      Alto (M)
                    </label>
                    <input
                      v-model.number="formData.alto"
                      type="number"
                      step="0.01"
                      class="w-full bg-slate-50 dark:bg-slate-800 border border-slate-200 dark:border-slate-700 rounded-xl px-4 py-3 focus:ring-2 focus:ring-primary/20 focus:border-primary outline-none transition-all font-medium text-slate-900 dark:text-white"
                    />
                  </div>
                </div>
              </section>

              <!-- Sección: Refuerzo de Hierro -->
              <section
                class="bg-white dark:bg-surface-dark p-6 rounded-2xl shadow-sm border border-border-light dark:border-border-dark"
              >
                <h2
                  class="text-sm font-bold tracking-widest text-slate-500 dark:text-slate-400 mb-6 uppercase"
                >
                  Refuerzo de Hierro
                </h2>
                <div class="grid grid-cols-1 md:grid-cols-2 gap-6 mb-6">
                  <div>
                    <label
                      class="block text-sm font-medium text-slate-600 dark:text-slate-400 mb-2"
                    >
                      Diámetro Hierro Longitudinal (mm)
                    </label>
                    <select
                      v-model.number="formData.diametro_longitudinal"
                      class="w-full bg-slate-50 dark:bg-slate-800 border border-slate-200 dark:border-slate-700 rounded-xl px-4 py-3 focus:ring-2 focus:ring-primary/20 focus:border-primary outline-none appearance-none transition-all font-medium text-slate-900 dark:text-white"
                    >
                      <option :value="6">ø 6 mm</option>
                      <option :value="8">ø 8 mm</option>
                      <option :value="10">ø 10 mm</option>
                      <option :value="12">ø 12 mm</option>
                    </select>
                  </div>
                  <div>
                    <label
                      class="block text-sm font-medium text-slate-600 dark:text-slate-400 mb-2"
                    >
                      Diámetro Hierro Estribos (mm)
                    </label>
                    <select
                      v-model.number="formData.diametro_estribos"
                      class="w-full bg-slate-50 dark:bg-slate-800 border border-slate-200 dark:border-slate-700 rounded-xl px-4 py-3 focus:ring-2 focus:ring-primary/20 focus:border-primary outline-none appearance-none transition-all font-medium text-slate-900 dark:text-white"
                    >
                      <option :value="6">ø 6 mm</option>
                      <option :value="8">ø 8 mm</option>
                      <option :value="10">ø 10 mm</option>
                      <option :value="12">ø 12 mm</option>
                    </select>
                  </div>
                </div>
                <div>
                  <label
                    class="block text-sm font-medium text-slate-600 dark:text-slate-400 mb-2"
                  >
                    Separación Entre Estribos (cm)
                  </label>
                  <input
                    v-model.number="separacionEstriboCm"
                    type="number"
                    class="w-full bg-slate-50 dark:bg-slate-800 border border-slate-200 dark:border-slate-700 rounded-xl px-4 py-3 focus:ring-2 focus:ring-primary/20 focus:border-primary outline-none transition-all font-medium text-slate-900 dark:text-white"
                  />
                </div>
              </section>

              <!-- Sección: Mezcla de Hormigón -->
              <section
                class="bg-white dark:bg-surface-dark p-6 rounded-2xl shadow-sm border border-border-light dark:border-border-dark"
              >
                <h2
                  class="text-sm font-bold tracking-widest text-slate-500 dark:text-slate-400 mb-6 uppercase"
                >
                  Mezcla de Hormigón
                </h2>
                <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
                  <div>
                    <label
                      class="block text-sm font-medium text-slate-600 dark:text-slate-400 mb-2"
                    >
                      Dosificación de Cemento (kg/m³)
                    </label>
                    <select
                      v-model="formData.tipo_hormigon"
                      class="w-full bg-slate-50 dark:bg-slate-800 border border-slate-200 dark:border-slate-700 rounded-xl px-4 py-3 focus:ring-2 focus:ring-primary/20 focus:border-primary outline-none transition-all font-medium text-slate-900 dark:text-white"
                    >
                      <option value="H21">300 kg/m³ - H21</option>
                      <option value="H30">350 kg/m³ - H30</option>
                      <option value="H38">400 kg/m³ - H38</option>
                    </select>
                  </div>
                  <div>
                    <label
                      class="block text-sm font-medium text-slate-600 dark:text-slate-400 mb-2"
                    >
                      Tipo de Cemento (Bolsa)
                    </label>
                    <select
                      v-model.number="formData.tipo_bolsa_cemento"
                      class="w-full bg-slate-50 dark:bg-slate-800 border border-slate-200 dark:border-slate-700 rounded-xl px-4 py-3 focus:ring-2 focus:ring-primary/20 focus:border-primary outline-none transition-all font-medium text-slate-900 dark:text-white"
                    >
                      <option :value="25">Bolsa de 25 kg</option>
                      <option :value="50">Bolsa de 50 kg</option>
                    </select>
                  </div>
                </div>
              </section>

              <!-- Botón calcular -->
              <div class="pt-4 flex justify-center">
                <BaseButton
                  variant="primary"
                  size="lg"
                  @click="calculate"
                  :loading="calculating"
                  class="shadow-lg shadow-primary/20"
                >
                  Calcular Materiales
                </BaseButton>
              </div>
            </div>
          </div>
        </div>

        <!-- Panel lateral de resultados -->
        <div
          class="w-[420px] bg-white dark:bg-surface-dark border-l border-border-light dark:border-border-dark flex flex-col"
        >
          <!-- Header del panel -->
          <div
            class="p-6 flex items-center justify-between border-b border-border-light dark:border-border-dark"
          >
            <h2 class="text-xl font-bold dark:text-white">
              Resumen de Materiales
            </h2>
            <div class="flex items-center gap-2">
              <div class="relative inline-flex items-center cursor-pointer">
                <input
                  v-model="manualEdit"
                  type="checkbox"
                  class="sr-only peer"
                />
                <div
                  class="w-9 h-5 bg-slate-200 peer-focus:outline-none rounded-full peer dark:bg-slate-700 peer-checked:after:translate-x-full rtl:peer-checked:after:-translate-x-full peer-checked:after:border-white after:content-[''] after:absolute after:top-[2px] after:start-[2px] after:bg-white after:border-gray-300 after:border after:rounded-full after:h-4 after:w-4 after:transition-all dark:border-gray-600 peer-checked:bg-primary"
                ></div>
              </div>
              <span
                class="text-xs font-medium text-slate-500 dark:text-slate-400"
                >Manual</span
              >
            </div>
          </div>

          <!-- Contenido de resultados -->
          <div
            v-if="hasResults"
            class="flex-1 overflow-y-auto p-6 space-y-8 custom-scrollbar"
          >
            <!-- Refuerzo de Hierro -->
            <div>
              <div
                class="bg-primary px-4 py-2 rounded-lg text-white font-bold text-sm mb-4"
              >
                Refuerzo de Hierro
              </div>
              <div class="space-y-3">
                <ResultItem
                  label="Barras longitudinales (ø 10 mm)"
                  :value="results.barrasLongitudinales"
                  :editable="manualEdit"
                />
                <ResultItem
                  label="Barras de estribos (ø 6 mm)"
                  :value="results.barrasEstribos"
                  :editable="manualEdit"
                />
                <ResultItem
                  label="Alambre de Fardo (kg)"
                  :value="results.alambreFardo"
                  :editable="manualEdit"
                />
              </div>
            </div>

            <!-- Hormigón -->
            <div>
              <div
                class="bg-primary px-4 py-2 rounded-lg text-white font-bold text-sm mb-4"
              >
                Hormigón
              </div>
              <div class="space-y-3">
                <ResultItem
                  label="Volumen Total"
                  :value="`${results.volumenTotal} m³`"
                  :editable="false"
                />
                <ResultItem
                  label="Cemento"
                  :value="`${results.cemento} bolsas de 25 kg`"
                  :editable="manualEdit"
                />
                <ResultItem
                  label="Arena"
                  :value="`${results.arena} m³`"
                  :editable="manualEdit"
                />
                <ResultItem
                  label="Piedra"
                  :value="`${results.piedra} m³`"
                  :editable="manualEdit"
                />
              </div>
            </div>
          </div>

          <!-- Estado vacío -->
          <div
            v-else
            class="flex-1 flex items-center justify-center p-6 text-center"
          >
            <div>
              <div
                class="w-20 h-20 bg-slate-100 dark:bg-slate-800 rounded-full flex items-center justify-center mx-auto mb-4"
              >
                <span
                  class="material-icons-round text-4xl text-slate-400 dark:text-slate-500"
                >
                  calculate
                </span>
              </div>
              <p class="text-slate-500 dark:text-slate-400 font-medium">
                Complete el formulario y presione
              </p>
              <p class="text-slate-500 dark:text-slate-400 font-medium">
                "Calcular Materiales"
              </p>
            </div>
          </div>

          <!-- Footer con acciones -->
          <div
            class="p-6 grid grid-cols-3 gap-3 border-t border-border-light dark:border-border-dark bg-white/50 dark:bg-surface-dark/50 backdrop-blur-sm"
          >
            <BaseButton variant="ghost" size="sm" @click="goBack">
              Volver
            </BaseButton>
            <BaseButton
              variant="primary"
              size="sm"
              @click="save"
              :disabled="!hasResults"
              :loading="saving"
            >
              Guardar
            </BaseButton>
            <BaseButton
              variant="ghost"
              size="sm"
              class="text-primary"
              @click="exportPDF"
              :disabled="!hasResults"
            >
              PDF
            </BaseButton>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, reactive, watch } from "vue";
import { useRoute, useRouter } from "vue-router";
import { useAuthStore } from "@/stores/auth";
import { useCalculoStore, type VigaInputs } from "@/stores/calculos";
import AppSidebar from "@/components/layout/AppSideBar.vue";
import BaseButton from "@/components/ui/BaseButton.vue";
import ResultItem from "@/components/calculations/ResultItem.vue";

const route = useRoute();
const router = useRouter();
const authStore = useAuthStore();
const calculoStore = useCalculoStore();

const projectId = computed(() => route.params.id as string);
const calculationId = computed(() => route.params.calculationId as string);
const user = computed(() => authStore.currentUser);

const userInitials = computed(() => {
  if (!user.value) return "U";
  const names = user.value.name.split(" ");
  return names
    .map((n) => n[0])
    .join("")
    .toUpperCase()
    .slice(0, 2);
});

const calculationName = ref("Calculadora Viga de Fundación");
const calculating = ref(false);
const saving = ref(false);
const manualEdit = ref(true);
const hasResults = ref(false);

// Campo auxiliar para separación en cm
const separacionEstriboCm = ref(20);

// Datos del formulario
const formData = reactive<VigaInputs>({
  longitud: 20,
  ancho: 0.15,
  alto: 0.25,
  diametro_longitudinal: 10,
  diametro_estribos: 6,
  separacion_estribos: 0.2,
  tipo_hormigon: "H21",
  tipo_bolsa_cemento: 25,
});

// Watch para sincronizar separación cm -> metros
watch(separacionEstriboCm, (newVal) => {
  formData.separacion_estribos = newVal / 100;
});

// Resultados del cálculo - FORMATO PARA DISPLAY
const results = reactive({
  barrasLongitudinales: "6.67",
  barrasEstribos: "8.33",
  alambreFardo: "0.96",
  volumenTotal: "0.750",
  cemento: "9",
  arena: "0.375",
  piedra: "0.375",
});

// CALCULAR
const calculate = async () => {
  calculating.value = true;
  hasResults.value = false;

  try {
    // Llamar al backend
    const resultados = await calculoStore.calcularVigaFundacion(formData);

    // Mapear resultados al formato de display
    results.barrasLongitudinales =
      resultados.hierro_longitudinal_barras.toFixed(2);
    results.barrasEstribos = resultados.hierro_estribos_barras.toFixed(2);
    results.alambreFardo = resultados.alambre_kg.toFixed(2);
    results.volumenTotal = resultados.volumen_hormigon.toFixed(3);
    results.cemento = resultados.cemento_bolsas.toFixed(0);
    results.arena = resultados.arena_m3.toFixed(3);
    results.piedra = resultados.piedra_m3.toFixed(3);

    hasResults.value = true;
  } catch (error: any) {
    console.error("Error al calcular:", error);
    alert("Error al realizar el cálculo. Intente nuevamente.");
  } finally {
    calculating.value = false;
  }
};

// GUARDAR
const save = async () => {
  if (!hasResults.value) return;

  saving.value = true;

  try {
    await calculoStore.crearCalculoCompleto(
      projectId.value,
      `Viga de Fundación - ${new Date().toLocaleDateString()}`,
      formData,
    );

    alert("Cálculo guardado exitosamente");
    router.push(`/projects/${projectId.value}/calculations`);
  } catch (error: any) {
    console.error("Error al guardar:", error);
    alert("Error al guardar el cálculo. Intente nuevamente.");
  } finally {
    saving.value = false;
  }
};

const exportPDF = () => {
  console.log("Exportando a PDF...");
};

const goBack = () => {
  router.push(`/projects/${projectId.value}/calculations`);
};
</script>

<style scoped>
.custom-scrollbar::-webkit-scrollbar {
  width: 6px;
}

.custom-scrollbar::-webkit-scrollbar-track {
  background: transparent;
}

.custom-scrollbar::-webkit-scrollbar-thumb {
  background: #475569;
  border-radius: 10px;
}
</style>
