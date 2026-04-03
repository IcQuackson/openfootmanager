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

const DEFAULT_MATCHES_PER_LEG: u32 = 12;

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

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize)]
enum FitProfile {
    BadFit,
    NeutralFit,
    IdealFit,
}

impl FitProfile {
    fn label(self) -> &'static str {
        match self {
            FitProfile::BadFit => "BadFit",
            FitProfile::NeutralFit => "NeutralFit",
            FitProfile::IdealFit => "IdealFit",
        }
    }
}

#[derive(Clone)]
struct SystemDef {
    id: &'static str,
    formation: &'static str,
    style: PlayStyle,
    style_label: &'static str,
    target_package: TraitPackage,
    roles: Vec<TacticalRole>,
}

#[derive(Clone, Copy)]
struct VariantDef {
    id: &'static str,
    system_index: usize,
    fit: FitProfile,
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
    opponent_system_id: String,
    opponent_formation: String,
    opponent_style: String,
    opponent_fit: String,
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

#[derive(Clone, Serialize)]
struct ProfileResult {
    fit_profile: String,
    overall: AggregateView,
    role_fingerprints: Vec<RoleFingerprintView>,
    strongest_against: Vec<MatchupView>,
    weakest_against: Vec<MatchupView>,
}

#[derive(Clone, Serialize)]
struct CounterResponseView {
    opponent_system_id: String,
    opponent_formation: String,
    opponent_style: String,
    bad_fit: AggregateView,
    neutral_fit: AggregateView,
    ideal_fit: AggregateView,
}

#[derive(Clone, Serialize)]
struct FitCurveView {
    classification: String,
    bad_fit: AggregateView,
    neutral_fit: AggregateView,
    ideal_fit: AggregateView,
    ideal_minus_bad_ppg: f64,
    ideal_minus_neutral_ppg: f64,
    neutral_minus_bad_ppg: f64,
}

#[derive(Clone, Serialize)]
struct SystemResult {
    id: String,
    formation: String,
    style: String,
    target_package: String,
    fit_curve: FitCurveView,
    profiles: Vec<ProfileResult>,
    ideal_punishes: Vec<CounterResponseView>,
    ideal_counters: Vec<CounterResponseView>,
}

#[derive(Clone, Serialize)]
struct FitSummaryView {
    id: String,
    formation: String,
    style: String,
    classification: String,
    bad_fit_ppg: f64,
    neutral_fit_ppg: f64,
    ideal_fit_ppg: f64,
    ideal_minus_bad_ppg: f64,
    ideal_minus_neutral_ppg: f64,
}

#[derive(Serialize)]
struct Output {
    matches_per_leg: u32,
    notes: Vec<String>,
    fit_sensitivity_summary: Vec<FitSummaryView>,
    player_dependent_watchlist: Vec<FitSummaryView>,
    mechanically_overpowered_watchlist: Vec<FitSummaryView>,
    systems: Vec<SystemResult>,
}

fn main() {
    let matches_per_leg = parse_matches_per_leg(env::args().skip(1).collect());
    let output = run_matrix(matches_per_leg);
    println!(
        "{}",
        serde_json::to_string_pretty(&output).expect("serialize fit profile matrix")
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
                    "Usage: cargo run -p engine --example fit_profile_matrix --release -- [--matches-per-leg N]"
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
    let systems = system_defs();
    let variants = variant_defs(&systems);

    let mut overall: HashMap<&'static str, Aggregate> = HashMap::new();
    let mut versus: HashMap<(&'static str, &'static str), Aggregate> = HashMap::new();
    let mut role_stats: HashMap<&'static str, HashMap<String, RoleAggregate>> = HashMap::new();

    for (left_index, left_variant) in variants.iter().enumerate() {
        for (right_index, right_variant) in variants.iter().enumerate() {
            if right_index < left_index {
                continue;
            }

            let left_system = &systems[left_variant.system_index];
            let right_system = &systems[right_variant.system_index];

            let left_home = make_team("left_home", "Left Home", left_system, left_variant.fit);
            let right_away = make_team("right_away", "Right Away", right_system, right_variant.fit);
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
                    left_variant.id,
                    right_variant.id,
                );
            }

            if left_index != right_index {
                let right_home =
                    make_team("right_home", "Right Home", right_system, right_variant.fit);
                let left_away = make_team("left_away", "Left Away", left_system, left_variant.fit);
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
                        right_variant.id,
                        left_variant.id,
                    );
                }
            }
        }
    }

    let mut fit_sensitivity_summary = Vec::new();
    let mut systems_output = Vec::new();

    for (system_index, system) in systems.iter().enumerate() {
        let profiles = [
            FitProfile::BadFit,
            FitProfile::NeutralFit,
            FitProfile::IdealFit,
        ]
        .into_iter()
        .map(|fit| {
            build_profile_result(
                system,
                system_index,
                fit,
                &systems,
                &variants,
                &overall,
                &versus,
                &role_stats,
            )
        })
        .collect::<Vec<_>>();

        let bad_fit = profiles
            .iter()
            .find(|profile| profile.fit_profile == FitProfile::BadFit.label())
            .expect("bad fit profile")
            .overall
            .clone();
        let neutral_fit = profiles
            .iter()
            .find(|profile| profile.fit_profile == FitProfile::NeutralFit.label())
            .expect("neutral fit profile")
            .overall
            .clone();
        let ideal_fit = profiles
            .iter()
            .find(|profile| profile.fit_profile == FitProfile::IdealFit.label())
            .expect("ideal fit profile")
            .overall
            .clone();

        let fit_curve = build_fit_curve(&bad_fit, &neutral_fit, &ideal_fit);
        let counter_responses =
            build_counter_responses(system, system_index, &systems, &variants, &versus);
        let mut punishes = counter_responses.clone();
        punishes.sort_by(|left, right| compare_aggregate_views(&right.ideal_fit, &left.ideal_fit));
        let mut counters = counter_responses.clone();
        counters.sort_by(|left, right| compare_aggregate_views(&left.ideal_fit, &right.ideal_fit));

        let summary = FitSummaryView {
            id: system.id.to_string(),
            formation: system.formation.to_string(),
            style: system.style_label.to_string(),
            classification: fit_curve.classification.clone(),
            bad_fit_ppg: bad_fit.points_per_match,
            neutral_fit_ppg: neutral_fit.points_per_match,
            ideal_fit_ppg: ideal_fit.points_per_match,
            ideal_minus_bad_ppg: fit_curve.ideal_minus_bad_ppg,
            ideal_minus_neutral_ppg: fit_curve.ideal_minus_neutral_ppg,
        };
        fit_sensitivity_summary.push(summary);

        systems_output.push(SystemResult {
            id: system.id.to_string(),
            formation: system.formation.to_string(),
            style: system.style_label.to_string(),
            target_package: system.target_package.label().to_string(),
            fit_curve,
            profiles,
            ideal_punishes: punishes.into_iter().take(5).collect(),
            ideal_counters: counters.into_iter().take(5).collect(),
        });
    }

    fit_sensitivity_summary.sort_by(|left, right| {
        right
            .ideal_fit_ppg
            .partial_cmp(&left.ideal_fit_ppg)
            .unwrap_or(Ordering::Equal)
            .then_with(|| {
                right
                    .ideal_minus_bad_ppg
                    .partial_cmp(&left.ideal_minus_bad_ppg)
                    .unwrap_or(Ordering::Equal)
            })
    });

    systems_output.sort_by(|left, right| {
        compare_aggregate_views(&right.fit_curve.ideal_fit, &left.fit_curve.ideal_fit)
    });

    let mut player_dependent_watchlist = fit_sensitivity_summary
        .iter()
        .filter(|row| row.classification == "player_dependent")
        .cloned()
        .collect::<Vec<_>>();
    player_dependent_watchlist.sort_by(|left, right| {
        right
            .ideal_minus_bad_ppg
            .partial_cmp(&left.ideal_minus_bad_ppg)
            .unwrap_or(Ordering::Equal)
            .then_with(|| {
                right
                    .ideal_fit_ppg
                    .partial_cmp(&left.ideal_fit_ppg)
                    .unwrap_or(Ordering::Equal)
            })
    });

    let mut mechanically_overpowered_watchlist = fit_sensitivity_summary
        .iter()
        .filter(|row| row.classification == "mechanically_overpowered")
        .cloned()
        .collect::<Vec<_>>();
    mechanically_overpowered_watchlist.sort_by(|left, right| {
        right
            .ideal_fit_ppg
            .partial_cmp(&left.ideal_fit_ppg)
            .unwrap_or(Ordering::Equal)
            .then_with(|| {
                left.ideal_minus_bad_ppg
                    .partial_cmp(&right.ideal_minus_bad_ppg)
                    .unwrap_or(Ordering::Equal)
            })
    });

    Output {
        matches_per_leg,
        notes: vec![
            "Each system is benchmarked with bad-fit, neutral-fit, and ideal-fit squads to measure how dependent the setup is on tailored players.".to_string(),
            "Bad-fit squads intentionally misalign role attributes and traits with the chosen style; ideal-fit squads receive the full style-aligned role/trait package.".to_string(),
            "Counter-response tables show how an ideal-fit system performs against bad-fit, neutral-fit, and ideal-fit versions of every opponent system.".to_string(),
            "A mechanically overpowered system is one that posts a high ideal-fit PPG without needing a large jump from its bad-fit baseline.".to_string(),
        ],
        fit_sensitivity_summary,
        player_dependent_watchlist: player_dependent_watchlist.into_iter().take(12).collect(),
        mechanically_overpowered_watchlist: mechanically_overpowered_watchlist.into_iter().take(12).collect(),
        systems: systems_output,
    }
}

