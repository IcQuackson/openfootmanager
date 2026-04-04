# Engine Fit Profile Analysis

Recorded: 2026-04-03T21:37:12Z
Commit: 6036f4c (custom)
Matches per leg: 12

This report tests whether a system only becomes strong when the squad is tailored to it, or whether it stays too strong even with the wrong players. Each section is keyed off the ideal-fit version of the system, then shows how that ideal-fit system performs against bad-fit, neutral-fit, and ideal-fit versions of the opponent systems.

## 3-5-2__Counter

- Formation: `3-5-2`
- Style: `Counter`
- Target package: `LaunchCounter`
- Classification: `player_dependent`
- Fit curve: bad 1.336 PPG, neutral 1.746, ideal 2.238
- Fit deltas: ideal-bad 0.902, ideal-neutral 0.492, neutral-bad 0.410

Profiles:

| Profile | PPG | GF | GA | GD | Poss% |
| --- | --- | --- | --- | --- | --- |
| `BadFit` | 1.336 | 1.870 | 2.027 | -0.158 | 48.3 |
| `NeutralFit` | 1.746 | 2.473 | 1.846 | 0.627 | 48.8 |
| `IdealFit` | 2.238 | 3.390 | 1.594 | 1.795 | 48.9 |

Ideal-fit role fingerprints:

| Role | Min | Goals | Shots | Passes C | Pass% | Tackles | Interceptions | Rating |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| `Poacher` | 94.0 | 2.03 | 5.66 | 0.81 | 70.0 | 0.00 | 0.00 | 9.01 |
| `TargetForward` | 94.0 | 1.36 | 5.11 | 0.82 | 70.1 | 0.00 | 0.00 | 8.51 |
| `AdvancedPlaymaker` | 93.9 | 0.00 | 0.00 | 3.41 | 79.5 | 2.39 | 2.44 | 7.65 |
| `BoxToBoxMidfielder` | 93.9 | 0.00 | 0.00 | 2.65 | 74.1 | 2.50 | 2.40 | 7.53 |
| `HoldingMidfielder` | 94.0 | 0.00 | 0.00 | 2.57 | 74.5 | 2.19 | 2.21 | 7.47 |
| `WideProgressor` | 94.0 | 0.00 | 0.00 | 2.32 | 75.0 | 2.18 | 2.21 | 7.44 |

Ideal-fit punishes:

| Opponent | Formation | Style | Vs Bad | Vs Neutral | Vs Ideal | Ideal GD |
| --- | --- | --- | --- | --- | --- | --- |
| `4-1-4-1__Attacking` | `4-1-4-1` | `Attacking` | 2.708 | 2.083 | 2.708 | 2.708 |
| `4-2-3-1__HighPress` | `4-2-3-1` | `HighPress` | 2.792 | 2.375 | 2.667 | 2.542 |
| `4-2-3-1__Attacking` | `4-2-3-1` | `Attacking` | 2.542 | 2.833 | 2.583 | 2.750 |
| `4-1-4-1__Balanced` | `4-1-4-1` | `Balanced` | 2.833 | 2.500 | 2.583 | 2.667 |
| `4-5-1__Balanced` | `4-5-1` | `Balanced` | 2.792 | 2.542 | 2.583 | 2.250 |

Ideal-fit counters:

| Opponent | Formation | Style | Vs Bad | Vs Neutral | Vs Ideal | Ideal GD |
| --- | --- | --- | --- | --- | --- | --- |
| `4-4-2__Counter` | `4-4-2` | `Counter` | 2.417 | 2.292 | 1.042 | -0.583 |
| `4-3-3__Counter` | `4-3-3` | `Counter` | 2.792 | 2.000 | 1.208 | -0.042 |
| `3-5-2__Possession` | `3-5-2` | `Possession` | 2.292 | 1.333 | 1.292 | -0.250 |
| `4-4-2__Defensive` | `4-4-2` | `Defensive` | 2.000 | 1.250 | 1.375 | 0.250 |
| `5-3-2__Possession` | `5-3-2` | `Possession` | 2.042 | 1.542 | 1.500 | 0.167 |

## 4-4-2__Counter

- Formation: `4-4-2`
- Style: `Counter`
- Target package: `LaunchCounter`
- Classification: `player_dependent`
- Fit curve: bad 1.177 PPG, neutral 1.700, ideal 2.217
- Fit deltas: ideal-bad 1.040, ideal-neutral 0.517, neutral-bad 0.523

Profiles:

| Profile | PPG | GF | GA | GD | Poss% |
| --- | --- | --- | --- | --- | --- |
| `BadFit` | 1.177 | 1.778 | 2.183 | -0.405 | 46.7 |
| `NeutralFit` | 1.700 | 2.422 | 1.923 | 0.499 | 47.7 |
| `IdealFit` | 2.217 | 3.364 | 1.654 | 1.710 | 48.3 |

Ideal-fit role fingerprints:

| Role | Min | Goals | Shots | Passes C | Pass% | Tackles | Interceptions | Rating |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| `Poacher` | 94.0 | 2.03 | 5.64 | 0.78 | 68.2 | 0.00 | 0.00 | 9.00 |
| `TargetForward` | 94.0 | 1.33 | 5.04 | 0.83 | 69.6 | 0.00 | 0.00 | 8.48 |
| `DeepPlaymaker` | 93.9 | 0.00 | 0.00 | 4.62 | 81.9 | 3.17 | 3.69 | 8.13 |
| `BoxToBoxMidfielder` | 94.0 | 0.00 | 0.00 | 3.04 | 72.8 | 2.90 | 2.80 | 7.72 |
| `WideProgressor` | 93.9 | 0.00 | 0.00 | 2.68 | 72.9 | 2.58 | 2.52 | 7.63 |
| `SweeperKeeper` | 94.0 | 0.00 | 0.00 | 1.44 | 100.0 | 0.00 | 0.00 | 6.55 |

Ideal-fit punishes:

| Opponent | Formation | Style | Vs Bad | Vs Neutral | Vs Ideal | Ideal GD |
| --- | --- | --- | --- | --- | --- | --- |
| `4-5-1__HighPress` | `4-5-1` | `HighPress` | 2.833 | 2.583 | 2.792 | 2.458 |
| `4-2-3-1__Balanced` | `4-2-3-1` | `Balanced` | 2.708 | 2.750 | 2.708 | 2.375 |
| `4-1-4-1__HighPress` | `4-1-4-1` | `HighPress` | 2.792 | 2.333 | 2.708 | 2.375 |
| `4-5-1__Balanced` | `4-5-1` | `Balanced` | 2.708 | 2.708 | 2.667 | 2.625 |
| `4-1-4-1__Balanced` | `4-1-4-1` | `Balanced` | 2.917 | 2.250 | 2.625 | 2.500 |

Ideal-fit counters:

| Opponent | Formation | Style | Vs Bad | Vs Neutral | Vs Ideal | Ideal GD |
| --- | --- | --- | --- | --- | --- | --- |
| `5-3-2__Counter` | `5-3-2` | `Counter` | 2.167 | 1.750 | 1.292 | -0.375 |
| `5-3-2__Defensive` | `5-3-2` | `Defensive` | 2.208 | 1.750 | 1.333 | -0.042 |
| `3-4-3__Counter` | `3-4-3` | `Counter` | 2.375 | 1.333 | 1.417 | -0.292 |
| `3-5-2__Possession` | `3-5-2` | `Possession` | 1.583 | 1.708 | 1.417 | 0.250 |
| `4-3-3__Defensive` | `4-3-3` | `Defensive` | 2.458 | 1.875 | 1.667 | 0.250 |

## 5-3-2__Counter

- Formation: `5-3-2`
- Style: `Counter`
- Target package: `LaunchCounter`
- Classification: `player_dependent`
- Fit curve: bad 1.232 PPG, neutral 1.630, ideal 2.150
- Fit deltas: ideal-bad 0.918, ideal-neutral 0.521, neutral-bad 0.398

Profiles:

| Profile | PPG | GF | GA | GD | Poss% |
| --- | --- | --- | --- | --- | --- |
| `BadFit` | 1.232 | 1.760 | 2.056 | -0.296 | 46.4 |
| `NeutralFit` | 1.630 | 2.301 | 1.870 | 0.431 | 46.8 |
| `IdealFit` | 2.150 | 3.160 | 1.644 | 1.516 | 46.9 |

Ideal-fit role fingerprints:

| Role | Min | Goals | Shots | Passes C | Pass% | Tackles | Interceptions | Rating |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| `Poacher` | 94.0 | 1.89 | 5.31 | 0.72 | 67.4 | 0.00 | 0.00 | 8.91 |
| `TargetForward` | 94.0 | 1.27 | 4.72 | 0.79 | 69.9 | 0.00 | 0.00 | 8.39 |
| `AdvancedPlaymaker` | 93.9 | 0.00 | 0.00 | 5.24 | 78.4 | 4.01 | 4.01 | 8.33 |
| `BoxToBoxMidfielder` | 93.8 | 0.00 | 0.00 | 3.97 | 71.9 | 4.01 | 3.89 | 8.15 |
| `HoldingMidfielder` | 93.9 | 0.00 | 0.00 | 3.90 | 73.0 | 3.59 | 3.59 | 8.05 |
| `SweeperKeeper` | 94.0 | 0.00 | 0.00 | 1.40 | 100.0 | 0.00 | 0.00 | 6.55 |

Ideal-fit punishes:

| Opponent | Formation | Style | Vs Bad | Vs Neutral | Vs Ideal | Ideal GD |
| --- | --- | --- | --- | --- | --- | --- |
| `4-5-1__HighPress` | `4-5-1` | `HighPress` | 2.542 | 2.792 | 2.708 | 2.875 |
| `4-2-3-1__HighPress` | `4-2-3-1` | `HighPress` | 2.875 | 2.208 | 2.583 | 2.750 |
| `4-1-4-1__Balanced` | `4-1-4-1` | `Balanced` | 2.625 | 3.000 | 2.542 | 2.250 |
| `4-2-3-1__Balanced` | `4-2-3-1` | `Balanced` | 3.000 | 2.708 | 2.542 | 2.208 |
| `4-2-3-1__Counter` | `4-2-3-1` | `Counter` | 2.542 | 2.292 | 2.542 | 1.500 |

Ideal-fit counters:

| Opponent | Formation | Style | Vs Bad | Vs Neutral | Vs Ideal | Ideal GD |
| --- | --- | --- | --- | --- | --- | --- |
| `5-3-2__Defensive` | `5-3-2` | `Defensive` | 1.792 | 1.417 | 0.833 | -0.625 |
| `3-5-2__Counter` | `3-5-2` | `Counter` | 2.542 | 2.042 | 1.125 | -0.708 |
| `4-3-3__Defensive` | `4-3-3` | `Defensive` | 2.167 | 2.042 | 1.125 | -0.250 |
| `3-5-2__Balanced` | `3-5-2` | `Balanced` | 2.667 | 1.458 | 1.208 | 0.083 |
| `3-4-3__Counter` | `3-4-3` | `Counter` | 2.375 | 1.625 | 1.250 | -0.125 |

## 3-4-3__Counter

- Formation: `3-4-3`
- Style: `Counter`
- Target package: `LaunchCounter`
- Classification: `player_dependent`
- Fit curve: bad 1.200 PPG, neutral 1.684, ideal 2.145
- Fit deltas: ideal-bad 0.944, ideal-neutral 0.461, neutral-bad 0.484

Profiles:

| Profile | PPG | GF | GA | GD | Poss% |
| --- | --- | --- | --- | --- | --- |
| `BadFit` | 1.200 | 1.712 | 2.097 | -0.385 | 46.7 |
| `NeutralFit` | 1.684 | 2.440 | 1.895 | 0.546 | 47.6 |
| `IdealFit` | 2.145 | 3.008 | 1.583 | 1.426 | 48.4 |

Ideal-fit role fingerprints:

| Role | Min | Goals | Shots | Passes C | Pass% | Tackles | Interceptions | Rating |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| `Poacher` | 94.0 | 1.39 | 3.85 | 0.55 | 70.7 | 0.00 | 0.00 | 8.44 |
| `DeepPlaymaker` | 94.0 | 0.00 | 0.00 | 4.56 | 82.1 | 3.12 | 3.65 | 8.06 |
| `ChannelRunner` | 94.0 | 0.81 | 3.77 | 0.62 | 72.8 | 0.00 | 0.00 | 7.84 |
| `BoxToBoxMidfielder` | 93.9 | 0.00 | 0.00 | 3.00 | 72.9 | 2.90 | 2.78 | 7.61 |
| `WideProgressor` | 94.0 | 0.00 | 0.00 | 2.71 | 74.0 | 2.56 | 2.55 | 7.57 |
| `SweeperKeeper` | 94.0 | 0.00 | 0.00 | 1.38 | 100.0 | 0.00 | 0.00 | 6.55 |

Ideal-fit punishes:

| Opponent | Formation | Style | Vs Bad | Vs Neutral | Vs Ideal | Ideal GD |
| --- | --- | --- | --- | --- | --- | --- |
| `4-2-3-1__Attacking` | `4-2-3-1` | `Attacking` | 2.750 | 2.542 | 2.917 | 2.708 |
| `4-2-3-1__Balanced` | `4-2-3-1` | `Balanced` | 2.917 | 2.917 | 2.917 | 2.625 |
| `4-5-1__Attacking` | `4-5-1` | `Attacking` | 2.500 | 2.417 | 2.750 | 2.875 |
| `4-2-3-1__HighPress` | `4-2-3-1` | `HighPress` | 2.833 | 2.458 | 2.667 | 1.833 |
| `4-1-4-1__Attacking` | `4-1-4-1` | `Attacking` | 2.417 | 2.917 | 2.542 | 2.917 |

Ideal-fit counters:

| Opponent | Formation | Style | Vs Bad | Vs Neutral | Vs Ideal | Ideal GD |
| --- | --- | --- | --- | --- | --- | --- |
| `3-5-2__Counter` | `3-5-2` | `Counter` | 1.833 | 1.958 | 1.125 | -1.000 |
| `4-4-2__Defensive` | `4-4-2` | `Defensive` | 1.625 | 2.250 | 1.125 | -0.125 |
| `3-4-3__Attacking` | `3-4-3` | `Attacking` | 2.792 | 2.083 | 1.250 | -0.458 |
| `3-4-3__Possession` | `3-4-3` | `Possession` | 1.708 | 1.583 | 1.333 | 0.042 |
| `5-3-2__Defensive` | `5-3-2` | `Defensive` | 1.958 | 1.375 | 1.458 | 0.292 |

## 3-5-2__Attacking

- Formation: `3-5-2`
- Style: `Attacking`
- Target package: `ChaosRaiders`
- Classification: `player_dependent`
- Fit curve: bad 1.284 PPG, neutral 1.700, ideal 2.121
- Fit deltas: ideal-bad 0.837, ideal-neutral 0.421, neutral-bad 0.416

Profiles:

| Profile | PPG | GF | GA | GD | Poss% |
| --- | --- | --- | --- | --- | --- |
| `BadFit` | 1.284 | 2.187 | 2.467 | -0.280 | 50.0 |
| `NeutralFit` | 1.700 | 3.036 | 2.432 | 0.604 | 50.4 |
| `IdealFit` | 2.121 | 4.009 | 2.375 | 1.633 | 50.6 |

Ideal-fit role fingerprints:

| Role | Min | Goals | Shots | Passes C | Pass% | Tackles | Interceptions | Rating |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| `Poacher` | 94.0 | 2.48 | 6.95 | 0.83 | 68.6 | 0.00 | 0.00 | 9.32 |
| `TargetForward` | 94.0 | 1.53 | 6.27 | 1.08 | 69.8 | 0.00 | 0.00 | 8.72 |
| `AdvancedPlaymaker` | 93.9 | 0.00 | 0.00 | 4.57 | 81.8 | 3.01 | 2.89 | 7.94 |
| `BoxToBoxMidfielder` | 93.9 | 0.00 | 0.00 | 3.52 | 75.6 | 2.88 | 2.83 | 7.78 |
| `HoldingMidfielder` | 93.8 | 0.00 | 0.00 | 3.27 | 76.0 | 2.48 | 2.43 | 7.62 |
| `WideProgressor` | 93.9 | 0.00 | 0.00 | 2.86 | 75.7 | 2.37 | 2.35 | 7.58 |

Ideal-fit punishes:

| Opponent | Formation | Style | Vs Bad | Vs Neutral | Vs Ideal | Ideal GD |
| --- | --- | --- | --- | --- | --- | --- |
| `4-2-3-1__Attacking` | `4-2-3-1` | `Attacking` | 2.583 | 2.750 | 2.750 | 4.458 |
| `4-1-4-1__Attacking` | `4-1-4-1` | `Attacking` | 2.667 | 3.000 | 2.583 | 3.417 |
| `4-4-2__HighPress` | `4-4-2` | `HighPress` | 2.125 | 2.167 | 2.583 | 2.292 |
| `4-1-4-1__Balanced` | `4-1-4-1` | `Balanced` | 2.583 | 2.333 | 2.542 | 2.000 |
| `4-5-1__Balanced` | `4-5-1` | `Balanced` | 2.792 | 2.708 | 2.500 | 2.250 |

Ideal-fit counters:

| Opponent | Formation | Style | Vs Bad | Vs Neutral | Vs Ideal | Ideal GD |
| --- | --- | --- | --- | --- | --- | --- |
| `4-4-2__Counter` | `4-4-2` | `Counter` | 2.458 | 1.458 | 0.833 | -1.542 |
| `3-5-2__Counter` | `3-5-2` | `Counter` | 1.875 | 2.000 | 0.958 | -1.125 |
| `5-3-2__Counter` | `5-3-2` | `Counter` | 2.625 | 1.792 | 1.167 | -0.542 |
| `4-3-3__Counter` | `4-3-3` | `Counter` | 2.292 | 2.417 | 1.167 | -0.083 |
| `3-5-2__Defensive` | `3-5-2` | `Defensive` | 1.625 | 1.583 | 1.208 | -0.292 |

## 5-3-2__Attacking

- Formation: `5-3-2`
- Style: `Attacking`
- Target package: `ChaosRaiders`
- Classification: `player_dependent`
- Fit curve: bad 1.183 PPG, neutral 1.554, ideal 2.026
- Fit deltas: ideal-bad 0.843, ideal-neutral 0.472, neutral-bad 0.370

Profiles:

| Profile | PPG | GF | GA | GD | Poss% |
| --- | --- | --- | --- | --- | --- |
| `BadFit` | 1.183 | 2.044 | 2.521 | -0.477 | 48.5 |
| `NeutralFit` | 1.554 | 2.793 | 2.510 | 0.284 | 48.5 |
| `IdealFit` | 2.026 | 3.738 | 2.397 | 1.340 | 49.0 |

Ideal-fit role fingerprints:

| Role | Min | Goals | Shots | Passes C | Pass% | Tackles | Interceptions | Rating |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| `Poacher` | 94.0 | 2.24 | 6.46 | 0.79 | 68.3 | 0.00 | 0.00 | 9.17 |
| `TargetForward` | 94.0 | 1.50 | 5.91 | 0.99 | 69.8 | 0.00 | 0.00 | 8.66 |
| `AdvancedPlaymaker` | 93.8 | 0.00 | 0.00 | 6.84 | 79.5 | 4.77 | 4.65 | 8.63 |
| `BoxToBoxMidfielder` | 93.8 | 0.00 | 0.00 | 5.24 | 73.8 | 4.73 | 4.43 | 8.45 |
| `HoldingMidfielder` | 93.8 | 0.00 | 0.00 | 4.99 | 74.0 | 3.96 | 3.90 | 8.25 |
| `Goalkeeper` | 94.0 | 0.00 | 0.00 | 1.32 | 100.0 | 0.00 | 0.00 | 6.54 |

Ideal-fit punishes:

| Opponent | Formation | Style | Vs Bad | Vs Neutral | Vs Ideal | Ideal GD |
| --- | --- | --- | --- | --- | --- | --- |
| `4-1-4-1__Attacking` | `4-1-4-1` | `Attacking` | 2.667 | 2.542 | 2.917 | 3.042 |
| `4-2-3-1__Attacking` | `4-2-3-1` | `Attacking` | 2.667 | 2.500 | 2.625 | 2.542 |
| `4-1-4-1__Defensive` | `4-1-4-1` | `Defensive` | 2.208 | 2.333 | 2.625 | 1.917 |
| `4-5-1__Attacking` | `4-5-1` | `Attacking` | 2.750 | 2.542 | 2.583 | 3.833 |
| `4-2-3-1__HighPress` | `4-2-3-1` | `HighPress` | 2.125 | 2.208 | 2.583 | 2.333 |

Ideal-fit counters:

