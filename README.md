#  Photoshop_rust

Éditeur d'images dans le navigateur, avec un moteur de traitement (filtres) écrit en Rust et compilé en WebAssembly, comparé en performance à une implémentation JavaScript équivalente.

🔗 **Démo en ligne** : https://photoshop-rust.esnaultclement4.workers.dev

## Stack

- **Front** : Vue.js 3 + TypeScript, Vite, Tailwind CSS
- **Moteur de traitement** : Rust compilé en WASM via `wasm-bindgen` / `wasm-pack`
- **Déploiement** : Cloudflare Pages (build automatique : install Rust, build WASM, build front)

## Lancer en local

Prérequis : Node.js, Rust (`rustup`), `wasm-pack`.

\`\`\`bash
# Build le moteur WASM
cd rust-core
wasm-pack build --target web --out-dir ../frontend/src/wasm

# Lancer le front
cd ../frontend
npm install
npm run dev
\`\`\`

## Objectif

Comparer les performances de traitement d'image entre une implémentation JavaScript classique et une implémentation Rust/WASM, directement dans le navigateur.