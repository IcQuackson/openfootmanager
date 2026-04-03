use engine::{
    MatchConfig, MatchReport, PlayStyle, PlayerData, Position, TeamData, simulate_with_rng,
};
use rand::SeedableRng;
use rand::rngs::StdRng;
use serde::Serialize;
use std::collections::HashMap;
use std::env;

const DEFAULT_MATCHES_PER_LEG: u32 = 200;
const BASE_SKILL: u8 = 65;

#[derive(Clone, Copy)]
struct Shape {
    label: &'static str,
    aliases: &'static [&'static str],
    defenders: usize,
    midfielders: usize,
    forwards: usize,
}

#[derive(Clone, Copy)]
struct TacticDef {
    id: &'static str,
    shape: Shape,
    style: PlayStyle,
    style_label: &'static str,
}

#[derive(Default, Clone, Copy)]
struct Aggregate {
    matches: u32,
    wins: u32,
    draws: u32,
    losses: u32,
    goals_for: u32,
    goals_against: u32,
    shots_for: u32,
    shots_against: u32,
    possession_for: f64,
}

#[derive(Clone, Serialize)]
struct AggregateView {
    matches: u32,
    points_per_match: f64,
    win_rate: f64,
    draw_rate: f64,
    loss_rate: f64,
    goals_for_per_match: f64,
    goals_against_per_match: f64,
    goal_diff_per_match: f64,
    shots_for_per_match: f64,
    shots_against_per_match: f64,
    avg_possession: f64,
}

#[derive(Clone, Serialize)]
struct MatchupView {
    opponent_id: String,
    opponent_shape: String,
    opponent_style: String,
    summary: AggregateView,
}

#[derive(Serialize)]
struct TacticResult {
    id: String,
    shape: String,
    shape_aliases: Vec<String>,
    style: String,
    overall: AggregateView,
    matchups: Vec<MatchupView>,
    strongest_against: Vec<MatchupView>,
    weakest_against: Vec<MatchupView>,
}

#[derive(Serialize)]
struct SummaryView {
    id: String,
    average_points_per_match: f64,
    average_goal_diff_per_match: f64,
    average_goals_for_per_match: f64,
    average_goals_against_per_match: f64,
    average_possession: f64,
}

#[derive(Serialize)]
struct Output {
    matches_per_leg: u32,
    notes: Vec<String>,
    style_summary: Vec<SummaryView>,
    shape_summary: Vec<SummaryView>,
    tactics: Vec<TacticResult>,
}

fn main() {
    let matches_per_leg = parse_matches_per_leg(env::args().skip(1).collect());
    let output = run_matrix(matches_per_leg);
    println!(
        "{}",
        serde_json::to_string_pretty(&output).expect("serialize tactic matrix")
    );
}

fn parse_matches_per_leg(args: Vec<String>) -> u32 {
    let mut matches = DEFAULT_MATCHES_PER_LEG;
    let mut index = 0usize;

    while index < args.len() {
        match args[index].as_str() {
            "--matches-per-leg" => {
                let Some(raw) = args.get(index + 1) else {
                    eprintln!("Missing value for --matches-per-leg");
                    std::process::exit(1);
                };
                matches = raw.parse::<u32>().unwrap_or_else(|_| {
                    eprintln!("Invalid value for --matches-per-leg: {}", raw);
                    std::process::exit(1);
                });
                index += 2;
            }
            "--help" | "-h" => {
                println!(
                    "Usage: cargo run -p engine --example tactic_matrix --release -- [--matches-per-leg N]"
                );
                std::process::exit(0);
            }
            other => {
                eprintln!("Unknown argument: {}", other);
                std::process::exit(1);
            }
        }
    }

    matches
}

