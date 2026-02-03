<template>
  <AppLayout title="Documentación">
    <!-- Banner principal -->
    <section
      class="bg-primary rounded-3xl p-10 text-white relative overflow-hidden shadow-xl shadow-primary/20 mb-8"
    >
      <div class="relative z-10">
        <h2 class="text-4xl font-bold mb-4">Documentación y Guía de Uso</h2>
        <p class="text-xl text-white/90 max-w-2xl">
          Aprende a maximizar el uso de GMétrica para tus cómputos métricos en
          obra.
        </p>
      </div>
      <div
        class="absolute top-0 right-0 w-64 h-64 bg-white/10 rounded-full -mr-20 -mt-20 blur-3xl"
      ></div>
    </section>
    <!-- Filosofía de Cálculo -->
    <section
      class="bg-white dark:bg-surface-dark p-8 rounded-3xl shadow-sm border border-border-light dark:border-border-dark mb-8"
    >
      <div class="flex items-center gap-3 mb-6">
        <span class="material-icons-round text-primary text-3xl"
          >lightbulb</span
        >
        <h3 class="text-2xl font-bold text-slate-800 dark:text-white">
          Filosofía de Cálculo: Práctica vs. Académica
        </h3>
      </div>
      <p
        class="text-slate-600 dark:text-slate-400 text-lg mb-8 leading-relaxed"
      >
        En GMétrica entendemos que la realidad de la obra difiere
        significativamente de los manuales teóricos. Nuestros algoritmos de
        cálculo están diseñados bajo una
        <span class="text-primary font-bold">metodología empírica</span>, basada
        en años de experiencia directa en sitios de construcción con
        profesionales.
      </p>
      <div class="grid md:grid-cols-2 gap-6">
        <div
          class="bg-slate-50 dark:bg-slate-800/50 p-6 rounded-2xl border border-slate-100 dark:border-slate-800"
        >
          <h4 class="text-xl font-bold mb-3 text-slate-700 dark:text-slate-300">
            Enfoque Académico
          </h4>
          <p class="text-slate-500 dark:text-slate-400">
            Cálculos matemáticos puros sin considerar desperdicios por
            manipulación, recortes en obra o variaciones de densidad por
            compactación manual.
          </p>
        </div>
        <div
          class="bg-primary/5 dark:bg-primary/10 p-6 rounded-2xl border border-primary/20"
        >
          <h4 class="text-xl font-bold mb-3 text-primary">
            Enfoque GMétrica (Práctico)
          </h4>
          <p class="text-slate-600 dark:text-slate-300">
            Incluimos coeficientes de aporte reales, solapes necesarios para
            hierros y márgenes de seguridad para compras de materiales
            granulados.
          </p>
        </div>
      </div>
    </section>
    <!-- Aclaración importante -->
    <div
      class="bg-amber-50 dark:bg-amber-900/20 border border-amber-100 dark:border-amber-900/30 p-6 rounded-2xl flex items-start gap-4 mb-8"
    >
      <span class="material-icons-round text-amber-500">warning</span>
      <div>
        <h4 class="font-bold text-amber-900 dark:text-amber-200">
          Aclaración Importante
        </h4>
        <p class="text-amber-800 dark:text-amber-300/80 text-sm mt-1">
          Los resultados generados son estimaciones basadas en los parámetros
          ingresados. Las cantidades finales pueden variar según métodos
          constructivos específicos, condiciones del terreno y habilidades de la
          mano de obra. Siempre recomendamos una validación final con el
          profesional a cargo.
        </p>
      </div>
    </div>
    <!-- Cómo utilizar + Módulos disponibles -->
    <div class="grid md:grid-cols-3 gap-8 mb-8">
      <!-- Cómo utilizar (2/3) -->
      <div
        class="md:col-span-2 bg-white dark:bg-surface-dark p-8 rounded-3xl shadow-sm border border-border-light dark:border-border-dark"
      >
        <div class="flex items-center gap-3 mb-8">
          <span class="material-icons-round text-primary text-2xl"
            >menu_book</span
          >
          <h3 class="text-2xl font-bold dark:text-white">
            Cómo utilizar la App
          </h3>
        </div>
        <div class="space-y-8">
          <div class="flex gap-5">
            <div
              class="flex-shrink-0 w-10 h-10 bg-primary text-white rounded-full flex items-center justify-center font-bold text-lg"
            >
              1
            </div>
            <div>
              <h5 class="font-bold text-lg dark:text-white">
                Crea un nuevo Proyecto
              </h5>
              <p class="text-slate-500 dark:text-slate-400">
                Ingresa el nombre y ubicación para organizar tus cálculos de
                manera eficiente.
              </p>
            </div>
          </div>
          <div class="flex gap-5">
            <div
              class="flex-shrink-0 w-10 h-10 bg-primary text-white rounded-full flex items-center justify-center font-bold text-lg"
            >
              2
            </div>
            <div>
              <h5 class="font-bold text-lg dark:text-white">
                Selecciona el módulo de cálculo
              </h5>
              <p class="text-slate-500 dark:text-slate-400">
                Desde vigas de fundación hasta revoques. Cada módulo tiene
                campos específicos para el tipo de tarea.
              </p>
            </div>
          </div>
          <div class="flex gap-5">
            <div
              class="flex-shrink-0 w-10 h-10 bg-primary text-white rounded-full flex items-center justify-center font-bold text-lg"
            >
              3
            </div>
            <div>
              <h5 class="font-bold text-lg dark:text-white">
                Ajusta los parámetros técnicos
              </h5>
              <p class="text-slate-500 dark:text-slate-400">
                Define dosificaciones de cemento, diámetros de hierro y
                dimensiones físicas. Los resultados se actualizan en tiempo
                real.
              </p>
            </div>
          </div>
          <div class="flex gap-5">
            <div
              class="flex-shrink-0 w-10 h-10 bg-primary text-white rounded-full flex items-center justify-center font-bold text-lg"
            >
              4
            </div>
            <div>
              <h5 class="font-bold text-lg dark:text-white">
                Exporta y Comparte
              </h5>
              <p class="text-slate-500 dark:text-slate-400">
                Genera un PDF profesional con el resumen de materiales para
                enviar a proveedores o clientes.
              </p>
            </div>
          </div>
        </div>
      </div>
      <!-- Módulos disponibles (1/3) -->
      <div
        class="bg-white dark:bg-surface-dark p-8 rounded-3xl shadow-sm border border-border-light dark:border-border-dark flex flex-col"
      >
        <h3 class="text-2xl font-bold dark:text-white mb-8">
          Módulos disponibles
        </h3>
        <div class="space-y-4 flex-1">
          <ModuleItem icon="foundation" label="Cimentación" active />
          <ModuleItem icon="layers" label="Mampostería" active />
          <ModuleItem icon="roofing" label="Techado" active />
          <ModuleItem icon="grid_view" label="Pisos" active />
          <ModuleItem icon="plumbing" label="Instalaciones" active />
        </div>
      </div>
    </div>
    <!-- Cálculos + Ayuda -->
    <div class="grid md:grid-cols-2 gap-8">
      <!-- ¿Cómo se realizaron los cálculos? -->
      <div
        class="bg-white dark:bg-surface-dark p-8 rounded-3xl shadow-sm border border-border-light dark:border-border-dark"
      >
        <div class="flex items-center gap-3 mb-6">
          <span class="material-icons-round text-primary">quiz</span>
          <h3 class="text-xl font-bold dark:text-white">
            ¿Cómo se realizaron los cálculos?
          </h3>
        </div>
        <p class="text-slate-500 dark:text-slate-400 mb-6 leading-relaxed">
          Los cálculos de GMétrica se basan en fórmulas y criterios técnicos
          comúnmente utilizados en el ámbito de la construcción, con el fin de
          ofrecer estimaciones prácticas y orientativas.
        </p>
        <BaseButton
          variant="ghost"
          full-width
          class="bg-primary/10 hover:bg-primary/20 text-primary"
        >
          Ver Documentación Técnica
        </BaseButton>
      </div>
      <!-- ¿Necesitas ayuda extra? -->
      <div
        class="bg-white dark:bg-surface-dark p-8 rounded-3xl shadow-sm border border-border-light dark:border-border-dark text-center flex flex-col items-center justify-center"
      >
        <div
          class="w-16 h-16 bg-primary/10 rounded-full flex items-center justify-center mb-6"
        >
          <span class="material-icons-round text-primary text-3xl">help</span>
        </div>
        <h3 class="text-xl font-bold dark:text-white mb-2">
          ¿Necesitas ayuda extra?
        </h3>
        <p class="text-slate-500 dark:text-slate-400 mb-8 max-w-xs">
          Nuestro equipo técnico está disponible para consultas personalizadas
          sobre cálculos complejos.
        </p>
        <BaseButton
          variant="primary"
          size="lg"
          @click="goToSupport"
          class="shadow-lg shadow-primary/20"
        >
          Contactar Soporte
        </BaseButton>
      </div>
    </div>
  </AppLayout>
</template>
<script setup lang="ts">
import { useRouter } from "vue-router";
import AppLayout from "@/components/layout/AppLayout.vue";
import BaseButton from "@/components/ui/BaseButton.vue";
import ModuleItem from "@/components/documentation/ModuleItem.vue";
const router = useRouter();
const goToSupport = () => {
  router.push("/support");
};
</script>
