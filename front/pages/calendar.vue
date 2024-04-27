<template>
    <h1 class="title">{{ message }}</h1>
    <div v-for="zone in zones">
        {{ zone.name }}
    </div>
</template>

<script setup>
// let authToken = useState('authToken')
import { useAuthStore } from '~/stores/auth'
import { useClubStore } from '~/stores/club'
const authStore = useAuthStore()
const authToken = authStore.getToken()

const clubStore = useClubStore()
const club = clubStore.getClub()
const date = ref(new Date());
const zones = ref([]);

const getZones = async () => {
    const response = await apiGet('/zones/' + club.id, {
        headers: {
            Authorization: `Bearer ${authToken}`,
        },
    });
    const data = await response.json();
    console.log(data);
    zones.value = data;
};

const getSlots = async () => {
    const response = await apiGet('/slots', {
        body: JSON.stringify({
            date: date.value,
        }),
    });
    const data = await response.json();
    console.log(data);
};
getZones()
</script>


<script>
export default {
    name: 'Calendar',

    data() {
        return {
            message: 'Calendar',
        };
    },
};
</script>