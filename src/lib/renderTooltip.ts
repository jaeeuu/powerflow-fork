import type { Component } from 'vue'
import { createApp } from 'vue'

export function renderTooltip(component: Component, props: Record<string, unknown>) {
  const container = document.createElement('div')
  const app = createApp(component, props)
  try {
    app.mount(container)
    return container.innerHTML
  }
  finally {
    app.unmount()
  }
}
