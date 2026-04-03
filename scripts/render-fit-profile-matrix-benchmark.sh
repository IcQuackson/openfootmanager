#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
output_dir="$repo_root/docs/benchmarks"
matches_per_leg=12
allow_dirty=0

usage() {
  cat <<'USAGE'
Usage: scripts/render-fit-profile-matrix-benchmark.sh [--matches-per-leg N] [--output-dir PATH] [--allow-dirty]

Runs the engine fit-profile matrix benchmark and writes JSON plus Markdown
analysis artifacts under docs/benchmarks by default.
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
  cargo run --quiet -p engine --example fit_profile_matrix --release -- --matches-per-leg "$matches_per_leg"
) > "$tmp_benchmark_json"

mkdir -p "$output_dir"
json_file="$output_dir/engine-fit-profile-matrix.json"
rankings_file="$output_dir/engine-fit-profile-rankings.md"
analysis_file="$output_dir/engine-fit-profile-analysis.md"

COMMIT_SHA="$commit_sha" \
COMMIT_SHORT="$commit_short" \
COMMIT_BRANCH="$branch" \
COMMIT_DATE="$commit_date" \
RECORDED_AT="$recorded_at" \
DIRTY="$dirty" \
BENCHMARK_JSON_FILE="$tmp_benchmark_json" \
JSON_FILE="$json_file" \
RANKINGS_FILE="$rankings_file" \
ANALYSIS_FILE="$analysis_file" \
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
  const rows = Array.isArray(items) ? items : [];
  const header = `| ${columns.map((c) => c.label).join(' | ')} |`;
  const sep = `| ${columns.map(() => '---').join(' | ')} |`;
  const body = rows.map((item, index) => `| ${columns.map((c) => c.render(item, index)).join(' | ')} |`);
  return [header, sep, ...body].join('\n');
}

function responseTable(items) {
  return summaryTable(items, [
    { label: 'Opponent', render: (row) => `\`${row.opponent_system_id}\`` },
    { label: 'Formation', render: (row) => `\`${row.opponent_formation}\`` },
    { label: 'Style', render: (row) => `\`${row.opponent_style}\`` },
    { label: 'Vs Bad', render: (row) => fmt(row.bad_fit.points_per_match) },
    { label: 'Vs Neutral', render: (row) => fmt(row.neutral_fit.points_per_match) },
    { label: 'Vs Ideal', render: (row) => fmt(row.ideal_fit.points_per_match) },
    { label: 'Ideal GD', render: (row) => fmt(row.ideal_fit.goal_diff_per_match) },
  ]);
}

function roleTable(rows) {
  return summaryTable(rows, [
    { label: 'Role', render: (row) => `\`${row.role}\`` },
    { label: 'Min', render: (row) => fmt(row.avg_minutes, 1) },
    { label: 'Goals', render: (row) => fmt(row.avg_goals, 2) },
    { label: 'Shots', render: (row) => fmt(row.avg_shots, 2) },
    { label: 'Passes C', render: (row) => fmt(row.avg_passes_completed, 2) },
    { label: 'Pass%', render: (row) => fmt(row.avg_pass_accuracy, 1) },
    { label: 'Tackles', render: (row) => fmt(row.avg_tackles, 2) },
    { label: 'Interceptions', render: (row) => fmt(row.avg_interceptions, 2) },
    { label: 'Rating', render: (row) => fmt(row.avg_rating, 2) },
  ]);
}

const rankings = [];
rankings.push('# Engine Fit Profile Rankings');
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
rankings.push('## Fit Sensitivity Summary');
rankings.push('');
rankings.push(summaryTable(benchmark.fit_sensitivity_summary, [
  { label: 'Rank', render: (_row, index) => String(index + 1) },
  { label: 'System', render: (row) => `\`${row.id}\`` },
  { label: 'Formation', render: (row) => `\`${row.formation}\`` },
  { label: 'Style', render: (row) => `\`${row.style}\`` },
  { label: 'Class', render: (row) => `\`${row.classification}\`` },
  { label: 'Bad PPG', render: (row) => fmt(row.bad_fit_ppg) },
  { label: 'Neutral PPG', render: (row) => fmt(row.neutral_fit_ppg) },
  { label: 'Ideal PPG', render: (row) => fmt(row.ideal_fit_ppg) },
  { label: 'Ideal-Bad', render: (row) => fmt(row.ideal_minus_bad_ppg) },
  { label: 'Ideal-Neutral', render: (row) => fmt(row.ideal_minus_neutral_ppg) },
]));
rankings.push('');
rankings.push('## Player-Dependent Watchlist');
rankings.push('');
rankings.push(summaryTable(benchmark.player_dependent_watchlist, [
  { label: 'System', render: (row) => `\`${row.id}\`` },
  { label: 'Ideal PPG', render: (row) => fmt(row.ideal_fit_ppg) },
  { label: 'Bad PPG', render: (row) => fmt(row.bad_fit_ppg) },
  { label: 'Ideal-Bad', render: (row) => fmt(row.ideal_minus_bad_ppg) },
  { label: 'Ideal-Neutral', render: (row) => fmt(row.ideal_minus_neutral_ppg) },
]));
rankings.push('');
rankings.push('## Mechanically Overpowered Watchlist');
rankings.push('');
rankings.push(summaryTable(benchmark.mechanically_overpowered_watchlist, [
  { label: 'System', render: (row) => `\`${row.id}\`` },
  { label: 'Ideal PPG', render: (row) => fmt(row.ideal_fit_ppg) },
  { label: 'Bad PPG', render: (row) => fmt(row.bad_fit_ppg) },
  { label: 'Ideal-Bad', render: (row) => fmt(row.ideal_minus_bad_ppg) },
  { label: 'Ideal-Neutral', render: (row) => fmt(row.ideal_minus_neutral_ppg) },
]));

