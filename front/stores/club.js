import { defineStore } from "pinia";
export const useClubStore = defineStore('club', {
    state: () => ({
        club: ref({})
    }),
    actions: {
        setClub(newClub) {
            this.club = newClub
        },
        getClub() {
            return this.club
        }
    },
    persist: {
        storage: persistedState.localStorage,
    },
})