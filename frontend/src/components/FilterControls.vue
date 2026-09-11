<template>
  <div class="flex flex-wrap gap-2">
    <button
      @click="$emit('apply-filter', 'grayscale')"
      class="px-4 py-2 bg-blue-500 hover:bg-blue-600 text-white rounded transition-colors"
    >
      Niveaux de gris
    </button>
    <button
      @click="$emit('apply-filter', 'sepia')"
      class="px-4 py-2 bg-amber-600 hover:bg-amber-700 text-white rounded transition-colors"
    >
      Sépia
    </button>
    <button
      @click="$emit('reset')"
      class="px-4 py-2 bg-gray-500 hover:bg-gray-600 text-white rounded transition-colors"
    >
      Réinitialiser
    </button>
    <button
      @click="$emit('benchmark')"
      class="px-4 py-2 bg-green-600 hover:bg-green-700 text-white rounded transition-colors"
    >
      Benchmark
    </button>

    <button
    @click="$emit('download')"
    class="px-4 py-2 bg-purple-600 hover:bg-purple-700 text-white rounded transition-colors"
    >
    Télécharger
    </button>
  </div>
</template>

<script setup lang="ts">
defineEmits<{
  'apply-filter': [filter: 'grayscale' | 'sepia']
  'reset': []
  'benchmark': []
  'download': []
}>()
</script>