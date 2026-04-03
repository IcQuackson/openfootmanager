use engine::{
    MatchConfig, MatchReport, PlayStyle, PlayerData, Position, TeamData, simulate_with_rng,
};
use rand::SeedableRng;
use rand::rngs::StdRng;
use serde::Serialize;
use std::collections::HashMap;
use std::env;

const DEFAULT_MATCHES_PER_LEG: u32 = 50;

#[derive(Clone, Copy)]
struct Shape {
    label: &'static str,
    defenders: usize,
    midfielders: usize,
    forwards: usize,
}

#[derive(Clone, Copy)]
enum GoalkeeperArchetype {
    ShotStopper,
    SweeperKeeper,
}

#[derive(Clone, Copy)]
enum DefenderArchetype {
    Stopper,
    BallPlayingCb,
    FullBack,
    WingBack,
}

#[derive(Clone, Copy)]
enum MidfielderArchetype {
    Destroyer,
    Regista,
    BoxToBox,
    Creator,
    WideWorker,
}

#[derive(Clone, Copy)]
enum ForwardArchetype {
    Poacher,
    TargetMan,
    Runner,
    CreatorForward,
}

#[derive(Clone, Copy)]
struct TemplateDef {
    id: &'static str,
    shape: Shape,
    style: PlayStyle,
    style_label: &'static str,
    squad_label: &'static str,
    summary: &'static str,
    goalkeeper: GoalkeeperArchetype,
    defenders: &'static [DefenderArchetype],
    midfielders: &'static [MidfielderArchetype],
    forwards: &'static [ForwardArchetype],
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
    opponent_squad: String,
    summary: AggregateView,
}

#[derive(Clone, Serialize)]
struct SquadMatchupView {
    squad_type: String,
    summary: AggregateView,
}

#[derive(Serialize)]
struct TemplateResult {
    id: String,
    shape: String,
    style: String,
    squad_type: String,
    summary: String,
    overall: AggregateView,
    matchups: Vec<MatchupView>,
    strongest_against: Vec<MatchupView>,
    weakest_against: Vec<MatchupView>,
    thrives_against_squads: Vec<SquadMatchupView>,
    struggles_against_squads: Vec<SquadMatchupView>,
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
    squad_summary: Vec<SummaryView>,
    templates: Vec<TemplateResult>,
}

