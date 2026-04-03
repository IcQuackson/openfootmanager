use crate::types::{MatchConfig, PlayStyle, PlayerData, Side};

// ---------------------------------------------------------------------------
// PlayerSnap — lightweight snapshot of a player to avoid borrow conflicts
// ---------------------------------------------------------------------------

#[derive(Clone)]
#[allow(dead_code)]
pub(crate) struct PlayerSnap {
    pub id: String,
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

/// Compute a multiplicative trait bonus for a specific action context.
/// Returns a modifier >= 1.0 (bonus) based on relevant traits.
pub(crate) fn trait_bonus(snap: &PlayerSnap, context: TraitContext) -> f64 {
    let mut bonus = 1.0;
    match context {
        TraitContext::Shooting => {
            if snap.has_trait("Sharpshooter") {
                bonus *= 1.08;
            }
            if snap.has_trait("CoolHead") {
                bonus *= 1.04;
            }
            if snap.has_trait("CompleteForward") {
                bonus *= 1.05;
            }
        }
        TraitContext::Dribbling => {
            if snap.has_trait("Dribbler") {
                bonus *= 1.08;
            }
            if snap.has_trait("Speedster") {
                bonus *= 1.04;
            }
            if snap.has_trait("Agile") {
                bonus *= 1.04;
            }
        }
        TraitContext::Passing => {
            if snap.has_trait("Playmaker") {
                bonus *= 1.08;
            }
            if snap.has_trait("Visionary") {
                bonus *= 1.05;
            }
            if snap.has_trait("SetPieceSpecialist") {
                bonus *= 1.03;
            }
        }
        TraitContext::Tackling => {
            if snap.has_trait("BallWinner") {
                bonus *= 1.08;
            }
            if snap.has_trait("Rock") {
                bonus *= 1.05;
            }
            if snap.has_trait("Tank") {
                bonus *= 1.04;
            }
        }
        TraitContext::Goalkeeping => {
            if snap.has_trait("SafeHands") {
                bonus *= 1.08;
            }
            if snap.has_trait("CatReflexes") {
                bonus *= 1.06;
            }
            if snap.has_trait("AerialDominance") {
                bonus *= 1.04;
            }
        }
        TraitContext::Foul => {
            if snap.has_trait("HotHead") {
                bonus *= 1.25;
            }
            if snap.has_trait("CoolHead") {
                bonus *= 0.70;
            }
        }
        TraitContext::Midfield => {
            if snap.has_trait("Engine") {
                bonus *= 1.06;
            }
            if snap.has_trait("TeamPlayer") {
                bonus *= 1.04;
            }
            if snap.has_trait("Tireless") {
                bonus *= 1.03;
            }
        }
    }
    bonus
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
            tempo: 1.08,
            buildup_retain: 0.98,
            midfield_control: 0.99,
            attack_intent: 1.08,
            defensive_solidity: 0.94,
            press_intensity: 1.03,
            transition_directness: 1.02,
            fatigue_burden: 1.05,
        },
        PlayStyle::Defensive => StyleProfile {
            tempo: 0.92,
            buildup_retain: 0.99,
            midfield_control: 0.96,
            attack_intent: 0.93,
            defensive_solidity: 1.10,
            press_intensity: 0.98,
            transition_directness: 0.98,
            fatigue_burden: 0.95,
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
            tempo: 1.06,
            buildup_retain: 0.99,
            midfield_control: 1.02,
            attack_intent: 1.01,
            defensive_solidity: 0.96,
            press_intensity: 1.16,
            transition_directness: 1.08,
            fatigue_burden: 1.12,
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

pub(crate) fn transition_progress_chance(style: PlayStyle) -> f64 {
    let directness = style_profile(style).transition_directness;
    ((directness - 1.0) * 1.15 + 0.10).clamp(0.05, 0.40)
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
            buildup_width: 1.06,
            midfield_support: 0.95,
            box_presence: 1.08,
            rest_defense: 0.94,
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
