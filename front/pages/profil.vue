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
    <div class="divider"></div>
    <label class="form-control w-full max-w-xs">
        <span class="label-text mb-4">Changer votre mot de passe ?</span>
        <div class="label">
            <span class="label-text">Mot de passe actuel</span>
        </div>
        <div class="flex flex-row justify-center gap-5"><input v-model="password" type="password"
                class="input input-bordered w-full max-w-xs" /></div>

        <div class="label">
            <span class="label-text">Nouveau Mot de passe</span>
        </div>

        <div class="flex flex-row justify-center gap-5"><input v-model="new_password" type="password"
                class="input input-bordered w-full max-w-xs" />
        </div>
        <button @click="saveNewPassword" class="btn btn-primary wimYellow mt-4">Modifier</button>
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
const new_password = ref('')
const password = ref('')

let popUpAlertParams = ref({
    title: "",
    text: "",
});

const saveEmail = async () => {

    try {
        if (email.value === '') {
            showAlertModal("Champ vide", "Veuillez renseigner un email")
            return
        }
        if (email.value === authStore.getEmail()) {
            showAlertModal("Champ inchangé", "Veuillez renseigner un email différent de celui actuel")
            return
        }
        if (email.value.indexOf('@') === -1) {
            showAlertModal("🚨 Email invalide", "Veuillez renseigner un email valide")
            return
        }
        console.log(email)
        console.log(userid)
        await apiPut("/user-email", {
            body: JSON.stringify({
                id: +userid,
                email: email.value,
            }),
        });

        notifyStore.notify("User modifié avec succès", NotificationType.Success);
        showAlertModal("Modification réussie", "Votre email a bien été modifié")
    } catch (err) {
        showAlertModal("Il semble que la modication ait échouée", "Veuillez vérifier votre saisie et réessayer.");
        notifyStore.notify(" 🚨 Une erreur est survenue lors de l'enregistrement", NotificationType.Error);
        console.error(err)
    }
}

const saveNewPassword = async () => {

    try {
        // regarder si il y a pas d'espaces, une majuscule, un chiffre, un caractère spécial, une longueur de 8 caractères
        const regex = /^(?=.*[a-z])(?=.*[A-Z])(?=.*\d)(?=.*[@$!%*?&])[A-Za-z\d@$!%*?&]{8,}$/
        if (!regex.test(new_password.value)) {
            showAlertModal("🚨 Mot de passe invalide", "Votre mot de passe doit comporter au moins une majuscule, un chiffre, un caractère spécial et doit respecter une longueur minimum de 8 caractères")
            return
        }
        if (new_password.value === '') {
            showAlertModal("🚨 Champ vide", "Veuillez renseigner un nouveau mot de passe")
            return
        }
        if (new_password.value === password.value) {
            showAlertModal("🚨 Mot de passe inchangé", "Veuillez renseigner un mot de passe différent de celui actuel")
            return
        }


        await apiPut("/user-new-password", {
            body: JSON.stringify({
                id: +userid,
                password: password.value,
                new_password: new_password.value,
            }),
        });

        notifyStore.notify("User modifié avec succès", NotificationType.Success);
        showAlertModal("Modification réussie", "Votre nouveau mot de passe a été pris en compte");
        new_password.value = ''
        password.value = ''
    } catch (err) {
        showAlertModal("Il semble que la modication ait échouée", "Veuillez vérifier vos informations et réessayer.");
        notifyStore.notify(" 🚨 Une erreur est survenue lors de l'enregistrement", NotificationType.Error);
        console.error(err)
    }
}

const showAlertModal = (title, data) => {
    popUpAlertParams.value.title = title
    popUpAlertParams.value.text = data
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