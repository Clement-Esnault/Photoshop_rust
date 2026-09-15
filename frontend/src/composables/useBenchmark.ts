import { useWasm } from './useWasm'
import { getImageData, asPixels } from './useCanvasPixels'

export function grayscaleJs(data: Uint8ClampedArray) {
  for (let i = 0; i < data.length; i += 4) {
    const gray = (data[i] + data[i + 1] + data[i + 2]) / 3
    data[i] = gray
    data[i + 1] = gray
    data[i + 2] = gray
  }
}

export function measureTime(fn: () => void): number {
  const start = performance.now()
  fn()
  const end = performance.now()
  return end - start
}

export function useBenchmark() {
  const { grayscale, box_blur, box_blur_fast } = useWasm()

  function benchmarkGrayscale(canvas: HTMLCanvasElement) {
    const dataForJs = getImageData(canvas)
    const dataForRust = getImageData(canvas)
    if (!dataForJs || !dataForRust) return null

    const jsTime = measureTime(() => grayscaleJs(dataForJs.data))
    const rustTime = measureTime(() => grayscale(asPixels(dataForRust)))

    return { js: jsTime, rust: rustTime }
  }

  function benchmarkBlur(canvas: HTMLCanvasElement, radius = 8) {
    const dataForNaive = getImageData(canvas)
    const dataForFast = getImageData(canvas)
    if (!dataForNaive || !dataForFast) return null

    const naiveTime = measureTime(() =>
      box_blur(asPixels(dataForNaive), canvas.width, canvas.height, radius)
    )
    const fastTime = measureTime(() =>
      box_blur_fast(asPixels(dataForFast), canvas.width, canvas.height, radius)
    )

    return { naive: naiveTime, fast: fastTime }
  }

  return { benchmarkGrayscale, benchmarkBlur }
}