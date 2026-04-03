use engine::{
    MatchConfig, PlayStyle, PlayerData, Position, TacticalRole, TeamData, simulate_with_rng,
};
use rand::SeedableRng;
use rand::rngs::StdRng;
use serde::Serialize;
use std::env;

const DEFAULT_MATCHES: u32 = 5_000;
const DEFAULT_SKILL: u8 = 65;

#[derive(Debug, Serialize)]
struct BenchmarkScenario {
    matches: u32,
    home_style: &'static str,
    away_style: &'static str,
    home_skill: u8,
    away_skill: u8,
    formation: &'static str,
    seed_start: u64,
}

#[derive(Debug, Default, Serialize)]
struct BenchmarkTotals {
    total_goals: u32,
    home_goals: u32,
    away_goals: u32,
    home_shots: u32,
    away_shots: u32,
    defender_pass_attempts: u32,
    defender_pass_completions: u32,
    midfielder_pass_attempts: u32,
    forward_pass_attempts: u32,
}

#[derive(Debug, Serialize)]
struct BenchmarkAverages {
    avg_total_goals: f64,
    avg_home_goals: f64,
    avg_away_goals: f64,
    avg_home_shots: f64,
    avg_away_shots: f64,
    avg_home_possession: f64,
    avg_defender_pass_attempts: f64,
    avg_defender_pass_completions: f64,
    avg_midfielder_pass_attempts: f64,
    avg_forward_pass_attempts: f64,
}

#[derive(Debug, Serialize)]
struct BenchmarkResult {
    scenario: BenchmarkScenario,
    totals: BenchmarkTotals,
    averages: BenchmarkAverages,
}

fn main() {
    let matches = parse_matches(env::args().skip(1).collect());
    let result = run_benchmark(matches);
    println!(
        "{}",
        serde_json::to_string_pretty(&result).expect("serialize benchmark result"),
    );
}

fn parse_matches(args: Vec<String>) -> u32 {
    let mut matches = DEFAULT_MATCHES;
    let mut index = 0usize;

    while index < args.len() {
        match args[index].as_str() {
            "--matches" => {
                let Some(raw_value) = args.get(index + 1) else {
                    eprintln!("Missing value for --matches");
                    std::process::exit(1);
                };
                matches = raw_value.parse::<u32>().unwrap_or_else(|_| {
                    eprintln!("Invalid value for --matches: {}", raw_value);
                    std::process::exit(1);
                });
                index += 2;
            }
            "--help" | "-h" => {
                println!(
                    "Usage: cargo run -p engine --example balanced_benchmark --release -- [--matches N]"
                );
                std::process::exit(0);
            }
            unknown => {
                eprintln!("Unknown argument: {}", unknown);
                std::process::exit(1);
            }
        }
    }

    matches
}

fn run_benchmark(matches: u32) -> BenchmarkResult {
    let home = make_team("home", "Home FC", DEFAULT_SKILL, PlayStyle::Balanced);
    let away = make_team("away", "Away FC", DEFAULT_SKILL, PlayStyle::Balanced);
    let config = MatchConfig::default();

    let mut totals = BenchmarkTotals::default();
    let mut home_possession_sum = 0.0f64;

    for seed in 0..matches {
        let mut rng = StdRng::seed_from_u64(seed as u64);
        let report = simulate_with_rng(&home, &away, &config, &mut rng);

        totals.total_goals += u32::from(report.home_goals) + u32::from(report.away_goals);
        totals.home_goals += u32::from(report.home_goals);
        totals.away_goals += u32::from(report.away_goals);
        totals.home_shots += u32::from(report.home_stats.shots);
        totals.away_shots += u32::from(report.away_stats.shots);
        home_possession_sum += report.home_possession;

        for player in home.players.iter().chain(away.players.iter()) {
            let Some(stats) = report.player_stats.get(&player.id) else {
                continue;
            };

            match player.position {
                Position::Goalkeeper => {}
                Position::Defender => {
                    totals.defender_pass_attempts += u32::from(stats.passes_attempted);
                    totals.defender_pass_completions += u32::from(stats.passes_completed);
                }
                Position::Midfielder => {
                    totals.midfielder_pass_attempts += u32::from(stats.passes_attempted);
                }
                Position::Forward => {
                    totals.forward_pass_attempts += u32::from(stats.passes_attempted);
                }
            }
        }
    }

    let matches_f64 = f64::from(matches.max(1));
    BenchmarkResult {
        scenario: BenchmarkScenario {
            matches,
            home_style: "Balanced",
            away_style: "Balanced",
            home_skill: DEFAULT_SKILL,
            away_skill: DEFAULT_SKILL,
            formation: "4-4-2",
            seed_start: 0,
        },
        averages: BenchmarkAverages {
            avg_total_goals: totals.total_goals as f64 / matches_f64,
            avg_home_goals: totals.home_goals as f64 / matches_f64,
            avg_away_goals: totals.away_goals as f64 / matches_f64,
            avg_home_shots: totals.home_shots as f64 / matches_f64,
            avg_away_shots: totals.away_shots as f64 / matches_f64,
            avg_home_possession: home_possession_sum / matches_f64,
            avg_defender_pass_attempts: totals.defender_pass_attempts as f64 / matches_f64,
            avg_defender_pass_completions: totals.defender_pass_completions as f64 / matches_f64,
            avg_midfielder_pass_attempts: totals.midfielder_pass_attempts as f64 / matches_f64,
            avg_forward_pass_attempts: totals.forward_pass_attempts as f64 / matches_f64,
        },
        totals,
    }
}

fn make_team(id: &str, name: &str, skill: u8, play_style: PlayStyle) -> TeamData {
    TeamData {
        id: id.to_string(),
        name: name.to_string(),
        formation: "4-4-2".to_string(),
        play_style,
        players: vec![
            make_player(&format!("{id}_gk1"), "GK1", Position::Goalkeeper, skill),
            make_player(&format!("{id}_def1"), "DEF1", Position::Defender, skill),
            make_player(&format!("{id}_def2"), "DEF2", Position::Defender, skill),
            make_player(&format!("{id}_def3"), "DEF3", Position::Defender, skill),
            make_player(&format!("{id}_def4"), "DEF4", Position::Defender, skill),
            make_player(&format!("{id}_mid1"), "MID1", Position::Midfielder, skill),
            make_player(&format!("{id}_mid2"), "MID2", Position::Midfielder, skill),
            make_player(&format!("{id}_mid3"), "MID3", Position::Midfielder, skill),
            make_player(&format!("{id}_mid4"), "MID4", Position::Midfielder, skill),
            make_player(&format!("{id}_fwd1"), "FWD1", Position::Forward, skill),
            make_player(&format!("{id}_fwd2"), "FWD2", Position::Forward, skill),
        ],
    }
}

fn make_player(id: &str, name: &str, position: Position, skill: u8) -> PlayerData {
    PlayerData {
        id: id.to_string(),
        name: name.to_string(),
        position,
        condition: 90,
        fitness: 75,
        pace: skill,
        stamina: skill,
        strength: skill,
        agility: skill,
        passing: skill,
        shooting: skill,
        tackling: skill,
        dribbling: skill,
        defending: skill,
        positioning: skill,
        vision: skill,
        decisions: skill,
        composure: skill,
        aggression: skill,
        teamwork: skill,
        leadership: skill,
        handling: skill,
        reflexes: skill,
        aerial: skill,
        traits: vec![],
        role: TacticalRole::default_for_position(position),
    }
}
