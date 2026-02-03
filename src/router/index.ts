import { createRouter, createWebHistory } from "vue-router";
import { useAuthStore } from "@/stores/auth";

/**
 * Configuración principal del router
 * - Usa history mode (sin hash)
 * - Define todas las rutas de la aplicación
 */
const router = createRouter({
  history: createWebHistory(),
  routes: [
    /**
     * Ruta raíz
     * Redirige automáticamente al login
     */
    {
      path: "/",
      redirect: "/login",
    },

    /**
     * Login
     * Ruta pública
     */
    {
      path: "/login",
      name: "login",
      component: () => import("@/views/LoginView.vue"),
      meta: {
        title: "Iniciar Sesión",
        requiresAuth: false,
      },
    },

    /**
     * Gestión de proyectos
     */
    {
      path: "/projects",
      name: "projects",
      component: () => import("@/views/ProjectsView.vue"),
      meta: {
        title: "Gestión de Proyectos",
        requiresAuth: true,
      },
    },

    /**
     * Lista de cálculos de un proyecto
     */
    {
      path: "/projects/:id/calculations",
      name: "calculations",
      component: () => import("@/views/CalculationsView.vue"),
      meta: {
        title: "Gestión de Cálculos",
        requiresAuth: true,
      },
    },

    /**
     * Calculadora específica
     */
    {
      path: "/projects/:id/calculations/:calculationId",
      name: "calculator",
      component: () => import("@/views/CalculatorView.vue"),
      meta: {
        title: "Calculadora",
        requiresAuth: true,
      },
    },

    /**
     * Resumen total de materiales del proyecto
     */
    {
      path: "/projects/:id/summary",
      name: "summary",
      component: () => import("@/views/SummaryView.vue"),
      meta: {
        title: "Resumen Total",
        requiresAuth: true,
      },
    },

    /**
     * Gestión de materiales
     */
    {
      path: "/materials",
      name: "materials",
      component: () => import("@/views/MaterialsView.vue"),
      meta: {
        title: "Gestión de Materiales",
        requiresAuth: true,
      },
    },

    /**
     * Documentación de la app
     */
    {
      path: "/documentation",
      name: "documentation",
      component: () => import("@/views/DocumentationView.vue"),
      meta: {
        title: "Documentación",
        requiresAuth: true,
      },
    },

    /**
     * Soporte y ayuda
     */
    {
      path: "/support",
      name: "support",
      component: () => import("@/views/SupportView.vue"),
      meta: {
        title: "Soporte",
        requiresAuth: true,
      },
    },
  ],
});

/**
 * Navigation Guard global
 * Se ejecuta antes de cada cambio de ruta
 * Maneja:
 * - Protección de rutas privadas
 * - Redirección automática del login
 */
router.beforeEach((to, from, next) => {
  const authStore = useAuthStore();
  const requiresAuth = to.meta.requiresAuth;

  if (requiresAuth && !authStore.isAuthenticated) {
    // Ruta protegida y usuario no autenticado → login
    next({ name: "login" });
  } else if (to.name === "login" && authStore.isAuthenticated) {
    // Usuario autenticado intentando acceder al login → projects
    next({ name: "projects" });
  } else {
    // Navegación permitida
    next();
  }
});

/**
 * Export del router para usar en main.ts
 */
export default router;
