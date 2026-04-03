use engine::{
    MatchConfig, MatchReport, PlayStyle, PlayerData, Position, TacticalRole, TeamData,
    simulate_with_rng,
};
use rand::SeedableRng;
use rand::rngs::StdRng;
use serde::Serialize;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::env;

const DEFAULT_MATCHES_PER_LEG: u32 = 60;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize)]
enum TraitPackage {
    Baseline,
    ControlCore,
    LaunchCounter,
    ChaosRaiders,
    LaneThiefBlock,
    PressTrap,
}

impl TraitPackage {
    fn label(self) -> &'static str {
        match self {
            TraitPackage::Baseline => "Baseline",
            TraitPackage::ControlCore => "ControlCore",
            TraitPackage::LaunchCounter => "LaunchCounter",
            TraitPackage::ChaosRaiders => "ChaosRaiders",
            TraitPackage::LaneThiefBlock => "LaneThiefBlock",
            TraitPackage::PressTrap => "PressTrap",
        }
    }
}

#[derive(Clone)]
struct BlueprintDef {
    id: &'static str,
    formation: &'static str,
    style: PlayStyle,
    style_label: &'static str,
    package: TraitPackage,
    roles: Vec<TacticalRole>,
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

#[derive(Default, Clone, Copy)]
struct RoleAggregate {
    appearances: u32,
    minutes: u32,
    goals: u32,
    shots: u32,
    passes_completed: u32,
    passes_attempted: u32,
    tackles: u32,
    interceptions: u32,
    fouls: u32,
    rating_total: f64,
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
    opponent_formation: String,
    opponent_style: String,
    opponent_package: String,
    summary: AggregateView,
}

#[derive(Clone, Serialize)]
struct RoleFingerprintView {
    role: String,
    avg_minutes: f64,
    avg_goals: f64,
    avg_shots: f64,
    avg_passes_completed: f64,
    avg_pass_accuracy: f64,
    avg_tackles: f64,
    avg_interceptions: f64,
    avg_fouls: f64,
    avg_rating: f64,
}

#[derive(Serialize)]
struct BlueprintResult {
    id: String,
    formation: String,
    style: String,
    package: String,
    overall: AggregateView,
    role_fingerprints: Vec<RoleFingerprintView>,
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
struct ExploitView {
    blueprint_id: String,
    formation: String,
    style: String,
    package: String,
    overall: AggregateView,
    punished_blueprints: Vec<MatchupView>,
    counter_blueprints: Vec<MatchupView>,
    key_role_fingerprints: Vec<RoleFingerprintView>,
}

#[derive(Serialize)]
struct Output {
    matches_per_leg: u32,
    notes: Vec<String>,
    style_summary: Vec<SummaryView>,
    formation_summary: Vec<SummaryView>,
    package_summary: Vec<SummaryView>,
    exploit_watchlist: Vec<ExploitView>,
    blueprints: Vec<BlueprintResult>,
}

fn main() {
    let matches_per_leg = parse_matches_per_leg(env::args().skip(1).collect());
    let output = run_matrix(matches_per_leg);
    println!(
        "{}",
        serde_json::to_string_pretty(&output).expect("serialize role/trait matrix")
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
                    "Usage: cargo run -p engine --example role_trait_matrix --release -- [--matches-per-leg N]"
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
    let blueprints = blueprint_defs();
    let mut overall: HashMap<&'static str, Aggregate> = HashMap::new();
    let mut versus: HashMap<(&'static str, &'static str), Aggregate> = HashMap::new();
    let mut role_stats: HashMap<&'static str, HashMap<String, RoleAggregate>> = HashMap::new();

    for (left_index, left_blueprint) in blueprints.iter().enumerate() {
        for (right_index, right_blueprint) in blueprints.iter().enumerate() {
            if right_index < left_index {
                continue;
            }

            let left_home = make_team("left_home", "Left Home", left_blueprint);
            let right_away = make_team("right_away", "Right Away", right_blueprint);
            for seed in 0..matches_per_leg {
                let mut rng = StdRng::seed_from_u64(pair_seed(left_index, right_index, 0, seed));
                let report = simulate_with_rng(&left_home, &right_away, &config, &mut rng);
                record_match(
                    &report,
                    &left_home,
                    &right_away,
                    &mut overall,
                    &mut versus,
                    &mut role_stats,
                    left_blueprint.id,
                    right_blueprint.id,
                );
            }

            if left_index != right_index {
                let right_home = make_team("right_home", "Right Home", right_blueprint);
                let left_away = make_team("left_away", "Left Away", left_blueprint);
                for seed in 0..matches_per_leg {
                    let mut rng =
                        StdRng::seed_from_u64(pair_seed(left_index, right_index, 1, seed));
                    let report = simulate_with_rng(&right_home, &left_away, &config, &mut rng);
                    record_match(
                        &report,
                        &right_home,
                        &left_away,
                        &mut overall,
                        &mut versus,
                        &mut role_stats,
                        right_blueprint.id,
                        left_blueprint.id,
                    );
                }
            }
        }
    }

    let mut results = blueprints
        .iter()
        .map(|blueprint| {
            let mut matchups = blueprints
                .iter()
                .filter(|opponent| opponent.id != blueprint.id)
                .map(|opponent| MatchupView {
                    opponent_id: opponent.id.to_string(),
                    opponent_formation: opponent.formation.to_string(),
                    opponent_style: opponent.style_label.to_string(),
                    opponent_package: opponent.package.label().to_string(),
                    summary: to_view(
                        *versus
                            .get(&(blueprint.id, opponent.id))
                            .unwrap_or(&Aggregate::default()),
                    ),
                })
                .collect::<Vec<_>>();
            matchups.sort_by(|left, right| compare_aggregate_views(&right.summary, &left.summary));

            let role_fingerprints = role_stats
                .get(blueprint.id)
                .map(|stats| build_role_fingerprints(stats))
                .unwrap_or_default();

            BlueprintResult {
                id: blueprint.id.to_string(),
                formation: blueprint.formation.to_string(),
                style: blueprint.style_label.to_string(),
                package: blueprint.package.label().to_string(),
                overall: to_view(*overall.get(blueprint.id).unwrap_or(&Aggregate::default())),
                role_fingerprints,
                strongest_against: matchups.iter().take(5).cloned().collect(),
                weakest_against: matchups.iter().rev().take(5).cloned().collect(),
            }
        })
        .collect::<Vec<_>>();

    results.sort_by(|left, right| compare_aggregate_views(&right.overall, &left.overall));

    let exploit_watchlist = results
        .iter()
        .take(8)
        .map(|result| ExploitView {
            blueprint_id: result.id.clone(),
            formation: result.formation.clone(),
            style: result.style.clone(),
            package: result.package.clone(),
            overall: result.overall.clone(),
            punished_blueprints: result.strongest_against.iter().take(3).cloned().collect(),
            counter_blueprints: result.weakest_against.iter().take(3).cloned().collect(),
            key_role_fingerprints: result.role_fingerprints.iter().take(5).cloned().collect(),
        })
        .collect::<Vec<_>>();

    Output {
        matches_per_leg,
        notes: vec![
            "Each blueprint combines formation, play style, tactical role layout, and a style-aligned trait package.".to_string(),
            "The role/trait packages are intentionally synthetic so the benchmark surfaces engine exploits and counters rather than mirroring a specific save.".to_string(),
            "Exploit watchlist entries are simply the highest-performing blueprints in the current matrix; treat them as engine pressure tests, not final game balance recommendations.".to_string(),
        ],
        style_summary: build_group_summary(&results, |row| &row.style),
        formation_summary: build_group_summary(&results, |row| &row.formation),
        package_summary: build_group_summary(&results, |row| &row.package),
        exploit_watchlist,
        blueprints: results,
    }
}

fn build_group_summary<'a, F>(rows: &'a [BlueprintResult], key_fn: F) -> Vec<SummaryView>
where
    F: Fn(&'a BlueprintResult) -> &'a str,
{
    let mut groups: HashMap<&str, Vec<&BlueprintResult>> = HashMap::new();
    for row in rows {
        groups.entry(key_fn(row)).or_default().push(row);
    }

    let mut summary = groups
        .into_iter()
        .map(|(id, items)| {
            let count = items.len().max(1) as f64;
            SummaryView {
                id: id.to_string(),
                average_points_per_match: items
                    .iter()
                    .map(|row| row.overall.points_per_match)
                    .sum::<f64>()
                    / count,
                average_goal_diff_per_match: items
                    .iter()
                    .map(|row| row.overall.goal_diff_per_match)
                    .sum::<f64>()
                    / count,
                average_goals_for_per_match: items
                    .iter()
                    .map(|row| row.overall.goals_for_per_match)
                    .sum::<f64>()
                    / count,
                average_goals_against_per_match: items
                    .iter()
                    .map(|row| row.overall.goals_against_per_match)
                    .sum::<f64>()
                    / count,
                average_possession: items
                    .iter()
                    .map(|row| row.overall.avg_possession)
                    .sum::<f64>()
                    / count,
            }
        })
        .collect::<Vec<_>>();

    summary.sort_by(|left, right| {
        right
            .average_points_per_match
            .partial_cmp(&left.average_points_per_match)
            .unwrap_or(Ordering::Equal)
            .then_with(|| {
                right
                    .average_goal_diff_per_match
                    .partial_cmp(&left.average_goal_diff_per_match)
                    .unwrap_or(Ordering::Equal)
            })
    });
    summary
}

fn build_role_fingerprints(stats: &HashMap<String, RoleAggregate>) -> Vec<RoleFingerprintView> {
    let mut views = stats
        .iter()
        .map(|(role, aggregate)| {
            let appearances = aggregate.appearances.max(1) as f64;
            let pass_attempts = aggregate.passes_attempted.max(1) as f64;
            RoleFingerprintView {
                role: role.clone(),
                avg_minutes: aggregate.minutes as f64 / appearances,
                avg_goals: aggregate.goals as f64 / appearances,
                avg_shots: aggregate.shots as f64 / appearances,
                avg_passes_completed: aggregate.passes_completed as f64 / appearances,
                avg_pass_accuracy: aggregate.passes_completed as f64 / pass_attempts * 100.0,
                avg_tackles: aggregate.tackles as f64 / appearances,
                avg_interceptions: aggregate.interceptions as f64 / appearances,
                avg_fouls: aggregate.fouls as f64 / appearances,
                avg_rating: aggregate.rating_total / appearances,
            }
        })
        .collect::<Vec<_>>();

    views.sort_by(|left, right| {
        right
            .avg_rating
            .partial_cmp(&left.avg_rating)
            .unwrap_or(Ordering::Equal)
            .then_with(|| {
                right
                    .avg_shots
                    .partial_cmp(&left.avg_shots)
                    .unwrap_or(Ordering::Equal)
            })
    });
    views
}

fn compare_aggregate_views(left: &AggregateView, right: &AggregateView) -> Ordering {
    left.points_per_match
        .partial_cmp(&right.points_per_match)
        .unwrap_or(Ordering::Equal)
        .then_with(|| {
            left.goal_diff_per_match
                .partial_cmp(&right.goal_diff_per_match)
                .unwrap_or(Ordering::Equal)
        })
}

fn pair_seed(left_index: usize, right_index: usize, leg: u64, seed: u32) -> u64 {
    ((left_index as u64) << 40) ^ ((right_index as u64) << 24) ^ (leg << 20) ^ u64::from(seed)
}

fn record_match(
    report: &MatchReport,
    home_team: &TeamData,
    away_team: &TeamData,
    overall: &mut HashMap<&'static str, Aggregate>,
    versus: &mut HashMap<(&'static str, &'static str), Aggregate>,
    role_stats: &mut HashMap<&'static str, HashMap<String, RoleAggregate>>,
    home_id: &'static str,
    away_id: &'static str,
) {
    {
        let aggregate = overall.entry(home_id).or_default();
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
        let aggregate = overall.entry(away_id).or_default();
        update_aggregate(
            aggregate,
            report.away_goals as u32,
            report.home_goals as u32,
            report.away_stats.shots as u32,
            report.home_stats.shots as u32,
            100.0 - report.home_possession,
        );
    }
    {
        let aggregate = versus.entry((home_id, away_id)).or_default();
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
        let aggregate = versus.entry((away_id, home_id)).or_default();
        update_aggregate(
            aggregate,
            report.away_goals as u32,
            report.home_goals as u32,
            report.away_stats.shots as u32,
            report.home_stats.shots as u32,
            100.0 - report.home_possession,
        );
    }

    record_role_stats(
        role_stats.entry(home_id).or_default(),
        home_team,
        &report.player_stats,
    );
    record_role_stats(
        role_stats.entry(away_id).or_default(),
        away_team,
        &report.player_stats,
    );
}

fn record_role_stats(
    aggregates: &mut HashMap<String, RoleAggregate>,
    team: &TeamData,
    player_stats: &HashMap<String, engine::report::PlayerMatchStats>,
) {
    for player in &team.players {
        if let Some(stats) = player_stats.get(&player.id) {
            let key = role_label(player.role).to_string();
            let aggregate = aggregates.entry(key).or_default();
            aggregate.appearances += 1;
            aggregate.minutes += stats.minutes_played as u32;
            aggregate.goals += stats.goals as u32;
            aggregate.shots += stats.shots as u32;
            aggregate.passes_completed += stats.passes_completed as u32;
            aggregate.passes_attempted += stats.passes_attempted as u32;
            aggregate.tackles += stats.tackles_won as u32;
            aggregate.interceptions += stats.interceptions as u32;
            aggregate.fouls += stats.fouls_committed as u32;
            aggregate.rating_total += f64::from(stats.rating);
        }
    }
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

fn blueprint_defs() -> Vec<BlueprintDef> {
    let formations = [
        "4-4-2", "4-3-3", "3-5-2", "4-5-1", "4-2-3-1", "4-1-4-1", "3-4-3", "5-3-2",
    ];
    let styles = [
        (PlayStyle::Balanced, "Balanced"),
        (PlayStyle::Attacking, "Attacking"),
        (PlayStyle::Defensive, "Defensive"),
        (PlayStyle::Possession, "Possession"),
        (PlayStyle::Counter, "Counter"),
        (PlayStyle::HighPress, "HighPress"),
    ];

    formations
        .iter()
        .flat_map(|formation| {
            styles.iter().map(move |(style, style_label)| {
                let package = package_for_style(*style);
                let id = Box::leak(
                    format!("{}__{}__{}", formation, style_label, package.label()).into_boxed_str(),
                );
                BlueprintDef {
                    id,
                    formation,
                    style: *style,
                    style_label,
                    package,
                    roles: default_roles_for_formation(formation),
                }
            })
        })
        .collect()
}

fn package_for_style(style: PlayStyle) -> TraitPackage {
    match style {
        PlayStyle::Balanced => TraitPackage::Baseline,
        PlayStyle::Attacking => TraitPackage::ChaosRaiders,
        PlayStyle::Defensive => TraitPackage::LaneThiefBlock,
        PlayStyle::Possession => TraitPackage::ControlCore,
        PlayStyle::Counter => TraitPackage::LaunchCounter,
        PlayStyle::HighPress => TraitPackage::PressTrap,
    }
}

fn make_team(id: &str, name: &str, blueprint: &BlueprintDef) -> TeamData {
    let players = blueprint
        .roles
        .iter()
        .enumerate()
        .map(|(index, role)| make_role_player(id, index, *role, blueprint.package))
        .collect::<Vec<_>>();

    TeamData {
        id: id.to_string(),
        name: name.to_string(),
        formation: blueprint.formation.to_string(),
        play_style: blueprint.style,
        players,
    }
}

fn make_role_player(
    team_id: &str,
    index: usize,
    role: TacticalRole,
    package: TraitPackage,
) -> PlayerData {
    let mut player = base_player_for_role(
        format!(
            "{}_{}_{}",
            team_id,
            index,
            role_label(role).to_lowercase().replace(' ', "_")
        ),
        format!("{} {}", role_label(role), index + 1),
        role,
    );
    apply_package(&mut player, package);
    player
}

fn base_player_for_role(id: String, name: String, role: TacticalRole) -> PlayerData {
    let position = position_for_role(role);
    let mut player = match role {
        TacticalRole::Goalkeeper => PlayerData {
            id,
            name,
            position,
            condition: 90,
            fitness: 75,
            pace: 44,
            stamina: 58,
            strength: 66,
            agility: 60,
            passing: 54,
            shooting: 25,
            tackling: 28,
            dribbling: 34,
            defending: 40,
            positioning: 72,
            vision: 58,
            decisions: 70,
            composure: 68,
            aggression: 45,
            teamwork: 62,
            leadership: 64,
            handling: 76,
            reflexes: 74,
            aerial: 72,
            traits: vec![],
            role,
        },
        TacticalRole::SweeperKeeper => PlayerData {
            id,
            name,
            position,
            condition: 90,
            fitness: 75,
            pace: 50,
            stamina: 60,
            strength: 66,
            agility: 64,
            passing: 62,
            shooting: 28,
            tackling: 34,
            dribbling: 42,
            defending: 44,
            positioning: 72,
            vision: 64,
            decisions: 72,
            composure: 70,
            aggression: 50,
            teamwork: 64,
            leadership: 64,
            handling: 74,
            reflexes: 72,
            aerial: 70,
            traits: vec![],
            role,
        },
        TacticalRole::CenterBackStopper => outfield_player(
            id, name, position, role, 60, 66, 72, 56, 56, 42, 74, 50, 76, 72, 58, 66, 64, 68,
        ),
        TacticalRole::CenterBackCover => outfield_player(
            id, name, position, role, 66, 68, 68, 60, 58, 40, 72, 52, 74, 74, 60, 66, 66, 66,
        ),
        TacticalRole::CenterBackPlaymaker => outfield_player(
            id, name, position, role, 62, 66, 68, 58, 70, 38, 68, 56, 72, 70, 70, 72, 68, 68,
        ),
        TacticalRole::FullBackSupport => outfield_player(
            id, name, position, role, 72, 72, 60, 68, 66, 40, 68, 66, 66, 64, 64, 68, 66, 58,
        ),
        TacticalRole::WingBackAttack => outfield_player(
            id, name, position, role, 76, 78, 58, 72, 68, 46, 66, 74, 62, 60, 66, 66, 68, 54,
        ),
        TacticalRole::HoldingMidfielder => outfield_player(
            id, name, position, role, 62, 74, 66, 62, 72, 48, 72, 56, 70, 72, 68, 72, 70, 60,
        ),
        TacticalRole::DeepPlaymaker => outfield_player(
            id, name, position, role, 64, 72, 60, 64, 78, 50, 64, 62, 64, 68, 80, 78, 72, 54,
        ),
        TacticalRole::BoxToBoxMidfielder => outfield_player(
            id, name, position, role, 70, 78, 64, 68, 72, 56, 68, 68, 66, 68, 70, 72, 74, 56,
        ),
        TacticalRole::AdvancedPlaymaker => outfield_player(
            id, name, position, role, 68, 70, 56, 72, 80, 62, 56, 74, 64, 70, 82, 80, 70, 48,
        ),
        TacticalRole::WideProgressor => outfield_player(
            id, name, position, role, 76, 72, 56, 76, 70, 60, 56, 78, 60, 66, 70, 70, 66, 48,
        ),
        TacticalRole::Poacher => outfield_player(
            id, name, position, role, 78, 68, 62, 74, 56, 82, 38, 72, 46, 80, 62, 72, 60, 66,
        ),
        TacticalRole::TargetForward => outfield_player(
            id, name, position, role, 68, 68, 76, 60, 60, 78, 34, 58, 48, 78, 60, 70, 58, 76,
        ),
        TacticalRole::ChannelRunner => outfield_player(
            id, name, position, role, 82, 70, 60, 78, 58, 78, 36, 78, 44, 78, 60, 70, 58, 60,
        ),
        TacticalRole::LinkForward => outfield_player(
            id, name, position, role, 70, 70, 68, 68, 70, 72, 38, 66, 48, 74, 72, 74, 68, 68,
        ),
    };
    player.role = role;
    player
}

fn outfield_player(
    id: String,
    name: String,
    position: Position,
    role: TacticalRole,
    pace: u8,
    stamina: u8,
    strength: u8,
    agility: u8,
    passing: u8,
    shooting: u8,
    tackling: u8,
    dribbling: u8,
    defending: u8,
    positioning: u8,
    vision: u8,
    decisions: u8,
    teamwork: u8,
    aerial: u8,
) -> PlayerData {
    PlayerData {
        id,
        name,
        position,
        condition: 90,
        fitness: 75,
        pace,
        stamina,
        strength,
        agility,
        passing,
        shooting,
        tackling,
        dribbling,
        defending,
        positioning,
        vision,
        decisions,
        composure: decisions.saturating_sub(2).max(52),
        aggression: tackling.saturating_sub(4).max(45),
        teamwork,
        leadership: teamwork.saturating_sub(2).max(50),
        handling: 35,
        reflexes: 35,
        aerial,
        traits: vec![],
        role,
    }
}

fn apply_package(player: &mut PlayerData, package: TraitPackage) {
    match package {
        TraitPackage::Baseline => apply_baseline_package(player),
        TraitPackage::ControlCore => apply_control_core_package(player),
        TraitPackage::LaunchCounter => apply_launch_counter_package(player),
        TraitPackage::ChaosRaiders => apply_chaos_raiders_package(player),
        TraitPackage::LaneThiefBlock => apply_lane_thief_block_package(player),
        TraitPackage::PressTrap => apply_press_trap_package(player),
    }
}

fn apply_baseline_package(player: &mut PlayerData) {
    match player.role {
        TacticalRole::DeepPlaymaker | TacticalRole::AdvancedPlaymaker => {
            add_traits(player, &["Playmaker", "TeamPlayer"])
        }
        TacticalRole::BoxToBoxMidfielder => add_traits(player, &["Engine"]),
        TacticalRole::CenterBackPlaymaker => add_traits(player, &["EarlyScanner"]),
        TacticalRole::Poacher => add_traits(player, &["Sharpshooter"]),
        TacticalRole::ChannelRunner => add_traits(player, &["BlindSideRunner"]),
        TacticalRole::Goalkeeper | TacticalRole::SweeperKeeper => {
            add_traits(player, &["SafeHands"])
        }
        _ => {}
    }
}

fn apply_control_core_package(player: &mut PlayerData) {
    match player.role {
        TacticalRole::CenterBackPlaymaker => {
            bump(player, 0, 0, 0, 0, 8, 0, 0, 0, 0, 4, 8, 6);
            add_traits(player, &["EarlyScanner", "PressBaiter", "Playmaker"]);
        }
        TacticalRole::DeepPlaymaker | TacticalRole::AdvancedPlaymaker => {
            bump(player, 0, 0, 0, 0, 10, 0, 0, 2, 0, 2, 10, 8);
            add_traits(
                player,
                &[
                    "Playmaker",
                    "Visionary",
                    "DelayedPasser",
                    "OneTouchSpecialist",
                    "TempoManipulator",
                ],
            );
        }
        TacticalRole::HoldingMidfielder | TacticalRole::LinkForward => {
            bump(player, 0, 0, 0, 0, 6, 0, 0, 0, 0, 4, 6, 6);
            add_traits(player, &["TeamPlayer", "SpaceMagnet", "RiskCalibrator"]);
        }
        TacticalRole::Goalkeeper | TacticalRole::SweeperKeeper => {
            bump(player, 0, 0, 0, 0, 4, 0, 0, 0, 0, 0, 4, 4);
            add_traits(player, &["SafeHands", "TrafficCommander"]);
        }
        _ => add_traits(player, &["TeamPlayer"]),
    }
}

fn apply_launch_counter_package(player: &mut PlayerData) {
    match player.role {
        TacticalRole::Goalkeeper | TacticalRole::SweeperKeeper => {
            player.role = TacticalRole::SweeperKeeper;
            bump(player, 4, 0, 0, 2, 10, 0, 0, 0, 0, 0, 8, 8);
            player.handling = player.handling.saturating_add(6);
            player.reflexes = player.reflexes.saturating_add(4);
            add_traits(player, &["ThrowLauncher", "SafeHands", "CatReflexes"]);
        }
        TacticalRole::CenterBackPlaymaker | TacticalRole::DeepPlaymaker => {
            bump(player, 0, 0, 0, 0, 6, 0, 0, 0, 0, 2, 8, 6);
            add_traits(
                player,
                &["EarlyScanner", "PressBaiter", "TransitionAnticipator"],
            );
        }
        TacticalRole::WingBackAttack
        | TacticalRole::WideProgressor
        | TacticalRole::ChannelRunner => {
            bump(player, 6, 2, 0, 4, 0, 2, 0, 6, 0, 4, 0, 2);
            add_traits(
                player,
                &["TransitionAnticipator", "BlindSideRunner", "ChannelDrifter"],
            );
        }
        TacticalRole::Poacher | TacticalRole::TargetForward => {
            bump(player, 4, 0, 2, 2, 0, 6, 0, 2, 0, 4, 0, 2);
            add_traits(
                player,
                &["BlindSideRunner", "NearPostHunter", "TransitionAnticipator"],
            );
        }
        _ => add_traits(player, &["TransitionAnticipator"]),
    }
}

fn apply_chaos_raiders_package(player: &mut PlayerData) {
    match player.role {
        TacticalRole::WideProgressor | TacticalRole::ChannelRunner | TacticalRole::Poacher => {
            bump(player, 4, 0, 0, 6, 0, 6, 0, 8, 0, 4, 0, 0);
            add_traits(
                player,
                &[
                    "ChaosCreator",
                    "Dribbler",
                    "BlindSideRunner",
                    "NearPostHunter",
                ],
            );
        }
        TacticalRole::AdvancedPlaymaker | TacticalRole::BoxToBoxMidfielder => {
            bump(player, 0, 2, 0, 2, 2, 4, 0, 2, 0, 6, 2, 2);
            add_traits(player, &["LateBoxArriver", "FarPostGhost", "ChaosCreator"]);
        }
        TacticalRole::TargetForward => {
            bump(player, 0, 0, 4, 0, 0, 6, 0, 0, 0, 4, 0, 0);
            add_traits(player, &["NearPostHunter", "ReboundInstinct"]);
        }
        _ => add_traits(player, &["ChaosCreator"]),
    }
}

fn apply_lane_thief_block_package(player: &mut PlayerData) {
    match player.role {
        TacticalRole::CenterBackStopper
        | TacticalRole::CenterBackCover
        | TacticalRole::HoldingMidfielder => {
            bump(player, 0, 2, 2, 0, 0, 0, 8, 0, 6, 8, 4, 4);
            add_traits(
                player,
                &[
                    "PassingLaneThief",
                    "ContainmentSpecialist",
                    "TacticalFouler",
                ],
            );
        }
        TacticalRole::FullBackSupport
        | TacticalRole::WingBackAttack
        | TacticalRole::BoxToBoxMidfielder => {
            bump(player, 2, 2, 0, 0, 0, 0, 4, 0, 4, 6, 2, 4);
            add_traits(player, &["ContainmentSpecialist", "RecoverySprinter"]);
        }
        TacticalRole::Goalkeeper | TacticalRole::SweeperKeeper => {
            player.handling = player.handling.saturating_add(6);
            player.positioning = player.positioning.saturating_add(4);
            add_traits(player, &["SafeHands", "TrafficCommander"]);
        }
        _ => add_traits(player, &["TeamPlayer"]),
    }
}

fn apply_press_trap_package(player: &mut PlayerData) {
    match player.role {
        TacticalRole::HoldingMidfielder
        | TacticalRole::BoxToBoxMidfielder
        | TacticalRole::ChannelRunner => {
            bump(player, 4, 8, 0, 2, 0, 0, 6, 0, 2, 4, 2, 6);
            add_traits(
                player,
                &["Engine", "Tireless", "TransitionAnticipator", "BallWinner"],
            );
        }
        TacticalRole::Poacher | TacticalRole::LinkForward => {
            bump(player, 4, 4, 0, 2, 0, 2, 4, 2, 0, 2, 0, 2);
            add_traits(player, &["KeeperDisruptor", "TransitionAnticipator"]);
        }
        TacticalRole::CenterBackCover | TacticalRole::CenterBackPlaymaker => {
            bump(player, 2, 2, 0, 0, 2, 0, 4, 0, 4, 4, 4, 2);
            add_traits(player, &["PassingLaneThief", "RecoverySprinter"]);
        }
        _ => add_traits(player, &["Engine"]),
    }
}

fn add_traits(player: &mut PlayerData, traits: &[&str]) {
    for trait_name in traits {
        if !player.traits.iter().any(|existing| existing == trait_name) {
            player.traits.push((*trait_name).to_string());
        }
    }
}

fn bump(
    player: &mut PlayerData,
    pace: u8,
    stamina: u8,
    strength: u8,
    agility: u8,
    passing: u8,
    shooting: u8,
    tackling: u8,
    dribbling: u8,
    defending: u8,
    positioning: u8,
    vision: u8,
    decisions: u8,
) {
    player.pace = player.pace.saturating_add(pace);
    player.stamina = player.stamina.saturating_add(stamina);
    player.strength = player.strength.saturating_add(strength);
    player.agility = player.agility.saturating_add(agility);
    player.passing = player.passing.saturating_add(passing);
    player.shooting = player.shooting.saturating_add(shooting);
    player.tackling = player.tackling.saturating_add(tackling);
    player.dribbling = player.dribbling.saturating_add(dribbling);
    player.defending = player.defending.saturating_add(defending);
    player.positioning = player.positioning.saturating_add(positioning);
    player.vision = player.vision.saturating_add(vision);
    player.decisions = player.decisions.saturating_add(decisions);
    player.composure = player.composure.saturating_add(decisions / 2);
    player.teamwork = player.teamwork.saturating_add((stamina / 2).max(1));
}

fn position_for_role(role: TacticalRole) -> Position {
    match role {
        TacticalRole::Goalkeeper | TacticalRole::SweeperKeeper => Position::Goalkeeper,
        TacticalRole::CenterBackStopper
        | TacticalRole::CenterBackCover
        | TacticalRole::CenterBackPlaymaker
        | TacticalRole::FullBackSupport
        | TacticalRole::WingBackAttack => Position::Defender,
        TacticalRole::HoldingMidfielder
        | TacticalRole::DeepPlaymaker
        | TacticalRole::BoxToBoxMidfielder
        | TacticalRole::AdvancedPlaymaker
        | TacticalRole::WideProgressor => Position::Midfielder,
        TacticalRole::Poacher
        | TacticalRole::TargetForward
        | TacticalRole::ChannelRunner
        | TacticalRole::LinkForward => Position::Forward,
    }
}

fn role_label(role: TacticalRole) -> &'static str {
    match role {
        TacticalRole::Goalkeeper => "Goalkeeper",
        TacticalRole::SweeperKeeper => "SweeperKeeper",
        TacticalRole::CenterBackStopper => "CenterBackStopper",
        TacticalRole::CenterBackCover => "CenterBackCover",
        TacticalRole::CenterBackPlaymaker => "CenterBackPlaymaker",
        TacticalRole::FullBackSupport => "FullBackSupport",
        TacticalRole::WingBackAttack => "WingBackAttack",
        TacticalRole::HoldingMidfielder => "HoldingMidfielder",
        TacticalRole::DeepPlaymaker => "DeepPlaymaker",
        TacticalRole::BoxToBoxMidfielder => "BoxToBoxMidfielder",
        TacticalRole::AdvancedPlaymaker => "AdvancedPlaymaker",
        TacticalRole::WideProgressor => "WideProgressor",
        TacticalRole::Poacher => "Poacher",
        TacticalRole::TargetForward => "TargetForward",
        TacticalRole::ChannelRunner => "ChannelRunner",
        TacticalRole::LinkForward => "LinkForward",
    }
}

fn default_roles_for_formation(formation: &str) -> Vec<TacticalRole> {
    let rows = formation_role_rows(formation);
    let mut roles = Vec::new();

    for row in rows {
        let count = row.len();
        for (index, role) in row.into_iter().enumerate() {
            let resolved = match role {
                TacticalRole::CenterBackStopper if count >= 3 && index == 0 => {
                    TacticalRole::CenterBackCover
                }
                TacticalRole::CenterBackStopper if count >= 3 && index + 1 == count => {
                    TacticalRole::CenterBackPlaymaker
                }
                TacticalRole::Poacher if count == 1 => TacticalRole::LinkForward,
                TacticalRole::Poacher if count == 2 && index == 0 => TacticalRole::TargetForward,
                TacticalRole::BoxToBoxMidfielder if count == 2 && index == 0 => {
                    TacticalRole::DeepPlaymaker
                }
                other => other,
            };
            roles.push(resolved);
        }
    }

    roles
}

fn formation_role_rows(formation: &str) -> Vec<Vec<TacticalRole>> {
    let parts = formation
        .split('-')
        .filter_map(|part| part.parse::<usize>().ok())
        .collect::<Vec<_>>();

    match parts.as_slice() {
        [defenders, midfielders, forwards] => vec![
            vec![TacticalRole::Goalkeeper],
            defender_role_row(*defenders),
            midfield_role_row(*midfielders),
            forward_role_row(*forwards),
        ],
        [defenders, deep_midfielders, attacking_midfielders, forwards] => vec![
            vec![TacticalRole::Goalkeeper],
            defender_role_row(*defenders),
            deep_midfield_role_row(*deep_midfielders),
            attacking_midfield_role_row(*attacking_midfielders),
            forward_role_row(*forwards),
        ],
        _ => formation_role_rows("4-4-2"),
    }
}

fn defender_role_row(count: usize) -> Vec<TacticalRole> {
    match count {
        3 => vec![
            TacticalRole::CenterBackStopper,
            TacticalRole::CenterBackStopper,
            TacticalRole::CenterBackStopper,
        ],
        4 => vec![
            TacticalRole::FullBackSupport,
            TacticalRole::CenterBackPlaymaker,
            TacticalRole::CenterBackStopper,
            TacticalRole::FullBackSupport,
        ],
        5 => vec![
            TacticalRole::WingBackAttack,
            TacticalRole::CenterBackStopper,
            TacticalRole::CenterBackStopper,
            TacticalRole::CenterBackStopper,
            TacticalRole::WingBackAttack,
        ],
        _ => vec![TacticalRole::CenterBackStopper; count],
    }
}

fn midfield_role_row(count: usize) -> Vec<TacticalRole> {
    match count {
        2 => vec![
            TacticalRole::BoxToBoxMidfielder,
            TacticalRole::BoxToBoxMidfielder,
        ],
        3 => vec![
            TacticalRole::HoldingMidfielder,
            TacticalRole::BoxToBoxMidfielder,
            TacticalRole::AdvancedPlaymaker,
        ],
        4 => vec![
            TacticalRole::WideProgressor,
            TacticalRole::DeepPlaymaker,
            TacticalRole::BoxToBoxMidfielder,
            TacticalRole::WideProgressor,
        ],
        5 => vec![
            TacticalRole::WideProgressor,
            TacticalRole::HoldingMidfielder,
            TacticalRole::BoxToBoxMidfielder,
            TacticalRole::AdvancedPlaymaker,
            TacticalRole::WideProgressor,
        ],
        _ => vec![TacticalRole::BoxToBoxMidfielder; count],
    }
}

fn deep_midfield_role_row(count: usize) -> Vec<TacticalRole> {
    match count {
        1 => vec![TacticalRole::HoldingMidfielder],
        2 => vec![TacticalRole::HoldingMidfielder, TacticalRole::DeepPlaymaker],
        _ => vec![TacticalRole::HoldingMidfielder; count],
    }
}

fn attacking_midfield_role_row(count: usize) -> Vec<TacticalRole> {
    match count {
        1 => vec![TacticalRole::AdvancedPlaymaker],
        2 => vec![
            TacticalRole::WideProgressor,
            TacticalRole::AdvancedPlaymaker,
        ],
        3 => vec![
            TacticalRole::WideProgressor,
            TacticalRole::AdvancedPlaymaker,
            TacticalRole::WideProgressor,
        ],
        _ => vec![TacticalRole::AdvancedPlaymaker; count],
    }
}

fn forward_role_row(count: usize) -> Vec<TacticalRole> {
    match count {
        1 => vec![TacticalRole::Poacher],
        2 => vec![TacticalRole::Poacher, TacticalRole::Poacher],
        3 => vec![
            TacticalRole::ChannelRunner,
            TacticalRole::Poacher,
            TacticalRole::ChannelRunner,
        ],
        _ => vec![TacticalRole::Poacher; count],
    }
}
