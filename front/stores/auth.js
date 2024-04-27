import { defineStore } from "pinia";
export const useAuthStore = defineStore('auth', {
    state: () => ({
        token: ref('')
    }),
    actions: {
        setToken(newToken) {
            this.token = newToken
        },
        getToken() {
            return this.token
        }
    },
    persist: {
        storage: persistedState.localStorage,
    },
})