| Opponent | Formation | Style | Vs Bad | Vs Neutral | Vs Ideal | Ideal GD |
| --- | --- | --- | --- | --- | --- | --- |
| `4-4-2__Counter` | `4-4-2` | `Counter` | 2.083 | 1.583 | 0.792 | -1.250 |
| `3-4-3__Counter` | `3-4-3` | `Counter` | 2.292 | 1.583 | 1.000 | -0.792 |
| `3-5-2__Counter` | `3-5-2` | `Counter` | 1.833 | 1.167 | 1.125 | -0.958 |
| `4-3-3__Counter` | `4-3-3` | `Counter` | 2.042 | 2.000 | 1.167 | -0.667 |
| `5-3-2__Counter` | `5-3-2` | `Counter` | 1.917 | 1.917 | 1.167 | -0.333 |

## 4-3-3__Counter

- Formation: `4-3-3`
- Style: `Counter`
- Target package: `LaunchCounter`
- Classification: `player_dependent`
- Fit curve: bad 1.166 PPG, neutral 1.593, ideal 1.963
- Fit deltas: ideal-bad 0.797, ideal-neutral 0.370, neutral-bad 0.427

Profiles:

| Profile | PPG | GF | GA | GD | Poss% |
| --- | --- | --- | --- | --- | --- |
| `BadFit` | 1.166 | 1.723 | 2.183 | -0.460 | 46.4 |
| `NeutralFit` | 1.593 | 2.326 | 2.022 | 0.304 | 47.2 |
| `IdealFit` | 1.963 | 2.782 | 1.710 | 1.071 | 47.0 |

Ideal-fit role fingerprints:

| Role | Min | Goals | Shots | Passes C | Pass% | Tackles | Interceptions | Rating |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| `Poacher` | 94.0 | 1.28 | 3.58 | 0.52 | 70.5 | 0.00 | 0.00 | 8.32 |
| `AdvancedPlaymaker` | 93.9 | 0.00 | 0.00 | 5.31 | 78.5 | 3.94 | 3.92 | 8.21 |
| `BoxToBoxMidfielder` | 93.9 | 0.00 | 0.00 | 4.01 | 72.1 | 3.96 | 3.81 | 8.03 |
| `HoldingMidfielder` | 93.8 | 0.00 | 0.00 | 3.93 | 73.0 | 3.49 | 3.48 | 7.91 |
| `ChannelRunner` | 94.0 | 0.75 | 3.42 | 0.58 | 72.5 | 0.00 | 0.00 | 7.72 |
| `SweeperKeeper` | 94.0 | 0.00 | 0.00 | 1.52 | 100.0 | 0.00 | 0.00 | 6.57 |

Ideal-fit punishes:

| Opponent | Formation | Style | Vs Bad | Vs Neutral | Vs Ideal | Ideal GD |
| --- | --- | --- | --- | --- | --- | --- |
| `4-2-3-1__Attacking` | `4-2-3-1` | `Attacking` | 2.625 | 2.667 | 2.542 | 2.208 |
| `4-1-4-1__Attacking` | `4-1-4-1` | `Attacking` | 2.458 | 2.292 | 2.417 | 1.958 |
| `4-5-1__Balanced` | `4-5-1` | `Balanced` | 2.625 | 2.375 | 2.417 | 1.917 |
| `4-5-1__Attacking` | `4-5-1` | `Attacking` | 2.917 | 2.292 | 2.417 | 1.917 |
| `4-2-3-1__HighPress` | `4-2-3-1` | `HighPress` | 2.792 | 2.667 | 2.333 | 1.958 |

Ideal-fit counters:

| Opponent | Formation | Style | Vs Bad | Vs Neutral | Vs Ideal | Ideal GD |
| --- | --- | --- | --- | --- | --- | --- |
| `5-3-2__Counter` | `5-3-2` | `Counter` | 2.042 | 1.833 | 0.708 | -1.625 |
| `4-4-2__Counter` | `4-4-2` | `Counter` | 2.458 | 1.958 | 0.750 | -1.792 |
| `3-5-2__Defensive` | `3-5-2` | `Defensive` | 2.042 | 1.083 | 0.750 | -0.292 |
| `4-4-2__Attacking` | `4-4-2` | `Attacking` | 2.083 | 1.792 | 1.000 | -0.708 |
| `3-4-3__Counter` | `3-4-3` | `Counter` | 2.250 | 1.417 | 1.083 | -0.583 |

## 4-4-2__Attacking

- Formation: `4-4-2`
- Style: `Attacking`
- Target package: `ChaosRaiders`
- Classification: `player_dependent`
- Fit curve: bad 1.170 PPG, neutral 1.575, ideal 1.962
- Fit deltas: ideal-bad 0.791, ideal-neutral 0.387, neutral-bad 0.405

Profiles:

| Profile | PPG | GF | GA | GD | Poss% |
| --- | --- | --- | --- | --- | --- |
| `BadFit` | 1.170 | 2.110 | 2.604 | -0.494 | 48.6 |
| `NeutralFit` | 1.575 | 2.924 | 2.630 | 0.294 | 48.9 |
| `IdealFit` | 1.962 | 3.845 | 2.627 | 1.218 | 49.1 |

Ideal-fit role fingerprints:

| Role | Min | Goals | Shots | Passes C | Pass% | Tackles | Interceptions | Rating |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| `Poacher` | 94.0 | 2.34 | 6.64 | 0.80 | 68.0 | 0.00 | 0.00 | 9.21 |
| `TargetForward` | 94.0 | 1.50 | 5.98 | 1.05 | 70.2 | 0.00 | 0.00 | 8.69 |
| `DeepPlaymaker` | 93.9 | 0.00 | 0.00 | 5.18 | 80.0 | 3.47 | 3.43 | 8.13 |
| `BoxToBoxMidfielder` | 93.8 | 0.00 | 0.00 | 4.19 | 74.9 | 3.60 | 3.51 | 8.02 |
| `WideProgressor` | 93.9 | 0.00 | 0.00 | 3.44 | 74.5 | 2.91 | 2.91 | 7.85 |
| `Goalkeeper` | 94.0 | 0.00 | 0.00 | 1.38 | 100.0 | 0.00 | 0.00 | 6.54 |

Ideal-fit punishes:

| Opponent | Formation | Style | Vs Bad | Vs Neutral | Vs Ideal | Ideal GD |
| --- | --- | --- | --- | --- | --- | --- |
| `4-5-1__Attacking` | `4-5-1` | `Attacking` | 2.833 | 2.625 | 2.917 | 4.500 |
| `4-2-3-1__Attacking` | `4-2-3-1` | `Attacking` | 2.375 | 2.375 | 2.708 | 3.167 |
| `4-1-4-1__HighPress` | `4-1-4-1` | `HighPress` | 2.417 | 2.083 | 2.667 | 2.250 |
| `4-5-1__Balanced` | `4-5-1` | `Balanced` | 2.500 | 2.500 | 2.667 | 2.167 |
| `4-1-4-1__Attacking` | `4-1-4-1` | `Attacking` | 2.708 | 2.500 | 2.458 | 2.917 |

Ideal-fit counters:

| Opponent | Formation | Style | Vs Bad | Vs Neutral | Vs Ideal | Ideal GD |
| --- | --- | --- | --- | --- | --- | --- |
| `3-5-2__Counter` | `3-5-2` | `Counter` | 1.417 | 1.042 | 0.542 | -1.458 |
| `3-4-3__Counter` | `3-4-3` | `Counter` | 2.042 | 1.500 | 0.792 | -0.708 |
| `4-4-2__Counter` | `4-4-2` | `Counter` | 2.500 | 1.542 | 0.917 | -1.125 |
| `5-3-2__Counter` | `5-3-2` | `Counter` | 2.000 | 1.792 | 1.000 | -0.667 |
| `3-4-3__Defensive` | `3-4-3` | `Defensive` | 1.625 | 2.333 | 1.000 | -0.417 |

## 3-4-3__Attacking

- Formation: `3-4-3`
- Style: `Attacking`
- Target package: `ChaosRaiders`
- Classification: `player_dependent`
- Fit curve: bad 1.185 PPG, neutral 1.588, ideal 1.953
- Fit deltas: ideal-bad 0.767, ideal-neutral 0.365, neutral-bad 0.403

Profiles:

| Profile | PPG | GF | GA | GD | Poss% |
| --- | --- | --- | --- | --- | --- |
| `BadFit` | 1.185 | 2.056 | 2.549 | -0.492 | 48.8 |
| `NeutralFit` | 1.588 | 2.899 | 2.528 | 0.371 | 49.1 |
| `IdealFit` | 1.953 | 3.716 | 2.552 | 1.164 | 48.9 |

Ideal-fit role fingerprints:

| Role | Min | Goals | Shots | Passes C | Pass% | Tackles | Interceptions | Rating |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| `Poacher` | 93.9 | 1.63 | 4.61 | 0.56 | 71.5 | 0.00 | 0.00 | 8.72 |
| `ChannelRunner` | 93.9 | 1.04 | 4.35 | 0.54 | 73.9 | 0.00 | 0.00 | 8.16 |
| `DeepPlaymaker` | 93.8 | 0.00 | 0.00 | 5.07 | 79.8 | 3.44 | 3.37 | 8.12 |
| `BoxToBoxMidfielder` | 93.8 | 0.00 | 0.00 | 4.12 | 74.3 | 3.57 | 3.40 | 8.01 |
| `WideProgressor` | 93.8 | 0.00 | 0.00 | 3.44 | 74.7 | 2.91 | 2.89 | 7.81 |
| `Goalkeeper` | 93.9 | 0.00 | 0.00 | 1.36 | 100.0 | 0.00 | 0.00 | 6.55 |

Ideal-fit punishes:

| Opponent | Formation | Style | Vs Bad | Vs Neutral | Vs Ideal | Ideal GD |
| --- | --- | --- | --- | --- | --- | --- |
| `4-1-4-1__Attacking` | `4-1-4-1` | `Attacking` | 2.875 | 2.500 | 2.750 | 3.167 |
| `4-2-3-1__Attacking` | `4-2-3-1` | `Attacking` | 2.625 | 2.792 | 2.583 | 3.000 |
| `4-2-3-1__Balanced` | `4-2-3-1` | `Balanced` | 2.542 | 2.667 | 2.542 | 2.375 |
| `4-5-1__HighPress` | `4-5-1` | `HighPress` | 2.625 | 2.167 | 2.542 | 1.875 |
| `4-1-4-1__Balanced` | `4-1-4-1` | `Balanced` | 2.542 | 2.208 | 2.500 | 2.167 |

Ideal-fit counters:

| Opponent | Formation | Style | Vs Bad | Vs Neutral | Vs Ideal | Ideal GD |
| --- | --- | --- | --- | --- | --- | --- |
| `5-3-2__Counter` | `5-3-2` | `Counter` | 2.000 | 1.875 | 0.625 | -1.750 |
| `3-5-2__Counter` | `3-5-2` | `Counter` | 1.917 | 2.042 | 0.625 | -1.333 |
| `4-4-2__Counter` | `4-4-2` | `Counter` | 2.375 | 1.833 | 0.708 | -1.125 |
| `5-3-2__Defensive` | `5-3-2` | `Defensive` | 1.875 | 1.625 | 0.708 | -0.708 |
| `3-5-2__Attacking` | `3-5-2` | `Attacking` | 2.375 | 1.333 | 1.083 | -1.000 |

## 4-3-3__Attacking

- Formation: `4-3-3`
- Style: `Attacking`
- Target package: `ChaosRaiders`
- Classification: `player_dependent`
- Fit curve: bad 1.084 PPG, neutral 1.499, ideal 1.905
- Fit deltas: ideal-bad 0.821, ideal-neutral 0.406, neutral-bad 0.415

Profiles:

| Profile | PPG | GF | GA | GD | Poss% |
| --- | --- | --- | --- | --- | --- |
| `BadFit` | 1.084 | 2.009 | 2.662 | -0.654 | 48.7 |
| `NeutralFit` | 1.499 | 2.790 | 2.655 | 0.134 | 48.7 |
| `IdealFit` | 1.905 | 3.624 | 2.576 | 1.048 | 49.3 |

Ideal-fit role fingerprints:

| Role | Min | Goals | Shots | Passes C | Pass% | Tackles | Interceptions | Rating |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| `Poacher` | 94.0 | 1.55 | 4.42 | 0.51 | 69.2 | 0.00 | 0.00 | 8.62 |
| `AdvancedPlaymaker` | 93.9 | 0.00 | 0.00 | 6.92 | 80.2 | 4.56 | 4.47 | 8.60 |
| `BoxToBoxMidfielder` | 93.8 | 0.00 | 0.00 | 5.30 | 74.0 | 4.46 | 4.31 | 8.38 |
| `HoldingMidfielder` | 93.9 | 0.00 | 0.00 | 5.02 | 74.2 | 3.85 | 3.77 | 8.22 |
| `ChannelRunner` | 94.0 | 1.04 | 4.27 | 0.54 | 72.7 | 0.00 | 0.00 | 8.15 |
| `Goalkeeper` | 94.0 | 0.00 | 0.00 | 1.40 | 100.0 | 0.00 | 0.00 | 6.55 |

Ideal-fit punishes:

| Opponent | Formation | Style | Vs Bad | Vs Neutral | Vs Ideal | Ideal GD |
| --- | --- | --- | --- | --- | --- | --- |
| `4-2-3-1__Attacking` | `4-2-3-1` | `Attacking` | 2.667 | 2.583 | 2.458 | 2.458 |
| `4-5-1__Attacking` | `4-5-1` | `Attacking` | 2.542 | 3.000 | 2.375 | 2.583 |
| `4-1-4-1__Attacking` | `4-1-4-1` | `Attacking` | 2.375 | 3.000 | 2.375 | 2.417 |
| `4-1-4-1__Balanced` | `4-1-4-1` | `Balanced` | 2.667 | 2.250 | 2.375 | 1.750 |
| `4-2-3-1__Counter` | `4-2-3-1` | `Counter` | 2.625 | 2.500 | 2.292 | 1.750 |

Ideal-fit counters:

| Opponent | Formation | Style | Vs Bad | Vs Neutral | Vs Ideal | Ideal GD |
| --- | --- | --- | --- | --- | --- | --- |
| `3-5-2__Counter` | `3-5-2` | `Counter` | 1.958 | 1.750 | 0.208 | -2.083 |
| `3-4-3__Balanced` | `3-4-3` | `Balanced` | 1.833 | 1.292 | 0.667 | -1.042 |
| `5-3-2__Attacking` | `5-3-2` | `Attacking` | 2.292 | 1.583 | 0.792 | -1.750 |
| `3-4-3__Counter` | `3-4-3` | `Counter` | 2.167 | 1.917 | 0.833 | -1.083 |
| `5-3-2__Counter` | `5-3-2` | `Counter` | 2.042 | 1.167 | 0.958 | -1.417 |

## 3-5-2__Possession

- Formation: `3-5-2`
- Style: `Possession`
- Target package: `ControlCore`
- Classification: `player_dependent`
- Fit curve: bad 1.478 PPG, neutral 1.724, ideal 1.862
- Fit deltas: ideal-bad 0.385, ideal-neutral 0.138, neutral-bad 0.246

Profiles:

| Profile | PPG | GF | GA | GD | Poss% |
| --- | --- | --- | --- | --- | --- |
| `BadFit` | 1.478 | 1.845 | 1.715 | 0.130 | 50.6 |
| `NeutralFit` | 1.724 | 2.096 | 1.528 | 0.568 | 53.0 |
| `IdealFit` | 1.862 | 2.235 | 1.435 | 0.800 | 53.8 |

Ideal-fit role fingerprints:

| Role | Min | Goals | Shots | Passes C | Pass% | Tackles | Interceptions | Rating |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| `Poacher` | 94.0 | 1.29 | 4.99 | 1.14 | 65.5 | 0.00 | 0.00 | 8.39 |
| `TargetForward` | 94.0 | 0.95 | 4.50 | 1.14 | 67.2 | 0.00 | 0.00 | 8.01 |
| `AdvancedPlaymaker` | 93.9 | 0.00 | 0.00 | 6.77 | 87.5 | 2.59 | 2.63 | 7.69 |
| `HoldingMidfielder` | 93.9 | 0.00 | 0.00 | 4.27 | 80.8 | 2.24 | 2.25 | 7.39 |
| `BoxToBoxMidfielder` | 94.0 | 0.00 | 0.00 | 3.15 | 76.8 | 2.06 | 2.02 | 7.20 |
| `WideProgressor` | 94.0 | 0.00 | 0.00 | 2.89 | 76.8 | 1.72 | 1.79 | 7.13 |

Ideal-fit punishes:

| Opponent | Formation | Style | Vs Bad | Vs Neutral | Vs Ideal | Ideal GD |
| --- | --- | --- | --- | --- | --- | --- |
| `4-2-3-1__Attacking` | `4-2-3-1` | `Attacking` | 2.208 | 2.250 | 2.708 | 1.958 |
| `4-2-3-1__HighPress` | `4-2-3-1` | `HighPress` | 2.167 | 2.250 | 2.500 | 1.708 |
| `4-1-4-1__Attacking` | `4-1-4-1` | `Attacking` | 1.917 | 2.083 | 2.417 | 2.042 |
| `4-5-1__Counter` | `4-5-1` | `Counter` | 2.083 | 2.167 | 2.250 | 1.000 |
| `4-5-1__Attacking` | `4-5-1` | `Attacking` | 2.083 | 2.458 | 2.208 | 1.375 |

Ideal-fit counters:

| Opponent | Formation | Style | Vs Bad | Vs Neutral | Vs Ideal | Ideal GD |
| --- | --- | --- | --- | --- | --- | --- |
| `3-5-2__Attacking` | `3-5-2` | `Attacking` | 1.958 | 1.375 | 0.833 | -0.708 |
| `4-4-2__Attacking` | `4-4-2` | `Attacking` | 2.292 | 1.542 | 0.958 | -0.417 |
| `5-3-2__Attacking` | `5-3-2` | `Attacking` | 2.042 | 1.292 | 1.083 | -0.792 |
| `3-4-3__Attacking` | `3-4-3` | `Attacking` | 2.500 | 1.667 | 1.083 | -0.417 |
| `3-4-3__Counter` | `3-4-3` | `Counter` | 1.750 | 1.458 | 1.167 | -0.333 |

## 3-5-2__Defensive

- Formation: `3-5-2`
- Style: `Defensive`
- Target package: `LaneThiefBlock`
- Classification: `mixed`
- Fit curve: bad 1.558 PPG, neutral 1.738, ideal 1.848
- Fit deltas: ideal-bad 0.290, ideal-neutral 0.110, neutral-bad 0.179

Profiles:

| Profile | PPG | GF | GA | GD | Poss% |
| --- | --- | --- | --- | --- | --- |
| `BadFit` | 1.558 | 1.650 | 1.377 | 0.273 | 50.8 |
| `NeutralFit` | 1.738 | 1.674 | 1.158 | 0.516 | 51.3 |
| `IdealFit` | 1.848 | 1.672 | 0.983 | 0.688 | 51.4 |

Ideal-fit role fingerprints:

| Role | Min | Goals | Shots | Passes C | Pass% | Tackles | Interceptions | Rating |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| `Poacher` | 94.1 | 0.96 | 3.79 | 0.99 | 65.8 | 0.00 | 0.00 | 7.96 |
| `TargetForward` | 94.1 | 0.71 | 3.52 | 0.97 | 67.5 | 0.00 | 0.00 | 7.64 |
| `AdvancedPlaymaker` | 94.0 | 0.00 | 0.00 | 3.60 | 79.9 | 2.25 | 2.16 | 7.25 |
| `BoxToBoxMidfielder` | 94.1 | 0.00 | 0.00 | 2.93 | 75.6 | 2.73 | 1.92 | 7.20 |
| `HoldingMidfielder` | 93.9 | 0.00 | 0.00 | 2.89 | 76.0 | 2.67 | 2.12 | 7.18 |
| `WideProgressor` | 94.0 | 0.00 | 0.00 | 2.75 | 75.2 | 1.91 | 1.85 | 7.06 |

Ideal-fit punishes:

| Opponent | Formation | Style | Vs Bad | Vs Neutral | Vs Ideal | Ideal GD |
| --- | --- | --- | --- | --- | --- | --- |
| `4-2-3-1__Attacking` | `4-2-3-1` | `Attacking` | 2.542 | 2.333 | 2.667 | 1.875 |
| `4-5-1__Attacking` | `4-5-1` | `Attacking` | 2.375 | 2.208 | 2.500 | 1.958 |
| `4-5-1__Balanced` | `4-5-1` | `Balanced` | 2.125 | 2.125 | 2.500 | 1.375 |
| `4-2-3-1__HighPress` | `4-2-3-1` | `HighPress` | 2.083 | 1.917 | 2.417 | 1.375 |
| `4-1-4-1__Attacking` | `4-1-4-1` | `Attacking` | 2.125 | 2.250 | 2.333 | 1.375 |

Ideal-fit counters:

| Opponent | Formation | Style | Vs Bad | Vs Neutral | Vs Ideal | Ideal GD |
| --- | --- | --- | --- | --- | --- | --- |
| `3-5-2__Counter` | `3-5-2` | `Counter` | 2.208 | 1.708 | 0.875 | -0.708 |
| `3-5-2__Possession` | `3-5-2` | `Possession` | 1.833 | 1.625 | 0.958 | -0.708 |
| `3-4-3__Counter` | `3-4-3` | `Counter` | 1.583 | 1.917 | 1.000 | -0.542 |
| `4-4-2__Counter` | `4-4-2` | `Counter` | 2.083 | 1.250 | 1.000 | -0.375 |
| `4-3-3__Possession` | `4-3-3` | `Possession` | 1.667 | 1.625 | 1.125 | -0.333 |

