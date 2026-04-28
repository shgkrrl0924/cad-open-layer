# Golden Corpus — Expected Semantic Extraction Results

This directory holds **expected `Floorplan` extraction results** for each test corpus file. Used for regression testing of the extraction algorithms.

## Files

| Golden | Source DXF | Style |
|---|---|---|
| `small_floorplan_simple_r2000.golden.json` | `corpus/synthetic/small_floorplan_simple_r2000.dxf` | **Element-precise** — every wall, opening, room, dimension specified |
| `medium_multifamily_floorplan_synthetic_r2000.golden.json` | `corpus/synthetic/medium_multifamily_floorplan_synthetic_r2000.dxf` | **Statistical** — aggregate counts and structural invariants (file too large for element-precise golden) |

## Two-tier approach

**Tier 1 — Element-precise (small file).**
Each wall, opening, room is named with exact coordinates and IDs. Used for unit-level regression. The extraction algorithm must produce structurally equivalent output (matching by spatial proximity, not ID).

**Tier 2 — Statistical (medium file).**
Aggregate counts (wall ranges, opening counts, room ranges) plus structural invariants (each unit has one entry door, grid is 19×19, etc.). Used for stress and integration testing. Confirms scale + correctness on realistic data without requiring perfect element match.

## How algorithms verify

```rust
// Tier 1
fn test_small_corpus_extraction() {
    let dxf = include_bytes!("../corpus/synthetic/small_floorplan_simple_r2000.dxf");
    let plan = extract_floorplan_from_dxf(dxf).unwrap();
    let golden: GoldenSpec = serde_json::from_str(
        include_str!("small_floorplan_simple_r2000.golden.json")
    ).unwrap();
    
    assert_floorplan_matches_element_precise(&plan, &golden);
}

// Tier 2
fn test_medium_corpus_extraction() {
    let plan = extract_floorplan_from_file("medium_multifamily_floorplan_synthetic_r2000.dxf").unwrap();
    let golden: StatisticalGolden = serde_json::from_str(/* ... */).unwrap();
    
    assert_within_range(plan.walls.len(), golden.expected_summary.wall_count_range);
    assert_within_range(plan.openings.iter().filter(|o| o.is_door()).count(), 1296, 5%);
    assert_eq!(plan.grids.len(), 1);
    // ... etc
}
```

## Updating goldens

When the algorithms intentionally change behavior (e.g., new wall classification rule), the goldens must be re-validated and updated. Do NOT update goldens to match buggy output.

Procedure:
1. Run extraction on corpus files
2. Diff output against current golden
3. If diff is intended improvement: update golden + commit with explanation
4. If diff is unexpected regression: fix algorithm, do not update golden
