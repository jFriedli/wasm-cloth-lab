# Contact research

The production flag still has no self-collision. This is a deliberate scope decision, not an overlooked checkbox.

## Practical pipeline considered

A real-time approximation would use a refitted uniform grid or BVH broadphase, exclude vertices/edges within a topological neighborhood, test vertex–face and edge–edge candidates, and solve finite-thickness unilateral XPBD constraints. Relative tangential velocity could then be reduced with a bounded Coulomb-style velocity correction. Swept bounds or conservative displacement caps would be needed for fast shake inputs; discrete corrections alone can miss tunneling.

This is materially more work than a vertex–vertex repulsion pass, which neither prevents edge crossings nor establishes robust shell thickness. Pair counts can spike exactly during the violent window-shake case where latency matters most.

## Research comparison

- Penalty/contact constraints are cheap but penetrable and stiffness/step dependent.
- XPBD unilateral constraints are stable and fit this solver, but discrete detection has no global nonintersection guarantee.
- IPC (Li et al., SIGGRAPH 2020, [DOI](https://doi.org/10.1145/3394176.3394198)) combines barrier potentials, CCD, and nonlinear optimization.
- C-IPC (Li, Kaufman, Jiang, SIGGRAPH 2021, [DOI](https://doi.org/10.1145/3450626.3459767)) handles codimensional finite-thickness shells, strain barriers, friction, and additive CCD.
- OGC (Chen et al., SIGGRAPH 2025, [DOI](https://doi.org/10.1145/3731205)) uses offset geometry and displacement bounds to expose local, highly parallel contact work and reduce CCD cost.

## Decision

No contact prototype was promoted. The primary top-edge flag has limited sustained self-contact, while a robust implementation would increase variable frame cost and threaten the defining low-latency shake response. A future curtain/drop-on-sphere mode would change that tradeoff. The next credible prototype should be explicitly named “finite-thickness XPBD contact,” include vertex–face and edge–edge cases and a deterministic folded-cloth benchmark, and must not be represented as IPC, C-IPC, or OGC.
