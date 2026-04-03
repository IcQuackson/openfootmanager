#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
output_dir="$repo_root/docs/benchmarks"
matches_per_leg=50
allow_dirty=0

usage() {
  cat <<'USAGE'
Usage: scripts/render-archetype-tactic-matrix-benchmark.sh [--matches-per-leg N] [--output-dir PATH] [--allow-dirty]

Runs the archetype-aware tactic template benchmark and writes JSON plus Markdown
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
  cargo run --quiet -p engine --example archetype_tactic_matrix --release -- --matches-per-leg "$matches_per_leg"
) > "$tmp_benchmark_json"

mkdir -p "$output_dir"
json_file="$output_dir/engine-archetype-tactic-matrix.json"
rankings_file="$output_dir/engine-archetype-tactic-rankings.md"
matchups_file="$output_dir/engine-archetype-tactic-matchups.md"

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

const payload = { metadata: meta, benchmark };
const fmt = (value, digits = 3) => Number(value).toFixed(digits);

function summaryTable(items, columns) {
  const header = `| ${columns.map((c) => c.label).join(' | ')} |`;
  const sep = `| ${columns.map(() => '---').join(' | ')} |`;
  const rows = items.map((item, index) => `| ${columns.map((c) => c.render(item, index)).join(' | ')} |`);
  return [header, sep, ...rows].join('\n');
}

function templateLabel(row) {
  return `\`${row.id}\``;
}

const rankings = [];
rankings.push('# Engine Archetype Tactic Rankings');
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
rankings.push('## Squad Summary');
rankings.push('');
rankings.push(summaryTable(benchmark.squad_summary, [
  { label: 'Squad Type', render: (row) => `\`${row.id}\`` },
  { label: 'Avg PPG', render: (row) => fmt(row.average_points_per_match) },
  { label: 'Avg GD', render: (row) => fmt(row.average_goal_diff_per_match) },
  { label: 'Avg GF', render: (row) => fmt(row.average_goals_for_per_match) },
  { label: 'Avg GA', render: (row) => fmt(row.average_goals_against_per_match) },
  { label: 'Avg Poss%', render: (row) => fmt(row.average_possession, 1) },
]));
rankings.push('');
rankings.push('## Overall Template Ranking');
rankings.push('');
rankings.push(summaryTable(benchmark.templates, [
  { label: 'Rank', render: (_row, index) => String(index + 1) },
  { label: 'Template', render: (row) => templateLabel(row) },
  { label: 'Shape', render: (row) => `\`${row.shape}\`` },
  { label: 'Style', render: (row) => `\`${row.style}\`` },
  { label: 'Squad', render: (row) => `\`${row.squad_type}\`` },
  { label: 'PPG', render: (row) => fmt(row.overall.points_per_match) },
  { label: 'GD', render: (row) => fmt(row.overall.goal_diff_per_match) },
  { label: 'GF', render: (row) => fmt(row.overall.goals_for_per_match) },
  { label: 'GA', render: (row) => fmt(row.overall.goals_against_per_match) },
  { label: 'Best Squad To Face', render: (row) => row.thrives_against_squads[0] ? `\`${row.thrives_against_squads[0].squad_type}\` (${fmt(row.thrives_against_squads[0].summary.points_per_match)})` : '-' },
  { label: 'Worst Squad To Face', render: (row) => row.struggles_against_squads[0] ? `\`${row.struggles_against_squads[0].squad_type}\` (${fmt(row.struggles_against_squads[0].summary.points_per_match)})` : '-' },
]));

const matchups = [];
matchups.push('# Engine Archetype Tactic Matchups');
matchups.push('');
matchups.push(`Recorded: ${meta.recorded_at}`);
matchups.push(`Commit: ${meta.commit_short} (${meta.branch})`);
matchups.push(`Matches per leg: ${benchmark.matches_per_leg}`);
matchups.push('');
matchups.push('Each section shows the tactic template overall, then which opponent squad types it thrives with or suffers against, followed by the full template-by-template matchup table.');
matchups.push('');

for (const template of benchmark.templates) {
  matchups.push(`## ${template.id}`);
  matchups.push('');
  matchups.push(`Shape: \`${template.shape}\``);
  matchups.push(`Style: \`${template.style}\``);
  matchups.push(`Squad Type: \`${template.squad_type}\``);
  matchups.push(`Profile: ${template.summary}`);
  matchups.push('');
  matchups.push(summaryTable([template], [
    { label: 'Template', render: (row) => templateLabel(row) },
    { label: 'PPG', render: (row) => fmt(row.overall.points_per_match) },
    { label: 'GD', render: (row) => fmt(row.overall.goal_diff_per_match) },
    { label: 'GF', render: (row) => fmt(row.overall.goals_for_per_match) },
    { label: 'GA', render: (row) => fmt(row.overall.goals_against_per_match) },
    { label: 'Poss%', render: (row) => fmt(row.overall.avg_possession, 1) },
  ]));
  matchups.push('');
  matchups.push('### Best Opponent Squad Types');
  matchups.push('');
  matchups.push(summaryTable(template.thrives_against_squads, [
    { label: 'Opponent Squad', render: (row) => `\`${row.squad_type}\`` },
    { label: 'PPG', render: (row) => fmt(row.summary.points_per_match) },
    { label: 'GD', render: (row) => fmt(row.summary.goal_diff_per_match) },
    { label: 'GF', render: (row) => fmt(row.summary.goals_for_per_match) },
    { label: 'GA', render: (row) => fmt(row.summary.goals_against_per_match) },
    { label: 'Poss%', render: (row) => fmt(row.summary.avg_possession, 1) },
  ]));
  matchups.push('');
  matchups.push('### Toughest Opponent Squad Types');
  matchups.push('');
  matchups.push(summaryTable(template.struggles_against_squads, [
    { label: 'Opponent Squad', render: (row) => `\`${row.squad_type}\`` },
    { label: 'PPG', render: (row) => fmt(row.summary.points_per_match) },
    { label: 'GD', render: (row) => fmt(row.summary.goal_diff_per_match) },
    { label: 'GF', render: (row) => fmt(row.summary.goals_for_per_match) },
    { label: 'GA', render: (row) => fmt(row.summary.goals_against_per_match) },
    { label: 'Poss%', render: (row) => fmt(row.summary.avg_possession, 1) },
  ]));
  matchups.push('');
  matchups.push('### Template Matchups');
  matchups.push('');
  matchups.push(summaryTable(template.matchups, [
    { label: 'Opponent', render: (row) => `\`${row.opponent_id}\`` },
    { label: 'Shape', render: (row) => `\`${row.opponent_shape}\`` },
    { label: 'Style', render: (row) => `\`${row.opponent_style}\`` },
    { label: 'Squad', render: (row) => `\`${row.opponent_squad}\`` },
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

echo "Wrote archetype tactic benchmark artifacts:"
echo "  json: $json_file"
echo "  rankings: $rankings_file"
echo "  matchups: $matchups_file"
