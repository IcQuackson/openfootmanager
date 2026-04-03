use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

use crate::event::{EventType, MatchEvent};
use crate::types::Side;

// ---------------------------------------------------------------------------
// TeamStats — aggregate stats for one side
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TeamStats {
    pub goals: u8,
    pub shots: u16,
    pub shots_on_target: u16,
    pub shots_off_target: u16,
    pub shots_blocked: u16,
    pub passes_completed: u16,
    pub passes_intercepted: u16,
    pub tackles: u16,
    pub interceptions: u16,
    pub fouls: u16,
    pub corners: u16,
    pub free_kicks: u16,
    pub penalties: u16,
    pub yellow_cards: u8,
    pub red_cards: u8,
    pub possession_ticks: u32,
}

impl TeamStats {
    pub fn pass_accuracy(&self) -> f64 {
        let total = self.passes_completed as f64 + self.passes_intercepted as f64;
        if total == 0.0 {
            return 0.0;
        }
        self.passes_completed as f64 / total * 100.0
    }
}

// ---------------------------------------------------------------------------
// PlayerMatchStats — individual player performance
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PlayerMatchStats {
    pub minutes_played: u8,
    pub goals: u8,
    pub assists: u8,
    pub shots: u8,
    pub shots_on_target: u8,
    pub passes_completed: u8,
    pub passes_attempted: u8,
    pub tackles_won: u8,
    pub interceptions: u8,
    pub fouls_committed: u8,
    pub yellow_cards: u8,
    pub red_cards: u8,
    /// Match rating 0.0–10.0, computed after the match.
    pub rating: f32,
}

// ---------------------------------------------------------------------------
// GoalDetail — enriched goal info for the report
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GoalDetail {
    pub minute: u8,
    pub scorer_id: String,
    pub assist_id: Option<String>,
    pub is_penalty: bool,
    pub side: Side,
}

// ---------------------------------------------------------------------------
// MatchReport — the complete output of a simulated match
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MatchReport {
    pub home_goals: u8,
    pub away_goals: u8,
    pub home_stats: TeamStats,
    pub away_stats: TeamStats,
    pub events: Vec<MatchEvent>,
    pub goals: Vec<GoalDetail>,
    pub player_stats: HashMap<String, PlayerMatchStats>,
    /// Possession percentage for the home team (0–100).
    pub home_possession: f64,
    /// Total simulated minutes (90 + stoppage).
    pub total_minutes: u8,
}

