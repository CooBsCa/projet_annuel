<template>

    <div class="flex">
        <div class="grid grid-cols-1 gap-4">
            <h1 class="text-2xl font-bold">Vos réservations</h1>
            <template v-for="slot in claimedSlots" :key="slot.id">
                <Reservation @delete="getClaimedSlots" :reservation="slot" />
            </template>
        </div>
        <div class="grow flex flex-col items-center">
            <h1 class="text-2xl font-bold">Progression</h1>
            <div class="radial-progress mt-5 text-accent border-4 drop-shadow-lg"
                :style="'--value:' + percentage + '; --size: 20rem; --thickness: 2rem; '" role="progressbar">
                <div class="text-6xl">{{ level }}</div>
            </div>


            <div class="stats shadow mt-10">

                <div class="stat place-items-center">
                    <div class="stat-title">Total</div>
                    <div class="stat-value">{{ totalReservations }}</div>
                    <div class="stat-desc">Depuis la création de votre compte</div>
                </div>

                <div class="stat place-items-center">
                    <div class="stat-title">Mois courant</div>
                    <div class="stat-value text-secondary">{{ currentMonthNumber }}</div>
                    <div class="stat-desc text-secondary">Réservations</div>
                </div>

                <div class="stat place-items-center">
                    <div class="stat-title">Niveau</div>
                    <div class="stat-value">{{ slotBeforeNextLevel }}</div>
                    <div class="stat-desc">Réservations avant le prochain palier</div>
                </div>

            </div>
        </div>
    </div>


</template>
<script setup>
import { useClubStore } from '~/stores/club'
import { useLevelStore } from '~/stores/level'

const levelStore = useLevelStore()
const clubStore = useClubStore()
const club = clubStore.getClub()

const claimedSlots = ref([])
const level = levelStore.getLevel()
const percentage = levelStore.getPercentage()
const totalReservations = levelStore.getPastClaimsNumber()

const currentMonthNumber = computed(() => {
    return claimedSlots.value.filter(slot => {
        return new Date(slot.start_at).getMonth() === new Date().getMonth()
    }).length
})

const slotBeforeNextLevel = computed(() => {
    return levelStore.getSlotBeforeNextLevel()
})

const getZones = async () => {
    const response = await apiGet('/zones/' + club.id, {

    });
    const zones = await response.json();
    console.log(zones)
    let slots = await getClaimedSlots()
    console.log(slots)
    slots.forEach(element => {
        element.zone_name = zones.find(zone => zone.id === element.zone_id).name
    });
    slots.sort((a, b) => b.start_at - a.start_at)
    claimedSlots.value = slots
};
const getClaimedSlots = async () => {
    try {
        const response = await apiGet('/claimed-slots', {
        });

        return await response.json()
    } catch (err) {
        console.error(err)
    }
}
const getMockClaimedSlots = () => {
    const current_date = new Date()
    const previous_date = new Date(current_date)
    previous_date.setDate(current_date.getDate() - 1)
    return [
        {
            id: 1,
            zone_id: 6,
            start_at: get_date_with_hour(current_date, 10),
            end_at: get_date_with_hour(current_date, 11)
        },
        {
            id: 2,
            zone_id: 7,
            start_at: get_date_with_hour(current_date, 18),
            end_at: get_date_with_hour(current_date, 19)
        },
        {
            id: 3,
            zone_id: 8,
            start_at: get_date_with_hour(previous_date, 20),
            end_at: get_date_with_hour(previous_date, 21)
        }
    ]
}

const get_date_with_hour = (date, hour) => {
    let new_date = new Date(date)
    new_date.setHours(hour, 0, 0, 0)
    return new_date
}

getZones()
</script>