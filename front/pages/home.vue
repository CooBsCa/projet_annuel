<template>
  <div class="flex justify-center items-center h-screen bg bg-cover">
    <!-- Colonne pour le formulaire d'inscription -->
    <div class="flex justify-center items-center w-1/2">
      <div class="card p-8 shadow-xl rounded-lg">
        <h2 class="text-2xl font-bold mb-4 form-title">Rejoignez nous ! 🎾</h2>
        <form @submit.prevent="register" class="flex flex-col">
          <div class="mb-4">
            <input type="text" placeholder="Username" class="input w-full" v-model="username" required />
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

    <!-- Colonne pour le formulaire de login -->
    <div class="flex justify-center items-center w-1/2">
      <div class="card p-8 shadow-xl rounded-lg">
        <h2 class="text-2xl font-bold mb-4 form-title">Un petit Match ? 🏆</h2>
        <form @submit.prevent="login" class="flex flex-col">
          <div class="mb-4">
            <input type="text" placeholder="Username" class="input w-full" v-model="usernameLogin" required />
          </div>
          <div class="mb-4">
            <input type="password" placeholder="Password" class="input w-full" v-model="passwordLogin" required>
          </div>
          <button type="submit" class="btn btn-primary self-end">Login</button>
        </form>
      </div>
    </div>
  </div>
</template>

<script setup>
import { ref } from 'vue'
import { useAuthStore } from '~/stores/auth'
import { useClubStore } from '~/stores/club'

const authStore = useAuthStore();
const clubStore = useClubStore()
authStore.setToken("");

const error = ref(false)
const errorMessage = ref('')

const username = ref('')
const email = ref('')
const password = ref('')
const id_club = ref('')
const usernameLogin = ref('')
const passwordLogin = ref('')

const login = async () => {
  try {
    const response = await fetch('http://localhost:3001/login', {
      method: 'POST',
      headers: {
        'accept': 'application/json',
        'Content-Type': 'application/json'
      },
      body: JSON.stringify({
        user_login_input: usernameLogin.value,
        password: passwordLogin.value,
      })
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
const register = async () => {
  try {
    const response = await fetch('http://localhost:3001/register', {
      method: 'POST',
      headers: {
        'accept': 'application/json',
        'Content-Type': 'application/json'
      },
      body: JSON.stringify({
        username: username.value,
        email: email.value,
        password: password.value,
        id_club: +id_club.value,
      })
    });

    let res = await response.json();

    authStore.setToken(res.uuid)
    authStore.setIsAdmin(res.is_admin)
    authStore.setUsername(res.username)

    await getClub();

  } catch (err) {
    error.value = true
    console.error(err)
    errorMessage.value = 'Impossible de créer le compte, veuillez réessayer.'
  }
}
const getClub = async () => {
  let token = authStore.getToken()
  try {
    const response = await apiGet("/club", {
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
  background-color: #388564;
}

.input {
  width: 100%;
  background-color: white;
}

.input::placeholder {
  color: #388564;
}

.form-title {
  color: white;
}

.bg {
  background-image: url('../../images/wimbledon_background.jpeg');
}

.btn-primary {
  background-color: #34156f;
  border: #e6c9a4;
  color: white;
}
</style>