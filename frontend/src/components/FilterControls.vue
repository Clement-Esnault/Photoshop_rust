<template>
  <div class="flex flex-wrap items-center gap-2">
    <button @click="$emit('apply-filter', 'grayscale')" class="px-4 py-2 bg-blue-500 hover:bg-blue-600 text-white rounded transition-colors">
      Niveaux de gris
    </button>
    <button @click="$emit('apply-filter', 'grayscale_weighted')" class="px-4 py-2 bg-indigo-500 hover:bg-indigo-600 text-white rounded transition-colors">
      Gris pondéré
    </button>
    <button @click="$emit('apply-filter', 'sepia')" class="px-4 py-2 bg-amber-600 hover:bg-amber-700 text-white rounded transition-colors">
      Sépia
    </button>
    <button @click="$emit('apply-filter', 'invert')" class="px-4 py-2 bg-pink-600 hover:bg-pink-700 text-white rounded transition-colors">
      Inverser
    </button>

    <div class="flex items-center gap-2">
      <label class="text-sm text-gray-600">Radius :</label>
      <input type="range" min="1" max="15" v-model.number="blurRadius" class="w-24" />
      <span class="text-sm">{{ blurRadius }}</span>
      <button @click="$emit('apply-blur', blurRadius, false)" class="px-4 py-2 bg-teal-600 hover:bg-teal-700 text-white rounded transition-colors">
        Flou
      </button>
      <button @click="$emit('apply-blur', blurRadius, true)" class="px-4 py-2 bg-cyan-600 hover:bg-cyan-700 text-white rounded transition-colors">
        Flou rapide
      </button>
    </div>

    <button @click="$emit('reset')" class="px-4 py-2 bg-gray-500 hover:bg-gray-600 text-white rounded transition-colors">
      Réinitialiser
    </button>
    <button @click="$emit('benchmark')" class="px-4 py-2 bg-green-600 hover:bg-green-700 text-white rounded transition-colors">
      Benchmark
    </button>
    <button @click="$emit('download')" class="px-4 py-2 bg-purple-600 hover:bg-purple-700 text-white rounded transition-colors">
      Télécharger
    </button>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue'

const blurRadius = ref(3)

defineEmits<{
  'apply-filter': [filter: 'grayscale' | 'grayscale_weighted' | 'sepia' | 'invert']
  'apply-blur': [radius: number, fast: boolean]
  'reset': []
  'benchmark': []
  'download': []
}>()
</script>