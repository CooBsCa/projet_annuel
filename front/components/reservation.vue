<template>

    <div class="card w-96 bg-primary text-primary-content shadow-xl" :class="{
        'opacity-50': isPast
    }">
        <div class="card-body wimYellow">
            <h2 class="card-title wimYellow">{{ reservation.zone_name }}</h2>
            <p>{{ date_txt }}</p>
            <p>{{ start_at_txt }} - {{ end_at_txt }}</p>
            <p>{{ username + " - " + opponentName }}</p>
            <div class="card-actions justify-end wimYellow" v-if="!isPast">
                <button class="btn btn-secondary wimYellow" @click="modaleDelete">Annuler</button>
            </div>
        </div>
    </div>
    <Modal ref="deleteModale" @confirm="deleteReservation">Voulez-vous vraiment annuler la réservation du {{ date_txt
        }}
        de {{ start_at_txt }} à {{
        end_at_txt }} ?
    </Modal>
</template>

<script setup>
import { useAuthStore } from '~/stores/auth';
import { defineEmits } from 'vue';
const emit = defineEmits(['delete']);
const authStore = useAuthStore();
const authToken = authStore.getToken();
const username = authStore.getUsername();
const opponentName = ref('');
const props = defineProps({
    reservation: Object,
    required: true
})
const reservation = props.reservation
const date_txt = ref("")
const start_at_txt = ref("")
const end_at_txt = ref("")
const isPast = computed(() => {
    return new Date() > new Date(reservation.start_at);
});
const deleteModale = ref()

const modaleDelete = () => {
    deleteModale.value.show()
}

const deleteReservation = async () => {
    try {
        await apiDelete("/cancel-slot/" + reservation.id, {})
        emit("delete")
        getAvailableClaims()
    } catch (err) {
        console.error(err)
    }
}

onMounted(() => {
    const startDate = new Date(reservation.start_at);
    const endDate = new Date(reservation.end_at);
    date_txt.value = startDate.toLocaleDateString("fr-FR", { weekday: 'long', year: 'numeric', month: 'long', day: 'numeric' });
    start_at_txt.value = startDate.toLocaleTimeString("fr-FR", { hour: '2-digit', minute: '2-digit' });
    end_at_txt.value = endDate.toLocaleTimeString("fr-FR", { hour: '2-digit', minute: '2-digit' });
    getOpponentName(reservation.opponent_user_name);
});


const getOpponentName = async (user_name) => {
    opponentName.value = capitalizeFirstLetter(user_name);
};

const capitalizeFirstLetter = (string) => {
    return string.charAt(0).toUpperCase() + string.slice(1).toLowerCase();
};

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
</script>

<style scoped>
.winYellow {
    text-color: #FFD700;
}
</style>