fn build_profile_result(
    _system: &SystemDef,
    system_index: usize,
    fit: FitProfile,
    systems: &[SystemDef],
    variants: &[VariantDef],
    overall: &HashMap<&'static str, Aggregate>,
    versus: &HashMap<(&'static str, &'static str), Aggregate>,
    role_stats: &HashMap<&'static str, HashMap<String, RoleAggregate>>,
) -> ProfileResult {
    let variant = find_variant(variants, system_index, fit);
    let mut matchups = variants
        .iter()
        .filter(|opponent| opponent.id != variant.id)
        .map(|opponent| {
            let opponent_system = &systems[opponent.system_index];
            MatchupView {
                opponent_id: opponent.id.to_string(),
                opponent_system_id: opponent_system.id.to_string(),
                opponent_formation: opponent_system.formation.to_string(),
                opponent_style: opponent_system.style_label.to_string(),
                opponent_fit: opponent.fit.label().to_string(),
                summary: to_view(
                    *versus
                        .get(&(variant.id, opponent.id))
                        .unwrap_or(&Aggregate::default()),
                ),
            }
        })
        .collect::<Vec<_>>();
    matchups.sort_by(|left, right| compare_aggregate_views(&right.summary, &left.summary));

    let role_fingerprints = role_stats
        .get(variant.id)
        .map(build_role_fingerprints)
        .unwrap_or_default();

    ProfileResult {
        fit_profile: fit.label().to_string(),
        overall: to_view(*overall.get(variant.id).unwrap_or(&Aggregate::default())),
        role_fingerprints,
        strongest_against: matchups.iter().take(5).cloned().collect(),
        weakest_against: matchups.iter().rev().take(5).cloned().collect(),
    }
}

