# Engine Spatial 2D Implementation Plan

## Objective

Implement a spatial football simulation layer that can drive an accurate 2D match view while preserving existing tactical/role/trait philosophy.

## Deliverables

1. per-tick simulation in live engine (`sub-second`)
2. persistent player `x/y` and ball `x/y/z` state over time
3. movement/duel/ball-flight resolution at tick level
4. spatially-aware action selection (passing lanes, distances, pressing shape)
5. benchmark path updated to run against the spatial live simulation

## Architecture

- Keep public match commands and high-level lifecycle unchanged (`step_minute`, phase transitions, substitutions).
- Add an internal spatial subsystem under `engine/src/live_match/spatial.rs`.
- Extend `MatchSnapshot` with spatial telemetry:
  - `current_tick`
  - `ticks_per_minute`
  - `player_positions`
  - `ball` (`x/y/z`, velocity, holder)
- Keep current event schema; add optional tick metadata for replay precision.

## Simulation Loop

For each minute:

1. run `ticks_per_minute` tick updates
2. move players toward dynamic role anchors
3. integrate ball physics (`x/y/z`, velocity, drag/gravity)
4. resolve possession recovery if ball is free
5. resolve one holder action on a tick cooldown:
   - pass (lane-openness and progression scoring)
   - dribble (pressure and duel-driven)
   - shot (distance/angle/pressure + keeper check)
   - clearance (deep pressure emergency)
6. collapse tick outcomes into minute result events

## Tactical and Role Integration

- Role anchors define baseline position and line depth.
- Team style influences:
  - compactness
  - line push/drop
  - pressure behavior
  - transition directness
- Traits and attributes modulate execution success but do not bypass spatial constraints.

## Migration and Compatibility

- Live match commands and existing UI contracts remain backward compatible.
- Legacy zone-resolution path remains available as fallback while spatial path stabilizes.
- Benchmarks are switched to spatial path by routing `simulate_with_rng` through live simulation.

## Benchmark and Regression Plan

Run after each phase:

1. `cd src-tauri && cargo test -p engine`
2. `npm run build`
3. `bash scripts/record-engine-benchmark.sh --allow-dirty`
4. `bash scripts/render-tactic-matrix-benchmark.sh --allow-dirty`
5. `bash scripts/render-role-trait-matrix-benchmark.sh --allow-dirty`
6. `bash scripts/render-fit-profile-matrix-benchmark.sh --allow-dirty`

Track drift against:

- goals/shots/possession target bands in roadmap
- exploit watchlists in role-trait and fit-profile reports

## Risks

- Event inflation from over-frequent tick actions
- Excessive turnovers from strict lane checks
- Performance regression in large benchmark batches

## Mitigations

- action cooldown in ticks
- bounded probability curves and clamps
- stable default constants with benchmark-driven tuning
- keep temporary fallback path until spatial output is calibrated
