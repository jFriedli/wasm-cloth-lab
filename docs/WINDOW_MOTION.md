# Window motion

The browser samples `window.screenX` and `window.screenY` on animation frames. Screen coordinates are CSS pixels, so no device-pixel-ratio conversion is applied; DPR and zoom affect canvas backing resolution independently. Negative multi-monitor coordinates are valid.

Two distinct mappings are preserved:

```text
relative airflow = world wind - container velocity
effective acceleration = gravity - container acceleration
```

Thus a window moving right produces leftward relative airflow. A window accelerating right produces a leftward inertial pseudo-force. A sudden stop reverses acceleration while the cloth retains velocity, creating overshoot. Rust and TypeScript tests lock these signs.

Finite differences operate on measured elapsed time. An exponential low-pass filter is normalized approximately to 60 Hz. Velocity and acceleration are independently clamped. Samples with implausibly large position jumps are rejected; samples delayed over 160 ms decay toward zero; pauses over one second reset history. Visibility changes also reset history. These measures suppress integer-coordinate jitter, tab suspension spikes, and many window-manager discontinuities.

The scaling from CSS pixels/s and pixels/s² to simulation units is deliberately configurable through Window response. Reporting quality varies by OS/browser and cannot be feature-detected beyond observing stable samples. Debug mode shows raw position and filtered derivatives.

