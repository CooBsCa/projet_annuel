<template>
  <div v-if="showLayout">
    <NuxtLayout>
      <NuxtPage />
    </NuxtLayout>
  </div>
  <div v-else>
    <NuxtPage />
  </div>
</template>

<script>
definePageMeta({
  layout: 'default'
})
export default {
  data() {
    return {
      authentificated: false
    };
  },
  computed: {
    showLayout() {
      return !['/login', '/register'].includes(this.$route.path);
    },
  },
  methods: {
    isLogin() {
      true
    }
  }
};
</script>
