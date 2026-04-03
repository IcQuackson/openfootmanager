#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
output_dir="$repo_root/docs/benchmarks"
matches_per_leg=200
allow_dirty=0

usage() {
  cat <<'USAGE'
Usage: scripts/render-tactic-matrix-benchmark.sh [--matches-per-leg N] [--output-dir PATH] [--allow-dirty]

Runs the engine tactic matrix benchmark and writes JSON plus Markdown matchup
reports under docs/benchmarks by default.
USAGE
}

while [[ $# -gt 0 ]]; do
  case "$1" in
    --matches-per-leg)
      matches_per_leg="${2:?Missing value for --matches-per-leg}"
      shift 2
      ;;
    --output-dir)
      output_dir="${2:?Missing value for --output-dir}"
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

tmp_benchmark_json="$(mktemp)"
cleanup() {
  rm -f "$tmp_benchmark_json"
}
trap cleanup EXIT

(
  cd "$repo_root/src-tauri"
  cargo run --quiet -p engine --example tactic_matrix --release -- --matches-per-leg "$matches_per_leg"
) > "$tmp_benchmark_json"

mkdir -p "$output_dir"
json_file="$output_dir/engine-tactic-matrix.json"
rankings_file="$output_dir/engine-tactic-rankings.md"
matchups_file="$output_dir/engine-tactic-matchups.md"

COMMIT_SHA="$commit_sha" \
COMMIT_SHORT="$commit_short" \
COMMIT_BRANCH="$branch" \
COMMIT_DATE="$commit_date" \
RECORDED_AT="$recorded_at" \
DIRTY="$dirty" \
BENCHMARK_JSON_FILE="$tmp_benchmark_json" \
JSON_FILE="$json_file" \
RANKINGS_FILE="$rankings_file" \
MATCHUPS_FILE="$matchups_file" \
node <<'NODE'
const fs = require('fs');

const benchmark = JSON.parse(fs.readFileSync(process.env.BENCHMARK_JSON_FILE, 'utf8'));
const meta = {
  recorded_at: process.env.RECORDED_AT,
  commit_sha: process.env.COMMIT_SHA,
  commit_short: process.env.COMMIT_SHORT,
  branch: process.env.COMMIT_BRANCH,
  commit_date: process.env.COMMIT_DATE,
  dirty: process.env.DIRTY === '1',
};

const payload = {
  metadata: meta,
  benchmark,
};

const fmt = (value, digits = 3) => Number(value).toFixed(digits);

function summaryTable(items, columns) {
  const header = `| ${columns.map((c) => c.label).join(' | ')} |`;
  const sep = `| ${columns.map(() => '---').join(' | ')} |`;
  const rows = items.map((item, index) => `| ${columns.map((c) => c.render(item, index)).join(' | ')} |`);
  return [header, sep, ...rows].join('\n');
}

function tacticLabel(tactic) {
  return `\`${tactic.id}\``;
}

