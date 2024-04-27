// https://nuxt.com/docs/api/configuration/nuxt-config
export default defineNuxtConfig({
  modules: [
    "@nuxtjs/tailwindcss",
    "@pinia/nuxt",
    "@pinia-plugin-persistedstate/nuxt",
  ],
  devtools: { enabled: true },
  $development: {
    runtimeConfig: {
      public: {
        baseURL: process.env.BASE_URL || "http://localhost:3001",
      },
    },
  },
});
