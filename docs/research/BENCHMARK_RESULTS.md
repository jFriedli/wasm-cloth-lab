# Benchmark results

Recorded on the available Linux x86-64 development host on 21 August 2026 with a release-native build. These numbers are not browser or WASM claims. The full CSV is reproducible from `solver_bench`; medians are emphasized because host scheduling caused large p95/p99 tails.

## Default-mesh deterministic shake (B4, 50×32)

| Candidate | Median ms/step | Max structural strain | Final RMS structural | Final normalized kinetic energy | Result |
| --- | ---: | ---: | ---: | ---: | --- |
| Baseline compliant GS, 1×7 | 1.042 | 0.657 | 0.0238 | 4.285 | control |
| True XPBD, 1×7 | 1.020 | 0.858 | 0.0262 | 3.635 | existing compliance is too soft under full XPBD |
| Small Steps XPBD, 7×1 | 1.507 | 0.491 | 0.0170 | 0.077 | tighter at 50×32, over-damped and unstable at high resolution |
| Hybrid XPBD, 2×4 | 1.312 | 0.560 | 0.0213 | 0.995 | tighter, but slower and changes motion character |
| Baseline + relative aerodynamics | 1.088 | 0.624 | 0.0277 | 3.564 | selected |
| Baseline + isometric bending | 3.375 | 1.196 | 0.0490 | 4.504 | rejected without major retuning |

Relative aerodynamics added 0.047 ms (4.5%) to the B4 median. Its 50×32 B6 replay cost 1.273 versus 1.081 ms; the variation between otherwise identical B4/B6 input paths illustrates why results are reported rather than generalized into a browser speed claim.

## Resolution scaling under deterministic shake (B6)

| Solver | 30×20 median ms / max strain | 50×32 | 75×48 | 100×64 |
| --- | ---: | ---: | ---: | ---: |
| Baseline 1×7 | 0.345 / 0.301 | 1.081 / 0.657 | 2.564 / 1.209 | 5.994 / 1.810 |
| True XPBD 1×7 | 0.380 / 0.409 | 1.137 / 0.858 | 3.195 / 1.617 | 6.662 / 2.086 |
| Small Steps 7×1 | 0.504 / 0.098 | 1.551 / 0.491 | 3.864 / 59.012 | 7.067 / 2.96×10¹³ |
| Hybrid XPBD 2×4 | 0.453 / 0.231 | 1.411 / 0.560 | 5.005 / 1.181 | 8.463 / 1.802 |
| Relative aerodynamics | 0.377 / 0.323 | 1.273 / 0.624 | 2.756 / 1.210 | 7.449 / 1.819 |
| Isometric bending | 1.038 / 0.456 | 3.460 / 1.196 | 9.025 / 1.491 | 16.396 / 1.926 |

The Small Steps failure is the strongest result: a configuration attractive at 600–1,600 vertices cannot be shipped when the default is Ultra (6,400 vertices). The likely causes are resolution-dependent constraint density/compliance, more frequent aerodynamic/integration updates, and the existing bending network—not a refutation of the paper.

## Hanging and violent stress

Under B5 violent stress, baseline, relative-aero, and hybrid candidates all remained finite and pinned; maximum structural strain was respectively 0.216, 0.204, and 0.271.

## Bending ablation

The cotangent quadratic isometric energy is a materially better model class than a two-edge distance proxy, but a direct substitution with the old scalar `bend` compliance was not a fair material calibration. It took 3.375 ms median versus 1.042 ms in B4 and increased maximum strain from 0.657 to 1.196. At Ultra it took 16.396 ms versus 5.994 ms. The implementation is retained only for reproducible research; production keeps the cheap distance proxy until a resolution-aware calibration and visual capture demonstrate value.

## Memory and allocation note

The original baseline allocated a cloned index vector every aerodynamic pass. The production change iterates immutable topology directly, removing that per-step allocation. Experimental isometric matrices are lazily constructed only when that benchmark mode is used, so they do not increase production cloth memory. Earlier CSV memory columns collected before that lazy-build correction include the experimental bend storage for all candidates and must not be treated as production memory totals.

## Interpretation limits

Stretch peaks above 1.0 at fine resolution reveal that the deterministic shake is deliberately harsh and that maximum local error is resolution sensitive. No solver is presented as calibrated engineering cloth. Visual browser evaluation is still needed for subjective fold quality and real title-bar motion; it was not fabricated here.
