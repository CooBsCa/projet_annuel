<template>
  <div class="flex justify-center items-center h-screen bg bg-cover">
    <div class="card p-8 shadow-xl rounded-lg">
      <h2 class="text-2xl font-bold mb-4 form-title">Rejoignez nous ! 🎾</h2>
      <form @submit.prevent="login" class="flex flex-col">
        <div class="mb-4">
          <input type="text" placeholder="Username" class="input w-full" v-model="username" required />
        </div>
        <div class="mb-4">
          <input type="password" placeholder="Password" class="input w-full" v-model="password" required>
        </div>
        <button type="submit" class="btn btn-primary self-end">Login</button>
      </form>
    </div>
  </div>
</template>

<script setup>
import { ref } from 'vue'
import { useAuthStore } from '~/stores/auth'
import { useClubStore } from '~/stores/club'

const authStore = useAuthStore();
const clubStore = useClubStore()

const username = ref('clem')
const password = ref('aze')
const error = ref(false)
const errorMessage = ref('')

const login = async () => {


  try {

    const response = await fetch("http://localhost:3001/login", {
      method: "POST",
      headers: {
        accept: "application/json",
        "Content-Type": "application/json",
      },
      body: JSON.stringify({
        user_login_input: username.value,
        password: password.value,
      }),
    });
    let res = await response.json();

    authStore.setToken(res.uuid)
    authStore.setIsAdmin(res.is_admin)
    authStore.setUsername(res.username)

    await getClub();
  } catch (err) {
    error.value = true
    console.error(err)
    errorMessage.value = 'Impossible de se connecter, veuillez réessayer.'
  }
}

const getClub = async () => {

  let token = authStore.getToken()
  try {
    const response = await fetch("http://localhost:3001/club", {
      method: "GET",
      headers: {
        accept: "application/json",
        "Content-Type": "application/json",
        Authorization: `Bearer ${token}`,
      },
    });
    const club = await response.json();
    clubStore.setClub(club)
    await navigateTo('/calendar')
  } catch (error) {
    console.error("Erreur de connexion:", error);
  }
}
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