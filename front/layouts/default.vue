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
        <div class="bg-gray-800 text-white w-64 flex-shrink-0 relative" v-if="showSidebar">
            <button @click="toggleSidebar"
                class="absolute top-1/2 transform -translate-y-1/2 right-0 mr-2 focus:outline-none">
                <svg xmlns="http://www.w3.org/2000/svg" class="h-6 w-6" fill="none" viewBox="0 0 24 24"
                    stroke="currentColor">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 19l-7-7 7-7" />
                </svg>
            </button>


            <!-- Sidebar Content -->
            <div class="p-4">
                <h1 class="text-xl font-bold">{{ username }}</h1>
                <ul class="mt-4">
                    <li class="mb-2">
                        <nuxt-link to="/calendar" class="text-white">Calendrier</nuxt-link>
                    </li>
                    <li class="mb-2">
                        <nuxt-link to="/admin/club" class="text-white">Admin</nuxt-link>
                    </li>
                </ul>
            </div>
        </div>

        <!-- Main Content -->
        <div class=" flex flex-col flex-1">
            <!-- Header -->
            <div class="bg-white shadow-md p-4">
                <h1 class="text-2xl font-bold">{{ club.name }}</h1>
            </div>

            <!-- Page Content -->
            <div class="p-4">
                <slot></slot>
            </div>
        </div>
    </div>

</template>

<script setup>
import { ref } from 'vue'
import { useClubStore } from '~/stores/club'
import { useAuthStore } from '~/stores/auth'
const clubStore = useClubStore()
const club = clubStore.getClub()
const authStore = useAuthStore()
const username = authStore.getUsername()
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
