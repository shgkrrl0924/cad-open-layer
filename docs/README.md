# Docs

Design and architecture documentation for CAD Open Layer.

## Files (to be moved here from `~/.gstack/projects/rcad/`)

| File | Purpose |
|---|---|
| `architecture.md` | Top-level 3-layer design |
| `algorithms.md` | 5 core extraction algorithms (pseudocode + Rust signatures) |
| `considerations.md` | Edge cases, missing pieces, real-world quirks |
| `deep-dive.md` | Critical engineering gaps (parser FSM, DCEL, equivalence, etc.) |
| `engine-design.md` | Cargo workspace + 4-week PoC build plan |

## Recommended reading order

1. `README.md` (project root) — high-level "what and why"
2. `architecture.md` — 3-layer overview
3. `algorithms.md` — algorithm summaries (start here for implementation)
4. `considerations.md` — what could go wrong with real-world data
5. `deep-dive.md` — concrete code-level decisions

## Status

These docs currently live at `~/.gstack/projects/rcad/`. They will be copied
into this directory before the first public commit, after legal-package
ratification by counsel.