fn run_matrix(matches_per_leg: u32) -> Output {
    let config = MatchConfig::default();
    let tactics = tactic_defs();
    let mut overall: HashMap<&'static str, Aggregate> = HashMap::new();
    let mut versus: HashMap<(&'static str, &'static str), Aggregate> = HashMap::new();

    for (left_index, left_tactic) in tactics.iter().enumerate() {
        for (right_index, right_tactic) in tactics.iter().enumerate() {
            if right_index < left_index {
                continue;
            }

            let left_home = make_team(
                "left_home",
                "Left Home",
                left_tactic.shape,
                left_tactic.style,
            );
            let right_away = make_team(
                "right_away",
                "Right Away",
                right_tactic.shape,
                right_tactic.style,
            );
            for seed in 0..matches_per_leg {
                let mut rng = StdRng::seed_from_u64(pair_seed(left_index, right_index, 0, seed));
                let report = simulate_with_rng(&left_home, &right_away, &config, &mut rng);
                record_home_away(&report, &mut overall, left_tactic.id, right_tactic.id);
                record_home_away(
                    &report,
                    &mut versus,
                    (left_tactic.id, right_tactic.id),
                    (right_tactic.id, left_tactic.id),
                );
            }

            if left_index != right_index {
                let right_home = make_team(
                    "right_home",
                    "Right Home",
                    right_tactic.shape,
                    right_tactic.style,
                );
                let left_away = make_team(
                    "left_away",
                    "Left Away",
                    left_tactic.shape,
                    left_tactic.style,
                );
                for seed in 0..matches_per_leg {
                    let mut rng =
                        StdRng::seed_from_u64(pair_seed(left_index, right_index, 1, seed));
                    let report = simulate_with_rng(&right_home, &left_away, &config, &mut rng);
                    record_home_away(&report, &mut overall, right_tactic.id, left_tactic.id);
                    record_home_away(
                        &report,
                        &mut versus,
                        (right_tactic.id, left_tactic.id),
                        (left_tactic.id, right_tactic.id),
                    );
                }
            }
        }
    }

    let mut tactic_results = tactics
        .iter()
        .map(|tactic| {
            let mut matchups = tactics
                .iter()
                .filter(|opponent| opponent.id != tactic.id)
                .map(|opponent| MatchupView {
                    opponent_id: opponent.id.to_string(),
                    opponent_shape: opponent.shape.label.to_string(),
                    opponent_style: opponent.style_label.to_string(),
                    summary: to_view(
                        *versus
                            .get(&(tactic.id, opponent.id))
                            .unwrap_or(&Aggregate::default()),
                    ),
                })
                .collect::<Vec<_>>();

            sort_matchups(&mut matchups);
            let strongest_against = matchups.iter().take(5).cloned().collect::<Vec<_>>();
            let weakest_against = matchups.iter().rev().take(5).cloned().collect::<Vec<_>>();

            TacticResult {
                id: tactic.id.to_string(),
                shape: tactic.shape.label.to_string(),
                shape_aliases: tactic
                    .shape
                    .aliases
                    .iter()
                    .map(|alias| (*alias).to_string())
                    .collect(),
                style: tactic.style_label.to_string(),
                overall: to_view(*overall.get(tactic.id).unwrap_or(&Aggregate::default())),
                matchups,
                strongest_against,
                weakest_against,
            }
        })
        .collect::<Vec<_>>();

    tactic_results.sort_by(|left, right| compare_aggregate_views(&right.overall, &left.overall));

    Output {
        matches_per_leg,
        notes: vec![
            "Each unordered matchup is simulated home and away to reduce home-advantage bias."
                .to_string(),
            "The current engine only distinguishes defender-midfielder-forward counts, so 4-5-1, 4-2-3-1, and 4-1-4-1 are equivalent here."
                .to_string(),
            "All teams use equal-quality role-specific archetypes to isolate tactical interaction instead of squad-quality differences."
                .to_string(),
        ],
        style_summary: build_group_summary(&tactic_results, |t| &t.style),
        shape_summary: build_group_summary(&tactic_results, |t| &t.shape),
        tactics: tactic_results,
    }
}

fn sort_matchups(matchups: &mut [MatchupView]) {
    matchups.sort_by(|left, right| compare_aggregate_views(&right.summary, &left.summary));
}

fn compare_aggregate_views(left: &AggregateView, right: &AggregateView) -> std::cmp::Ordering {
    left.points_per_match
        .partial_cmp(&right.points_per_match)
        .unwrap_or(std::cmp::Ordering::Equal)
        .then_with(|| {
            left.goal_diff_per_match
                .partial_cmp(&right.goal_diff_per_match)
                .unwrap_or(std::cmp::Ordering::Equal)
        })
}

