<script setup lang="ts">
import { ref, computed } from 'vue';
import { listen } from '@tauri-apps/api/event';

const model = defineModel();

const weeks = ['Sun', 'Mon', 'Tue', 'Wed', 'Thu', 'Fri', 'Sat'];
const todayDate = ref(new Date());

const currentYear = computed(() => todayDate.value.getFullYear());
const currentMonth = computed(() => todayDate.value.getMonth());

const formattedMonthYear = computed(() => formatMonthYear(todayDate.value));

function formatMonthYear(date: Date) {
  const options: Intl.DateTimeFormatOptions = { month: 'short', year: 'numeric' };
  return date.toLocaleDateString(undefined, options);
}

const firstDayOfMonth = computed(() => new Date(currentYear.value, currentMonth.value, 1));
const lastDayOfMonth = computed(() => new Date(currentYear.value, currentMonth.value + 1, 0));
const lastDateOfPrevMonth = computed(() => new Date(currentYear.value, currentMonth.value, 0).getDate());

const isSameDate = (date1: Date, date2: Date): boolean => {
  return (
    date1.getFullYear() === date2.getFullYear() &&
    date1.getMonth() === date2.getMonth() &&
    date1.getDate() === date2.getDate()
  );
}

const weeksInMonth = computed(() => calculateWeeksInMonth());

function calculateWeeksInMonth() {
  const weeksArray: { date: Date; isToday: boolean; isCurrentMonth: boolean }[][] = [];
  let week: { date: Date; isToday: boolean; isCurrentMonth: boolean }[] = [];

  const startDay = firstDayOfMonth.value.getDay();

  for (let i = startDay; i > 0; i--) {
    const date = new Date(currentYear.value, currentMonth.value - 1, lastDateOfPrevMonth.value - i + 1);
    week.push({
      date,
      isToday: isSameDate(date, todayDate.value),
      isCurrentMonth: false
    });
  }

  for (let i = 1; i <= lastDayOfMonth.value.getDate(); i++) {
    const date = new Date(currentYear.value, currentMonth.value, i)
    week.push({
      date,
      isToday: isSameDate(date, todayDate.value),
      isCurrentMonth: true
    });

    if (week.length === 7) {
      weeksArray.push(week);
      week = [];
    };
  }

  if (week.length > 0) {
    for (let i = 1; week.length < 7; i++) {
      const date = new Date(currentYear.value, currentMonth.value + 1, i);
      week.push({
        date,
        isToday: isSameDate(date, todayDate.value),
        isCurrentMonth: false
      });
    }
    weeksArray.push(week);
  }

  return weeksArray;
}

model.value = '/picto/Rain.png';

listen("daily_reload", async () => {
  todayDate.value = new Date();
});
</script>

<template>
  <div :class="$style.calendar">
    <div :class="$style.header">
      <h2>{{ formattedMonthYear }}</h2>
    </div>
    <table>
      <thead>
        <tr>
          <th v-for="week in weeks" :key="week">{{ week }}</th>
        </tr>
      </thead>
      <tbody>
        <tr v-for="(week, index) in weeksInMonth" :key="index">
          <td v-for="day in week" :key="day.date.getTime()" :class="{
            [$style.today]: day.isToday,
            [$style.sunday]: day.date.getDay() === 0,
            [$style.saturday]: day.date.getDay() === 6,
            [$style.isCurrentMonth]: day.isCurrentMonth
          }">
            {{ day.date.getDate() }}
          </td>
        </tr>
      </tbody>
    </table>
  </div>
</template>

<style module>
.calendar {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  height: 100%;
  width: 100%;
  margin: auto;
}

.header {
  text-align: center;
  margin-bottom: 5%;
}

.header h2 {
  font-size: 6vmin;
}

table {
  width: 95%;
  border-collapse: collapse;
}

th {
  background-color: #f0f0f0;
  font-size: 3vmin;
}

td {
  font-size: 3vmin;
  width: 14.28%;
  text-align: center;
  vertical-align: middle;
  box-sizing: border-box;
  padding: 2.25%;
  border: 1px solid #ddd;
}

td.today {
  background-color: #ffeb3b;
  font-weight: bold;
}


td.isCurrentMonth {
  color: #333;
}

td.sunday {
  color: red;
}

td.saturday {
  color: blue;
}

td:not(.isCurrentMonth) {
  color: #aaa;
}
</style>
