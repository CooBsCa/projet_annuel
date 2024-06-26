import { defineStore } from "pinia";
export const useAuthStore = defineStore('auth', {
    state: () => ({
        token: ref(''),
        isAdmin: ref(false),
        username: ref(''),
        email: ref(''),
        futurClaimsNumber: ref(0),
        pastClaimsNumber: ref(0),
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
        setEmail(email) {
            this.email = email
        },
        getEmail() {
            return this.email
        },
        setUserId(userId) {
            this.userId = userId
        },
        getUserId() {
            return this.userId
        },
        setFuturClaimsNumber(futurClaimsNumber) {
            this.futurClaimsNumber = futurClaimsNumber
        },
        getFuturClaimsNumber() {
            return this.futurClaimsNumber
        },
        setPastClaimsNumber(pastClaimsNumber) {
            this.pastClaimsNumber = pastClaimsNumber
        },
        getPastClaimsNumber() {
            return this.pastClaimsNumber
        }
    },
    persist: {
        storage: persistedState.localStorage,
    },
})