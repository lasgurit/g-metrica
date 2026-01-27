/** @type {import('tailwindcss').Config} */
export default {
  content: [
    "./index.html",
    "./src/**/*.{vue,js,ts,jsx,tsx}",
  ],
  theme: {
    extend: {
      colors: {
        // Paleta de colores G-Métrica
        brand: {
          // F1876F - Color principal de marca (coral/salmón)
          50: '#fef5f3',
          100: '#fde9e4',
          200: '#fcd3c9',
          300: '#f9b4a3',
          400: '#f5917d',
          500: '#F1876F',  // Color principal
          600: '#e96d4d',
          700: '#d8542f',
          800: '#b54426',
          900: '#933a22',
        },
        brown: {
          // 732416 - Para acentos puntuales (marrón terracota)
          50: '#faf6f5',
          100: '#f3ebe8',
          200: '#e4d3cd',
          300: '#d0b3a8',
          400: '#b58a7c',
          500: '#9d6b5b',
          600: '#8a5649',
          700: '#732416',  // Acento principal
          800: '#5f1e12',
          900: '#4e1910',
        },
        sky: {
          // CCE4F8 - Para soporte, datos, UI (azul cielo)
          50: '#f0f8ff',
          100: '#e0f0fe',
          200: '#CCE4F8',  // Color principal
          300: '#a6d5f5',
          400: '#7ec2f0',
          500: '#5aaeeb',
          600: '#3d8fd9',
          700: '#2e73b8',
          800: '#295f96',
          900: '#274f7b',
        },
        beige: {
          // F5F4E2 - Fondo base (beige cálido)
          50: '#fcfcfa',
          100: '#F5F4E2',  // Fondo principal
          200: '#eeecca',
          300: '#e5e1b8',
          400: '#ddd79f',
          500: '#d3cb86',
          600: '#c4b567',
          700: '#a89549',
          800: '#88763a',
          900: '#6e5f30',
        },
        // Grises neutros para complementar
        neutral: {
          50: '#fafafa',
          100: '#f5f5f5',
          200: '#e5e5e5',
          300: '#d4d4d4',
          400: '#a3a3a3',
          500: '#737373',
          600: '#525252',
          700: '#404040',
          800: '#262626',
          900: '#171717',
        }
      },
      fontFamily: {
        sans: ['Inter', 'system-ui', '-apple-system', 'sans-serif'],
      },
      boxShadow: {
        'soft': '0 2px 8px rgba(115, 36, 22, 0.08)',
        'soft-lg': '0 8px 24px rgba(115, 36, 22, 0.12)',
        'brand': '0 4px 12px rgba(241, 135, 111, 0.3)',
      }
    },
  },
  plugins: [],
}