## 3-5-2__Balanced

- Formation: `3-5-2`
- Style: `Balanced`
- Target package: `Baseline`
- Classification: `player_dependent`
- Fit curve: bad 1.316 PPG, neutral 1.703, ideal 1.817
- Fit deltas: ideal-bad 0.501, ideal-neutral 0.114, neutral-bad 0.387

Profiles:

| Profile | PPG | GF | GA | GD | Poss% |
| --- | --- | --- | --- | --- | --- |
| `BadFit` | 1.316 | 1.962 | 2.135 | -0.173 | 50.1 |
| `NeutralFit` | 1.703 | 2.447 | 1.903 | 0.544 | 50.9 |
| `IdealFit` | 1.817 | 2.590 | 1.865 | 0.725 | 51.3 |

Ideal-fit role fingerprints:

| Role | Min | Goals | Shots | Passes C | Pass% | Tackles | Interceptions | Rating |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| `Poacher` | 94.0 | 1.51 | 5.41 | 1.23 | 68.3 | 0.00 | 0.00 | 8.68 |
| `TargetForward` | 94.0 | 1.08 | 5.03 | 1.18 | 70.8 | 0.00 | 0.00 | 8.25 |
| `AdvancedPlaymaker` | 94.0 | 0.00 | 0.00 | 4.26 | 81.3 | 2.52 | 2.50 | 7.57 |
| `BoxToBoxMidfielder` | 93.9 | 0.00 | 0.00 | 3.47 | 75.8 | 2.58 | 2.50 | 7.43 |
| `HoldingMidfielder` | 94.0 | 0.00 | 0.00 | 3.24 | 76.4 | 2.26 | 2.28 | 7.32 |
| `WideProgressor` | 94.0 | 0.00 | 0.00 | 3.15 | 75.7 | 2.15 | 2.14 | 7.31 |

Ideal-fit punishes:

| Opponent | Formation | Style | Vs Bad | Vs Neutral | Vs Ideal | Ideal GD |
| --- | --- | --- | --- | --- | --- | --- |
| `4-5-1__HighPress` | `4-5-1` | `HighPress` | 2.083 | 2.083 | 2.708 | 1.667 |
| `4-5-1__Counter` | `4-5-1` | `Counter` | 2.375 | 2.042 | 2.625 | 1.792 |
| `4-2-3-1__Attacking` | `4-2-3-1` | `Attacking` | 2.583 | 2.125 | 2.458 | 1.750 |
| `4-2-3-1__HighPress` | `4-2-3-1` | `HighPress` | 2.250 | 2.250 | 2.250 | 1.333 |
| `4-1-4-1__HighPress` | `4-1-4-1` | `HighPress` | 2.458 | 2.583 | 2.250 | 1.250 |

Ideal-fit counters:

| Opponent | Formation | Style | Vs Bad | Vs Neutral | Vs Ideal | Ideal GD |
| --- | --- | --- | --- | --- | --- | --- |
| `3-5-2__Counter` | `3-5-2` | `Counter` | 2.125 | 1.458 | 0.458 | -2.500 |
| `3-4-3__Counter` | `3-4-3` | `Counter` | 2.208 | 1.542 | 0.625 | -1.458 |
| `4-4-2__Counter` | `4-4-2` | `Counter` | 2.417 | 1.375 | 0.833 | -1.625 |
| `3-5-2__Attacking` | `3-5-2` | `Attacking` | 1.667 | 1.625 | 0.958 | -0.750 |
| `4-1-4-1__Defensive` | `4-1-4-1` | `Defensive` | 2.292 | 1.375 | 1.042 | -0.208 |

## 5-3-2__Defensive

- Formation: `5-3-2`
- Style: `Defensive`
- Target package: `LaneThiefBlock`
- Classification: `player_dependent`
- Fit curve: bad 1.426 PPG, neutral 1.613, ideal 1.797
- Fit deltas: ideal-bad 0.371, ideal-neutral 0.183, neutral-bad 0.188

Profiles:

| Profile | PPG | GF | GA | GD | Poss% |
| --- | --- | --- | --- | --- | --- |
| `BadFit` | 1.426 | 1.486 | 1.429 | 0.057 | 48.9 |
| `NeutralFit` | 1.613 | 1.541 | 1.194 | 0.346 | 49.3 |
| `IdealFit` | 1.797 | 1.585 | 0.982 | 0.603 | 49.9 |

Ideal-fit role fingerprints:

| Role | Min | Goals | Shots | Passes C | Pass% | Tackles | Interceptions | Rating |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| `Poacher` | 94.0 | 0.93 | 3.55 | 0.92 | 66.0 | 0.00 | 0.00 | 7.89 |
| `AdvancedPlaymaker` | 93.9 | 0.00 | 0.00 | 5.62 | 78.2 | 3.69 | 3.50 | 7.73 |
| `BoxToBoxMidfielder` | 93.9 | 0.00 | 0.00 | 4.43 | 73.1 | 4.50 | 3.25 | 7.72 |
| `HoldingMidfielder` | 93.7 | 0.00 | 0.00 | 4.38 | 73.6 | 4.33 | 3.34 | 7.66 |
| `TargetForward` | 94.0 | 0.66 | 3.28 | 0.89 | 67.2 | 0.00 | 0.00 | 7.54 |
| `Goalkeeper` | 94.0 | 0.00 | 0.00 | 0.62 | 100.0 | 0.00 | 0.00 | 6.41 |

Ideal-fit punishes:

| Opponent | Formation | Style | Vs Bad | Vs Neutral | Vs Ideal | Ideal GD |
| --- | --- | --- | --- | --- | --- | --- |
| `4-5-1__Attacking` | `4-5-1` | `Attacking` | 2.583 | 1.833 | 2.750 | 1.750 |
| `4-2-3-1__Attacking` | `4-2-3-1` | `Attacking` | 1.875 | 2.583 | 2.542 | 1.667 |
| `4-2-3-1__Defensive` | `4-2-3-1` | `Defensive` | 2.083 | 1.792 | 2.167 | 0.750 |
| `4-1-4-1__Counter` | `4-1-4-1` | `Counter` | 2.167 | 1.750 | 2.125 | 0.750 |
| `4-5-1__Possession` | `4-5-1` | `Possession` | 2.250 | 2.042 | 2.083 | 0.875 |

Ideal-fit counters:

| Opponent | Formation | Style | Vs Bad | Vs Neutral | Vs Ideal | Ideal GD |
| --- | --- | --- | --- | --- | --- | --- |
| `3-5-2__Attacking` | `3-5-2` | `Attacking` | 2.042 | 1.750 | 0.958 | -0.750 |
| `3-4-3__Defensive` | `3-4-3` | `Defensive` | 2.000 | 1.833 | 1.000 | -0.208 |
| `4-4-2__Defensive` | `4-4-2` | `Defensive` | 1.833 | 1.750 | 1.083 | -0.375 |
| `3-5-2__Counter` | `3-5-2` | `Counter` | 1.875 | 1.167 | 1.125 | -0.667 |
| `4-1-4-1__Possession` | `4-1-4-1` | `Possession` | 2.042 | 1.417 | 1.125 | -0.333 |

## 3-5-2__HighPress

- Formation: `3-5-2`
- Style: `HighPress`
- Target package: `PressTrap`
- Classification: `player_dependent`
- Fit curve: bad 1.379 PPG, neutral 1.637, ideal 1.793
- Fit deltas: ideal-bad 0.413, ideal-neutral 0.155, neutral-bad 0.258

Profiles:

| Profile | PPG | GF | GA | GD | Poss% |
| --- | --- | --- | --- | --- | --- |
| `BadFit` | 1.379 | 2.090 | 2.181 | -0.090 | 49.8 |
| `NeutralFit` | 1.637 | 2.386 | 1.992 | 0.394 | 51.1 |
| `IdealFit` | 1.793 | 2.543 | 1.861 | 0.681 | 51.6 |

Ideal-fit role fingerprints:

| Role | Min | Goals | Shots | Passes C | Pass% | Tackles | Interceptions | Rating |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| `Poacher` | 94.0 | 1.55 | 5.60 | 1.21 | 67.5 | 0.00 | 0.00 | 8.69 |
| `TargetForward` | 94.0 | 0.99 | 4.99 | 1.09 | 68.6 | 0.00 | 0.00 | 8.13 |
| `BoxToBoxMidfielder` | 93.9 | 0.00 | 0.00 | 3.53 | 76.5 | 2.90 | 2.78 | 7.51 |
| `AdvancedPlaymaker` | 93.9 | 0.00 | 0.00 | 4.14 | 81.0 | 2.51 | 2.48 | 7.51 |
| `HoldingMidfielder` | 93.9 | 0.00 | 0.00 | 3.57 | 77.9 | 2.64 | 2.63 | 7.47 |
| `WideProgressor` | 94.0 | 0.00 | 0.00 | 3.02 | 75.7 | 2.07 | 2.10 | 7.27 |

Ideal-fit punishes:

| Opponent | Formation | Style | Vs Bad | Vs Neutral | Vs Ideal | Ideal GD |
| --- | --- | --- | --- | --- | --- | --- |
| `4-5-1__HighPress` | `4-5-1` | `HighPress` | 2.167 | 2.208 | 2.250 | 1.458 |
| `4-5-1__Attacking` | `4-5-1` | `Attacking` | 2.458 | 2.375 | 2.208 | 1.375 |
| `4-1-4-1__Counter` | `4-1-4-1` | `Counter` | 2.208 | 2.083 | 2.167 | 0.875 |
| `5-3-2__Possession` | `5-3-2` | `Possession` | 1.708 | 1.458 | 2.167 | 0.792 |
| `4-1-4-1__Balanced` | `4-1-4-1` | `Balanced` | 2.292 | 2.042 | 2.125 | 1.333 |

Ideal-fit counters:

| Opponent | Formation | Style | Vs Bad | Vs Neutral | Vs Ideal | Ideal GD |
| --- | --- | --- | --- | --- | --- | --- |
| `3-5-2__Balanced` | `3-5-2` | `Balanced` | 2.042 | 1.458 | 0.708 | -0.917 |
| `4-4-2__Counter` | `4-4-2` | `Counter` | 2.333 | 1.125 | 0.792 | -1.542 |
| `3-5-2__Attacking` | `3-5-2` | `Attacking` | 2.083 | 1.375 | 0.792 | -1.542 |
| `3-4-3__Counter` | `3-4-3` | `Counter` | 1.875 | 1.750 | 0.792 | -0.917 |
| `3-5-2__Counter` | `3-5-2` | `Counter` | 2.083 | 1.250 | 0.833 | -1.125 |

## 3-4-3__HighPress

- Formation: `3-4-3`
- Style: `HighPress`
- Target package: `PressTrap`
- Classification: `player_dependent`
- Fit curve: bad 1.289 PPG, neutral 1.547, ideal 1.758
- Fit deltas: ideal-bad 0.469, ideal-neutral 0.211, neutral-bad 0.258

Profiles:

| Profile | PPG | GF | GA | GD | Poss% |
| --- | --- | --- | --- | --- | --- |
| `BadFit` | 1.289 | 1.959 | 2.216 | -0.257 | 48.6 |
| `NeutralFit` | 1.547 | 2.313 | 2.077 | 0.236 | 49.6 |
| `IdealFit` | 1.758 | 2.583 | 1.979 | 0.604 | 49.8 |

Ideal-fit role fingerprints:

| Role | Min | Goals | Shots | Passes C | Pass% | Tackles | Interceptions | Rating |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| `Poacher` | 93.9 | 1.02 | 3.60 | 0.71 | 67.1 | 0.00 | 0.00 | 8.03 |
| `BoxToBoxMidfielder` | 93.9 | 0.00 | 0.00 | 4.39 | 76.2 | 3.66 | 3.46 | 7.83 |
| `ChannelRunner` | 93.9 | 0.78 | 3.57 | 0.59 | 70.7 | 0.00 | 0.00 | 7.78 |
| `DeepPlaymaker` | 93.8 | 0.00 | 0.00 | 4.95 | 80.5 | 3.08 | 3.09 | 7.78 |
| `WideProgressor` | 93.9 | 0.00 | 0.00 | 3.62 | 74.9 | 2.62 | 2.59 | 7.50 |
| `Goalkeeper` | 93.9 | 0.00 | 0.00 | 1.03 | 100.0 | 0.00 | 0.00 | 6.50 |

Ideal-fit punishes:

| Opponent | Formation | Style | Vs Bad | Vs Neutral | Vs Ideal | Ideal GD |
| --- | --- | --- | --- | --- | --- | --- |
| `4-5-1__HighPress` | `4-5-1` | `HighPress` | 2.542 | 2.292 | 2.458 | 1.583 |
| `4-2-3-1__HighPress` | `4-2-3-1` | `HighPress` | 2.583 | 2.250 | 2.417 | 1.542 |
| `4-5-1__Counter` | `4-5-1` | `Counter` | 2.125 | 2.292 | 2.375 | 1.333 |
| `4-5-1__Balanced` | `4-5-1` | `Balanced` | 2.292 | 2.292 | 2.250 | 1.625 |
| `4-2-3-1__Counter` | `4-2-3-1` | `Counter` | 2.125 | 1.958 | 2.125 | 0.833 |

Ideal-fit counters:

| Opponent | Formation | Style | Vs Bad | Vs Neutral | Vs Ideal | Ideal GD |
| --- | --- | --- | --- | --- | --- | --- |
| `4-4-2__Counter` | `4-4-2` | `Counter` | 2.000 | 1.167 | 0.583 | -1.625 |
| `3-5-2__Counter` | `3-5-2` | `Counter` | 1.917 | 1.208 | 0.708 | -1.875 |
| `4-3-3__Counter` | `4-3-3` | `Counter` | 2.125 | 1.208 | 0.958 | -1.208 |
| `3-5-2__Attacking` | `3-5-2` | `Attacking` | 1.917 | 1.625 | 0.958 | -0.750 |
| `3-5-2__Defensive` | `3-5-2` | `Defensive` | 1.833 | 1.292 | 1.042 | -0.375 |

## 5-3-2__Possession

- Formation: `5-3-2`
- Style: `Possession`
- Target package: `ControlCore`
- Classification: `mixed`
- Fit curve: bad 1.415 PPG, neutral 1.637, ideal 1.747
- Fit deltas: ideal-bad 0.332, ideal-neutral 0.110, neutral-bad 0.222

Profiles:

| Profile | PPG | GF | GA | GD | Poss% |
| --- | --- | --- | --- | --- | --- |
| `BadFit` | 1.415 | 1.726 | 1.723 | 0.003 | 48.9 |
| `NeutralFit` | 1.637 | 1.951 | 1.566 | 0.385 | 51.3 |
| `IdealFit` | 1.747 | 2.106 | 1.484 | 0.622 | 52.6 |

Ideal-fit role fingerprints:

| Role | Min | Goals | Shots | Passes C | Pass% | Tackles | Interceptions | Rating |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| `Poacher` | 94.0 | 1.20 | 4.74 | 1.15 | 65.7 | 0.00 | 0.00 | 8.25 |
| `AdvancedPlaymaker` | 93.9 | 0.00 | 0.00 | 9.87 | 86.0 | 4.06 | 4.07 | 8.23 |
| `TargetForward` | 94.0 | 0.91 | 4.35 | 1.06 | 66.6 | 0.00 | 0.00 | 7.93 |
| `HoldingMidfielder` | 93.9 | 0.00 | 0.00 | 6.21 | 79.4 | 3.45 | 3.51 | 7.87 |
| `BoxToBoxMidfielder` | 93.9 | 0.00 | 0.00 | 4.57 | 74.8 | 3.13 | 3.12 | 7.64 |
| `Goalkeeper` | 94.0 | 0.00 | 0.00 | 0.78 | 100.0 | 0.00 | 0.00 | 6.45 |

Ideal-fit punishes:

| Opponent | Formation | Style | Vs Bad | Vs Neutral | Vs Ideal | Ideal GD |
| --- | --- | --- | --- | --- | --- | --- |
| `4-5-1__Balanced` | `4-5-1` | `Balanced` | 2.333 | 2.083 | 2.500 | 1.833 |
| `4-2-3-1__Attacking` | `4-2-3-1` | `Attacking` | 2.250 | 2.458 | 2.375 | 1.958 |
| `4-2-3-1__HighPress` | `4-2-3-1` | `HighPress` | 2.167 | 2.250 | 2.375 | 1.583 |
| `4-3-3__Balanced` | `4-3-3` | `Balanced` | 2.250 | 1.875 | 2.292 | 1.167 |
| `4-1-4-1__Attacking` | `4-1-4-1` | `Attacking` | 2.375 | 2.625 | 2.250 | 1.208 |

Ideal-fit counters:

| Opponent | Formation | Style | Vs Bad | Vs Neutral | Vs Ideal | Ideal GD |
| --- | --- | --- | --- | --- | --- | --- |
| `3-5-2__HighPress` | `3-5-2` | `HighPress` | 2.000 | 1.750 | 0.792 | -0.792 |
| `3-5-2__Balanced` | `3-5-2` | `Balanced` | 1.958 | 1.167 | 0.792 | -0.750 |
| `5-3-2__Balanced` | `5-3-2` | `Balanced` | 1.792 | 1.583 | 0.917 | -0.667 |
| `4-4-2__Counter` | `4-4-2` | `Counter` | 1.625 | 1.542 | 1.000 | -0.958 |
| `5-3-2__Counter` | `5-3-2` | `Counter` | 1.875 | 1.792 | 1.042 | -0.500 |

## 5-3-2__HighPress

- Formation: `5-3-2`
- Style: `HighPress`
- Target package: `PressTrap`
- Classification: `player_dependent`
- Fit curve: bad 1.308 PPG, neutral 1.521, ideal 1.746
- Fit deltas: ideal-bad 0.438, ideal-neutral 0.225, neutral-bad 0.212

Profiles:

| Profile | PPG | GF | GA | GD | Poss% |
| --- | --- | --- | --- | --- | --- |
| `BadFit` | 1.308 | 1.933 | 2.155 | -0.222 | 47.9 |
| `NeutralFit` | 1.521 | 2.210 | 2.033 | 0.177 | 49.3 |
| `IdealFit` | 1.746 | 2.492 | 1.868 | 0.624 | 49.9 |

Ideal-fit role fingerprints:

| Role | Min | Goals | Shots | Passes C | Pass% | Tackles | Interceptions | Rating |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| `Poacher` | 94.0 | 1.51 | 5.41 | 1.10 | 65.9 | 0.00 | 0.00 | 8.63 |
| `BoxToBoxMidfielder` | 93.8 | 0.00 | 0.00 | 5.35 | 75.2 | 4.70 | 4.44 | 8.18 |
| `AdvancedPlaymaker` | 94.0 | 0.00 | 0.00 | 6.41 | 80.2 | 4.02 | 4.05 | 8.14 |
| `TargetForward` | 94.0 | 0.98 | 4.64 | 1.08 | 68.4 | 0.00 | 0.00 | 8.09 |
| `HoldingMidfielder` | 93.9 | 0.00 | 0.00 | 5.30 | 76.2 | 4.28 | 4.14 | 8.08 |
| `Goalkeeper` | 94.0 | 0.00 | 0.00 | 1.01 | 100.0 | 0.00 | 0.00 | 6.49 |

Ideal-fit punishes:

| Opponent | Formation | Style | Vs Bad | Vs Neutral | Vs Ideal | Ideal GD |
| --- | --- | --- | --- | --- | --- | --- |
| `4-2-3-1__HighPress` | `4-2-3-1` | `HighPress` | 2.292 | 2.333 | 2.333 | 1.208 |
| `4-1-4-1__Attacking` | `4-1-4-1` | `Attacking` | 1.917 | 2.208 | 2.250 | 1.292 |
| `4-2-3-1__Counter` | `4-2-3-1` | `Counter` | 2.250 | 2.042 | 2.250 | 1.208 |
| `4-5-1__Attacking` | `4-5-1` | `Attacking` | 2.375 | 2.458 | 2.167 | 1.833 |
| `4-2-3-1__Attacking` | `4-2-3-1` | `Attacking` | 2.125 | 1.542 | 2.167 | 1.167 |

Ideal-fit counters:

| Opponent | Formation | Style | Vs Bad | Vs Neutral | Vs Ideal | Ideal GD |
| --- | --- | --- | --- | --- | --- | --- |
| `3-5-2__Attacking` | `3-5-2` | `Attacking` | 2.333 | 2.083 | 0.583 | -1.292 |
| `5-3-2__Counter` | `5-3-2` | `Counter` | 2.125 | 1.375 | 0.708 | -1.083 |
| `4-3-3__Counter` | `4-3-3` | `Counter` | 1.792 | 1.083 | 0.958 | -1.125 |
| `3-5-2__Defensive` | `3-5-2` | `Defensive` | 1.792 | 1.000 | 0.958 | -0.167 |
| `4-4-2__Counter` | `4-4-2` | `Counter` | 1.708 | 1.667 | 1.000 | -1.458 |

