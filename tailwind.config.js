/** @type {import('tailwindcss').Config} */
export default {
  content: ["./index.html", "./src/**/*.{vue,js,ts,jsx,tsx}"],
  darkMode: "class",
  theme: {
    extend: {
      colors: {
        // Color principal de GMétrica (coral/naranja)
        primary: {
          DEFAULT: "#FF8A71",
          50: "#FFF4F1",
          100: "#FFE9E3",
          200: "#FFD3C7",
          300: "#FFBDAB",
          400: "#FFA38E",
          500: "#FF8A71", // Color principal
          600: "#FF6B4D",
          700: "#FF4D29",
          800: "#E6340F",
          900: "#B82800",
        },

        // Fondos
        background: {
          light: "#F9FAFB",
          dark: "#111827",
        },

        // Superficies (cards, modales, etc)
        surface: {
          light: "#FFFFFF",
          dark: "#1F2937",
        },

        // Sidebar
        sidebar: {
          light: "#FFFFFF",
          dark: "#0B1120",
        },

        // Bordes
        border: {
          light: "#E5E7EB",
          dark: "#374151",
        },
      },

      fontFamily: {
        sans: ["Inter", "system-ui", "sans-serif"],
        display: ["Inter", "system-ui", "sans-serif"],
      },

      borderRadius: {
        DEFAULT: "0.75rem", // 12px
        sm: "0.5rem", // 8px
        md: "0.75rem", // 12px
        lg: "1rem", // 16px
        xl: "1.5rem", // 24px
        "2xl": "2rem", // 32px
      },

      boxShadow: {
        primary: "0 10px 40px -10px rgba(255, 138, 113, 0.3)",
        "primary-lg": "0 20px 60px -15px rgba(255, 138, 113, 0.4)",
      },

      animation: {
        "scale-in": "scaleIn 0.2s ease-out",
        "fade-in": "fadeIn 0.3s ease-out",
      },

      keyframes: {
        scaleIn: {
          "0%": { transform: "scale(0.95)", opacity: "0" },
          "100%": { transform: "scale(1)", opacity: "1" },
        },
        fadeIn: {
          "0%": { opacity: "0" },
          "100%": { opacity: "1" },
        },
      },
    },
  },
  plugins: [require("@tailwindcss/forms"), require("@tailwindcss/typography")],
};
