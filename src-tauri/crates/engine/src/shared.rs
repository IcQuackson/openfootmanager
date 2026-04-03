use rand::Rng;

use crate::types::{MatchConfig, PlayStyle, PlayerData, Side, TacticalRole};

// ---------------------------------------------------------------------------
// PlayerSnap — lightweight snapshot of a player to avoid borrow conflicts
// ---------------------------------------------------------------------------

#[derive(Clone)]
#[allow(dead_code)]
pub(crate) struct PlayerSnap {
    pub id: String,
    pub role: TacticalRole,
    pub pace: u8,
    pub stamina: u8,
    pub strength: u8,
    pub agility: u8,
    pub passing: u8,
    pub shooting: u8,
    pub tackling: u8,
    pub dribbling: u8,
    pub defending: u8,
    pub positioning: u8,
    pub vision: u8,
    pub decisions: u8,
    pub composure: u8,
    pub aggression: u8,
    pub teamwork: u8,
    pub leadership: u8,
    pub handling: u8,
    pub reflexes: u8,
    pub aerial: u8,
    pub traits: Vec<String>,
}

impl PlayerSnap {
    pub fn from(p: &PlayerData) -> Self {
        Self {
            id: p.id.clone(),
            role: p.role,
            pace: p.pace,
            stamina: p.stamina,
            strength: p.strength,
            agility: p.agility,
            passing: p.passing,
            shooting: p.shooting,
            tackling: p.tackling,
            dribbling: p.dribbling,
            defending: p.defending,
            positioning: p.positioning,
            vision: p.vision,
            decisions: p.decisions,
            composure: p.composure,
            aggression: p.aggression,
            teamwork: p.teamwork,
            leadership: p.leadership,
            handling: p.handling,
            reflexes: p.reflexes,
            aerial: p.aerial,
            traits: p.traits.clone(),
        }
    }

    pub fn has_trait(&self, name: &str) -> bool {
        self.traits.iter().any(|t| t == name)
    }
}

fn weighted_mean(weighted_values: &[(u8, f64)]) -> f64 {
    let total_weight: f64 = weighted_values.iter().map(|(_, weight)| *weight).sum();
    if total_weight <= f64::EPSILON {
        return 0.0;
    }

    weighted_values
        .iter()
        .map(|(value, weight)| f64::from(*value) * *weight)
        .sum::<f64>()
        / total_weight
}

fn normalize_strength(score: f64, floor: f64, ceiling: f64) -> f64 {
    ((score - floor) / (ceiling - floor)).clamp(0.0, 1.0)
}

fn inverse_attr(value: u8) -> f64 {
    100.0 - f64::from(value)
}

