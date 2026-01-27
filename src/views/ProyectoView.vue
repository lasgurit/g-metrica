<script setup lang="ts">
import { ref, onMounted, computed } from 'vue';
import { useCalculos } from '../composables/useCalculos';
import type { Proyecto, VigaInputs, VigaResultados } from '../types';

const props = defineProps<{
  proyecto: Proyecto;
}>();

const emit = defineEmits<{
  (e: 'volver'): void;
}>();

const { 
  calculos, 
  loading, 
  error, 
  cargarCalculosProyecto, 
  cargarTiposCalculo,
  tiposCalculo,
  calcularViga,
  crearCalculoCompleto
} = useCalculos();

const mostrarCalculadora = ref(false);
const calculadoraActiva = ref<string | null>(null);
const calculando = ref(false);
const guardando = ref(false);
const errorValidacion = ref<string | null>(null);

const vigaInputs = ref<VigaInputs>({
  longitud: 0,
  ancho: 0,
  alto: 0,
  diametro_hierro: 12
});

const vigaResultados = ref<VigaResultados | null>(null);
const nombreCalculo = ref('');

const totalHierro = computed(() => {
  if (!vigaResultados.value) return 0;
  return vigaResultados.value.hierro_longitudinal_barras + vigaResultados.value.hierro_estribos_barras;
});

onMounted(async () => {
  await cargarTiposCalculo();
  await cargarCalculosProyecto(props.proyecto.id);
});

function abrirCalculadora(tipo: string) {
  calculadoraActiva.value = tipo;
  mostrarCalculadora.value = true;
  vigaResultados.value = null;
  vigaInputs.value = { longitud: 0, ancho: 0, alto: 0, diametro_hierro: 12 };
  nombreCalculo.value = '';
  errorValidacion.value = null;
}

function cerrarCalculadora() {
  mostrarCalculadora.value = false;
  calculadoraActiva.value = null;
  errorValidacion.value = null;
}

function validarInputsViga(): string | null {
  const { longitud, ancho, alto } = vigaInputs.value;
  
  if (longitud <= 0 || ancho <= 0 || alto <= 0) {
    return 'Todas las dimensiones deben ser mayores a cero';
  }
  
  if (longitud > 50) {
    return 'La longitud máxima soportada es 50 metros';
  }
  
  if (ancho < 0.10 || ancho > 1.0) {
    return 'El ancho debe estar entre 0.10m y 1.0m';
  }
  
  if (alto < 0.15 || alto > 1.5) {
    return 'El alto debe estar entre 0.15m y 1.5m';
  }
  
  return null;
}

function obtenerAdvertencias(): string[] {
  const advertencias: string[] = [];
  const { longitud, ancho, alto } = vigaInputs.value;
  
  if (longitud > 30) {
    advertencias.push('⚠️ Longitud mayor a 30m - Verificar si necesita juntas de dilatación');
  }
  
  if (alto / ancho > 3) {
    advertencias.push('⚠️ Relación alto/ancho muy alta - Verificar estabilidad');
  }
  
  if (longitud / alto > 20) {
    advertencias.push('⚠️ Viga muy esbelta - Considerar aumentar el alto');
  }
  
  return advertencias;
}

async function ejecutarCalculoViga() {
  errorValidacion.value = validarInputsViga();
  if (errorValidacion.value) {
    return;
  }
  
  calculando.value = true;
  try {
    vigaResultados.value = await calcularViga(vigaInputs.value);
  } catch (e) {
    errorValidacion.value = 'Error al calcular: ' + e;
  } finally {
    calculando.value = false;
  }
}

async function guardarCalculoViga() {
  if (!vigaResultados.value) return;
  
  guardando.value = true;
  
  try {
    await crearCalculoCompleto(
      props.proyecto.id,
      nombreCalculo.value || undefined,
      vigaInputs.value
    );
    
    cerrarCalculadora();
    await cargarCalculosProyecto(props.proyecto.id);
  } catch (e) {
    errorValidacion.value = 'Error al guardar: ' + e;
  } finally {
    guardando.value = false;
  }
}

function getTipoCalculoNombre(tipoId: number): string {
  const tipo = tiposCalculo.value.find(t => t.id === tipoId);
  return tipo?.nombre || 'Cálculo';
}
</script>

