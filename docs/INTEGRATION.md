# Future website integration

Do not merge this repository's history into `jFriedli.github.io`. Preferred integration:

1. Add this repository as `labs/cloth-src` (or a similarly named build-source directory) using a Git submodule.
2. Checkout the website recursively in its deployment workflow.
3. Install Rust, `wasm-pack`, and Node.
4. Build WASM into `web/src/wasm`.
5. Run `npm ci` in `web` and `VITE_BASE=/labs/cloth/ npm run build`.
6. Copy `web/dist/*` into the website output at `labs/cloth/`.
7. Deploy the website output normally.

The checked-in source never assumes `/`. Vite rewrites the generated WASM, JS, and CSS URLs under the configured base. An alternative is for this repository's release workflow to upload the production `dist` directory as a versioned artifact; the website workflow can download, verify, and copy that artifact. Submodules provide reproducible source builds, while release artifacts shorten the website build and create a clearer binary trust boundary.

