# Tests

| Directory | Purpose |
|---|---|
| `corpus/synthetic/` | Hand-authored synthetic DXF test files. Apache-2.0 / public. |
| `corpus/golden/` | Expected `Floorplan` JSON for each corpus file. |
| `corpus/maket-real/` | NDA-restricted Maket sample files. **Gitignored. Never public.** |
| `property/` | Property-based tests (proptest). |
| `fuzz/` | Fuzzing targets (cargo-fuzz). |

## Adding new test files

1. Add corpus entry to `legal-package/06-test-corpus-license-log.md`
2. Drop the file in the appropriate subdirectory
3. CI `scripts/check_corpus_log_consistency.sh` will verify in PR

## Running tests

```bash
cargo test --workspace               # unit + integration
cargo test --release -- --ignored    # property tests (slow)
cd fuzz && cargo fuzz run parse_dxf  # fuzz (nightly)
```
