<template>
    <header class="flex justify-center items-center py-4 bg-header">
        <h1 class="text-4xl font-site">Central Court</h1>
    </header>
    <div class="flex justify-center items-center h-screen bg bg-cover">
        <nuxt-link to="/home" class="absolute top-6 left-4 z-10">
            <div class="imgArrowBack"></div>
        </nuxt-link>
        <div class="flex justify-center w-1/2">
            <div class="card p-8 shadow-xl rounded-lg">
                <h2 class="text-2xl font-bold mb-4 form-title">📪 Un petit trou de mémoire ?</h2>
                <p class="text-white pb-5">Entrez votre adresse email.
                    Si cette adresse est liée à un compte, vous recevrez un email avec un lien pour réinitialiser
                    votre
                    mot de passe.
                </p>
                <form @submit.prevent="password_reset(email)" class="flex flex-col">
                    <div class="mb-4">
                        <input type="email" placeholder="Email" class="input w-full" v-model="email" required>
                    </div>
                    <button type="submit" class="btn btn-primary self-end">Envoyer</button>
                </form>
            </div>
        </div>
    </div>

    <Modal ref="deleteModale" :showCancel="false" @confirm="goToLogin">
        <h2 class="text-black text-2xl font-bold mb-4">{{ popUpParams.title }}</h2>
        <p class="text-black pb-5">{{ popUpParams.text }}</p>
    </Modal>
</template>

<script setup>
import { ref } from 'vue'

const deleteModale = ref()

const popUpParams = ref({
    title: "Surveillez votre boîte 📩",
    text: "Si un compte existe a cette adresse, alors un mail a été envoyé",
});

const email = ref('')

const showInfoModal = () => {
    deleteModale.value.show()
}

const goToLogin = () => {
    navigateTo("/home")
}

const password_reset = async (email) => {
    showInfoModal()
    try {
        await fetch('http://localhost:3001/password_reset', {
            method: 'POST',
            headers: {
                'accept': 'application/json',
                'Content-Type': 'application/json'
            },
            body: JSON.stringify({
                email: email
            }),
        });
    } catch (err) {
        console.error(err)
    }
}

</script>

<style scoped>
.card {
    width: 400px;
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
    background-color: rgba(225, 96, 205, 1);
    border: rgba(225, 96, 205, 1);
    color: rgba(58, 11, 125, 1);
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