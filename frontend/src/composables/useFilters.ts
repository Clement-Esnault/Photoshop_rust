import { ref } from 'vue'
import { useWasm } from './useWasm'
import { getImageData, putImageData, asPixels } from './useCanvasPixels'

export type FilterName = 'grayscale' | 'grayscale_weighted' | 'sepia' | 'invert'

export function useFilters() {
  const { grayscale, grayscale_weighted, sepia, invert, box_blur, box_blur_fast } = useWasm()

  // Garde en mémoire le dernier filtre appliqué, pour que l'UI puisse
  // le mettre en évidence (bouton surligné).
  const activeFilter = ref<FilterName | 'blur' | null>(null)

  function applyFilter(canvas: HTMLCanvasElement, filter: FilterName) {
    const imageData = getImageData(canvas)
    if (!imageData) return
    const pixels = asPixels(imageData)

    if (filter === 'grayscale') grayscale(pixels)
    if (filter === 'grayscale_weighted') grayscale_weighted(pixels)
    if (filter === 'sepia') sepia(pixels)
    if (filter === 'invert') invert(pixels)

    putImageData(canvas, imageData)
    activeFilter.value = filter
  }

  function applyBlur(canvas: HTMLCanvasElement, radius: number, fast: boolean) {
    const imageData = getImageData(canvas)
    if (!imageData) return
    const pixels = asPixels(imageData)

    if (fast) box_blur_fast(pixels, canvas.width, canvas.height, radius)
    else box_blur(pixels, canvas.width, canvas.height, radius)

    putImageData(canvas, imageData)
    activeFilter.value = 'blur'
  }

  function clearActiveFilter() {
    activeFilter.value = null
  }

  return { applyFilter, applyBlur, activeFilter, clearActiveFilter }
}