fn build_counter_responses(
    system: &SystemDef,
    system_index: usize,
    systems: &[SystemDef],
    variants: &[VariantDef],
    versus: &HashMap<(&'static str, &'static str), Aggregate>,
) -> Vec<CounterResponseView> {
    let ideal = find_variant(variants, system_index, FitProfile::IdealFit);
    let mut responses = Vec::new();

    for (opponent_index, opponent) in systems
        .iter()
        .enumerate()
        .filter(|(_, opponent)| opponent.id != system.id)
    {
        let bad = find_variant(variants, opponent_index, FitProfile::BadFit);
        let neutral = find_variant(variants, opponent_index, FitProfile::NeutralFit);
        let ideal_opp = find_variant(variants, opponent_index, FitProfile::IdealFit);
        responses.push(CounterResponseView {
            opponent_system_id: opponent.id.to_string(),
            opponent_formation: opponent.formation.to_string(),
            opponent_style: opponent.style_label.to_string(),
            bad_fit: to_view(
                *versus
                    .get(&(ideal.id, bad.id))
                    .unwrap_or(&Aggregate::default()),
            ),
            neutral_fit: to_view(
                *versus
                    .get(&(ideal.id, neutral.id))
                    .unwrap_or(&Aggregate::default()),
            ),
            ideal_fit: to_view(
                *versus
                    .get(&(ideal.id, ideal_opp.id))
                    .unwrap_or(&Aggregate::default()),
            ),
        });
    }

    responses
}

