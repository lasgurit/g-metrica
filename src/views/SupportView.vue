<template>
  <AppLayout title="Soporte y Ayuda">
    <!-- Banner principal -->
    <div
      class="bg-primary rounded-2xl p-8 mb-8 text-white relative overflow-hidden shadow-lg shadow-primary/20"
    >
      <div class="relative z-10">
        <h1 class="text-3xl font-bold mb-2">Soporte y Ayuda</h1>
        <p class="text-white/80">
          Estamos aquí para ayudarte a optimizar tus cálculos de obra.
        </p>
      </div>
      <div
        class="absolute top-0 right-0 w-64 h-64 bg-white/10 rounded-full -mr-20 -mt-20 blur-3xl"
      ></div>
    </div>

    <!-- FAQs y Contacto -->
    <div class="grid grid-cols-1 lg:grid-cols-2 gap-8 mb-8">
      <!-- Preguntas Frecuentes -->
      <div
        class="bg-white dark:bg-surface-dark rounded-2xl p-6 shadow-sm border border-border-light dark:border-border-dark"
      >
        <div class="flex items-center gap-3 mb-6">
          <span class="material-icons-round text-primary">contact_support</span>
          <h2 class="text-xl font-bold dark:text-white">
            Preguntas Frecuentes
          </h2>
        </div>

        <div class="space-y-3">
          <FAQItem
            question="¿Cómo se calculan los desperdicios?"
            answer="GMétrica aplica automáticamente coeficientes de desperdicio estándar de la industria (5-10% dependiendo del material). Estos pueden ser ajustados manualmente en la sección de configuración de cada proyecto."
            :open="true"
          />

          <FAQItem
            question="¿Puedo exportar mis cómputos a PDF?"
            answer="Sí, puedes exportar cualquier cálculo o el resumen total de materiales a PDF desde el botón correspondiente en cada vista."
          />

          <FAQItem
            question="¿La aplicación funciona sin conexión?"
            answer="GMétrica es una aplicación de escritorio que funciona completamente offline. Todos tus datos se guardan localmente en tu computadora."
          />

          <FAQItem
            question="¿Cómo guardo un nuevo proyecto?"
            answer="Desde la pantalla de Proyectos, haz clic en 'Crear Nuevo Proyecto', ingresa el nombre y automáticamente se guardará en tu base de datos local."
          />
        </div>
      </div>

      <!-- Contáctanos -->
      <div
        class="bg-white dark:bg-surface-dark rounded-2xl p-6 shadow-sm border border-border-light dark:border-border-dark"
      >
        <div class="flex items-center gap-3 mb-6">
          <span class="material-icons-round text-primary"
            >quick_contacts_mail</span
          >
          <h2 class="text-xl font-bold dark:text-white">Contáctanos</h2>
        </div>

        <p class="text-sm text-slate-600 dark:text-slate-400 mb-6">
          Desarrollado por
          <span class="font-semibold text-slate-800 dark:text-slate-200"
            >GurIT Solutions</span
          >. Nuestro equipo técnico está disponible para resolver cualquier duda
          o sugerencia técnica.
        </p>

        <div class="space-y-4">
          <ContactOption
            icon="chat"
            title="WhatsApp Soporte"
            subtitle="Respuesta inmediata"
            color="emerald"
            href="#"
          />

          <ContactOption
            icon="mail"
            title="Email Técnico"
            subtitle="soporte@gurit.com"
            color="blue"
            href="mailto:soporte@gurit.com"
          />
        </div>
      </div>
    </div>

    <!-- Formulario de Feedback -->
    <div
      class="bg-white dark:bg-surface-dark rounded-2xl p-8 shadow-sm border border-border-light dark:border-border-dark"
    >
      <div class="flex items-center gap-3 mb-8">
        <span class="material-icons-round text-primary">feedback</span>
        <h2 class="text-xl font-bold dark:text-white">
          Envíanos tus sugerencias
        </h2>
      </div>

      <form
        @submit.prevent="submitFeedback"
        class="grid grid-cols-1 md:grid-cols-2 gap-8"
      >
        <div class="space-y-6">
          <div>
            <label
              class="block text-sm font-medium text-slate-700 dark:text-slate-300 mb-2"
            >
              Asunto
            </label>
            <input
              v-model="feedback.subject"
              type="text"
              placeholder="Ej: Sugerencias de material"
              class="w-full px-4 py-3 rounded-xl bg-slate-50 dark:bg-input-dark border border-slate-200 dark:border-slate-700 text-slate-900 dark:text-white focus:ring-2 focus:ring-primary/20 focus:border-primary transition-all outline-none"
            />
          </div>

          <div>
            <label
              class="block text-sm font-medium text-slate-700 dark:text-slate-300 mb-2"
            >
              Prioridad
            </label>
            <div class="relative">
              <select
                v-model="feedback.priority"
                class="w-full px-4 py-3 rounded-xl bg-slate-50 dark:bg-input-dark border border-slate-200 dark:border-slate-700 text-slate-900 dark:text-white focus:ring-2 focus:ring-primary/20 focus:border-primary transition-all appearance-none outline-none"
              >
                <option value="low">Baja</option>
                <option value="medium">Media</option>
                <option value="high">Alta</option>
              </select>
              <span
                class="material-icons-round absolute right-4 top-1/2 -translate-y-1/2 text-slate-400 pointer-events-none"
              >
                expand_more
              </span>
            </div>
          </div>
        </div>

        <div>
          <label
            class="block text-sm font-medium text-slate-700 dark:text-slate-300 mb-2"
          >
            Mensaje
          </label>
          <textarea
            v-model="feedback.message"
            rows="6"
            placeholder="Contanos en qué podemos mejorar..."
            class="w-full px-4 py-3 rounded-xl bg-slate-50 dark:bg-input-dark border border-slate-200 dark:border-slate-700 text-slate-900 dark:text-white focus:ring-2 focus:ring-primary/20 focus:border-primary transition-all resize-none outline-none"
          ></textarea>
        </div>

        <div class="md:col-span-2 flex justify-end mt-2">
          <BaseButton
            type="submit"
            variant="primary"
            size="lg"
            :loading="submitting"
            class="shadow-lg shadow-primary/30 hover:-translate-y-0.5 active:translate-y-0"
          >
            Enviar Feedback
          </BaseButton>
        </div>
      </form>
    </div>

    <!-- Footer -->
    <footer class="mt-12 text-center text-xs text-slate-500 pb-8">
      <p>© 2026 GMétrica by GurIT. Todos los derechos reservados</p>
    </footer>
  </AppLayout>
</template>

<script setup lang="ts">
import { ref } from "vue";
import AppLayout from "@/components/layout/AppLayout.vue";
import BaseButton from "@/components/ui/BaseButton.vue";
import FAQItem from "@/components/support/FAQItem.vue";
import ContactOption from "@/components/support/ContactOption.vue";

const feedback = ref({
  subject: "",
  priority: "medium",
  message: "",
});

const submitting = ref(false);

const submitFeedback = async () => {
  submitting.value = true;

  try {
    // Aquí iría la lógica para enviar el feedback
    await new Promise((resolve) => setTimeout(resolve, 1000));

    // Limpiar formulario
    feedback.value = {
      subject: "",
      priority: "medium",
      message: "",
    };

    alert("¡Gracias por tu feedback!");
  } catch (error) {
    console.error("Error al enviar feedback:", error);
    alert("Hubo un error al enviar el feedback. Intenta nuevamente.");
  } finally {
    submitting.value = false;
  }
};
</script>

<style scoped>
.input-dark {
  background-color: #334155;
}
</style>
