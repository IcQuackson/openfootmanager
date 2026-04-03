# Trait-Originated Action Phases

## Goal

Move traits away from raw attribute buffs and make them originate distinct match behavior.

The engine should answer:

- who gets involved in a phase
- what they are trying to do
- how traits change the chosen action
- what kind of aftermath the action creates

This keeps the public match event model compact while letting the internal simulation become more football-like.

## Phase 1: Internal action intents

Keep the current zone model and add internal action intents behind it.

The engine should choose an `ActionIntent` before resolving a duel or pass. Traits and formation roles should bias which intent is selected.

Initial intent families:

- buildup:
  - `SafeRecyclePass`
  - `SplitLinePass`
  - `BaitPressTouch`
- midfield:
  - `OneTouchCombination`
  - `DelayedRelease`
  - `ProgressiveCarry`
  - `ThirdManLayoff`
  - `PressEscapeTurn`
- final third:
  - `LinkPlaySlip`
  - `DirectDribble`
  - `BlindSideRun`
  - `LateBoxDelivery`
  - `NearPostAttack`
- defending:
  - `StepInInterception`
  - `ContainAndShowWide`
  - `TacticalFoulStop`
  - `AerialClearance`
- goalkeeper aftermath:
  - `SecureClaim`
  - `LaunchThrow`

### First implementation slice

The first engine slice does not expose new public event types.

It uses existing surfaced events:

- `PassCompleted`
- `PassIntercepted`
- `Dribble`
- `DribbleTackled`
- `Cross`
- `Interception`
- `Tackle`
- `Clearance`
- `Foul`
- `ShotSaved`
- `ShotBlocked`
- `Goal`
- `GoalKick`

The intent system only changes how those events are produced and chained.

## Phase 2: Pressure and sequence context

Intent selection should depend on match context, not just player traits.

Initial context flags:

- `under_pressure`
- `in_transition`
- `late_game`
- `protecting_lead`
- `chasing_game`

This lets the same player behave differently depending on game state.

Examples:

- `PressBaiter` matters more under pressure in buildup
- `TacticalFouler` matters more when defending transition
- `ClutchExecutor` and `TimeKiller` matter more late

## Phase 3: Aftermath chains

Some actions should create a second consequence instead of immediately resetting the phase.

First aftermath chains:

- saved shot -> rebound attack
- blocked shot -> loose-box rebound
- goalkeeper save -> quick throw launch
- successful press bait -> accelerated progression

These chains should still be represented through existing public events in the first slice.

## Phase 4: Visible commentary/report expansion

Only after the internal model is stable should more of these micro-actions become explicit UI events.

The current plan is to keep internal richness higher than public event granularity.

That avoids flooding match reports with noise while still improving match behavior and player identity.

## First targeted traits

The first trait-driven action slice focuses on traits that visibly change possession flow:

- `PressBaiter`
- `DelayedPasser`
- `OneTouchSpecialist`
- `BlindSideRunner`
- `LateBoxArriver`
- `PassingLaneThief`
- `ContainmentSpecialist`
- `TacticalFouler`
- `ThrowLauncher`
- `ReboundInstinct`

## Success criteria

The first slice is successful when:

- traits change event chains, not just scalar success rates
- quick-transition and rebound sequences appear in benchmarks
- live and instant simulation remain aligned
- benchmark output stays inside the current global plausibility bands