impl MatchReport {
    /// Build the report from the raw event log and possession counters.
    pub fn from_events(
        events: Vec<MatchEvent>,
        home_possession_ticks: u32,
        away_possession_ticks: u32,
        total_minutes: u8,
        home_player_ids: &[String],
        away_player_ids: &[String],
    ) -> Self {
        let mut home_stats = TeamStats::default();
        let mut away_stats = TeamStats::default();
        let mut goals = Vec::new();
        let mut player_stats: HashMap<String, PlayerMatchStats> = HashMap::new();

        home_stats.possession_ticks = home_possession_ticks;
        away_stats.possession_ticks = away_possession_ticks;

        for event in &events {
            let stats = match event.side {
                Side::Home => &mut home_stats,
                Side::Away => &mut away_stats,
            };

            // Update player stats helper
            let pid = event.player_id.as_deref().unwrap_or("");

            match &event.event_type {
                EventType::Goal => {
                    stats.goals += 1;
                    stats.shots += 1;
                    stats.shots_on_target += 1;
                    goals.push(GoalDetail {
                        minute: event.minute,
                        scorer_id: pid.to_string(),
                        assist_id: event.secondary_player_id.clone(),
                        is_penalty: false,
                        side: event.side,
                    });
                    if !pid.is_empty() {
                        let ps = player_stats.entry(pid.to_string()).or_default();
                        ps.goals += 1;
                        ps.shots += 1;
                        ps.shots_on_target += 1;
                    }
                    if let Some(ref assist_id) = event.secondary_player_id {
                        let ps = player_stats.entry(assist_id.clone()).or_default();
                        ps.assists += 1;
                    }
                }
                EventType::PenaltyGoal => {
                    stats.goals += 1;
                    stats.shots += 1;
                    stats.shots_on_target += 1;
                    stats.penalties += 1;
                    goals.push(GoalDetail {
                        minute: event.minute,
                        scorer_id: pid.to_string(),
                        assist_id: None,
                        is_penalty: true,
                        side: event.side,
                    });
                    if !pid.is_empty() {
                        let ps = player_stats.entry(pid.to_string()).or_default();
                        ps.goals += 1;
                        ps.shots += 1;
                        ps.shots_on_target += 1;
                    }
                }
                EventType::PenaltyMiss => {
                    stats.shots += 1;
                    stats.penalties += 1;
                    if !pid.is_empty() {
                        let ps = player_stats.entry(pid.to_string()).or_default();
                        ps.shots += 1;
                    }
                }
                EventType::ShotOnTarget | EventType::ShotSaved => {
                    stats.shots += 1;
                    stats.shots_on_target += 1;
                    if !pid.is_empty() {
                        let ps = player_stats.entry(pid.to_string()).or_default();
                        ps.shots += 1;
                        ps.shots_on_target += 1;
                    }
                }
                EventType::ShotOffTarget => {
                    stats.shots += 1;
                    stats.shots_off_target += 1;
                    if !pid.is_empty() {
                        let ps = player_stats.entry(pid.to_string()).or_default();
                        ps.shots += 1;
                    }
                }
                EventType::ShotBlocked => {
                    stats.shots += 1;
                    stats.shots_blocked += 1;
                    if !pid.is_empty() {
                        let ps = player_stats.entry(pid.to_string()).or_default();
                        ps.shots += 1;
                    }
                }
                EventType::PassCompleted => {
                    stats.passes_completed += 1;
                    if !pid.is_empty() {
                        let ps = player_stats.entry(pid.to_string()).or_default();
                        ps.passes_completed += 1;
                        ps.passes_attempted += 1;
                    }
                }
                EventType::PassIntercepted => {
                    stats.passes_intercepted += 1;
                    if !pid.is_empty() {
                        let ps = player_stats.entry(pid.to_string()).or_default();
                        ps.passes_attempted += 1;
                    }
                }
                EventType::Tackle => {
                    stats.tackles += 1;
                    if !pid.is_empty() {
                        let ps = player_stats.entry(pid.to_string()).or_default();
                        ps.tackles_won += 1;
                    }
                }
                EventType::Interception => {
                    stats.interceptions += 1;
                    if !pid.is_empty() {
                        let ps = player_stats.entry(pid.to_string()).or_default();
                        ps.interceptions += 1;
                    }
                }
                EventType::Foul => {
                    stats.fouls += 1;
                    if !pid.is_empty() {
                        let ps = player_stats.entry(pid.to_string()).or_default();
                        ps.fouls_committed += 1;
                    }
                }
                EventType::YellowCard | EventType::SecondYellow => {
                    stats.yellow_cards += 1;
                    if !pid.is_empty() {
                        let ps = player_stats.entry(pid.to_string()).or_default();
                        ps.yellow_cards += 1;
                    }
                }
                EventType::RedCard => {
                    stats.red_cards += 1;
                    if !pid.is_empty() {
                        let ps = player_stats.entry(pid.to_string()).or_default();
                        ps.red_cards += 1;
                    }
                }
                EventType::Corner => {
                    stats.corners += 1;
                }
                EventType::FreeKick => {
                    stats.free_kicks += 1;
                }
                EventType::PenaltyAwarded => {
                    stats.penalties += 1;
                }
                _ => {}
            }
        }

        populate_minutes_played(
            &events,
            home_player_ids,
            away_player_ids,
            total_minutes,
            &mut player_stats,
        );
        populate_ratings(&mut player_stats, total_minutes);

        let total_poss = home_possession_ticks + away_possession_ticks;
        let home_possession = if total_poss > 0 {
            home_possession_ticks as f64 / total_poss as f64 * 100.0
        } else {
            50.0
        };

        Self {
            home_goals: home_stats.goals,
            away_goals: away_stats.goals,
            home_stats,
            away_stats,
            events,
            goals,
            player_stats,
            home_possession,
            total_minutes,
        }
    }
}

fn populate_minutes_played(
    events: &[MatchEvent],
    home_player_ids: &[String],
    away_player_ids: &[String],
    total_minutes: u8,
    player_stats: &mut HashMap<String, PlayerMatchStats>,
) {
    populate_minutes_for_side(
        events,
        Side::Home,
        home_player_ids,
        total_minutes,
        player_stats,
    );
    populate_minutes_for_side(
        events,
        Side::Away,
        away_player_ids,
        total_minutes,
        player_stats,
    );
}

