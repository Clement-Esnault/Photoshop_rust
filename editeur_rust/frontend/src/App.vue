<script setup>
import { onMounted } from 'vue'
import init, { add } from './wasm/rust_core.js'

onMounted(async () => {
  await init()
  console.log(add(2, 3)) // doit afficher 5
})
</script>