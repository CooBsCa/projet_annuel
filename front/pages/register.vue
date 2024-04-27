<template>
  <div class="flex justify-center items-center h-screen bg bg-cover">
    <div class="card p-8 shadow-xl rounded-lg">
      <h2 class="text-2xl font-bold mb-4 form-title">Rejoignez nous ! 🎾</h2>
      <form @submit.prevent="register" class="flex flex-col">
        <div class="mb-4">
          <input type="text" placeholder="Username" class="input w-full" v-model="username" required />
          <!--<input type="text" placeholder="Username" class="input w-full" v-model="username" required>-->
        </div>
        <div class="mb-4">
          <input type="email" placeholder="Email" class="input w-full" v-model="email" required>
        </div>
        <div class="mb-4">
          <input type="password" placeholder="Password" class="input w-full" v-model="password" required>
        </div>
        <div class="mb-4">
          <input type="text" placeholder="Identifiant Club" class="input w-full" v-model="id_club" required>
        </div>
        <button type="submit" class="btn btn-primary self-end">Register</button>
      </form>
    </div>
  </div>
</template>

<script>
export default {
  data() {
    return {
      username: '',
      email: '',
      password: '',
      idClub: '',
      showSidebar: true
    };
  },
  methods: {
    async register() {
      try {
        const response = await apiPost('/register', {
          body: JSON.stringify({
            username: this.username,
            email: this.email,
            password: this.password,
            id_club: +this.id_club,
          })
        });
        this.$router.push('/login')
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
  background-color: rgba(58, 11, 125, 0.8);
}

.input {
  width: 100%;
  background-color: rgba(58, 11, 125, 1);
  border-color: rgba(254, 237, 107, 0.6);
}

.input::placeholder {
  color: rgba(254, 237, 107, 0.7);
}

.form-title {
  color: rgba(254, 237, 107, 1);
}

.bg {
  background-image: url('../../images/wimbledon_background.jpeg');
}

.btn-primary {
  background-color: rgba(225, 96, 205, 1);
  border: rgba(225, 96, 205, 1);
  color: rgba(58, 11, 125, 1);
}
</style>