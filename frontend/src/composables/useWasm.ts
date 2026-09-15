import { ref } from 'vue'
import init, {
  grayscale,
  grayscale_weighted,
  sepia,
  invert,
  box_blur,
  box_blur_fast,
  brightness,
  contrast,
} from '../wasm/rust_core.js'

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
    grayscale_weighted,
    sepia,
    invert,
    box_blur,
    box_blur_fast,
    brightness,
    contrast,
  }
}