<template>
  <div class="flex flex-wrap items-center gap-2">
    <button
      @click="$emit('apply-filter', 'grayscale')"
      :class="[
        'px-4 py-2 text-white rounded transition-colors',
        activeFilter === 'grayscale' ? 'bg-blue-700 ring-2 ring-blue-300' : 'bg-blue-500 hover:bg-blue-600',
      ]"
    >
      Niveaux de gris
    </button>
    <button
      @click="$emit('apply-filter', 'grayscale_weighted')"
      :class="[
        'px-4 py-2 text-white rounded transition-colors',
        activeFilter === 'grayscale_weighted' ? 'bg-indigo-700 ring-2 ring-indigo-300' : 'bg-indigo-500 hover:bg-indigo-600',
      ]"
    >
      Gris pondéré
    </button>
    <button
      @click="$emit('apply-filter', 'sepia')"
      :class="[
        'px-4 py-2 text-white rounded transition-colors',
        activeFilter === 'sepia' ? 'bg-amber-800 ring-2 ring-amber-300' : 'bg-amber-600 hover:bg-amber-700',
      ]"
    >
      Sépia
    </button>
    <button
      @click="$emit('apply-filter', 'invert')"
      :class="[
        'px-4 py-2 text-white rounded transition-colors',
        activeFilter === 'invert' ? 'bg-pink-800 ring-2 ring-pink-300' : 'bg-pink-600 hover:bg-pink-700',
      ]"
    >
      Inverser
    </button>

    <div class="flex items-center gap-2">
      <label class="text-sm text-gray-600">Radius :</label>
      <input type="range" min="1" max="15" v-model.number="blurRadius" class="w-24" />
      <span class="text-sm">{{ blurRadius }}</span>
      <button
        @click="$emit('apply-blur', blurRadius, false)"
        :class="[
          'px-4 py-2 text-white rounded transition-colors',
          activeFilter === 'blur' ? 'bg-teal-800 ring-2 ring-teal-300' : 'bg-teal-600 hover:bg-teal-700',
        ]"
      >
        Flou
      </button>
      <button
        @click="$emit('apply-blur', blurRadius, true)"
        :class="[
          'px-4 py-2 text-white rounded transition-colors',
          activeFilter === 'blur' ? 'bg-cyan-800 ring-2 ring-cyan-300' : 'bg-cyan-600 hover:bg-cyan-700',
        ]"
      >
        Flou rapide
      </button>
    </div>

    <div class="flex items-center gap-2">
      <label class="text-sm text-gray-600">Luminosité :</label>
      <input type="range" min="-100" max="100" v-model.number="brightnessAmount" class="w-24" />
      <button
        @click="$emit('apply-brightness', brightnessAmount)"
        :class="[
          'px-4 py-2 text-white rounded transition-colors',
          activeFilter === 'brightness' ? 'bg-yellow-700 ring-2 ring-yellow-300' : 'bg-yellow-500 hover:bg-yellow-600',
        ]"
      >
        Appliquer
      </button>
    </div>

    <div class="flex items-center gap-2">
      <label class="text-sm text-gray-600">Contraste :</label>
      <input type="range" min="0.1" max="3" step="0.1" v-model.number="contrastAmount" class="w-24" />
      <button
        @click="$emit('apply-contrast', contrastAmount)"
        :class="[
          'px-4 py-2 text-white rounded transition-colors',
          activeFilter === 'contrast' ? 'bg-red-800 ring-2 ring-red-300' : 'bg-red-600 hover:bg-red-700',
        ]"
      >
        Appliquer
      </button>
    </div>

    <button @click="$emit('reset')" class="px-4 py-2 bg-gray-500 hover:bg-gray-600 text-white rounded transition-colors">
      Réinitialiser
    </button>
    <button @click="$emit('benchmark')" class="px-4 py-2 bg-green-600 hover:bg-green-700 text-white rounded transition-colors">
      Benchmark
    </button>
    <button @click="$emit('benchmark-blur')" class="px-4 py-2 bg-orange-600 hover:bg-orange-700 text-white rounded transition-colors">
      Benchmark flou
    </button>
    <button @click="$emit('download')" class="px-4 py-2 bg-purple-600 hover:bg-purple-700 text-white rounded transition-colors">
      Télécharger
    </button>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue'

defineProps<{
  activeFilter: string | null
}>()

const blurRadius = ref(3)
const brightnessAmount = ref(20)
const contrastAmount = ref(1.5)

defineEmits<{
  'apply-filter': [filter: 'grayscale' | 'grayscale_weighted' | 'sepia' | 'invert']
  'apply-blur': [radius: number, fast: boolean]
  'apply-brightness': [amount: number]
  'apply-contrast': [amount: number]
  'reset': []
  'benchmark': []
  'benchmark-blur': []
  'download': []
}>()
</script>