## 4-3-3__HighPress

- Formation: `4-3-3`
- Style: `HighPress`
- Target package: `PressTrap`
- Classification: `player_dependent`
- Fit curve: bad 1.176 PPG, neutral 1.463, ideal 1.731
- Fit deltas: ideal-bad 0.556, ideal-neutral 0.268, neutral-bad 0.288

Profiles:

| Profile | PPG | GF | GA | GD | Poss% |
| --- | --- | --- | --- | --- | --- |
| `BadFit` | 1.176 | 1.889 | 2.353 | -0.463 | 48.0 |
| `NeutralFit` | 1.463 | 2.235 | 2.124 | 0.111 | 49.5 |
| `IdealFit` | 1.731 | 2.558 | 1.983 | 0.576 | 50.4 |

Ideal-fit role fingerprints:

| Role | Min | Goals | Shots | Passes C | Pass% | Tackles | Interceptions | Rating |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| `BoxToBoxMidfielder` | 93.9 | 0.00 | 0.00 | 5.45 | 75.5 | 4.52 | 4.34 | 8.16 |
| `AdvancedPlaymaker` | 94.0 | 0.00 | 0.00 | 6.29 | 79.4 | 3.86 | 3.83 | 8.11 |
| `HoldingMidfielder` | 93.8 | 0.00 | 0.00 | 5.30 | 75.8 | 4.05 | 4.04 | 8.07 |
| `Poacher` | 94.0 | 1.01 | 3.60 | 0.71 | 67.5 | 0.00 | 0.00 | 8.02 |
| `ChannelRunner` | 94.0 | 0.78 | 3.59 | 0.60 | 72.0 | 0.00 | 0.00 | 7.78 |
| `Goalkeeper` | 94.0 | 0.00 | 0.00 | 1.06 | 100.0 | 0.00 | 0.00 | 6.50 |

Ideal-fit punishes:

| Opponent | Formation | Style | Vs Bad | Vs Neutral | Vs Ideal | Ideal GD |
| --- | --- | --- | --- | --- | --- | --- |
| `4-5-1__Balanced` | `4-5-1` | `Balanced` | 2.375 | 1.750 | 2.250 | 1.750 |
| `4-2-3-1__Balanced` | `4-2-3-1` | `Balanced` | 2.250 | 1.875 | 2.250 | 1.583 |
| `4-2-3-1__Attacking` | `4-2-3-1` | `Attacking` | 2.208 | 2.458 | 2.167 | 1.458 |
| `4-5-1__Attacking` | `4-5-1` | `Attacking` | 2.000 | 2.375 | 2.167 | 1.125 |
| `4-5-1__Counter` | `4-5-1` | `Counter` | 2.167 | 2.417 | 2.042 | 1.125 |

Ideal-fit counters:

| Opponent | Formation | Style | Vs Bad | Vs Neutral | Vs Ideal | Ideal GD |
| --- | --- | --- | --- | --- | --- | --- |
| `3-5-2__Attacking` | `3-5-2` | `Attacking` | 1.375 | 1.375 | 0.708 | -1.458 |
| `3-5-2__Counter` | `3-5-2` | `Counter` | 1.958 | 1.750 | 0.875 | -1.042 |
| `5-3-2__Counter` | `5-3-2` | `Counter` | 1.583 | 2.000 | 0.875 | -0.750 |
| `4-3-3__Counter` | `4-3-3` | `Counter` | 1.792 | 1.333 | 0.875 | -0.625 |
| `4-4-2__Possession` | `4-4-2` | `Possession` | 2.000 | 1.792 | 0.917 | -1.042 |

## 3-4-3__Defensive

- Formation: `3-4-3`
- Style: `Defensive`
- Target package: `LaneThiefBlock`
- Classification: `mixed`
- Fit curve: bad 1.446 PPG, neutral 1.619, ideal 1.730
- Fit deltas: ideal-bad 0.283, ideal-neutral 0.111, neutral-bad 0.172

Profiles:

| Profile | PPG | GF | GA | GD | Poss% |
| --- | --- | --- | --- | --- | --- |
| `BadFit` | 1.446 | 1.556 | 1.479 | 0.077 | 49.6 |
| `NeutralFit` | 1.619 | 1.533 | 1.192 | 0.341 | 49.7 |
| `IdealFit` | 1.730 | 1.594 | 1.061 | 0.533 | 49.9 |

Ideal-fit role fingerprints:

| Role | Min | Goals | Shots | Passes C | Pass% | Tackles | Interceptions | Rating |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| `Poacher` | 94.0 | 0.66 | 2.50 | 0.65 | 69.0 | 0.00 | 0.00 | 7.46 |
| `DeepPlaymaker` | 94.0 | 0.00 | 0.00 | 4.25 | 79.1 | 2.77 | 2.66 | 7.43 |
| `BoxToBoxMidfielder` | 94.0 | 0.00 | 0.00 | 3.53 | 73.7 | 3.46 | 2.37 | 7.38 |
| `ChannelRunner` | 94.0 | 0.46 | 2.48 | 0.56 | 71.4 | 0.00 | 0.00 | 7.23 |
| `WideProgressor` | 94.0 | 0.00 | 0.00 | 3.21 | 74.0 | 2.39 | 2.21 | 7.20 |
| `Goalkeeper` | 94.0 | 0.00 | 0.00 | 0.67 | 100.0 | 0.00 | 0.00 | 6.42 |

Ideal-fit punishes:

| Opponent | Formation | Style | Vs Bad | Vs Neutral | Vs Ideal | Ideal GD |
| --- | --- | --- | --- | --- | --- | --- |
| `4-1-4-1__Attacking` | `4-1-4-1` | `Attacking` | 2.500 | 2.167 | 2.500 | 1.917 |
| `4-2-3-1__Attacking` | `4-2-3-1` | `Attacking` | 2.500 | 2.375 | 2.458 | 1.167 |
| `4-2-3-1__Counter` | `4-2-3-1` | `Counter` | 1.958 | 1.625 | 2.333 | 0.750 |
| `4-5-1__HighPress` | `4-5-1` | `HighPress` | 2.000 | 2.167 | 2.208 | 1.000 |
| `4-5-1__Attacking` | `4-5-1` | `Attacking` | 2.125 | 2.542 | 2.125 | 1.125 |

Ideal-fit counters:

| Opponent | Formation | Style | Vs Bad | Vs Neutral | Vs Ideal | Ideal GD |
| --- | --- | --- | --- | --- | --- | --- |
| `5-3-2__Attacking` | `5-3-2` | `Attacking` | 2.000 | 1.792 | 0.792 | -1.125 |
| `3-5-2__Attacking` | `3-5-2` | `Attacking` | 2.042 | 2.208 | 0.792 | -1.000 |
| `3-5-2__Counter` | `3-5-2` | `Counter` | 1.792 | 1.000 | 0.917 | -0.958 |
| `4-2-3-1__Possession` | `4-2-3-1` | `Possession` | 2.000 | 2.208 | 0.917 | -0.042 |
| `4-4-2__Counter` | `4-4-2` | `Counter` | 1.375 | 1.292 | 0.958 | -0.500 |

## 4-4-2__Defensive

- Formation: `4-4-2`
- Style: `Defensive`
- Target package: `LaneThiefBlock`
- Classification: `mixed`
- Fit curve: bad 1.503 PPG, neutral 1.600, ideal 1.715
- Fit deltas: ideal-bad 0.212, ideal-neutral 0.115, neutral-bad 0.097

Profiles:

| Profile | PPG | GF | GA | GD | Poss% |
| --- | --- | --- | --- | --- | --- |
| `BadFit` | 1.503 | 1.632 | 1.476 | 0.156 | 49.7 |
| `NeutralFit` | 1.600 | 1.582 | 1.245 | 0.337 | 49.9 |
| `IdealFit` | 1.715 | 1.601 | 1.101 | 0.500 | 50.0 |

Ideal-fit role fingerprints:

| Role | Min | Goals | Shots | Passes C | Pass% | Tackles | Interceptions | Rating |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| `Poacher` | 94.0 | 0.94 | 3.59 | 0.93 | 66.0 | 0.00 | 0.00 | 7.89 |
| `TargetForward` | 94.0 | 0.66 | 3.27 | 0.90 | 68.3 | 0.00 | 0.00 | 7.54 |
| `DeepPlaymaker` | 93.9 | 0.00 | 0.00 | 4.34 | 78.9 | 2.81 | 2.70 | 7.42 |
| `BoxToBoxMidfielder` | 93.9 | 0.00 | 0.00 | 3.55 | 74.2 | 3.41 | 2.43 | 7.40 |
| `WideProgressor` | 93.9 | 0.00 | 0.00 | 3.26 | 73.9 | 2.34 | 2.28 | 7.20 |
| `Goalkeeper` | 94.0 | 0.00 | 0.00 | 0.72 | 100.0 | 0.00 | 0.00 | 6.44 |

Ideal-fit punishes:

| Opponent | Formation | Style | Vs Bad | Vs Neutral | Vs Ideal | Ideal GD |
| --- | --- | --- | --- | --- | --- | --- |
| `4-1-4-1__Attacking` | `4-1-4-1` | `Attacking` | 2.250 | 2.542 | 2.333 | 1.333 |
| `4-2-3-1__Attacking` | `4-2-3-1` | `Attacking` | 2.083 | 2.333 | 2.292 | 1.333 |
| `4-2-3-1__Balanced` | `4-2-3-1` | `Balanced` | 1.833 | 2.125 | 2.083 | 1.208 |
| `4-5-1__Possession` | `4-5-1` | `Possession` | 2.500 | 2.083 | 2.042 | 1.042 |
| `4-5-1__Attacking` | `4-5-1` | `Attacking` | 2.500 | 2.375 | 1.958 | 0.917 |

Ideal-fit counters:

| Opponent | Formation | Style | Vs Bad | Vs Neutral | Vs Ideal | Ideal GD |
| --- | --- | --- | --- | --- | --- | --- |
| `3-5-2__Possession` | `3-5-2` | `Possession` | 1.542 | 1.375 | 0.708 | -0.792 |
| `4-4-2__Counter` | `4-4-2` | `Counter` | 1.958 | 1.542 | 0.917 | -0.792 |
| `3-5-2__Defensive` | `3-5-2` | `Defensive` | 1.500 | 0.875 | 0.917 | -0.375 |
| `4-3-3__Attacking` | `4-3-3` | `Attacking` | 2.458 | 2.167 | 0.958 | -0.583 |
| `4-3-3__Counter` | `4-3-3` | `Counter` | 1.083 | 1.375 | 0.958 | -0.417 |

## 3-4-3__Balanced

- Formation: `3-4-3`
- Style: `Balanced`
- Target package: `Baseline`
- Classification: `player_dependent`
- Fit curve: bad 1.257 PPG, neutral 1.643, ideal 1.707
- Fit deltas: ideal-bad 0.450, ideal-neutral 0.064, neutral-bad 0.387

Profiles:

| Profile | PPG | GF | GA | GD | Poss% |
| --- | --- | --- | --- | --- | --- |
| `BadFit` | 1.257 | 1.894 | 2.216 | -0.323 | 48.3 |
| `NeutralFit` | 1.643 | 2.431 | 2.020 | 0.411 | 49.6 |
| `IdealFit` | 1.707 | 2.489 | 1.954 | 0.535 | 49.7 |

Ideal-fit role fingerprints:

| Role | Min | Goals | Shots | Passes C | Pass% | Tackles | Interceptions | Rating |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| `Poacher` | 94.0 | 1.01 | 3.56 | 0.79 | 69.7 | 0.00 | 0.00 | 8.04 |
| `DeepPlaymaker` | 93.9 | 0.00 | 0.00 | 5.05 | 80.4 | 3.13 | 3.11 | 7.80 |
| `ChannelRunner` | 94.0 | 0.74 | 3.54 | 0.71 | 73.3 | 0.00 | 0.00 | 7.75 |
| `BoxToBoxMidfielder` | 93.8 | 0.00 | 0.00 | 4.04 | 75.0 | 3.18 | 3.06 | 7.65 |
| `WideProgressor` | 93.9 | 0.00 | 0.00 | 3.75 | 75.4 | 2.60 | 2.63 | 7.51 |
| `Goalkeeper` | 94.0 | 0.00 | 0.00 | 1.07 | 100.0 | 0.00 | 0.00 | 6.50 |

Ideal-fit punishes:

| Opponent | Formation | Style | Vs Bad | Vs Neutral | Vs Ideal | Ideal GD |
| --- | --- | --- | --- | --- | --- | --- |
| `4-2-3-1__Attacking` | `4-2-3-1` | `Attacking` | 2.125 | 1.958 | 2.542 | 2.083 |
| `4-3-3__Attacking` | `4-3-3` | `Attacking` | 2.042 | 1.792 | 2.292 | 1.042 |
| `4-1-4-1__Attacking` | `4-1-4-1` | `Attacking` | 1.917 | 2.208 | 2.250 | 1.750 |
| `4-5-1__Attacking` | `4-5-1` | `Attacking` | 2.375 | 2.167 | 2.208 | 1.125 |
| `4-2-3-1__HighPress` | `4-2-3-1` | `HighPress` | 2.250 | 2.250 | 2.167 | 1.208 |

Ideal-fit counters:

| Opponent | Formation | Style | Vs Bad | Vs Neutral | Vs Ideal | Ideal GD |
| --- | --- | --- | --- | --- | --- | --- |
| `4-4-2__Counter` | `4-4-2` | `Counter` | 1.625 | 1.458 | 0.500 | -1.917 |
| `3-4-3__Attacking` | `3-4-3` | `Attacking` | 1.083 | 1.750 | 0.583 | -1.750 |
| `5-3-2__Possession` | `5-3-2` | `Possession` | 1.333 | 1.500 | 0.875 | -0.875 |
| `3-5-2__Counter` | `3-5-2` | `Counter` | 2.167 | 1.167 | 0.917 | -1.000 |
| `5-3-2__Attacking` | `5-3-2` | `Attacking` | 1.583 | 1.375 | 0.958 | -0.958 |

## 4-3-3__Possession

- Formation: `4-3-3`
- Style: `Possession`
- Target package: `ControlCore`
- Classification: `player_dependent`
- Fit curve: bad 1.258 PPG, neutral 1.539, ideal 1.690
- Fit deltas: ideal-bad 0.432, ideal-neutral 0.151, neutral-bad 0.282

Profiles:

| Profile | PPG | GF | GA | GD | Poss% |
| --- | --- | --- | --- | --- | --- |
| `BadFit` | 1.258 | 1.670 | 1.860 | -0.189 | 49.3 |
| `NeutralFit` | 1.539 | 1.899 | 1.652 | 0.246 | 51.5 |
| `IdealFit` | 1.690 | 2.055 | 1.548 | 0.506 | 52.6 |

Ideal-fit role fingerprints:

| Role | Min | Goals | Shots | Passes C | Pass% | Tackles | Interceptions | Rating |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| `AdvancedPlaymaker` | 93.9 | 0.00 | 0.00 | 10.06 | 86.4 | 3.89 | 3.90 | 8.19 |
| `HoldingMidfielder` | 93.9 | 0.00 | 0.00 | 6.30 | 80.2 | 3.32 | 3.38 | 7.83 |
| `Poacher` | 94.0 | 0.83 | 3.20 | 0.72 | 64.8 | 0.00 | 0.00 | 7.73 |
| `BoxToBoxMidfielder` | 93.9 | 0.00 | 0.00 | 4.56 | 74.5 | 3.10 | 3.08 | 7.62 |
| `ChannelRunner` | 94.0 | 0.61 | 3.19 | 0.65 | 69.1 | 0.00 | 0.00 | 7.49 |
| `Goalkeeper` | 94.0 | 0.00 | 0.00 | 0.83 | 100.0 | 0.00 | 0.00 | 6.46 |

Ideal-fit punishes:

| Opponent | Formation | Style | Vs Bad | Vs Neutral | Vs Ideal | Ideal GD |
| --- | --- | --- | --- | --- | --- | --- |
| `4-2-3-1__Attacking` | `4-2-3-1` | `Attacking` | 2.250 | 2.458 | 2.375 | 1.375 |
| `4-5-1__Defensive` | `4-5-1` | `Defensive` | 1.750 | 1.625 | 2.125 | 0.958 |
| `4-5-1__Attacking` | `4-5-1` | `Attacking` | 2.208 | 1.917 | 2.083 | 1.167 |
| `4-1-4-1__Balanced` | `4-1-4-1` | `Balanced` | 1.917 | 1.833 | 2.042 | 0.625 |
| `4-2-3-1__Defensive` | `4-2-3-1` | `Defensive` | 1.917 | 1.875 | 1.917 | 0.458 |

Ideal-fit counters:

| Opponent | Formation | Style | Vs Bad | Vs Neutral | Vs Ideal | Ideal GD |
| --- | --- | --- | --- | --- | --- | --- |
| `3-5-2__Attacking` | `3-5-2` | `Attacking` | 2.000 | 1.125 | 0.625 | -1.667 |
| `5-3-2__Attacking` | `5-3-2` | `Attacking` | 1.958 | 1.750 | 0.708 | -1.042 |
| `4-4-2__Counter` | `4-4-2` | `Counter` | 1.625 | 1.792 | 0.750 | -1.083 |
| `3-5-2__Counter` | `3-5-2` | `Counter` | 1.875 | 0.667 | 0.792 | -1.083 |
| `3-4-3__Counter` | `3-4-3` | `Counter` | 2.125 | 1.792 | 0.792 | -0.625 |

## 3-4-3__Possession

- Formation: `3-4-3`
- Style: `Possession`
- Target package: `ControlCore`
- Classification: `mixed`
- Fit curve: bad 1.406 PPG, neutral 1.537, ideal 1.685
- Fit deltas: ideal-bad 0.280, ideal-neutral 0.149, neutral-bad 0.131

Profiles:

| Profile | PPG | GF | GA | GD | Poss% |
| --- | --- | --- | --- | --- | --- |
| `BadFit` | 1.406 | 1.769 | 1.794 | -0.025 | 49.4 |
| `NeutralFit` | 1.537 | 1.939 | 1.662 | 0.277 | 51.5 |
| `IdealFit` | 1.685 | 2.024 | 1.530 | 0.494 | 51.8 |

Ideal-fit role fingerprints:

| Role | Min | Goals | Shots | Passes C | Pass% | Tackles | Interceptions | Rating |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| `DeepPlaymaker` | 93.8 | 0.00 | 0.00 | 8.10 | 86.4 | 3.27 | 3.31 | 7.91 |
| `Poacher` | 94.0 | 0.82 | 3.17 | 0.72 | 66.4 | 0.00 | 0.00 | 7.72 |
| `ChannelRunner` | 94.0 | 0.60 | 3.11 | 0.66 | 70.3 | 0.00 | 0.00 | 7.48 |
| `BoxToBoxMidfielder` | 94.0 | 0.00 | 0.00 | 3.81 | 75.8 | 2.64 | 2.56 | 7.41 |
| `WideProgressor` | 93.9 | 0.00 | 0.00 | 3.47 | 75.2 | 2.18 | 2.24 | 7.27 |
| `CenterBackPlaymaker` | 93.9 | 0.00 | 0.00 | 2.64 | 73.6 | 1.54 | 0.57 | 6.51 |

Ideal-fit punishes:

| Opponent | Formation | Style | Vs Bad | Vs Neutral | Vs Ideal | Ideal GD |
| --- | --- | --- | --- | --- | --- | --- |
| `4-2-3-1__Attacking` | `4-2-3-1` | `Attacking` | 2.250 | 2.000 | 2.125 | 0.875 |
| `4-2-3-1__HighPress` | `4-2-3-1` | `HighPress` | 2.708 | 2.167 | 2.083 | 1.583 |
| `4-2-3-1__Counter` | `4-2-3-1` | `Counter` | 2.125 | 1.875 | 2.042 | 0.667 |
| `4-4-2__Balanced` | `4-4-2` | `Balanced` | 2.042 | 1.625 | 1.958 | 0.958 |
| `4-1-4-1__Balanced` | `4-1-4-1` | `Balanced` | 2.167 | 2.167 | 1.917 | 0.708 |

Ideal-fit counters:

| Opponent | Formation | Style | Vs Bad | Vs Neutral | Vs Ideal | Ideal GD |
| --- | --- | --- | --- | --- | --- | --- |
| `3-5-2__Attacking` | `3-5-2` | `Attacking` | 1.833 | 1.292 | 0.708 | -1.333 |
| `3-5-2__HighPress` | `3-5-2` | `HighPress` | 1.958 | 1.125 | 0.833 | -0.958 |
| `5-3-2__Defensive` | `5-3-2` | `Defensive` | 1.458 | 2.042 | 0.917 | -0.583 |
| `4-4-2__Counter` | `4-4-2` | `Counter` | 1.667 | 1.375 | 0.958 | -0.333 |
| `3-5-2__Defensive` | `3-5-2` | `Defensive` | 1.833 | 1.083 | 1.000 | -0.417 |