fn build_group_summary<'a, F>(tactics: &'a [TacticResult], key_fn: F) -> Vec<SummaryView>
where
    F: Fn(&'a TacticResult) -> &'a str,
{
    let mut groups: HashMap<&str, Vec<&TacticResult>> = HashMap::new();
    for tactic in tactics {
        groups.entry(key_fn(tactic)).or_default().push(tactic);
    }

    let mut summary = groups
        .into_iter()
        .map(|(id, items)| {
            let count = items.len().max(1) as f64;
            SummaryView {
                id: id.to_string(),
                average_points_per_match: items
                    .iter()
                    .map(|tactic| tactic.overall.points_per_match)
                    .sum::<f64>()
                    / count,
                average_goal_diff_per_match: items
                    .iter()
                    .map(|tactic| tactic.overall.goal_diff_per_match)
                    .sum::<f64>()
                    / count,
                average_goals_for_per_match: items
                    .iter()
                    .map(|tactic| tactic.overall.goals_for_per_match)
                    .sum::<f64>()
                    / count,
                average_goals_against_per_match: items
                    .iter()
                    .map(|tactic| tactic.overall.goals_against_per_match)
                    .sum::<f64>()
                    / count,
                average_possession: items
                    .iter()
                    .map(|tactic| tactic.overall.avg_possession)
                    .sum::<f64>()
                    / count,
            }
        })
        .collect::<Vec<_>>();

    summary.sort_by(|left, right| {
        right
            .average_points_per_match
            .partial_cmp(&left.average_points_per_match)
            .unwrap_or(std::cmp::Ordering::Equal)
            .then_with(|| {
                right
                    .average_goal_diff_per_match
                    .partial_cmp(&left.average_goal_diff_per_match)
                    .unwrap_or(std::cmp::Ordering::Equal)
            })
    });
    summary
}

fn record_home_away<K: std::cmp::Eq + std::hash::Hash + Copy>(
    report: &MatchReport,
    map: &mut HashMap<K, Aggregate>,
    home_key: K,
    away_key: K,
) {
    {
        let aggregate = map.entry(home_key).or_default();
        update_aggregate(
            aggregate,
            report.home_goals as u32,
            report.away_goals as u32,
            report.home_stats.shots as u32,
            report.away_stats.shots as u32,
            report.home_possession,
        );
    }
    {
        let aggregate = map.entry(away_key).or_default();
        update_aggregate(
            aggregate,
            report.away_goals as u32,
            report.home_goals as u32,
            report.away_stats.shots as u32,
            report.home_stats.shots as u32,
            100.0 - report.home_possession,
        );
    }
}

fn pair_seed(left_index: usize, right_index: usize, leg: u64, seed: u32) -> u64 {
    ((left_index as u64) << 40) ^ ((right_index as u64) << 24) ^ (leg << 20) ^ u64::from(seed)
}

fn update_aggregate(
    aggregate: &mut Aggregate,
    goals_for: u32,
    goals_against: u32,
    shots_for: u32,
    shots_against: u32,
    possession_for: f64,
) {
    aggregate.matches += 1;
    aggregate.goals_for += goals_for;
    aggregate.goals_against += goals_against;
    aggregate.shots_for += shots_for;
    aggregate.shots_against += shots_against;
    aggregate.possession_for += possession_for;
    if goals_for > goals_against {
        aggregate.wins += 1;
    } else if goals_for == goals_against {
        aggregate.draws += 1;
    } else {
        aggregate.losses += 1;
    }
}

fn to_view(aggregate: Aggregate) -> AggregateView {
    let matches = aggregate.matches.max(1) as f64;
    AggregateView {
        matches: aggregate.matches,
        points_per_match: ((aggregate.wins * 3 + aggregate.draws) as f64) / matches,
        win_rate: aggregate.wins as f64 / matches,
        draw_rate: aggregate.draws as f64 / matches,
        loss_rate: aggregate.losses as f64 / matches,
        goals_for_per_match: aggregate.goals_for as f64 / matches,
        goals_against_per_match: aggregate.goals_against as f64 / matches,
        goal_diff_per_match: (aggregate.goals_for as f64 - aggregate.goals_against as f64)
            / matches,
        shots_for_per_match: aggregate.shots_for as f64 / matches,
        shots_against_per_match: aggregate.shots_against as f64 / matches,
        avg_possession: aggregate.possession_for / matches,
    }
}

fn tactic_defs() -> Vec<TacticDef> {
    let shapes = [
        Shape {
            label: "4-4-2",
            aliases: &["4-4-2"],
            defenders: 4,
            midfielders: 4,
            forwards: 2,
        },
        Shape {
            label: "4-3-3",
            aliases: &["4-3-3"],
            defenders: 4,
            midfielders: 3,
            forwards: 3,
        },
        Shape {
            label: "3-5-2",
            aliases: &["3-5-2"],
            defenders: 3,
            midfielders: 5,
            forwards: 2,
        },
        Shape {
            label: "4-5-1",
            aliases: &["4-5-1", "4-2-3-1", "4-1-4-1"],
            defenders: 4,
            midfielders: 5,
            forwards: 1,
        },
        Shape {
            label: "3-4-3",
            aliases: &["3-4-3"],
            defenders: 3,
            midfielders: 4,
            forwards: 3,
        },
        Shape {
            label: "5-3-2",
            aliases: &["5-3-2"],
            defenders: 5,
            midfielders: 3,
            forwards: 2,
        },
    ];
    let styles = [
        (PlayStyle::Balanced, "Balanced"),
        (PlayStyle::Attacking, "Attacking"),
        (PlayStyle::Defensive, "Defensive"),
        (PlayStyle::Possession, "Possession"),
        (PlayStyle::Counter, "Counter"),
        (PlayStyle::HighPress, "HighPress"),
    ];

    let mut tactics = Vec::new();
    for shape in shapes {
        for (style, style_label) in styles {
            let id = Box::leak(format!("{}__{}", shape.label, style_label).into_boxed_str());
            tactics.push(TacticDef {
                id,
                shape,
                style,
                style_label,
            });
        }
    }
    tactics
}

fn make_team(id: &str, name: &str, shape: Shape, play_style: PlayStyle) -> TeamData {
    let mut players = Vec::with_capacity(11);
    players.push(make_goalkeeper(&format!("{id}_gk"), "GK"));
    for index in 0..shape.defenders {
        players.push(make_defender(
            &format!("{id}_def{index}"),
            &format!("DEF{}", index + 1),
        ));
    }
    for index in 0..shape.midfielders {
        players.push(make_midfielder(
            &format!("{id}_mid{index}"),
            &format!("MID{}", index + 1),
        ));
    }
    for index in 0..shape.forwards {
        players.push(make_forward(
            &format!("{id}_fwd{index}"),
            &format!("FWD{}", index + 1),
        ));
    }
    TeamData {
        id: id.to_string(),
        name: name.to_string(),
        formation: shape.label.to_string(),
        play_style,
        players,
    }
}

fn make_goalkeeper(id: &str, name: &str) -> PlayerData {
    PlayerData {
        id: id.to_string(),
        name: name.to_string(),
        position: Position::Goalkeeper,
        condition: 90,
        fitness: 75,
        pace: 46,
        stamina: 58,
        strength: 68,
        agility: 62,
        passing: 54,
        shooting: 30,
        tackling: 32,
        dribbling: 38,
        defending: 42,
        positioning: 72,
        vision: 58,
        decisions: 70,
        composure: 68,
        aggression: 48,
        teamwork: 62,
        leadership: 64,
        handling: 74,
        reflexes: 74,
        aerial: 72,
        traits: vec![],
    }
}

fn make_defender(id: &str, name: &str) -> PlayerData {
    let skill = BASE_SKILL;
    PlayerData {
        id: id.to_string(),
        name: name.to_string(),
        position: Position::Defender,
        condition: 90,
        fitness: 75,
        pace: skill - 3,
        stamina: skill + 1,
        strength: skill + 5,
        agility: skill - 4,
        passing: skill - 5,
        shooting: skill - 18,
        tackling: skill + 8,
        dribbling: skill - 8,
        defending: skill + 10,
        positioning: skill + 7,
        vision: skill - 4,
        decisions: skill + 3,
        composure: skill + 2,
        aggression: skill + 4,
        teamwork: skill + 1,
        leadership: skill,
        handling: 35,
        reflexes: 35,
        aerial: skill + 7,
        traits: vec![],
    }
}

fn make_midfielder(id: &str, name: &str) -> PlayerData {
    let skill = BASE_SKILL;
    PlayerData {
        id: id.to_string(),
        name: name.to_string(),
        position: Position::Midfielder,
        condition: 90,
        fitness: 75,
        pace: skill,
        stamina: skill + 5,
        strength: skill - 3,
        agility: skill + 1,
        passing: skill + 8,
        shooting: skill - 2,
        tackling: skill + 2,
        dribbling: skill + 4,
        defending: skill,
        positioning: skill + 2,
        vision: skill + 8,
        decisions: skill + 6,
        composure: skill + 3,
        aggression: skill - 1,
        teamwork: skill + 6,
        leadership: skill,
        handling: 35,
        reflexes: 35,
        aerial: skill - 6,
        traits: vec![],
    }
}

fn make_forward(id: &str, name: &str) -> PlayerData {
    let skill = BASE_SKILL;
    PlayerData {
        id: id.to_string(),
        name: name.to_string(),
        position: Position::Forward,
        condition: 90,
        fitness: 75,
        pace: skill + 5,
        stamina: skill,
        strength: skill + 1,
        agility: skill + 4,
        passing: skill - 4,
        shooting: skill + 10,
        tackling: skill - 14,
        dribbling: skill + 8,
        defending: skill - 15,
        positioning: skill + 8,
        vision: skill - 1,
        decisions: skill + 4,
        composure: skill + 5,
        aggression: skill + 1,
        teamwork: skill - 2,
        leadership: skill - 1,
        handling: 35,
        reflexes: 35,
        aerial: skill + 1,
        traits: vec![],
    }
}
