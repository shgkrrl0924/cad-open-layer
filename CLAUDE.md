# CAD Open Layer — Claude instructions

## Design System

Always read `DESIGN.md` before making any visual or UI decisions.
All font choices, colors, spacing, layout, and aesthetic direction are
defined there. Do not deviate without explicit user approval.
In QA mode, flag any code that doesn't match `DESIGN.md`.

The design system covers two surfaces with one coherent language:
- **cadopenlayer.dev** (marketing landing site)
- **WASM Playground** (browser-based interactive demo)

When introducing new components, derive them from the existing tokens and
patterns in `DESIGN.md`. Do not invent new colors, type sizes, or spacing
values without updating the system first.

## Engineering

Workspace layout: 10 Rust crates under `crates/`, one `fuzz/` cargo-fuzz
project (separate from workspace), one `legal-package/` directory with
compliance docs.

- **Layer 1:** `cad-dxf-parser`, `cad-dxf-writer`
- **Layer 2:** `cad-geometry`
- **Layer 3:** `cad-semantic`, `cad-extract`, `cad-synthesize`
- **Bindings:** `cad-wasm`, `cad-mcp`
- **Adapter:** `cad-maket-adapter` (stub, scope-call dependent)
- **Foundation:** `cad-core`

Test corpus is at `tests/corpus/synthetic/`. Every file there must be
registered in `legal-package/06-test-corpus-license-log.md` (CI enforces
this).

Every contributor must have a declaration in
`legal-package/contributors/{github-username}.md` before their commits
land (CI enforces this via `scripts/verify_contributors.sh`).

Parser is intentionally lenient — malformed numerics surface to
`DxfDocument::parse_warnings` rather than aborting parse. Do not change
to strict mode without discussion.

Synthesis preserves per-wall layer names. Do not collapse walls onto a
single layer in the writer.

`raw_extras` (unrecognized DXF group codes) must round-trip through
parse → write. Adding new emitter logic? Make sure `emit_extras` still
fires.
