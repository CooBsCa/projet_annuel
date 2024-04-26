export const useClubStore = defineStore('club', {
    state: () => ({
        club: {}
    }),
    actions: {
        setClub(newClub) {
            this.club = newClub
        },
        getClub() {
            return this.club
        }
    }
})