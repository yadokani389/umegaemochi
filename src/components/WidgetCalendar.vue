<script setup lang="ts">

const weeks = ['Sun', 'Mon', 'Tue', 'Wed', 'Thu', 'Fri', 'Sat'];
const todayDate = new Date();
const currentYear = todayDate.getFullYear();
const currentMonth = todayDate.getMonth();

const formattedMonthYear = (() => {
  const options: Intl.DateTimeFormatOptions = { month: 'short', year: 'numeric' };
  return todayDate.toLocaleDateString(undefined, options);
})();

const firstDayOfMonth = new Date(currentYear, currentMonth, 1);
const lastDayOfMonth = new Date(currentYear, currentMonth + 1, 0);

const lastDateOfPrevMonth = new Date(currentYear, currentMonth, 0).getDate();

const isSameDate = (date1: Date, date2: Date): boolean => {
  return (
    date1.getFullYear() === date2.getFullYear() &&
    date1.getMonth() === date2.getMonth() &&
    date1.getDate() === date2.getDate()
  );
}

const weeksInMonth = (() => {
  const weeksArray: { date: Date; isToday: boolean; isCurrentMonth: boolean }[][] = [];
  let week: { date: Date; isToday: boolean; isCurrentMonth: boolean }[] = [];

  const startDay = firstDayOfMonth.getDay();

  for (let i = startDay; i > 0; i--) {
    const date = new Date(currentYear, currentMonth - 1, lastDateOfPrevMonth - i + 1);
    week.push({
      date,
      isToday: isSameDate(date, todayDate),
      isCurrentMonth: false
    });
  }

  for (let i = 1; i <= lastDayOfMonth.getDate(); i++) {
    const date = new Date(currentYear, currentMonth, i)
    week.push({
      date,
      isToday: isSameDate(date, todayDate),
      isCurrentMonth: true
    });

    if (week.length === 7) {
      weeksArray.push(week);
      week = [];
    };
  }

  if (week.length > 0) {
    for (let i = 1; week.length < 7; i++) {
      const date = new Date(currentYear, currentMonth + 1, i);
      week.push({
        date,
        isToday: isSameDate(date, todayDate),
        isCurrentMonth: false
      });
    }
    weeksArray.push(week);
  }

  return weeksArray;
})()
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
  height: 100%;
  width: 100%;
  margin: auto;
}

.header {
  text-align: center;
  margin-bottom: 10px;
  font-size: small;
}

table {
  width: 95%;
  border-collapse: collapse;
}

th {
  background-color: #f0f0f0;
  font-size: small;
}

td {
  font-size: medium;
  width: 14.28%;
  text-align: center;
  vertical-align: middle;
  box-sizing: border-box;
  padding: 1.5%;
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