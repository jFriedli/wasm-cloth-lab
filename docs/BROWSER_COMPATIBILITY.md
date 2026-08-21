# Browser compatibility

The required APIs are WebGL2, ES modules, WebAssembly, `requestAnimationFrame`, and `createImageBitmap`. The intended targets are current Firefox, Chrome, and Edge desktop; current mobile browsers can use ambient wind, gusts, controls, and imported images. Safari is desirable but has not been manually verified.

An automated runtime smoke check was performed in headless Chromium on Linux: nested assets loaded, WASM and WebGL2 initialized, and a visibly sagging/textured mesh rendered without console errors. It establishes startup/rendering only and does not validate title-bar dragging or interactive performance. Window position sampling varies substantially across browsers, compositors, Wayland/X11, macOS, Windows, multiple monitors, and privacy modes. Maximized/mobile windows may report no movement. The app degrades to ambient/manual gust wind without failing.

Manual browser results belong in this file only after executing `docs/ACCEPTANCE.md`; none are claimed merely from compilation.
