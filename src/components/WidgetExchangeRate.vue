<script setup lang="ts">
import { ref } from 'vue'
import { invoke } from "@tauri-apps/api/core";

const currencyMap = new Map([
  ["JPY", "日本円"],
  ["IDR", "インドネシアルピア"],
  ["INR", "インドルピー"],
  ["SGD", "シンガポールドル"],
  ["THB", "タイバーツ"],
  ["BDT", "バングラデシュタカ"],
  ["PHP", "フィリピンペソ"],
  ["VND", "ベトナムドン"],
  ["MYR", "マレーシアリンギット"],
  ["KRW", "韓国ウォン"],
  ["CNY", "中国人民元"],
  ["AUD", "オーストラリアドル"],
  ["NZD", "ニュージーランドドル"],
  ["CAD", "カナダドル"],
  ["MXN", "メキシコペソ"],
  ["USD", "アメリカドル"],
  ["BRL", "ブラジルレアル"],
  ["GBP", "イギリスポンド"],
  ["TRY", "トルコリラ"],
  ["EUR", "ユーロ"],
  ["RUB", "ロシアルーブル"],
]);

const currencyList = ref<string[]>([]);

const getExchangeRate = async () => {
  currencyList.value = [];
  const response = await invoke<Map<string, number>>("get_exchange_rate");
  for (const [currency, rate] of Object.entries(response)) {
    if (currencyMap.has(currency)) {
      const currencyName = currencyMap.get(currency);
      var price = 1 / rate;
      currencyList.value.push("１" + currencyName + ": " + price.toFixed(2) + "円");
    }
  }
};

getExchangeRate();
</script>

<template>
  <div :class="$style.container">
    <div :class="$style.header">
      <h1>為替レート</h1>
    </div>
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
  height: 80%;
}

.header {
  align-items: center;
  justify-content: center;
  font-family: '';
}

.exchanges {
  display: flex;
  flex-direction: column;
  align-items: left;
}
</style>