fn populate_minutes_for_side(
    events: &[MatchEvent],
    side: Side,
    final_player_ids: &[String],
    total_minutes: u8,
    player_stats: &mut HashMap<String, PlayerMatchStats>,
) {
    let mut subbed_on_ids = HashSet::new();
    let mut subbed_off_ids = HashSet::new();
    let mut sent_off_ids = HashSet::new();
    let mut timeline: Vec<(u8, TimelineEvent)> = Vec::new();

    for event in events.iter().filter(|event| event.side == side) {
        match event.event_type {
            EventType::Substitution => {
                if let (Some(player_on_id), Some(player_off_id)) =
                    (event.player_id.as_ref(), event.secondary_player_id.as_ref())
                {
                    subbed_on_ids.insert(player_on_id.clone());
                    subbed_off_ids.insert(player_off_id.clone());
                    timeline.push((
                        event.minute,
                        TimelineEvent::Substitution {
                            player_off_id: player_off_id.clone(),
                            player_on_id: player_on_id.clone(),
                        },
                    ));
                }
            }
            EventType::RedCard | EventType::SecondYellow => {
                if let Some(player_id) = event.player_id.as_ref() {
                    sent_off_ids.insert(player_id.clone());
                    timeline.push((event.minute, TimelineEvent::Dismissal(player_id.clone())));
                }
            }
            _ => {}
        }
    }

    let mut active_since: HashMap<String, u8> = HashMap::new();
    let final_active_ids = final_player_ids
        .iter()
        .filter(|player_id| !sent_off_ids.contains(player_id.as_str()))
        .cloned()
        .collect::<HashSet<_>>();

    for player_id in final_active_ids {
        if !subbed_on_ids.contains(player_id.as_str()) {
            active_since.insert(player_id, 0);
        }
    }

    for player_id in subbed_off_ids.iter().chain(sent_off_ids.iter()) {
        if !subbed_on_ids.contains(player_id.as_str()) {
            active_since.entry(player_id.clone()).or_insert(0);
        }
    }

    timeline.sort_by_key(|(minute, event)| {
        let rank = match event {
            TimelineEvent::Dismissal(_) => 0,
            TimelineEvent::Substitution { .. } => 1,
        };
        (*minute, rank)
    });

    for (minute, event) in timeline {
        match event {
            TimelineEvent::Dismissal(player_id) => {
                close_appearance(player_stats, &mut active_since, &player_id, minute);
            }
            TimelineEvent::Substitution {
                player_off_id,
                player_on_id,
            } => {
                close_appearance(player_stats, &mut active_since, &player_off_id, minute);
                active_since.insert(player_on_id, minute);
            }
        }
    }

    for (player_id, start_minute) in active_since {
        let minutes_played = total_minutes.saturating_sub(start_minute);
        player_stats.entry(player_id).or_default().minutes_played = minutes_played;
    }
}

fn close_appearance(
    player_stats: &mut HashMap<String, PlayerMatchStats>,
    active_since: &mut HashMap<String, u8>,
    player_id: &str,
    minute: u8,
) {
    if let Some(start_minute) = active_since.remove(player_id) {
        let minutes_played = minute.saturating_sub(start_minute);
        player_stats
            .entry(player_id.to_string())
            .or_default()
            .minutes_played = minutes_played;
    }
}

fn populate_ratings(player_stats: &mut HashMap<String, PlayerMatchStats>, total_minutes: u8) {
    let full_match_minutes = total_minutes.max(1) as f32;

    for stats in player_stats.values_mut() {
        if stats.minutes_played == 0 {
            continue;
        }

        let minutes_factor = stats.minutes_played as f32 / full_match_minutes;
        let pass_accuracy_bonus = if stats.passes_attempted > 0 {
            let accuracy = stats.passes_completed as f32 / stats.passes_attempted as f32;
            (accuracy - 0.7) * 1.5
        } else {
            0.0
        };

        let rating = 6.0
            + (minutes_factor - 0.5) * 0.4
            + stats.goals as f32 * 1.5
            + stats.assists as f32 * 1.0
            + stats.shots_on_target as f32 * 0.15
            + stats.shots as f32 * 0.05
            + stats.passes_completed as f32 * 0.01
            + pass_accuracy_bonus
            + stats.tackles_won as f32 * 0.12
            + stats.interceptions as f32 * 0.12
            - stats.fouls_committed as f32 * 0.08
            - stats.yellow_cards as f32 * 0.35
            - stats.red_cards as f32 * 1.5;

        stats.rating = rating.clamp(1.0, 10.0);
    }
}

enum TimelineEvent {
    Dismissal(String),
    Substitution {
        player_off_id: String,
        player_on_id: String,
    },
}
