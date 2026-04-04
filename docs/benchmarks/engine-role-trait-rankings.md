# Engine Role/Trait Rankings

Recorded: 2026-04-03T20:43:00Z
Commit: 6036f4c (custom)
Commit date: 2026-04-03T21:14:14+01:00
Dirty tracked worktree: yes
Matches per leg: 25

## Notes

- Each blueprint combines formation, play style, tactical role layout, and a style-aligned trait package.
- The role/trait packages are intentionally synthetic so the benchmark surfaces engine exploits and counters rather than mirroring a specific save.
- Exploit watchlist entries are simply the highest-performing blueprints in the current matrix; treat them as engine pressure tests, not final game balance recommendations.

## Style Summary

| Style | Avg PPG | Avg GD | Avg GF | Avg GA | Avg Poss% |
| --- | --- | --- | --- | --- | --- |
| `Counter` | 1.576 | 0.374 | 2.052 | 1.678 | 47.8 |
| `Possession` | 1.455 | 0.110 | 1.640 | 1.529 | 52.5 |
| `Defensive` | 1.439 | 0.111 | 1.203 | 1.092 | 50.4 |
| `Attacking` | 1.322 | -0.151 | 2.509 | 2.659 | 49.6 |
| `Balanced` | 1.289 | -0.211 | 1.824 | 2.036 | 49.6 |
| `HighPress` | 1.259 | -0.233 | 1.848 | 2.081 | 50.2 |

## Formation Summary

| Formation | Avg PPG | Avg GD | Avg GF | Avg GA | Avg Poss% |
| --- | --- | --- | --- | --- | --- |
| `3-5-2` | 1.708 | 0.523 | 2.294 | 1.771 | 50.7 |
| `5-3-2` | 1.583 | 0.325 | 2.139 | 1.814 | 49.1 |
| `3-4-3` | 1.573 | 0.302 | 2.170 | 1.868 | 49.1 |
| `4-4-2` | 1.537 | 0.253 | 2.213 | 1.960 | 49.2 |
| `4-3-3` | 1.472 | 0.135 | 2.084 | 1.949 | 49.3 |
| `4-1-4-1` | 1.128 | -0.425 | 1.317 | 1.742 | 51.5 |
| `4-2-3-1` | 1.077 | -0.535 | 1.291 | 1.826 | 50.8 |
| `4-5-1` | 1.044 | -0.579 | 1.260 | 1.839 | 50.4 |

## Trait Package Summary

| Package | Avg PPG | Avg GD | Avg GF | Avg GA | Avg Poss% |
| --- | --- | --- | --- | --- | --- |
| `LaunchCounter` | 1.576 | 0.374 | 2.052 | 1.678 | 47.8 |
| `ControlCore` | 1.455 | 0.110 | 1.640 | 1.529 | 52.5 |
| `LaneThiefBlock` | 1.439 | 0.111 | 1.203 | 1.092 | 50.4 |
| `ChaosRaiders` | 1.322 | -0.151 | 2.509 | 2.659 | 49.6 |
| `Baseline` | 1.289 | -0.211 | 1.824 | 2.036 | 49.6 |
| `PressTrap` | 1.259 | -0.233 | 1.848 | 2.081 | 50.2 |

## Overall Blueprint Ranking

