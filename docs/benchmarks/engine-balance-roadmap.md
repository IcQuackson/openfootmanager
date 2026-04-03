# Engine Balance Roadmap

## Goal

Move the match engine from flat style multipliers toward a more realistic football model where:

- tactical styles create tradeoffs instead of universal buffs
- formations have structural identities beyond raw defender/midfielder/forward counts
- transitions matter, especially for `Counter` and `HighPress`
- player stat distributions by role look believable over many matches
- benchmark deltas can be tracked after every balance change

## Calibration Targets

These are the intended operating bands for equal-strength benchmark matches.

### Global match profile

- goals per match: `2.4` to `3.0`
- shots per team: `9` to `15`
- shots on target per team: `3` to `6`
- corners per team: `3` to `7`
- fouls per team: `8` to `16`
- yellow cards per team: `1.0` to `3.5`
- red cards per team: `0.02` to `0.12`
- home advantage: `+0.15` to `+0.35` goals and `+0.10` to `+0.25` PPG

### Possession and style ecology

- equal matches should usually land around `48%` to `55%` home possession
- `Possession` should tend to `53%` to `58%`, not dominate every tactical matchup
- `Counter` should tend to `42%` to `49%`, but remain competitively viable
- style spread should usually stay within `0.05` to `0.15` average PPG from best to worst
- shape spread inside one style should usually stay within `0.02` to `0.08` average PPG

### Player stat shape

- defenders should attempt more passes than forwards
- midfielders should dominate involvement and progression volume
- forwards should lead shot share and box actions
- regular starting defenders should almost never finish with zero pass attempts

## Current problems

1. Styles are mostly flat phase multipliers.
2. `Possession` gains too much from midfield control without enough downside.
3. `Counter` lacks true transition creation mechanics.
4. `HighPress` does not pay enough fatigue or exposure cost.
5. `Defensive` does not suppress shot volume strongly enough.
6. Formations flatten too aggressively, especially `4-2-3-1` and `4-1-4-1` into `4-5-1` behavior.
7. Actor selection is too random inside broad position buckets.

## Implementation sequence

### Fix 1: Tactical profile refactor

Introduce explicit style and shape profile helpers so future rebalances are data-driven instead of scattered scalar tweaks.

Expected result:

- no intentional gameplay change
- benchmark outputs should remain close to the pre-refactor baseline

Validation:

- `cargo test -p engine`
- `npm run build`
- quick balanced benchmark sample
- quick tactic matrix benchmark sample

### Fix 2: Style and transition rebalance

Implement the core tactical tradeoffs.

Changes:

- split style effects into tempo, buildup retention, transition directness, press intensity, defensive compactness, and fatigue burden
- nerf `Possession` direct chance creation while keeping retention strength
- buff `Counter` specifically in transition states after recoveries
- make `HighPress` stronger in regains but costlier in stamina and defensive stability
- make `Defensive` reduce box access and shots against more reliably

Expected result:

- `Possession` remains strong but no longer universally dominant
- `Counter` becomes matchup-sensitive instead of simply weak
- `Defensive` gains a clear low-event niche

Validation:

- `cargo test -p engine`
- `npm run build`
- benchmark samples versus previous commit

Observed sample outcome after implementation:

- balanced benchmark sample moved to roughly `3.03` goals per match, `10.69` home shots, `8.49` away shots
- `Possession` stopped being the default top style in every shape
- `HighPress` became the new strongest style, which is acceptable for this intermediate patch but still needs checking after formation semantics land
- `Counter` became competitive instead of sitting at the bottom of the matrix

### Fix 3: Formation semantics and actor weighting

Make structural shapes and player roles matter more.

Changes:

- add formation profiles that influence buildup width, midfield support, final-third presence, and rest-defense stability
- stop treating `4-2-3-1` and `4-1-4-1` as mere aliases of generic `4-5-1` behavior in the engine
- replace random preferred-bucket actor selection with weighted involvement by zone and role

Expected result:

- shape spread increases modestly without exploding balance
- more believable per-role passing and shooting distributions
- closer alignment between tactical shape and match flow

Validation:

- `cargo test -p engine`
- `npm run build`
- benchmark samples versus previous commit

## Benchmark workflow

For each fix:

1. implement the change in isolation
2. run `cargo test -p engine`
3. run `npm run build`
4. run quick local benchmarks:
   - `bash scripts/record-engine-benchmark.sh --matches 500 --history-file /tmp/engine-balanced-history.jsonl --allow-dirty`
   - `bash scripts/render-tactic-matrix-benchmark.sh --matches-per-leg 20 --output-dir /tmp/ofm-tactic-bench --allow-dirty`
5. inspect the deltas
6. commit only when the change behaves as intended

## Success criteria

The balancing pass is considered successful when:

- benchmark match stats land inside the target bands
- `Possession` is no longer the best style in every shape by default
- `Counter` has at least some favorable matchups into aggressive or possession-heavy styles
- `Defensive` visibly lowers shot concession
- formation changes matter more than they do now
- player role stat distributions look believable in aggregate
