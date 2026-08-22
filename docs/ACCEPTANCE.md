# Manual acceptance

Record browser, browser version, OS/window manager, commit, viewport, quality, and observed FPS before claiming manual compatibility.

1. Open the deployed app in Firefox.
2. Restore the browser window so it is not maximized.
3. Let flag settle.
4. Confirm ambient breeze produces gentle motion.
5. Move browser sharply right.
6. Verify apparent wind pushes flag left.
7. Stop quickly.
8. Verify inertia causes overshoot/flutter.
9. Move browser sharply left.
10. Verify response reverses.
11. Shake browser repeatedly.
12. Verify chaotic but stable flapping.
13. Stop moving browser.
14. Verify cloth settles.
15. Set ambient wind to zero, then restore it.
16. Trigger a gust with `F`.
17. Change Cotton → Silk.
18. Confirm silk is visibly more flexible.
19. Change Silk → Heavy canvas.
20. Confirm canvas is heavier/stiffer.
21. Drag a point on the flag with mouse or touch.
22. Release it and confirm stable recovery.
23. Import a local image.
24. Confirm image appears on the deforming flag.
25. In DevTools Network, confirm no image request/upload occurs.
26. Resize browser.
27. Confirm flag and texture survive.
28. Enable debug overlay with `D`.
29. Verify window velocity and acceleration change appropriately.
30. Verify relative-wind sign reverses relative to browser movement.
31. Serve the production build beneath `/labs/cloth/` and reload a nested URL.
32. Check browser console for errors.
33. Check the repository Actions run.
34. Check the deployed GitHub Pages version.

## Mobile physical-motion test

1. Open `https://jfriedli.com/labs/cloth-simulation/` directly on a phone.
2. Tap **TUNE**, then **Enable phone motion** if shown.
3. Grant motion permission.
4. Hold the phone in portrait and let the cloth settle.
5. Move the phone sharply toward visible right; verify the cloth trails left.
6. Stop sharply; verify overshoot and flutter.
7. Move left; verify the response reverses.
8. Shake side to side; verify strong stable flutter with no detached pins or exploding mesh.
9. Hold still; verify motion decays without persistent jitter.
10. Rotate to landscape and wait briefly for estimator reset.
11. Move toward visible right again; verify the cloth still trails toward visible left.
12. Move gently toward/away from you and assess whether Z billowing is intuitive.
13. Drag by touch, release, and verify stable recovery.
14. Open debug mode and verify source, permission, rate, orientation, acceleration, gesture velocity, wind, and inertia update.

Record phone model, OS, browser/version, orientation, direct/fallback acceleration, and subjective noise/latency. Real-device tuning should adjust documented bounded constants, not remove safety limits.
