import { defineStore } from "pinia";

export const useLevelStore = defineStore('level', {
    state: () => ({
        pastClaimsNumber: ref(0),
        level: ref(0),
        percentage: ref(0),
        slotBeforeNextLevel: ref(0),
    }),
    actions: {
        setPastClaimsNumber(pastClaimsNumber) {
            this.pastClaimsNumber = pastClaimsNumber
            this.level = Math.floor(pastClaimsNumber / 10)
            this.percentage = (pastClaimsNumber % 10) * 10
            this.slotBeforeNextLevel = 10 - (pastClaimsNumber % 10)
        },
        getPastClaimsNumber() {
            return this.pastClaimsNumber
        },
        getLevel() {
            return this.level
        },
        getPercentage() {
            return this.percentage
        },
        getSlotBeforeNextLevel() {
            return this.slotBeforeNextLevel
        }
    },
    persist: {
        storage: persistedState.localStorage,
    },
})