# Aerodynamics research

## Baseline audit

At the preserved baseline each triangle computes an area normal, signed normal incidence, quadratic pressure, and smaller tangential drag. Forces are distributed equally to its vertices. The browser mapping is physically signed correctly: container velocity is subtracted from world wind, and container acceleration is negated separately.

The missing term was cloth motion. A deforming surface does not experience world air velocity directly:

`v_relative = v_air - (v_a + v_b + v_c) / 3`.

The candidate derives vertex velocity from Verlet state, applies the expression above per triangle, then uses the same two-sided normal pressure and skin-drag coefficients. It also iterates the immutable topology directly rather than cloning the index buffer every physics step.

## Interpretation

Surface-relative loading prevents a moving cloth patch from receiving the same force as a stationary one. It supplies physically motivated aerodynamic damping and changes transient flutter during stops and reversals. It does not resolve wakes, lift curves, vortex shedding, or surrounding fluid.

Experimental and numerical flag literature describes flutter as fluid–structure coupling; see Virot, Amandolese, Hémon, “Fluttering Flags: An Experimental Study of Fluid Forces,” *Journal of Fluids and Structures* 43 (2013), [DOI](https://doi.org/10.1016/j.jfluidstructs.2013.09.012). A browser solver cannot afford full FSI, so changing local surface velocity/normals plus deterministic gust/turbulence is the defensible approximation. No sinusoidal displacement is injected.

## Results and decision

In deterministic shake, surface-relative aerodynamics reduced maximum structural strain from 0.657 to 0.624 and normalized final kinetic energy from 4.285 to 3.564 in the final recorded 50×32 native run. Median step time changed from 1.042 to 1.088 ms in B4. It increased maximum transient vertex velocity, consistent with stronger opposing load during rapid relative motion, and remained finite under the violent scene.

This candidate is the only experimental physics change selected for production. Coefficients and browser responsiveness gains remain unchanged, so stationary ambient wind is not inflated and browser velocity/acceleration signs do not regress.
