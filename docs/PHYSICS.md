# Physics

The cloth is a rectangular particle grid embedded in 3D. Verlet-style prediction applies velocity damping and acceleration, then iterative XPBD distance projections enforce:

- structural constraints between horizontal and vertical neighbors;
- shear constraints across both diagonals of each quad;
- bending constraints between particles two grid positions apart.

Compliance is divided by `dt²`, so stiffness is substantially less frame-rate-dependent than direct springs. The current implementation does not retain a Lagrange multiplier across iterations, so it is an XPBD-style compliant projection rather than a full warm-started XPBD implementation. Default medium quality is 50 × 32 (1,600 vertices), fixed at 1/120 s with seven constraint iterations. The render loop executes at most six catch-up steps and discards long-pause history.

Pinned particles have zero inverse mass and are restored to exact pin targets after projection. Full edge, two-point, and top-edge layouts are implemented. Material presets vary mass, compliance, damping, and aerodynamic drag; values are artistic real-time parameters, not claims of measured material fidelity.

Gravity is world-down. Window acceleration contributes the pseudo-acceleration `-a_container`. Self-collision and object collision are not implemented; extreme folding can therefore pass through itself. Non-finite particles are recovered to their initial locations as a final safety guard.

