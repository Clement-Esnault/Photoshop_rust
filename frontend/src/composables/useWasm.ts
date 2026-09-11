import { ref } from 'vue'
import init, { grayscale, sepia } from '../../src/wasm/rust_core.js'

const ready = ref(false)
let initPromise: Promise<void> | null = null

export function useWasm() {
  if (!initPromise) {
    initPromise = init().then(() => {
      ready.value = true
    })
  }

  return {
    ready,
    grayscale,
    sepia,
  }
}