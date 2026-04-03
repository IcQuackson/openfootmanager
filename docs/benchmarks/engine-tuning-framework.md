## Engine Tuning Framework

The engine should be tuned by changing what situations get created, not by stacking flat tactical bonuses on final outcomes.

### Principles

1. Tune `who gets involved`
2. Tune `what action they choose`
3. Tune `what state that action creates next`
4. Tune `what failure mode the system pays`

This keeps tactical strength emergent from:

- player attributes
- player traits
- formation slot roles
- team play style
- formation support shape
- score and transition context

### Primary tuning levers

#### Action menu weighting

Change the likely internal intents available in each phase instead of giving a tactic a generic attack or midfield bonus.

Examples:

- `Possession`: more `SafeRecyclePass`, `OneTouchCombination`, `ThirdManLayoff`, `BaitPressTouch`
- `Counter`: more `SplitLinePass`, `BlindSideRun`, `NearPostAttack`, quick goalkeeper release
- `HighPress`: more `StepInInterception`, `TacticalFoulStop`, press-bait punish windows
- `Defensive`: more `ContainAndShowWide`, safer exits, stronger rest-defense suppression of counters

#### Support shape

The engine should model how many safe outlets and progression lanes a system creates in each phase.

Required support dimensions:

- `safe_outlets`: how easy it is to recycle or retain under pressure
- `progression_lanes`: how easy it is to find the next forward pass or carry lane
- `box_support`: how many realistic targets exist once the ball reaches the final third
- `rest_defense`: how much transition protection remains behind the ball
- `width_access`: how available wide exits and wide progression are

These should be derived from `style + formation`, then filtered through the active role and trait mix.

#### State transitions

Transitions are the highest-leverage tuning point in the current engine.

A transition should depend on:

- turnover zone
- outlet role on the recovering side
- transition directness of the recovering style
- rest-defense security of the opponent
- whether the previous defensive action was aggressive and got beaten

This is where `Counter` should become dangerous for the right reasons, and where `Possession` or `Defensive` should protect themselves structurally.

#### Role responsibility

Roles should decide which players are likely to take part in which internal actions.

Examples:

- `DeepPlaymaker`: first-phase progression and outlet responsibility
- `CenterBackPlaymaker`: split-line buildup access
- `WingBackAttack`: wide progression and late box support
- `LinkForward`: link actions before shots
- `Poacher`: box occupation rather than buildup responsibility

This is a better tuning lever than generic formation attack or defense buffs.

#### Failure modes

Strong systems must pay realistic costs.

Examples:

- `HighPress`: if the jump is beaten, the opponent should reach a better zone
- `Counter`: without outlet quality or runner support, recoveries should die into midfield instead of becoming instant chances
- `Possession`: better retention, but fewer direct box entries without enough box support
- `Defensive`: stronger shot suppression, but fewer proactive box entries of its own

### File map

The current engine tuning work should primarily happen in:

- [shared.rs](/home/quackson/Desktop/Coding/openfootmanager/src-tauri/crates/engine/src/shared.rs)
  - support-shape context
  - intent weighting
  - transition and box-entry helper functions
- [resolution.rs](/home/quackson/Desktop/Coding/openfootmanager/src-tauri/crates/engine/src/engine/resolution.rs)
  - instant-sim state transitions and failure aftermath
- [zone_resolution.rs](/home/quackson/Desktop/Coding/openfootmanager/src-tauri/crates/engine/src/live_match/zone_resolution.rs)
  - live/delegated mirror of the same rules

### Current implementation checklist

1. Add support-shape fields to `IntentContext`
2. Derive those fields from `style + formation`
3. Use support-shape fields in intent selection
4. Gate transition progression by outlet role and opponent rest defense
5. Gate box-entry jumps by support shape and whether an aggressive defense was beaten
6. Route goalkeeper fast distribution through the same transition gating
7. Keep tactical profiles as coarse flavor only, not the main balancing surface

### Benchmark workflow for this framework

After each tuning pass:

1. `cd src-tauri && cargo test -p engine`
2. `npm run build`
3. `bash scripts/record-engine-benchmark.sh --matches 500 --history-file /tmp/engine-history.jsonl --allow-dirty`
4. `bash scripts/render-tactic-matrix-benchmark.sh --matches-per-leg 20 --output-dir /tmp/ofm-tactic-bench --allow-dirty`
5. `bash scripts/render-fit-profile-matrix-benchmark.sh --matches-per-leg 12 --output-dir /tmp/ofm-fit-bench --allow-dirty`

Inspect:

- global event temperature
- style and formation spread
- fit sensitivity curves
- whether top systems are player-dependent or mechanically overpowered
