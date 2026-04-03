use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Player {
    pub id: String,
    pub match_name: String,
    pub full_name: String,
    pub date_of_birth: String,
    pub nationality: String,

    pub position: Position,

    // The player's natural/preferred position (never changed by formation logic)
    #[serde(default)]
    pub natural_position: Position,

    // Alternate positions this player can also play (with reduced effectiveness)
    #[serde(default)]
    pub alternate_positions: Vec<Position>,

    #[serde(default)]
    pub footedness: Footedness,

    #[serde(default = "default_weak_foot")]
    pub weak_foot: u8,

    // Core attributes 0-100
    pub attributes: PlayerAttributes,

    // Dynamic match/season values
    pub condition: u8, // 0-100 (short-term energy; depletes during matches, recovers daily)
    pub morale: u8,    // 0-100
    /// Long-term physical shape (0–100). Determines how fast condition depletes and
    /// recovers, and modulates injury risk. Changes slowly over weeks.
    #[serde(default = "default_fitness")]
    pub fitness: u8,

    pub injury: Option<Injury>,
    pub team_id: Option<String>,

    // Traits / flairs derived from attributes
    #[serde(default)]
    pub traits: Vec<PlayerTrait>,

    // Contract & value
    pub contract_end: Option<String>,
    pub wage: u32, // weekly wage
    pub market_value: u64,

    // Season stats
    pub stats: PlayerSeasonStats,

    // Per-match stats history (used for deeper aggregates in the player profile)
    #[serde(default)]
    pub match_stats: Vec<PlayerMatchStatsEntry>,

    // Career history
    pub career: Vec<CareerEntry>,

    // Individual training focus override (takes priority over group and team default)
    #[serde(default)]
    pub training_focus: Option<crate::team::TrainingFocus>,

    // Transfer status
    #[serde(default)]
    pub transfer_listed: bool,
    #[serde(default)]
    pub loan_listed: bool,
    #[serde(default)]
    pub transfer_offers: Vec<TransferOffer>,
    #[serde(default)]
    pub morale_core: PlayerMoraleCore,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
pub enum Position {
    #[default]
    Goalkeeper,
    Defender,
    Midfielder,
    Forward,
    RightBack,
    CenterBack,
    LeftBack,
    RightWingBack,
    LeftWingBack,
    DefensiveMidfielder,
    CentralMidfielder,
    AttackingMidfielder,
    RightMidfielder,
    LeftMidfielder,
    RightWinger,
    LeftWinger,
    Striker,
}

impl Position {
    pub fn is_legacy_bucket(&self) -> bool {
        matches!(
            self,
            Position::Goalkeeper | Position::Defender | Position::Midfielder | Position::Forward
        )
    }

    pub fn to_group_position(&self) -> Position {
        match self {
            Position::Goalkeeper => Position::Goalkeeper,
            Position::Defender
            | Position::RightBack
            | Position::CenterBack
            | Position::LeftBack
            | Position::RightWingBack
            | Position::LeftWingBack => Position::Defender,
            Position::Midfielder
            | Position::DefensiveMidfielder
            | Position::CentralMidfielder
            | Position::AttackingMidfielder
            | Position::RightMidfielder
            | Position::LeftMidfielder => Position::Midfielder,
            Position::Forward
            | Position::RightWinger
            | Position::LeftWinger
            | Position::Striker => Position::Forward,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum Footedness {
    Left,
    #[default]
    Right,
    Both,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerAttributes {
    // Physical
    pub pace: u8,
    pub stamina: u8,
    pub strength: u8,
    #[serde(default = "default_attr")]
    pub agility: u8,

    // Technical
    pub passing: u8,
    pub shooting: u8,
    pub tackling: u8,
    pub dribbling: u8,
    pub defending: u8,

    // Mental
    pub positioning: u8,
    pub vision: u8,
    pub decisions: u8,
    #[serde(default = "default_attr")]
    pub composure: u8,
    #[serde(default = "default_attr")]
    pub aggression: u8,
    #[serde(default = "default_attr")]
    pub teamwork: u8,
    #[serde(default = "default_attr")]
    pub leadership: u8,

    // Goalkeeper
    #[serde(default = "default_attr")]
    pub handling: u8,
    #[serde(default = "default_attr")]
    pub reflexes: u8,
    #[serde(default = "default_attr")]
    pub aerial: u8,
}

fn default_attr() -> u8 {
    50
}

fn default_weak_foot() -> u8 {
    2
}

fn default_fitness() -> u8 {
    75
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Injury {
    pub name: String,
    pub days_remaining: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum PlayerIssueCategory {
    Contract,
    PlayingTime,
    Morale,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct PlayerIssue {
    pub category: PlayerIssueCategory,
    pub severity: u8,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(default)]
pub struct RecentTreatmentMemory {
    pub action_key: String,
    pub times_recently_used: u8,
}

impl Default for RecentTreatmentMemory {
    fn default() -> Self {
        Self {
            action_key: String::new(),
            times_recently_used: 0,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum PlayerPromiseKind {
    PlayingTime,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum RenewalSessionStatus {
    #[default]
    Idle,
    Open,
    Agreed,
    Blocked,
    Stalled,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum RenewalSessionOutcome {
    #[default]
    None,
    AcceptedByManager,
    AcceptedByAssistant,
    RejectedByPlayer,
    BlockedByManager,
    Stalled,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(default)]
pub struct ContractRenewalState {
    pub status: RenewalSessionStatus,
    pub manager_blocked_until: Option<String>,
    pub last_attempt_date: Option<String>,
    pub last_assistant_attempt_date: Option<String>,
    pub last_outcome: Option<RenewalSessionOutcome>,
    pub conversation_round: u8,
}

impl Default for ContractRenewalState {
    fn default() -> Self {
        Self {
            status: RenewalSessionStatus::Idle,
            manager_blocked_until: None,
            last_attempt_date: None,
            last_assistant_attempt_date: None,
            last_outcome: None,
            conversation_round: 0,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(default)]
pub struct PlayerPromise {
    pub kind: PlayerPromiseKind,
    pub matches_remaining: u8,
}

impl Default for PlayerPromise {
    fn default() -> Self {
        Self {
            kind: PlayerPromiseKind::PlayingTime,
            matches_remaining: 0,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(default)]
pub struct PlayerMoraleCore {
    pub manager_trust: u8,
    pub unresolved_issue: Option<PlayerIssue>,
    pub recent_treatment: Option<RecentTreatmentMemory>,
    pub pending_promise: Option<PlayerPromise>,
    pub talk_cooldown_until: Option<String>,
    pub renewal_state: Option<ContractRenewalState>,
}

impl Default for PlayerMoraleCore {
    fn default() -> Self {
        Self {
            manager_trust: 50,
            unresolved_issue: None,
            recent_treatment: None,
            pending_promise: None,
            talk_cooldown_until: None,
            renewal_state: None,
        }
    }
}

fn default_transfer_offer_status() -> TransferOfferStatus {
    TransferOfferStatus::Pending
}

fn default_transfer_offer_date() -> String {
    String::new()
}

fn default_transfer_offer_round() -> u8 {
    0
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct PlayerSeasonStats {
    pub appearances: u32,
    pub goals: u32,
    pub assists: u32,
    pub clean_sheets: u32,
    pub yellow_cards: u32,
    pub red_cards: u32,
    pub avg_rating: f32,
    pub minutes_played: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerMatchStatsEntry {
    pub fixture_id: String,
    pub season: u32,
    pub matchday: u32,
    pub date: String,
    pub team_id: Option<String>,
    pub opponent_team_id: Option<String>,
    pub was_home: bool,
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
    pub rating: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CareerEntry {
    pub season: u32,
    pub team_id: String,
    pub team_name: String,
    pub appearances: u32,
    pub goals: u32,
    pub assists: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransferOffer {
    pub id: String,
    pub from_team_id: String,
    pub fee: u64,
    pub wage_offered: u32,
    #[serde(default)]
    pub last_manager_fee: Option<u64>,
    #[serde(default = "default_transfer_offer_round")]
    pub negotiation_round: u8,
    #[serde(default)]
    pub suggested_counter_fee: Option<u64>,
    #[serde(default = "default_transfer_offer_status")]
    pub status: TransferOfferStatus,
    #[serde(default = "default_transfer_offer_date")]
    pub date: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum TransferOfferStatus {
    Pending,
    Accepted,
    Rejected,
    Withdrawn,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PlayerTrait {
    // Physical
    Speedster, // pace >= 85
    Tank,      // strength >= 85 && stamina >= 75
    Agile,     // agility >= 85
    Tireless,  // stamina >= 90
    // Technical
    Playmaker,    // passing >= 80 && vision >= 80
    Sharpshooter, // shooting >= 85
    Dribbler,     // dribbling >= 85
    BallWinner,   // tackling >= 80 && aggression >= 70
    Rock,         // defending >= 85 && positioning >= 75
    // Mental
    Leader,     // leadership >= 85 && teamwork >= 75
    CoolHead,   // composure >= 85 && decisions >= 80
    Visionary,  // vision >= 85
    HotHead,    // aggression >= 85 && composure < 50
    TeamPlayer, // teamwork >= 85
    // Goalkeeper
    SafeHands,       // handling >= 85 (GK only)
    CatReflexes,     // reflexes >= 85 (GK only)
    AerialDominance, // aerial >= 85
    // Combo / Special
    CompleteForward, // FWD: shooting >= 75 && dribbling >= 75 && pace >= 70 && strength >= 70
    Engine,          // MID: stamina >= 85 && pace >= 70 && teamwork >= 75
    SetPieceSpecialist, // passing >= 80 && shooting >= 75 && vision >= 75
    // Perceptual / cognitive
    EarlyScanner,
    BlindSideAwareness,
    TempoManipulator,
    DelayedPasser,
    RiskCalibrator,
    SpaceMagnet,
    PressBaiter,
    TransitionAnticipator,
    // Technical micro-traits
    OneTouchSpecialist,
    ToePokeFinisher,
    OutsideFootPasser,
    DisguisedFirstTouch,
    BounceTechnician,
    AerialRedirection,
    HalfVolleyComfort,
    RecoveryTouch,
    // Movement / off-ball
    LateBoxArriver,
    NearPostHunter,
    BlindSideRunner,
    DecoyMover,
    SecondBallPredator,
    StaticLure,
    ChannelDrifter,
    FarPostGhost,
    ReboundInstinct,
    // Defensive / duel
    ContainmentSpecialist,
    RecoverySprinter,
    PassingLaneThief,
    BodyAngleManipulator,
    TacticalFouler,
    AerialGrappler,
    SecondContactWinner,
    // Psychological / competitive
    BigMatchRiser,
    BigMatchShrinker,
    MomentumPlayer,
    ErrorImmunity,
    CrowdReactive,
    Provocable,
    RefereeManipulator,
    PainMasker,
    StatusSensitive,
    ClutchExecutor,
    // Dark arts / matchcraft
    ContactSeller,
    ShieldAddict,
    LineStepTrapper,
    QuickRestartOpportunist,
    TimeKiller,
    ChaosCreator,
    NutmegOpportunist,
    BounceRoomDribbler,
    KeeperDisruptor,
    // Goalkeeper-specific
    ReboundDirector,
    CrossPoker,
    BreakawayHypnotist,
    LineDictator,
    ThrowLauncher,
    PenaltyReader,
    TrafficCommander,
}

/// Derive engine-facing traits from a player's attributes and natural role context.
pub fn compute_traits(attrs: &PlayerAttributes, position: &Position) -> Vec<PlayerTrait> {
    let mut traits = Vec::new();
    let grouped_position = position.to_group_position();
    let is_midfielder = grouped_position == Position::Midfielder;
    let is_forward = grouped_position == Position::Forward;

    // Physical
    if attrs.pace >= 85 {
        traits.push(PlayerTrait::Speedster);
    }
    if attrs.strength >= 85 && attrs.stamina >= 75 {
        traits.push(PlayerTrait::Tank);
    }
    if attrs.agility >= 85 {
        traits.push(PlayerTrait::Agile);
    }
    if attrs.stamina >= 90 {
        traits.push(PlayerTrait::Tireless);
    }

    // Technical
    if attrs.passing >= 80 && attrs.vision >= 80 {
        traits.push(PlayerTrait::Playmaker);
    }
    if attrs.shooting >= 85 {
        traits.push(PlayerTrait::Sharpshooter);
    }
    if attrs.dribbling >= 85 {
        traits.push(PlayerTrait::Dribbler);
    }
    if attrs.tackling >= 80 && attrs.aggression >= 70 {
        traits.push(PlayerTrait::BallWinner);
    }
    if attrs.defending >= 85 && attrs.positioning >= 75 {
        traits.push(PlayerTrait::Rock);
    }

    // Mental
    if attrs.leadership >= 85 && attrs.teamwork >= 75 {
        traits.push(PlayerTrait::Leader);
    }
    if attrs.composure >= 85 && attrs.decisions >= 80 {
        traits.push(PlayerTrait::CoolHead);
    }
    if attrs.vision >= 85 {
        traits.push(PlayerTrait::Visionary);
    }
    if attrs.aggression >= 85 && attrs.composure < 50 {
        traits.push(PlayerTrait::HotHead);
    }
    if attrs.teamwork >= 85 {
        traits.push(PlayerTrait::TeamPlayer);
    }

    // Goalkeeper-oriented (any player with high GK stats can earn these)
    if attrs.handling >= 85 {
        traits.push(PlayerTrait::SafeHands);
    }
    if attrs.reflexes >= 85 {
        traits.push(PlayerTrait::CatReflexes);
    }
    if attrs.aerial >= 85 {
        traits.push(PlayerTrait::AerialDominance);
    }

    // Combo / Special — purely attribute-based
    if attrs.shooting >= 75 && attrs.dribbling >= 75 && attrs.pace >= 70 && attrs.strength >= 70 {
        traits.push(PlayerTrait::CompleteForward);
    }
    if attrs.stamina >= 85 && attrs.pace >= 70 && attrs.teamwork >= 75 {
        traits.push(PlayerTrait::Engine);
    }
    if attrs.passing >= 80 && attrs.shooting >= 75 && attrs.vision >= 75 {
        traits.push(PlayerTrait::SetPieceSpecialist);
    }

    // Perceptual / cognitive
    if attrs.vision >= 82 && attrs.decisions >= 80 && attrs.composure >= 75 {
        traits.push(PlayerTrait::EarlyScanner);
    }
    if attrs.positioning >= 84 && attrs.decisions >= 80 {
        traits.push(PlayerTrait::BlindSideAwareness);
    }
    if attrs.passing >= 82 && attrs.decisions >= 82 && attrs.composure >= 78 {
        traits.push(PlayerTrait::TempoManipulator);
    }
    if attrs.passing >= 80 && attrs.vision >= 84 && attrs.composure >= 80 {
        traits.push(PlayerTrait::DelayedPasser);
    }
    if attrs.decisions >= 85 && attrs.composure >= 78 && attrs.vision >= 75 {
        traits.push(PlayerTrait::RiskCalibrator);
    }
    if attrs.positioning >= 84 && attrs.vision >= 78 && attrs.agility >= 72 {
        traits.push(PlayerTrait::SpaceMagnet);
    }
    if attrs.composure >= 82 && attrs.passing >= 76 && attrs.strength >= 68 {
        traits.push(PlayerTrait::PressBaiter);
    }
    if attrs.positioning >= 80 && attrs.pace >= 78 && attrs.decisions >= 76 {
        traits.push(PlayerTrait::TransitionAnticipator);
    }

    // Technical micro-traits
    if attrs.passing >= 80 && attrs.vision >= 78 && attrs.composure >= 74 {
        traits.push(PlayerTrait::OneTouchSpecialist);
    }
    if attrs.shooting >= 82 && attrs.agility >= 70 && attrs.composure >= 70 {
        traits.push(PlayerTrait::ToePokeFinisher);
    }
    if attrs.passing >= 83 && attrs.dribbling >= 74 {
        traits.push(PlayerTrait::OutsideFootPasser);
    }
    if attrs.dribbling >= 84 && attrs.agility >= 80 && attrs.composure >= 75 {
        traits.push(PlayerTrait::DisguisedFirstTouch);
    }
    if attrs.agility >= 78 && attrs.composure >= 72 && attrs.strength >= 68 {
        traits.push(PlayerTrait::BounceTechnician);
    }
    if attrs.aerial >= 80 && attrs.positioning >= 76 && attrs.shooting >= 68 {
        traits.push(PlayerTrait::AerialRedirection);
    }
    if attrs.shooting >= 82 && attrs.agility >= 72 && attrs.composure >= 74 {
        traits.push(PlayerTrait::HalfVolleyComfort);
    }
    if attrs.dribbling >= 78 && attrs.agility >= 76 && attrs.composure >= 70 {
        traits.push(PlayerTrait::RecoveryTouch);
    }

    // Movement / off-ball
    if is_midfielder && attrs.positioning >= 80 && attrs.stamina >= 76 && attrs.shooting >= 68 {
        traits.push(PlayerTrait::LateBoxArriver);
    }
    if is_forward && attrs.positioning >= 80 && attrs.pace >= 76 {
        traits.push(PlayerTrait::NearPostHunter);
    }
    if is_forward && attrs.pace >= 82 && attrs.positioning >= 80 {
        traits.push(PlayerTrait::BlindSideRunner);
    }
    if attrs.teamwork >= 82 && attrs.positioning >= 78 && attrs.decisions >= 76 {
        traits.push(PlayerTrait::DecoyMover);
    }
    if attrs.positioning >= 80 && attrs.aggression >= 72 && attrs.strength >= 70 {
        traits.push(PlayerTrait::SecondBallPredator);
    }
    if attrs.composure >= 80 && attrs.positioning >= 82 && attrs.decisions >= 78 {
        traits.push(PlayerTrait::StaticLure);
    }
    if attrs.dribbling >= 80 && attrs.positioning >= 76 && attrs.pace >= 76 {
        traits.push(PlayerTrait::ChannelDrifter);
    }
    if is_forward && attrs.positioning >= 82 && attrs.composure >= 74 {
        traits.push(PlayerTrait::FarPostGhost);
    }
    if (is_forward || is_midfielder)
        && attrs.positioning >= 80
        && attrs.shooting >= 74
        && attrs.aggression >= 68
    {
        traits.push(PlayerTrait::ReboundInstinct);
    }

    // Defensive / duel
    if attrs.defending >= 78 && attrs.positioning >= 82 && attrs.decisions >= 76 {
        traits.push(PlayerTrait::ContainmentSpecialist);
    }
    if attrs.pace >= 82 && attrs.defending >= 72 && attrs.composure >= 72 {
        traits.push(PlayerTrait::RecoverySprinter);
    }
    if attrs.tackling >= 76 && attrs.vision >= 78 && attrs.decisions >= 78 {
        traits.push(PlayerTrait::PassingLaneThief);
    }
    if attrs.defending >= 76 && attrs.positioning >= 80 && attrs.composure >= 74 {
        traits.push(PlayerTrait::BodyAngleManipulator);
    }
    if attrs.aggression >= 78 && attrs.tackling >= 75 && attrs.decisions >= 72 {
        traits.push(PlayerTrait::TacticalFouler);
    }
    if attrs.aerial >= 82 && attrs.strength >= 78 {
        traits.push(PlayerTrait::AerialGrappler);
    }
    if attrs.strength >= 74 && attrs.aggression >= 72 && attrs.agility >= 68 {
        traits.push(PlayerTrait::SecondContactWinner);
    }

    // Psychological / competitive
    if attrs.leadership >= 82 && attrs.composure >= 80 {
        traits.push(PlayerTrait::BigMatchRiser);
    }
    if attrs.composure <= 42 && attrs.decisions <= 48 {
        traits.push(PlayerTrait::BigMatchShrinker);
    }
    if attrs.aggression >= 76 && (45..=70).contains(&attrs.composure) {
        traits.push(PlayerTrait::MomentumPlayer);
    }
    if attrs.composure >= 84 && attrs.decisions >= 80 {
        traits.push(PlayerTrait::ErrorImmunity);
    }
    if attrs.leadership <= 45 && attrs.aggression >= 70 {
        traits.push(PlayerTrait::CrowdReactive);
    }
    if attrs.aggression >= 82 && attrs.composure <= 58 {
        traits.push(PlayerTrait::Provocable);
    }
    if attrs.decisions >= 80 && attrs.composure >= 78 && (45..=75).contains(&attrs.aggression) {
        traits.push(PlayerTrait::RefereeManipulator);
    }
    if attrs.stamina >= 84 && attrs.strength >= 76 && attrs.composure >= 72 {
        traits.push(PlayerTrait::PainMasker);
    }
    if attrs.leadership >= 82 && attrs.teamwork >= 70 {
        traits.push(PlayerTrait::StatusSensitive);
    }
    if attrs.composure >= 86
        && attrs.decisions >= 82
        && (attrs.shooting >= 74 || attrs.passing >= 78)
    {
        traits.push(PlayerTrait::ClutchExecutor);
    }

    // Dark arts / matchcraft
    if attrs.agility >= 78 && attrs.dribbling >= 74 && attrs.composure >= 72 {
        traits.push(PlayerTrait::ContactSeller);
    }
    if attrs.strength >= 80 && attrs.composure >= 74 && attrs.teamwork >= 68 {
        traits.push(PlayerTrait::ShieldAddict);
    }
    if attrs.positioning >= 82 && attrs.decisions >= 80 {
        traits.push(PlayerTrait::LineStepTrapper);
    }
    if attrs.decisions >= 82 && attrs.pace >= 74 && attrs.vision >= 72 {
        traits.push(PlayerTrait::QuickRestartOpportunist);
    }
    if attrs.composure >= 82 && attrs.teamwork >= 70 && attrs.strength >= 68 {
        traits.push(PlayerTrait::TimeKiller);
    }
    if attrs.dribbling >= 80 && attrs.aggression >= 76 && attrs.decisions >= 70 {
        traits.push(PlayerTrait::ChaosCreator);
    }
    if attrs.dribbling >= 84 && attrs.agility >= 80 && attrs.composure >= 70 {
        traits.push(PlayerTrait::NutmegOpportunist);
    }
    if attrs.dribbling >= 80 && attrs.agility >= 76 && attrs.strength >= 68 {
        traits.push(PlayerTrait::BounceRoomDribbler);
    }
    if is_forward && attrs.strength >= 74 && attrs.positioning >= 78 && attrs.aggression >= 68 {
        traits.push(PlayerTrait::KeeperDisruptor);
    }

    // Goalkeeper-specific
    if attrs.handling >= 82 && attrs.reflexes >= 78 && attrs.decisions >= 78 {
        traits.push(PlayerTrait::ReboundDirector);
    }
    if attrs.aerial >= 82 && attrs.strength >= 74 && attrs.aggression >= 68 {
        traits.push(PlayerTrait::CrossPoker);
    }
    if attrs.reflexes >= 84 && attrs.composure >= 76 && attrs.positioning >= 80 {
        traits.push(PlayerTrait::BreakawayHypnotist);
    }
    if attrs.leadership >= 80 && attrs.positioning >= 82 && attrs.decisions >= 80 {
        traits.push(PlayerTrait::LineDictator);
    }
    if attrs.passing >= 78 && attrs.vision >= 76 && attrs.decisions >= 74 {
        traits.push(PlayerTrait::ThrowLauncher);
    }
    if attrs.reflexes >= 82 && attrs.decisions >= 80 && attrs.composure >= 74 {
        traits.push(PlayerTrait::PenaltyReader);
    }
    if attrs.aerial >= 80 && attrs.positioning >= 80 && attrs.leadership >= 76 {
        traits.push(PlayerTrait::TrafficCommander);
    }

    traits
}

impl Player {
    pub fn new(
        id: String,
        match_name: String,
        full_name: String,
        date_of_birth: String,
        nationality: String,
        position: Position,
        attributes: PlayerAttributes,
    ) -> Self {
        let traits = compute_traits(&attributes, &position);
        Self {
            id,
            match_name,
            full_name,
            date_of_birth,
            nationality,
            natural_position: position.clone(),
            position,
            alternate_positions: Vec::new(),
            footedness: Footedness::default(),
            weak_foot: default_weak_foot(),
            attributes,
            condition: 100,
            morale: 100,
            fitness: 75,
            injury: None,
            team_id: None,
            traits,
            contract_end: None,
            wage: 0,
            market_value: 0,
            stats: PlayerSeasonStats::default(),
            match_stats: Vec::new(),
            career: Vec::new(),
            training_focus: None,
            transfer_listed: false,
            loan_listed: false,
            transfer_offers: Vec::new(),
            morale_core: PlayerMoraleCore::default(),
        }
    }

    pub fn repair_missing_traits(&mut self) -> bool {
        if !self.traits.is_empty() {
            return false;
        }
        let position = if self.natural_position.is_legacy_bucket() {
            self.position.clone()
        } else {
            self.natural_position.clone()
        };
        self.traits = compute_traits(&self.attributes, &position);
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_attributes() -> PlayerAttributes {
        PlayerAttributes {
            pace: 70,
            stamina: 72,
            strength: 65,
            agility: 68,
            passing: 74,
            shooting: 61,
            tackling: 58,
            dribbling: 69,
            defending: 56,
            positioning: 67,
            vision: 73,
            decisions: 71,
            composure: 66,
            aggression: 54,
            teamwork: 76,
            leadership: 49,
            handling: 20,
            reflexes: 24,
            aerial: 44,
        }
    }

    #[test]
    fn player_new_defaults_footedness_and_weak_foot() {
        let player = Player::new(
            "p-001".to_string(),
            "J. Smith".to_string(),
            "John Smith".to_string(),
            "2000-01-15".to_string(),
            "GB".to_string(),
            Position::Midfielder,
            sample_attributes(),
        );

        assert_eq!(player.footedness, Footedness::Right);
        assert_eq!(player.weak_foot, 2);
    }

    #[test]
    fn position_group_conversion_maps_granular_positions_back_to_legacy_groups() {
        assert_eq!(Position::RightBack.to_group_position(), Position::Defender);
        assert_eq!(
            Position::AttackingMidfielder.to_group_position(),
            Position::Midfielder,
        );
        assert_eq!(Position::LeftWinger.to_group_position(), Position::Forward);
    }

    #[test]
    fn player_deserialization_defaults_missing_foot_fields() {
        let player: Player = serde_json::from_value(serde_json::json!({
            "id": "p-legacy",
            "match_name": "J. Legacy",
            "full_name": "John Legacy",
            "date_of_birth": "2000-01-15",
            "nationality": "GB",
            "position": "Midfielder",
            "natural_position": "Midfielder",
            "alternate_positions": [],
            "attributes": sample_attributes(),
            "condition": 100,
            "morale": 100,
            "injury": null,
            "team_id": null,
            "traits": [],
            "contract_end": null,
            "wage": 0,
            "market_value": 0,
            "stats": {},
            "career": [],
            "transfer_listed": false,
            "loan_listed": false,
            "transfer_offers": [],
            "morale_core": {}
        }))
        .expect("legacy player json should deserialize");

        assert_eq!(player.footedness, Footedness::Right);
        assert_eq!(player.weak_foot, 2);
        assert_eq!(player.natural_position, Position::Midfielder);
    }

    #[test]
    fn compute_traits_assigns_new_cognitive_and_duel_traits() {
        let mut attrs = sample_attributes();
        attrs.passing = 90;
        attrs.vision = 92;
        attrs.decisions = 85;
        attrs.composure = 84;
        attrs.positioning = 86;
        attrs.tackling = 82;
        attrs.aggression = 79;
        attrs.defending = 80;
        attrs.aerial = 84;
        attrs.strength = 80;

        let traits = compute_traits(&attrs, &Position::CentralMidfielder);

        assert!(traits.contains(&PlayerTrait::EarlyScanner));
        assert!(traits.contains(&PlayerTrait::RiskCalibrator));
        assert!(traits.contains(&PlayerTrait::OneTouchSpecialist));
        assert!(traits.contains(&PlayerTrait::PassingLaneThief));
        assert!(traits.contains(&PlayerTrait::AerialGrappler));
    }
}