pub(crate) fn trait_strength(snap: &PlayerSnap, trait_name: &str) -> f64 {
    if !snap.has_trait(trait_name) {
        return 0.0;
    }

    let raw = match trait_name {
        "Sharpshooter" => weighted_mean(&[
            (snap.shooting, 0.5),
            (snap.composure, 0.3),
            (snap.decisions, 0.2),
        ]),
        "CoolHead" => weighted_mean(&[(snap.composure, 0.55), (snap.decisions, 0.45)]),
        "CompleteForward" => weighted_mean(&[
            (snap.shooting, 0.3),
            (snap.dribbling, 0.2),
            (snap.pace, 0.15),
            (snap.strength, 0.15),
            (snap.positioning, 0.1),
            (snap.composure, 0.1),
        ]),
        "ToePokeFinisher" => weighted_mean(&[
            (snap.shooting, 0.5),
            (snap.agility, 0.2),
            (snap.composure, 0.3),
        ]),
        "HalfVolleyComfort" => weighted_mean(&[
            (snap.shooting, 0.45),
            (snap.composure, 0.3),
            (snap.agility, 0.25),
        ]),
        "ClutchExecutor" => weighted_mean(&[
            (snap.composure, 0.45),
            (snap.decisions, 0.35),
            (snap.shooting, 0.2),
        ]),
        "Dribbler" => weighted_mean(&[
            (snap.dribbling, 0.55),
            (snap.agility, 0.25),
            (snap.pace, 0.2),
        ]),
        "Speedster" => weighted_mean(&[(snap.pace, 0.75), (snap.agility, 0.25)]),
        "Agile" => weighted_mean(&[(snap.agility, 0.7), (snap.dribbling, 0.3)]),
        "DisguisedFirstTouch" => weighted_mean(&[
            (snap.dribbling, 0.45),
            (snap.agility, 0.3),
            (snap.composure, 0.25),
        ]),
        "RecoveryTouch" => weighted_mean(&[
            (snap.dribbling, 0.4),
            (snap.agility, 0.35),
            (snap.strength, 0.25),
        ]),
        "BounceRoomDribbler" => weighted_mean(&[
            (snap.dribbling, 0.45),
            (snap.agility, 0.3),
            (snap.strength, 0.25),
        ]),
        "Playmaker" => weighted_mean(&[
            (snap.passing, 0.4),
            (snap.vision, 0.35),
            (snap.decisions, 0.25),
        ]),
        "Visionary" => weighted_mean(&[
            (snap.vision, 0.55),
            (snap.passing, 0.2),
            (snap.decisions, 0.25),
        ]),
        "SetPieceSpecialist" => weighted_mean(&[
            (snap.passing, 0.4),
            (snap.shooting, 0.25),
            (snap.vision, 0.2),
            (snap.composure, 0.15),
        ]),
        "EarlyScanner" => weighted_mean(&[
            (snap.vision, 0.4),
            (snap.decisions, 0.35),
            (snap.composure, 0.25),
        ]),
        "OutsideFootPasser" => weighted_mean(&[
            (snap.passing, 0.55),
            (snap.dribbling, 0.2),
            (snap.vision, 0.25),
        ]),
        "OneTouchSpecialist" => weighted_mean(&[
            (snap.passing, 0.4),
            (snap.vision, 0.25),
            (snap.composure, 0.2),
            (snap.decisions, 0.15),
        ]),
        "BallWinner" => weighted_mean(&[
            (snap.tackling, 0.45),
            (snap.aggression, 0.2),
            (snap.defending, 0.2),
            (snap.stamina, 0.15),
        ]),
        "Rock" => weighted_mean(&[
            (snap.defending, 0.4),
            (snap.positioning, 0.3),
            (snap.strength, 0.2),
            (snap.tackling, 0.1),
        ]),
        "Tank" => weighted_mean(&[
            (snap.strength, 0.55),
            (snap.stamina, 0.25),
            (snap.aerial, 0.2),
        ]),
        "ContainmentSpecialist" => weighted_mean(&[
            (snap.defending, 0.35),
            (snap.positioning, 0.35),
            (snap.decisions, 0.15),
            (snap.composure, 0.15),
        ]),
        "PassingLaneThief" => weighted_mean(&[
            (snap.tackling, 0.25),
            (snap.vision, 0.3),
            (snap.decisions, 0.25),
            (snap.positioning, 0.2),
        ]),
        "AerialGrappler" => weighted_mean(&[
            (snap.aerial, 0.45),
            (snap.strength, 0.35),
            (snap.aggression, 0.2),
        ]),
        "SafeHands" => weighted_mean(&[
            (snap.handling, 0.6),
            (snap.reflexes, 0.2),
            (snap.composure, 0.2),
        ]),
        "CatReflexes" => weighted_mean(&[
            (snap.reflexes, 0.65),
            (snap.agility, 0.2),
            (snap.positioning, 0.15),
        ]),
        "AerialDominance" => weighted_mean(&[
            (snap.aerial, 0.55),
            (snap.strength, 0.25),
            (snap.positioning, 0.2),
        ]),
        "ReboundDirector" => weighted_mean(&[
            (snap.handling, 0.4),
            (snap.reflexes, 0.2),
            (snap.decisions, 0.2),
            (snap.positioning, 0.2),
        ]),
        "BreakawayHypnotist" => weighted_mean(&[
            (snap.reflexes, 0.35),
            (snap.composure, 0.25),
            (snap.positioning, 0.25),
            (snap.decisions, 0.15),
        ]),
        "TrafficCommander" => weighted_mean(&[
            (snap.aerial, 0.3),
            (snap.positioning, 0.3),
            (snap.leadership, 0.2),
            (snap.composure, 0.2),
        ]),
        "HotHead" => {
            weighted_mean(&[
                (snap.aggression, 0.7),
                (snap.tackling, 0.1),
                (snap.strength, 0.2),
            ]) + inverse_attr(snap.composure) * 0.25
        }
        "TacticalFouler" => weighted_mean(&[
            (snap.aggression, 0.3),
            (snap.tackling, 0.3),
            (snap.decisions, 0.2),
            (snap.positioning, 0.2),
        ]),
        "Provocable" => {
            weighted_mean(&[
                (snap.aggression, 0.6),
                (snap.strength, 0.1),
                (snap.decisions, 0.1),
            ]) + inverse_attr(snap.composure) * 0.4
        }
        "RefereeManipulator" => weighted_mean(&[
            (snap.decisions, 0.4),
            (snap.composure, 0.35),
            (snap.vision, 0.25),
        ]),
        "Engine" => weighted_mean(&[
            (snap.stamina, 0.45),
            (snap.pace, 0.2),
            (snap.teamwork, 0.2),
            (snap.decisions, 0.15),
        ]),
        "TeamPlayer" => weighted_mean(&[
            (snap.teamwork, 0.55),
            (snap.decisions, 0.2),
            (snap.passing, 0.1),
            (snap.stamina, 0.15),
        ]),
        "Tireless" => weighted_mean(&[
            (snap.stamina, 0.75),
            (snap.teamwork, 0.1),
            (snap.pace, 0.15),
        ]),
        "TempoManipulator" => weighted_mean(&[
            (snap.passing, 0.25),
            (snap.decisions, 0.35),
            (snap.composure, 0.2),
            (snap.vision, 0.2),
        ]),
        "RiskCalibrator" => weighted_mean(&[
            (snap.decisions, 0.45),
            (snap.composure, 0.3),
            (snap.vision, 0.25),
        ]),
        "TransitionAnticipator" => weighted_mean(&[
            (snap.positioning, 0.35),
            (snap.pace, 0.25),
            (snap.decisions, 0.25),
            (snap.stamina, 0.15),
        ]),
        "LineDictator" => weighted_mean(&[
            (snap.leadership, 0.45),
            (snap.positioning, 0.3),
            (snap.decisions, 0.25),
        ]),
        "ThrowLauncher" => weighted_mean(&[
            (snap.passing, 0.45),
            (snap.vision, 0.3),
            (snap.decisions, 0.25),
        ]),
        "RecoverySprinter" => weighted_mean(&[
            (snap.pace, 0.45),
            (snap.defending, 0.2),
            (snap.composure, 0.2),
            (snap.positioning, 0.15),
        ]),
        "BodyAngleManipulator" => weighted_mean(&[
            (snap.defending, 0.3),
            (snap.positioning, 0.3),
            (snap.composure, 0.2),
            (snap.decisions, 0.2),
        ]),
        "SecondContactWinner" => weighted_mean(&[
            (snap.strength, 0.35),
            (snap.aggression, 0.25),
            (snap.agility, 0.2),
            (snap.stamina, 0.2),
        ]),
        "SpaceMagnet" => weighted_mean(&[
            (snap.positioning, 0.35),
            (snap.vision, 0.25),
            (snap.agility, 0.2),
            (snap.decisions, 0.2),
        ]),
        "LateBoxArriver" => weighted_mean(&[
            (snap.positioning, 0.35),
            (snap.stamina, 0.25),
            (snap.shooting, 0.2),
            (snap.decisions, 0.2),
        ]),
        "BlindSideRunner" => weighted_mean(&[
            (snap.pace, 0.35),
            (snap.positioning, 0.3),
            (snap.agility, 0.15),
            (snap.decisions, 0.2),
        ]),
        "NearPostHunter" => weighted_mean(&[
            (snap.positioning, 0.4),
            (snap.pace, 0.2),
            (snap.shooting, 0.25),
            (snap.agility, 0.15),
        ]),
        "FarPostGhost" => weighted_mean(&[
            (snap.positioning, 0.4),
            (snap.composure, 0.25),
            (snap.aerial, 0.15),
            (snap.decisions, 0.2),
        ]),
        "ReboundInstinct" => weighted_mean(&[
            (snap.positioning, 0.35),
            (snap.shooting, 0.25),
            (snap.aggression, 0.2),
            (snap.decisions, 0.2),
        ]),
        "SecondBallPredator" => weighted_mean(&[
            (snap.positioning, 0.35),
            (snap.aggression, 0.25),
            (snap.agility, 0.2),
            (snap.decisions, 0.2),
        ]),
        "StaticLure" => weighted_mean(&[
            (snap.composure, 0.35),
            (snap.positioning, 0.25),
            (snap.decisions, 0.2),
            (snap.teamwork, 0.2),
        ]),
        "ChannelDrifter" => weighted_mean(&[
            (snap.dribbling, 0.3),
            (snap.positioning, 0.25),
            (snap.pace, 0.2),
            (snap.decisions, 0.25),
        ]),
        "ChaosCreator" => weighted_mean(&[
            (snap.dribbling, 0.3),
            (snap.aggression, 0.25),
            (snap.decisions, 0.2),
            (snap.vision, 0.25),
        ]),
        "KeeperDisruptor" => weighted_mean(&[
            (snap.strength, 0.3),
            (snap.positioning, 0.3),
            (snap.aggression, 0.2),
            (snap.composure, 0.2),
        ]),
        "DecoyMover" => weighted_mean(&[
            (snap.teamwork, 0.35),
            (snap.positioning, 0.25),
            (snap.decisions, 0.25),
            (snap.stamina, 0.15),
        ]),
        "PressBaiter" => weighted_mean(&[
            (snap.composure, 0.35),
            (snap.passing, 0.25),
            (snap.strength, 0.2),
            (snap.decisions, 0.2),
        ]),
        "ShieldAddict" => weighted_mean(&[
            (snap.strength, 0.35),
            (snap.composure, 0.25),
            (snap.teamwork, 0.15),
            (snap.decisions, 0.25),
        ]),
        "DelayedPasser" => weighted_mean(&[
            (snap.passing, 0.3),
            (snap.vision, 0.25),
            (snap.composure, 0.25),
            (snap.decisions, 0.2),
        ]),
        "TimeKiller" => weighted_mean(&[
            (snap.composure, 0.35),
            (snap.strength, 0.25),
            (snap.teamwork, 0.15),
            (snap.decisions, 0.25),
        ]),
        "NutmegOpportunist" => weighted_mean(&[
            (snap.dribbling, 0.45),
            (snap.agility, 0.3),
            (snap.composure, 0.25),
        ]),
        _ => weighted_mean(&[
            (snap.decisions, 0.34),
            (snap.composure, 0.33),
            (snap.positioning, 0.33),
        ]),
    };

    normalize_strength(raw, 62.0, 92.0)
}

