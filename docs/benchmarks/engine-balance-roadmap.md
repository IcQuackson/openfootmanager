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

Observed sample outcome after implementation:

- balanced benchmark sample stayed close to target at roughly `2.98` goals per match
- `4-2-3-1`, `4-5-1`, and `4-1-4-1` now separate in the tactic matrix instead of behaving as aliases
- shape spread is materially larger, with `4-2-3-1`, `4-5-1`, and `3-5-2` outperforming flatter structures in the current sample
- role-weighted selection now consistently favors higher-involvement specialists, which is covered by regression tests for ball-playing defenders

### Fix 4: Press, home, and shape correction pass

The current benchmark still has three clear issues:

- `HighPress` is the strongest style by too wide a margin
- `Defensive` is not strong enough as a low-event counter-style
- home advantage is still too large in equal-strength samples
- `3-4-3` is underperforming across multiple styles

Planned sequence:

1. nerf `HighPress` and buff `Defensive`
2. reduce home advantage
3. repair `3-4-3` semantics without collapsing shape identity again

Target outcomes for this pass:

- reduce `HighPress` average PPG so it is still competitive but not the default best style
- improve `Defensive` shot suppression and matchup viability, especially into aggressive setups
- move equal-strength home goals edge closer to the `+0.15` to `+0.35` target band
- lift `3-4-3` out of the bottom tier while keeping its risk/reward identity

Observed sample outcome after step 1:

- `HighPress` dropped from top style to the middle of the pack at roughly `1.36` average PPG
- `Defensive` became the strongest style in the sample at roughly `1.41` average PPG with lower goals against
- style spread compressed substantially, which is good
- home advantage and `3-4-3` underperformance were unchanged, so the remaining two steps are still needed

Observed sample outcome after step 2:

- equal-strength home goals edge moved from roughly `+0.52` to `+0.34`, which lands inside the target band
- home possession moved closer to neutral at roughly `50.6%`
- overall scoring stayed close to target at roughly `3.02` goals per match
- `3-4-3` underperformance still remained, so the final step is still needed

Observed sample outcome after step 3:

- `3-4-3` moved from the bottom shape to the middle of the table at roughly `1.37` average PPG
- `3-4-3` tactics now range from competitive to clearly viable instead of clustering in the bottom tier
- the balanced benchmark stayed unchanged, which is expected because it uses equal `4-4-2` teams and this step only touched `3-4-3`

### Fix 5: Trait-aware role analysis

The next engine phase should stop treating all defenders, midfielders, and forwards as interchangeable generic pieces.

Changes:

- remove engine-side player archetypes
- keep formation slot roles as the tactical instruction layer
- use player traits from [engine-traits-players.md](/home/quackson/Desktop/Coding/openfootmanager/docs/benchmarks/engine-traits-players.md) to bias actor selection and event outcomes
- refresh stored player traits from attributes during save-load repair and training updates

Expected result:

- player behavior becomes more individualized without introducing a second parallel role taxonomy
- tactical fit emerges from `attributes + traits + formation role + team style`
- future engine balancing can be guided by role and trait fit rather than formation modifiers alone

Observed sample outcome after implementation:

- balanced benchmark sample moved to roughly `4.22` goals per match, `10.70` home shots, `9.54` away shots
- defender pass attempts stayed healthy at roughly `20.39` per match and midfielders still dominated circulation at roughly `42.83`
- quick-distribution and rebound chains are now visible in event data and covered by regression tests
- the engine is now more behaviorally differentiated, but the first slice is too high-event and will need a follow-up cooling pass on rebound frequency, transition carry-through, and attacking style shot volume

### Fix 6: Trait-originated action phases

The next engine pass should stop treating traits primarily as multipliers and use them to originate specific match actions.

Detailed design note:

- [engine-trait-action-phases.md](/home/quackson/Desktop/Coding/openfootmanager/docs/benchmarks/engine-trait-action-phases.md)

Changes:

- add internal action intents behind the existing zone model
- compute lightweight pressure and score-state context per action
- let traits and formation roles bias intent selection
- add first aftermath chains for rebounds, quick keeper distribution, and pressure-bait progression
- keep the public event schema compact during this phase

Expected result:

- players with the right traits create distinct possession chains
- rebounds and quick transitions appear for structural reasons, not as raw stat buffs
- the engine becomes easier to extend toward richer commentary later

