# Aerodynamics

Every cloth triangle computes its current area vector and normalized face normal. Signed pressure is proportional to `|v·n|(v·n)`, current triangle area, and material drag. The force is shared across the triangle's three particles. A smaller tangential skin-drag term lets airflow parallel to a flat flag begin deforming it; subsequent folds expose faces and amplify pressure response.

Airflow combines ambient horizontal flow, browser-relative flow, two incommensurate temporal gust components, and triggered gust decay. The variation is deterministic and inexpensive, but it is procedural turbulence rather than CFD. Aerodynamic forces use triangle geometry, not a uniform arbitrary per-particle push.

