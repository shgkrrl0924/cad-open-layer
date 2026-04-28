# CAD Open Layer

Programmable, vendor-neutral, open-source layer for CAD file formats. Apache-2.0.

```rust
use cad_open_layer::prelude::*;

let plan = parse_dxf_file("floorplan.dxf")?;
for wall in &plan.walls {
    println!("Wall {}: {:.2}m, thickness {:.0}mm",
        wall.id, wall.length(), wall.thickness * 1000.0);
}
```

Reads and writes DXF files. Extracts architectural semantics (walls, openings, rooms, dimensions). Round-trip preserving. Works in Rust, JavaScript/TypeScript (via WebAssembly), Python (planned).

## Status

**v0.1 — Pre-release.** Stage 1 PoC in development. Expected first release: 2026 Q3.

## Why

Today, developers and AI agents accessing CAD files must compromise on one of three axes:

1. **Autodesk Platform Services** — works, but vendor lock-in and high cost.
2. **Existing open-source libraries** — round-trip breaks, semantic loss.
3. **AutoCAD COM/ActiveX automation** — Windows-only, slow, server-incompatible.

CAD Open Layer is a fourth path: open source, programmable, vendor-neutral, with semantic preservation as a first-class goal.

## Goals

- **High fidelity:** parse → manipulate → write produces files that open in AutoCAD without redraw or cleanup
- **Semantic-aware:** wall, opening, room, dimension as first-class types — not just raw geometry
- **Vendor-neutral:** no Autodesk SDK dependency, no ODA proprietary library
- **Multi-language:** Rust core, with WASM/Python/Node bindings (planned)
- **AI-ready:** designed for programmatic access by LLMs and automation tools

## Non-goals (for now)

- 3D modeling (this is a 2D drawing library)
- Rendering / visualization (use existing libraries)
- Replacing AutoCAD as an authoring tool
- Stage 1: full DWG binary support (DXF first; DWG in Stage 2+)

## Quick start

(Coming soon — Stage 1 v0.1 release pending.)

## Architecture

Three-layer architecture:

```
Layer 3: Semantic     | Wall, Opening, Room, Dimension, Grid
Layer 2: Geometry     | Point, Polyline, Polygon, intersect, area, DCEL
Layer 1: DXF format   | Tokenizer, streaming entity parser, writer
```

Each layer is independently usable. See `docs/architecture.md` for detail.

## Crate map

| Crate | Role |
|---|---|
| `cad-core` | Common types, errors, units, transforms |
| `cad-dxf-parser` | Layer 1 read (streaming) |
| `cad-dxf-writer` | Layer 1 write |
| `cad-geometry` | Layer 2 spatial primitives + algorithms (DCEL, spatial index) |
| `cad-semantic` | Layer 3 data model |
| `cad-extract` | Layer 2 → Layer 3 (semantic inference) |
| `cad-synthesize` | Layer 3 → Layer 2/1 (CAD synthesis) |
| `cad-maket-adapter` | Application-specific adapter (Maket residential PoC) |

## License

Apache License 2.0 — see `LICENSE` and `NOTICE`.

## Trademark Notice

DWG is a trademark of Autodesk, Inc., and AutoCAD is a registered trademark of Autodesk, Inc. CAD Open Layer is an independent open-source project and is not affiliated with, endorsed by, sponsored by, or otherwise connected to Autodesk, Inc. References to "DWG", "DXF", "AutoCAD", or other Autodesk product names are made solely for descriptive purposes to indicate file format compatibility.

## Contributing

See `CONTRIBUTING.md`. All contributors must complete the contributor declaration in `legal-package/04-contributor-declaration-template.md` before submitting code.

## Acknowledgments

This project's positioning was inspired by [rhwp](https://github.com/edwardkim/rhwp) (Apache-2.0), which demonstrated the model of a programmable open layer for the Korean HWP format.
