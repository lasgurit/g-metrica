<template>
  <Teleport to="body">
    <Transition name="modal">
      <div
        v-if="modelValue"
        class="fixed inset-0 z-50 flex items-center justify-center p-4 bg-slate-900/60 dark:bg-black/80 backdrop-blur-sm"
        @click.self="closeModal"
      >
        <!-- Contenedor -->
        <div
          class="w-full rounded-2xl shadow-2xl bg-white dark:bg-surface-dark border border-border-light dark:border-border-dark flex flex-col max-h-[90vh] overflow-hidden transform transition-all animate-scale-in"
          :class="sizeClasses"
        >
          <!-- Header -->
          <div
            v-if="$slots.header || title"
            class="relative p-6 text-center flex-shrink-0"
          >
            <button
              v-if="closable"
              @click="closeModal"
              class="absolute top-6 right-6 text-slate-400 hover:text-slate-600 dark:hover:text-slate-200 transition-colors"
            >
              <span class="material-icons-round">close</span>
            </button>

            <div
              v-if="icon"
              class="inline-flex items-center justify-center w-16 h-16 rounded-full bg-primary/10 text-primary mb-4"
            >
              <span class="material-icons-round text-4xl">{{ icon }}</span>
            </div>

            <slot name="header">
              <h3 class="text-3xl font-bold text-primary mb-2">
                {{ title }}
              </h3>
              <p
                v-if="description"
                class="text-slate-500 dark:text-slate-400 text-sm"
              >
                {{ description }}
              </p>
            </slot>
          </div>

          <!-- Body (scroll interno) -->
          <div class="p-8 overflow-y-auto flex-1">
            <slot />
          </div>

          <!-- Footer -->
          <div v-if="$slots.footer" class="px-8 pb-8 pt-4 flex-shrink-0">
            <slot name="footer" />
          </div>
        </div>
      </div>
    </Transition>
  </Teleport>
</template>

<script setup lang="ts">
import { computed, watch } from "vue";

interface Props {
  modelValue: boolean;
  title?: string;
  description?: string;
  icon?: string;
  size?: "sm" | "md" | "lg" | "xl";
  closable?: boolean;
}

const props = withDefaults(defineProps<Props>(), {
  size: "md",
  closable: true,
});

const emit = defineEmits<{
  "update:modelValue": [value: boolean];
}>();

const sizeClasses = computed(
  () =>
    ({
      sm: "max-w-md",
      md: "max-w-lg",
      lg: "max-w-2xl",
      xl: "max-w-4xl",
    })[props.size],
);

const closeModal = () => {
  if (props.closable) {
    emit("update:modelValue", false);
  }
};

watch(
  () => props.modelValue,
  (open) => {
    document.body.style.overflow = open ? "hidden" : "";
  },
);
</script>
