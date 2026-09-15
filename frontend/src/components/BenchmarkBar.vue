<template>
  <div class="space-y-1">
    <div class="flex justify-between text-sm text-gray-600">
      <span>{{ label }}</span>
      <span>{{ value.toFixed(2) }} ms</span>
    </div>
    <div class="w-full bg-gray-200 rounded h-4">
      <div
        class="h-4 rounded transition-all"
        :class="color"
        :style="{ width: percentage + '%' }"
      ></div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue'

const props = defineProps<{
  label: string
  value: number
  max: number
  color: string
}>()

// `computed` : valeur recalculée automatiquement à chaque changement
// de `value` ou `max`, sans qu'on ait à appeler une fonction manuellement.
const percentage = computed(() => (props.value / props.max) * 100)
</script>