const rankings = [];
rankings.push('# Engine Tactic Rankings');
rankings.push('');
rankings.push(`Recorded: ${meta.recorded_at}`);
rankings.push(`Commit: ${meta.commit_short} (${meta.branch})`);
rankings.push(`Commit date: ${meta.commit_date}`);
rankings.push(`Dirty tracked worktree: ${meta.dirty ? 'yes' : 'no'}`);
rankings.push(`Matches per leg: ${benchmark.matches_per_leg}`);
rankings.push('');
rankings.push('## Notes');
rankings.push('');
for (const note of benchmark.notes) rankings.push(`- ${note}`);
rankings.push('');
rankings.push('## Style Summary');
rankings.push('');
rankings.push(summaryTable(benchmark.style_summary, [
  { label: 'Style', render: (row) => `\`${row.id}\`` },
  { label: 'Avg PPG', render: (row) => fmt(row.average_points_per_match) },
  { label: 'Avg GD', render: (row) => fmt(row.average_goal_diff_per_match) },
  { label: 'Avg GF', render: (row) => fmt(row.average_goals_for_per_match) },
  { label: 'Avg GA', render: (row) => fmt(row.average_goals_against_per_match) },
  { label: 'Avg Poss%', render: (row) => fmt(row.average_possession, 1) },
]));
rankings.push('');
rankings.push('## Shape Summary');
rankings.push('');
rankings.push(summaryTable(benchmark.shape_summary, [
  { label: 'Shape', render: (row) => `\`${row.id}\`` },
  { label: 'Avg PPG', render: (row) => fmt(row.average_points_per_match) },
  { label: 'Avg GD', render: (row) => fmt(row.average_goal_diff_per_match) },
  { label: 'Avg GF', render: (row) => fmt(row.average_goals_for_per_match) },
  { label: 'Avg GA', render: (row) => fmt(row.average_goals_against_per_match) },
  { label: 'Avg Poss%', render: (row) => fmt(row.average_possession, 1) },
]));
rankings.push('');
rankings.push('## Overall Tactic Ranking');
rankings.push('');
rankings.push(summaryTable(benchmark.tactics, [
  { label: 'Rank', render: (_row, index) => String(index + 1) },
  { label: 'Tactic', render: (row) => tacticLabel(row) },
  { label: 'PPG', render: (row) => fmt(row.overall.points_per_match) },
  { label: 'GD', render: (row) => fmt(row.overall.goal_diff_per_match) },
  { label: 'GF', render: (row) => fmt(row.overall.goals_for_per_match) },
  { label: 'GA', render: (row) => fmt(row.overall.goals_against_per_match) },
  { label: 'Poss%', render: (row) => fmt(row.overall.avg_possession, 1) },
  { label: 'Best Matchup', render: (row) => row.strongest_against[0] ? `\`${row.strongest_against[0].opponent_id}\` (${fmt(row.strongest_against[0].summary.points_per_match)})` : '-' },
  { label: 'Worst Matchup', render: (row) => row.weakest_against[0] ? `\`${row.weakest_against[0].opponent_id}\` (${fmt(row.weakest_against[0].summary.points_per_match)})` : '-' },
]));

const matchups = [];
matchups.push('# Engine Tactic Matchups');
matchups.push('');
matchups.push(`Recorded: ${meta.recorded_at}`);
matchups.push(`Commit: ${meta.commit_short} (${meta.branch})`);
matchups.push(`Matches per leg: ${benchmark.matches_per_leg}`);
matchups.push('');
matchups.push('Each section lists the tactic overall first, then its full opponent table ordered from strongest to weakest matchup.');
matchups.push('');

for (const tactic of benchmark.tactics) {
  matchups.push(`## ${tactic.id}`);
  matchups.push('');
  if (tactic.shape_aliases.length > 1) {
    matchups.push(`Aliases in current engine: ${tactic.shape_aliases.map((alias) => `\`${alias}\``).join(', ')}`);
    matchups.push('');
  }
  matchups.push(summaryTable([tactic], [
    { label: 'Tactic', render: (row) => tacticLabel(row) },
    { label: 'PPG', render: (row) => fmt(row.overall.points_per_match) },
    { label: 'GD', render: (row) => fmt(row.overall.goal_diff_per_match) },
    { label: 'GF', render: (row) => fmt(row.overall.goals_for_per_match) },
    { label: 'GA', render: (row) => fmt(row.overall.goals_against_per_match) },
    { label: 'Poss%', render: (row) => fmt(row.overall.avg_possession, 1) },
  ]));
  matchups.push('');
  matchups.push(summaryTable(tactic.matchups, [
    { label: 'Opponent', render: (row) => `\`${row.opponent_id}\`` },
    { label: 'Shape', render: (row) => `\`${row.opponent_shape}\`` },
    { label: 'Style', render: (row) => `\`${row.opponent_style}\`` },
    { label: 'PPG', render: (row) => fmt(row.summary.points_per_match) },
    { label: 'GD', render: (row) => fmt(row.summary.goal_diff_per_match) },
    { label: 'GF', render: (row) => fmt(row.summary.goals_for_per_match) },
    { label: 'GA', render: (row) => fmt(row.summary.goals_against_per_match) },
    { label: 'Poss%', render: (row) => fmt(row.summary.avg_possession, 1) },
  ]));
  matchups.push('');
}

fs.writeFileSync(process.env.JSON_FILE, JSON.stringify(payload, null, 2) + '\n');
fs.writeFileSync(process.env.RANKINGS_FILE, rankings.join('\n') + '\n');
fs.writeFileSync(process.env.MATCHUPS_FILE, matchups.join('\n') + '\n');
NODE

echo "Wrote tactic benchmark artifacts:"
echo "  json: $json_file"
echo "  rankings: $rankings_file"
echo "  matchups: $matchups_file"
