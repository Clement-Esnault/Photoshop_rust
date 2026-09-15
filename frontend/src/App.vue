<template>
  <div class="max-w-2xl mx-auto p-8 space-y-6">
    <h1 class="text-2xl font-bold">Éditeur d'images</h1>

    <ImageCanvas ref="imageCanvasRef" />

    <FilterControls
      @apply-filter="onApplyFilter"
      @apply-blur="onApplyBlur"
      @reset="onReset"
      @benchmark="onBenchmark"
      @download="onDownload"
    />

    <p v-if="benchmarkResult" class="text-sm text-gray-600">
      JS : {{ benchmarkResult.js.toFixed(2) }} ms —
      Rust/WASM : {{ benchmarkResult.rust.toFixed(2) }} ms
    </p>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import ImageCanvas from './components/ImageCanvas.vue'
import FilterControls from './components/FilterControls.vue'
import { useWasm } from './composables/useWasm'
import { grayscaleJs, measureTime } from './composables/useBenchmark'
import { downloadCanvas } from './composables/useImageExport'

const imageCanvasRef = ref<InstanceType<typeof ImageCanvas> | null>(null)
const { grayscale, grayscale_weighted, sepia, invert, box_blur, box_blur_fast } = useWasm()
const benchmarkResult = ref<{ js: number; rust: number } | null>(null)

function onApplyFilter(filter: 'grayscale' | 'grayscale_weighted' | 'sepia' | 'invert') {
  const canvas = imageCanvasRef.value?.canvasRef
  if (!canvas) return
  const ctx = canvas.getContext('2d', { willReadFrequently: true })
  if (!ctx) return

  const imageData = ctx.getImageData(0, 0, canvas.width, canvas.height)
  const pixels = imageData.data as unknown as Uint8Array

  if (filter === 'grayscale') grayscale(pixels)
  if (filter === 'grayscale_weighted') grayscale_weighted(pixels)
  if (filter === 'sepia') sepia(pixels)
  if (filter === 'invert') invert(pixels)

  ctx.putImageData(imageData, 0, 0)
}

function onApplyBlur(radius: number, fast: boolean) {
  const canvas = imageCanvasRef.value?.canvasRef
  if (!canvas) return
  const ctx = canvas.getContext('2d', { willReadFrequently: true })
  if (!ctx) return

  const imageData = ctx.getImageData(0, 0, canvas.width, canvas.height)
  const pixels = imageData.data as unknown as Uint8Array

  if (fast) box_blur_fast(pixels, canvas.width, canvas.height, radius)
  else box_blur(pixels, canvas.width, canvas.height, radius)

  ctx.putImageData(imageData, 0, 0)
}

function onReset() {
  imageCanvasRef.value?.reset()
}

function onBenchmark() {
  const canvas = imageCanvasRef.value?.canvasRef
  if (!canvas) return
  const ctx = canvas.getContext('2d', { willReadFrequently: true })
  if (!ctx) return

  const dataForJs = ctx.getImageData(0, 0, canvas.width, canvas.height)
  const dataForRust = ctx.getImageData(0, 0, canvas.width, canvas.height)

  const jsTime = measureTime(() => grayscaleJs(dataForJs.data))
  const rustTime = measureTime(() => grayscale(dataForRust.data as unknown as Uint8Array))

  benchmarkResult.value = { js: jsTime, rust: rustTime }
}

function onDownload() {
  const canvas = imageCanvasRef.value?.canvasRef
  if (!canvas) return
  downloadCanvas(canvas)
}
</script>