## 4-3-3__Defensive

- Formation: `4-3-3`
- Style: `Defensive`
- Target package: `LaneThiefBlock`
- Classification: `mixed`
- Fit curve: bad 1.355 PPG, neutral 1.554, ideal 1.683
- Fit deltas: ideal-bad 0.328, ideal-neutral 0.129, neutral-bad 0.199

Profiles:

| Profile | PPG | GF | GA | GD | Poss% |
| --- | --- | --- | --- | --- | --- |
| `BadFit` | 1.355 | 1.487 | 1.548 | -0.061 | 49.0 |
| `NeutralFit` | 1.554 | 1.501 | 1.273 | 0.228 | 49.7 |
| `IdealFit` | 1.683 | 1.558 | 1.094 | 0.464 | 50.0 |

Ideal-fit role fingerprints:

| Role | Min | Goals | Shots | Passes C | Pass% | Tackles | Interceptions | Rating |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| `AdvancedPlaymaker` | 93.9 | 0.00 | 0.00 | 5.64 | 78.7 | 3.63 | 3.39 | 7.73 |
| `BoxToBoxMidfielder` | 93.9 | 0.00 | 0.00 | 4.57 | 73.6 | 4.34 | 3.12 | 7.67 |
| `HoldingMidfielder` | 93.8 | 0.00 | 0.00 | 4.41 | 74.1 | 4.21 | 3.29 | 7.63 |
| `Poacher` | 94.0 | 0.63 | 2.46 | 0.61 | 66.0 | 0.00 | 0.00 | 7.41 |
| `ChannelRunner` | 94.0 | 0.46 | 2.37 | 0.56 | 70.0 | 0.00 | 0.00 | 7.20 |
| `Goalkeeper` | 94.0 | 0.00 | 0.00 | 0.69 | 100.0 | 0.00 | 0.00 | 6.43 |

Ideal-fit punishes:

| Opponent | Formation | Style | Vs Bad | Vs Neutral | Vs Ideal | Ideal GD |
| --- | --- | --- | --- | --- | --- | --- |
| `4-1-4-1__Attacking` | `4-1-4-1` | `Attacking` | 2.000 | 2.042 | 2.375 | 1.625 |
| `4-2-3-1__Attacking` | `4-2-3-1` | `Attacking` | 2.292 | 2.250 | 2.292 | 1.958 |
| `4-1-4-1__Balanced` | `4-1-4-1` | `Balanced` | 1.958 | 1.917 | 2.292 | 1.125 |
| `4-5-1__HighPress` | `4-5-1` | `HighPress` | 1.792 | 2.000 | 2.125 | 0.833 |
| `4-5-1__Attacking` | `4-5-1` | `Attacking` | 2.250 | 2.083 | 1.958 | 0.958 |

Ideal-fit counters:

| Opponent | Formation | Style | Vs Bad | Vs Neutral | Vs Ideal | Ideal GD |
| --- | --- | --- | --- | --- | --- | --- |
| `3-5-2__Counter` | `3-5-2` | `Counter` | 1.583 | 1.375 | 0.542 | -0.958 |
| `3-5-2__Defensive` | `3-5-2` | `Defensive` | 1.583 | 1.125 | 0.708 | -0.750 |
| `3-4-3__Counter` | `3-4-3` | `Counter` | 1.625 | 1.375 | 0.833 | -0.417 |
| `4-3-3__Possession` | `4-3-3` | `Possession` | 1.583 | 1.792 | 0.917 | -0.417 |
| `3-5-2__Possession` | `3-5-2` | `Possession` | 1.458 | 1.208 | 1.083 | -0.458 |

## 4-4-2__Possession

- Formation: `4-4-2`
- Style: `Possession`
- Target package: `ControlCore`
- Classification: `mixed`
- Fit curve: bad 1.333 PPG, neutral 1.555, ideal 1.671
- Fit deltas: ideal-bad 0.338, ideal-neutral 0.116, neutral-bad 0.222

Profiles:

| Profile | PPG | GF | GA | GD | Poss% |
| --- | --- | --- | --- | --- | --- |
| `BadFit` | 1.333 | 1.719 | 1.848 | -0.129 | 49.2 |
| `NeutralFit` | 1.555 | 1.944 | 1.683 | 0.261 | 51.4 |
| `IdealFit` | 1.671 | 2.082 | 1.594 | 0.488 | 52.1 |

Ideal-fit role fingerprints:

| Role | Min | Goals | Shots | Passes C | Pass% | Tackles | Interceptions | Rating |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| `Poacher` | 94.0 | 1.19 | 4.58 | 1.10 | 65.1 | 0.00 | 0.00 | 8.25 |
| `DeepPlaymaker` | 93.9 | 0.00 | 0.00 | 8.17 | 86.3 | 3.31 | 3.31 | 7.94 |
| `TargetForward` | 94.0 | 0.89 | 4.25 | 1.06 | 67.3 | 0.00 | 0.00 | 7.93 |
| `BoxToBoxMidfielder` | 94.0 | 0.00 | 0.00 | 3.84 | 75.8 | 2.65 | 2.57 | 7.41 |
| `WideProgressor` | 93.9 | 0.00 | 0.00 | 3.51 | 75.5 | 2.18 | 2.23 | 7.29 |
| `Goalkeeper` | 94.0 | 0.00 | 0.00 | 0.85 | 100.0 | 0.00 | 0.00 | 6.46 |

Ideal-fit punishes:

| Opponent | Formation | Style | Vs Bad | Vs Neutral | Vs Ideal | Ideal GD |
| --- | --- | --- | --- | --- | --- | --- |
| `4-1-4-1__Balanced` | `4-1-4-1` | `Balanced` | 2.250 | 2.000 | 2.208 | 1.625 |
| `4-2-3-1__HighPress` | `4-2-3-1` | `HighPress` | 2.042 | 2.458 | 2.167 | 1.542 |
| `4-5-1__Balanced` | `4-5-1` | `Balanced` | 2.167 | 1.792 | 2.083 | 1.042 |
| `4-5-1__Defensive` | `4-5-1` | `Defensive` | 1.875 | 1.417 | 2.083 | 0.792 |
| `4-1-4-1__Attacking` | `4-1-4-1` | `Attacking` | 2.208 | 2.333 | 2.000 | 0.833 |

Ideal-fit counters:

| Opponent | Formation | Style | Vs Bad | Vs Neutral | Vs Ideal | Ideal GD |
| --- | --- | --- | --- | --- | --- | --- |
| `4-4-2__Counter` | `4-4-2` | `Counter` | 1.708 | 1.708 | 0.625 | -1.125 |
| `4-3-3__Attacking` | `4-3-3` | `Attacking` | 1.958 | 1.875 | 0.917 | -0.625 |
| `5-3-2__Attacking` | `5-3-2` | `Attacking` | 2.333 | 1.375 | 0.958 | -0.833 |
| `4-4-2__Attacking` | `4-4-2` | `Attacking` | 1.708 | 1.292 | 0.958 | -0.792 |
| `3-4-3__Attacking` | `3-4-3` | `Attacking` | 1.833 | 1.542 | 0.958 | -0.542 |

## 4-4-2__Balanced

- Formation: `4-4-2`
- Style: `Balanced`
- Target package: `Baseline`
- Classification: `player_dependent`
- Fit curve: bad 1.189 PPG, neutral 1.580, ideal 1.659
- Fit deltas: ideal-bad 0.471, ideal-neutral 0.079, neutral-bad 0.391

Profiles:

| Profile | PPG | GF | GA | GD | Poss% |
| --- | --- | --- | --- | --- | --- |
| `BadFit` | 1.189 | 1.859 | 2.281 | -0.422 | 48.4 |
| `NeutralFit` | 1.580 | 2.378 | 2.086 | 0.292 | 49.6 |
| `IdealFit` | 1.659 | 2.465 | 2.030 | 0.435 | 49.8 |

Ideal-fit role fingerprints:

| Role | Min | Goals | Shots | Passes C | Pass% | Tackles | Interceptions | Rating |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| `Poacher` | 94.0 | 1.41 | 5.14 | 1.19 | 68.7 | 0.00 | 0.00 | 8.56 |
| `TargetForward` | 94.0 | 1.06 | 4.81 | 1.16 | 69.2 | 0.00 | 0.00 | 8.19 |
| `DeepPlaymaker` | 93.9 | 0.00 | 0.00 | 5.01 | 80.1 | 3.17 | 3.07 | 7.78 |
| `BoxToBoxMidfielder` | 93.9 | 0.00 | 0.00 | 4.14 | 75.5 | 3.13 | 3.05 | 7.63 |
| `WideProgressor` | 93.9 | 0.00 | 0.00 | 3.77 | 75.3 | 2.65 | 2.60 | 7.51 |
| `Goalkeeper` | 94.0 | 0.00 | 0.00 | 1.11 | 100.0 | 0.00 | 0.00 | 6.51 |

Ideal-fit punishes:

| Opponent | Formation | Style | Vs Bad | Vs Neutral | Vs Ideal | Ideal GD |
| --- | --- | --- | --- | --- | --- | --- |
| `4-2-3-1__Attacking` | `4-2-3-1` | `Attacking` | 2.583 | 2.208 | 2.167 | 2.000 |
| `4-1-4-1__HighPress` | `4-1-4-1` | `HighPress` | 1.625 | 2.167 | 2.042 | 0.792 |
| `4-2-3-1__Defensive` | `4-2-3-1` | `Defensive` | 2.125 | 1.167 | 2.042 | 0.625 |
| `4-5-1__Balanced` | `4-5-1` | `Balanced` | 2.458 | 2.708 | 1.958 | 0.875 |
| `4-2-3-1__HighPress` | `4-2-3-1` | `HighPress` | 1.833 | 1.542 | 1.875 | 1.250 |

Ideal-fit counters:

| Opponent | Formation | Style | Vs Bad | Vs Neutral | Vs Ideal | Ideal GD |
| --- | --- | --- | --- | --- | --- | --- |
| `3-5-2__Counter` | `3-5-2` | `Counter` | 1.750 | 1.542 | 0.500 | -1.917 |
| `5-3-2__Counter` | `5-3-2` | `Counter` | 2.417 | 1.000 | 0.667 | -1.375 |
| `4-3-3__Counter` | `4-3-3` | `Counter` | 1.958 | 1.375 | 0.708 | -1.500 |
| `3-4-3__Counter` | `3-4-3` | `Counter` | 2.000 | 1.333 | 0.792 | -1.167 |
| `3-4-3__Possession` | `3-4-3` | `Possession` | 1.792 | 1.542 | 0.833 | -0.958 |

## 5-3-2__Balanced

- Formation: `5-3-2`
- Style: `Balanced`
- Target package: `Baseline`
- Classification: `player_dependent`
- Fit curve: bad 1.242 PPG, neutral 1.620, ideal 1.654
- Fit deltas: ideal-bad 0.412, ideal-neutral 0.034, neutral-bad 0.378

Profiles:

| Profile | PPG | GF | GA | GD | Poss% |
| --- | --- | --- | --- | --- | --- |
| `BadFit` | 1.242 | 1.834 | 2.160 | -0.326 | 48.0 |
| `NeutralFit` | 1.620 | 2.319 | 1.957 | 0.362 | 48.9 |
| `IdealFit` | 1.654 | 2.380 | 1.932 | 0.448 | 49.1 |

Ideal-fit role fingerprints:

| Role | Min | Goals | Shots | Passes C | Pass% | Tackles | Interceptions | Rating |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| `Poacher` | 94.0 | 1.36 | 5.04 | 1.13 | 67.2 | 0.00 | 0.00 | 8.47 |
| `AdvancedPlaymaker` | 93.9 | 0.00 | 0.00 | 6.47 | 80.0 | 4.19 | 4.09 | 8.18 |
| `TargetForward` | 94.0 | 1.02 | 4.61 | 1.13 | 69.6 | 0.00 | 0.00 | 8.13 |
| `BoxToBoxMidfielder` | 93.8 | 0.00 | 0.00 | 5.18 | 74.7 | 4.27 | 4.04 | 8.04 |
| `HoldingMidfielder` | 93.8 | 0.00 | 0.00 | 5.01 | 75.1 | 3.68 | 3.65 | 7.88 |
| `Goalkeeper` | 94.0 | 0.00 | 0.00 | 1.06 | 100.0 | 0.00 | 0.00 | 6.51 |

Ideal-fit punishes:

| Opponent | Formation | Style | Vs Bad | Vs Neutral | Vs Ideal | Ideal GD |
| --- | --- | --- | --- | --- | --- | --- |
| `4-1-4-1__HighPress` | `4-1-4-1` | `HighPress` | 2.125 | 2.125 | 2.375 | 1.292 |
| `4-2-3-1__Attacking` | `4-2-3-1` | `Attacking` | 2.333 | 1.583 | 2.333 | 1.417 |
| `4-1-4-1__Counter` | `4-1-4-1` | `Counter` | 1.708 | 1.750 | 2.292 | 1.208 |
| `4-2-3-1__Counter` | `4-2-3-1` | `Counter` | 2.000 | 1.917 | 2.208 | 1.000 |
| `4-2-3-1__Possession` | `4-2-3-1` | `Possession` | 2.292 | 1.667 | 2.042 | 0.625 |

Ideal-fit counters:

| Opponent | Formation | Style | Vs Bad | Vs Neutral | Vs Ideal | Ideal GD |
| --- | --- | --- | --- | --- | --- | --- |
| `5-3-2__Counter` | `5-3-2` | `Counter` | 2.208 | 1.375 | 0.500 | -1.958 |
| `3-4-3__Counter` | `3-4-3` | `Counter` | 1.542 | 0.958 | 0.542 | -1.875 |
| `4-3-3__Attacking` | `4-3-3` | `Attacking` | 1.875 | 2.333 | 0.542 | -1.125 |
| `3-5-2__Defensive` | `3-5-2` | `Defensive` | 1.125 | 1.500 | 0.708 | -0.917 |
| `3-5-2__Attacking` | `3-5-2` | `Attacking` | 1.542 | 1.667 | 0.750 | -1.625 |

## 4-4-2__HighPress

- Formation: `4-4-2`
- Style: `HighPress`
- Target package: `PressTrap`
- Classification: `player_dependent`
- Fit curve: bad 1.264 PPG, neutral 1.489, ideal 1.623
- Fit deltas: ideal-bad 0.359, ideal-neutral 0.134, neutral-bad 0.225

Profiles:

| Profile | PPG | GF | GA | GD | Poss% |
| --- | --- | --- | --- | --- | --- |
| `BadFit` | 1.264 | 1.996 | 2.264 | -0.269 | 48.5 |
| `NeutralFit` | 1.489 | 2.267 | 2.151 | 0.116 | 49.4 |
| `IdealFit` | 1.623 | 2.413 | 2.035 | 0.378 | 50.0 |

Ideal-fit role fingerprints:

| Role | Min | Goals | Shots | Passes C | Pass% | Tackles | Interceptions | Rating |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| `Poacher` | 94.0 | 1.48 | 5.32 | 1.07 | 66.1 | 0.00 | 0.00 | 8.58 |
| `TargetForward` | 94.0 | 0.93 | 4.61 | 1.04 | 68.0 | 0.00 | 0.00 | 8.01 |
| `BoxToBoxMidfielder` | 93.8 | 0.00 | 0.00 | 4.33 | 76.0 | 3.61 | 3.49 | 7.78 |
| `DeepPlaymaker` | 93.9 | 0.00 | 0.00 | 4.97 | 79.8 | 3.09 | 3.15 | 7.74 |
| `WideProgressor` | 94.0 | 0.00 | 0.00 | 3.66 | 74.6 | 2.64 | 2.62 | 7.47 |
| `Goalkeeper` | 94.0 | 0.00 | 0.00 | 1.13 | 100.0 | 0.00 | 0.00 | 6.51 |

Ideal-fit punishes:

| Opponent | Formation | Style | Vs Bad | Vs Neutral | Vs Ideal | Ideal GD |
| --- | --- | --- | --- | --- | --- | --- |
| `4-2-3-1__Attacking` | `4-2-3-1` | `Attacking` | 2.083 | 2.250 | 2.375 | 1.708 |
| `4-2-3-1__HighPress` | `4-2-3-1` | `HighPress` | 2.167 | 1.875 | 2.292 | 1.333 |
| `4-5-1__HighPress` | `4-5-1` | `HighPress` | 2.292 | 2.042 | 2.083 | 1.167 |
| `4-1-4-1__HighPress` | `4-1-4-1` | `HighPress` | 2.458 | 1.667 | 1.958 | 0.833 |
| `4-5-1__Attacking` | `4-5-1` | `Attacking` | 2.083 | 1.792 | 1.917 | 0.875 |

Ideal-fit counters:

| Opponent | Formation | Style | Vs Bad | Vs Neutral | Vs Ideal | Ideal GD |
| --- | --- | --- | --- | --- | --- | --- |
| `3-5-2__Attacking` | `3-5-2` | `Attacking` | 1.625 | 0.917 | 0.333 | -2.292 |
| `3-5-2__Counter` | `3-5-2` | `Counter` | 2.000 | 1.542 | 0.750 | -1.792 |
| `5-3-2__Defensive` | `5-3-2` | `Defensive` | 1.542 | 1.125 | 0.750 | -0.542 |
| `3-4-3__Counter` | `3-4-3` | `Counter` | 1.833 | 0.667 | 0.792 | -1.667 |
| `5-3-2__Counter` | `5-3-2` | `Counter` | 1.958 | 0.833 | 0.833 | -1.125 |

## 4-1-4-1__Possession

- Formation: `4-1-4-1`
- Style: `Possession`
- Target package: `ControlCore`
- Classification: `player_dependent`
- Fit curve: bad 0.896 PPG, neutral 1.358, ideal 1.599
- Fit deltas: ideal-bad 0.703, ideal-neutral 0.241, neutral-bad 0.462

Profiles:

| Profile | PPG | GF | GA | GD | Poss% |
| --- | --- | --- | --- | --- | --- |
| `BadFit` | 0.896 | 0.910 | 1.666 | -0.757 | 51.3 |
| `NeutralFit` | 1.358 | 1.382 | 1.453 | -0.071 | 54.1 |
| `IdealFit` | 1.599 | 1.623 | 1.292 | 0.331 | 56.1 |

Ideal-fit role fingerprints:

| Role | Min | Goals | Shots | Passes C | Pass% | Tackles | Interceptions | Rating |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| `LinkForward` | 94.0 | 1.62 | 11.09 | 5.17 | 76.9 | 0.00 | 0.00 | 9.14 |
| `AdvancedPlaymaker` | 93.9 | 0.00 | 0.00 | 5.53 | 86.7 | 2.02 | 2.18 | 7.33 |
| `HoldingMidfielder` | 93.9 | 0.00 | 0.00 | 3.44 | 80.7 | 1.77 | 1.84 | 7.08 |
| `Goalkeeper` | 94.0 | 0.00 | 0.00 | 0.69 | 100.0 | 0.00 | 0.00 | 6.43 |
| `CenterBackPlaymaker` | 94.0 | 0.00 | 0.00 | 1.72 | 72.1 | 1.07 | 0.38 | 6.41 |
| `FullBackSupport` | 94.0 | 0.00 | 0.00 | 1.22 | 63.5 | 0.95 | 0.30 | 6.28 |

Ideal-fit punishes:

| Opponent | Formation | Style | Vs Bad | Vs Neutral | Vs Ideal | Ideal GD |
| --- | --- | --- | --- | --- | --- | --- |
| `4-1-4-1__Balanced` | `4-1-4-1` | `Balanced` | 1.917 | 2.000 | 2.042 | 1.125 |
| `4-5-1__Counter` | `4-5-1` | `Counter` | 2.083 | 2.000 | 2.042 | 0.708 |
| `4-5-1__Balanced` | `4-5-1` | `Balanced` | 1.917 | 1.792 | 1.958 | 1.125 |
| `4-2-3-1__Balanced` | `4-2-3-1` | `Balanced` | 2.125 | 1.875 | 1.958 | 0.833 |
| `4-1-4-1__HighPress` | `4-1-4-1` | `HighPress` | 1.625 | 1.917 | 1.958 | 0.625 |

Ideal-fit counters:

| Opponent | Formation | Style | Vs Bad | Vs Neutral | Vs Ideal | Ideal GD |
| --- | --- | --- | --- | --- | --- | --- |
| `3-4-3__HighPress` | `3-4-3` | `HighPress` | 2.000 | 1.125 | 0.708 | -0.667 |
| `3-5-2__Balanced` | `3-5-2` | `Balanced` | 1.208 | 1.375 | 0.750 | -1.208 |
| `3-5-2__Counter` | `3-5-2` | `Counter` | 2.208 | 1.083 | 0.750 | -0.708 |
| `4-4-2__Counter` | `4-4-2` | `Counter` | 2.125 | 1.583 | 0.833 | -0.792 |
| `3-5-2__HighPress` | `3-5-2` | `HighPress` | 2.125 | 1.292 | 0.917 | -1.083 |

