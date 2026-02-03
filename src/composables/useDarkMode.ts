import { ref, watch, onMounted } from "vue";

/**
 * Estado global del modo oscuro
 * Se mantiene fuera de la función para que sea compartido
 * entre todos los componentes que usen el composable
 */
const isDark = ref(false);

/**
 * Composable para manejar el modo oscuro
 * - Alterna tema claro / oscuro
 * - Persiste preferencia en localStorage
 * - Respeta preferencia del sistema
 */
export function useDarkMode() {
  /**
   * Alterna el estado del modo oscuro
   */
  const toggleDarkMode = () => {
    isDark.value = !isDark.value;
    updateDarkMode();
  };

  /**
   * Setea explícitamente el modo oscuro
   * @param value true = dark, false = light
   */
  const setDarkMode = (value: boolean) => {
    isDark.value = value;
    updateDarkMode();
  };

  /**
   * Aplica el tema al documento
   * - Agrega o quita la clase `dark` en <html>
   * - Guarda la preferencia en localStorage
   */
  const updateDarkMode = () => {
    if (isDark.value) {
      document.documentElement.classList.add("dark");
      localStorage.setItem("theme", "dark");
    } else {
      document.documentElement.classList.remove("dark");
      localStorage.setItem("theme", "light");
    }
  };

  /**
   * Inicializa el modo oscuro al cargar la app
   * - Prioriza la preferencia guardada
   * - Si no existe, usa la preferencia del sistema
   */
  const initDarkMode = () => {
    // Verificar preferencia guardada en localStorage
    const savedTheme = localStorage.getItem("theme");

    if (savedTheme) {
      isDark.value = savedTheme === "dark";
    } else {
      // Fallback: usar preferencia del sistema operativo
      isDark.value = window.matchMedia("(prefers-color-scheme: dark)").matches;
    }

    updateDarkMode();
  };

  /**
   * Inicializar el tema cuando el componente se monta
   */
  onMounted(() => {
    initDarkMode();
  });

  /**
   * API pública del composable
   */
  return {
    isDark,
    toggleDarkMode,
    setDarkMode,
  };
}
