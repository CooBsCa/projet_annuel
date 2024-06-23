<template>
  <div v-if="showLayout" data-theme="wimbledon">
    <NuxtLayout>
      <NuxtPage />
    </NuxtLayout>
  </div>
  <div v-else data-theme="wimbledon">
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
      return !['/home', '/password_reset', '/register'].includes(this.$route.path);
    },
  },
  methods: {
    isLogin() {
      true
    }
  }
};
</script>