## 4-3-3__Balanced

- Formation: `4-3-3`
- Style: `Balanced`
- Target package: `Baseline`
- Classification: `player_dependent`
- Fit curve: bad 1.162 PPG, neutral 1.526, ideal 1.542
- Fit deltas: ideal-bad 0.380, ideal-neutral 0.016, neutral-bad 0.365

Profiles:

| Profile | PPG | GF | GA | GD | Poss% |
| --- | --- | --- | --- | --- | --- |
| `BadFit` | 1.162 | 1.846 | 2.319 | -0.473 | 48.3 |
| `NeutralFit` | 1.526 | 2.248 | 2.060 | 0.189 | 49.4 |
| `IdealFit` | 1.542 | 2.350 | 2.132 | 0.218 | 49.5 |

Ideal-fit role fingerprints:

| Role | Min | Goals | Shots | Passes C | Pass% | Tackles | Interceptions | Rating |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| `AdvancedPlaymaker` | 93.9 | 0.00 | 0.00 | 6.53 | 79.6 | 4.01 | 3.95 | 8.13 |
| `BoxToBoxMidfielder` | 93.9 | 0.00 | 0.00 | 5.24 | 74.8 | 4.11 | 3.84 | 7.98 |
| `Poacher` | 94.0 | 0.95 | 3.42 | 0.75 | 68.2 | 0.00 | 0.00 | 7.95 |
| `HoldingMidfielder` | 94.0 | 0.00 | 0.00 | 5.01 | 74.6 | 3.52 | 3.55 | 7.86 |
| `ChannelRunner` | 94.0 | 0.70 | 3.40 | 0.67 | 71.7 | 0.00 | 0.00 | 7.66 |
| `Goalkeeper` | 94.0 | 0.00 | 0.00 | 1.12 | 100.0 | 0.00 | 0.00 | 6.51 |

Ideal-fit punishes:

| Opponent | Formation | Style | Vs Bad | Vs Neutral | Vs Ideal | Ideal GD |
| --- | --- | --- | --- | --- | --- | --- |
| `4-5-1__Attacking` | `4-5-1` | `Attacking` | 1.917 | 2.208 | 2.250 | 1.292 |
| `4-2-3-1__HighPress` | `4-2-3-1` | `HighPress` | 2.292 | 1.958 | 1.958 | 1.167 |
| `4-1-4-1__Balanced` | `4-1-4-1` | `Balanced` | 2.167 | 1.375 | 1.958 | 0.917 |
| `4-1-4-1__Counter` | `4-1-4-1` | `Counter` | 1.875 | 2.125 | 1.958 | 0.833 |
| `4-2-3-1__Balanced` | `4-2-3-1` | `Balanced` | 2.208 | 1.875 | 1.958 | 0.792 |

Ideal-fit counters:

| Opponent | Formation | Style | Vs Bad | Vs Neutral | Vs Ideal | Ideal GD |
| --- | --- | --- | --- | --- | --- | --- |
| `4-4-2__Counter` | `4-4-2` | `Counter` | 1.500 | 1.042 | 0.375 | -2.250 |
| `4-4-2__Attacking` | `4-4-2` | `Attacking` | 1.583 | 1.167 | 0.542 | -1.958 |
| `5-3-2__Possession` | `5-3-2` | `Possession` | 1.292 | 1.333 | 0.542 | -1.167 |
| `3-5-2__Counter` | `3-5-2` | `Counter` | 2.208 | 1.500 | 0.667 | -1.500 |
| `3-5-2__Defensive` | `3-5-2` | `Defensive` | 1.417 | 1.042 | 0.875 | -0.833 |

## 4-2-3-1__Possession

- Formation: `4-2-3-1`
- Style: `Possession`
- Target package: `ControlCore`
- Classification: `mixed`
- Fit curve: bad 0.826 PPG, neutral 1.268, ideal 1.474
- Fit deltas: ideal-bad 0.648, ideal-neutral 0.206, neutral-bad 0.442

Profiles:

| Profile | PPG | GF | GA | GD | Poss% |
| --- | --- | --- | --- | --- | --- |
| `BadFit` | 0.826 | 0.862 | 1.719 | -0.857 | 50.5 |
| `NeutralFit` | 1.268 | 1.337 | 1.520 | -0.183 | 53.1 |
| `IdealFit` | 1.474 | 1.566 | 1.418 | 0.148 | 54.3 |

Ideal-fit role fingerprints:

| Role | Min | Goals | Shots | Passes C | Pass% | Tackles | Interceptions | Rating |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| `LinkForward` | 94.0 | 1.57 | 10.38 | 4.81 | 77.5 | 0.00 | 0.00 | 9.08 |
| `AdvancedPlaymaker` | 94.0 | 0.00 | 0.00 | 6.33 | 87.3 | 2.41 | 2.46 | 7.45 |
| `DeepPlaymaker` | 94.0 | 0.00 | 0.00 | 6.29 | 87.2 | 2.45 | 2.50 | 7.44 |
| `HoldingMidfielder` | 94.0 | 0.00 | 0.00 | 4.00 | 81.0 | 2.13 | 2.13 | 7.22 |
| `WideProgressor` | 94.0 | 0.00 | 0.00 | 2.70 | 76.5 | 1.60 | 1.67 | 6.96 |
| `Goalkeeper` | 94.0 | 0.00 | 0.00 | 0.75 | 100.0 | 0.00 | 0.00 | 6.44 |

Ideal-fit punishes:

| Opponent | Formation | Style | Vs Bad | Vs Neutral | Vs Ideal | Ideal GD |
| --- | --- | --- | --- | --- | --- | --- |
| `4-2-3-1__Attacking` | `4-2-3-1` | `Attacking` | 2.083 | 1.875 | 2.000 | 1.125 |
| `4-2-3-1__Counter` | `4-2-3-1` | `Counter` | 1.792 | 1.917 | 1.875 | 0.375 |
| `4-1-4-1__HighPress` | `4-1-4-1` | `HighPress` | 1.708 | 1.667 | 1.875 | 0.375 |
| `4-5-1__Balanced` | `4-5-1` | `Balanced` | 2.000 | 1.583 | 1.833 | 0.667 |
| `4-5-1__Counter` | `4-5-1` | `Counter` | 1.917 | 1.667 | 1.833 | 0.583 |

Ideal-fit counters:

| Opponent | Formation | Style | Vs Bad | Vs Neutral | Vs Ideal | Ideal GD |
| --- | --- | --- | --- | --- | --- | --- |
| `3-5-2__Counter` | `3-5-2` | `Counter` | 1.250 | 1.000 | 0.625 | -1.292 |
| `3-4-3__Attacking` | `3-4-3` | `Attacking` | 1.292 | 1.250 | 0.667 | -1.208 |
| `4-4-2__Counter` | `4-4-2` | `Counter` | 2.000 | 1.125 | 0.667 | -1.125 |
| `3-5-2__Possession` | `3-5-2` | `Possession` | 1.458 | 1.250 | 0.667 | -1.083 |
| `4-4-2__Attacking` | `4-4-2` | `Attacking` | 1.625 | 1.292 | 0.667 | -1.042 |

## 4-5-1__Possession

- Formation: `4-5-1`
- Style: `Possession`
- Target package: `ControlCore`
- Classification: `mixed`
- Fit curve: bad 0.825 PPG, neutral 1.202, ideal 1.433
- Fit deltas: ideal-bad 0.608, ideal-neutral 0.231, neutral-bad 0.377

Profiles:

| Profile | PPG | GF | GA | GD | Poss% |
| --- | --- | --- | --- | --- | --- |
| `BadFit` | 0.825 | 0.864 | 1.729 | -0.865 | 50.4 |
| `NeutralFit` | 1.202 | 1.289 | 1.576 | -0.287 | 52.7 |
| `IdealFit` | 1.433 | 1.475 | 1.441 | 0.034 | 53.6 |

Ideal-fit role fingerprints:

| Role | Min | Goals | Shots | Passes C | Pass% | Tackles | Interceptions | Rating |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| `LinkForward` | 93.9 | 1.47 | 9.80 | 4.51 | 76.4 | 0.00 | 0.00 | 8.97 |
| `AdvancedPlaymaker` | 93.9 | 0.00 | 0.00 | 6.71 | 87.0 | 2.61 | 2.64 | 7.49 |
| `HoldingMidfielder` | 93.9 | 0.00 | 0.00 | 4.24 | 80.8 | 2.19 | 2.30 | 7.23 |
| `BoxToBoxMidfielder` | 93.9 | 0.00 | 0.00 | 3.11 | 76.0 | 2.05 | 2.06 | 7.07 |
| `WideProgressor` | 93.9 | 0.00 | 0.00 | 2.88 | 76.0 | 1.74 | 1.76 | 6.99 |
| `Goalkeeper` | 93.9 | 0.00 | 0.00 | 0.80 | 100.0 | 0.00 | 0.00 | 6.45 |

Ideal-fit punishes:

| Opponent | Formation | Style | Vs Bad | Vs Neutral | Vs Ideal | Ideal GD |
| --- | --- | --- | --- | --- | --- | --- |
| `4-5-1__HighPress` | `4-5-1` | `HighPress` | 2.375 | 1.667 | 2.125 | 0.958 |
| `4-2-3-1__Attacking` | `4-2-3-1` | `Attacking` | 1.583 | 1.958 | 1.833 | 0.458 |
| `4-5-1__Counter` | `4-5-1` | `Counter` | 1.958 | 1.458 | 1.792 | 0.458 |
| `4-2-3-1__HighPress` | `4-2-3-1` | `HighPress` | 2.042 | 2.417 | 1.750 | 0.542 |
| `4-1-4-1__Balanced` | `4-1-4-1` | `Balanced` | 2.167 | 1.542 | 1.708 | 0.542 |

Ideal-fit counters:

| Opponent | Formation | Style | Vs Bad | Vs Neutral | Vs Ideal | Ideal GD |
| --- | --- | --- | --- | --- | --- | --- |
| `3-5-2__Counter` | `3-5-2` | `Counter` | 1.833 | 1.042 | 0.375 | -1.833 |
| `4-4-2__Attacking` | `4-4-2` | `Attacking` | 2.125 | 1.167 | 0.417 | -1.792 |
| `3-5-2__Attacking` | `3-5-2` | `Attacking` | 1.583 | 1.250 | 0.583 | -1.417 |
| `5-3-2__Defensive` | `5-3-2` | `Defensive` | 1.417 | 1.375 | 0.583 | -0.875 |
| `4-1-4-1__Defensive` | `4-1-4-1` | `Defensive` | 1.792 | 1.333 | 0.625 | -0.583 |

## 4-1-4-1__Defensive

- Formation: `4-1-4-1`
- Style: `Defensive`
- Target package: `LaneThiefBlock`
- Classification: `mixed`
- Fit curve: bad 1.084 PPG, neutral 1.241, ideal 1.335
- Fit deltas: ideal-bad 0.251, ideal-neutral 0.094, neutral-bad 0.157

Profiles:

| Profile | PPG | GF | GA | GD | Poss% |
| --- | --- | --- | --- | --- | --- |
| `BadFit` | 1.084 | 0.944 | 1.354 | -0.410 | 51.3 |
| `NeutralFit` | 1.241 | 0.959 | 1.122 | -0.163 | 51.9 |
| `IdealFit` | 1.335 | 0.961 | 0.992 | -0.031 | 52.0 |

Ideal-fit role fingerprints:

| Role | Min | Goals | Shots | Passes C | Pass% | Tackles | Interceptions | Rating |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| `LinkForward` | 94.0 | 0.96 | 7.62 | 3.28 | 73.2 | 0.00 | 0.00 | 8.36 |
| `AdvancedPlaymaker` | 93.9 | 0.00 | 0.00 | 3.37 | 79.7 | 2.08 | 2.04 | 7.05 |
| `HoldingMidfielder` | 93.8 | 0.00 | 0.00 | 2.69 | 75.6 | 2.43 | 1.91 | 6.98 |
| `Goalkeeper` | 94.0 | 0.00 | 0.00 | 0.65 | 100.0 | 0.00 | 0.00 | 6.42 |
| `FullBackSupport` | 93.9 | 0.00 | 0.00 | 1.15 | 64.6 | 1.32 | 0.30 | 6.33 |
| `CenterBackPlaymaker` | 94.0 | 0.00 | 0.00 | 1.26 | 63.5 | 1.02 | 0.36 | 6.29 |

Ideal-fit punishes:

| Opponent | Formation | Style | Vs Bad | Vs Neutral | Vs Ideal | Ideal GD |
| --- | --- | --- | --- | --- | --- | --- |
| `4-5-1__Possession` | `4-5-1` | `Possession` | 2.000 | 1.708 | 2.000 | 0.583 |
| `4-2-3-1__Counter` | `4-2-3-1` | `Counter` | 1.792 | 1.333 | 1.875 | 0.583 |
| `4-5-1__Counter` | `4-5-1` | `Counter` | 1.708 | 1.667 | 1.750 | 0.333 |
| `4-1-4-1__Attacking` | `4-1-4-1` | `Attacking` | 1.708 | 1.875 | 1.708 | 0.583 |
| `4-2-3-1__Attacking` | `4-2-3-1` | `Attacking` | 1.833 | 1.667 | 1.708 | 0.458 |

Ideal-fit counters:

| Opponent | Formation | Style | Vs Bad | Vs Neutral | Vs Ideal | Ideal GD |
| --- | --- | --- | --- | --- | --- | --- |
| `5-3-2__Attacking` | `5-3-2` | `Attacking` | 1.958 | 1.375 | 0.250 | -1.917 |
| `5-3-2__Counter` | `5-3-2` | `Counter` | 1.375 | 1.333 | 0.417 | -1.333 |
| `3-5-2__Counter` | `3-5-2` | `Counter` | 1.458 | 1.250 | 0.500 | -1.125 |
| `5-3-2__Possession` | `5-3-2` | `Possession` | 0.875 | 1.125 | 0.583 | -0.833 |
| `3-4-3__Counter` | `3-4-3` | `Counter` | 1.583 | 0.875 | 0.625 | -0.917 |

## 4-2-3-1__Defensive

- Formation: `4-2-3-1`
- Style: `Defensive`
- Target package: `LaneThiefBlock`
- Classification: `mixed`
- Fit curve: bad 1.087 PPG, neutral 1.217, ideal 1.331
- Fit deltas: ideal-bad 0.244, ideal-neutral 0.115, neutral-bad 0.129

Profiles:

| Profile | PPG | GF | GA | GD | Poss% |
| --- | --- | --- | --- | --- | --- |
| `BadFit` | 1.087 | 0.955 | 1.392 | -0.437 | 50.9 |
| `NeutralFit` | 1.217 | 0.949 | 1.147 | -0.198 | 51.4 |
| `IdealFit` | 1.331 | 0.973 | 1.020 | -0.047 | 51.4 |

Ideal-fit role fingerprints:

| Role | Min | Goals | Shots | Passes C | Pass% | Tackles | Interceptions | Rating |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| `LinkForward` | 94.0 | 0.97 | 7.55 | 3.28 | 74.9 | 0.00 | 0.00 | 8.42 |
| `AdvancedPlaymaker` | 93.9 | 0.00 | 0.00 | 3.59 | 79.7 | 2.20 | 2.23 | 7.11 |
| `DeepPlaymaker` | 94.0 | 0.00 | 0.00 | 3.60 | 80.3 | 2.17 | 2.14 | 7.10 |
| `HoldingMidfielder` | 93.8 | 0.00 | 0.00 | 2.82 | 75.8 | 2.65 | 2.09 | 7.04 |
| `WideProgressor` | 94.0 | 0.00 | 0.00 | 2.63 | 74.3 | 1.88 | 1.84 | 6.90 |
| `Goalkeeper` | 94.0 | 0.00 | 0.00 | 0.66 | 100.0 | 0.00 | 0.00 | 6.42 |

Ideal-fit punishes:

| Opponent | Formation | Style | Vs Bad | Vs Neutral | Vs Ideal | Ideal GD |
| --- | --- | --- | --- | --- | --- | --- |
| `4-2-3-1__Attacking` | `4-2-3-1` | `Attacking` | 1.750 | 1.292 | 2.042 | 0.708 |
| `4-5-1__Defensive` | `4-5-1` | `Defensive` | 2.000 | 1.625 | 2.000 | 0.708 |
| `4-5-1__Attacking` | `4-5-1` | `Attacking` | 1.375 | 2.000 | 1.958 | 0.458 |
| `4-1-4-1__Attacking` | `4-1-4-1` | `Attacking` | 1.708 | 1.333 | 1.792 | 0.417 |
| `5-3-2__Balanced` | `5-3-2` | `Balanced` | 1.417 | 1.250 | 1.792 | 0.167 |

Ideal-fit counters:

| Opponent | Formation | Style | Vs Bad | Vs Neutral | Vs Ideal | Ideal GD |
| --- | --- | --- | --- | --- | --- | --- |
| `5-3-2__Attacking` | `5-3-2` | `Attacking` | 1.500 | 1.375 | 0.542 | -1.333 |
| `4-4-2__Attacking` | `4-4-2` | `Attacking` | 1.875 | 1.375 | 0.667 | -0.958 |
| `5-3-2__Defensive` | `5-3-2` | `Defensive` | 1.250 | 1.458 | 0.667 | -0.750 |
| `5-3-2__Possession` | `5-3-2` | `Possession` | 1.042 | 1.083 | 0.667 | -0.750 |
| `4-4-2__Counter` | `4-4-2` | `Counter` | 1.208 | 1.042 | 0.708 | -1.167 |

## 4-1-4-1__Counter

- Formation: `4-1-4-1`
- Style: `Counter`
- Target package: `LaunchCounter`
- Classification: `mixed`
- Fit curve: bad 0.996 PPG, neutral 1.128, ideal 1.294
- Fit deltas: ideal-bad 0.297, ideal-neutral 0.166, neutral-bad 0.132

Profiles:

| Profile | PPG | GF | GA | GD | Poss% |
| --- | --- | --- | --- | --- | --- |
| `BadFit` | 0.996 | 1.268 | 1.936 | -0.668 | 48.9 |
| `NeutralFit` | 1.128 | 1.363 | 1.796 | -0.433 | 49.4 |
| `IdealFit` | 1.294 | 1.343 | 1.524 | -0.181 | 49.3 |

Ideal-fit role fingerprints:

| Role | Min | Goals | Shots | Passes C | Pass% | Tackles | Interceptions | Rating |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| `LinkForward` | 94.0 | 1.34 | 10.63 | 4.05 | 75.4 | 0.00 | 0.00 | 8.94 |
| `AdvancedPlaymaker` | 93.9 | 0.00 | 0.00 | 3.16 | 78.9 | 2.28 | 2.40 | 7.18 |
| `HoldingMidfielder` | 93.9 | 0.00 | 0.00 | 2.38 | 74.5 | 1.98 | 2.11 | 7.00 |
| `SweeperKeeper` | 94.0 | 0.00 | 0.00 | 1.38 | 100.0 | 0.00 | 0.00 | 6.55 |
| `CenterBackPlaymaker` | 94.0 | 0.00 | 0.00 | 1.84 | 66.7 | 1.33 | 0.47 | 6.37 |
| `FullBackSupport` | 93.9 | 0.00 | 0.00 | 1.43 | 63.6 | 1.21 | 0.38 | 6.31 |

Ideal-fit punishes:

| Opponent | Formation | Style | Vs Bad | Vs Neutral | Vs Ideal | Ideal GD |
| --- | --- | --- | --- | --- | --- | --- |
| `4-2-3-1__Attacking` | `4-2-3-1` | `Attacking` | 1.542 | 1.667 | 2.167 | 0.958 |
| `4-2-3-1__HighPress` | `4-2-3-1` | `HighPress` | 2.083 | 1.833 | 2.083 | 0.625 |
| `4-5-1__Attacking` | `4-5-1` | `Attacking` | 1.792 | 1.500 | 1.833 | 0.375 |
| `4-1-4-1__Defensive` | `4-1-4-1` | `Defensive` | 1.333 | 1.750 | 1.750 | 0.167 |
| `4-2-3-1__Balanced` | `4-2-3-1` | `Balanced` | 2.167 | 1.625 | 1.708 | 0.417 |

Ideal-fit counters:

| Opponent | Formation | Style | Vs Bad | Vs Neutral | Vs Ideal | Ideal GD |
| --- | --- | --- | --- | --- | --- | --- |
| `4-4-2__Counter` | `4-4-2` | `Counter` | 1.250 | 1.125 | 0.292 | -2.042 |
| `5-3-2__Counter` | `5-3-2` | `Counter` | 1.125 | 0.750 | 0.375 | -1.958 |
| `3-5-2__Counter` | `3-5-2` | `Counter` | 1.417 | 0.500 | 0.500 | -2.375 |
| `5-3-2__Attacking` | `5-3-2` | `Attacking` | 1.167 | 0.958 | 0.500 | -1.500 |
| `5-3-2__Balanced` | `5-3-2` | `Balanced` | 0.833 | 0.875 | 0.542 | -1.208 |

