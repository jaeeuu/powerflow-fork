<script setup lang="ts">
import { defineAsyncComponent } from 'vue'

const tab = useTab()
const BatteryHealthChart = defineAsyncComponent(() => import('@/components/chart/BatteryHealthChart.vue'))
const PowerUsageChart = defineAsyncComponent(() =>
  import('@/components/chart/PowerUsageChart.vue'),
)
</script>

<template>
  <div class="flex flex-col gap-4 min-w-min pb-2 px-4 pt-2">
    <div class="flex gap-6">
      <PowerStatus />
      <PowerFlow />
    </div>
    <Suspense>
      <PowerUsageChart />
      <template #fallback>
        <Skeleton class="w-full h-[300px]" />
      </template>
    </Suspense>
    <TechnicalDetail />
    <BatteryHealthChart v-if="tab === 'local'" />
    <ProcessEnergyList v-if="tab === 'local'" />
  </div>
</template>
