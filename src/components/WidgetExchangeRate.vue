<script setup lang="ts">
import { ref, watch } from 'vue'
import { invoke } from "@tauri-apps/api/core";
import { listen } from '@tauri-apps/api/event';

const currencyMap = [
  { code: "USD", name: "アメリカドル" },
  { code: "EUR", name: "ユーロ" },
  { code: "RUB", name: "ロシアルーブル" },
  { code: "GBP", name: "イギリスポンド" },
  { code: "KRW", name: "韓国ウォン" },
  { code: "CNY", name: "中国人民元" },
  { code: "AUD", name: "オーストラリアドル" },
  { code: "THB", name: "タイバーツ" },
  { code: "VND", name: "ベトナムドン" },
  { code: "TRY", name: "トルコリラ" },
];

const currencyList = ref<string[]>([]);
const firstList = ref<string[]>([]);
const secondList = ref<string[]>([]);
const isFlipped = ref(false);

const getExchangeRate = async () => {
  currencyList.value = [];
  const response = await invoke<{ [key: string]: number }>("get_exchange_rate");
  for (const { code: currency, name: currencyName } of currencyMap) {
    if (response[currency] !== undefined) {
      let price = response[currency];
      price = 1 / price;
      currencyList.value.push(`1${currencyName}: ${price.toFixed(2)}円`);
    }
  }
  firstList.value = currencyList.value.slice(0, 5);
  secondList.value = currencyList.value.slice(5, 10);

  console.log(firstList.value);
  console.log(secondList.value);
};

const flip = () => {
  isFlipped.value = !isFlipped.value;
};

getExchangeRate();
setInterval(flip, 5000);

listen("daily_reload", async () => {
  getExchangeRate();
});

const { widgetName } = defineProps<{ widgetName: string; }>();
const model = defineModel();
watch(() => widgetName, () => {
  if (widgetName === 'WidgetExchangeRate') {
    model.value = '/picto/gorogoro.gif';
  }
});
</script>

<template>
  <div :class="$style.container">
    <h1 :class="$style.header">為替レート</h1>
    <div :class="$style.exchanges">
      <div v-for="currency, index in firstList" :key="currency" :class="$style.card">
        <div :class="[$style.cardInner, { [$style.flipped]: isFlipped }]">
          <div :class="$style.cardFront">
            <div :class="$style.unit">{{ currency }}</div>
          </div>
          <div :class="$style.cardBack">
            <div :class="$style.unit">{{ secondList[index] }}</div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<style module>
.container {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  height: 100%;
  padding: 5%;
  background-color: #e8f5e9;
  color: #2e7d32;
  font-family: Arial, sans-serif;
  border-radius: 5px;
}

.header {
  font-weight: bold;
  margin-bottom: 10px;
}

.exchanges {
  display: flex;
  flex-wrap: wrap;
  justify-content: center;
  gap: 8px;
  width: 80%;
  height: 100%;
  overflow-y: auto;
  padding: 10px;
  background: #ffffff;
  border-radius: 8px;
}

.card {
  white-space: nowrap;
  position: relative;
  height: 15%;
  width: 100%;
  margin: 5px;
}
.cardInner {
  width: 100%;
  height: 100%;
  position: relative;
  transform-style: preserve-3d;
  transition: transform 1s ease-in-out;
}

.cardFront {
  position: absolute;
  top: 0;
  left: 0;
  width: 100%;
  height: 100%;
  display: flex;
  align-items: center;
  justify-content: center;
  backface-visibility: hidden;
}

.cardBack {
  position: absolute;
  top: 0;
  left: 0;
  width: 100%;
  height: 100%;
  display: flex;
  align-items: center;
  justify-content: center;
  backface-visibility: hidden;
  transform: rotateY(180deg);
}

.unit {
  align-items: center;
  padding: 1.5%;
  background: #c8e6c9;
  border-radius: 5px;
  text-align: center;
  max-width: 100%;
  font-size: 2.5vw;
}

.flipped {
  transform: rotateY(180deg);
}
</style>
