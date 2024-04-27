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
                </tr>
            </tbody>
        </table>
    </div>
</template>

<script setup>
const zones = defineModel()
</script>