// ---------------------------------------------------------------------------
// TraitContext — which game action context we're computing a bonus for
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Copy)]
pub(crate) enum TraitContext {
    Shooting,
    Dribbling,
    Passing,
    Tackling,
    Goalkeeping,
    Foul,
    Midfield,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ActionIntent {
    SafeRecyclePass,
    SplitLinePass,
    BaitPressTouch,
    OneTouchCombination,
    DelayedRelease,
    ProgressiveCarry,
    ThirdManLayoff,
    PressEscapeTurn,
    LinkPlaySlip,
    DirectDribble,
    BlindSideRun,
    LateBoxDelivery,
    NearPostAttack,
    StepInInterception,
    ContainAndShowWide,
    TacticalFoulStop,
    AerialClearance,
    SecureClaim,
    LaunchThrow,
}

#[derive(Debug, Clone, Copy, Default)]
pub(crate) struct IntentContext {
    pub under_pressure: bool,
    pub in_transition: bool,
    pub late_game: bool,
    pub protecting_lead: bool,
    pub chasing_game: bool,
    pub safe_outlets: f64,
    pub progression_lanes: f64,
    pub box_support: f64,
    pub rest_defense: f64,
    pub width_access: f64,
}

pub(crate) fn enrich_intent_context(
    mut context: IntentContext,
    style: PlayStyle,
    formation: &str,
) -> IntentContext {
    let style = style_profile(style);
    let formation = formation_profile(formation);

    context.safe_outlets = (style.buildup_retain * 0.45
        + formation.midfield_support * 0.30
        + formation.buildup_width * 0.15
        + formation.rest_defense * 0.10
        + if context.protecting_lead { 0.06 } else { 0.0 }
        - if context.chasing_game { 0.03 } else { 0.0 })
    .clamp(0.82, 1.24);

    context.progression_lanes = (style.midfield_control * 0.36
        + style.transition_directness * 0.14
        + formation.midfield_support * 0.28
        + formation.buildup_width * 0.10
        + formation.box_presence * 0.12
        + if context.in_transition { 0.06 } else { 0.0 })
    .clamp(0.80, 1.26);

    context.box_support = (style.attack_intent * 0.24
        + formation.box_presence * 0.46
        + formation.midfield_support * 0.18
        + formation.buildup_width * 0.12
        + if context.chasing_game { 0.05 } else { 0.0 }
        - if context.protecting_lead { 0.03 } else { 0.0 })
    .clamp(0.78, 1.28);

    context.rest_defense = (style.defensive_solidity * 0.35
        + style.buildup_retain * 0.10
        + formation.rest_defense * 0.42
        + formation.midfield_support * 0.13
        + if context.protecting_lead { 0.06 } else { 0.0 })
    .clamp(0.82, 1.28);

    context.width_access = (formation.buildup_width * 0.56
        + style.attack_intent * 0.12
        + style.transition_directness * 0.14
        + formation.box_presence * 0.10
        + formation.midfield_support * 0.08)
        .clamp(0.78, 1.28);

    context
}

/// Compute a multiplicative trait bonus for a specific action context.
/// Returns a modifier >= 1.0 (bonus) based on relevant traits.
pub(crate) fn trait_bonus(snap: &PlayerSnap, context: TraitContext) -> f64 {
    let mut bonus = 1.0;
    match context {
        TraitContext::Shooting => {
            bonus *= 1.0 + trait_strength(snap, "Sharpshooter") * 0.08;
            bonus *= 1.0 + trait_strength(snap, "CoolHead") * 0.04;
            bonus *= 1.0 + trait_strength(snap, "CompleteForward") * 0.05;
            bonus *= 1.0 + trait_strength(snap, "ToePokeFinisher") * 0.04;
            bonus *= 1.0 + trait_strength(snap, "HalfVolleyComfort") * 0.03;
            bonus *= 1.0 + trait_strength(snap, "ClutchExecutor") * 0.04;
        }
        TraitContext::Dribbling => {
            bonus *= 1.0 + trait_strength(snap, "Dribbler") * 0.08;
            bonus *= 1.0 + trait_strength(snap, "Speedster") * 0.04;
            bonus *= 1.0 + trait_strength(snap, "Agile") * 0.04;
            bonus *= 1.0 + trait_strength(snap, "DisguisedFirstTouch") * 0.05;
            bonus *= 1.0 + trait_strength(snap, "RecoveryTouch") * 0.03;
            bonus *= 1.0 + trait_strength(snap, "BounceRoomDribbler") * 0.03;
        }
        TraitContext::Passing => {
            bonus *= 1.0 + trait_strength(snap, "Playmaker") * 0.08;
            bonus *= 1.0 + trait_strength(snap, "Visionary") * 0.05;
            bonus *= 1.0 + trait_strength(snap, "SetPieceSpecialist") * 0.03;
            bonus *= 1.0 + trait_strength(snap, "EarlyScanner") * 0.03;
            bonus *= 1.0 + trait_strength(snap, "OutsideFootPasser") * 0.03;
            bonus *= 1.0 + trait_strength(snap, "OneTouchSpecialist") * 0.03;
        }
        TraitContext::Tackling => {
            bonus *= 1.0 + trait_strength(snap, "BallWinner") * 0.08;
            bonus *= 1.0 + trait_strength(snap, "Rock") * 0.05;
            bonus *= 1.0 + trait_strength(snap, "Tank") * 0.04;
            bonus *= 1.0 + trait_strength(snap, "ContainmentSpecialist") * 0.03;
            bonus *= 1.0 + trait_strength(snap, "PassingLaneThief") * 0.03;
            bonus *= 1.0 + trait_strength(snap, "AerialGrappler") * 0.02;
        }
        TraitContext::Goalkeeping => {
            bonus *= 1.0 + trait_strength(snap, "SafeHands") * 0.08;
            bonus *= 1.0 + trait_strength(snap, "CatReflexes") * 0.06;
            bonus *= 1.0 + trait_strength(snap, "AerialDominance") * 0.04;
            bonus *= 1.0 + trait_strength(snap, "ReboundDirector") * 0.03;
            bonus *= 1.0 + trait_strength(snap, "BreakawayHypnotist") * 0.03;
            bonus *= 1.0 + trait_strength(snap, "TrafficCommander") * 0.02;
        }
        TraitContext::Foul => {
            bonus *= 1.0 + trait_strength(snap, "HotHead") * 0.25;
            bonus *= 1.0 - trait_strength(snap, "CoolHead") * 0.30;
            bonus *= 1.0 + trait_strength(snap, "TacticalFouler") * 0.08;
            bonus *= 1.0 + trait_strength(snap, "Provocable") * 0.12;
            bonus *= 1.0 - trait_strength(snap, "RefereeManipulator") * 0.10;
        }
        TraitContext::Midfield => {
            bonus *= 1.0 + trait_strength(snap, "Engine") * 0.06;
            bonus *= 1.0 + trait_strength(snap, "TeamPlayer") * 0.04;
            bonus *= 1.0 + trait_strength(snap, "Tireless") * 0.03;
            bonus *= 1.0 + trait_strength(snap, "TempoManipulator") * 0.03;
            bonus *= 1.0 + trait_strength(snap, "RiskCalibrator") * 0.03;
            bonus *= 1.0 + trait_strength(snap, "TransitionAnticipator") * 0.02;
        }
    }
    bonus
}

pub(crate) fn choose_buildup_intent<R: Rng>(
    snap: &PlayerSnap,
    _style: PlayStyle,
    context: IntentContext,
    rng: &mut R,
) -> ActionIntent {
    let weights = vec![
        (ActionIntent::SafeRecyclePass, {
            0.72 + context.safe_outlets * 0.60
                + trait_strength(snap, "TempoManipulator") * 0.25
                + trait_strength(snap, "RiskCalibrator") * 0.15
                + if context.protecting_lead { 0.45 } else { 0.0 }
        }),
        (ActionIntent::SplitLinePass, {
            0.30 + context.progression_lanes * 0.62
                + trait_strength(snap, "Playmaker") * 0.55
                + trait_strength(snap, "Visionary") * 0.45
                + trait_strength(snap, "EarlyScanner") * 0.35
                + trait_strength(snap, "OutsideFootPasser") * 0.20
                + if matches!(
                    snap.role,
                    TacticalRole::CenterBackPlaymaker | TacticalRole::DeepPlaymaker
                ) {
                    0.35
                } else {
                    0.0
                }
                + if context.in_transition { 0.22 } else { 0.0 }
        }),
        (ActionIntent::BaitPressTouch, {
            0.10 + context.safe_outlets * 0.18
                + context.progression_lanes * 0.08
                + trait_strength(snap, "PressBaiter") * 0.95
                + trait_strength(snap, "ShieldAddict") * 0.25
                + trait_strength(snap, "RecoveryTouch") * 0.18
                + if context.under_pressure { 0.60 } else { 0.0 }
                + if context.chasing_game { 0.15 } else { 0.0 }
        }),
    ];

    weights[weighted_index(
        &weights
            .iter()
            .map(|(_, weight)| *weight)
            .collect::<Vec<_>>(),
        rng,
    )]
    .0
}

pub(crate) fn choose_midfield_intent<R: Rng>(
    snap: &PlayerSnap,
    _style: PlayStyle,
    context: IntentContext,
    rng: &mut R,
) -> ActionIntent {
    let weights = vec![
        (ActionIntent::OneTouchCombination, {
            0.24 + context.safe_outlets * 0.20
                + context.progression_lanes * 0.24
                + trait_strength(snap, "OneTouchSpecialist") * 0.90
                + trait_strength(snap, "TeamPlayer") * 0.25
                + if context.under_pressure { 0.20 } else { 0.0 }
        }),
        (ActionIntent::DelayedRelease, {
            0.18 + context.safe_outlets * 0.28
                + context.progression_lanes * 0.14
                + trait_strength(snap, "DelayedPasser") * 0.95
                + trait_strength(snap, "TempoManipulator") * 0.45
                + trait_strength(snap, "RiskCalibrator") * 0.35
                + if context.protecting_lead { 0.18 } else { 0.0 }
        }),
        (ActionIntent::ProgressiveCarry, {
            0.24 + context.progression_lanes * 0.18
                + context.width_access * 0.22
                + trait_strength(snap, "Dribbler") * 0.35
                + trait_strength(snap, "ChannelDrifter") * 0.45
                + trait_strength(snap, "ChaosCreator") * 0.28
                + if context.chasing_game { 0.18 } else { 0.0 }
        }),
        (ActionIntent::ThirdManLayoff, {
            0.16 + context.progression_lanes * 0.28
                + context.box_support * 0.18
                + trait_strength(snap, "SpaceMagnet") * 0.30
                + trait_strength(snap, "DecoyMover") * 0.45
                + trait_strength(snap, "Playmaker") * 0.20
                + if context.in_transition { 0.14 } else { 0.0 }
        }),
        (ActionIntent::PressEscapeTurn, {
            0.12 + context.progression_lanes * 0.12
                + context.width_access * 0.12
                + trait_strength(snap, "PressBaiter") * 0.35
                + trait_strength(snap, "DisguisedFirstTouch") * 0.45
                + trait_strength(snap, "RecoveryTouch") * 0.30
                + if context.under_pressure { 0.45 } else { 0.0 }
        }),
    ];

    weights[weighted_index(
        &weights
            .iter()
            .map(|(_, weight)| *weight)
            .collect::<Vec<_>>(),
        rng,
    )]
    .0
}

pub(crate) fn choose_attacking_intent<R: Rng>(
    snap: &PlayerSnap,
    _style: PlayStyle,
    context: IntentContext,
    rng: &mut R,
) -> ActionIntent {
    let weights = vec![
        (ActionIntent::LinkPlaySlip, {
            0.18 + context.progression_lanes * 0.22
                + context.box_support * 0.18
                + trait_strength(snap, "Playmaker") * 0.28
                + trait_strength(snap, "DelayedPasser") * 0.28
                + if matches!(
                    snap.role,
                    TacticalRole::LinkForward | TacticalRole::AdvancedPlaymaker
                ) {
                    0.35
                } else {
                    0.0
                }
        }),
        (ActionIntent::DirectDribble, {
            0.18 + context.width_access * 0.30
                + context.progression_lanes * 0.10
                + trait_strength(snap, "Dribbler") * 0.45
                + trait_strength(snap, "ChaosCreator") * 0.25
                + trait_strength(snap, "NutmegOpportunist") * 0.20
                + if context.chasing_game { 0.15 } else { 0.0 }
        }),
        (ActionIntent::BlindSideRun, {
            0.10 + context.progression_lanes * 0.18
                + context.box_support * 0.22
                + trait_strength(snap, "BlindSideRunner") * 0.95
                + trait_strength(snap, "TransitionAnticipator") * 0.25
                + if context.in_transition { 0.35 } else { 0.0 }
        }),
        (ActionIntent::LateBoxDelivery, {
            0.10 + context.box_support * 0.30
                + context.width_access * 0.10
                + trait_strength(snap, "LateBoxArriver") * 0.60
                + trait_strength(snap, "FarPostGhost") * 0.25
                + trait_strength(snap, "DecoyMover") * 0.25
        }),
        (ActionIntent::NearPostAttack, {
            0.08 + context.box_support * 0.34
                + trait_strength(snap, "NearPostHunter") * 0.90
                + trait_strength(snap, "ChannelDrifter") * 0.20
                + if context.in_transition { 0.12 } else { 0.0 }
        }),
    ];

    weights[weighted_index(
        &weights
            .iter()
            .map(|(_, weight)| *weight)
            .collect::<Vec<_>>(),
        rng,
    )]
    .0
}

pub(crate) fn choose_defensive_intent<R: Rng>(
    snap: &PlayerSnap,
    style: PlayStyle,
    context: IntentContext,
    rng: &mut R,
) -> ActionIntent {
    let weights = vec![
        (ActionIntent::StepInInterception, {
            0.18 + style_profile(style).press_intensity * 0.08
                + context.rest_defense * 0.18
                + trait_strength(snap, "PassingLaneThief") * 0.95
                + trait_strength(snap, "EarlyScanner") * 0.18
                + if context.chasing_game { 0.12 } else { 0.0 }
        }),
        (ActionIntent::ContainAndShowWide, {
            0.18 + context.rest_defense * 0.34
                + context.width_access * 0.08
                + trait_strength(snap, "ContainmentSpecialist") * 0.95
                + trait_strength(snap, "BodyAngleManipulator") * 0.45
                + if context.protecting_lead { 0.20 } else { 0.0 }
        }),
        (ActionIntent::TacticalFoulStop, {
            0.05 + style_profile(style).press_intensity * 0.03
                + trait_strength(snap, "TacticalFouler") * 0.95
                + trait_strength(snap, "RefereeManipulator") * 0.18
                + if context.in_transition { 0.55 } else { 0.0 }
                + if context.chasing_game { 0.08 } else { 0.0 }
        }),
        (ActionIntent::AerialClearance, {
            0.10 + context.rest_defense * 0.16
                + context.box_support * 0.06
                + trait_strength(snap, "AerialGrappler") * 0.55
                + trait_strength(snap, "SecondContactWinner") * 0.25
        }),
    ];

    weights[weighted_index(
        &weights
            .iter()
            .map(|(_, weight)| *weight)
            .collect::<Vec<_>>(),
        rng,
    )]
    .0
}

pub(crate) fn choose_goalkeeper_distribution_intent<R: Rng>(
    snap: &PlayerSnap,
    _style: PlayStyle,
    context: IntentContext,
    rng: &mut R,
) -> ActionIntent {
    let weights = vec![
        (ActionIntent::SecureClaim, {
            0.78 + context.safe_outlets * 0.34
                + context.rest_defense * 0.16
                + trait_strength(snap, "SafeHands") * 0.20
                + if context.protecting_lead { 0.18 } else { 0.0 }
        }),
        (ActionIntent::LaunchThrow, {
            0.06 + context.progression_lanes * 0.14
                + context.width_access * 0.12
                + trait_strength(snap, "ThrowLauncher") * 0.72
                + trait_strength(snap, "TransitionAnticipator") * 0.18
                + if context.in_transition { 0.18 } else { 0.0 }
                + if context.chasing_game { 0.08 } else { 0.0 }
                - if context.protecting_lead { 0.05 } else { 0.0 }
        }),
    ];

    weights[weighted_index(
        &weights
            .iter()
            .map(|(_, weight)| *weight)
            .collect::<Vec<_>>(),
        rng,
    )]
    .0
}

// ---------------------------------------------------------------------------
// Tactical profiles
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
pub(crate) enum PlayStylePhase {
    Midfield,
    Attack,
    Defense,
    Press,
}

#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
pub(crate) struct StyleProfile {
    pub tempo: f64,
    pub buildup_retain: f64,
    pub midfield_control: f64,
    pub attack_intent: f64,
    pub defensive_solidity: f64,
    pub press_intensity: f64,
    pub transition_directness: f64,
    pub fatigue_burden: f64,
}

pub(crate) fn style_profile(style: PlayStyle) -> StyleProfile {
    match style {
        PlayStyle::Balanced => StyleProfile {
            tempo: 1.0,
            buildup_retain: 1.0,
            midfield_control: 1.0,
            attack_intent: 1.0,
            defensive_solidity: 1.0,
            press_intensity: 1.0,
            transition_directness: 1.0,
            fatigue_burden: 1.0,
        },
        PlayStyle::Attacking => StyleProfile {
            tempo: 1.04,
            buildup_retain: 0.98,
            midfield_control: 0.99,
            attack_intent: 1.03,
            defensive_solidity: 0.94,
            press_intensity: 1.03,
            transition_directness: 1.00,
            fatigue_burden: 1.05,
        },
        PlayStyle::Defensive => StyleProfile {
            tempo: 0.90,
            buildup_retain: 1.02,
            midfield_control: 0.98,
            attack_intent: 0.95,
            defensive_solidity: 1.16,
            press_intensity: 1.01,
            transition_directness: 1.03,
            fatigue_burden: 0.93,
        },
        PlayStyle::Possession => StyleProfile {
            tempo: 0.94,
            buildup_retain: 1.12,
            midfield_control: 1.08,
            attack_intent: 0.93,
            defensive_solidity: 1.01,
            press_intensity: 0.98,
            transition_directness: 0.94,
            fatigue_burden: 0.97,
        },
        PlayStyle::Counter => StyleProfile {
            tempo: 0.96,
            buildup_retain: 0.94,
            midfield_control: 0.92,
            attack_intent: 0.95,
            defensive_solidity: 1.04,
            press_intensity: 0.97,
            transition_directness: 1.22,
            fatigue_burden: 0.96,
        },
        PlayStyle::HighPress => StyleProfile {
            tempo: 1.00,
            buildup_retain: 0.97,
            midfield_control: 1.00,
            attack_intent: 0.95,
            defensive_solidity: 0.92,
            press_intensity: 1.08,
            transition_directness: 1.00,
            fatigue_burden: 1.22,
        },
    }
}

#[allow(dead_code)]
pub(crate) fn play_style_modifier(
    style: PlayStyle,
    phase: PlayStylePhase,
    is_own_phase: bool,
) -> f64 {
    if !is_own_phase {
        return 1.0;
    }
    let profile = style_profile(style);
    match phase {
        PlayStylePhase::Midfield => profile.midfield_control,
        PlayStylePhase::Attack => profile.attack_intent,
        PlayStylePhase::Defense => profile.defensive_solidity,
        PlayStylePhase::Press => profile.press_intensity,
    }
}

pub(crate) fn tempo_modifier(style: PlayStyle) -> f64 {
    style_profile(style).tempo
}

pub(crate) fn buildup_modifier(style: PlayStyle, in_transition: bool) -> f64 {
    let profile = style_profile(style);
    let transition_boost = if in_transition {
        1.0 + (profile.transition_directness - 1.0) * 0.35
    } else {
        1.0
    };
    profile.buildup_retain * transition_boost
}

pub(crate) fn midfield_attack_modifier(style: PlayStyle, in_transition: bool) -> f64 {
    let profile = style_profile(style);
    let transition_boost = if in_transition {
        1.0 + (profile.transition_directness - 1.0) * 0.55
    } else {
        1.0
    };
    profile.midfield_control * transition_boost
}

pub(crate) fn midfield_defense_modifier(style: PlayStyle) -> f64 {
    let profile = style_profile(style);
    profile.press_intensity * 0.55 + profile.defensive_solidity * 0.45
}

pub(crate) fn attack_modifier(style: PlayStyle, in_transition: bool) -> f64 {
    let profile = style_profile(style);
    let transition_boost = if in_transition {
        profile.transition_directness
    } else {
        1.0
    };
    profile.attack_intent * transition_boost
}

pub(crate) fn defense_modifier(style: PlayStyle) -> f64 {
    style_profile(style).defensive_solidity
}

pub(crate) fn press_modifier(style: PlayStyle) -> f64 {
    style_profile(style).press_intensity
}

pub(crate) fn fatigue_modifier(style: PlayStyle) -> f64 {
    style_profile(style).fatigue_burden
}

pub(crate) fn role_transition_outlet_bias(role: TacticalRole) -> f64 {
    match role {
        TacticalRole::DeepPlaymaker | TacticalRole::AdvancedPlaymaker => 0.16,
        TacticalRole::CenterBackPlaymaker | TacticalRole::SweeperKeeper => 0.12,
        TacticalRole::WingBackAttack | TacticalRole::WideProgressor => 0.10,
        TacticalRole::LinkForward | TacticalRole::ChannelRunner => 0.12,
        TacticalRole::TargetForward => 0.10,
        TacticalRole::BoxToBoxMidfielder => 0.06,
        _ => 0.0,
    }
}

pub(crate) fn role_box_support_bias(role: TacticalRole) -> f64 {
    match role {
        TacticalRole::Poacher | TacticalRole::TargetForward | TacticalRole::ChannelRunner => 0.18,
        TacticalRole::LinkForward | TacticalRole::AdvancedPlaymaker => 0.12,
        TacticalRole::WingBackAttack | TacticalRole::WideProgressor => 0.10,
        TacticalRole::BoxToBoxMidfielder => 0.08,
        TacticalRole::FullBackSupport => 0.04,
        _ => 0.0,
    }
}

pub(crate) fn transition_entry_chance(
    style: PlayStyle,
    formation: &str,
    outlet_role: TacticalRole,
    context: IntentContext,
    opposition_rest_defense: f64,
) -> f64 {
    let profile = style_profile(style);
    let formation = formation_profile(formation);
    (0.08
        + (profile.transition_directness - 1.0) * 0.28
        + (context.progression_lanes - 1.0) * 0.30
        + (context.safe_outlets - 1.0) * 0.08
        + (formation.midfield_support - 1.0) * 0.12
        + role_transition_outlet_bias(outlet_role) * 0.85
        + if context.in_transition { 0.08 } else { 0.0 }
        - (opposition_rest_defense - 1.0) * 0.30)
        .clamp(0.05, 0.52)
}

pub(crate) fn box_entry_chance(
    style: PlayStyle,
    formation: &str,
    attacker_role: TacticalRole,
    context: IntentContext,
    opposition_rest_defense: f64,
    press_beaten: bool,
) -> f64 {
    let profile = style_profile(style);
    let formation = formation_profile(formation);
    (0.08
        + (profile.attack_intent - 1.0) * 0.14
        + (profile.transition_directness - 1.0) * 0.16
        + (context.progression_lanes - 1.0) * 0.22
        + (context.box_support - 1.0) * 0.28
        + (formation.box_presence - 1.0) * 0.20
        + role_box_support_bias(attacker_role) * 0.75
        + if context.in_transition { 0.08 } else { 0.0 }
        + if press_beaten { 0.08 } else { 0.0 }
        - (opposition_rest_defense - 1.0) * 0.32)
        .clamp(0.05, 0.48)
}

#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
pub(crate) struct FormationProfile {
    pub buildup_width: f64,
    pub midfield_support: f64,
    pub box_presence: f64,
    pub rest_defense: f64,
}

#[allow(dead_code)]
pub(crate) fn formation_profile(formation: &str) -> FormationProfile {
    match formation {
        "4-4-2" => FormationProfile {
            buildup_width: 1.0,
            midfield_support: 1.0,
            box_presence: 1.0,
            rest_defense: 1.0,
        },
        "4-3-3" => FormationProfile {
            buildup_width: 1.04,
            midfield_support: 0.97,
            box_presence: 1.05,
            rest_defense: 0.98,
        },
        "3-5-2" => FormationProfile {
            buildup_width: 0.96,
            midfield_support: 1.06,
            box_presence: 1.01,
            rest_defense: 0.99,
        },
        "4-5-1" => FormationProfile {
            buildup_width: 0.98,
            midfield_support: 1.05,
            box_presence: 0.94,
            rest_defense: 1.03,
        },
        "4-2-3-1" => FormationProfile {
            buildup_width: 1.0,
            midfield_support: 1.05,
            box_presence: 0.97,
            rest_defense: 1.04,
        },
        "4-1-4-1" => FormationProfile {
            buildup_width: 0.98,
            midfield_support: 1.03,
            box_presence: 0.93,
            rest_defense: 1.06,
        },
        "3-4-3" => FormationProfile {
            buildup_width: 1.08,
            midfield_support: 1.00,
            box_presence: 1.12,
            rest_defense: 0.99,
        },
        "5-3-2" => FormationProfile {
            buildup_width: 0.94,
            midfield_support: 0.96,
            box_presence: 0.99,
            rest_defense: 1.08,
        },
        _ => formation_profile("4-4-2"),
    }
}

pub(crate) fn formation_buildup_modifier(formation: &str) -> f64 {
    formation_profile(formation).buildup_width
}

pub(crate) fn formation_midfield_modifier(formation: &str) -> f64 {
    formation_profile(formation).midfield_support
}

pub(crate) fn formation_attack_modifier(formation: &str) -> f64 {
    formation_profile(formation).box_presence
}

pub(crate) fn formation_rest_defense_modifier(formation: &str) -> f64 {
    formation_profile(formation).rest_defense
}

pub(crate) fn player_action_weight(
    player: &PlayerData,
    preferred: crate::types::Position,
    formation: &str,
) -> f64 {
    use crate::types::Position;

    let profile = formation_profile(formation);
    let position_fit = if player.position == preferred {
        1.7
    } else {
        0.55
    };
    let role_skill = match preferred {
        Position::Goalkeeper => {
            (player.handling as f64 + player.reflexes as f64 + player.positioning as f64) / 3.0
        }
        Position::Defender => {
            ((player.passing as f64
                + player.vision as f64
                + player.composure as f64
                + player.teamwork as f64)
                / 4.0)
                * (profile.buildup_width * 0.65 + profile.rest_defense * 0.35)
        }
        Position::Midfielder => {
            ((player.passing as f64
                + player.vision as f64
                + player.decisions as f64
                + player.teamwork as f64
                + player.stamina as f64)
                / 5.0)
                * profile.midfield_support
        }
        Position::Forward => {
            ((player.shooting as f64
                + player.positioning as f64
                + player.composure as f64
                + player.decisions as f64
                + player.pace as f64)
                / 5.0)
                * profile.box_presence
        }
    };

    let trait_fit = trait_involvement_modifier(player, preferred);
    let tactical_fit = tactical_role_involvement_modifier(player.role, preferred);

    (role_skill * position_fit * trait_fit * tactical_fit).max(1.0)
}

pub(crate) fn trait_involvement_modifier(
    player: &PlayerData,
    preferred: crate::types::Position,
) -> f64 {
    use crate::types::Position;

    let snap = PlayerSnap::from(player);
    let bonus = match preferred {
        Position::Goalkeeper => sum_trait_weights(
            &snap,
            &[
                ("SafeHands", 0.06),
                ("CatReflexes", 0.06),
                ("AerialDominance", 0.04),
                ("LineDictator", 0.04),
                ("TrafficCommander", 0.04),
                ("ThrowLauncher", 0.03),
            ],
        ),
        Position::Defender => sum_trait_weights(
            &snap,
            &[
                ("Rock", 0.06),
                ("BallWinner", 0.04),
                ("ContainmentSpecialist", 0.05),
                ("RecoverySprinter", 0.04),
                ("PassingLaneThief", 0.05),
                ("BodyAngleManipulator", 0.04),
                ("AerialGrappler", 0.05),
                ("SecondContactWinner", 0.03),
                ("EarlyScanner", 0.02),
            ],
        ),
        Position::Midfielder => sum_trait_weights(
            &snap,
            &[
                ("Playmaker", 0.06),
                ("Visionary", 0.05),
                ("Engine", 0.04),
                ("TeamPlayer", 0.04),
                ("EarlyScanner", 0.04),
                ("TempoManipulator", 0.05),
                ("RiskCalibrator", 0.04),
                ("SpaceMagnet", 0.05),
                ("TransitionAnticipator", 0.03),
                ("LateBoxArriver", 0.03),
            ],
        ),
        Position::Forward => sum_trait_weights(
            &snap,
            &[
                ("Sharpshooter", 0.06),
                ("CompleteForward", 0.05),
                ("BlindSideRunner", 0.06),
                ("NearPostHunter", 0.04),
                ("FarPostGhost", 0.04),
                ("ReboundInstinct", 0.04),
                ("ChannelDrifter", 0.04),
                ("ChaosCreator", 0.03),
                ("KeeperDisruptor", 0.04),
                ("ClutchExecutor", 0.03),
            ],
        ),
    };

    match (player.position, preferred) {
        (Position::Defender, Position::Midfielder) if snap.has_trait("ChannelDrifter") => {
            (1.0 + bonus + 0.03).clamp(0.96, 1.24)
        }
        (Position::Midfielder, Position::Forward) if snap.has_trait("LateBoxArriver") => {
            (1.0 + bonus + 0.03).clamp(0.96, 1.24)
        }
        (Position::Forward, Position::Midfielder) if snap.has_trait("DecoyMover") => {
            (1.0 + bonus + 0.02).clamp(0.96, 1.24)
        }
        _ => (1.0 + bonus).clamp(0.96, 1.24),
    }
}

pub(crate) fn tactical_role_involvement_modifier(
    role: TacticalRole,
    preferred: crate::types::Position,
) -> f64 {
    use crate::types::Position;

    match (role, preferred) {
        (TacticalRole::Goalkeeper | TacticalRole::SweeperKeeper, Position::Goalkeeper) => 1.20,
        (
            TacticalRole::CenterBackStopper
            | TacticalRole::CenterBackCover
            | TacticalRole::CenterBackPlaymaker
            | TacticalRole::FullBackSupport
            | TacticalRole::WingBackAttack,
            Position::Defender,
        ) => 1.14,
        (
            TacticalRole::HoldingMidfielder
            | TacticalRole::DeepPlaymaker
            | TacticalRole::BoxToBoxMidfielder
            | TacticalRole::AdvancedPlaymaker
            | TacticalRole::WideProgressor,
            Position::Midfielder,
        ) => 1.14,
        (
            TacticalRole::Poacher
            | TacticalRole::TargetForward
            | TacticalRole::ChannelRunner
            | TacticalRole::LinkForward,
            Position::Forward,
        ) => 1.14,
        (TacticalRole::WingBackAttack, Position::Midfielder) => 1.06,
        (TacticalRole::DeepPlaymaker | TacticalRole::AdvancedPlaymaker, Position::Forward) => 1.04,
        (TacticalRole::ChannelRunner, Position::Midfielder) => 1.04,
        _ => 0.99,
    }
}

pub(crate) fn role_pass_bias(role: TacticalRole) -> f64 {
    match role {
        TacticalRole::CenterBackPlaymaker
        | TacticalRole::DeepPlaymaker
        | TacticalRole::AdvancedPlaymaker
        | TacticalRole::LinkForward => 1.18,
        TacticalRole::FullBackSupport
        | TacticalRole::HoldingMidfielder
        | TacticalRole::WideProgressor => 1.08,
        TacticalRole::CenterBackStopper | TacticalRole::TargetForward => 0.94,
        TacticalRole::Poacher => 0.86,
        _ => 1.0,
    }
}

pub(crate) fn role_carry_bias(role: TacticalRole) -> f64 {
    match role {
        TacticalRole::WingBackAttack
        | TacticalRole::WideProgressor
        | TacticalRole::ChannelRunner => 1.18,
        TacticalRole::BoxToBoxMidfielder | TacticalRole::AdvancedPlaymaker => 1.08,
        TacticalRole::CenterBackCover | TacticalRole::HoldingMidfielder => 0.92,
        TacticalRole::TargetForward => 0.90,
        _ => 1.0,
    }
}

pub(crate) fn role_shot_bias(role: TacticalRole) -> f64 {
    match role {
        TacticalRole::Poacher | TacticalRole::TargetForward => 1.14,
        TacticalRole::ChannelRunner => 1.08,
        TacticalRole::AdvancedPlaymaker => 0.97,
        TacticalRole::LinkForward => 0.93,
        TacticalRole::HoldingMidfielder | TacticalRole::CenterBackCover => 0.82,
        _ => 1.0,
    }
}

pub(crate) fn role_clearance_bias(role: TacticalRole) -> f64 {
    match role {
        TacticalRole::CenterBackStopper | TacticalRole::CenterBackCover => 1.16,
        TacticalRole::CenterBackPlaymaker | TacticalRole::SweeperKeeper => 0.88,
        _ => 1.0,
    }
}

pub(crate) fn role_press_bias(role: TacticalRole) -> f64 {
    match role {
        TacticalRole::WingBackAttack
        | TacticalRole::BoxToBoxMidfielder
        | TacticalRole::ChannelRunner => 1.12,
        TacticalRole::HoldingMidfielder | TacticalRole::CenterBackCover => 0.96,
        TacticalRole::Poacher => 0.92,
        _ => 1.0,
    }
}

pub(crate) fn trait_pass_bias(snap: &PlayerSnap) -> f64 {
    (1.0 + sum_trait_weights(
        snap,
        &[
            ("Playmaker", 0.07),
            ("Visionary", 0.05),
            ("EarlyScanner", 0.04),
            ("TempoManipulator", 0.04),
            ("DelayedPasser", 0.04),
            ("RiskCalibrator", 0.03),
            ("PressBaiter", 0.04),
            ("OneTouchSpecialist", 0.03),
            ("OutsideFootPasser", 0.03),
            ("ThrowLauncher", 0.03),
        ],
    ))
    .clamp(0.88, 1.22)
}

pub(crate) fn trait_carry_bias(snap: &PlayerSnap) -> f64 {
    (1.0 + sum_trait_weights(
        snap,
        &[
            ("Dribbler", 0.06),
            ("Speedster", 0.04),
            ("Agile", 0.03),
            ("DisguisedFirstTouch", 0.04),
            ("RecoveryTouch", 0.03),
            ("ChannelDrifter", 0.05),
            ("ChaosCreator", 0.04),
            ("BounceRoomDribbler", 0.03),
            ("NutmegOpportunist", 0.03),
            ("BlindSideRunner", 0.02),
        ],
    ))
    .clamp(0.88, 1.22)
}

pub(crate) fn trait_shot_bias(snap: &PlayerSnap) -> f64 {
    (1.0 + sum_trait_weights(
        snap,
        &[
            ("Sharpshooter", 0.06),
            ("CompleteForward", 0.04),
            ("ToePokeFinisher", 0.04),
            ("HalfVolleyComfort", 0.03),
            ("LateBoxArriver", 0.03),
            ("NearPostHunter", 0.04),
            ("BlindSideRunner", 0.04),
            ("FarPostGhost", 0.03),
            ("ReboundInstinct", 0.03),
            ("ClutchExecutor", 0.03),
        ],
    ))
    .clamp(0.84, 1.24)
}

pub(crate) fn trait_clearance_bias(snap: &PlayerSnap) -> f64 {
    (1.0 + sum_trait_weights(
        snap,
        &[
            ("Rock", 0.05),
            ("ContainmentSpecialist", 0.03),
            ("RecoverySprinter", 0.02),
            ("BodyAngleManipulator", 0.04),
            ("AerialGrappler", 0.05),
            ("SecondContactWinner", 0.03),
        ],
    ) - sum_trait_weights(
        snap,
        &[
            ("Playmaker", 0.03),
            ("Visionary", 0.02),
            ("ThrowLauncher", 0.02),
        ],
    ))
    .clamp(0.84, 1.20)
}

pub(crate) fn trait_press_bias(snap: &PlayerSnap) -> f64 {
    (1.0 + sum_trait_weights(
        snap,
        &[
            ("BallWinner", 0.06),
            ("Engine", 0.04),
            ("Tireless", 0.03),
            ("TransitionAnticipator", 0.04),
            ("PassingLaneThief", 0.04),
            ("TacticalFouler", 0.03),
            ("KeeperDisruptor", 0.03),
        ],
    ) - sum_trait_weights(snap, &[("DelayedPasser", 0.02), ("TimeKiller", 0.03)]))
    .clamp(0.88, 1.20)
}

fn sum_trait_weights(snap: &PlayerSnap, weighted_traits: &[(&str, f64)]) -> f64 {
    weighted_traits
        .iter()
        .map(|(name, weight)| trait_strength(snap, name) * *weight)
        .sum()
}

pub(crate) fn weighted_index<R: Rng>(weights: &[f64], rng: &mut R) -> usize {
    let total = weights.iter().copied().sum::<f64>();
    if total <= f64::EPSILON {
        return rng.gen_range(0..weights.len());
    }

    let mut target = rng.gen_range(0.0..total);
    for (index, weight) in weights.iter().copied().enumerate() {
        if target <= weight {
            return index;
        }
        target -= weight;
    }

    weights.len().saturating_sub(1)
}

// ---------------------------------------------------------------------------
// Home advantage modifier
// ---------------------------------------------------------------------------

pub(crate) fn home_mod(side: Side, config: &MatchConfig) -> f64 {
    match side {
        Side::Home => config.home_advantage,
        Side::Away => 1.0,
    }
}

#[cfg(test)]
mod tests {
    use super::{PlayStylePhase, formation_profile, play_style_modifier, style_profile};
    use crate::types::PlayStyle;

