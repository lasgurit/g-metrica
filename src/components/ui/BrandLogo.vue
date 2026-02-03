<template>
  <div class="flex items-center gap-2" :class="containerClass">
    <div
      class="rounded-full flex items-center justify-center text-white font-bold"
      :class="[
        size === 'sm' ? 'w-8 h-8' : size === 'md' ? 'w-10 h-10' : 'w-16 h-16',
        showBackground ? 'bg-primary' : '',
      ]"
    >
      <img
        src="../../assets/logo.png"
        alt="Logo"
        class="object-contain"
        :class="
          size === 'sm' ? 'w-5 h-5' : size === 'md' ? 'w-6 h-6' : 'w-10 h-10'
        "
      />
    </div>

    <h1
      v-if="showText"
      class="font-bold tracking-tight"
      :class="[
        size === 'sm' ? 'text-lg' : size === 'md' ? 'text-xl' : 'text-3xl',
        textClass,
      ]"
    >
      {{ brandName }}
    </h1>
  </div>
</template>

<script setup lang="ts">
interface Props {
  size?: "sm" | "md" | "lg";
  showText?: boolean;
  showBackground?: boolean;
  brandName?: string;
  textClass?: string;
  containerClass?: string;
}

// Valores por defecto
withDefaults(defineProps<Props>(), {
  size: "md",
  showText: true,
  showBackground: true,
  brandName: "G-Métrica",
  textClass: "text-slate-800 dark:text-white",
  containerClass: "",
});
</script>
