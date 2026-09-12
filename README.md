# haqumei-wasm

Browser/mobile WebAssembly port of [o24s/haqumei](https://github.com/o24s/haqumei) with external runtime dictionary support.

This repository contains the browser WASM port, reproducible build overlays, and distributable runtime artifacts for Haqumei.

## Pinned upstream

- Haqumei: `0.12.0`
- Upstream commit: `f31c4342f0d94ffba08ce4b5abeb19b07bac5ac4`
- Port version: `0.12.0-wasm.3`
- Target: `wasm32-unknown-unknown`
- wasm-bindgen target: `web`

## Runtime assets

`runtime/v0.12.0-wasm.3/` contains the immutable browser runtime:

- `haqumei_wasm_bg.wasm`
- `haqumei_wasm.js`
- `haqumei_wasm.d.ts`
- `haqumei_dictionary_v0.12.0.bin.gz`

The dictionary gzip concatenates the pinned Haqumei `system.bin`, `char.bin`, and `matrix.bin` blobs in that order. See `runtime/manifest.json` for sizes and SHA-256 values.

## Rebuild

`./scripts/prepare-upstream.sh` creates a clean checkout of the pinned upstream commit and applies the port patches and overlays. `./scripts/build-web.sh` then builds the WASM target and verifies that generated wasm-bindgen output is byte-identical to the committed runtime package.

The browser port target-gates native-only filesystem/mmap, Rayon, ONNX Runtime, and Kanalizer facilities. The original tiny `何` classifier is represented by equivalent generated Rust decision-tree data. `predict_kana_english` remains unavailable in this browser build.

## Dictionary inspection

The browser `Dictionary` wrapper exposes read-only inspection of the loaded system lexicon for consumers that need dictionary-derived indexes without copying the full dictionary into JavaScript. Feature records are available through bounded read-only APIs. Consumers can request raw feature batches, POS/POS1-filtered feature batches over bounded word-id ranges, exact-surface feature records, or an exact-surface existence check. These APIs avoid requiring consumers to materialize the full system lexicon in JavaScript.

## Distribution

Runtime files are committed under a versioned `runtime/` directory so consumers can pin an immutable Git commit URL. Tag pushes also publish the same files as GitHub Release assets.

## License

Haqumei is Apache-2.0; see `LICENSE`. Dictionary and pyopenjtalk-related notices are in `COPYING-dictionary.txt` and `LICENSE-pyopenjtalk.txt`.
