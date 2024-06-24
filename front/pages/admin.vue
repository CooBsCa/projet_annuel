<template>
    <!-- Form to modify Club name and crud for the zones-->
    <div class="flex flex-row justify-center">
        <label class="form-control w-full max-w-xs">
            <div class="label">
                <span class="label-text">Nom du club</span>
            </div>
            <div class="flex flex-row justify-center gap-5"><input v-model="club.name" type="text"
                    class="input input-bordered w-full max-w-xs" /><button @click="saveClubName"
                    class="btn btn-primary wimYellow">Enregistrer</button></div>
        </label>

    </div>

    <ZoneList v-model="zones" />
    <div class="flex justify-center">
        <CreateZone @submit="getZones" />
    </div>

</template>

<script setup>
import { useAuthStore } from '~/stores/auth'
import { useClubStore } from '~/stores/club'
import { useNotifyStore, NotificationType } from '~/stores/notify'

const notifyStore = useNotifyStore()
const authStore = useAuthStore();
const clubStore = useClubStore()
const club = clubStore.getClub()

const zones = ref([])
const getZones = async () => {
    let token = authStore.getToken()
    try {
        const response = await apiGet("/zones/" + club.id, {});
        zones.value = await response.json();
    } catch (err) {
        console.error(err)
    }
}

const saveClubName = async () => {

    try {
        await apiPut("/club", {
            body: JSON.stringify({
                name: club.name,
                id: club.id
            }),
        });
        notifyStore.notify("Club modifié avec succès", NotificationType.Success);
    } catch (err) {
        notifyStore.notify("Une erreur est survenue lors de l'enregistrement", NotificationType.Error);
        console.error(err)
    }
}

await getZones()
</script>

<style scoped>
.wimYellow {
    text-color: #FFD700;
}
</style>