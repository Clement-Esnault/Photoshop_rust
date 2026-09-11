<template>
  <div class="space-y-3">
    <input
      type="file"
      accept="image/*"
      @change="onFileChange"
      class="block w-full text-sm text-gray-600 file:mr-4 file:py-2 file:px-4 file:rounded file:border-0 file:bg-gray-100 file:text-gray-700 hover:file:bg-gray-200"
    />
    <canvas ref="canvasRef" class="border border-gray-300 rounded max-w-full"></canvas>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue'

const canvasRef = ref<HTMLCanvasElement | null>(null)
const originalImage = ref<HTMLImageElement | null>(null)

function onFileChange(event: Event) {
  const input = event.target as HTMLInputElement
  const file = input.files?.[0]
  if (!file) return

  const img = new Image()
  img.onload = () => {
    originalImage.value = img
    drawImage(img)
  }
  img.src = URL.createObjectURL(file)
}
function drawImage(img: HTMLImageElement) {
  const canvas = canvasRef.value
  if (!canvas) return
  canvas.width = img.width
  canvas.height = img.height
  const ctx = canvas.getContext('2d', { willReadFrequently: true })
  ctx?.drawImage(img, 0, 0)
}

function reset() {
  if (originalImage.value) drawImage(originalImage.value)
}

defineExpose({ canvasRef, reset })
</script>