# Port overlay

Base source: `o24s/haqumei` commit `f31c4342f0d94ffba08ce4b5abeb19b07bac5ac4` (Haqumei 0.12.0).

`upstream.patch` contains the target-gated changes required by browser WASM. `haqumei-wasm/` is the wasm-bindgen wrapper crate. `haqumei/src/nani_model_data.rs` is generated decision-tree data equivalent to Haqumei's tiny pretrained `何` ONNX model. `vibrato-rkyv-0.7.17.patch` removes native-only mmap/cache dependencies from the WASM target and adds aligned in-memory dictionary ownership.

Run `../scripts/prepare-upstream.sh` to materialize a patched upstream checkout, then `../scripts/build-web.sh` to compile and compare the generated package with the committed runtime assets.
