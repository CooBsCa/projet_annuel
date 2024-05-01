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
          "accent": "#34156f",
          "neutral": "#73ce9f",
          "base-100": "#ffffff",
        },
      },
      "dark",
      "cupcake",
    ],
  },
}

