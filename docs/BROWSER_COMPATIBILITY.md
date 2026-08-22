# Browser compatibility

The required APIs are WebGL2, ES modules, WebAssembly, `requestAnimationFrame`, and `createImageBitmap`. Mobile physical motion additionally requires a secure context and usable `devicemotion` samples.

| Browser family (current 2026 versions) | Published API support | Permission behavior used by the app | Runtime tested here |
| --- | --- | --- | --- |
| iOS Safari | Device Motion supported; WebKit exposes explicit permission | `requestPermission()` is feature-detected and called on button activation | No physical device |
| Android Chrome | Device Motion supported; site motion-sensor permission may apply | explicit request when exposed, otherwise passive listener | Synthetic/headless only |
| Android Firefox | Device Motion supported in published compatibility data | passive unless explicit request is exposed | No physical device |
| Samsung Internet | Device Motion supported in published compatibility data | passive unless explicit request is exposed | No physical device |
| Desktop Chrome/Edge/Firefox/Safari | interface availability varies; hardware samples are not assumed | desktop remains active until a valid sensor sample arrives | Chromium desktop startup only |

Published API availability does not guarantee that a phone supplies gravity-free acceleration, frequency, or equal axis quality. The code feature-detects permission and tolerates null fields.

An automated runtime smoke check was performed in headless Chromium on Linux: nested assets loaded, WASM and WebGL2 initialized, and a visibly sagging/textured mesh rendered without console errors. It establishes startup/rendering only and does not validate title-bar dragging or interactive performance. Window position sampling varies substantially across browsers, compositors, Wayland/X11, macOS, Windows, multiple monitors, and privacy modes. Maximized/mobile windows may report no movement. The app degrades to ambient/manual gust wind without failing.

Research sources checked on 22 August 2026: W3C Device Orientation and Motion, current MDN API/permission pages, current Can I Use tables, WebKit Safari 26.4 notes, and WebKit permission behavior. Manual browser results belong here only after executing `docs/ACCEPTANCE.md`; none are claimed merely from compilation.