const analysis = [];
analysis.push('# Engine Fit Profile Analysis');
analysis.push('');
analysis.push(`Recorded: ${meta.recorded_at}`);
analysis.push(`Commit: ${meta.commit_short} (${meta.branch})`);
analysis.push(`Matches per leg: ${benchmark.matches_per_leg}`);
analysis.push('');
analysis.push('This report tests whether a system only becomes strong when the squad is tailored to it, or whether it stays too strong even with the wrong players. Each section is keyed off the ideal-fit version of the system, then shows how that ideal-fit system performs against bad-fit, neutral-fit, and ideal-fit versions of the opponent systems.');
analysis.push('');

for (const system of benchmark.systems) {
  analysis.push(`## ${system.id}`);
  analysis.push('');
  analysis.push(`- Formation: \`${system.formation}\``);
  analysis.push(`- Style: \`${system.style}\``);
  analysis.push(`- Target package: \`${system.target_package}\``);
  analysis.push(`- Classification: \`${system.fit_curve.classification}\``);
  analysis.push(`- Fit curve: bad ${fmt(system.fit_curve.bad_fit.points_per_match)} PPG, neutral ${fmt(system.fit_curve.neutral_fit.points_per_match)}, ideal ${fmt(system.fit_curve.ideal_fit.points_per_match)}`);
  analysis.push(`- Fit deltas: ideal-bad ${fmt(system.fit_curve.ideal_minus_bad_ppg)}, ideal-neutral ${fmt(system.fit_curve.ideal_minus_neutral_ppg)}, neutral-bad ${fmt(system.fit_curve.neutral_minus_bad_ppg)}`);
  analysis.push('');
  analysis.push('Profiles:');
  analysis.push('');
  analysis.push(summaryTable(system.profiles, [
    { label: 'Profile', render: (row) => `\`${row.fit_profile}\`` },
    { label: 'PPG', render: (row) => fmt(row.overall.points_per_match) },
    { label: 'GF', render: (row) => fmt(row.overall.goals_for_per_match) },
    { label: 'GA', render: (row) => fmt(row.overall.goals_against_per_match) },
    { label: 'GD', render: (row) => fmt(row.overall.goal_diff_per_match) },
    { label: 'Poss%', render: (row) => fmt(row.overall.avg_possession, 1) },
  ]));
  analysis.push('');

  const ideal = system.profiles.find((profile) => profile.fit_profile === 'IdealFit');
  if (ideal) {
    analysis.push('Ideal-fit role fingerprints:');
    analysis.push('');
    analysis.push(roleTable(ideal.role_fingerprints.slice(0, 6)));
    analysis.push('');
  }

  analysis.push('Ideal-fit punishes:');
  analysis.push('');
  analysis.push(responseTable(system.ideal_punishes));
  analysis.push('');
  analysis.push('Ideal-fit counters:');
  analysis.push('');
  analysis.push(responseTable(system.ideal_counters));
  analysis.push('');
}

fs.writeFileSync(process.env.JSON_FILE, JSON.stringify(payload, null, 2) + '\n');
fs.writeFileSync(process.env.RANKINGS_FILE, rankings.join('\n') + '\n');
fs.writeFileSync(process.env.ANALYSIS_FILE, analysis.join('\n') + '\n');
NODE

echo "Wrote fit-profile benchmark artifacts:"
echo "  json: $json_file"
echo "  rankings: $rankings_file"
echo "  analysis: $analysis_file"