fn build_fit_curve(
    bad_fit: &AggregateView,
    neutral_fit: &AggregateView,
    ideal_fit: &AggregateView,
) -> FitCurveView {
    let ideal_minus_bad = ideal_fit.points_per_match - bad_fit.points_per_match;
    let ideal_minus_neutral = ideal_fit.points_per_match - neutral_fit.points_per_match;
    let neutral_minus_bad = neutral_fit.points_per_match - bad_fit.points_per_match;
    let classification = classify_system(
        ideal_fit.points_per_match,
        bad_fit.points_per_match,
        ideal_minus_bad,
        ideal_minus_neutral,
    );

    FitCurveView {
        classification: classification.to_string(),
        bad_fit: bad_fit.clone(),
        neutral_fit: neutral_fit.clone(),
        ideal_fit: ideal_fit.clone(),
        ideal_minus_bad_ppg: ideal_minus_bad,
        ideal_minus_neutral_ppg: ideal_minus_neutral,
        neutral_minus_bad_ppg: neutral_minus_bad,
    }
}

fn classify_system(
    ideal_ppg: f64,
    bad_ppg: f64,
    fit_gap: f64,
    ideal_minus_neutral: f64,
) -> &'static str {
    if ideal_ppg >= 1.75 && fit_gap <= 0.22 {
        "mechanically_overpowered"
    } else if ideal_ppg >= 1.50 && fit_gap >= 0.35 {
        "player_dependent"
    } else if ideal_ppg < 1.25 && fit_gap >= 0.30 {
        "fit_limited"
    } else if ideal_minus_neutral.abs() < 0.10 && (ideal_ppg - bad_ppg).abs() < 0.18 {
        "flat_curve"
    } else {
        "mixed"
    }
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

fn find_variant<'a>(
    variants: &'a [VariantDef],
    system_index: usize,
    fit: FitProfile,
) -> &'a VariantDef {
    variants
        .iter()
        .find(|variant| variant.system_index == system_index && variant.fit == fit)
        .expect("variant for system")
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

fn system_defs() -> Vec<SystemDef> {
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
                let id = Box::leak(format!("{}__{}", formation, style_label).into_boxed_str());
                SystemDef {
                    id,
                    formation,
                    style: *style,
                    style_label,
                    target_package: package_for_style(*style),
                    roles: default_roles_for_formation(formation),
                }
            })
        })
        .collect()
}

