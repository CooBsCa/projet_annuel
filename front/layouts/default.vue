<template>
    <div class="flex h-screen">
        <!-- Toggle Button -->
        <button @click="toggleSidebar" v-if="!showSidebar"
            class="absolute top-1/2 transform -translate-y-1/2 left-0 mr-2 focus:outline-none">

            <svg xmlns="http://www.w3.org/2000/svg" class="h-6 w-6" fill="none" viewBox="0 0 24 24"
                stroke="currentColor">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 5l7 7-7 7" />
            </svg>
        </button>
        <!-- Sidebar -->
        <div class="bg-secondary text-white w-64 flex-shrink-0 relative" v-if="showSidebar">
            <button @click="toggleSidebar"
                class="absolute top-1/2 transform -translate-y-1/2 right-0 mr-2 focus:outline-none">
                <svg xmlns="http://www.w3.org/2000/svg" class="h-6 w-6" fill="none" viewBox="0 0 24 24"
                    stroke="currentColor">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 19l-7-7 7-7" />
                </svg>
            </button>


            <!-- Sidebar Content -->
            <div class="p-4">
                <div class="flex items-center gap-2">
                    <h1 class="text-xl font-bold">{{ username }}</h1>
                    <div class="radial-progress text-secondary-content bg-accent drop-shadow-md"
                        :style="'--value:' + percentage + '; --size: 2rem; --thickness: 0.2rem; '" role="progressbar">
                        <div class="text-xs">{{ level }}</div>
                    </div>


                </div>
                <div class="divider"></div>
                <ul class="mt-4 flex flex-col justify-center">
                    <li class="mb-2" v-for="path in paths">
                        <nuxt-link :to="path.path" class="text-white text-xl">{{
            path.name }}</nuxt-link>

                    </li>


                </ul>
            </div>
            <nuxt-link to="/home"
                class="absolute bottom-0 mb-4 ml-4 text-2xl bg-transparent border-none flex items-center">
                <div class="imgLogout"></div>
            </nuxt-link>
            <nuxt-link to="/profil"
                class="absolute bottom-0 left-48 mb-4 ml-4 text-2xl bg-transparent border-none flex items-center">
                <div class="imgParameters"></div>
            </nuxt-link>
        </div>

        <!-- Main Content -->
        <div class="flex flex-col flex-1 overflow-auto">
            <!-- Header -->
            <div class="bg-primary shadow-md p-4 flex flex-row justify-between">
                <h1 class="text-2xl wimYellow font-bold">{{ club.name }}</h1>
                <div class="flex justify-end items-center">
                    <div class="text-xl wimYellow">Réservations disponibles : </div>
                    <div class="badge text-xl mx-2 py-4" :class="[claimsColor]">{{ availableClaimsNumber }}
                        / {{
            totalClaimsNumber }}</div>
                </div>
            </div>


            <!-- Page Content -->
            <div class="p-4">
                <slot></slot>
            </div>
            <div class="toast toast-right toast-top">
                <div v-for="notification in notifications" :class="notification.type"
                    @click.prevent="notifyStore.removeNotification(notification)">
                    <span>{{ notification.message }}</span>
                </div>
            </div>
        </div>
    </div>

</template>

<script setup>

import { useClubStore } from '~/stores/club'
import { useAuthStore } from '~/stores/auth'
import { useNotifyStore } from '~/stores/notify'
import { useLevelStore } from '~/stores/level'

const levelStore = useLevelStore()
const notifyStore = useNotifyStore()
const { notifications } = storeToRefs(notifyStore);

const clubStore = useClubStore()
const club = clubStore.getClub()
const authStore = useAuthStore()
const username = authStore.getUsername()
const isAdmin = authStore.getIsAdmin()
const totalClaimsNumber = 2;
const futurClaimsNumber = computed(() => authStore.getFuturClaimsNumber())

const level = levelStore.getLevel()
const percentage = levelStore.getPercentage()

const availableClaimsNumber = computed(() => {
    return totalClaimsNumber - futurClaimsNumber.value
})

const claimsColor = computed(() => {
    if (availableClaimsNumber.value === 0) {
        return 'badge-error'
    } else if (availableClaimsNumber.value === 1) {
        return 'badge-secondary'
    } else {
        return 'badge-secondary'
    }
})
const paths = [
    { name: '🗓️ Calendrier', path: '/calendar', admin: false },
    { name: '📋 Réservations', path: '/reservations', admin: false },
    { name: '🔓 Admin', path: '/admin', admin: true },
].filter(path => isAdmin || !path.admin)
</script>

<script>
export default {
    data() {
        return {
            showSidebar: true
        };
    },
    methods: {
        toggleSidebar() {
            this.showSidebar = !this.showSidebar;
        },
    }
};
</script>

<style>
.imgLogout {
    background-image: url('../../images/Logout.png');
    background-size: cover;
    width: 25px;
    height: 25px;
}

.imgParameters {
    background-image: url('../../images/parameters.png');
    background-size: cover;
    width: 25px;
    height: 25px;
}

.wimYellow {
    color: rgba(254, 237, 107, 1);
}
</style>