fn main() {
    let matches_per_leg = parse_matches_per_leg(env::args().skip(1).collect());
    let output = run_matrix(matches_per_leg);
    println!(
        "{}",
        serde_json::to_string_pretty(&output).expect("serialize archetype tactic matrix")
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
                    "Usage: cargo run -p engine --example archetype_tactic_matrix --release -- [--matches-per-leg N]"
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
    let templates = template_defs();
    let mut overall: HashMap<&'static str, Aggregate> = HashMap::new();
    let mut versus: HashMap<(&'static str, &'static str), Aggregate> = HashMap::new();

    for (left_index, left_template) in templates.iter().enumerate() {
        for (right_index, right_template) in templates.iter().enumerate() {
            if right_index < left_index {
                continue;
            }

            let left_home = make_team("left_home", "Left Home", *left_template);
            let right_away = make_team("right_away", "Right Away", *right_template);
            for seed in 0..matches_per_leg {
                let mut rng = StdRng::seed_from_u64(pair_seed(left_index, right_index, 0, seed));
                let report = simulate_with_rng(&left_home, &right_away, &config, &mut rng);
                record_home_away(&report, &mut overall, left_template.id, right_template.id);
                record_home_away(
                    &report,
                    &mut versus,
                    (left_template.id, right_template.id),
                    (right_template.id, left_template.id),
                );
            }

            if left_index != right_index {
                let right_home = make_team("right_home", "Right Home", *right_template);
                let left_away = make_team("left_away", "Left Away", *left_template);
                for seed in 0..matches_per_leg {
                    let mut rng =
                        StdRng::seed_from_u64(pair_seed(left_index, right_index, 1, seed));
                    let report = simulate_with_rng(&right_home, &left_away, &config, &mut rng);
                    record_home_away(&report, &mut overall, right_template.id, left_template.id);
                    record_home_away(
                        &report,
                        &mut versus,
                        (right_template.id, left_template.id),
                        (left_template.id, right_template.id),
                    );
                }
            }
        }
    }

    let mut template_results = templates
        .iter()
        .map(|template| build_template_result(template, &templates, &overall, &versus))
        .collect::<Vec<_>>();

    template_results.sort_by(|left, right| compare_aggregate_views(&right.overall, &left.overall));

    Output {
        matches_per_leg,
        notes: vec![
            "Each unordered matchup is simulated home and away to reduce home-advantage bias."
                .to_string(),
            "Squads use benchmark-only archetype templates so tactic analysis includes player profile fit, not just raw formation and play style."
                .to_string(),
            "Template matchup sections include both exact opponent templates and opponent squad-type aggregates."
                .to_string(),
        ],
        style_summary: build_group_summary(&template_results, |template| &template.style),
        shape_summary: build_group_summary(&template_results, |template| &template.shape),
        squad_summary: build_group_summary(&template_results, |template| &template.squad_type),
        templates: template_results,
    }
}

fn build_template_result(
    template: &TemplateDef,
    templates: &[TemplateDef],
    overall: &HashMap<&'static str, Aggregate>,
    versus: &HashMap<(&'static str, &'static str), Aggregate>,
) -> TemplateResult {
    let mut matchups = templates
        .iter()
        .filter(|opponent| opponent.id != template.id)
        .map(|opponent| MatchupView {
            opponent_id: opponent.id.to_string(),
            opponent_shape: opponent.shape.label.to_string(),
            opponent_style: opponent.style_label.to_string(),
            opponent_squad: opponent.squad_label.to_string(),
            summary: to_view(
                *versus
                    .get(&(template.id, opponent.id))
                    .unwrap_or(&Aggregate::default()),
            ),
        })
        .collect::<Vec<_>>();
    sort_matchups(&mut matchups);

    let mut squad_aggregates: HashMap<&str, Aggregate> = HashMap::new();
    for opponent in templates.iter().filter(|opponent| opponent.id != template.id) {
        let aggregate = *versus
            .get(&(template.id, opponent.id))
            .unwrap_or(&Aggregate::default());
        squad_aggregates
            .entry(opponent.squad_label)
            .and_modify(|current| merge_aggregate(current, aggregate))
            .or_insert(aggregate);
    }

    let mut squad_matchups = squad_aggregates
        .into_iter()
        .map(|(squad_type, aggregate)| SquadMatchupView {
            squad_type: squad_type.to_string(),
            summary: to_view(aggregate),
        })
        .collect::<Vec<_>>();
    squad_matchups.sort_by(|left, right| compare_aggregate_views(&right.summary, &left.summary));

    TemplateResult {
        id: template.id.to_string(),
        shape: template.shape.label.to_string(),
        style: template.style_label.to_string(),
        squad_type: template.squad_label.to_string(),
        summary: template.summary.to_string(),
        overall: to_view(*overall.get(template.id).unwrap_or(&Aggregate::default())),
        strongest_against: matchups.iter().take(5).cloned().collect(),
        weakest_against: matchups.iter().rev().take(5).cloned().collect(),
        thrives_against_squads: squad_matchups.iter().take(3).cloned().collect(),
        struggles_against_squads: squad_matchups.iter().rev().take(3).cloned().collect(),
        matchups,
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

fn build_group_summary<'a, F>(templates: &'a [TemplateResult], key_fn: F) -> Vec<SummaryView>
where
    F: Fn(&'a TemplateResult) -> &'a str,
{
    let mut groups: HashMap<&str, Vec<&TemplateResult>> = HashMap::new();
    for template in templates {
        groups.entry(key_fn(template)).or_default().push(template);
    }

    let mut summary = groups
        .into_iter()
        .map(|(id, items)| {
            let count = items.len().max(1) as f64;
            SummaryView {
                id: id.to_string(),
                average_points_per_match: items
                    .iter()
                    .map(|template| template.overall.points_per_match)
                    .sum::<f64>()
                    / count,
                average_goal_diff_per_match: items
                    .iter()
                    .map(|template| template.overall.goal_diff_per_match)
                    .sum::<f64>()
                    / count,
                average_goals_for_per_match: items
                    .iter()
                    .map(|template| template.overall.goals_for_per_match)
                    .sum::<f64>()
                    / count,
                average_goals_against_per_match: items
                    .iter()
                    .map(|template| template.overall.goals_against_per_match)
                    .sum::<f64>()
                    / count,
                average_possession: items
                    .iter()
                    .map(|template| template.overall.avg_possession)
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

fn merge_aggregate(target: &mut Aggregate, source: Aggregate) {
    target.matches += source.matches;
    target.wins += source.wins;
    target.draws += source.draws;
    target.losses += source.losses;
    target.goals_for += source.goals_for;
    target.goals_against += source.goals_against;
    target.shots_for += source.shots_for;
    target.shots_against += source.shots_against;
    target.possession_for += source.possession_for;
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

fn template_defs() -> Vec<TemplateDef> {
    let shape_442 = Shape {
        label: "4-4-2",
        defenders: 4,
        midfielders: 4,
        forwards: 2,
    };
    let shape_433 = Shape {
        label: "4-3-3",
        defenders: 4,
        midfielders: 3,
        forwards: 3,
    };
    let shape_352 = Shape {
        label: "3-5-2",
        defenders: 3,
        midfielders: 5,
        forwards: 2,
    };
    let shape_451 = Shape {
        label: "4-5-1",
        defenders: 4,
        midfielders: 5,
        forwards: 1,
    };
    let shape_4231 = Shape {
        label: "4-2-3-1",
        defenders: 4,
        midfielders: 5,
        forwards: 1,
    };
    let shape_4141 = Shape {
        label: "4-1-4-1",
        defenders: 4,
        midfielders: 5,
        forwards: 1,
    };
    let shape_343 = Shape {
        label: "3-4-3",
        defenders: 3,
        midfielders: 4,
        forwards: 3,
    };
    let shape_532 = Shape {
        label: "5-3-2",
        defenders: 5,
        midfielders: 3,
        forwards: 2,
    };

    vec![
        TemplateDef {
            id: "4-2-3-1__Possession__ControlSpine",
            shape: shape_4231,
            style: PlayStyle::Possession,
            style_label: "Possession",
            squad_label: "Control Spine",
            summary: "Regista-led control team with technical support around a creator-forward.",
            goalkeeper: GoalkeeperArchetype::SweeperKeeper,
            defenders: &[
                DefenderArchetype::FullBack,
                DefenderArchetype::BallPlayingCb,
                DefenderArchetype::Stopper,
                DefenderArchetype::FullBack,
            ],
            midfielders: &[
                MidfielderArchetype::Regista,
                MidfielderArchetype::BoxToBox,
                MidfielderArchetype::Creator,
                MidfielderArchetype::WideWorker,
                MidfielderArchetype::WideWorker,
            ],
            forwards: &[ForwardArchetype::CreatorForward],
        },
        TemplateDef {
            id: "4-5-1__Possession__ControlSpine",
            shape: shape_451,
            style: PlayStyle::Possession,
            style_label: "Possession",
            squad_label: "Control Spine",
            summary: "Patient possession shell with one outlet striker and technical midfield support.",
            goalkeeper: GoalkeeperArchetype::SweeperKeeper,
            defenders: &[
                DefenderArchetype::FullBack,
                DefenderArchetype::BallPlayingCb,
                DefenderArchetype::BallPlayingCb,
                DefenderArchetype::FullBack,
            ],
            midfielders: &[
                MidfielderArchetype::Regista,
                MidfielderArchetype::BoxToBox,
                MidfielderArchetype::Creator,
                MidfielderArchetype::WideWorker,
                MidfielderArchetype::WideWorker,
            ],
            forwards: &[ForwardArchetype::CreatorForward],
        },
        TemplateDef {
            id: "4-3-3__HighPress__PressingWave",
            shape: shape_433,
            style: PlayStyle::HighPress,
            style_label: "HighPress",
            squad_label: "Pressing Wave",
            summary: "Runner-heavy front line backed by an engine midfield and a sweeper keeper.",
            goalkeeper: GoalkeeperArchetype::SweeperKeeper,
            defenders: &[
                DefenderArchetype::FullBack,
                DefenderArchetype::BallPlayingCb,
                DefenderArchetype::Stopper,
                DefenderArchetype::FullBack,
            ],
            midfielders: &[
                MidfielderArchetype::Destroyer,
                MidfielderArchetype::BoxToBox,
                MidfielderArchetype::WideWorker,
            ],
            forwards: &[
                ForwardArchetype::Runner,
                ForwardArchetype::CreatorForward,
                ForwardArchetype::Runner,
            ],
        },
        TemplateDef {
            id: "3-4-3__HighPress__PressingWave",
            shape: shape_343,
            style: PlayStyle::HighPress,
            style_label: "HighPress",
            squad_label: "Pressing Wave",
            summary: "Three-at-the-back pressing setup that relies on wing-backs and recovery pace.",
            goalkeeper: GoalkeeperArchetype::SweeperKeeper,
            defenders: &[
                DefenderArchetype::Stopper,
                DefenderArchetype::BallPlayingCb,
                DefenderArchetype::Stopper,
            ],
            midfielders: &[
                MidfielderArchetype::WideWorker,
                MidfielderArchetype::BoxToBox,
                MidfielderArchetype::Destroyer,
                MidfielderArchetype::WideWorker,
            ],
            forwards: &[
                ForwardArchetype::Runner,
                ForwardArchetype::Poacher,
                ForwardArchetype::Runner,
            ],
        },
        TemplateDef {
            id: "3-5-2__Counter__TransitionCore",
            shape: shape_352,
            style: PlayStyle::Counter,
            style_label: "Counter",
            squad_label: "Transition Core",
            summary: "Compact central block with a target man and runner pairing for fast breaks.",
            goalkeeper: GoalkeeperArchetype::ShotStopper,
            defenders: &[
                DefenderArchetype::Stopper,
                DefenderArchetype::BallPlayingCb,
                DefenderArchetype::Stopper,
            ],
            midfielders: &[
                MidfielderArchetype::WideWorker,
                MidfielderArchetype::BoxToBox,
                MidfielderArchetype::Regista,
                MidfielderArchetype::BoxToBox,
                MidfielderArchetype::WideWorker,
            ],
            forwards: &[ForwardArchetype::TargetMan, ForwardArchetype::Runner],
        },
        TemplateDef {
            id: "5-3-2__Counter__TransitionCore",
            shape: shape_532,
            style: PlayStyle::Counter,
            style_label: "Counter",
            squad_label: "Transition Core",
            summary: "Back-five counter shape with secure defenders and direct outlet forwards.",
            goalkeeper: GoalkeeperArchetype::ShotStopper,
            defenders: &[
                DefenderArchetype::FullBack,
                DefenderArchetype::Stopper,
                DefenderArchetype::BallPlayingCb,
                DefenderArchetype::Stopper,
                DefenderArchetype::FullBack,
            ],
            midfielders: &[
                MidfielderArchetype::Destroyer,
                MidfielderArchetype::BoxToBox,
                MidfielderArchetype::Creator,
            ],
            forwards: &[ForwardArchetype::TargetMan, ForwardArchetype::Runner],
        },
        TemplateDef {
            id: "4-5-1__Defensive__LowBlockOutlet",
            shape: shape_451,
            style: PlayStyle::Defensive,
            style_label: "Defensive",
            squad_label: "Low Block Outlet",
            summary: "Deep block with narrow protection and a single target-man release valve.",
            goalkeeper: GoalkeeperArchetype::ShotStopper,
            defenders: &[
                DefenderArchetype::FullBack,
                DefenderArchetype::Stopper,
                DefenderArchetype::Stopper,
                DefenderArchetype::FullBack,
            ],
            midfielders: &[
                MidfielderArchetype::Destroyer,
                MidfielderArchetype::BoxToBox,
                MidfielderArchetype::Destroyer,
                MidfielderArchetype::WideWorker,
                MidfielderArchetype::WideWorker,
            ],
            forwards: &[ForwardArchetype::TargetMan],
        },
        TemplateDef {
            id: "4-1-4-1__Defensive__LowBlockOutlet",
            shape: shape_4141,
            style: PlayStyle::Defensive,
            style_label: "Defensive",
            squad_label: "Low Block Outlet",
            summary: "Screened back four with destroyer shield and narrow support around one outlet striker.",
            goalkeeper: GoalkeeperArchetype::ShotStopper,
            defenders: &[
                DefenderArchetype::FullBack,
                DefenderArchetype::Stopper,
                DefenderArchetype::Stopper,
                DefenderArchetype::FullBack,
            ],
            midfielders: &[
                MidfielderArchetype::Destroyer,
                MidfielderArchetype::WideWorker,
                MidfielderArchetype::Regista,
                MidfielderArchetype::BoxToBox,
                MidfielderArchetype::WideWorker,
            ],
            forwards: &[ForwardArchetype::TargetMan],
        },
        TemplateDef {
            id: "3-4-3__Attacking__WingSurge",
            shape: shape_343,
            style: PlayStyle::Attacking,
            style_label: "Attacking",
            squad_label: "Wing Surge",
            summary: "Wide overload team with wing-backs supplying runners and a poacher in the middle.",
            goalkeeper: GoalkeeperArchetype::SweeperKeeper,
            defenders: &[
                DefenderArchetype::BallPlayingCb,
                DefenderArchetype::Stopper,
                DefenderArchetype::BallPlayingCb,
            ],
            midfielders: &[
                MidfielderArchetype::WideWorker,
                MidfielderArchetype::Creator,
                MidfielderArchetype::BoxToBox,
                MidfielderArchetype::WideWorker,
            ],
            forwards: &[
                ForwardArchetype::Runner,
                ForwardArchetype::Poacher,
                ForwardArchetype::Runner,
            ],
        },
        TemplateDef {
            id: "4-2-3-1__Attacking__CreatorTen",
            shape: shape_4231,
            style: PlayStyle::Attacking,
            style_label: "Attacking",
            squad_label: "Creator Ten",
            summary: "Central creator-focused attack with a mobile nine and technical support around the box.",
            goalkeeper: GoalkeeperArchetype::SweeperKeeper,
            defenders: &[
                DefenderArchetype::FullBack,
                DefenderArchetype::BallPlayingCb,
                DefenderArchetype::Stopper,
                DefenderArchetype::WingBack,
            ],
            midfielders: &[
                MidfielderArchetype::Regista,
                MidfielderArchetype::BoxToBox,
                MidfielderArchetype::Creator,
                MidfielderArchetype::Creator,
                MidfielderArchetype::WideWorker,
            ],
            forwards: &[ForwardArchetype::Poacher],
        },
        TemplateDef {
            id: "4-4-2__Balanced__ClassicPair",
            shape: shape_442,
            style: PlayStyle::Balanced,
            style_label: "Balanced",
            squad_label: "Classic Pair",
            summary: "Traditional partnership up front with even wide support and one all-round midfield pair.",
            goalkeeper: GoalkeeperArchetype::ShotStopper,
            defenders: &[
                DefenderArchetype::FullBack,
                DefenderArchetype::Stopper,
                DefenderArchetype::BallPlayingCb,
                DefenderArchetype::FullBack,
            ],
            midfielders: &[
                MidfielderArchetype::WideWorker,
                MidfielderArchetype::BoxToBox,
                MidfielderArchetype::Regista,
                MidfielderArchetype::WideWorker,
            ],
            forwards: &[ForwardArchetype::TargetMan, ForwardArchetype::Poacher],
        },
    ]
}

fn make_team(id: &str, name: &str, template: TemplateDef) -> TeamData {
    debug_assert_eq!(template.shape.defenders, template.defenders.len());
    debug_assert_eq!(template.shape.midfielders, template.midfielders.len());
    debug_assert_eq!(template.shape.forwards, template.forwards.len());

    let mut players = Vec::with_capacity(11);
    players.push(make_goalkeeper(
        &format!("{id}_gk"),
        "GK",
        template.goalkeeper,
    ));
    for (index, archetype) in template.defenders.iter().copied().enumerate() {
        players.push(make_defender(
            &format!("{id}_def{index}"),
            &format!("DEF{}", index + 1),
            archetype,
        ));
    }
    for (index, archetype) in template.midfielders.iter().copied().enumerate() {
        players.push(make_midfielder(
            &format!("{id}_mid{index}"),
            &format!("MID{}", index + 1),
            archetype,
        ));
    }
    for (index, archetype) in template.forwards.iter().copied().enumerate() {
        players.push(make_forward(
            &format!("{id}_fwd{index}"),
            &format!("FWD{}", index + 1),
            archetype,
        ));
    }
    TeamData {
        id: id.to_string(),
        name: name.to_string(),
        formation: template.shape.label.to_string(),
        play_style: template.style,
        players,
    }
}

fn make_goalkeeper(id: &str, name: &str, archetype: GoalkeeperArchetype) -> PlayerData {
    match archetype {
        GoalkeeperArchetype::ShotStopper => make_player(
            id,
            name,
            Position::Goalkeeper,
            [46, 58, 68, 62, 50, 30, 32, 38, 42, 74, 56, 72, 68, 48, 60, 62, 80, 82, 76],
            &["SafeHands", "CatReflexes"],
        ),
        GoalkeeperArchetype::SweeperKeeper => make_player(
            id,
            name,
            Position::Goalkeeper,
            [58, 64, 66, 68, 68, 30, 34, 48, 46, 74, 66, 78, 72, 46, 66, 64, 74, 76, 70],
            &["SafeHands", "CoolHead", "Visionary"],
        ),
    }
}

fn make_defender(id: &str, name: &str, archetype: DefenderArchetype) -> PlayerData {
    match archetype {
        DefenderArchetype::Stopper => make_player(
            id,
            name,
            Position::Defender,
            [60, 71, 77, 58, 56, 42, 77, 50, 78, 76, 54, 72, 63, 71, 66, 65, 35, 35, 80],
            &["BallWinner", "Rock"],
        ),
        DefenderArchetype::BallPlayingCb => make_player(
            id,
            name,
            Position::Defender,
            [61, 68, 70, 62, 74, 40, 69, 60, 73, 72, 71, 74, 72, 58, 72, 67, 35, 35, 73],
            &["Playmaker", "Visionary", "CoolHead"],
        ),
        DefenderArchetype::FullBack => make_player(
            id,
            name,
            Position::Defender,
            [73, 76, 64, 70, 69, 45, 70, 68, 69, 69, 64, 70, 66, 60, 73, 68, 35, 35, 60],
            &["TeamPlayer", "Engine"],
        ),
        DefenderArchetype::WingBack => make_player(
            id,
            name,
            Position::Defender,
            [78, 80, 63, 75, 70, 48, 66, 74, 66, 64, 68, 69, 67, 63, 74, 66, 35, 35, 58],
            &["Speedster", "Dribbler", "Tireless"],
        ),
    }
}

fn make_midfielder(id: &str, name: &str, archetype: MidfielderArchetype) -> PlayerData {
    match archetype {
        MidfielderArchetype::Destroyer => make_player(
            id,
            name,
            Position::Midfielder,
            [66, 76, 68, 64, 60, 52, 77, 58, 72, 68, 58, 70, 64, 75, 70, 67, 35, 35, 58],
            &["BallWinner", "Engine"],
        ),
        MidfielderArchetype::Regista => make_player(
            id,
            name,
            Position::Midfielder,
            [60, 69, 60, 67, 80, 58, 58, 64, 61, 70, 82, 78, 76, 52, 78, 68, 35, 35, 52],
            &["Playmaker", "Visionary", "CoolHead"],
        ),
        MidfielderArchetype::BoxToBox => make_player(
            id,
            name,
            Position::Midfielder,
            [73, 81, 66, 70, 71, 65, 71, 69, 66, 69, 68, 71, 67, 66, 76, 66, 35, 35, 58],
            &["Engine", "TeamPlayer", "Tireless"],
        ),
        MidfielderArchetype::Creator => make_player(
            id,
            name,
            Position::Midfielder,
            [67, 68, 59, 76, 79, 69, 57, 77, 58, 66, 82, 77, 75, 50, 74, 64, 35, 35, 48],
            &["Playmaker", "Visionary", "Dribbler"],
        ),
        MidfielderArchetype::WideWorker => make_player(
            id,
            name,
            Position::Midfielder,
            [76, 79, 61, 73, 70, 60, 64, 74, 60, 65, 66, 70, 67, 58, 74, 62, 35, 35, 50],
            &["Speedster", "TeamPlayer", "Tireless"],
        ),
    }
}

fn make_forward(id: &str, name: &str, archetype: ForwardArchetype) -> PlayerData {
    match archetype {
        ForwardArchetype::Poacher => make_player(
            id,
            name,
            Position::Forward,
            [74, 70, 64, 71, 58, 81, 42, 70, 40, 80, 58, 78, 78, 52, 61, 60, 35, 35, 64],
            &["Sharpshooter", "CoolHead"],
        ),
        ForwardArchetype::TargetMan => make_player(
            id,
            name,
            Position::Forward,
            [61, 69, 79, 60, 62, 77, 45, 58, 42, 76, 58, 74, 74, 61, 68, 65, 35, 35, 82],
            &["CompleteForward", "CoolHead"],
        ),
        ForwardArchetype::Runner => make_player(
            id,
            name,
            Position::Forward,
            [83, 74, 63, 76, 63, 75, 44, 76, 40, 74, 60, 72, 70, 58, 64, 60, 35, 35, 58],
            &["Speedster", "Agile"],
        ),
        ForwardArchetype::CreatorForward => make_player(
            id,
            name,
            Position::Forward,
            [72, 72, 65, 77, 73, 73, 45, 78, 42, 73, 72, 74, 75, 52, 72, 61, 35, 35, 60],
            &["CompleteForward", "Dribbler", "Playmaker"],
        ),
    }
}

fn make_player(
    id: &str,
    name: &str,
    position: Position,
    stats: [u8; 19],
    traits: &[&str],
) -> PlayerData {
    PlayerData {
        id: id.to_string(),
        name: name.to_string(),
        position,
        condition: 90,
        fitness: 75,
        pace: stats[0],
        stamina: stats[1],
        strength: stats[2],
        agility: stats[3],
        passing: stats[4],
        shooting: stats[5],
        tackling: stats[6],
        dribbling: stats[7],
        defending: stats[8],
        positioning: stats[9],
        vision: stats[10],
        decisions: stats[11],
        composure: stats[12],
        aggression: stats[13],
        teamwork: stats[14],
        leadership: stats[15],
        handling: stats[16],
        reflexes: stats[17],
        aerial: stats[18],
        traits: traits.iter().map(|trait_name| (*trait_name).to_string()).collect(),
    }
}
