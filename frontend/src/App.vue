<template>
  <div class="max-w-2xl mx-auto p-8 space-y-6">
    <h1 class="text-2xl font-bold">Éditeur d'images</h1>

    <ImageCanvas ref="imageCanvasRef" />

    <FilterControls
      :active-filter="activeFilter"
      @apply-filter="onApplyFilter"
      @apply-blur="onApplyBlur"
      @reset="onReset"
      @benchmark="onBenchmark"
      @benchmark-blur="onBenchmarkBlur"
      @download="onDownload"
    />

    <p v-if="benchmarkResult" class="text-sm text-gray-600">
      JS : {{ benchmarkResult.js.toFixed(2) }} ms —
      Rust/WASM : {{ benchmarkResult.rust.toFixed(2) }} ms
    </p>
    <p v-if="blurBenchmarkResult" class="text-sm text-gray-600">
      Flou naïf : {{ blurBenchmarkResult.naive.toFixed(2) }} ms —
      Flou rapide : {{ blurBenchmarkResult.fast.toFixed(2) }} ms
    </p>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import ImageCanvas from './components/ImageCanvas.vue'
import FilterControls from './components/FilterControls.vue'
import { useFilters, type FilterName } from './composables/useFilters'
import { useBenchmark } from './composables/useBenchmark'
import { downloadCanvas } from './composables/useImageExport'

const imageCanvasRef = ref<InstanceType<typeof ImageCanvas> | null>(null)
const { applyFilter, applyBlur, activeFilter, clearActiveFilter } = useFilters()
const { benchmarkGrayscale, benchmarkBlur } = useBenchmark()

const benchmarkResult = ref<{ js: number; rust: number } | null>(null)
const blurBenchmarkResult = ref<{ naive: number; fast: number } | null>(null)

function onApplyFilter(filter: FilterName) {
  const canvas = imageCanvasRef.value?.canvasRef
  if (canvas) applyFilter(canvas, filter)
}

function onApplyBlur(radius: number, fast: boolean) {
  const canvas = imageCanvasRef.value?.canvasRef
  if (canvas) applyBlur(canvas, radius, fast)
}

function onReset() {
  imageCanvasRef.value?.reset()
  clearActiveFilter()
}

function onBenchmark() {
  const canvas = imageCanvasRef.value?.canvasRef
  if (canvas) benchmarkResult.value = benchmarkGrayscale(canvas)
}

function onBenchmarkBlur() {
  const canvas = imageCanvasRef.value?.canvasRef
  if (canvas) blurBenchmarkResult.value = benchmarkBlur(canvas)
}

function onDownload() {
  const canvas = imageCanvasRef.value?.canvasRef
  if (canvas) downloadCanvas(canvas)
}
</script>