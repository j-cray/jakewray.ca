/** @type {import('tailwindcss').Config} */
module.exports = {
  content: {
    files: ["*.html", "./frontend/src/**/*.rs"],
  },
  theme: {
    extend: {
      colors: {
        // Define brand colors if needed to match main.scss
        // --brand-color: #00fff2;
        brand: {
          DEFAULT: '#00fff2',
          dim: '#00cccc',
        },
        dark: {
          bg: '#0a0a0a',
          surface: '#1a1a1a',
        }
      },
      fontFamily: {
        heading: ['Outfit', 'sans-serif'],
        body: ['Inter', 'sans-serif'],
      }
    },
  },
  plugins: [],
}
