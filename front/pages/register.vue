<template>
    <div class="flex justify-center items-center h-screen">
      <div class="card p-8 bg-white shadow-xl rounded-lg">
        <h2 class="text-2xl font-bold mb-4">Rejoignez nous ! 🎾</h2>
        <input type="text" placeholder="Username" class="input mb-4" v-model="username">
        <input type="text" placeholder="Email" class="input mb-4" v-model="email">
        <input type="password" placeholder="Password" class="input mb-4" v-model="password">
        <button class="btn btn-primary w-full" @click="register">Register</button>
      </div>
    </div>
  </template>
  
  <script>
  export default {
    data() {
      return {
        username: '',
        email: '',
        password: ''
      };
    },
    methods: {
    async register() {
      try {
        const response = $fetch('http://localhost:3001/register', {
          method: 'POST',
          headers: {
            'accept' : 'applicaton/json',
            'Content-Type': 'application/json'
          },
          body: JSON.stringify({
            username: this.username,
            email: this.email,
            password: this.password
          })
        });

        console.log(response);
      } catch (error) {
        console.error('Erreur de connexion:', error);
      }
    }
  }
};
</script>
  
<style scoped>
  .card {
    width: 400px;
  }
</style>
  