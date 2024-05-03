<template>
    <div class="calendar">
      <div class="day">
        <div v-for="(zone, index) in zones" :key="index" class="court-column">
          <div class="court-name">{{ zone.name }}</div>
          <div
            v-for="(slot, slotIndex) in slots(zone)"
            :key="slotIndex"
            :class="{ 'slot': true, 'current-hour-slot': isCurrentHourSlot(slotIndex) }"
            @click="handleSlotClick(slot, court, slotIndex)"
            v-if="!isCurrentHourSlot(slotIndex)"
          >
            {{ slot }} <span v-if="isCurrentHourSlot(slotIndex)">"En cours"</span>
          </div>
          <!-- <div class="empty-slot" v-if="emptySlots.length> 0" v-for="n in emptySlots" :key="n"></div> -->
        </div>
      </div>
    </div>
  </template>

<script setup>
import { useAuthStore } from '~/stores/auth'
import { useClubStore } from '~/stores/club'
const authStore = useAuthStore()
const authToken = authStore.getToken()

const clubStore = useClubStore()
const club = clubStore.getClub()
const date = ref(new Date());
const zones = ref([]);
const endHour = '';

const getZones = async () => {
    const response = await apiGet('/zones/' + club.id, {
        headers: {
            Authorization: `Bearer ${authToken}`,
        },
    });
    const data = await response.json();
    console.log(data);
    zones.value = data;
};

const getAvailableSlots = async () => {
    const response = await apiGet('/slots', {
        body: JSON.stringify({
            date: date.value,
        }),
    });
    const data = await response.json();
};

const slots = (data) => {
    const startHour = new Date().getHours();
    const close_at = data.close_at.split(":")[0];
    const totalSlots = close_at - startHour;
    return Array.from({ length: totalSlots }, (_, index) => {
      const hour = startHour + index; // Heure du créneau
      return `${hour.toString().padStart(2, '0')}:00`;
    });
  };

const emptySlots = () => {
    const totalSlots = this.endHour - new Date().getHours();
    return Array.from({ length: 24 - this.endHour }, (_, index) => index + totalSlots);
  };

const handleSlotClick = (slot, court, slotIndex) => {
    console.log(`Créneau ${slot} cliqué pour le court ${court}`);
  };

const isCurrentHourSlot = (slotIndex) => {
    return Math.floor(slotIndex === 0);
  };

getZones()
</script>
  <!-- <script>
  export default {
    data() {
      return {
        courts: getZones(),
        endHour: 22,
        slotSize: 60, // Taille de la case en pixels
      };
    },
    computed: {
      availableSlots() {
        const startHour = new Date().getHours(); // Heure actuelle
        const totalSlots = this.endHour - startHour; // Nombre total de créneaux
        return Array.from({ length: totalSlots }, (_, index) => {
          const hour = startHour + index; // Heure du créneau
          return `${hour.toString().padStart(2, '0')}:00`;
        });
      },
      emptySlots() {
        const totalSlots = this.endHour - new Date().getHours();
        return Array.from({ length: 24 - this.endHour }, (_, index) => index + totalSlots);
      },
    },
    methods: {
      handleSlotClick(slot, court, slotIndex) {
        // Gérer l'événement de clic sur le créneau
        console.log(`Créneau ${slot} cliqué pour le court ${court}`);
      },
      isCurrentHourSlot(slotIndex) {
        return Math.floor(slotIndex === 0);
        //return Math.floor(slotIndex / 60) === currentHour || Math.floor(slotIndex / 60) === currentHour - 1;
      },
      async getZones(){
            const response = await apiGet('/zones/' + club.id, {
                headers: {
                    Authorization: `Bearer ${authToken}`,
                },
            });
            const data = await response.json();
            console.log(data);
            zones.value = data;
        },
    },
  };
  </script> -->
  
  <style scoped>
  .calendar {
    display: flex;
    flex-direction: column;
    height: 100vh;
    width: 100%;
  }
  
  .day {
    display: flex;
    flex: 1;
  }
  
  .court-column {
    flex: 1;
    display: flex;
    flex-direction: column;
    border-left: 1px solid #ccc;
    border-right: 1px solid #ccc;
  }
  
  .court-name {
    padding: 5px;
    font-weight: bold;
    text-align: center;
  }
  
  .slot {
    border-bottom: 1px solid #ccc;
    cursor: pointer;
    height: 60px; /* Taille de la case */
    display: flex;
    align-items: center;
    justify-content: center;
  }
  
  .empty-slot {
    height: 60px; /* Taille de la case */
  }
  
  .slot:hover {
    background-color: #f0f0f0;
  }
  
  .current-hour-slot {
    background-color: #ccc;
    pointer-events: none; /* Désactiver les événements de souris */
  }
  </style>
  