fn variant_defs(systems: &[SystemDef]) -> Vec<VariantDef> {
    let mut variants = Vec::new();
    for (system_index, system) in systems.iter().enumerate() {
        for fit in [
            FitProfile::BadFit,
            FitProfile::NeutralFit,
            FitProfile::IdealFit,
        ] {
            let id = Box::leak(format!("{}__{}", system.id, fit.label()).into_boxed_str());
            variants.push(VariantDef {
                id,
                system_index,
                fit,
            });
        }
    }
    variants
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

fn make_team(id: &str, name: &str, system: &SystemDef, fit: FitProfile) -> TeamData {
    let players = system
        .roles
        .iter()
        .enumerate()
        .map(|(index, role)| make_role_player(id, index, *role, system.style, fit))
        .collect::<Vec<_>>();

    TeamData {
        id: id.to_string(),
        name: name.to_string(),
        formation: system.formation.to_string(),
        play_style: system.style,
        players,
    }
}

fn make_role_player(
    team_id: &str,
    index: usize,
    role: TacticalRole,
    style: PlayStyle,
    fit: FitProfile,
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

    apply_baseline_package(&mut player);
    match fit {
        FitProfile::BadFit => apply_bad_fit_profile(&mut player, style),
        FitProfile::NeutralFit => apply_neutral_fit_profile(&mut player, style),
        FitProfile::IdealFit => apply_ideal_fit_profile(&mut player, style),
    }

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

fn apply_neutral_fit_profile(player: &mut PlayerData, style: PlayStyle) {
    match style {
        PlayStyle::Balanced => {
            shift(&mut player.decisions, 2);
            shift(&mut player.composure, 2);
            shift(&mut player.teamwork, 2);
        }
        PlayStyle::Attacking => match player.role {
            TacticalRole::Poacher
            | TacticalRole::TargetForward
            | TacticalRole::ChannelRunner
            | TacticalRole::WideProgressor
            | TacticalRole::AdvancedPlaymaker => {
                tune(player, 2, 0, 0, 2, 0, 3, 0, 2, 0, 3, 0, 1);
                add_traits(player, &["ChaosCreator"]);
            }
            _ => {}
        },
        PlayStyle::Defensive => match player.role {
            TacticalRole::CenterBackStopper
            | TacticalRole::CenterBackCover
            | TacticalRole::FullBackSupport
            | TacticalRole::HoldingMidfielder => {
                tune(player, 0, 2, 2, 0, 0, 0, 3, 0, 3, 3, 0, 2);
                add_traits(player, &["ContainmentSpecialist"]);
            }
            TacticalRole::Goalkeeper | TacticalRole::SweeperKeeper => {
                shift(&mut player.handling, 2);
                shift(&mut player.reflexes, 2);
            }
            _ => {}
        },
        PlayStyle::Possession => match player.role {
            TacticalRole::CenterBackPlaymaker
            | TacticalRole::DeepPlaymaker
            | TacticalRole::AdvancedPlaymaker
            | TacticalRole::HoldingMidfielder
            | TacticalRole::LinkForward => {
                tune(player, 0, 0, 0, 0, 4, 0, 0, 0, 0, 1, 4, 4);
                shift(&mut player.composure, 3);
                shift(&mut player.teamwork, 3);
                add_traits(player, &["Playmaker", "TeamPlayer"]);
            }
            _ => {}
        },
        PlayStyle::Counter => match player.role {
            TacticalRole::Poacher
            | TacticalRole::TargetForward
            | TacticalRole::ChannelRunner
            | TacticalRole::WideProgressor
            | TacticalRole::WingBackAttack => {
                tune(player, 4, 0, 0, 2, 0, 2, 0, 4, 0, 3, 0, 2);
                add_traits(player, &["TransitionAnticipator"]);
            }
            TacticalRole::DeepPlaymaker
            | TacticalRole::CenterBackPlaymaker
            | TacticalRole::Goalkeeper
            | TacticalRole::SweeperKeeper => {
                tune(player, 0, 0, 0, 0, 3, 0, 0, 0, 0, 1, 3, 3);
                add_traits(player, &["EarlyScanner"]);
            }
            _ => {}
        },
        PlayStyle::HighPress => match player.role {
            TacticalRole::HoldingMidfielder
            | TacticalRole::BoxToBoxMidfielder
            | TacticalRole::ChannelRunner
            | TacticalRole::Poacher
            | TacticalRole::CenterBackCover => {
                tune(player, 3, 4, 0, 1, 0, 0, 3, 0, 1, 2, 0, 2);
                shift(&mut player.aggression, 4);
                shift(&mut player.teamwork, 2);
                add_traits(player, &["Engine"]);
            }
            _ => {}
        },
    }
}

fn apply_ideal_fit_profile(player: &mut PlayerData, style: PlayStyle) {
    apply_neutral_fit_profile(player, style);
    match style {
        PlayStyle::Balanced => {
            shift(&mut player.decisions, 2);
            shift(&mut player.teamwork, 2);
            add_traits(player, &["TeamPlayer"]);
        }
        other => apply_package(player, package_for_style(other)),
    }
}

fn apply_bad_fit_profile(player: &mut PlayerData, style: PlayStyle) {
    match style {
        PlayStyle::Balanced => {
            tune(player, -2, -4, 0, -2, 0, 2, -2, 2, -2, -4, 0, -6);
            shift(&mut player.composure, -6);
            shift(&mut player.teamwork, -8);
            add_traits(player, &["ChaosCreator"]);
        }
        PlayStyle::Attacking => match player.role {
            TacticalRole::Poacher
            | TacticalRole::TargetForward
            | TacticalRole::ChannelRunner
            | TacticalRole::WideProgressor
            | TacticalRole::AdvancedPlaymaker => {
                tune(player, -6, -2, 0, -4, 2, -8, 0, -6, 0, -6, 2, -6);
                shift(&mut player.composure, -4);
                add_traits(player, &["TeamPlayer"]);
            }
            _ => tune(player, 0, 0, 0, 0, 2, -2, 0, 0, 0, 0, 2, -2),
        },
        PlayStyle::Defensive => match player.role {
            TacticalRole::CenterBackStopper
            | TacticalRole::CenterBackCover
            | TacticalRole::FullBackSupport
            | TacticalRole::HoldingMidfielder
            | TacticalRole::WingBackAttack => {
                tune(player, 2, -2, -2, 2, 0, 2, -8, 2, -8, -8, 0, -6);
                shift(&mut player.aggression, -4);
            }
            TacticalRole::Goalkeeper | TacticalRole::SweeperKeeper => {
                shift(&mut player.handling, -5);
                shift(&mut player.positioning, -5);
            }
            _ => tune(player, 2, 0, 0, 2, 0, 2, -2, 2, -2, -2, 0, -2),
        },
        PlayStyle::Possession => match player.role {
            TacticalRole::CenterBackPlaymaker
            | TacticalRole::DeepPlaymaker
            | TacticalRole::AdvancedPlaymaker
            | TacticalRole::HoldingMidfielder
            | TacticalRole::LinkForward
            | TacticalRole::WideProgressor => {
                tune(player, 2, 0, 0, 2, -8, 2, 0, 2, 0, -6, -8, -8);
                shift(&mut player.composure, -8);
                shift(&mut player.teamwork, -8);
                add_traits(player, &["ChaosCreator"]);
            }
            _ => tune(player, 2, 0, 0, 2, -4, 2, 0, 2, 0, -2, -4, -4),
        },
        PlayStyle::Counter => match player.role {
            TacticalRole::Poacher
            | TacticalRole::TargetForward
            | TacticalRole::ChannelRunner
            | TacticalRole::WideProgressor
            | TacticalRole::WingBackAttack => {
                tune(player, -10, -2, 0, -6, 2, -4, 0, -4, 0, -8, 0, -6);
                shift(&mut player.composure, -4);
                add_traits(player, &["TeamPlayer"]);
            }
            TacticalRole::DeepPlaymaker
            | TacticalRole::CenterBackPlaymaker
            | TacticalRole::Goalkeeper
            | TacticalRole::SweeperKeeper => {
                tune(player, -2, 0, 0, 0, -6, 0, 0, 0, 0, -2, -6, -6);
                shift(&mut player.handling, -4);
            }
            _ => tune(player, -4, 0, 0, -2, -2, 0, 0, -2, 0, -2, -2, -2),
        },
        PlayStyle::HighPress => match player.role {
            TacticalRole::HoldingMidfielder
            | TacticalRole::BoxToBoxMidfielder
            | TacticalRole::ChannelRunner
            | TacticalRole::Poacher
            | TacticalRole::CenterBackCover
            | TacticalRole::CenterBackPlaymaker => {
                tune(player, -8, -10, 0, -4, 0, 0, -8, -2, -4, -4, 0, -6);
                shift(&mut player.aggression, -10);
                shift(&mut player.teamwork, -6);
            }
            _ => tune(player, -4, -6, 0, -2, 0, 0, -4, 0, -2, -2, 0, -2),
        },
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
            tune(player, 0, 0, 0, 0, 8, 0, 0, 0, 0, 4, 8, 6);
            add_traits(player, &["EarlyScanner", "PressBaiter", "Playmaker"]);
        }
        TacticalRole::DeepPlaymaker | TacticalRole::AdvancedPlaymaker => {
            tune(player, 0, 0, 0, 0, 10, 0, 0, 2, 0, 2, 10, 8);
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
            tune(player, 0, 0, 0, 0, 6, 0, 0, 0, 0, 4, 6, 6);
            add_traits(player, &["TeamPlayer", "SpaceMagnet", "RiskCalibrator"]);
        }
        TacticalRole::Goalkeeper | TacticalRole::SweeperKeeper => {
            tune(player, 0, 0, 0, 0, 4, 0, 0, 0, 0, 0, 4, 4);
            add_traits(player, &["SafeHands", "TrafficCommander"]);
        }
        _ => add_traits(player, &["TeamPlayer"]),
    }
}

fn apply_launch_counter_package(player: &mut PlayerData) {
    match player.role {
        TacticalRole::Goalkeeper | TacticalRole::SweeperKeeper => {
            player.role = TacticalRole::SweeperKeeper;
            tune(player, 4, 0, 0, 2, 10, 0, 0, 0, 0, 0, 8, 8);
            shift(&mut player.handling, 6);
            shift(&mut player.reflexes, 4);
            add_traits(player, &["ThrowLauncher", "SafeHands", "CatReflexes"]);
        }
        TacticalRole::CenterBackPlaymaker | TacticalRole::DeepPlaymaker => {
            tune(player, 0, 0, 0, 0, 6, 0, 0, 0, 0, 2, 8, 6);
            add_traits(
                player,
                &["EarlyScanner", "PressBaiter", "TransitionAnticipator"],
            );
        }
        TacticalRole::WingBackAttack
        | TacticalRole::WideProgressor
        | TacticalRole::ChannelRunner => {
            tune(player, 6, 2, 0, 4, 0, 2, 0, 6, 0, 4, 0, 2);
            add_traits(
                player,
                &["TransitionAnticipator", "BlindSideRunner", "ChannelDrifter"],
            );
        }
        TacticalRole::Poacher | TacticalRole::TargetForward => {
            tune(player, 4, 0, 2, 2, 0, 6, 0, 2, 0, 4, 0, 2);
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
            tune(player, 4, 0, 0, 6, 0, 6, 0, 8, 0, 4, 0, 0);
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
            tune(player, 0, 2, 0, 2, 2, 4, 0, 2, 0, 6, 2, 2);
            add_traits(player, &["LateBoxArriver", "FarPostGhost", "ChaosCreator"]);
        }
        TacticalRole::TargetForward => {
            tune(player, 0, 0, 4, 0, 0, 6, 0, 0, 0, 4, 0, 0);
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
            tune(player, 0, 2, 2, 0, 0, 0, 8, 0, 6, 8, 4, 4);
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
            tune(player, 2, 2, 0, 0, 0, 0, 4, 0, 4, 6, 2, 4);
            add_traits(player, &["ContainmentSpecialist", "RecoverySprinter"]);
        }
        TacticalRole::Goalkeeper | TacticalRole::SweeperKeeper => {
            shift(&mut player.handling, 6);
            shift(&mut player.positioning, 4);
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
            tune(player, 4, 8, 0, 2, 0, 0, 6, 0, 2, 4, 2, 6);
            add_traits(
                player,
                &["Engine", "Tireless", "TransitionAnticipator", "BallWinner"],
            );
        }
        TacticalRole::Poacher | TacticalRole::LinkForward => {
            tune(player, 4, 4, 0, 2, 0, 2, 4, 2, 0, 2, 0, 2);
            add_traits(player, &["KeeperDisruptor", "TransitionAnticipator"]);
        }
        TacticalRole::CenterBackCover | TacticalRole::CenterBackPlaymaker => {
            tune(player, 2, 2, 0, 0, 2, 0, 4, 0, 4, 4, 4, 2);
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

fn tune(
    player: &mut PlayerData,
    pace: i16,
    stamina: i16,
    strength: i16,
    agility: i16,
    passing: i16,
    shooting: i16,
    tackling: i16,
    dribbling: i16,
    defending: i16,
    positioning: i16,
    vision: i16,
    decisions: i16,
) {
    shift(&mut player.pace, pace);
    shift(&mut player.stamina, stamina);
    shift(&mut player.strength, strength);
    shift(&mut player.agility, agility);
    shift(&mut player.passing, passing);
    shift(&mut player.shooting, shooting);
    shift(&mut player.tackling, tackling);
    shift(&mut player.dribbling, dribbling);
    shift(&mut player.defending, defending);
    shift(&mut player.positioning, positioning);
    shift(&mut player.vision, vision);
    shift(&mut player.decisions, decisions);
}

fn shift(value: &mut u8, delta: i16) {
    let shifted = i16::from(*value) + delta;
    *value = shifted.clamp(20, 99) as u8;
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
