<template data-theme="wimbledon">
    <div class="navbar bg-secondary mb-5 rounded-full">
        <div class="flex-1">
            <div class="text-bl text-lg font-bold text-white pl-5">Modification des informations de votre profil</div>
        </div>
    </div>
    <label class="form-control w-full max-w-xs">
        <div class="label">
            <span class="label-text">Votre email</span>
        </div>


        <div class="flex flex-row justify-center gap-5"><input v-model="email" type="email"
                class="input input-bordered w-full max-w-xs" /><button @click="saveEmail"
                class="btn btn-primary wimYellow">Modifier</button></div>
    </label>
    <Modal ref="alertModal" :showCancel="false">
        <h2 class="text-black text-2xl font-bold mb-4">{{ popUpAlertParams.title }}</h2>
        <p class="text-black pb-5">{{ popUpAlertParams.text }}</p>
    </Modal>
</template>

<script setup>
import { useAuthStore } from '~/stores/auth'
import { useClubStore } from '~/stores/club'
import { useNotifyStore, NotificationType } from '~/stores/notify'
const notifyStore = useNotifyStore()
const authStore = useAuthStore()
const authToken = authStore.getToken()
const userid = authStore.getUserId()
const email = ref(authStore.getEmail())
const alertModal = ref()

let popUpAlertParams = ref({
    title: "Modification de réussie",
    text: "",
});

const saveEmail = async () => {

    try {
        console.log(email)
        console.log(userid)
        const response = await apiPut("/user-email", {
            body: JSON.stringify({
                id: +userid,
                email: email.value,
            }),
        });

        notifyStore.notify("User modifié avec succès", NotificationType.Success);
        showAlertModal()
    } catch (err) {
        notifyStore.notify("Une erreur est survenue lors de l'enregistrement", NotificationType.Error);
        console.error(err)
    }
}

const showAlertModal = () => {
    popUpAlertParams.value.text = "Votre email a bien été modifié"
    alertModal.value.show()
}
</script>

<style scoped>
.calendar {
    overflow: scroll;
    display: flex;
    flex-direction: column;
    min-width: 100%;
}

.slotReserved {
    background-color: rgba(58, 11, 125, 0.9);
}

.day {
    display: flex;
    width: 1500px;
    height: 100%;
    flex: 1
}

.court-column {
    overflow: hidden;
    flex: 1;
    display: flex;
    flex-direction: column;
    border-left: 1px solid #ccc;
    border-right: 1px solid #ccc;
}

.court-name {
    padding: 5px;
    font-weight: bold;
    text-align: center;
}

.slot {
    border-bottom: 1px solid #ccc;
    cursor: pointer;
    /* height: 60px; */
    display: flex;
    align-items: center;
    justify-content: center;
}

.empty-slot {
    height: 60px;
}

.slot:hover {
    background-color: #f0f0f0;
}

.current-hour-slot {
    background-color: #ccc;
    pointer-events: none;
}

.img1 {
    background-image: url('../../images/fleche_droite.png');
    background-size: cover;
    width: 50px;
    height: 50px;
}

.img2 {
    background-image: url('../../images/fleche_gauche.png');
    background-size: cover;
    width: 50px;
    height: 50px;
}
</style>