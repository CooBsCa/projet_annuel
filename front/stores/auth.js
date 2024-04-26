
export const useAuthStore = defineStore('auth', {
    state: () => ({
        token: ''
    }),
    actions: {
        setToken(newToken) {
            this.token = newToken
        },
        getToken() {
            return this.token
        }
    }
})