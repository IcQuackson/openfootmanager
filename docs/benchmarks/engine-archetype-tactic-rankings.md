# Engine Archetype Tactic Rankings

Recorded: 2026-04-03T15:19:06Z
Commit: f71e82d (custom)
Commit date: 2026-04-03T16:18:41+01:00
Dirty tracked worktree: no
Matches per leg: 50

## Notes

- Each unordered matchup is simulated home and away to reduce home-advantage bias.
- Squads use benchmark-only archetype templates so tactic analysis includes player profile fit, not just raw formation and play style.
- Template matchup sections include both exact opponent templates and opponent squad-type aggregates.

## Style Summary

| Style | Avg PPG | Avg GD | Avg GF | Avg GA | Avg Poss% |
| --- | --- | --- | --- | --- | --- |
| `Balanced` | 1.554 | 0.353 | 1.927 | 1.575 | 49.8 |
| `Attacking` | 1.464 | 0.114 | 2.536 | 2.422 | 50.2 |
| `Defensive` | 1.463 | 0.183 | 1.097 | 0.914 | 50.3 |
| `Counter` | 1.420 | 0.084 | 1.274 | 1.190 | 47.9 |
| `Possession` | 1.322 | -0.045 | 1.229 | 1.273 | 53.0 |
| `HighPress` | 1.098 | -0.513 | 1.654 | 2.167 | 48.7 |

## Shape Summary

| Shape | Avg PPG | Avg GD | Avg GF | Avg GA | Avg Poss% |
| --- | --- | --- | --- | --- | --- |
| `4-4-2` | 1.554 | 0.353 | 1.927 | 1.575 | 49.8 |
| `4-2-3-1` | 1.525 | 0.251 | 2.068 | 1.817 | 52.1 |
| `3-5-2` | 1.504 | 0.187 | 1.341 | 1.154 | 49.3 |
| `4-1-4-1` | 1.500 | 0.224 | 1.111 | 0.887 | 50.5 |
| `4-5-1` | 1.360 | 0.026 | 1.147 | 1.120 | 51.5 |
| `5-3-2` | 1.336 | -0.019 | 1.207 | 1.226 | 46.5 |
| `3-4-3` | 1.245 | -0.245 | 2.030 | 2.275 | 49.2 |
| `4-3-3` | 0.936 | -0.808 | 1.430 | 2.238 | 48.3 |

## Squad Summary

| Squad Type | Avg PPG | Avg GD | Avg GF | Avg GA | Avg Poss% |
| --- | --- | --- | --- | --- | --- |
| `Creator Ten` | 1.697 | 0.501 | 2.890 | 2.389 | 51.0 |
| `Classic Pair` | 1.554 | 0.353 | 1.927 | 1.575 | 49.8 |
| `Low Block Outlet` | 1.463 | 0.183 | 1.097 | 0.914 | 50.3 |
| `Transition Core` | 1.420 | 0.084 | 1.274 | 1.190 | 47.9 |
| `Control Spine` | 1.322 | -0.045 | 1.229 | 1.273 | 53.0 |
| `Wing Surge` | 1.231 | -0.274 | 2.182 | 2.455 | 49.5 |
| `Pressing Wave` | 1.098 | -0.513 | 1.654 | 2.167 | 48.7 |

## Overall Template Ranking

| Rank | Template | Shape | Style | Squad | PPG | GD | GF | GA | Best Squad To Face | Worst Squad To Face |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| 1 | `4-2-3-1__Attacking__CreatorTen` | `4-2-3-1` | `Attacking` | `Creator Ten` | 1.697 | 0.501 | 2.890 | 2.389 | `Wing Surge` (2.170) | `Low Block Outlet` (1.410) |
| 2 | `4-4-2__Balanced__ClassicPair` | `4-4-2` | `Balanced` | `Classic Pair` | 1.554 | 0.353 | 1.927 | 1.575 | `Wing Surge` (2.030) | `Creator Ten` (1.340) |
| 3 | `3-5-2__Counter__TransitionCore` | `3-5-2` | `Counter` | `Transition Core` | 1.504 | 0.187 | 1.341 | 1.154 | `Pressing Wave` (1.810) | `Creator Ten` (1.240) |
| 4 | `4-1-4-1__Defensive__LowBlockOutlet` | `4-1-4-1` | `Defensive` | `Low Block Outlet` | 1.500 | 0.224 | 1.111 | 0.887 | `Pressing Wave` (1.725) | `Classic Pair` (1.150) |
| 5 | `4-5-1__Defensive__LowBlockOutlet` | `4-5-1` | `Defensive` | `Low Block Outlet` | 1.426 | 0.143 | 1.083 | 0.940 | `Pressing Wave` (1.910) | `Low Block Outlet` (1.070) |
| 6 | `4-2-3-1__Possession__ControlSpine` | `4-2-3-1` | `Possession` | `Control Spine` | 1.352 | 0.001 | 1.246 | 1.245 | `Pressing Wave` (1.565) | `Classic Pair` (1.230) |
| 7 | `5-3-2__Counter__TransitionCore` | `5-3-2` | `Counter` | `Transition Core` | 1.336 | -0.019 | 1.207 | 1.226 | `Pressing Wave` (1.690) | `Creator Ten` (1.160) |
| 8 | `4-5-1__Possession__ControlSpine` | `4-5-1` | `Possession` | `Control Spine` | 1.293 | -0.090 | 1.211 | 1.301 | `Pressing Wave` (1.510) | `Creator Ten` (1.130) |
| 9 | `3-4-3__HighPress__PressingWave` | `3-4-3` | `HighPress` | `Pressing Wave` | 1.260 | -0.217 | 1.878 | 2.095 | `Pressing Wave` (1.510) | `Creator Ten` (1.050) |
| 10 | `3-4-3__Attacking__WingSurge` | `3-4-3` | `Attacking` | `Wing Surge` | 1.231 | -0.274 | 2.182 | 2.455 | `Pressing Wave` (1.650) | `Creator Ten` (0.730) |
| 11 | `4-3-3__HighPress__PressingWave` | `4-3-3` | `HighPress` | `Pressing Wave` | 0.936 | -0.808 | 1.430 | 2.238 | `Pressing Wave` (1.240) | `Creator Ten` (0.510) |
