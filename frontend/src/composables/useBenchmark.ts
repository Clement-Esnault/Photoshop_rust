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