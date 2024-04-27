import { defineStore } from "pinia";
export const useAuthStore = defineStore('auth', {
    state: () => ({
        token: ref(''),
        isAdmin: ref(false),
        username: ref(''),
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
    },
    persist: {
        storage: persistedState.localStorage,
    },
})