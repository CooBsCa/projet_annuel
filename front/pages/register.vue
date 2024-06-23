<template>
  <header class="flex justify-center items-center py-4 bg-header">
    <h1 class="text-4xl font-site">Central Court</h1>
  </header>
  <div class="flex justify-center items-center h-screen bg bg-cover">
    <nuxt-link to="/home" class="absolute top-6 left-4 z-10">
      <div class="imgArrowBack"></div>
    </nuxt-link>
    <!-- Colonne pour le formulaire d'inscription -->
    <div class="flex justify-center items-center w-1/2">
      <div class="card p-8 shadow-xl rounded-lg w-96">
        <h2 class="text-2xl font-bold mb-4 form-title">Rejoignez nous ! 🎾</h2>
        <form @submit.prevent="register" class="flex flex-col">
          <div class="mb-4">
            <input type="text" placeholder="Identifiant" class="input w-full" v-model="username" required />
          </div>
          <div class="mb-4">
            <input type="email" placeholder="Email" class="input w-full" v-model="email" required>
          </div>
          <div class="mb-4">
            <input type="password" placeholder="Mot de Passe" class="input w-full" v-model="password" required>
          </div>
          <div class="mb-4">
            <input type="text" placeholder="Identifiant Club" class="input w-full" v-model="id_club" required>
          </div>
          <button type="submit" class="btn btn-primary self-end">Register</button>
        </form>
      </div>
    </div>
  </div>

  <Modal ref="errorModal" :showCancel="false" @confirm="goToLogin">
    <h2 class="text-black text-2xl font-bold mb-4">{{ popUpParams.title }}</h2>
    <p class="text-black pb-5">{{ popUpParams.text }}</p>
  </Modal>
</template>

<script setup>
import { ref } from 'vue'
import { useAuthStore } from '~/stores/auth'
import { useClubStore } from '~/stores/club'
import { useLevelStore } from '~/stores/level'

const authStore = useAuthStore()
const clubStore = useClubStore()
const levelStore = useLevelStore()

const errorModal = ref()
const popUpParams = ref({
  title: "",
  text: "",
});


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
    errorMessage.value = 'Impossible de se connecter, email ou mot de passe incorrect veuillez réessayer.'
    showErrorModal("de connexion")
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
    showErrorModal("d'inscription")
  }
}
const getClub = async () => {
  try {
    const response = await apiGet("/club", {
    });
    const club = await response.json();
    clubStore.setClub(club)
    await getAvailableClaims();
    await getPastClaims();
    await navigateTo('/calendar')
  } catch (error) {
    console.error("Erreur de connexion:", error);
  }
}

const getAvailableClaims = async () => {
  try {
    const response = await apiGet("/future-claimed-slots", {
    });
    const data = await response.json();
    authStore.setFuturClaimsNumber(data)
  } catch (error) {
    console.error("Erreur de connexion:", error);
  }
}

const getPastClaims = async () => {
  try {
    const response = await apiGet("/past-claimed-slots", {
    });
    const data = await response.json();
    levelStore.setPastClaimsNumber(data)
  } catch (error) {
    console.error("Erreur de connexion:", error);
  }
}

const goToLogin = () => {
  error.value = false
}

const showErrorModal = (input) => {
  popUpParams.value.title = "Erreur lors de la tentative " + input
  popUpParams.value.text = errorMessage.value
  errorModal.value.show()
  error.value = false
}

</script>

<style scoped>
@import url('https://fonts.googleapis.com/css2?family=Montserrat:wght@700&display=swap');

.card {
  background-color: rgba(56, 133, 100, 0.8);
}

.input {
  width: 100%;
  background-color: white;
}

.input::placeholder {
  color: #388564;
}

.form-title {
  color: rgba(254, 237, 107, 1);
}

.bg {
  background-image: url('../../images/wimbledon_background.jpeg');
}


.btn-primary {
  background-color: rgba(58, 11, 125, 0.9);
  border: rgba(58, 11, 125, 0.9);
  color: rgba(254, 237, 107, 1);
}

.forgot-password-link {
  color: rgba(254, 237, 107, 1);
  font-size: 14px;
  text-decoration: underline;
}

.bg-header {
  background-color: rgba(58, 11, 125, 0.9);
}

.font-site {
  font-family: 'Montserrat', sans-serif;
  color: rgba(254, 237, 107, 1);
}

.imgArrowBack {
  background-image: url('../../images/arrow_back.png');
  background-size: cover;
  width: 25px;
  height: 25px;
}
</style>
