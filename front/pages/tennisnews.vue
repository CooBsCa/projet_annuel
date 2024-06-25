<template>
    <div class="navbar bg-secondary mb-5 rounded-full items-center justify-center">
        <div class="justify-center">
            <div class="text-bl text-lg font-bold text-white pl-5">Actualité Matchs</div>
        </div>
    </div>
    <div class="flex justify-between mx-10">
        <div class="card bg-base-100 w-96 shadow-xl">
            <div class="flex items-center justify-center">
                <h1><strong>🏆 Classement ATP</strong></h1>
            </div>

            <div class="overflow-x-auto">
                <table class="table table-zebra">
                    <!-- head -->
                    <thead>
                        <tr>
                            <th></th>
                            <th>Joueur</th>
                        </tr>
                    </thead>
                    <tbody v-for="(player, index) in top_ten_atp" :key="index">
                        <tr>
                            <th>{{ index }}</th>
                            <td>{{ player.rowName }}</td>
                        </tr>
                    </tbody>
                </table>
            </div>
        </div>
        <div class="card bg-base-100 w-96 shadow-xl ml-auto">
            <div class="flex items-center justify-center">
                <h1><strong>Qualifiers pour Wimbledon</strong></h1>
            </div>

            <div class="overflow-x-auto">
                <table class="table table-zebra">
                    <!-- head -->
                    <thead>
                        <tr>
                            <th>Joueur 1</th>
                            <th></th>
                            <th>Joueur 2</th>
                        </tr>
                    </thead>
                    <tbody v-for="(match, index) in match_of_the_day.slice(0, 10) " :key="index">
                        <tr>
                            <th>{{ match.homeTeam.name }}</th>
                            <th>VS</th>
                            <td>{{ match.awayTeam.name }}</td>
                        </tr>
                    </tbody>
                </table>
            </div>
        </div>
    </div>
</template>

<script setup>
import { ref } from 'vue'
import { useAuthStore } from '~/stores/auth'
import { useClubStore } from '~/stores/club'
import { useLevelStore } from '~/stores/level'
const authStore = useAuthStore()


authStore.setToken("");

const error = ref(false)
const errorMessage = ref('')
const match_of_the_day = ref([])
const top_ten_atp = ref([])

const requestNews = async () => {
    try {
        const response = await fetch('https://tennisapi1.p.rapidapi.com/api/tennis/events/25/6/2024', {
            method: 'GET',
            headers: {
                'x-rapidapi-key': '264cbf4f32msh4e697ab9fe2686cp1f12b7jsn124ec7918e8c',
                'x-rapidapi-host': 'tennisapi1.p.rapidapi.com'
            },
        });


        let res = await response.json();
        match_of_the_day.value = res.events.filter(event => (event.tournament.id === 132759 || event.tournament.id === 132847) && event.status.code === 0);
    } catch (err) {
        error.value = true
        console.error(err)
        errorMessage.value = 'Impossible de se connecter, email ou mot de passe incorrect veuillez réessayer.'
    }
}

const topTenAtp = async () => {
    try {
        const response = await fetch('https://tennisapi1.p.rapidapi.com/api/tennis/rankings/atp', {
            method: 'GET',
            headers: {
                'x-rapidapi-key': '264cbf4f32msh4e697ab9fe2686cp1f12b7jsn124ec7918e8c',
                'x-rapidapi-host': 'tennisapi1.p.rapidapi.com'
            },
        });


        let res = await response.json();

        //récupérer seulement les 10 premiers joueurs
        top_ten_atp.value = res.rankings.slice(0, 10);
    } catch (err) {
        error.value = true
        console.error(err)
        errorMessage.value = 'Impossible de se connecter, email ou mot de passe incorrect veuillez réessayer.'
    }
}

requestNews()
topTenAtp()
</script>