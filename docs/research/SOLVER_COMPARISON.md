# Solver comparison

| Family | Stiffness/control | Parallelism | Contact path | Browser fit | Experimental outcome |
| --- | --- | --- | --- | --- | --- |
| Compliant serial PBD (production) | iteration/step dependent, artist tuned | serial GS | XPBD contacts possible | excellent simplicity/latency | most robust current behavior |
| Full XPBD | compliance + total multipliers | GS/Jacobi/coloring | natural unilateral constraints | excellent | old preset compliances became softer; needs complete retuning |
| Small Steps XPBD | frequent re-linearization | same per substep | frequent detection required | more integration/aero work | lower B4 stretch, catastrophic High/Ultra stress failure |
| Hybrid 2×4 XPBD | stronger default-mesh control | same | natural | acceptable but slower | lower B4 stretch, heavily reduced motion energy |
| Isometric-bend PBD | better curvature energy | four-vertex stencil | unchanged | feasible | ~2.9× B4 median and worse uncalibrated strain |
| Projective Dynamics | local/global stiff solve | sparse/global; DD variants parallel | requires changing system/iterations | possible but high infrastructure cost | researched, no knowingly incomplete prototype |
| Shell FEM | constitutive membrane/bending | sparse nonlinear | separate robust contact required | poor for current scope | rejected for runtime architecture |
| IPC/C-IPC | physical energies + barriers | expensive nonlinear/CCD | strongest guarantees | poor for low-latency Pages demo | research reference only |
| WebGPU XPBD/PD | usually Jacobi/color batches | highly parallel | GPU broadphase viable | future high-resolution mode | no migration at current mesh/cost |

## Why no Projective Dynamics toy benchmark

A credible PD comparison needs the same membrane/bending energies, external forces, pins, time integration, and convergence target, plus a sparse global solve. A dense or distance-only toy would measure a different problem and its result would be misleading. The engineering estimate and literature review showed that maintaining sparse factorization/CG and dynamic pin/grab variants was unlikely to beat the tiny serial next-step path at 1,600–6,400 vertices. Following the task’s stop condition, implementation stopped before producing a knowingly useless number.

## Decision matrix rationale

- **Visual response and latency:** serial CPU projection consumes motion on the next fixed step with no GPU queue/readback.
- **Stability:** production baseline remained finite in every tested resolution/stress case; direct Small Steps did not.
- **Stretch:** hybrid improved B4 maximum strain by ~15%, but normalized final kinetic energy fell ~77%, changing the lively interaction.
- **Cost:** relative aerodynamics was essentially neutral at 50×32 in the recorded median; isometric bending and extra substeps were not.
- **Maintainability:** the selected change adds one explicit physical velocity subtraction and removes an allocation. It does not create a second production solver.
