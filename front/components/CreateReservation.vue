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
                        <select class="select select-error w-full max-w-xs">
                            <option disabled selected>Choisissez botre adversaire</option>
                            <option v-for="user in users" :key="user.id">{{ user.username }}</option>
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

</template>

<script setup>
import { useClubStore } from '~/stores/club'
import { useAuthStore } from '~/stores/auth'
const clubStore = useClubStore()
const club = clubStore.getClub()
const authStore = useAuthStore()
const token = authStore.getToken()
const nom = ref('')
const open_at = ref('')
const close_at = ref('')
const reservation_time = ref('')
let users = ref([])

const props = defineProps({
    selectedSchedule: Object,
    required: true
})

const selectedSchedule = props.selectedSchedule

const emit = defineEmits(['submit'])

// const CreateReservation = async () => {
//     try {
//         await apiPost("/zone", {
//             body: JSON.stringify({
//                 club_id: club.id,
//                 name: nom.value,
//                 open_at: open_at.value,
//                 close_at: close_at.value,
//                 reservation_time: reservation_time.value,
//             }),
//         });
//         emit('submit')
//         document.getElementById('CreateReservationModal').close()
//     } catch (err) {
//         console.error(err)
//     }
// }

const getAllUsers = async () => {
    const response = await apiGet('/users', {
        headers: {
            Authorization: `Bearer ${token}`,
        },
    });
    const data = await response.json();
    console.log(data);
    users.value = data;
};

getAllUsers()
</script>