<script setup lang="ts">
import { commands } from '@/bindings'

provide('isPopover', true)
</script>

<template>
  <div class="w-full" @click="commands.openApp()">
    <slot is-popover />
  </div>
</template>
