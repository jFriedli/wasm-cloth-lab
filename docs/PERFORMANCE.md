# Performance

The hot solver data and render arrays are contiguous Rust vectors. The browser performs one WASM step per fixed tick and bulk `bufferSubData` uploads for positions and normals. Indices and UVs are static. Frame delta, physics-call time, FPS, topology counts, and viewport/DPR are visible in debug mode.

Quality presets are 30 × 20, 50 × 32, 75 × 48, and 100 × 64 (default Ultra). All currently use seven constraint iterations. Catch-up is capped at six 1/120 s steps and the accumulator at 50 ms. The aerodynamic pass iterates static topology without per-step cloning. No JavaScript comparison benchmark is included because an equivalent maintained kernel was outside the core scope; no speedup claim is made.

Recorded performance should be measured on the deployed build and specific hardware. CI and headless correctness checks are not representative GPU benchmarks.

The deterministic native benchmark reports distributions and strain/stability metrics across solver ablations and resolutions. Results and important caveats are in [`docs/research/BENCHMARK_RESULTS.md`](research/BENCHMARK_RESULTS.md). No native timing is presented as browser/WASM performance.
