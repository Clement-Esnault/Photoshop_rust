export function getImageData(canvas: HTMLCanvasElement): ImageData | null {
  const ctx = canvas.getContext('2d', { willReadFrequently: true })
  if (!ctx) return null
  return ctx.getImageData(0, 0, canvas.width, canvas.height)
}

export function putImageData(canvas: HTMLCanvasElement, imageData: ImageData) {
  const ctx = canvas.getContext('2d', { willReadFrequently: true })
  ctx?.putImageData(imageData, 0, 0)
}

// Le cast Uint8ClampedArray -> Uint8Array (nécessaire pour les fonctions
// WASM) est centralisé ici, plus besoin de le répéter partout.
export function asPixels(imageData: ImageData): Uint8Array {
  return imageData.data as unknown as Uint8Array
}