<template>
    <!-- Form to modify Club name and crud for the zones-->
    <div class="flex flex-row justify-center">
        <label class="form-control w-full max-w-xs">
            <div class="label">
                <span class="label-text">Nom du club</span>
            </div>


            <div class="flex flex-row justify-center gap-5"><input v-model="club.name" type="text"
                    class="input input-bordered w-full max-w-xs" /><button @click="saveClubName"
                    class="btn btn-primary">Enregistrer</button></div>
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
const authStore = useAuthStore();
const clubStore = useClubStore()
const club = clubStore.getClub()

const zones = ref([])
const getZones = async () => {
    let token = authStore.getToken()
    try {
        const response = await fetch("http://localhost:3001/zones/" + club.id, {
            method: "GET",
            headers: {
                accept: "application/json",
                "Content-Type": "application/json",
                Authorization: `Bearer ${token}`,
            },
        });
        zones.value = await response.json();
    } catch (err) {
        console.error(err)
    }
}

const saveClubName = async () => {
    let token = authStore.getToken()
    try {
        await fetch("http://localhost:3001/club", {
            method: "PUT",
            headers: {
                accept: "application/json",
                "Content-Type": "application/json",
                Authorization: `Bearer ${token}`,
            },
            body: JSON.stringify({
                name: club.name,
                id: club.id
            }),
        });
    } catch (err) {
        console.error(err)
    }
}

await getZones()
</script>