    #[test]
    fn style_profile_preserves_existing_phase_biases() {
        let attacking = style_profile(PlayStyle::Attacking);
        assert!(attacking.attack_intent > 1.0);
        assert!(attacking.defensive_solidity < 1.0);

        let defensive = style_profile(PlayStyle::Defensive);
        assert!(defensive.defensive_solidity > 1.0);
        assert!(defensive.attack_intent < 1.0);

        let possession = style_profile(PlayStyle::Possession);
        assert!(possession.midfield_control > 1.0);

        let counter = style_profile(PlayStyle::Counter);
        assert!(counter.attack_intent < 1.0);
        assert!(counter.midfield_control < 1.0);
        assert!(counter.transition_directness > 1.0);

        let high_press = style_profile(PlayStyle::HighPress);
        assert!(high_press.press_intensity > 1.0);
        assert!(high_press.defensive_solidity < 1.0);
    }

    #[test]
    fn play_style_modifier_reads_profile_values() {
        assert_eq!(
            play_style_modifier(PlayStyle::Balanced, PlayStylePhase::Midfield, true),
            1.0
        );
        assert!(play_style_modifier(PlayStyle::Possession, PlayStylePhase::Midfield, true) > 1.0);
        assert!(play_style_modifier(PlayStyle::Counter, PlayStylePhase::Attack, true) < 1.0);
        assert_eq!(
            play_style_modifier(PlayStyle::Attacking, PlayStylePhase::Attack, false),
            1.0
        );
    }

    #[test]
    fn formation_profiles_distinguish_four_five_one_variants() {
        let generic = formation_profile("4-5-1");
        let attacking_mid = formation_profile("4-2-3-1");
        let holding_mid = formation_profile("4-1-4-1");

        assert_ne!(generic.box_presence, attacking_mid.box_presence);
        assert_ne!(generic.rest_defense, holding_mid.rest_defense);
        assert_ne!(attacking_mid.buildup_width, holding_mid.buildup_width);
    }
}