Observed sample outcome after implementation:

- balanced benchmark (`500` seeded `4-4-2 Balanced vs Balanced` matches) cooled to `2.21` goals per match with `5.50` home shots and `5.05` away shots
- defender circulation stayed healthy at `19.79` pass attempts per match, midfielders led volume at `38.44`, and forwards dropped to `7.31`
- tactic matrix (`20` matches per leg) shifted toward low-event structures:
  - best style: `Defensive` at `1.433` average PPG
  - weakest style: `HighPress` at `1.230`
  - best shape: `3-4-3` at `1.427`
  - weakest shape: `4-3-3` at `1.246`
- fit-profile matrix (`12` matches per leg) still shows the top attacking and counter systems as `player_dependent`, not mechanically overpowered:
  - `3-5-2 Counter`: `1.344 -> 1.625 -> 2.019`
  - `3-4-3 Counter`: `1.311 -> 1.634 -> 1.972`
  - `4-4-2 Counter`: `1.194 -> 1.525 -> 1.948`
- conclusion:
  - the new support-shape and transition gates make strength come from structure and player fit
  - generic flow is now too cold, so the next pass should reopen baseline shot creation without falling back to flat tactical bonuses

### Fix 7: Non-flat tuning framework

The next tuning pass should move more of the engine away from direct style modifiers and into structural context.

Detailed design note:

- [engine-tuning-framework.md](/home/quackson/Desktop/Coding/openfootmanager/docs/benchmarks/engine-tuning-framework.md)

Changes:

- add support-shape fields to `IntentContext`
- derive those support numbers from `style + formation`
- use those support numbers in intent weighting
- gate transition progression by outlet role and opponent rest defense
- gate box-entry jumps by support shape and whether an aggressive defense was beaten
- route goalkeeper fast distribution through the same transition logic

Expected result:

- systems should create different chains because they create different structural contexts
- strong systems should depend more on outlet quality, support shape, and transition protection
- benchmarks should become more informative about why a system is strong, not just whether it wins

Observed sample outcome after implementation:

- balanced baseline cooled to `2.21` goals per match with `5.50` home shots and `5.05` away shots
- tactic matrix over-rewarded low-event structures, with `Defensive` leading at `1.433` average PPG
- fit-profile matrix still showed top counter systems as `player_dependent`, not mechanically overpowered

### Fix 8: Reopen baseline chance creation

The structural gates from Fix 7 were directionally correct, but too restrictive for generic flow and single-striker systems.

Changes:

- eased transition and box-entry penalties against opponent rest defense
- increased generic progression and box-support context slightly
- lifted `4-4-2`, `4-2-3-1`, `4-5-1`, and `4-1-4-1` support/box-presence profiles
- improved box support contribution for `LinkForward`, `AdvancedPlaymaker`, and `FullBackSupport`

Expected result:

- bring balanced baseline back into the roadmap target band
- improve `4-4-2` and single-striker shape viability without using flat tactical bonuses
- preserve the player-dependent nature of the strongest counter and attacking systems

Observed sample outcome after implementation:

- balanced benchmark (`500` seeded `4-4-2 Balanced vs Balanced` matches) recovered from `2.21` to `2.95` goals per match
- shot volume recovered from `5.50 / 5.05` to `7.66 / 6.84` home/away shots
- tactic matrix (`20` matches per leg) now favors stronger support shapes instead of only low-event defensive suppression:
  - best style remains `Defensive` at `1.455`, but `Possession`, `Balanced`, and `Counter` sit much closer
  - best shape is now `4-2-3-1` at `1.504`
  - `4-5-1` improved from `1.314` to `1.418`
  - `4-1-4-1` improved from `1.323` to `1.383`
  - `4-4-2` improved from `1.284` to `1.349`
- fit-profile matrix still shows no mechanically overpowered watchlist entries
- top systems remain player-dependent:
  - `4-4-2 Counter`: `1.223 -> 1.572 -> 2.099`
  - `3-5-2 Counter`: `1.286 -> 1.579 -> 2.097`
  - `3-4-3 Counter`: `1.189 -> 1.606 -> 1.965`

### Fix 9: HighPress fit payoff and weak-shape repair

This pass stays inside the current philosophy and targets two concrete issues:

