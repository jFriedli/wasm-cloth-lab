# Browser compatibility

The required APIs are WebGL2, ES modules, WebAssembly, `requestAnimationFrame`, and `createImageBitmap`. The intended targets are current Firefox, Chrome, and Edge desktop; current mobile browsers can use ambient wind, gusts, controls, and imported images. Safari is desirable but has not been manually verified.

Automated build/runtime checks only establish API startup and rendering in their recorded browser environment. They do not validate title-bar dragging. Window position sampling varies substantially across browsers, compositors, Wayland/X11, macOS, Windows, multiple monitors, and privacy modes. Maximized/mobile windows may report no movement. The app degrades to ambient/manual gust wind without failing.

Manual browser results belong in this file only after executing `docs/ACCEPTANCE.md`; none are claimed merely from compilation.

