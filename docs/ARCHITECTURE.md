# Architecture

`cloth-core` owns vector math, particles, materials, topology, surface-relative aerodynamic loading, compliant distance constraints, integration, pinning, and normal generation. It contains no browser dependency and is unit-tested natively. Benchmark-only configuration can select full XPBD multipliers, substeps, or isometric bending without widening the production WASM API. `cloth-wasm` is a deliberately narrow `wasm-bindgen` wrapper: one simulation step crosses the boundary, and positions, normals, and indices are exposed as pointers into contiguous WASM arrays.

`web/src/motion.ts` estimates desktop window motion. `mobile-motion.ts` owns permission-aware sensor sampling, gravity fallback, viewport-axis conversion, and bounded gesture-velocity estimation. `motion-source.ts` converts either source into the same relative-wind and inertial-acceleration vectors; only one source is active, so forces are not double counted. `main.ts` owns the fixed-step accumulator, input composition, metrics, and UI. `renderer.ts` owns WebGL2 buffers, shaders, DPR-aware resizing, and replaceable textures. One `bufferSubData` uploads each packed attribute per rendered frame; there are no per-vertex JS/WASM calls or JSON mesh transfers.

Normals are computed in Rust after solving because triangle traversal already lives there, this keeps the topology in one place, and the packed result is also useful to non-WebGL consumers. The GPU interpolates vertex normals for lighting.

The solver attachment enum avoids assuming every future scene is a left-edge flag. Curtain, drop, or parachute scenes can provide different pin targets without replacing the constraint system.
