<template>
    <dialog id="CreateReservationModal" class="modal">
        <div class="modal-box">
            <form method="dialog">
                <button class="btn btn-sm btn-circle btn-ghost absolute right-2 top-2">✕</button>
            </form>
            <h2 class="text-2xl font-bold mb-4 form-title">Nouvelle Partie ! 🎾</h2>
            <form @submit.prevent="CreateReservation" class="flex flex-col">
                <div class="mb-4">
                    Terrain
                    <div>{{ selectedSchedule.zone }}</div>
                </div>
                <div class="mb-4">
                    Horaire d'ouverture
                    <div>{{ selectedSchedule.start_at }}</div>
                </div>
                <div class="mb-4">
                    Horaire de fermeture
                    <div>{{ selectedSchedule.end_at }}</div>
                </div>
                <div class="mb-4">
                    Joueur adverse
                    <div>
                        <select class="select select-error w-full max-w-xs" v-model="opponent_user">
                            <option disabled selected>Choisissez votre adversaire</option>
                            <option v-for="user in users" :key="user.id" :value="user">{{ user.username }}</option>
                        </select>
                    </div>
                </div>
                <button type="submit" class="btn btn-primary self-end">Créer</button>
            </form>
        </div>
        <form method="dialog" class="modal-backdrop">
            <button>close</button>
        </form>
    </dialog>


    <Modal ref="deleteModale" :showCancel="false">
        <h2 class=" text-black text-2xl font-bold mb-4">{{ popUpParams.title }}</h2>
        <p class="text-black pb-5">{{ popUpParams.text }}</p>
    </Modal>

</template>

<script setup>
import { useClubStore } from '~/stores/club'
import { useAuthStore } from '~/stores/auth'
const clubStore = useClubStore()
const club = clubStore.getClub()
const authStore = useAuthStore()
const token = authStore.getToken()
const opponent_user = ref(null)
let users = ref([])
const deleteModale = ref()

const popUpParams = ref({
    title: "Terrain réservé",
    text: "Vous allez recevoir un email de confirmation. Bonne partie ! 🎾",
});

const props = defineProps({
    selectedSchedule: Object,
    required: true
})

const showInfoModal = () => {
    deleteModale.value.show()
}

const selectedSchedule = props.selectedSchedule

const emit = defineEmits(['submit'])

const CreateReservation = async () => {
    try {
        const day = selectedSchedule.day;
        const startAt = selectedSchedule.start_at;
        const endAt = selectedSchedule.end_at;

        // Combine day and time into a single Date object
        const startDateTime = new Date(`${day}T${startAt}`);
        const endDateTime = new Date(`${day}T${endAt}`);

        // Format the date to match NaiveDateTime format without the 'Z' character
        const formatDateTime = (date) => {
            const year = date.getFullYear();
            const month = String(date.getMonth() + 1).padStart(2, '0');
            const day = String(date.getDate()).padStart(2, '0');
            const hours = String(date.getHours()).padStart(2, '0');
            const minutes = String(date.getMinutes()).padStart(2, '0');
            const seconds = String(date.getSeconds()).padStart(2, '0');
            return `${year}-${month}-${day}T${hours}:${minutes}:${seconds}`;
        };

        const formattedStartAt = formatDateTime(startDateTime);
        const formattedEndAt = formatDateTime(endDateTime);

        console.log(selectedSchedule.zone_id);
        console.log(formattedStartAt);
        console.log(formattedEndAt);
        console.log(opponent_user.value.id);

        await apiPost("/claim-slot", {
            body: JSON.stringify({
                zone_id: selectedSchedule.zone_id,
                start_at: formattedStartAt,
                end_at: formattedEndAt,
                opponent_user_id: opponent_user.value.id,
            }),
        });
        emit('submit');
        document.getElementById('CreateReservationModal').close();
        getAvailableClaims();
        showInfoModal();
    } catch (err) {
        console.error(err);
    }
}


const getAllUsers = async () => {
    const response = await apiGet('/users', {
        headers: {
            Authorization: `Bearer ${token}`,
        },
    });
    const data = await response.json();
    users.value = data;
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

getAllUsers()
</script>