<template data-theme="wimbledon">
  <div class="navbar">
    <div class="flex-1">
      <div class="text-bl text-lg font-bold">Terrains disponibles</div>
    </div>
    <div class="flex-none">
      <button @click="decrementDate" class="block left-0 m-3 text-2xl bg-transparent border-none cursor-pointer">
        <div class="img2" v-if="isPreviousAvailable()"></div>
      </button>
      <input id="date" type="date"
        class="block p-3 text-lg border border-gray-300 rounded-md text-gray-700 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-blue-500 hover:border-blue-400 mb-4"
        v-model="formattedDate" />
      <button @click="incrementDate" class="block right-0 m-3 text-2xl bg-transparent border-none cursor-pointer">
        <div class="img1"></div>
      </button>
    </div>
  </div>

  <div class="calendar">
    <div class="day">
      <div v-for="( zone, index ) in   zones  " :key="index" class="court-column">
        <div class="court-name">{{ zone.name }}</div>
        <span class="text-center" v-if="slots(zone).length == 0">Aucun créneau disponible</span>
        <div v-for="( slot, slotIndex ) in   slots(zone)  " :key="slotIndex"
          :class="{ 'slot': true, 'current-hour-slot': slot.isCurrentHour, 'slotReserved text-white pointer-events-none': slot.isReserved, 'slotReservedNotParticipant': !slot.isParticipant && slot.isReserved }"
          @click="handleSlotClick(slot,
        zone, index, slotIndex)" :style="{ height: slotHeight(slot.slot_duration) + 'px' }">
          <span v-if="!slot.isCurrentHour && !slot.isReserved">{{ slot.start_at }} </span>
          <span v-if="slot.isCurrentHour && !slot.isReserved">🚫 {{ slot.start_at }} 🚫</span>
          <span v-if="slot.isReserved">{{ slot.user_name + " - " +
        slot.opponent_user_name }}</span>
        </div>
        <div class="court-name">{{ zone.name }}</div>
        <div class="empty-slot" v-if="emptySlots.length > 0" v-for="  n   in   emptySlots  " :key="n"></div>
      </div>
    </div>
  </div>

  <CreateReservation :selectedSchedule="selectedSchedule" @submit="onReservationSubmit">
  </CreateReservation>

  <Modal ref="alertModal" :showCancel="false">
    <h2 class="text-black text-2xl font-bold mb-4">{{ popUpAlertParams.title }}</h2>
    <p class="text-black pb-5">{{ popUpAlertParams.text }}</p>
  </Modal>
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

const popUpAlertParams = ref({
  title: " ⛔️ Réservation Impossible",
  text: "Vous avez atteint le nombre maximum de réservations",
});

const currentDate = new Date();
const dateToday = currentDate.toISOString().split('T')[0];
const formattedDate = ref(dateToday);
const slotIndex = ref(0);
const clubStore = useClubStore()
const club = clubStore.getClub()
const date = ref(new Date());
const zones = ref([]);
const reservedSlots = ref([]);
const user_name = authStore.getUsername();
const totalClaimsNumber = 2;
const futurClaimsNumber = authStore.getFuturClaimsNumber()
const availableClaimsNumber = ref(totalClaimsNumber - futurClaimsNumber)
const alertModal = ref()

const capitalizeFirstLetter = (string) => {
  return string.charAt(0).toUpperCase() + string.slice(1).toLowerCase();
};

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
  let slots = Array.from({ length: totalSlots }, (_, index) => {
    const hour = parseInt(startHour) + (index * slot_duration);
    const end_at = parseInt(hour) + parseInt(slot_duration);
    return {
      start_at: `${hour.toString().padStart(2, '0')}:00`,
      end_at: `${end_at}:00`,
      slot_duration
    };
  });

  for (let i = 0; i < slots.length; i++) {
    let matchingSlot = findMatching(slots[i], data);
    if (matchingSlot) {
      if (matchingSlot.user_name == user_name || matchingSlot.opponent_user_name == user_name) {
        slots[i].isParticipant = true;

      }
      slots[i].isReserved = true;
      slots[i].user_name = capitalizeFirstLetter(matchingSlot.user_name);
      slots[i].opponent_user_name = capitalizeFirstLetter(matchingSlot.opponent_user_name);
    }
  }

  for (let i = 0; i < slots.length; i++) {
    if (isCurrentHourSlot(i)) {
      slots[i].isCurrentHour = true;
    }
  }

  return slots;
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
  if (availableClaimsNumber.value === 0) {
    showAlertModal()
    return;
  }
  console.log(`Créneau ${slot} cliqué pour le court ${court}`);
  selectedSchedule.value.zone_id = court.id;
  selectedSchedule.value.zone = court.name;
  selectedSchedule.value.start_at = slot.start_at;
  selectedSchedule.value.end_at = slot.end_at;
  selectedSchedule.value.day = formattedDate.value;
  CreateReservationModal.showModal()
};

const isCurrentHourSlot = (slotIndex) => {
  const calendarDate = new Date(date.value).toISOString().split('T')[0];
  const dateOfTheDay = new Date().toISOString().split('T')[0];
  if (calendarDate == dateOfTheDay) {
    return Math.floor(slotIndex === 0);
  }
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
  getAllClaimedSlots();
  getZones();
};

const decrementDate = () => {
  const newDate = new Date(date.value);
  newDate.setDate(newDate.getDate() - 1);
  date.value = newDate;
  formattedDate.value = newDate.toISOString().split('T')[0];
  getAllClaimedSlots();
  getZones();
};

const showAlertModal = () => {
  alertModal.value.show()
}

const isPreviousAvailable = () => {
  return new Date(date.value) > new Date();
}

const formatDate = (date, hour, minute, seconds) => {
  const split = date.split('-');
  const year = String(split[0]);
  const month = String(split[1]).padStart(2, '0');
  const day = String(split[2]).padStart(2, '0');
  return `${year}-${month}-${day}T${hour}:${minute}:${seconds}`;
};

const getAllClaimedSlots = async () => {
  try {
    const formattedStartAt = formatDate(formattedDate.value, '00', '00', '00');
    const formattedEndAt = formatDate(formattedDate.value, '23', '59', '59');
    const response = await apiPost("/claimed-slots-by-day", {
      body: JSON.stringify({
        start_of_day: formattedStartAt,
        end_of_day: formattedEndAt,
      }),
    });
    const data = await response.json();
    console.log("claimed slots :")
    console.log(data);
    reservedSlots.value = data;
  } catch (err) {
    console.error(err)
  }
}

const findMatching = (slot, zone) => {
  return reservedSlots.value.find((reservedSlot) => {
    const slot_start_split = slot.start_at.split(":")
    return (reservedSlot.start_at === formatDate(formattedDate.value, slot_start_split[0], slot_start_split[1], '00') && reservedSlot.zone_id === zone.id);
  });
}

const onReservationSubmit = () => {
  getZones();
  getAllClaimedSlots();
}

getAllClaimedSlots()
getZones()
</script>

<style scoped>
.calendar {
  overflow: scroll;
  display: flex;
  flex-direction: column;
  min-width: 100%;
}

.slotReserved {
  background-color: rgba(58, 11, 125, 0.9);
}

.slotReservedNotParticipant {
  background-color: rgba(103, 79, 137, 0.5);
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

.img1 {
  background-image: url('../../images/fleche_droite.png');
  background-size: cover;
  width: 50px;
  height: 50px;
}

.img2 {
  background-image: url('../../images/fleche_gauche.png');
  background-size: cover;
  width: 50px;
  height: 50px;
}
</style>