- `HighPress` should get a clearer payoff when the squad actually fits it
- `4-3-3` and `5-3-2` should stop lagging behind other generic shapes

Changes:

- increased `HighPress` proactive defensive intent when the player has pressing-friendly traits
- reduced passive defensive fallback for high-press-fit players
- increased `WingBackAttack` and `WideProgressor` support influence slightly
- raised `4-3-3` support/box-presence and normalized its rest defense
- raised `5-3-2` buildup/support/box-presence while keeping its defensive identity

Expected result:

- ideal-fit `HighPress` systems should open a clearer gap over their bad-fit versions
- average `HighPress` should still stay below the top generic styles
- `4-3-3` and `5-3-2` should move out of the bottom two generic shapes

Observed sample outcome after implementation:

- balanced `4-4-2` baseline stayed unchanged at `2.95` goals per match, which is expected because this pass did not target that scenario
- `4-3-3` generic shape improved from `1.242` to `1.346` average PPG
- `5-3-2` generic shape improved from `1.236` to `1.330` average PPG
- `HighPress` remained weak on average (`1.253 -> 1.248`), so it did not become flatly strong
- ideal-fit `HighPress` payoff improved in the targeted shapes:
  - `4-3-3 HighPress`: `1.097 -> 1.486 -> 1.678` with `ideal-bad` gap `0.496`
  - `5-3-2 HighPress`: `1.283 -> 1.468 -> 1.672` with `ideal-bad` gap `0.389`
- mechanically overpowered watchlist remained empty

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

## Benchmark roadmap extension

The engine now needs a second benchmark family beyond the generic tactic matrix.

### Role/trait blueprint matrix

Purpose:

- test `formation + play style + formation slot roles + player trait packages` together
- surface exploit candidates that only appear when specific role/trait mixes are present
- identify which blueprints counter those pressure points
- expose role-level fingerprints so engine tuning can target the real source of the exploit

Artifacts:

- `engine-role-trait-matrix.json`
- `engine-role-trait-rankings.md`
- `engine-role-trait-exploits.md`
- `engine-role-trait-matchups.md`

Workflow:

1. run `bash scripts/render-role-trait-matrix-benchmark.sh --matches-per-leg 25 --allow-dirty`
2. inspect top blueprints in the exploit watchlist
3. inspect the role fingerprints driving those blueprints
4. compare their listed counters
5. tune the engine where the role fingerprint explains the exploit, not just the final style label

### Fit-profile matrix

Purpose:

- test whether a system is strong because it has the right players or because the engine is over-rewarding it regardless of fit
- compare `BadFit`, `NeutralFit`, and `IdealFit` versions of the same system
- measure counter-response against those fit levels
- separate genuinely player-dependent systems from mechanically overpowered ones

Artifacts:

- `engine-fit-profile-matrix.json`
- `engine-fit-profile-rankings.md`
- `engine-fit-profile-analysis.md`

Workflow:

1. run `bash scripts/render-fit-profile-matrix-benchmark.sh --matches-per-leg 12 --allow-dirty`
2. inspect the fit curve for each system: `BadFit -> NeutralFit -> IdealFit`
3. treat high-PPG systems with small fit gaps as exploit candidates
4. treat high-PPG systems with large fit gaps as healthier player-dependent systems
5. inspect the ideal-fit counter tables before changing style or formation logic

## Queued next items

The next benchmark/reporting pass should cover these concrete follow-ups:

1. commit the generated benchmark report files currently produced under `docs/benchmarks`
2. tune generic-shape ordering for `4-4-2` and `3-5-2` without falling back to flat tactic-strength modifiers
3. make `HighPress` counters more legible in the generated reports so the benchmark output explains not just that `HighPress` loses, but which systems and fit profiles are suppressing it

Success criteria for this queued pass:

- benchmark command scaffolding remains tracked, and the generated report outputs are intentionally versioned
- generic-shape rankings better reflect the intended place of `4-4-2` and `3-5-2`
- `HighPress` report sections include clearer counter relationships instead of forcing manual inspection of raw matchup tables

## Success criteria

The balancing pass is considered successful when:

- benchmark match stats land inside the target bands
- `Possession` is no longer the best style in every shape by default
- `Counter` has at least some favorable matchups into aggressive or possession-heavy styles
- `Defensive` visibly lowers shot concession
- formation changes matter more than they do now
- player role stat distributions look believable in aggregate
