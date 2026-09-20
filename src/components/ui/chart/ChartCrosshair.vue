<script setup lang="ts">
import type { BulletLegendItemInterface } from '@unovis/ts'
import type { Component } from 'vue'
import { omit } from '@unovis/ts'
import { VisCrosshair, VisTooltip } from '@unovis/vue'
import { renderTooltip } from '@/lib/renderTooltip'
import { ChartTooltip } from '.'

const props = withDefaults(defineProps<{
  colors?: string[]
  index: string
  items: BulletLegendItemInterface[]
  customTooltip?: Component
}>(), {
  colors: () => [],
})

const yAccessors = computed(() => props.items.map(item =>
  (datum: Record<string, number>) => datum[item.name ?? ''],
))

// Use weakmap to store reference to each datapoint for Tooltip
const wm = new WeakMap()
function template(d: any) {
  if (wm.has(d)) {
    return wm.get(d)
  }
  else {
    const omittedData = Object.entries(omit(d, [props.index])).map(([key, value]) => {
      const legendReference = props.items.find(i => i.name === key)
      return { ...legendReference, value }
    })
    const TooltipComponent = props.customTooltip ?? ChartTooltip
    const html = renderTooltip(TooltipComponent, { title: d[props.index].toString(), data: omittedData })
    wm.set(d, html)
    return html
  }
}

function color(_d: unknown, i: number) {
  return props.colors[i] ?? 'transparent'
}
</script>

<template>
  <VisTooltip :horizontal-shift="20" :vertical-shift="20" />
  <VisCrosshair
    :x="(_d: unknown, i: number) => i"
    :y="yAccessors"
    :template="template"
    :color="color"
  />
</template>
