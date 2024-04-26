// https://nuxt.com/docs/api/configuration/nuxt-config
export default defineNuxtConfig({
  modules: ["@nuxtjs/tailwindcss", "@pinia/nuxt"],
  devtools: { enabled: true },
  $development: {
    runtimeConfig: {
      public: {
        baseURL: process.env.BASE_URL || "http://locahost:3001",
      },
    },
  },
});