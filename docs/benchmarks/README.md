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
