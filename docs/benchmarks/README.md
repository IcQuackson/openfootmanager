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
- equal-quality role-specific archetype squads

Run it:

```bash
npm run benchmark:engine:tactics
```

Run a smaller local sample:

```bash
bash scripts/render-tactic-matrix-benchmark.sh --matches-per-leg 20 --allow-dirty
```

Important engine note:

- `4-5-1`, `4-2-3-1`, and `4-1-4-1` currently collapse to the same tactical shape in the benchmark because the engine only distinguishes defender/midfielder/forward counts there.

Roadmap:

- [engine-balance-roadmap.md](/home/quackson/Desktop/Coding/openfootmanager/docs/benchmarks/engine-balance-roadmap.md)
