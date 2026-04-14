## ADDED Requirements

### Requirement: Static deployment
The web build SHALL produce a self-contained set of static files (HTML, JS, WASM) deployable to GitHub Pages with no server-side component.

#### Scenario: Build output
- **WHEN** `wasm-pack build` completes for `maze-web`
- **THEN** the output directory contains an HTML shell, JS glue code, and a `.wasm` binary

### Requirement: GitHub Actions CI
The CI pipeline SHALL build and test the WASM target on every push.

#### Scenario: CI steps
- **WHEN** code is pushed
- **THEN** CI runs: `cargo test --workspace`, `wasm-pack test --node -p maze-core`, `wasm-pack build -p maze-web`

### Requirement: GitHub Pages deployment
A GitHub Actions workflow SHALL deploy the web build to GitHub Pages on pushes to main.

#### Scenario: Deploy on merge
- **WHEN** a commit is pushed to main
- **THEN** the workflow builds the WASM output and deploys it to GitHub Pages
