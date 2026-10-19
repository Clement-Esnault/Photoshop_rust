# Photoshop_rust

[![Rust CI](https://github.com/Clement-Esnault/Photoshop_rust/actions/workflows/rust-ci.yml/badge.svg)](https://github.com/Clement-Esnault/Photoshop_rust/actions/workflows/rust-ci.yml)
[![Licence MIT](https://img.shields.io/badge/licence-MIT-lightgrey)](LICENSE)

Éditeur d'images dans le navigateur, avec un moteur de traitement (filtres) écrit en Rust et compilé en WebAssembly, comparé en performance à une implémentation JavaScript équivalente.

🔗 **Démo en ligne** : https://photoshop-rust.esnaultclement4.workers.dev

## Fonctionnalités

- Chargement d'une image et rendu dans un canvas
- Filtres : inversion, niveaux de gris pondérés, luminosité, contraste, flou (version naïve et version rapide)
- Chaque filtre existe en **JavaScript** et en **Rust/WASM**
- Benchmark intégré avec graphiques comparatifs JS vs WASM
- Téléchargement de l'image modifiée

## Stack

- **Front** : Vue.js 3 + TypeScript, Vite, Tailwind CSS (logique métier en composables : `useFilters`, `useBenchmark`, `useCanvasPixels`)
- **Moteur de traitement** : Rust compilé en WASM via `wasm-bindgen` / `wasm-pack`
- **Qualité** : tests unitaires Rust + `clippy` en CI (GitHub Actions)
- **Déploiement** : Cloudflare (build automatique : install Rust, build WASM, build front)

## Lancer en local

Prérequis : Node.js, Rust (`rustup`), `wasm-pack`.

```bash
# Build le moteur WASM
cd rust-core
wasm-pack build --target web --out-dir ../frontend/src/wasm

# Lancer le front
cd ../frontend
npm install
npm run dev
```

Tests du moteur :

```bash
cd rust-core
cargo test
```

## Objectif

Comparer les performances de traitement d'image entre une implémentation JavaScript classique et une implémentation Rust/WASM, directement dans le navigateur.

## Licence

MIT, voir [LICENSE](LICENSE).
