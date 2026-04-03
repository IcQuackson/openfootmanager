use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Team {
    pub id: String,
    pub name: String,
    pub short_name: String,
    pub country: String,
    pub city: String,
    pub stadium_name: String,
    pub stadium_capacity: u32,

    // Current state
    pub finance: i64,
    pub manager_id: Option<String>,
    pub reputation: u32,

    // Financial breakdown
    pub wage_budget: i64,
    pub transfer_budget: i64,
    pub season_income: i64,
    pub season_expenses: i64,
    #[serde(default)]
    pub financial_ledger: Vec<FinancialTransaction>,
    #[serde(default)]
    pub sponsorship: Option<Sponsorship>,
    #[serde(default)]
    pub facilities: Facilities,

    // Tactical
    pub formation: String,
    pub play_style: PlayStyle,

    // Training
    #[serde(default)]
    pub training_focus: TrainingFocus,
    #[serde(default)]
    pub training_intensity: TrainingIntensity,
    #[serde(default)]
    pub training_schedule: TrainingSchedule,

    // Club info
    pub founded_year: u32,
    pub colors: TeamColors,

    // Training groups: allow per-group focus overrides for subsets of players
    #[serde(default)]
    pub training_groups: Vec<TrainingGroup>,

    // Persistent starting XI (player IDs). If empty, auto-select by OVR.
    #[serde(default)]
    pub starting_xi_ids: Vec<String>,

    #[serde(default)]
    pub match_roles: MatchRoles,

    #[serde(default)]
    pub tactical_roles: Vec<TacticalRole>,

    // Recent form: last 5 results as "W", "D", "L" (most recent last)
    #[serde(default)]
    pub form: Vec<String>,

    // History
    pub history: Vec<TeamSeasonRecord>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct MatchRoles {
    pub captain: Option<String>,
    pub vice_captain: Option<String>,
    pub penalty_taker: Option<String>,
    pub free_kick_taker: Option<String>,
    pub corner_taker: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TacticalRole {
    Goalkeeper,
    SweeperKeeper,
    CenterBackStopper,
    CenterBackCover,
    CenterBackPlaymaker,
    FullBackSupport,
    WingBackAttack,
    HoldingMidfielder,
    DeepPlaymaker,
    BoxToBoxMidfielder,
    AdvancedPlaymaker,
    WideProgressor,
    Poacher,
    TargetForward,
    ChannelRunner,
    LinkForward,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub enum TrainingFocus {
    #[default]
    Physical,
    Technical,
    Tactical,
    Defending,
    Attacking,
    Recovery,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub enum TrainingIntensity {
    Low,
    #[default]
    Medium,
    High,
}

/// Weekly training schedule controlling how many days per week are training vs rest.
/// Rest days give full condition recovery with no training cost.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub enum TrainingSchedule {
    /// 6 training days, 1 rest (Sunday). Max growth, minimal recovery.
    Intense,
    /// 4 training days (Mon, Tue, Thu, Fri), 3 rest (Wed, Sat, Sun). Good balance.
    #[default]
    Balanced,
    /// 2 training days (Tue, Thu), 5 rest. Minimal growth, excellent recovery.
    Light,
}

impl TrainingSchedule {
    /// Returns true if the given weekday (chrono::Weekday) is a training day.
    /// Mon=0, Tue=1, Wed=2, Thu=3, Fri=4, Sat=5, Sun=6
    pub fn is_training_day(&self, weekday_num: u32) -> bool {
        match self {
            // Intense: rest only on Sunday (6)
            TrainingSchedule::Intense => weekday_num != 6,
            // Balanced: train Mon(0), Tue(1), Thu(3), Fri(4); rest Wed(2), Sat(5), Sun(6)
            TrainingSchedule::Balanced => matches!(weekday_num, 0 | 1 | 3 | 4),
            // Light: train Tue(1), Thu(3) only
            TrainingSchedule::Light => matches!(weekday_num, 1 | 3),
        }
    }

    /// Human-readable description of training days per week.
    pub fn training_days_per_week(&self) -> u8 {
        match self {
            TrainingSchedule::Intense => 6,
            TrainingSchedule::Balanced => 4,
            TrainingSchedule::Light => 2,
        }
    }
}

/// A named training group with its own focus. Players in a group train
/// with the group's focus instead of the team-wide default.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TrainingGroup {
    pub id: String,
    pub name: String,
    pub focus: TrainingFocus,
    pub player_ids: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TeamColors {
    pub primary: String,
    pub secondary: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum PlayStyle {
    Balanced,
    Attacking,
    Defensive,
    Possession,
    Counter,
    HighPress,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TeamSeasonRecord {
    pub season: u32,
    pub league_position: u32,
    pub played: u32,
    pub won: u32,
    pub drawn: u32,
    pub lost: u32,
    pub goals_for: u32,
    pub goals_against: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum FinancialTransactionKind {
    PrizeMoney,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct FinancialTransaction {
    pub date: String,
    pub description: String,
    pub amount: i64,
    pub kind: FinancialTransactionKind,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum SponsorshipBonusCriterion {
    LeaguePosition {
        max_position: u32,
        bonus_amount: i64,
    },
    UnbeatenRun {
        required_matches: usize,
        bonus_amount: i64,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(default)]
pub struct Sponsorship {
    pub sponsor_name: String,
    pub base_value: i64,
    pub remaining_weeks: u32,
    pub bonus_criteria: Vec<SponsorshipBonusCriterion>,
}

impl Default for Sponsorship {
    fn default() -> Self {
        Self {
            sponsor_name: String::new(),
            base_value: 0,
            remaining_weeks: 0,
            bonus_criteria: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum FacilityType {
    Training,
    Medical,
    Scouting,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(default)]
pub struct Facilities {
    pub training: u8,
    pub medical: u8,
    pub scouting: u8,
}

impl Default for Facilities {
    fn default() -> Self {
        Self {
            training: 1,
            medical: 1,
            scouting: 1,
        }
    }
}

impl Team {
    pub fn new(
        id: String,
        name: String,
        short_name: String,
        country: String,
        city: String,
        stadium_name: String,
        stadium_capacity: u32,
    ) -> Self {
        Self {
            id,
            name,
            short_name,
            country,
            city,
            stadium_name,
            stadium_capacity,
            finance: 1_000_000,
            manager_id: None,
            reputation: 500,
            wage_budget: 200_000,
            transfer_budget: 500_000,
            season_income: 0,
            season_expenses: 0,
            financial_ledger: Vec::new(),
            sponsorship: None,
            facilities: Facilities::default(),
            formation: "4-4-2".to_string(),
            play_style: PlayStyle::Balanced,
            training_focus: TrainingFocus::default(),
            training_intensity: TrainingIntensity::default(),
            training_schedule: TrainingSchedule::default(),
            training_groups: Vec::new(),
            founded_year: 1900,
            colors: TeamColors {
                primary: "#10b981".to_string(),
                secondary: "#ffffff".to_string(),
            },
            starting_xi_ids: Vec::new(),
            match_roles: MatchRoles::default(),
            tactical_roles: default_tactical_roles_for_formation("4-4-2"),
            form: Vec::new(),
            history: Vec::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_roles_follow_formation_slot_count() {
        let roles = default_tactical_roles_for_formation("4-2-3-1");
        assert_eq!(roles.len(), 11);
        assert_eq!(roles[0], TacticalRole::Goalkeeper);
        assert_eq!(roles[1], TacticalRole::FullBackSupport);
        assert_eq!(roles[2], TacticalRole::CenterBackPlaymaker);
        assert_eq!(roles[5], TacticalRole::HoldingMidfielder);
        assert_eq!(roles[6], TacticalRole::DeepPlaymaker);
        assert_eq!(roles[7], TacticalRole::WideProgressor);
        assert_eq!(roles[8], TacticalRole::AdvancedPlaymaker);
        assert_eq!(roles[10], TacticalRole::LinkForward);
    }
}

pub fn default_tactical_roles_for_formation(formation: &str) -> Vec<TacticalRole> {
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
    let parts: Vec<usize> = formation
        .split('-')
        .filter_map(|part| part.parse::<usize>().ok())
        .collect();

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
