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

const getExchangeRate = async () => {
  currencyList.value = [];
  const response = await invoke<{ [key: string]: number }>("get_exchange_rate");
  for (const { code: currency, name: currencyName } of currencyMap) {
    if (response[currency] !== undefined) {
      let price = response[currency];
      price = 1 / price;
      currencyList.value.push(" 1 " + currencyName + ": " + price.toFixed(2) + "円");
    }
  }
};

getExchangeRate();
</script>

<template>
  <div :class="$style.container">
    <h1 :class="$style.header">為替レート</h1>
    <div :class="$style.exchanges">
      <div v-for="currency in currencyList" :key="currency">{{ currency }}</div>
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
  background-color: #e8f5e9;
  color: #2e7d32;
  font-family: Arial, sans-serif;
  border-radius: 5px;
}

.header {
  font-size: 1.8rem;
  font-weight: bold;
  margin-bottom: 10px;
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
  max-height: 50vh;
}

.exchanges div {
  padding: 8px 12px;
  background: #c8e6c9;
  border-radius: 5px;
  font-size: 1rem;
  min-width: 80px;
  text-align: center;
}
</style>