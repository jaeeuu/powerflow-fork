import type { NormalizedResource } from '@/bindings'
import { describe, expect, it } from 'vitest'
import { normalizePowerData, selectDeviceValue } from './power'

describe('selectDeviceValue', () => {
  it('returns local and connected remote values', () => {
    expect(selectDeviceValue(1, { phone: 2 }, 'local', 0)).toBe(1)
    expect(selectDeviceValue(1, { phone: 2 }, 'phone', 0)).toBe(2)
  })

  it('uses a safe fallback for a missing remote tab', () => {
    expect(selectDeviceValue(1, {}, 'detached-phone', 0)).toBe(0)
  })
})

describe('power telemetry normalization', () => {
  it('tolerates JSON null and invalid measurements without breaking display arithmetic', () => {
    const data = normalizePowerData({
      systemIn: null,
      systemLoad: Number.NaN,
      batteryPower: 12.5,
      temperature: Number.POSITIVE_INFINITY,
      isCharging: false,
    } as NormalizedResource)
    expect(data.systemIn).toBe(0)
    expect(data.systemLoad).toBe(0)
    expect(data.temperature).toBe(0)
    expect(data.batteryPower).toBe(12.5)
    expect(data.isCharging).toBe(false)
  })
})