<template>
  <div class="min-h-screen bg-beige-100">
    <!-- Header -->
    <header class="header-gradient text-white py-4 px-6 shadow-soft-lg">
      <div class="flex items-center gap-5">
        <button 
          @click="emit('volver')" 
          class="px-4 py-2 bg-white/20 hover:bg-white/30 rounded-lg text-sm font-medium transition-colors flex items-center gap-2"
        >
          <i class="ri-arrow-left-line"></i>
          Volver
        </button>
        <div class="flex items-center gap-3">
          <div class="w-10 h-10 bg-white/10 backdrop-blur rounded-lg flex items-center justify-center">
            <i class="ri-folder-open-line text-2xl"></i>
          </div>
          <div>
            <h1 class="text-2xl font-bold">{{ proyecto.nombre }}</h1>
            <p v-if="proyecto.ubicacion" class="text-sm opacity-90 mt-1 flex items-center gap-1">
              <i class="ri-map-pin-line text-xs"></i>
              {{ proyecto.ubicacion }}
            </p>
          </div>
        </div>
      </div>
    </header>

    <!-- Main Content -->
    <main class="max-w-7xl mx-auto px-6 py-6">
      <!-- Sección de Calculadoras -->
      <section class="mb-10">
        <h2 class="text-xl font-semibold text-neutral-800 mb-5 flex items-center gap-2">
          <i class="ri-calculator-line text-brand-500"></i>
          Calculadoras
        </h2>
        <div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-4 gap-4">
          <!-- Viga de Fundación -->
          <div 
            class="card-interactive p-6 text-center"
            @click="abrirCalculadora('viga_fundacion')"
          >
            <div class="w-16 h-16 mx-auto mb-3 bg-brand-100 rounded-xl flex items-center justify-center">
              <i class="ri-building-2-line text-3xl text-brand-600"></i>
            </div>
            <h3 class="font-semibold text-neutral-800 mb-2">Viga de Fundación</h3>
            <p class="text-sm text-neutral-600">Calcular materiales para vigas</p>
          </div>
          
          <!-- Próximamente -->
          <div class="card p-6 text-center opacity-50 cursor-not-allowed">
            <div class="w-16 h-16 mx-auto mb-3 bg-neutral-100 rounded-xl flex items-center justify-center">
              <i class="ri-layout-grid-line text-3xl text-neutral-400"></i>
            </div>
            <h3 class="font-semibold text-neutral-800 mb-2">Contrapisos</h3>
            <p class="text-sm text-neutral-600">Próximamente</p>
          </div>
          
          <div class="card p-6 text-center opacity-50 cursor-not-allowed">
            <div class="w-16 h-16 mx-auto mb-3 bg-neutral-100 rounded-xl flex items-center justify-center">
              <i class="ri-layout-masonry-line text-3xl text-neutral-400"></i>
            </div>
            <h3 class="font-semibold text-neutral-800 mb-2">Mampostería</h3>
            <p class="text-sm text-neutral-600">Próximamente</p>
          </div>
          
          <div class="card p-6 text-center opacity-50 cursor-not-allowed">
            <div class="w-16 h-16 mx-auto mb-3 bg-neutral-100 rounded-xl flex items-center justify-center">
              <i class="ri-paint-brush-line text-3xl text-neutral-400"></i>
            </div>
            <h3 class="font-semibold text-neutral-800 mb-2">Revoque Exterior</h3>
            <p class="text-sm text-neutral-600">Próximamente</p>
          </div>
        </div>
      </section>

      <!-- Cálculos Guardados -->
      <section>
        <h2 class="text-xl font-semibold text-neutral-800 mb-5 flex items-center gap-2">
          <i class="ri-file-list-3-line text-sky-700"></i>
          Cálculos Guardados
        </h2>
        
        <div v-if="loading" class="text-center py-20 text-neutral-500">
          <i class="ri-loader-4-line text-4xl animate-spin mb-4"></i>
          <p>Cargando cálculos...</p>
        </div>
        
        <div v-else-if="calculos.length === 0" class="text-center py-20">
          <div class="w-24 h-24 mx-auto mb-6 bg-neutral-100 rounded-full flex items-center justify-center">
            <i class="ri-file-list-line text-5xl text-neutral-400"></i>
          </div>
          <p class="text-lg mb-2 text-neutral-700">No hay cálculos guardados en este proyecto</p>
          <p class="text-neutral-500">Usá las calculadoras de arriba para empezar</p>
        </div>
        
        <div v-else class="grid gap-4">
          <div 
            v-for="calculo in calculos" 
            :key="calculo.id" 
            class="card p-5"
          >
            <div class="flex justify-between items-center mb-3">
              <span class="badge-brand flex items-center gap-1">
                <i class="ri-calculator-line"></i>
                {{ getTipoCalculoNombre(calculo.tipo_calculo_id) }}
              </span>
              <span class="text-sm text-neutral-500 flex items-center gap-1">
                <i class="ri-calendar-line"></i>
                {{ new Date(calculo.created_at!).toLocaleDateString() }}
              </span>
            </div>
            
            <h4 class="text-lg font-semibold text-neutral-800 mb-3">
              {{ calculo.nombre || 'Sin nombre' }}
            </h4>
            
            <div class="flex flex-wrap gap-4 text-sm">
              <div v-if="calculo.resultados.cemento_bolsas" class="flex items-center gap-2 bg-sky-50 px-3 py-2 rounded-lg">
                <i class="ri-drop-line text-sky-700"></i>
                <span class="text-neutral-600">Cemento:</span>
                <strong class="text-neutral-800">{{ calculo.resultados.cemento_bolsas.toFixed(2) }} bolsas</strong>
              </div>
              <div v-if="calculo.resultados.arena_m3" class="flex items-center gap-2 bg-sky-50 px-3 py-2 rounded-lg">
                <i class="ri-contrast-drop-line text-sky-700"></i>
                <span class="text-neutral-600">Arena:</span>
                <strong class="text-neutral-800">{{ calculo.resultados.arena_m3.toFixed(3) }} m³</strong>
              </div>
              <div v-if="calculo.resultados.piedra_m3" class="flex items-center gap-2 bg-sky-50 px-3 py-2 rounded-lg">
                <i class="ri-shining-2-line text-sky-700"></i>
                <span class="text-neutral-600">Piedra:</span>
                <strong class="text-neutral-800">{{ calculo.resultados.piedra_m3.toFixed(3) }} m³</strong>
              </div>
              <div v-if="calculo.resultados.hierro_longitudinal_barras || calculo.resultados.hierro_estribos_barras" class="flex items-center gap-2 bg-brown-50 px-3 py-2 rounded-lg">
                <i class="ri-git-commit-line text-brown-700"></i>
                <span class="text-neutral-600">Hierro:</span>
                <strong class="text-neutral-800">
                  {{ (calculo.resultados.hierro_longitudinal_barras + calculo.resultados.hierro_estribos_barras).toFixed(2) }} barras
                </strong>
              </div>
            </div>
          </div>
        </div>
      </section>
    </main>

    <!-- Modal Calculadora Viga -->
    <div 
      v-if="mostrarCalculadora && calculadoraActiva === 'viga_fundacion'" 
      class="fixed inset-0 bg-black/50 flex items-center justify-center p-5 z-50 overflow-y-auto"
      @click.self="cerrarCalculadora"
    >
      <div class="bg-white rounded-2xl w-full max-w-6xl max-h-[90vh] overflow-y-auto my-8 shadow-soft-lg">
        <!-- Modal Header -->
        <div class="flex justify-between items-center p-6 border-b border-neutral-200 sticky top-0 bg-white z-10 rounded-t-2xl">
          <h2 class="text-2xl font-semibold text-neutral-800 flex items-center gap-2">
            <div class="w-10 h-10 bg-brand-100 rounded-lg flex items-center justify-center">
              <i class="ri-building-2-line text-brand-600 text-xl"></i>
            </div>
            Viga de Fundación
          </h2>
          <button 
            @click="cerrarCalculadora" 
            class="text-neutral-400 hover:text-neutral-600 transition-colors"
          >
            <i class="ri-close-line text-3xl"></i>
          </button>
        </div>
        
        <!-- Modal Body -->
        <div class="p-6">
          <div class="grid lg:grid-cols-2 gap-8">
            <!-- Inputs -->
            <div>
              <h3 class="text-lg font-semibold text-neutral-800 mb-5 flex items-center gap-2">
                <i class="ri-input-method-line text-sky-700"></i>
                Datos de entrada
              </h3>
              
              <!-- Nombre del cálculo -->
              <div class="mb-4">
                <label for="nombre-calculo" class="label-field">
                  <i class="ri-text mr-1"></i>
                  Nombre del cálculo (opcional)
                </label>
                <input
                  id="nombre-calculo"
                  v-model="nombreCalculo"
                  type="text"
                  placeholder="Ej: Viga frente casa"
                  class="input-field"
                />
              </div>
              
              <!-- Longitud -->
              <div class="mb-4">
                <label for="longitud" class="label-field">
                  <i class="ri-ruler-line mr-1"></i>
                  Longitud (metros)
                  <span class="text-xs text-neutral-500 font-normal ml-2">Rango: 0.1 - 50m</span>
                </label>
                <input
                  id="longitud"
                  v-model.number="vigaInputs.longitud"
                  type="number"
                  step="0.01"
                  min="0.01"
                  max="50"
                  placeholder="0.00"
                  class="input-field"
                  :class="{ 'border-orange-500 focus:border-orange-500': vigaInputs.longitud > 30 }"
                />
              </div>
              
              <!-- Ancho -->
              <div class="mb-4">
                <label for="ancho" class="label-field">
                  <i class="ri-drag-move-line mr-1"></i>
                  Ancho (metros)
                  <span class="text-xs text-neutral-500 font-normal ml-2">Rango: 0.10 - 1.0m</span>
                </label>
                <input
                  id="ancho"
                  v-model.number="vigaInputs.ancho"
                  type="number"
                  step="0.01"
                  min="0.10"
                  max="1.0"
                  placeholder="0.00"
                  class="input-field"
                />
              </div>
              
              <!-- Alto -->
              <div class="mb-4">
                <label for="alto" class="label-field">
                  <i class="ri-expand-up-down-line mr-1"></i>
                  Alto (metros)
                  <span class="text-xs text-neutral-500 font-normal ml-2">Rango: 0.15 - 1.5m</span>
                </label>
                <input
                  id="alto"
                  v-model.number="vigaInputs.alto"
                  type="number"
                  step="0.01"
                  min="0.15"
                  max="1.5"
                  placeholder="0.00"
                  class="input-field"
                />
              </div>
              
              <!-- Diámetro del hierro -->
              <div class="mb-4">
                <label for="diametro" class="label-field">
                  <i class="ri-git-commit-line mr-1"></i>
                  Diámetro del hierro
                </label>
                <select 
                  id="diametro" 
                  v-model.number="vigaInputs.diametro_hierro"
                  class="input-field"
                >
                  <option :value="6">Ø 6mm</option>
                  <option :value="8">Ø 8mm</option>
                  <option :value="10">Ø 10mm</option>
                  <option :value="12">Ø 12mm</option>
                </select>
              </div>
              
              <!-- Advertencias -->
              <div v-if="obtenerAdvertencias().length > 0" class="alert-warning mb-4">
                <i class="ri-error-warning-line mr-2"></i>
                <div>
                  <div 
                    v-for="(advertencia, index) in obtenerAdvertencias()" 
                    :key="index" 
                    class="text-sm"
                  >
                    {{ advertencia }}
                  </div>
                </div>
              </div>
              
              <!-- Error de validación -->
              <div v-if="errorValidacion" class="alert-error mb-4">
                <i class="ri-close-circle-line mr-2"></i>
                {{ errorValidacion }}
              </div>
              
              <!-- Botón Calcular -->
              <button 
                @click="ejecutarCalculoViga" 
                class="btn-primary w-full flex items-center justify-center gap-2"
                :disabled="calculando"
              >
                <i v-if="calculando" class="ri-loader-4-line animate-spin text-xl"></i>
                <i v-else class="ri-calculator-line text-xl"></i>
                {{ calculando ? 'Calculando...' : 'Calcular materiales' }}
              </button>
            </div>
            
            <!-- Resultados -->
            <div>
              <h3 class="text-lg font-semibold text-neutral-800 mb-5 flex items-center gap-2">
                <i class="ri-bar-chart-box-line text-brand-500"></i>
                Resultados
              </h3>
              
              <div v-if="!vigaResultados" class="text-center py-20 bg-neutral-50 rounded-xl">
                <i class="ri-file-chart-line text-5xl text-neutral-300 mb-4"></i>
                <p class="text-neutral-500">Ingresá los datos y presioná "Calcular materiales"</p>
              </div>
              
              <div v-else class="space-y-3">
                <!-- Volumen (destacado) -->
                <div class="header-gradient text-white p-4 rounded-xl shadow-brand">
                  <span class="text-sm opacity-90 block mb-1 flex items-center gap-1">
                    <i class="ri-cube-line"></i>
                    Volumen hormigón
                  </span>
                  <span class="text-2xl font-bold">{{ vigaResultados.volumen_hormigon.toFixed(3) }} m³</span>
                </div>
                
                <!-- Cemento -->
                <div class="bg-sky-50 p-4 rounded-xl border border-sky-100">
                  <span class="text-sm text-sky-700 block mb-1 flex items-center gap-1">
                    <i class="ri-drop-line"></i>
                    Cemento
                  </span>
                  <span class="text-lg font-semibold text-neutral-800">
                    {{ vigaResultados.cemento_bolsas.toFixed(2) }} bolsas
                  </span>
                </div>
                
                <!-- Arena -->
                <div class="bg-sky-50 p-4 rounded-xl border border-sky-100">
                  <span class="text-sm text-sky-700 block mb-1 flex items-center gap-1">
                    <i class="ri-contrast-drop-line"></i>
                    Arena
                  </span>
                  <span class="text-lg font-semibold text-neutral-800">
                    {{ vigaResultados.arena_m3.toFixed(3) }} m³
                  </span>
                </div>
                
                <!-- Piedra -->
                <div class="bg-sky-50 p-4 rounded-xl border border-sky-100">
                  <span class="text-sm text-sky-700 block mb-1 flex items-center gap-1">
                    <i class="ri-shining-2-line"></i>
                    Piedra partida
                  </span>
                  <span class="text-lg font-semibold text-neutral-800">
                    {{ vigaResultados.piedra_m3.toFixed(3) }} m³
                  </span>
                </div>
                
                <!-- Hierro longitudinal -->
                <div class="bg-brown-50 p-4 rounded-xl border border-brown-100">
                  <span class="text-sm text-brown-700 block mb-1 flex items-center gap-1">
                    <i class="ri-git-commit-line"></i>
                    Hierro longitudinal Ø{{ vigaInputs.diametro_hierro }}mm
                  </span>
                  <span class="text-lg font-semibold text-neutral-800">
                    {{ vigaResultados.hierro_longitudinal_barras.toFixed(2) }} barras
                  </span>
                </div>
                
                <!-- Hierro estribos -->
                <div class="bg-brown-50 p-4 rounded-xl border border-brown-100">
                  <span class="text-sm text-brown-700 block mb-1 flex items-center gap-1">
                    <i class="ri-git-merge-line"></i>
                    Hierro estribos Ø{{ vigaInputs.diametro_hierro }}mm
                  </span>
                  <span class="text-lg font-semibold text-neutral-800">
                    {{ vigaResultados.hierro_estribos_barras.toFixed(2) }} barras
                  </span>
                </div>
                
                <!-- Total hierro (destacado) -->
                <div class="bg-brown-100 border-2 border-brown-300 p-4 rounded-xl">
                  <span class="text-sm text-brown-800 font-medium block mb-1 flex items-center gap-1">
                    <i class="ri-stack-line"></i>
                    Total hierro Ø{{ vigaInputs.diametro_hierro }}mm
                  </span>
                  <span class="text-xl font-bold text-brown-900">
                    {{ totalHierro.toFixed(2) }} barras
                  </span>
                </div>
                
                <!-- Alambre -->
                <div class="bg-neutral-50 p-4 rounded-xl border border-neutral-200">
                  <span class="text-sm text-neutral-600 block mb-1 flex items-center gap-1">
                    <i class="ri-link"></i>
                    Alambre de atar
                  </span>
                  <span class="text-lg font-semibold text-neutral-800">
                    {{ vigaResultados.alambre_kg.toFixed(2) }} kg
                  </span>
                </div>
                
                <!-- Agua -->
                <div class="bg-neutral-50 p-4 rounded-xl border border-neutral-200">
                  <span class="text-sm text-neutral-600 block mb-1 flex items-center gap-1">
                    <i class="ri-water-percent-line"></i>
                    Agua
                  </span>
                  <span class="text-lg font-semibold text-neutral-800">
                    {{ vigaResultados.agua_litros.toFixed(1) }} litros
                  </span>
                </div>
                
                <!-- Botón Guardar -->
                <button 
                  @click="guardarCalculoViga" 
                  class="w-full px-6 py-4 bg-green-600 hover:bg-green-700 text-white font-semibold rounded-xl transition-colors mt-4 flex items-center justify-center gap-2 shadow-lg"
                  :disabled="guardando"
                >
                  <i v-if="guardando" class="ri-loader-4-line animate-spin text-xl"></i>
                  <i v-else class="ri-save-line text-xl"></i>
                  {{ guardando ? 'Guardando...' : 'Guardar cálculo' }}
                </button>
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>