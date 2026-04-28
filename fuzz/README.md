# cad-open-layer fuzz targets

cargo-fuzz harness for Layer 1 (DXF parser) and the Layer 1 → Layer 3 pipeline.

## Running locally

Requires Rust nightly:

```sh
rustup install nightly
cargo +nightly install cargo-fuzz

cd fuzz
cargo +nightly fuzz run parse_dxf -- -max_total_time=60
cargo +nightly fuzz run parse_dxf_then_extract -- -max_total_time=60
```

## CI

`.github/workflows/fuzz.yml` runs:

- **Quick** (5 min) on every PR touching `crates/cad-dxf-parser/**` or `fuzz/**`.
- **Extended** (4 hours) every Sunday 04:00 UTC.

## Targets

- `parse_dxf` — fuzz the streaming parser with arbitrary byte input. Must
  not panic; well-formed-ish input must round-trip cleanly.
- `parse_dxf_then_extract` — fuzz the full Layer 1 → Layer 3 pipeline. The
  DCEL room extractor and parallel-line-pair wall extractor handle many
  adversarial geometry edges.

## Corpus

`corpus/parse_dxf/seed_small.dxf` is the same synthetic file used in
integration tests. It gives the fuzzer a known-valid starting point so it
mutates around real DXF structure rather than trying to discover the
format from scratch.
