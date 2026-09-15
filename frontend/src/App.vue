<template>
  <div class="max-w-2xl mx-auto p-8 space-y-6">
    <h1 class="text-2xl font-bold">Éditeur d'images</h1>

    <ImageCanvas ref="imageCanvasRef" />

    <FilterControls
      :active-filter="activeFilter"
      @apply-filter="onApplyFilter"
      @apply-blur="onApplyBlur"
      @apply-brightness="onApplyBrightness"
      @apply-contrast="onApplyContrast"
      @reset="onReset"
      @benchmark="onBenchmark"
      @benchmark-blur="onBenchmarkBlur"
      @download="onDownload"
    />

    <div v-if="benchmarkResult" class="space-y-2">
      <BenchmarkBar
        label="JavaScript"
        :value="benchmarkResult.js"
        :max="Math.max(benchmarkResult.js, benchmarkResult.rust)"
        color="bg-yellow-500"
      />
      <BenchmarkBar
        label="Rust/WASM"
        :value="benchmarkResult.rust"
        :max="Math.max(benchmarkResult.js, benchmarkResult.rust)"
        color="bg-orange-600"
      />
    </div>

    <div v-if="blurBenchmarkResult" class="space-y-2">
      <BenchmarkBar
        label="Flou naïf"
        :value="blurBenchmarkResult.naive"
        :max="Math.max(blurBenchmarkResult.naive, blurBenchmarkResult.fast)"
        color="bg-red-500"
      />
      <BenchmarkBar
        label="Flou rapide"
        :value="blurBenchmarkResult.fast"
        :max="Math.max(blurBenchmarkResult.naive, blurBenchmarkResult.fast)"
        color="bg-teal-600"
      />
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import ImageCanvas from './components/ImageCanvas.vue'
import FilterControls from './components/FilterControls.vue'
import BenchmarkBar from './components/BenchmarkBar.vue'
import { useFilters, type FilterName } from './composables/useFilters'
import { useBenchmark } from './composables/useBenchmark'
import { downloadCanvas } from './composables/useImageExport'

const imageCanvasRef = ref<InstanceType<typeof ImageCanvas> | null>(null)
const { applyFilter, applyBlur, applyBrightness, applyContrast, activeFilter, clearActiveFilter } = useFilters()
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

function onApplyBrightness(amount: number) {
  const canvas = imageCanvasRef.value?.canvasRef
  if (canvas) applyBrightness(canvas, amount)
}

function onApplyContrast(amount: number) {
  const canvas = imageCanvasRef.value?.canvasRef
  if (canvas) applyContrast(canvas, amount)
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