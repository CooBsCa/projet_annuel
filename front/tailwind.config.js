/** @type {import('tailwindcss').Config} */
export default {
  content: [],
  plugins: [require('daisyui')],
  daisyui: {
    themes: [
      {
        wimbledon: {
          "primary": "#e6c9a4",
          "secondary": "#388564",
          "secondary-200": "#CDE1D8",
          "accent": "#34156f",
          "neutral": "#73ce9f",
          "base-100": "#ffffff",
          "primary-content": "#000000",
          "secondary-content": "#ffffff",
          "accent-content": "#ffffff",
        },
      },
      "dark",
      "cupcake",
    ],
  },
}