## 4-5-1__Defensive

- Formation: `4-5-1`
- Style: `Defensive`
- Target package: `LaneThiefBlock`
- Classification: `mixed`
- Fit curve: bad 1.007 PPG, neutral 1.173, ideal 1.291
- Fit deltas: ideal-bad 0.284, ideal-neutral 0.118, neutral-bad 0.166

Profiles:

| Profile | PPG | GF | GA | GD | Poss% |
| --- | --- | --- | --- | --- | --- |
| `BadFit` | 1.007 | 0.908 | 1.407 | -0.499 | 50.8 |
| `NeutralFit` | 1.173 | 0.932 | 1.184 | -0.252 | 50.9 |
| `IdealFit` | 1.291 | 0.961 | 1.015 | -0.054 | 51.2 |

Ideal-fit role fingerprints:

| Role | Min | Goals | Shots | Passes C | Pass% | Tackles | Interceptions | Rating |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| `LinkForward` | 94.0 | 0.96 | 7.38 | 3.27 | 74.4 | 0.00 | 0.00 | 8.36 |
| `AdvancedPlaymaker` | 94.0 | 0.00 | 0.00 | 3.64 | 80.3 | 2.22 | 2.17 | 7.11 |
| `HoldingMidfielder` | 93.9 | 0.00 | 0.00 | 2.91 | 76.0 | 2.71 | 2.14 | 7.06 |
| `BoxToBoxMidfielder` | 94.0 | 0.00 | 0.00 | 2.88 | 74.3 | 2.70 | 2.03 | 7.03 |
| `WideProgressor` | 94.0 | 0.00 | 0.00 | 2.69 | 74.6 | 1.93 | 1.89 | 6.92 |
| `Goalkeeper` | 94.0 | 0.00 | 0.00 | 0.67 | 100.0 | 0.00 | 0.00 | 6.42 |

Ideal-fit punishes:

| Opponent | Formation | Style | Vs Bad | Vs Neutral | Vs Ideal | Ideal GD |
| --- | --- | --- | --- | --- | --- | --- |
| `4-1-4-1__HighPress` | `4-1-4-1` | `HighPress` | 1.875 | 1.708 | 2.000 | 0.625 |
| `4-1-4-1__Attacking` | `4-1-4-1` | `Attacking` | 1.500 | 1.583 | 1.667 | 0.417 |
| `4-5-1__HighPress` | `4-5-1` | `HighPress` | 1.375 | 1.833 | 1.583 | 0.458 |
| `4-2-3-1__Attacking` | `4-2-3-1` | `Attacking` | 1.333 | 1.625 | 1.583 | 0.375 |
| `4-4-2__Attacking` | `4-4-2` | `Attacking` | 1.833 | 0.875 | 1.583 | -0.042 |

Ideal-fit counters:

| Opponent | Formation | Style | Vs Bad | Vs Neutral | Vs Ideal | Ideal GD |
| --- | --- | --- | --- | --- | --- | --- |
| `4-4-2__Counter` | `4-4-2` | `Counter` | 1.750 | 1.625 | 0.417 | -1.250 |
| `5-3-2__Counter` | `5-3-2` | `Counter` | 1.667 | 1.208 | 0.458 | -0.958 |
| `4-4-2__Possession` | `4-4-2` | `Possession` | 0.708 | 1.583 | 0.583 | -0.792 |
| `3-4-3__Counter` | `3-4-3` | `Counter` | 1.208 | 0.958 | 0.625 | -1.000 |
| `4-3-3__Possession` | `4-3-3` | `Possession` | 1.417 | 1.292 | 0.625 | -0.958 |

## 4-2-3-1__Counter

- Formation: `4-2-3-1`
- Style: `Counter`
- Target package: `LaunchCounter`
- Classification: `mixed`
- Fit curve: bad 0.901 PPG, neutral 1.124, ideal 1.284
- Fit deltas: ideal-bad 0.382, ideal-neutral 0.159, neutral-bad 0.223

Profiles:

| Profile | PPG | GF | GA | GD | Poss% |
| --- | --- | --- | --- | --- | --- |
| `BadFit` | 0.901 | 1.222 | 2.054 | -0.832 | 48.2 |
| `NeutralFit` | 1.124 | 1.348 | 1.817 | -0.469 | 49.0 |
| `IdealFit` | 1.284 | 1.376 | 1.512 | -0.135 | 49.4 |

Ideal-fit role fingerprints:

| Role | Min | Goals | Shots | Passes C | Pass% | Tackles | Interceptions | Rating |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| `LinkForward` | 94.0 | 1.38 | 10.81 | 4.17 | 76.3 | 0.00 | 0.00 | 8.98 |
| `DeepPlaymaker` | 94.0 | 0.00 | 0.00 | 3.83 | 82.6 | 2.54 | 3.09 | 7.40 |
| `AdvancedPlaymaker` | 93.9 | 0.00 | 0.00 | 3.29 | 79.8 | 2.32 | 2.45 | 7.22 |
| `HoldingMidfielder` | 94.0 | 0.00 | 0.00 | 2.43 | 74.7 | 2.11 | 2.18 | 7.04 |
| `WideProgressor` | 94.0 | 0.00 | 0.00 | 2.21 | 74.3 | 2.07 | 2.14 | 7.02 |
| `SweeperKeeper` | 94.0 | 0.00 | 0.00 | 1.41 | 100.0 | 0.00 | 0.00 | 6.55 |

Ideal-fit punishes:

| Opponent | Formation | Style | Vs Bad | Vs Neutral | Vs Ideal | Ideal GD |
| --- | --- | --- | --- | --- | --- | --- |
| `4-5-1__Balanced` | `4-5-1` | `Balanced` | 2.250 | 1.833 | 1.958 | 0.875 |
| `4-2-3-1__HighPress` | `4-2-3-1` | `HighPress` | 2.083 | 1.708 | 1.917 | 0.542 |
| `4-5-1__Counter` | `4-5-1` | `Counter` | 1.917 | 1.500 | 1.542 | 0.333 |
| `4-5-1__HighPress` | `4-5-1` | `HighPress` | 1.458 | 1.917 | 1.542 | 0.292 |
| `4-2-3-1__Attacking` | `4-2-3-1` | `Attacking` | 1.542 | 1.958 | 1.542 | -0.042 |

Ideal-fit counters:

| Opponent | Formation | Style | Vs Bad | Vs Neutral | Vs Ideal | Ideal GD |
| --- | --- | --- | --- | --- | --- | --- |
| `4-4-2__Counter` | `4-4-2` | `Counter` | 1.417 | 1.125 | 0.292 | -2.292 |
| `5-3-2__Counter` | `5-3-2` | `Counter` | 1.667 | 0.792 | 0.292 | -1.500 |
| `3-4-3__Defensive` | `3-4-3` | `Defensive` | 1.625 | 0.792 | 0.458 | -0.750 |
| `5-3-2__HighPress` | `5-3-2` | `HighPress` | 1.250 | 1.000 | 0.500 | -1.208 |
| `4-3-3__Attacking` | `4-3-3` | `Attacking` | 2.250 | 0.958 | 0.542 | -1.750 |

## 4-5-1__Counter

- Formation: `4-5-1`
- Style: `Counter`
- Target package: `LaunchCounter`
- Classification: `fit_limited`
- Fit curve: bad 0.889 PPG, neutral 1.038, ideal 1.229
- Fit deltas: ideal-bad 0.340, ideal-neutral 0.192, neutral-bad 0.149

Profiles:

| Profile | PPG | GF | GA | GD | Poss% |
| --- | --- | --- | --- | --- | --- |
| `BadFit` | 0.889 | 1.240 | 2.084 | -0.845 | 48.0 |
| `NeutralFit` | 1.038 | 1.278 | 1.862 | -0.584 | 48.7 |
| `IdealFit` | 1.229 | 1.301 | 1.586 | -0.285 | 48.6 |

Ideal-fit role fingerprints:

| Role | Min | Goals | Shots | Passes C | Pass% | Tackles | Interceptions | Rating |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| `LinkForward` | 94.0 | 1.30 | 10.28 | 3.99 | 75.7 | 0.00 | 0.00 | 8.89 |
| `AdvancedPlaymaker` | 94.0 | 0.00 | 0.00 | 3.40 | 79.3 | 2.48 | 2.51 | 7.24 |
| `BoxToBoxMidfielder` | 94.0 | 0.00 | 0.00 | 2.60 | 73.7 | 2.47 | 2.55 | 7.13 |
| `HoldingMidfielder` | 94.0 | 0.00 | 0.00 | 2.52 | 74.8 | 2.24 | 2.23 | 7.06 |
| `WideProgressor` | 94.0 | 0.00 | 0.00 | 2.31 | 74.7 | 2.17 | 2.23 | 7.05 |
| `SweeperKeeper` | 94.0 | 0.00 | 0.00 | 1.42 | 100.0 | 0.00 | 0.00 | 6.55 |

Ideal-fit punishes:

| Opponent | Formation | Style | Vs Bad | Vs Neutral | Vs Ideal | Ideal GD |
| --- | --- | --- | --- | --- | --- | --- |
| `4-2-3-1__HighPress` | `4-2-3-1` | `HighPress` | 1.375 | 1.750 | 2.500 | 1.292 |
| `4-1-4-1__Attacking` | `4-1-4-1` | `Attacking` | 1.875 | 1.875 | 2.083 | 0.958 |
| `4-2-3-1__Balanced` | `4-2-3-1` | `Balanced` | 1.625 | 1.500 | 1.792 | 0.292 |
| `4-1-4-1__Balanced` | `4-1-4-1` | `Balanced` | 2.208 | 0.875 | 1.667 | 0.333 |
| `4-5-1__Attacking` | `4-5-1` | `Attacking` | 1.542 | 1.708 | 1.625 | 0.417 |

Ideal-fit counters:

| Opponent | Formation | Style | Vs Bad | Vs Neutral | Vs Ideal | Ideal GD |
| --- | --- | --- | --- | --- | --- | --- |
| `3-5-2__Balanced` | `3-5-2` | `Balanced` | 1.125 | 1.083 | 0.250 | -1.792 |
| `3-5-2__Attacking` | `3-5-2` | `Attacking` | 1.750 | 0.917 | 0.375 | -1.667 |
| `5-3-2__Attacking` | `5-3-2` | `Attacking` | 1.125 | 0.792 | 0.417 | -1.625 |
| `3-4-3__Counter` | `3-4-3` | `Counter` | 1.833 | 1.208 | 0.458 | -2.083 |
| `3-5-2__Defensive` | `3-5-2` | `Defensive` | 0.875 | 0.792 | 0.458 | -0.875 |

## 4-1-4-1__HighPress

- Formation: `4-1-4-1`
- Style: `HighPress`
- Target package: `PressTrap`
- Classification: `mixed`
- Fit curve: bad 0.966 PPG, neutral 1.084, ideal 1.190
- Fit deltas: ideal-bad 0.224, ideal-neutral 0.106, neutral-bad 0.118

Profiles:

| Profile | PPG | GF | GA | GD | Poss% |
| --- | --- | --- | --- | --- | --- |
| `BadFit` | 0.966 | 1.277 | 2.060 | -0.782 | 50.7 |
| `NeutralFit` | 1.084 | 1.358 | 1.915 | -0.557 | 51.6 |
| `IdealFit` | 1.190 | 1.517 | 1.839 | -0.322 | 52.0 |

Ideal-fit role fingerprints:

| Role | Min | Goals | Shots | Passes C | Pass% | Tackles | Interceptions | Rating |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| `LinkForward` | 94.0 | 1.52 | 11.04 | 3.86 | 74.4 | 0.00 | 0.00 | 9.07 |
| `AdvancedPlaymaker` | 94.0 | 0.00 | 0.00 | 3.86 | 80.6 | 2.30 | 2.41 | 7.25 |
| `HoldingMidfielder` | 93.9 | 0.00 | 0.00 | 3.31 | 77.3 | 2.44 | 2.47 | 7.20 |
| `Goalkeeper` | 94.0 | 0.00 | 0.00 | 1.02 | 100.0 | 0.00 | 0.00 | 6.49 |
| `FullBackSupport` | 93.9 | 0.00 | 0.00 | 1.47 | 63.7 | 1.17 | 0.37 | 6.30 |
| `CenterBackPlaymaker` | 94.0 | 0.00 | 0.00 | 1.83 | 63.2 | 1.07 | 0.52 | 6.30 |

Ideal-fit punishes:

| Opponent | Formation | Style | Vs Bad | Vs Neutral | Vs Ideal | Ideal GD |
| --- | --- | --- | --- | --- | --- | --- |
| `4-1-4-1__Attacking` | `4-1-4-1` | `Attacking` | 1.458 | 1.250 | 1.833 | 0.333 |
| `4-2-3-1__Attacking` | `4-2-3-1` | `Attacking` | 1.417 | 2.000 | 1.750 | 0.417 |
| `4-1-4-1__Balanced` | `4-1-4-1` | `Balanced` | 1.708 | 1.375 | 1.583 | 0.167 |
| `4-2-3-1__Balanced` | `4-2-3-1` | `Balanced` | 1.667 | 1.250 | 1.500 | 0.125 |
| `4-5-1__Counter` | `4-5-1` | `Counter` | 1.542 | 1.458 | 1.458 | 0.167 |

Ideal-fit counters:

| Opponent | Formation | Style | Vs Bad | Vs Neutral | Vs Ideal | Ideal GD |
| --- | --- | --- | --- | --- | --- | --- |
| `4-4-2__Counter` | `4-4-2` | `Counter` | 1.583 | 1.208 | 0.208 | -2.375 |
| `4-4-2__Attacking` | `4-4-2` | `Attacking` | 1.625 | 0.833 | 0.292 | -2.250 |
| `3-5-2__Attacking` | `3-5-2` | `Attacking` | 1.167 | 1.000 | 0.375 | -2.125 |
| `5-3-2__Counter` | `5-3-2` | `Counter` | 1.167 | 0.667 | 0.458 | -1.917 |
| `5-3-2__Balanced` | `5-3-2` | `Balanced` | 1.625 | 1.083 | 0.500 | -1.292 |

## 4-1-4-1__Balanced

- Formation: `4-1-4-1`
- Style: `Balanced`
- Target package: `Baseline`
- Classification: `fit_limited`
- Fit curve: bad 0.867 PPG, neutral 1.154, ideal 1.181
- Fit deltas: ideal-bad 0.315, ideal-neutral 0.028, neutral-bad 0.287

Profiles:

| Profile | PPG | GF | GA | GD | Poss% |
| --- | --- | --- | --- | --- | --- |
| `BadFit` | 0.867 | 1.154 | 2.066 | -0.912 | 50.4 |
| `NeutralFit` | 1.154 | 1.427 | 1.854 | -0.427 | 51.3 |
| `IdealFit` | 1.181 | 1.489 | 1.859 | -0.370 | 51.7 |

Ideal-fit role fingerprints:

| Role | Min | Goals | Shots | Passes C | Pass% | Tackles | Interceptions | Rating |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| `LinkForward` | 93.9 | 1.49 | 11.02 | 4.03 | 74.9 | 0.00 | 0.00 | 9.07 |
| `AdvancedPlaymaker` | 93.9 | 0.00 | 0.00 | 3.89 | 80.9 | 2.31 | 2.37 | 7.25 |
| `HoldingMidfielder` | 93.9 | 0.00 | 0.00 | 2.98 | 75.9 | 2.10 | 2.12 | 7.07 |
| `Goalkeeper` | 93.9 | 0.00 | 0.00 | 1.02 | 100.0 | 0.00 | 0.00 | 6.50 |
| `CenterBackPlaymaker` | 93.9 | 0.00 | 0.00 | 1.83 | 63.8 | 1.26 | 0.41 | 6.32 |
| `FullBackSupport` | 93.9 | 0.00 | 0.00 | 1.60 | 63.9 | 1.25 | 0.37 | 6.32 |

Ideal-fit punishes:

| Opponent | Formation | Style | Vs Bad | Vs Neutral | Vs Ideal | Ideal GD |
| --- | --- | --- | --- | --- | --- | --- |
| `4-2-3-1__Balanced` | `4-2-3-1` | `Balanced` | 1.792 | 1.958 | 1.875 | 0.625 |
| `4-2-3-1__HighPress` | `4-2-3-1` | `HighPress` | 1.375 | 1.667 | 1.875 | 0.542 |
| `4-2-3-1__Attacking` | `4-2-3-1` | `Attacking` | 1.542 | 2.042 | 1.750 | 0.333 |
| `4-5-1__HighPress` | `4-5-1` | `HighPress` | 1.958 | 1.333 | 1.708 | 0.333 |
| `4-1-4-1__Attacking` | `4-1-4-1` | `Attacking` | 1.500 | 1.583 | 1.708 | 0.208 |

Ideal-fit counters:

| Opponent | Formation | Style | Vs Bad | Vs Neutral | Vs Ideal | Ideal GD |
| --- | --- | --- | --- | --- | --- | --- |
| `3-5-2__Counter` | `3-5-2` | `Counter` | 1.375 | 0.333 | 0.208 | -2.667 |
| `4-4-2__Counter` | `4-4-2` | `Counter` | 1.292 | 0.542 | 0.250 | -2.500 |
| `3-5-2__Attacking` | `3-5-2` | `Attacking` | 1.625 | 1.208 | 0.292 | -2.000 |
| `3-4-3__Attacking` | `3-4-3` | `Attacking` | 1.208 | 1.083 | 0.375 | -2.167 |
| `5-3-2__Counter` | `5-3-2` | `Counter` | 1.333 | 0.833 | 0.417 | -2.250 |

## 4-2-3-1__Balanced

- Formation: `4-2-3-1`
- Style: `Balanced`
- Target package: `Baseline`
- Classification: `fit_limited`
- Fit curve: bad 0.816 PPG, neutral 1.100, ideal 1.155
- Fit deltas: ideal-bad 0.339, ideal-neutral 0.055, neutral-bad 0.285

Profiles:

| Profile | PPG | GF | GA | GD | Poss% |
| --- | --- | --- | --- | --- | --- |
| `BadFit` | 0.816 | 1.138 | 2.168 | -1.030 | 50.1 |
| `NeutralFit` | 1.100 | 1.418 | 1.929 | -0.511 | 51.0 |
| `IdealFit` | 1.155 | 1.471 | 1.933 | -0.462 | 51.1 |

Ideal-fit role fingerprints:

| Role | Min | Goals | Shots | Passes C | Pass% | Tackles | Interceptions | Rating |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| `LinkForward` | 93.9 | 1.47 | 10.79 | 3.99 | 75.6 | 0.00 | 0.00 | 9.04 |
| `DeepPlaymaker` | 93.9 | 0.00 | 0.00 | 4.12 | 81.0 | 2.55 | 2.50 | 7.31 |
| `AdvancedPlaymaker` | 93.9 | 0.00 | 0.00 | 4.15 | 80.9 | 2.49 | 2.55 | 7.31 |
| `HoldingMidfielder` | 93.9 | 0.00 | 0.00 | 3.16 | 76.4 | 2.29 | 2.24 | 7.14 |
| `WideProgressor` | 93.9 | 0.00 | 0.00 | 3.06 | 75.9 | 2.10 | 2.13 | 7.08 |
| `Goalkeeper` | 93.9 | 0.00 | 0.00 | 1.05 | 100.0 | 0.00 | 0.00 | 6.50 |

Ideal-fit punishes:

| Opponent | Formation | Style | Vs Bad | Vs Neutral | Vs Ideal | Ideal GD |
| --- | --- | --- | --- | --- | --- | --- |
| `4-5-1__Balanced` | `4-5-1` | `Balanced` | 1.667 | 1.458 | 2.042 | 0.667 |
| `4-5-1__HighPress` | `4-5-1` | `HighPress` | 1.167 | 1.125 | 1.542 | 0.083 |
| `4-2-3-1__Attacking` | `4-2-3-1` | `Attacking` | 1.625 | 2.042 | 1.500 | -0.083 |
| `4-2-3-1__Counter` | `4-2-3-1` | `Counter` | 2.167 | 1.375 | 1.458 | -0.042 |
| `4-3-3__Defensive` | `4-3-3` | `Defensive` | 1.250 | 0.792 | 1.417 | -0.500 |

Ideal-fit counters:

| Opponent | Formation | Style | Vs Bad | Vs Neutral | Vs Ideal | Ideal GD |
| --- | --- | --- | --- | --- | --- | --- |
| `3-4-3__Counter` | `3-4-3` | `Counter` | 1.208 | 0.750 | 0.042 | -2.625 |
| `4-4-2__Counter` | `4-4-2` | `Counter` | 1.542 | 0.750 | 0.208 | -2.375 |
| `5-3-2__Counter` | `5-3-2` | `Counter` | 1.875 | 1.000 | 0.292 | -2.208 |
| `3-4-3__Attacking` | `3-4-3` | `Attacking` | 1.708 | 0.875 | 0.417 | -2.375 |
| `3-5-2__Attacking` | `3-5-2` | `Attacking` | 1.375 | 0.875 | 0.417 | -2.167 |

