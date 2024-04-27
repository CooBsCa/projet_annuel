<template>
    <button class="btn btn-primary" onclick="CreateZoneModal.showModal()">Ajouter une zone</button>
    <dialog id="CreateZoneModal" class="modal">
        <div class="modal-box">
            <form method="dialog">
                <button class="btn btn-sm btn-circle btn-ghost absolute right-2 top-2">✕</button>
            </form>
            <h2 class="text-2xl font-bold mb-4 form-title">Nouvelle zone 🎾</h2>
            <form @submit.prevent="CreateZone" class="flex flex-col">
                <div class="mb-4">
                    Nom
                    <input type="text" class="input input-bordered w-full" v-model="nom" required />
                </div>
                <div class="mb-4">
                    Horaire d'ouverture
                    <input type="time" class="input input-bordered w-full" v-model="open_at" required>
                </div>
                <div class="mb-4">
                    Horaire de fermeture
                    <input type="time" class="input input-bordered w-full" v-model="close_at" required>
                </div>
                <div class="mb-4">
                    Durée de réservation (en minutes)
                    <input type="number" class="input input-bordered w-full" v-model="reservation_time" required>
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

const emit = defineEmits(['submit'])

const CreateZone = async () => {
    try {
        await fetch("http://localhost:3001/zone", {
            method: "POST",
            headers: {
                accept: "application/json",
                "Content-Type": "application/json",
                Authorization: `Bearer ${token}`,
            },
            body: JSON.stringify({
                club_id: club.id,
                name: nom.value,
                open_at: open_at.value,
                close_at: close_at.value,
                reservation_time: reservation_time.value,
            }),
        });
        emit('submit')
        document.getElementById('CreateZoneModal').close()
    } catch (err) {
        console.error(err)
    }
}
</script>