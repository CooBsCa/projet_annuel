import { defineStore } from "pinia";
export const useAuthStore = defineStore('auth', {
    state: () => ({
        token: ref(''),
        isAdmin: ref(false),
        username: ref(''),
        futurClaimsNumber: ref(0),
    }),
    actions: {
        setToken(newToken) {
            this.token = newToken
        },
        getToken() {
            return this.token
        },
        setIsAdmin(isAdmin) {
            this.isAdmin = isAdmin
        },
        getIsAdmin() {
            return this.isAdmin
        },
        setUsername(username) {
            this.username = username
        },
        getUsername() {
            return this.username
        },
        setFuturClaimsNumber(futurClaimsNumber) {
            this.futurClaimsNumber = futurClaimsNumber
        },
        getFuturClaimsNumber() {
            return this.futurClaimsNumber
        },
    },
    persist: {
        storage: persistedState.localStorage,
    },
})