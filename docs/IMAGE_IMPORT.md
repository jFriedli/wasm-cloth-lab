# Local image import

The file picker accepts browser-supported image types. The selected `File` is decoded with `createImageBitmap` and uploaded directly to the existing WebGL texture. It is never placed in a URL, sent through `fetch`, stored remotely, or uploaded. The static application has no backend or analytics.

Files over 20 MB and non-image MIME types are rejected with an on-screen error. PNG, JPEG, and WebP work where the browser's native decoder supports them. The current implementation stretches the decoded image across existing UVs. Explicit Fit/Fill controls, drag-and-drop, EXIF-specific handling, and pre-upload downsampling are deferred. GPU texture-size errors are reported by the browser console; very large pixel dimensions should be resized before selection.