## 4-5-1__HighPress

- Formation: `4-5-1`
- Style: `HighPress`
- Target package: `PressTrap`
- Classification: `mixed`
- Fit curve: bad 0.845 PPG, neutral 1.007, ideal 1.134
- Fit deltas: ideal-bad 0.289, ideal-neutral 0.127, neutral-bad 0.162

Profiles:

| Profile | PPG | GF | GA | GD | Poss% |
| --- | --- | --- | --- | --- | --- |
| `BadFit` | 0.845 | 1.197 | 2.149 | -0.952 | 49.8 |
| `NeutralFit` | 1.007 | 1.324 | 2.002 | -0.678 | 50.8 |
| `IdealFit` | 1.134 | 1.424 | 1.916 | -0.493 | 51.3 |

Ideal-fit role fingerprints:

| Role | Min | Goals | Shots | Passes C | Pass% | Tackles | Interceptions | Rating |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| `LinkForward` | 94.0 | 1.42 | 10.66 | 3.69 | 74.1 | 0.00 | 0.00 | 9.02 |
| `BoxToBoxMidfielder` | 93.8 | 0.00 | 0.00 | 3.53 | 77.0 | 2.90 | 2.81 | 7.32 |
| `AdvancedPlaymaker` | 93.9 | 0.00 | 0.00 | 4.10 | 80.8 | 2.51 | 2.57 | 7.28 |
| `HoldingMidfielder` | 93.8 | 0.00 | 0.00 | 3.54 | 77.8 | 2.65 | 2.63 | 7.23 |
| `WideProgressor` | 94.0 | 0.00 | 0.00 | 2.99 | 75.3 | 2.10 | 2.17 | 7.07 |
| `Goalkeeper` | 94.0 | 0.00 | 0.00 | 1.04 | 100.0 | 0.00 | 0.00 | 6.50 |

Ideal-fit punishes:

| Opponent | Formation | Style | Vs Bad | Vs Neutral | Vs Ideal | Ideal GD |
| --- | --- | --- | --- | --- | --- | --- |
| `4-5-1__Balanced` | `4-5-1` | `Balanced` | 1.458 | 1.458 | 1.833 | 0.500 |
| `4-1-4-1__Attacking` | `4-1-4-1` | `Attacking` | 1.292 | 1.500 | 1.792 | 0.375 |
| `4-2-3-1__HighPress` | `4-2-3-1` | `HighPress` | 1.792 | 1.625 | 1.792 | 0.292 |
| `4-1-4-1__Counter` | `4-1-4-1` | `Counter` | 1.375 | 1.292 | 1.583 | 0.250 |
| `4-1-4-1__HighPress` | `4-1-4-1` | `HighPress` | 1.625 | 1.417 | 1.542 | 0.167 |

Ideal-fit counters:

| Opponent | Formation | Style | Vs Bad | Vs Neutral | Vs Ideal | Ideal GD |
| --- | --- | --- | --- | --- | --- | --- |
| `4-4-2__Counter` | `4-4-2` | `Counter` | 1.458 | 0.750 | 0.167 | -2.458 |
| `5-3-2__Counter` | `5-3-2` | `Counter` | 0.875 | 0.875 | 0.208 | -2.875 |
| `3-5-2__Balanced` | `3-5-2` | `Balanced` | 0.958 | 0.667 | 0.208 | -1.667 |
| `3-4-3__Attacking` | `3-4-3` | `Attacking` | 1.083 | 0.583 | 0.292 | -1.875 |
| `3-4-3__Counter` | `3-4-3` | `Counter` | 1.208 | 0.625 | 0.375 | -1.833 |

## 4-5-1__Balanced

- Formation: `4-5-1`
- Style: `Balanced`
- Target package: `Baseline`
- Classification: `mixed`
- Fit curve: bad 0.814 PPG, neutral 1.048, ideal 1.080
- Fit deltas: ideal-bad 0.266, ideal-neutral 0.032, neutral-bad 0.234

Profiles:

| Profile | PPG | GF | GA | GD | Poss% |
| --- | --- | --- | --- | --- | --- |
| `BadFit` | 0.814 | 1.101 | 2.153 | -1.052 | 49.8 |
| `NeutralFit` | 1.048 | 1.398 | 1.954 | -0.556 | 50.8 |
| `IdealFit` | 1.080 | 1.390 | 1.923 | -0.532 | 50.8 |

Ideal-fit role fingerprints:

| Role | Min | Goals | Shots | Passes C | Pass% | Tackles | Interceptions | Rating |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| `LinkForward` | 94.0 | 1.39 | 10.48 | 3.92 | 75.9 | 0.00 | 0.00 | 9.01 |
| `AdvancedPlaymaker` | 94.0 | 0.00 | 0.00 | 4.24 | 81.2 | 2.57 | 2.60 | 7.32 |
| `BoxToBoxMidfielder` | 93.8 | 0.00 | 0.00 | 3.40 | 75.8 | 2.54 | 2.53 | 7.18 |
| `HoldingMidfielder` | 93.9 | 0.00 | 0.00 | 3.17 | 75.9 | 2.29 | 2.27 | 7.12 |
| `WideProgressor` | 94.0 | 0.00 | 0.00 | 3.13 | 76.2 | 2.17 | 2.20 | 7.09 |
| `Goalkeeper` | 94.0 | 0.00 | 0.00 | 1.06 | 100.0 | 0.00 | 0.00 | 6.50 |

Ideal-fit punishes:

| Opponent | Formation | Style | Vs Bad | Vs Neutral | Vs Ideal | Ideal GD |
| --- | --- | --- | --- | --- | --- | --- |
| `4-5-1__Attacking` | `4-5-1` | `Attacking` | 1.667 | 1.333 | 2.042 | 0.917 |
| `4-1-4-1__Attacking` | `4-1-4-1` | `Attacking` | 2.042 | 1.417 | 1.667 | 0.333 |
| `4-1-4-1__Defensive` | `4-1-4-1` | `Defensive` | 1.208 | 1.458 | 1.625 | 0.333 |
| `4-2-3-1__Attacking` | `4-2-3-1` | `Attacking` | 1.708 | 1.375 | 1.583 | 0.042 |
| `4-1-4-1__HighPress` | `4-1-4-1` | `HighPress` | 1.208 | 1.542 | 1.333 | 0.042 |

Ideal-fit counters:

| Opponent | Formation | Style | Vs Bad | Vs Neutral | Vs Ideal | Ideal GD |
| --- | --- | --- | --- | --- | --- | --- |
| `4-4-2__Attacking` | `4-4-2` | `Attacking` | 1.083 | 1.000 | 0.167 | -2.167 |
| `3-5-2__Defensive` | `3-5-2` | `Defensive` | 0.708 | 0.667 | 0.250 | -1.375 |
| `4-4-2__Counter` | `4-4-2` | `Counter` | 1.292 | 0.708 | 0.292 | -2.625 |
| `3-5-2__Counter` | `3-5-2` | `Counter` | 0.750 | 0.458 | 0.333 | -2.250 |
| `3-5-2__Attacking` | `3-5-2` | `Attacking` | 1.167 | 0.625 | 0.375 | -2.250 |

## 4-1-4-1__Attacking

- Formation: `4-1-4-1`
- Style: `Attacking`
- Target package: `ChaosRaiders`
- Classification: `flat_curve`
- Fit curve: bad 0.959 PPG, neutral 1.039, ideal 1.076
- Fit deltas: ideal-bad 0.116, ideal-neutral 0.036, neutral-bad 0.080

Profiles:

| Profile | PPG | GF | GA | GD | Poss% |
| --- | --- | --- | --- | --- | --- |
| `BadFit` | 0.959 | 1.534 | 2.435 | -0.900 | 50.9 |
| `NeutralFit` | 1.039 | 1.674 | 2.361 | -0.688 | 51.1 |
| `IdealFit` | 1.076 | 1.680 | 2.300 | -0.620 | 51.5 |

Ideal-fit role fingerprints:

| Role | Min | Goals | Shots | Passes C | Pass% | Tackles | Interceptions | Rating |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| `LinkForward` | 94.0 | 1.68 | 13.22 | 4.51 | 75.7 | 0.00 | 0.00 | 9.28 |
| `AdvancedPlaymaker` | 93.9 | 0.00 | 0.00 | 4.08 | 80.7 | 2.68 | 2.74 | 7.37 |
| `HoldingMidfielder` | 93.9 | 0.00 | 0.00 | 2.94 | 74.8 | 2.22 | 2.29 | 7.12 |
| `Goalkeeper` | 94.0 | 0.00 | 0.00 | 1.30 | 100.0 | 0.00 | 0.00 | 6.54 |
| `FullBackSupport` | 93.9 | 0.00 | 0.00 | 1.88 | 63.8 | 1.41 | 0.42 | 6.34 |
| `CenterBackPlaymaker` | 93.9 | 0.00 | 0.00 | 2.17 | 63.2 | 1.44 | 0.45 | 6.34 |

Ideal-fit punishes:

| Opponent | Formation | Style | Vs Bad | Vs Neutral | Vs Ideal | Ideal GD |
| --- | --- | --- | --- | --- | --- | --- |
| `4-2-3-1__Balanced` | `4-2-3-1` | `Balanced` | 1.708 | 1.500 | 1.583 | 0.083 |
| `4-1-4-1__Counter` | `4-1-4-1` | `Counter` | 1.208 | 1.250 | 1.458 | 0.083 |
| `4-2-3-1__Possession` | `4-2-3-1` | `Possession` | 2.083 | 1.292 | 1.458 | -0.167 |
| `4-5-1__Attacking` | `4-5-1` | `Attacking` | 1.375 | 1.792 | 1.417 | 0.167 |
| `4-2-3-1__Attacking` | `4-2-3-1` | `Attacking` | 1.250 | 1.417 | 1.375 | 0.125 |

Ideal-fit counters:

| Opponent | Formation | Style | Vs Bad | Vs Neutral | Vs Ideal | Ideal GD |
| --- | --- | --- | --- | --- | --- | --- |
| `5-3-2__Attacking` | `5-3-2` | `Attacking` | 1.250 | 0.875 | 0.042 | -3.042 |
| `3-5-2__Counter` | `3-5-2` | `Counter` | 1.292 | 0.958 | 0.208 | -2.708 |
| `3-4-3__Attacking` | `3-4-3` | `Attacking` | 2.000 | 0.792 | 0.250 | -3.167 |
| `3-4-3__Counter` | `3-4-3` | `Counter` | 1.125 | 0.625 | 0.292 | -2.917 |
| `3-5-2__Attacking` | `3-5-2` | `Attacking` | 1.417 | 0.750 | 0.333 | -3.417 |

## 4-2-3-1__HighPress

- Formation: `4-2-3-1`
- Style: `HighPress`
- Target package: `PressTrap`
- Classification: `flat_curve`
- Fit curve: bad 0.892 PPG, neutral 1.000, ideal 1.037
- Fit deltas: ideal-bad 0.145, ideal-neutral 0.037, neutral-bad 0.108

Profiles:

| Profile | PPG | GF | GA | GD | Poss% |
| --- | --- | --- | --- | --- | --- |
| `BadFit` | 0.892 | 1.277 | 2.147 | -0.870 | 50.0 |
| `NeutralFit` | 1.000 | 1.314 | 2.018 | -0.705 | 50.9 |
| `IdealFit` | 1.037 | 1.389 | 1.988 | -0.599 | 51.4 |

Ideal-fit role fingerprints:

| Role | Min | Goals | Shots | Passes C | Pass% | Tackles | Interceptions | Rating |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| `LinkForward` | 94.0 | 1.39 | 10.80 | 3.77 | 74.5 | 0.00 | 0.00 | 9.00 |
| `AdvancedPlaymaker` | 94.0 | 0.00 | 0.00 | 4.11 | 81.0 | 2.51 | 2.52 | 7.28 |
| `DeepPlaymaker` | 94.0 | 0.00 | 0.00 | 4.06 | 80.5 | 2.51 | 2.54 | 7.27 |
| `HoldingMidfielder` | 94.0 | 0.00 | 0.00 | 3.46 | 77.0 | 2.64 | 2.61 | 7.23 |
| `WideProgressor` | 94.0 | 0.00 | 0.00 | 2.98 | 75.8 | 2.08 | 2.15 | 7.06 |
| `Goalkeeper` | 94.0 | 0.00 | 0.00 | 1.08 | 100.0 | 0.00 | 0.00 | 6.50 |

Ideal-fit punishes:

| Opponent | Formation | Style | Vs Bad | Vs Neutral | Vs Ideal | Ideal GD |
| --- | --- | --- | --- | --- | --- | --- |
| `4-1-4-1__HighPress` | `4-1-4-1` | `HighPress` | 1.250 | 1.208 | 1.583 | 0.000 |
| `4-2-3-1__Balanced` | `4-2-3-1` | `Balanced` | 1.792 | 1.333 | 1.458 | 0.125 |
| `4-5-1__Balanced` | `4-5-1` | `Balanced` | 1.708 | 1.292 | 1.458 | 0.042 |
| `4-1-4-1__Attacking` | `4-1-4-1` | `Attacking` | 1.583 | 0.750 | 1.375 | 0.208 |
| `4-5-1__Defensive` | `4-5-1` | `Defensive` | 1.042 | 1.083 | 1.250 | -0.042 |

Ideal-fit counters:

| Opponent | Formation | Style | Vs Bad | Vs Neutral | Vs Ideal | Ideal GD |
| --- | --- | --- | --- | --- | --- | --- |
| `3-4-3__Counter` | `3-4-3` | `Counter` | 0.625 | 0.458 | 0.167 | -1.833 |
| `3-5-2__Counter` | `3-5-2` | `Counter` | 0.917 | 0.917 | 0.292 | -2.542 |
| `5-3-2__Counter` | `5-3-2` | `Counter` | 1.167 | 1.125 | 0.333 | -2.750 |
| `5-3-2__Attacking` | `5-3-2` | `Attacking` | 0.917 | 0.875 | 0.333 | -2.333 |
| `3-5-2__Possession` | `3-5-2` | `Possession` | 1.125 | 1.250 | 0.375 | -1.708 |

## 4-5-1__Attacking

- Formation: `4-5-1`
- Style: `Attacking`
- Target package: `ChaosRaiders`
- Classification: `flat_curve`
- Fit curve: bad 0.853 PPG, neutral 0.936, ideal 1.000
- Fit deltas: ideal-bad 0.148, ideal-neutral 0.064, neutral-bad 0.083

Profiles:

| Profile | PPG | GF | GA | GD | Poss% |
| --- | --- | --- | --- | --- | --- |
| `BadFit` | 0.853 | 1.459 | 2.518 | -1.059 | 49.9 |
| `NeutralFit` | 0.936 | 1.552 | 2.484 | -0.932 | 50.1 |
| `IdealFit` | 1.000 | 1.607 | 2.404 | -0.797 | 50.4 |

Ideal-fit role fingerprints:

| Role | Min | Goals | Shots | Passes C | Pass% | Tackles | Interceptions | Rating |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| `LinkForward` | 93.9 | 1.61 | 12.62 | 4.39 | 75.8 | 0.00 | 0.00 | 9.21 |
| `AdvancedPlaymaker` | 93.9 | 0.00 | 0.00 | 4.40 | 81.4 | 2.94 | 3.01 | 7.45 |
| `BoxToBoxMidfielder` | 93.8 | 0.00 | 0.00 | 3.48 | 75.7 | 2.92 | 2.88 | 7.32 |
| `HoldingMidfielder` | 93.9 | 0.00 | 0.00 | 3.22 | 74.6 | 2.47 | 2.49 | 7.18 |
| `WideProgressor` | 93.9 | 0.00 | 0.00 | 2.83 | 75.4 | 2.35 | 2.42 | 7.16 |
| `Goalkeeper` | 93.9 | 0.00 | 0.00 | 1.30 | 100.0 | 0.00 | 0.00 | 6.53 |

Ideal-fit punishes:

| Opponent | Formation | Style | Vs Bad | Vs Neutral | Vs Ideal | Ideal GD |
| --- | --- | --- | --- | --- | --- | --- |
| `4-2-3-1__Attacking` | `4-2-3-1` | `Attacking` | 1.375 | 0.875 | 1.625 | 0.083 |
| `4-2-3-1__HighPress` | `4-2-3-1` | `HighPress` | 1.250 | 1.792 | 1.500 | 0.500 |
| `4-1-4-1__HighPress` | `4-1-4-1` | `HighPress` | 1.417 | 1.333 | 1.500 | -0.083 |
| `4-5-1__Defensive` | `4-5-1` | `Defensive` | 1.333 | 1.292 | 1.458 | -0.042 |
| `4-4-2__Balanced` | `4-4-2` | `Balanced` | 1.458 | 0.833 | 1.417 | -0.167 |

Ideal-fit counters:

| Opponent | Formation | Style | Vs Bad | Vs Neutral | Vs Ideal | Ideal GD |
| --- | --- | --- | --- | --- | --- | --- |
| `4-4-2__Attacking` | `4-4-2` | `Attacking` | 0.875 | 0.625 | 0.042 | -4.500 |
| `3-4-3__Counter` | `3-4-3` | `Counter` | 0.833 | 0.583 | 0.250 | -2.875 |
| `5-3-2__Defensive` | `5-3-2` | `Defensive` | 1.083 | 0.792 | 0.250 | -1.750 |
| `5-3-2__Attacking` | `5-3-2` | `Attacking` | 0.500 | 0.917 | 0.333 | -3.833 |
| `3-5-2__Counter` | `3-5-2` | `Counter` | 1.042 | 0.708 | 0.375 | -2.750 |

## 4-2-3-1__Attacking

- Formation: `4-2-3-1`
- Style: `Attacking`
- Target package: `ChaosRaiders`
- Classification: `flat_curve`
- Fit curve: bad 0.896 PPG, neutral 0.971, ideal 0.988
- Fit deltas: ideal-bad 0.092, ideal-neutral 0.017, neutral-bad 0.075

Profiles:

| Profile | PPG | GF | GA | GD | Poss% |
| --- | --- | --- | --- | --- | --- |
| `BadFit` | 0.896 | 1.522 | 2.486 | -0.964 | 50.2 |
| `NeutralFit` | 0.971 | 1.623 | 2.472 | -0.850 | 50.5 |
| `IdealFit` | 0.988 | 1.650 | 2.445 | -0.795 | 50.7 |

Ideal-fit role fingerprints:

| Role | Min | Goals | Shots | Passes C | Pass% | Tackles | Interceptions | Rating |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| `LinkForward` | 94.0 | 1.65 | 12.96 | 4.49 | 76.4 | 0.00 | 0.00 | 9.27 |
| `AdvancedPlaymaker` | 93.9 | 0.00 | 0.00 | 4.39 | 80.6 | 2.96 | 3.01 | 7.46 |
| `DeepPlaymaker` | 94.0 | 0.00 | 0.00 | 4.15 | 80.1 | 2.75 | 2.82 | 7.38 |
| `HoldingMidfielder` | 93.9 | 0.00 | 0.00 | 3.20 | 75.7 | 2.45 | 2.45 | 7.18 |
| `WideProgressor` | 93.9 | 0.00 | 0.00 | 2.75 | 75.1 | 2.33 | 2.36 | 7.15 |
| `Goalkeeper` | 94.0 | 0.00 | 0.00 | 1.38 | 100.0 | 0.00 | 0.00 | 6.55 |

Ideal-fit punishes:

| Opponent | Formation | Style | Vs Bad | Vs Neutral | Vs Ideal | Ideal GD |
| --- | --- | --- | --- | --- | --- | --- |
| `4-5-1__HighPress` | `4-5-1` | `HighPress` | 2.000 | 1.333 | 1.583 | 0.042 |
| `4-1-4-1__Attacking` | `4-1-4-1` | `Attacking` | 1.583 | 1.375 | 1.500 | -0.125 |
| `4-2-3-1__HighPress` | `4-2-3-1` | `HighPress` | 1.417 | 1.708 | 1.458 | -0.042 |
| `4-2-3-1__Balanced` | `4-2-3-1` | `Balanced` | 1.667 | 1.458 | 1.375 | 0.083 |
| `4-2-3-1__Counter` | `4-2-3-1` | `Counter` | 1.208 | 1.208 | 1.292 | 0.042 |

Ideal-fit counters:

| Opponent | Formation | Style | Vs Bad | Vs Neutral | Vs Ideal | Ideal GD |
| --- | --- | --- | --- | --- | --- | --- |
| `3-4-3__Counter` | `3-4-3` | `Counter` | 1.333 | 1.042 | 0.042 | -2.708 |
| `3-5-2__Defensive` | `3-5-2` | `Defensive` | 0.792 | 0.458 | 0.167 | -1.875 |
| `4-4-2__Attacking` | `4-4-2` | `Attacking` | 0.583 | 0.875 | 0.208 | -3.167 |
| `3-5-2__Possession` | `3-5-2` | `Possession` | 1.250 | 0.542 | 0.208 | -1.958 |
| `3-5-2__Attacking` | `3-5-2` | `Attacking` | 1.208 | 0.667 | 0.250 | -4.458 |

