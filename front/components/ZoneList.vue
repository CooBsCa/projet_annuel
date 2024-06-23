<template>
    <div class="overflow-x-auto">
        <table class="table">
            <!-- head -->
            <thead>
                <tr>
                    <th>Nom</th>
                    <th>Horaire d'ouverture</th>
                    <th>Horaire de fermeture</th>
                    <th>Durée de réservation (minutes)</th>
                </tr>
            </thead>
            <tbody>
                <!-- row 1 -->
                <tr v-for="zone in zones">
                    <th>{{ zone.name }}</th>
                    <td>{{ zone.open_at }}</td>
                    <td>{{ zone.close_at }}</td>
                    <td>{{ zone.reservation_time }}</td>
                    <td>
                        <button class="btn btn-circle" @click="modaleDeleteZone(zone)">
                            <svg xmlns="http://www.w3.org/2000/svg" class="h-6 w-6" fill="none" viewBox="0 0 24 24"
                                stroke="currentColor">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
                                    d="M6 18L18 6M6 6l12 12" />
                            </svg>
                        </button>
                    </td>
                </tr>
            </tbody>
        </table>
    </div>
    <Modal ref="deleteModale" @confirm="deleteZone">Voulez-vous vraiment supprimer la zone {{ selectedZone.name }}
    </Modal>
</template>

<script setup>
const zones = defineModel()
const deleteModale = ref()
const selectedZone = ref()

const modaleDeleteZone = (zone) => {
    selectedZone.value = zone
    deleteModale.value.show()
}

const deleteZone = async () => {
    try {
        await apiDelete("/zone/" + selectedZone.value.id, {})
        zones.value = zones.value.filter((zone) => zone.id !== selectedZone.value.id)
    } catch (err) {
        console.error(err)
    }
}
</script>

<style scoped>
.winYellow {
    text-color: #FFD700;
}
</style>