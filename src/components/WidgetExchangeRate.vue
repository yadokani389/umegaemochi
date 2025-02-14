<script setup lang="ts">
import { ref } from 'vue'
import { invoke } from "@tauri-apps/api/core";

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
      currencyList.value.push("1 " + currencyName + ": " + price.toFixed(2) + "円");
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

<!-- 離席中 -->

<style module>
.container {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  height: 100%;
  background-color: #e8f5e9;
  color: #2e7d32;
  font-family: Arial, sans-serif;
  border-radius: 5px;
}

.header {
  font-size: 1.8rem;
  font-weight: bold;
  margin-bottom: 10px;
  margin-top: 10px;
}

.exchanges {
  display: flex;
  flex-wrap: wrap;
  justify-content: center;
  gap: 8px;
  max-width: 80%;
  overflow-y: auto;
  padding: 10px;
  background: #ffffff;
  border-radius: 8px;
  box-shadow: 2px 2px 10px rgba(0, 0, 0, 0.1);
}

.card {
  white-space: nowrap;
  position: relative;
  height: 15%;
  width: 100%;
}

.card .div {
  position: absolute;
  transition: transform 0.5s;
  align-items: center;
  top: 0;
  left: 0;
}

.cardInner {
  width: 100%;
  height: 100%;
  position: relative;
  transform-style: preserve-3d;
  transition: transform 1s ease-in-out;
}

.card .unit {
  align-items: center;
  padding: 8px 12px;
  background: #c8e6c9;
  border-radius: 5px;
  font-size: 1rem;
  text-align: center;
  backface-visibility: hidden;
}

.flipped {
  transform: rotateY(180deg);
}
</style>