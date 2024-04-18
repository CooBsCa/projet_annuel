<template>
  <div v-if="authentificated">
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
  methods: {
    isLogin() {
      true
    }
  }
};
</script>