| Rank | Blueprint | Formation | Style | Package | PPG | GD | GF | GA | Poss% | Exploit Edge | Counter |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| 1 | `3-5-2__Counter__LaunchCounter` | `3-5-2` | `Counter` | `LaunchCounter` | 1.990 | 1.070 | 2.677 | 1.607 | 48.5 | `4-5-1__Counter__LaunchCounter` (2.800) | `5-3-2__Defensive__LaneThiefBlock` (1.360) |
| 2 | `4-4-2__Counter__LaunchCounter` | `4-4-2` | `Counter` | `LaunchCounter` | 1.896 | 0.930 | 2.679 | 1.750 | 47.4 | `4-2-3-1__Attacking__ChaosRaiders` (2.760) | `3-5-2__Counter__LaunchCounter` (1.200) |
| 3 | `5-3-2__Counter__LaunchCounter` | `5-3-2` | `Counter` | `LaunchCounter` | 1.823 | 0.843 | 2.534 | 1.691 | 46.4 | `4-5-1__Balanced__Baseline` (2.580) | `3-5-2__Defensive__LaneThiefBlock` (1.140) |
| 4 | `3-4-3__Counter__LaunchCounter` | `3-4-3` | `Counter` | `LaunchCounter` | 1.822 | 0.779 | 2.443 | 1.664 | 47.4 | `4-2-3-1__Balanced__Baseline` (2.620) | `3-5-2__Possession__ControlCore` (0.780) |
| 5 | `3-5-2__Attacking__ChaosRaiders` | `3-5-2` | `Attacking` | `ChaosRaiders` | 1.723 | 0.671 | 3.231 | 2.560 | 50.3 | `4-5-1__Attacking__ChaosRaiders` (2.780) | `4-4-2__Counter__LaunchCounter` (0.720) |
| 6 | `3-5-2__Defensive__LaneThiefBlock` | `3-5-2` | `Defensive` | `LaneThiefBlock` | 1.695 | 0.450 | 1.485 | 1.036 | 51.2 | `4-2-3-1__HighPress__PressTrap` (2.280) | `3-5-2__Counter__LaunchCounter` (1.120) |
| 7 | `3-5-2__Possession__ControlCore` | `3-5-2` | `Possession` | `ControlCore` | 1.684 | 0.450 | 1.928 | 1.478 | 52.9 | `4-5-1__Attacking__ChaosRaiders` (2.380) | `5-3-2__Defensive__LaneThiefBlock` (1.100) |
| 8 | `4-3-3__Counter__LaunchCounter` | `4-3-3` | `Counter` | `LaunchCounter` | 1.669 | 0.511 | 2.315 | 1.804 | 46.8 | `4-1-4-1__HighPress__PressTrap` (2.480) | `3-5-2__Counter__LaunchCounter` (0.880) |
| 9 | `5-3-2__Defensive__LaneThiefBlock` | `5-3-2` | `Defensive` | `LaneThiefBlock` | 1.665 | 0.391 | 1.420 | 1.029 | 49.6 | `4-1-4-1__Attacking__ChaosRaiders` (2.320) | `4-4-2__Counter__LaunchCounter` (0.900) |
| 10 | `3-4-3__Attacking__ChaosRaiders` | `3-4-3` | `Attacking` | `ChaosRaiders` | 1.650 | 0.433 | 3.147 | 2.715 | 48.5 | `4-2-3-1__Attacking__ChaosRaiders` (2.700) | `4-4-2__Counter__LaunchCounter` (0.720) |
| 11 | `3-5-2__Balanced__Baseline` | `3-5-2` | `Balanced` | `Baseline` | 1.613 | 0.310 | 2.261 | 1.951 | 50.4 | `4-5-1__Attacking__ChaosRaiders` (2.440) | `3-5-2__Counter__LaunchCounter` (0.820) |
| 12 | `5-3-2__Attacking__ChaosRaiders` | `5-3-2` | `Attacking` | `ChaosRaiders` | 1.577 | 0.321 | 2.957 | 2.636 | 48.5 | `4-5-1__Attacking__ChaosRaiders` (2.660) | `3-4-3__Counter__LaunchCounter` (0.920) |
| 13 | `5-3-2__Possession__ControlCore` | `5-3-2` | `Possession` | `ControlCore` | 1.563 | 0.270 | 1.782 | 1.513 | 51.8 | `4-5-1__Attacking__ChaosRaiders` (2.200) | `3-5-2__Defensive__LaneThiefBlock` (1.000) |
| 14 | `4-4-2__Attacking__ChaosRaiders` | `4-4-2` | `Attacking` | `ChaosRaiders` | 1.558 | 0.343 | 3.140 | 2.797 | 48.7 | `4-5-1__Attacking__ChaosRaiders` (2.480) | `3-5-2__Counter__LaunchCounter` (0.840) |
| 15 | `4-4-2__Defensive__LaneThiefBlock` | `4-4-2` | `Defensive` | `LaneThiefBlock` | 1.545 | 0.250 | 1.401 | 1.152 | 49.6 | `4-5-1__Attacking__ChaosRaiders` (2.440) | `3-5-2__Defensive__LaneThiefBlock` (0.880) |
| 16 | `3-5-2__HighPress__PressTrap` | `3-5-2` | `HighPress` | `PressTrap` | 1.542 | 0.188 | 2.181 | 1.993 | 51.0 | `4-5-1__Attacking__ChaosRaiders` (2.280) | `5-3-2__Counter__LaunchCounter` (0.840) |
| 17 | `3-4-3__Possession__ControlCore` | `3-4-3` | `Possession` | `ControlCore` | 1.535 | 0.213 | 1.796 | 1.583 | 51.2 | `4-1-4-1__HighPress__PressTrap` (2.300) | `3-4-3__Counter__LaunchCounter` (1.000) |
| 18 | `3-4-3__Defensive__LaneThiefBlock` | `3-4-3` | `Defensive` | `LaneThiefBlock` | 1.534 | 0.220 | 1.334 | 1.114 | 49.5 | `4-2-3-1__Attacking__ChaosRaiders` (2.420) | `5-3-2__HighPress__PressTrap` (0.960) |
| 19 | `4-3-3__Possession__ControlCore` | `4-3-3` | `Possession` | `ControlCore` | 1.489 | 0.168 | 1.758 | 1.590 | 51.8 | `4-5-1__Attacking__ChaosRaiders` (2.120) | `3-5-2__Balanced__Baseline` (0.960) |
| 20 | `4-3-3__Attacking__ChaosRaiders` | `4-3-3` | `Attacking` | `ChaosRaiders` | 1.487 | 0.149 | 2.945 | 2.796 | 48.9 | `4-2-3-1__Attacking__ChaosRaiders` (2.600) | `4-4-2__Counter__LaunchCounter` (0.520) |
| 21 | `3-4-3__Balanced__Baseline` | `3-4-3` | `Balanced` | `Baseline` | 1.486 | 0.125 | 2.167 | 2.042 | 48.9 | `4-1-4-1__HighPress__PressTrap` (2.280) | `3-5-2__Counter__LaunchCounter` (0.560) |
| 22 | `4-4-2__Possession__ControlCore` | `4-4-2` | `Possession` | `ControlCore` | 1.474 | 0.125 | 1.785 | 1.660 | 51.2 | `4-5-1__Balanced__Baseline` (2.340) | `3-5-2__Counter__LaunchCounter` (0.760) |
| 23 | `5-3-2__Balanced__Baseline` | `5-3-2` | `Balanced` | `Baseline` | 1.456 | 0.098 | 2.081 | 1.983 | 48.5 | `4-1-4-1__HighPress__PressTrap` (2.280) | `4-4-2__Counter__LaunchCounter` (0.700) |
| 24 | `4-3-3__Defensive__LaneThiefBlock` | `4-3-3` | `Defensive` | `LaneThiefBlock` | 1.449 | 0.100 | 1.306 | 1.206 | 49.7 | `4-2-3-1__Attacking__ChaosRaiders` (2.080) | `3-5-2__Counter__LaunchCounter` (0.940) |
| 25 | `4-1-4-1__Possession__ControlCore` | `4-1-4-1` | `Possession` | `ControlCore` | 1.415 | 0.053 | 1.440 | 1.387 | 54.9 | `4-5-1__Attacking__ChaosRaiders` (2.020) | `3-5-2__Counter__LaunchCounter` (0.740) |
| 26 | `4-4-2__Balanced__Baseline` | `4-4-2` | `Balanced` | `Baseline` | 1.415 | -0.012 | 2.142 | 2.154 | 48.8 | `4-5-1__Balanced__Baseline` (2.280) | `3-5-2__Counter__LaunchCounter` (0.580) |
| 27 | `3-4-3__HighPress__PressTrap` | `3-4-3` | `HighPress` | `PressTrap` | 1.413 | 0.041 | 2.130 | 2.089 | 49.4 | `4-5-1__HighPress__PressTrap` (2.220) | `3-5-2__Counter__LaunchCounter` (0.800) |
| 28 | `5-3-2__HighPress__PressTrap` | `5-3-2` | `HighPress` | `PressTrap` | 1.410 | 0.027 | 2.058 | 2.030 | 49.5 | `4-1-4-1__Attacking__ChaosRaiders` (1.960) | `3-5-2__Counter__LaunchCounter` (0.380) |
| 29 | `4-3-3__HighPress__PressTrap` | `4-3-3` | `HighPress` | `PressTrap` | 1.384 | 0.000 | 2.133 | 2.133 | 49.8 | `4-5-1__Attacking__ChaosRaiders` (2.300) | `3-5-2__Defensive__LaneThiefBlock` (0.440) |
| 30 | `4-3-3__Balanced__Baseline` | `4-3-3` | `Balanced` | `Baseline` | 1.353 | -0.115 | 2.047 | 2.162 | 48.6 | `4-5-1__Attacking__ChaosRaiders` (2.000) | `5-3-2__Counter__LaunchCounter` (0.380) |
| 31 | `4-4-2__HighPress__PressTrap` | `4-4-2` | `HighPress` | `PressTrap` | 1.334 | -0.115 | 2.133 | 2.248 | 49.2 | `4-5-1__Balanced__Baseline` (2.180) | `5-3-2__Counter__LaunchCounter` (0.560) |
| 32 | `4-2-3-1__Possession__ControlCore` | `4-2-3-1` | `Possession` | `ControlCore` | 1.263 | -0.180 | 1.333 | 1.513 | 53.4 | `4-5-1__Attacking__ChaosRaiders` (2.140) | `3-4-3__Counter__LaunchCounter` (0.740) |
| 33 | `4-1-4-1__Defensive__LaneThiefBlock` | `4-1-4-1` | `Defensive` | `LaneThiefBlock` | 1.220 | -0.140 | 0.898 | 1.038 | 51.7 | `4-5-1__Attacking__ChaosRaiders` (2.040) | `5-3-2__Possession__ControlCore` (0.760) |
| 34 | `4-2-3-1__Defensive__LaneThiefBlock` | `4-2-3-1` | `Defensive` | `LaneThiefBlock` | 1.218 | -0.169 | 0.915 | 1.084 | 51.0 | `4-2-3-1__Attacking__ChaosRaiders` (1.920) | `3-4-3__Counter__LaunchCounter` (0.640) |
| 35 | `4-5-1__Possession__ControlCore` | `4-5-1` | `Possession` | `ControlCore` | 1.217 | -0.215 | 1.294 | 1.509 | 52.6 | `4-2-3-1__Balanced__Baseline` (1.880) | `4-4-2__Counter__LaunchCounter` (0.660) |
| 36 | `4-5-1__Defensive__LaneThiefBlock` | `4-5-1` | `Defensive` | `LaneThiefBlock` | 1.182 | -0.216 | 0.864 | 1.080 | 50.9 | `4-2-3-1__Balanced__Baseline` (1.680) | `3-5-2__Attacking__ChaosRaiders` (0.500) |
| 37 | `4-2-3-1__Counter__LaunchCounter` | `4-2-3-1` | `Counter` | `LaunchCounter` | 1.158 | -0.348 | 1.287 | 1.635 | 48.8 | `4-2-3-1__Attacking__ChaosRaiders` (1.880) | `4-4-2__Counter__LaunchCounter` (0.460) |
| 38 | `4-1-4-1__Counter__LaunchCounter` | `4-1-4-1` | `Counter` | `LaunchCounter` | 1.157 | -0.329 | 1.257 | 1.586 | 49.0 | `4-1-4-1__Attacking__ChaosRaiders` (1.900) | `4-4-2__Counter__LaunchCounter` (0.500) |
| 39 | `4-5-1__Counter__LaunchCounter` | `4-5-1` | `Counter` | `LaunchCounter` | 1.095 | -0.462 | 1.225 | 1.688 | 48.3 | `4-5-1__Balanced__Baseline` (1.680) | `3-5-2__Counter__LaunchCounter` (0.100) |
| 40 | `4-1-4-1__Balanced__Baseline` | `4-1-4-1` | `Balanced` | `Baseline` | 1.064 | -0.588 | 1.337 | 1.926 | 50.9 | `4-5-1__HighPress__PressTrap` (2.080) | `3-5-2__Counter__LaunchCounter` (0.340) |
| 41 | `4-1-4-1__HighPress__PressTrap` | `4-1-4-1` | `HighPress` | `PressTrap` | 1.003 | -0.632 | 1.392 | 2.024 | 51.1 | `4-2-3-1__Balanced__Baseline` (1.720) | `4-3-3__Counter__LaunchCounter` (0.380) |
| 42 | `4-2-3-1__HighPress__PressTrap` | `4-2-3-1` | `HighPress` | `PressTrap` | 1.000 | -0.692 | 1.370 | 2.063 | 50.7 | `4-1-4-1__Attacking__ChaosRaiders` (1.600) | `3-4-3__Counter__LaunchCounter` (0.380) |
| 43 | `4-5-1__HighPress__PressTrap` | `4-5-1` | `HighPress` | `PressTrap` | 0.989 | -0.684 | 1.386 | 2.070 | 50.8 | `4-2-3-1__Attacking__ChaosRaiders` (1.680) | `3-5-2__Counter__LaunchCounter` (0.260) |
| 44 | `4-2-3-1__Balanced__Baseline` | `4-2-3-1` | `Balanced` | `Baseline` | 0.964 | -0.770 | 1.276 | 2.046 | 50.4 | `4-5-1__Attacking__ChaosRaiders` (1.600) | `4-4-2__Counter__LaunchCounter` (0.200) |
| 45 | `4-5-1__Balanced__Baseline` | `4-5-1` | `Balanced` | `Baseline` | 0.964 | -0.737 | 1.283 | 2.020 | 50.0 | `4-2-3-1__HighPress__PressTrap` (1.740) | `4-4-2__Counter__LaunchCounter` (0.240) |
| 46 | `4-1-4-1__Attacking__ChaosRaiders` | `4-1-4-1` | `Attacking` | `ChaosRaiders` | 0.908 | -0.911 | 1.578 | 2.489 | 51.3 | `4-5-1__Attacking__ChaosRaiders` (1.520) | `4-3-3__Attacking__ChaosRaiders` (0.320) |
| 47 | `4-2-3-1__Attacking__ChaosRaiders` | `4-2-3-1` | `Attacking` | `ChaosRaiders` | 0.859 | -1.050 | 1.568 | 2.618 | 50.3 | `4-5-1__Attacking__ChaosRaiders` (1.760) | `4-4-2__Counter__LaunchCounter` (0.180) |
| 48 | `4-5-1__Attacking__ChaosRaiders` | `4-5-1` | `Attacking` | `ChaosRaiders` | 0.815 | -1.160 | 1.505 | 2.665 | 49.9 | `4-1-4-1__HighPress__PressTrap` (1.400) | `3-5-2__Attacking__ChaosRaiders` (0.140) |
