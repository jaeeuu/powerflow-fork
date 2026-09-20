import type { NormalizedData, NormalizedResource } from '@/bindings'

export function selectDeviceValue<T>(
  local: T,
  remote: Record<string, T>,
  tab: string,
  fallback: T,
): T {
  if (tab === 'local')
    return local
  return remote[tab] ?? fallback
}

export type DisplayPower = Omit<NormalizedResource, keyof NormalizedData> & {
  [Key in keyof NormalizedData]: number
}

export function normalizePowerData(data: NormalizedResource): DisplayPower {
  const finite = (value: number | null) => typeof value === 'number' && Number.isFinite(value) ? value : 0
  return {
    ...data,
    systemIn: finite(data.systemIn),
    systemLoad: finite(data.systemLoad),
    batteryPower: finite(data.batteryPower),
    adapterPower: finite(data.adapterPower),
    efficiencyLoss: finite(data.efficiencyLoss),
    brightnessPower: finite(data.brightnessPower),
    heatpipePower: finite(data.heatpipePower),
    batteryLevel: finite(data.batteryLevel),
    absoluteBatteryLevel: finite(data.absoluteBatteryLevel),
    temperature: finite(data.temperature),
    adapterWatts: finite(data.adapterWatts),
    adapterVoltage: finite(data.adapterVoltage),
    adapterAmperage: finite(data.adapterAmperage),
  }
}
