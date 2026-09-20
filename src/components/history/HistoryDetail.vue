<script setup lang="ts">
import type { ChargingHistory, ChargingHistoryDetail } from '@/bindings'
import { Download, EllipsisVertical, Loader2, Trash2 } from '@lucide/vue'
import { save } from '@tauri-apps/plugin-dialog'
import { error as logerror } from '@tauri-apps/plugin-log'
import { format } from 'date-fns'
import { useI18n } from 'vue-i18n'
import { commands } from '@/bindings'
import CustomChartTooltip from '@/components/chart/CustomChartTooltip.vue'
import { useHistory } from '@/composables/useHistory'
import { formatChargingDuration } from '@/lib/format'

const props = defineProps<ChargingHistory>()
const { selectedItem, history } = useHistory()
const { t } = useI18n()
const displayNumber = (value: number | null) => value === null ? '—' : value.toFixed(1)

const historyCategories = computed(() => [
  t('history.curve_system_in'),
  t('history.curve_battery_power'),
  t('history.curve_system_load'),
  t('history.curve_battery_level'),
])

const isLoading = ref(true)
const error = ref()
const data = asyncComputed(
  () => commands.getDetailById(props.id)
    .then((r) => {
      if (r.status === 'error') {
        error.value = r.error
        logerror(r.error)
        return {} as ChargingHistoryDetail
      }
      return r.data
    }),
  {} as ChargingHistoryDetail,
  isLoading,
)

const historyCurveData = computed(() =>
  (data.value.curve ?? []).map(d => ({
    lastUpdate: new Date(d.lastUpdate * 1000).toLocaleTimeString(undefined, { hour12: false }),
    [t('history.curve_system_in')]: d.systemIn,
    [t('history.curve_battery_power')]: d.batteryPower,
    [t('history.curve_system_load')]: d.systemLoad,
    [t('history.curve_battery_level')]: d.absoluteBatteryLevel,
  })),
)

async function exportData() {
  const path = await save({
    title: t('history.export'),
    filters: [
      {
        name: 'JSON',
        extensions: ['json'],
      },
    ],
  })

  if (path) {
    const result = await commands.exportHistoryById(props.id, path)
    if (result.status === 'error') {
      error.value = result.error
      await logerror(result.error)
    }
  }
}
</script>

<template>
  <div class="h-full overflow-y-auto">
    <div v-if="isLoading" class="w-full h-full flex items-center justify-center">
      <Loader2 class="animate-spin" />
    </div>
    <div v-else-if="error" class="w-full h-full flex items-center justify-center text-red-500">
      {{ error }}
    </div>
    <div v-else class="px-6 pb-8">
      <div class="flex justify-between items-center">
        <div>
          <h1 class="text-2xl font-bold">
            {{ name || $t('history.unknown_device') }}
          </h1>
          <h2 class="text-sm font-bold mt-1 text-muted-foreground">
            {{ $t('history.with_adapter', { adapter: adapterName }) }}
          </h2>
        </div>
        <div>
          <DropdownMenu>
            <DropdownMenuTrigger class="p-2 rounded-md hover:bg-muted transition-colors">
              <EllipsisVertical class="w-4 h-4" />
            </DropdownMenuTrigger>
            <DropdownMenuContent
              :side-offset="10"
              align="end"
            >
              <DropdownMenuItem
                @click="exportData"
              >
                <Download class="w-4 h-4" />
                {{ $t('history.export') }}
              </DropdownMenuItem>
              <DropdownMenuSeparator />
              <DropdownMenuItem
                class="text-red-500 focus:text-red-500 focus:bg-red-500/10"
                @click="async () => {
                  const result = await commands.deleteHistoryById(id)
                  if (result.status === 'error') {
                    logerror(result.error)
                    return
                  }
                  selectedItem = null
                  history.update()
                }"
              >
                <Trash2 />
                {{ $t('history.delete') }}
              </DropdownMenuItem>
            </DropdownMenuContent>
          </DropdownMenu>
        </div>
      </div>
      <div class="mt-4 grid gap-4 grid-cols-3">
        <div class="space-y-2">
          <div class="text-sm font-medium text-muted-foreground">
            {{ $t('history.duration') }}
          </div>
          <div class="text-2xl font-bold">
            {{ formatChargingDuration(chargingTime, t) }}
          </div>
          <div class="text-xs text-muted-foreground">
            {{ format(timestamp * 1000, 'yyyy-MM-dd HH:mm') }}
          </div>
        </div>
        <div class="space-y-2">
          <div class="text-sm font-medium text-muted-foreground">
            {{ $t('history.average_power') }}
          </div>
          <div class="text-2xl font-bold">
            {{ displayNumber(data.avg.adapterPower) }}W
          </div>
          <div class="text-xs text-muted-foreground">
            {{ $t('history.peak') }}: {{ displayNumber(data.peak.adapterPower) }}W
          </div>
        </div>
        <div class="space-y-2">
          <div class="text-sm font-medium text-muted-foreground">
            {{ $t('history.charging_rate') }}
          </div>
          <div class="text-2xl font-bold">
            {{
              chargingTime > 0
                ? `${((endLevel - fromLevel) / chargingTime * 60).toFixed(2)}%/${$t('time.minute', 1)}`
                : '—'
            }}
          </div>
          <div class="text-xs text-muted-foreground">
            {{ $t('history.average_temperature') }}: {{ displayNumber(data.avg.temperature) }}°C
          </div>
        </div>
      </div>

      <h2 class="mt-8 font-bold">
        {{ $t('history.charging_curve') }}
      </h2>
      <LineChart
        class="mt-8 max-h-[220px]"
        index="lastUpdate"
        :data="historyCurveData"
        :categories="historyCategories"
        :custom-tooltip="CustomChartTooltip"
        :show-legend="false"
      />

      <h2 class="mt-8 font-bold">
        {{ $t('history.additional_detail') }}
      </h2>
      <div class="mt-2 grid gap-4 text-sm">
        <div class="grid grid-cols-2 gap-4">
          <div>
            <div class="text-muted-foreground">
              {{ $t('history.temperature_peak') }}
            </div>
            <div>{{ displayNumber(data.peak.temperature) }}°C</div>
          </div>
          <div>
            <div class="text-muted-foreground">
              {{ $t('history.adapter_power_peak') }}
            </div>
            <div>{{ displayNumber(data.peak.adapterPower) }}W</div>
          </div>
          <div>
            <div class="text-muted-foreground">
              {{ $t('history.adapter_watts') }}
            </div>
            <div>{{ data.peak.adapterWatts }}W({{ data.peak.adapterVoltage }}V, {{ data.peak.adapterAmperage }}A)</div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>
