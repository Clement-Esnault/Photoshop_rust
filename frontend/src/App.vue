<template>
  <div class="p-8">
    <h1 class="text-2xl font-bold mb-4">Éditeur d'images</h1>
    <input type="file" accept="image/*" @change="onFileChange" />
    <canvas ref="canvasRef" class="mt-4 border"></canvas>
  </div>
</template>

<script setup>
import { onMounted } from 'vue'
import init, { add } from './wasm/rust_core.js'

onMounted(async () => {
  await init()
  console.log(add(2, 3)) // doit afficher 5
})
</script>