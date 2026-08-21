# Physics

The cloth is a rectangular particle grid embedded in 3D. Verlet-style prediction applies velocity damping and acceleration, then iterative compliant position projections enforce:

- structural constraints between horizontal and vertical neighbors;
- shear constraints across both diagonals of each quad;
- bending constraints between particles two grid positions apart.

Compliance is divided by `dt²`, so stiffness is substantially less frame-rate-dependent than direct springs. Production does not retain a Lagrange multiplier across iterations, so it is an XPBD-style compliant projection rather than full XPBD. True XPBD and substep configurations exist only in the native research benchmark. Default Ultra quality is 100 × 64 (6,400 vertices), fixed at 1/120 s with seven constraint iterations. The render loop executes at most six catch-up steps and discards long-pause history.

Pinned particles have zero inverse mass and are restored to exact pin targets after projection. Full edge, two-point, and top-edge layouts are implemented. Material presets vary mass, compliance, damping, and aerodynamic drag; values are artistic real-time parameters, not claims of measured material fidelity.

Gravity is world-down. Window acceleration contributes the pseudo-acceleration `-a_container`. Self-collision and object collision are not implemented; extreme folding can therefore pass through itself. Non-finite particles are recovered to their initial locations as a final safety guard.

Aerodynamic pressure and tangential drag operate per triangle using `air velocity - average triangle surface velocity`. Signed normal incidence and triangle area determine pressure, and forces are distributed to the three vertices. This is a local real-time approximation, not wake-resolving fluid–structure interaction. Research rationale and measurements are in [`docs/research`](research/DECISION.md).
