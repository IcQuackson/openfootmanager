## Engine Benchmarks

`engine-balanced-history.jsonl` stores one JSON object per recorded benchmark run.

Default scenario:

- `5000` seeded matches
- balanced vs balanced
- `4-4-2`
- skill `65` vs `65`

Record a result for the current commit:

```bash
npm run benchmark:engine:record
```

If you need to record against a dirty tracked worktree:

```bash
bash scripts/record-engine-benchmark.sh --allow-dirty
```

Run a smaller local sample into a temporary history file:

```bash
bash scripts/record-engine-benchmark.sh --matches 100 --history-file /tmp/engine-history.jsonl --allow-dirty
```

## Tactic Matrix Benchmark

`benchmark:engine:tactics` runs a full tactic-vs-tactic matrix and writes artifacts under `docs/benchmarks`:

- `engine-tactic-matrix.json`
- `engine-tactic-rankings.md`
- `engine-tactic-matchups.md`

Default scenario:

- `200` matches per leg
- home/away balancing for every unordered tactic pair
- `6` supported shapes by current engine interpretation
- `6` play styles
- equal-quality role-specific synthetic squads

Run it:

```bash
npm run benchmark:engine:tactics
```

Run a smaller local sample:

```bash
bash scripts/render-tactic-matrix-benchmark.sh --matches-per-leg 20 --allow-dirty
```

Run a larger lower-variance sample:

```bash
bash scripts/render-tactic-matrix-benchmark.sh --matches-per-leg 50 --allow-dirty
```

## Role, Formation, and Trait Matrix Benchmark

`benchmark:engine:role-traits` runs a deeper blueprint-vs-blueprint matrix. Each blueprint combines:

- formation
- team play style
- formation slot roles
- a style-aligned player trait package

Artifacts written under `docs/benchmarks`:

- `engine-role-trait-matrix.json`
- `engine-role-trait-rankings.md`
- `engine-role-trait-exploits.md`
- `engine-role-trait-matchups.md`

Default scenario:

- `60` matches per leg
- home/away balancing for every unordered blueprint pair
- `8` formations
- `6` play styles
- synthetic role-specific squads with trait packages to pressure-test engine behavior

Run it:

```bash
npm run benchmark:engine:role-traits
```

Run a smaller local sample:

```bash
bash scripts/render-role-trait-matrix-benchmark.sh --matches-per-leg 10 --allow-dirty
```

Run a larger lower-variance sample:

```bash
bash scripts/render-role-trait-matrix-benchmark.sh --matches-per-leg 25 --allow-dirty
```

## Fit Profile Matrix Benchmark

`benchmark:engine:fit-profiles` measures how much each system depends on having the right squad.

Each system is tested with three squad profiles:

- `BadFit`
- `NeutralFit`
- `IdealFit`

Artifacts written under `docs/benchmarks`:

- `engine-fit-profile-matrix.json`
- `engine-fit-profile-rankings.md`
- `engine-fit-profile-analysis.md`

What it answers:

- which systems only become strong with tailored players
- which systems stay too strong even with the wrong players
- how ideal-fit systems perform against bad-fit, neutral-fit, and ideal-fit versions of their opponents

Run it:

```bash
npm run benchmark:engine:fit-profiles
```

Run a smaller local sample:

```bash
bash scripts/render-fit-profile-matrix-benchmark.sh --matches-per-leg 4 --allow-dirty
```

Run a larger lower-variance sample:

```bash
bash scripts/render-fit-profile-matrix-benchmark.sh --matches-per-leg 12 --allow-dirty
```

Roadmap:

- [engine-balance-roadmap.md](/home/quackson/Desktop/Coding/openfootmanager/docs/benchmarks/engine-balance-roadmap.md)
- [engine-tuning-framework.md](/home/quackson/Desktop/Coding/openfootmanager/docs/benchmarks/engine-tuning-framework.md)
- [engine-traits-players.md](/home/quackson/Desktop/Coding/openfootmanager/docs/benchmarks/engine-traits-players.md)
- [engine-trait-action-phases.md](/home/quackson/Desktop/Coding/openfootmanager/docs/benchmarks/engine-trait-action-phases.md)
- [engine-spatial-2d-plan.md](/home/quackson/Desktop/Coding/openfootmanager/docs/benchmarks/engine-spatial-2d-plan.md)
