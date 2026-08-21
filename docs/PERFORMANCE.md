# Performance

The hot solver data and render arrays are contiguous Rust vectors. The browser performs one WASM step per fixed tick and bulk `bufferSubData` uploads for positions and normals. Indices and UVs are static. Frame delta, physics-call time, FPS, topology counts, and viewport/DPR are visible in debug mode.

Quality presets are 30 × 20, 50 × 32 (default), 75 × 48, and 100 × 64. All currently use seven constraint iterations; Ultra is intentionally opt-in. Catch-up is capped at six 1/120 s steps and the accumulator at 50 ms. No JavaScript comparison benchmark is included because an equivalent maintained kernel was outside the core scope; no speedup claim is made.

Recorded performance should be measured on the deployed build and specific hardware. CI and headless correctness checks are not representative GPU benchmarks.

