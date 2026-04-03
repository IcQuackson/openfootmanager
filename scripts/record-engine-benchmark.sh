#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
history_file="$repo_root/docs/benchmarks/engine-balanced-history.jsonl"
matches=5000
allow_dirty=0

usage() {
  cat <<'EOF'
Usage: scripts/record-engine-benchmark.sh [--matches N] [--history-file PATH] [--allow-dirty]

Runs the engine balanced-vs-balanced benchmark and appends a JSONL record with
commit metadata and aggregate results.
EOF
}

while [[ $# -gt 0 ]]; do
  case "$1" in
    --matches)
      matches="${2:?Missing value for --matches}"
      shift 2
      ;;
    --history-file)
      history_file="${2:?Missing value for --history-file}"
      shift 2
      ;;
    --allow-dirty)
      allow_dirty=1
      shift
      ;;
    --help|-h)
      usage
      exit 0
      ;;
    *)
      echo "Unknown argument: $1" >&2
      usage >&2
      exit 1
      ;;
  esac
done

commit_sha="$(git -C "$repo_root" rev-parse HEAD)"
commit_short="$(git -C "$repo_root" rev-parse --short HEAD)"
branch="$(git -C "$repo_root" rev-parse --abbrev-ref HEAD)"
commit_date="$(git -C "$repo_root" show -s --format=%cI HEAD)"
recorded_at="$(date -u +%Y-%m-%dT%H:%M:%SZ)"
dirty=0

if ! git -C "$repo_root" diff --quiet || ! git -C "$repo_root" diff --cached --quiet; then
  dirty=1
fi

if [[ "$dirty" -eq 1 && "$allow_dirty" -ne 1 ]]; then
  echo "Tracked changes are present. Commit them first or rerun with --allow-dirty." >&2
  exit 1
fi

benchmark_json="$(
  cd "$repo_root/src-tauri"
  cargo run --quiet -p engine --example balanced_benchmark --release -- --matches "$matches"
)"

mkdir -p "$(dirname "$history_file")"

entry_json="$(
  COMMIT_SHA="$commit_sha" \
  COMMIT_SHORT="$commit_short" \
  COMMIT_BRANCH="$branch" \
  COMMIT_DATE="$commit_date" \
  RECORDED_AT="$recorded_at" \
  DIRTY="$dirty" \
  BENCHMARK_JSON="$benchmark_json" \
  node <<'EOF'
const benchmark = JSON.parse(process.env.BENCHMARK_JSON);

const entry = {
  recorded_at: process.env.RECORDED_AT,
  commit_sha: process.env.COMMIT_SHA,
  commit_short: process.env.COMMIT_SHORT,
  branch: process.env.COMMIT_BRANCH,
  commit_date: process.env.COMMIT_DATE,
  dirty: process.env.DIRTY === "1",
  benchmark,
};

process.stdout.write(`${JSON.stringify(entry)}\n`);
EOF
)"

printf '%s' "$entry_json" >> "$history_file"

echo "Recorded engine benchmark:"
echo "  commit: $commit_short"
echo "  branch: $branch"
echo "  dirty: $dirty"
echo "  history: $history_file"
