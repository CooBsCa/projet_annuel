<template>
  <nav class="navbar text-right bg-wimbledon-purple">
    <button @click="decrementDate" class="arrow-button">&#9664;</button>
    <input id="date" type="date"
      class="block p-3 text-lg border border-gray-300 rounded-md text-gray-700 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-blue-500 hover:border-blue-400 mb-4"
      v-model="formattedDate" />
    <button @click="incrementDate" class="arrow-button">&#9654;</button>
  </nav>
  <div class="calendar">
    <div class="day">
      <div v-for="(zone, index) in zones" :key="index" class="court-column">
        <div class="court-name">{{ zone.name }}</div>
        <span class="text-center" v-if="slots(zone).length == 0">Aucun créneau disponible</span>
        <div v-for="(slot, slotIndex) in slots(zone)" :key="slotIndex"
          :class="{ 'slot': true, 'current-hour-slot': isCurrentHourSlot(slotIndex) }" @click="handleSlotClick(slot,
      zone, index, slotIndex)" :style="{ height: slotHeight(slot.slot_duration) + 'px' }">
          <span v-if="!isCurrentHourSlot(slotIndex)">{{ slot.start_at }} </span>
          <span v-if="isCurrentHourSlot(slotIndex)">🚫 {{ slot.start_at }} 🚫</span>
        </div>
        <div class="court-name">{{ zone.name }}</div>
        <div class="empty-slot" v-if="emptySlots.length > 0" v-for="n in emptySlots" :key="n"></div>
      </div>
    </div>
  </div>

  <CreateReservation :selectedSchedule="selectedSchedule">
  </CreateReservation>
</template>

<script setup>
import { useAuthStore } from '~/stores/auth'
import { useClubStore } from '~/stores/club'
const authStore = useAuthStore()
const authToken = authStore.getToken()
const selectedSchedule = ref({
  zone_id: '',
  start_at: '',
  end_at: '',
  zone: '',
  day: '',
});

const currentDate = new Date();
const dateToday = currentDate.toISOString().split('T')[0];
const formattedDate = ref(dateToday);
const slotIndex = ref(0);
const clubStore = useClubStore()
const club = clubStore.getClub()
const date = ref(new Date());
const zones = ref([]);

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
  let startHour = '';
  const calendarDate = new Date(date.value).toISOString().split('T')[0];
  const dateOfTheDay = new Date().toISOString().split('T')[0];
  if (calendarDate != dateOfTheDay) {
    startHour = data.open_at.split(":")[0];
  } else {
    if (data.open_at.split(":")[0] < new Date().getHours()) {
      startHour = new Date().getHours();
    } else {
      startHour = data.open_at.split(":")[0];
    }
  }


  let close_at = data.close_at.split(":")[0];
  if (close_at == '00') {
    close_at = '24';
  }

  const slot_duration = (euclideanDivision(data.reservation_time, 60)[0]);
  const totalSlots = euclideanDivision((parseInt(close_at) - parseInt(startHour)), slot_duration)[0];
  return Array.from({ length: totalSlots }, (_, index) => {
    const hour = parseInt(startHour) + (index * slot_duration);
    const end_at = parseInt(hour) + parseInt(slot_duration);
    return {
      start_at: `${hour.toString().padStart(2, '0')}:00`,
      end_at: `${end_at}:00`,
      slot_duration
    };
  });
};

const slotHeight = (slot_duration) => {
  const baseHeight = 60;
  const hourHeight = baseHeight * 60;
  return hourHeight / 60;
};

const emptySlots = () => {
  const totalSlots = this.endHour - new Date().getHours();
  return Array.from({ length: 24 - this.endHour }, (_, index) => index + totalSlots);
};

const handleSlotClick = (slot, court, slotIndex) => {
  console.log(`Créneau ${slot} cliqué pour le court ${court}`);
  selectedSchedule.value.zone_id = court.id;
  selectedSchedule.value.zone = court.name;
  selectedSchedule.value.start_at = slot.start_at;
  selectedSchedule.value.end_at = slot.end_at;
  selectedSchedule.value.day = formattedDate.value;
  CreateReservationModal.showModal();
};

const isCurrentHourSlot = (slotIndex) => {
  return Math.floor(slotIndex === 0);
};

const euclideanDivision = (dividend, divisor) => {
  var quotient = Math.floor(dividend / divisor);
  var remainder = dividend % divisor;
  return [quotient, remainder];
}

const incrementDate = () => {
  const newDate = new Date(date.value);
  newDate.setDate(newDate.getDate() + 1);
  date.value = newDate;
  formattedDate.value = newDate.toISOString().split('T')[0];
  getZones();
};

const decrementDate = () => {
  const newDate = new Date(date.value);
  newDate.setDate(newDate.getDate() - 1);
  date.value = newDate;
  formattedDate.value = newDate.toISOString().split('T')[0];
  getZones();
};

getZones()
</script>

<style scoped>
.calendar {
  overflow: scroll;
  display: flex;
  flex-direction: column;
  min-width: 100%;
}

.day {
  display: flex;
  width: 1500px;
  height: 100%;
  flex: 1
}

.court-column {
  overflow: hidden;
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
  /* height: 60px; */
  display: flex;
  align-items: center;
  justify-content: center;
}

.empty-slot {
  height: 60px;
}

.slot:hover {
  background-color: #f0f0f0;
}

.current-hour-slot {
  background-color: #ccc;
  pointer-events: none;
}
</style>