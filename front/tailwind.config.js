/** @type {import('tailwindcss').Config} */
export default {
  content: [
    "./node_modules/flowbite/**/*.{js,ts}"
  ],
  plugins: [require('daisyui'),require('flowbite/plugin')],
  daisyui: {
    themes: [
      {
        wimbledon: {
          "primary": "#3a0b7de6",
          "secondary": "#388564",
          "wimYellow": "#feed6b",
          "secondary-200": "#CDE1D8",
          "accent": "#34156f",
          "neutral": "#73ce9f",
          "base-100": "#ffffff",
          "primary-content": "#000000",
          "secondary-content": "#ffffff",
          "accent-content": "#ffffff",
          "wimbledonpurple": "#3a0b7d"
        },
      },
      "dark",
      "cupcake",
    ],
  },
}

