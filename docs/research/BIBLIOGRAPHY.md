# Research bibliography

Primary publications and author/project pages were preferred. Status labels describe their use in this repository, not a judgment of the work.

## Constraint and shell solvers

- **[foundational, implemented baseline]** Müller, Heidelberger, Hennix, Ratcliff. “Position Based Dynamics.” *Journal of Visual Communication and Image Representation*, 2007. [DOI](https://doi.org/10.1016/j.jvcir.2007.01.005). Introduced direct positional constraint projection for interactive simulation.
- **[foundational, implemented experiment]** Macklin, Müller, Chentanez. “XPBD: Position-Based Simulation of Compliant Constrained Dynamics.” *MIG*, 2016. [DOI](https://doi.org/10.1145/2994258.2994272), [author PDF](https://mmacklin.com/xpbd.pdf). Adds compliance and accumulated multipliers; the production baseline previously used the compliance denominator but omitted the accumulated multiplier term.
- **[foundational, implemented experiment]** Macklin, Storey, Lu, Terdiman, Chentanez. “Small Steps in Physics Simulation.” *SCA*, 2019. [author PDF](https://mmacklin.com/smallsteps.pdf). Evaluates one iteration over many substeps against many iterations in one large step.
- **[foundational, implemented experiment]** Bender, Müller, Macklin. “A Survey on Position Based Dynamics.” *Eurographics Tutorials*, 2017. [DOI](https://doi.org/10.2312/egt.20171034). Source for the isometric bending energy used by the benchmark implementation.
- **[foundational, considered]** Grinspun, Hirani, Desbrun, Schröder. “Discrete Shells.” *SCA*, 2003. [DOI](https://doi.org/10.2312/SCA03/062-067), [author PDF](https://www.multires.caltech.edu/pubs/ds.pdf). Discrete membrane and bending energies for thin shells.
- **[foundational, considered]** Müller, Chentanez, Kim, Macklin. “Strain Based Dynamics.” *SCA*, 2014. [author PDF](https://matthias-research.github.io/pages/publications/strainBasedDynamics.pdf). Direct triangle/tetrahedron strain constraints, including independent stretch and shear control.
- **[foundational, considered but rejected for production]** Bouaziz, Martin, Liu, Kavan, Pauly. “Projective Dynamics: Fusing Constraint Projections for Fast Simulation.” *SIGGRAPH*, 2014. [project and paper](https://www.projectivedynamics.org/). Local projections plus a prefactorized global linear solve.
- **[modern, considered]** Lu, Shao, Yuksel, Sueda. “High-performance CPU Cloth Simulation Using Domain-decomposed Projective Dynamics.” *ACM TOG / SIGGRAPH*, 2025. [DOI](https://doi.org/10.1145/3731182), [project](https://sig25ddmpd.github.io/). Multicore domain decomposition aimed at much larger CPU simulations.
- **[modern, considered]** Zhang, James, Kaufman. “Progressive Dynamics++: A Framework for Stable, Continuous, and Consistent Animation Across Resolution and Time.” *ACM TOG / SIGGRAPH*, 2025. [project](https://pcs-sim.github.io/pd%2B%2B/). Progressive space/time refinement; relevant conceptually to quality changes, but not a low-cost replacement solver.
- **[modern, considered]** “XPBD Simulation of Constitutive Materials with Exponential Strain Tensor.” *MIG*, 2025. [DOI](https://doi.org/10.1145/3769047.3769050). A modern constitutive-material extension rather than a necessary flag solver.

## Contact, collision, and friction

- **[foundational, considered but rejected]** Li et al. “Incremental Potential Contact: Intersection- and Inversion-free, Large-Deformation Dynamics.” *ACM TOG / SIGGRAPH*, 2020. [DOI](https://doi.org/10.1145/3394176.3394198), [project](https://ipc-sim.github.io/). Barrier contact, CCD, and nonlinear optimization.
- **[modern, considered but rejected]** Li, Kaufman, Jiang. “Codimensional Incremental Potential Contact.” *ACM TOG / SIGGRAPH*, 2021. [DOI](https://doi.org/10.1145/3450626.3459767), [arXiv](https://arxiv.org/abs/2012.04457). Extends IPC to codimensional rods/shells with finite thickness, strain barriers, friction, and additive CCD.
- **[modern, considered]** Chen et al. “Offset Geometric Contact.” *ACM TOG / SIGGRAPH*, 2025. [DOI](https://doi.org/10.1145/3731205), [project](https://ankachan.github.io/Projects/OGC/index.html). Offset geometry and displacement bounds reduce reliance on expensive CCD; published examples target GPU-parallel workloads.
- **[implementation reference]** Interactive Computer Graphics group. [PositionBasedDynamics library](https://github.com/InteractiveComputerGraphics/PositionBasedDynamics). Reference implementations for XPBD constraints, isometric bending, collision detection, and related methods.

## Aerodynamics, flags, and material identification

- **[foundational, implemented concept]** Eberhardt, Weber, Strasser. “A Fast, Flexible, Particle-System Model for Cloth Draping.” *IEEE Computer Graphics and Applications*, 1996. [DOI](https://doi.org/10.1109/38.491187). Early particle cloth with aerodynamic loading.
- **[foundational, considered]** Virot, Amandolese, Hémon. “Fluttering Flags: An Experimental Study of Fluid Forces.” *Journal of Fluids and Structures*, 2013. [DOI](https://doi.org/10.1016/j.jfluidstructs.2013.09.012). Demonstrates that flag flutter is a coupled fluid–structure instability, not merely prescribed sinusoidal motion.
- **[modern, considered]** “Estimating Cloth Elasticity Parameters from Homogeneous Tests.” 2022. [arXiv](https://arxiv.org/abs/2212.08790). Illustrates inverse calibration and the limits of assigning authoritative fabric labels to artist-tuned parameters.
- **[modern, considered]** DiffXPBD: differentiable XPBD material estimation. [project search record](https://arxiv.org/search/?query=DiffXPBD&searchtype=all). Useful future direction for calibration, not required at runtime.

## Browser execution

- **[platform reference]** W3C GPU for the Web Community Group. [WebGPU specification](https://gpuweb.github.io/gpuweb/). Compute makes highly parallel GPU solvers possible, with a substantially different architecture and compatibility surface.
- **[platform reference]** MDN. [WebAssembly SIMD](https://developer.mozilla.org/en-US/docs/WebAssembly/Guides/Understanding_the_text_format#simd), [SharedArrayBuffer security requirements](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/SharedArrayBuffer). WASM threads require cross-origin isolation; the current ordinary Pages deployment deliberately does not require it.

## Search cutoff and caveats

The search covered primary literature and project pages available through **21 August 2026**, including SIGGRAPH/SIGGRAPH Asia 2024–2025 and credible 2026 preprints. Search-result snippets were not treated as evidence. Recent preprints without a stable primary manuscript, reproducible implementation, or clear relevance were not used to drive production decisions.
