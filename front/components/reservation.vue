<template>

    <div class="card w-96 bg-primary text-primary-content shadow-xl" :class="{
        'opacity-50': isPast
    }">
        <div class="card-body">
            <h2 class="card-title">{{ reservation.zone_name }}</h2>
            <p>{{ date_txt }}</p>
            <p>{{ start_at_txt }} - {{ end_at_txt }}</p>
            <div class="card-actions justify-end" v-if="!isPast">
                <button class="btn btn-accent text-accent-content">Annuler</button>
            </div>
        </div>
    </div>
</template>

<script setup>
const props = defineProps({
    reservation: Object,
    required: true
})
const reservation = props.reservation
const date_txt = ref("")
const start_at_txt = ref("")
const end_at_txt = ref("")
const isPast = computed(() => {
    return new Date() > reservation.start_at
})


onMounted(() => {
    date_txt.value = reservation.start_at.toLocaleDateString("fr-FR", { weekday: 'long', year: 'numeric', month: 'long', day: 'numeric' })
    start_at_txt.value = reservation.start_at.toLocaleTimeString("fr-FR", { hour: '2-digit', minute: '2-digit' })
    end_at_txt.value = reservation.end_at.toLocaleTimeString("fr-FR", { hour: '2-digit', minute: '2-digit' })
})


</script>