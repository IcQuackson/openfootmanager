mod fitness_warnings;
pub use fitness_warnings::check_squad_fitness_warnings;

use crate::game::Game;
use domain::player::{
    Player, PlayerAttributes, PlayerTrait, Position, TraitTrainingMode, trait_is_learnable,
};
use domain::staff::{CoachingSpecialization, StaffRole};
use domain::team::{TrainingFocus, TrainingIntensity, TrainingSchedule};

/// Computed coaching quality for a team's staff.
pub struct TeamCoachingBonus {
    pub coaching_mult: f64, // Overall coaching quality multiplier (1.0 = no staff)
    pub specialization_mult: f64, // Extra bonus if a coach specializes in the current focus
    pub physio_mult: f64,   // Recovery bonus from physio staff
}

/// Compute coaching bonuses from a team's staff.
fn compute_coaching_bonus(game: &Game, team_id: &str, focus: &TrainingFocus) -> TeamCoachingBonus {
    let team_staff: Vec<_> = game
        .staff
        .iter()
        .filter(|s| s.team_id.as_deref() == Some(team_id))
        .collect();

    // Average coaching rating of coaches + assistant managers
    let coaching_staff: Vec<_> = team_staff
        .iter()
        .filter(|s| matches!(s.role, StaffRole::Coach | StaffRole::AssistantManager))
        .collect();

    let coaching_mult = if coaching_staff.is_empty() {
        0.8 // Penalty for having no coaching staff
    } else {
        let avg_coaching: f64 = coaching_staff
            .iter()
            .map(|s| s.attributes.coaching as f64)
            .sum::<f64>()
            / coaching_staff.len() as f64;
        // Range: 0.85 (coaching=0) to 1.35 (coaching=100)
        0.85 + (avg_coaching / 100.0) * 0.5
    };

    // Check if any coach specializes in the current training focus
    let focus_spec = match focus {
        TrainingFocus::Physical => Some(CoachingSpecialization::Fitness),
        TrainingFocus::Technical => Some(CoachingSpecialization::Technique),
        TrainingFocus::Tactical => Some(CoachingSpecialization::Tactics),
        TrainingFocus::Defending => Some(CoachingSpecialization::Defending),
        TrainingFocus::Attacking => Some(CoachingSpecialization::Attacking),
        TrainingFocus::Recovery => None,
    };

    let specialization_mult = if let Some(target_spec) = focus_spec {
        let has_specialist = coaching_staff
            .iter()
            .any(|s| s.specialization.as_ref() == Some(&target_spec));
        if has_specialist { 1.25 } else { 1.0 }
    } else {
        1.0
    };

    // Physio bonus for recovery
    let physio_staff: Vec<_> = team_staff
        .iter()
        .filter(|s| matches!(s.role, StaffRole::Physio))
        .collect();

    let physio_mult = if physio_staff.is_empty() {
        1.0
    } else {
        let avg_physio: f64 = physio_staff
            .iter()
            .map(|s| s.attributes.physiotherapy as f64)
            .sum::<f64>()
            / physio_staff.len() as f64;
        // Range: 1.0 (physio=0) to 1.4 (physio=100)
        1.0 + (avg_physio / 100.0) * 0.4
    };

    TeamCoachingBonus {
        coaching_mult,
        specialization_mult,
        physio_mult,
    }
}

/// Per-team data collected before mutating players.
struct TeamTrainingPlan {
    team_id: String,
    default_focus: TrainingFocus,
    intensity: TrainingIntensity,
    schedule: TrainingSchedule,
    bonus: TeamCoachingBonus,
    medical_facility_mult: f64,
    /// player_id → group focus override (players not in any group use default_focus)
    group_overrides: std::collections::HashMap<String, TrainingFocus>,
}

/// Process daily training for all teams.
/// On non-match days each team's players train according to the team's
/// current focus, intensity, and schedule. Rest days (determined by the
/// weekly schedule) give full condition recovery with no training cost.
/// Players assigned to a training group use that group's focus instead of
/// the team default.
/// `weekday_num` is 0=Mon .. 6=Sun (chrono Weekday::num_days_from_monday()).
pub fn process_training(game: &mut Game, weekday_num: u32) {
    // Collect plans for all teams (immutable borrow)
    let team_plans: Vec<TeamTrainingPlan> = game
        .teams
        .iter()
        .map(|t| {
            let bonus = compute_coaching_bonus(game, &t.id, &t.training_focus);
            let medical_facility_mult =
                1.0 + f64::from(t.facilities.medical.saturating_sub(1)) * 0.1;
            let mut group_overrides = std::collections::HashMap::new();
            for group in &t.training_groups {
                for pid in &group.player_ids {
                    group_overrides.insert(pid.clone(), group.focus.clone());
                }
            }
            TeamTrainingPlan {
                team_id: t.id.clone(),
                default_focus: t.training_focus.clone(),
                intensity: t.training_intensity.clone(),
                schedule: t.training_schedule.clone(),
                bonus,
                medical_facility_mult,
                group_overrides,
            }
        })
        .collect();

    for plan in &team_plans {
        let is_training_day = plan.schedule.is_training_day(weekday_num);

        let intensity_mult = match &plan.intensity {
            TrainingIntensity::Low => 0.5,
            TrainingIntensity::Medium => 1.0,
            TrainingIntensity::High => 1.5,
        };

        for player in game.players.iter_mut() {
            if player.team_id.as_deref() != Some(&plan.team_id) {
                continue;
            }

            // Determine this player's effective focus:
            // player override > group override > team default
            let player_focus = player
                .training_focus
                .as_ref()
                .or_else(|| plan.group_overrides.get(&player.id))
                .unwrap_or(&plan.default_focus)
                .clone();

            // On rest days or Recovery focus: no training cost
            let condition_cost: u8 = if !is_training_day {
                0
            } else {
                match (&player_focus, &plan.intensity) {
                    (TrainingFocus::Recovery, _) => 0,
                    (_, TrainingIntensity::Low) => 3,
                    (_, TrainingIntensity::Medium) => 6,
                    (_, TrainingIntensity::High) => 10,
                }
            };

            // Recovery amount: rest days get boosted recovery (like Recovery focus)
            let recovery_base: f64 = if !is_training_day {
                7.0 * plan.bonus.physio_mult * plan.medical_facility_mult
            } else {
                match &player_focus {
                    TrainingFocus::Recovery => {
                        9.0 * plan.bonus.physio_mult * plan.medical_facility_mult
                    }
                    _ => 3.0 * plan.bonus.physio_mult * plan.medical_facility_mult,
                }
            };

            // Age, morale, and current condition all affect recovery rate.
            // Older players recover more slowly; high morale aids recovery;
            // severely fatigued players have a harder time bouncing back.
            let age = estimate_age(&player.date_of_birth);
            let age_rec = recovery_factor_from_age(age);
            let morale_rec = recovery_factor_from_morale(player.morale);
            let condition_rec = recovery_factor_from_condition(player.condition);
            let fitness_rec = recovery_factor_from_fitness(player.fitness);

            // Injured players: half base recovery, scaled by age and morale.
            // Fitness decays slowly during injury (inactive = losing sharpness).
            if player.injury.is_some() {
                let recovery = (recovery_base * 0.5 * age_rec * morale_rec * fitness_rec) as u8;
                player.condition = (player.condition + recovery).min(100);
                player.fitness = clamp_fitness(player.fitness as i16 - 1);
                continue;
            }

            // On rest days: only recovery, no attribute gains
            if !is_training_day {
                let stamina_factor = player.attributes.stamina as f64 / 100.0;
                let recovery = (recovery_base
                    * (0.5 + stamina_factor * 0.5)
                    * age_rec
                    * morale_rec
                    * condition_rec
                    * fitness_rec) as u8;
                player.condition = (player.condition + recovery).min(100);
                continue;
            }

            // Age factor for attribute gains: younger players grow faster, older players slower
            let age_factor = if age <= 21 {
                1.5
            } else if age <= 25 {
                1.2
            } else if age <= 29 {
                1.0
            } else if age <= 33 {
                0.6
            } else {
                0.3
            };

            // Base gain per attribute per session, boosted by coaching staff
            let professionalism_training_mult =
                training_effectiveness_from_professionalism(player.attributes.professionalism);
            let gain = 0.15
                * intensity_mult
                * age_factor
                * plan.bonus.coaching_mult
                * plan.bonus.specialization_mult
                * professionalism_training_mult;

            // Apply attribute gains based on player's effective focus
            apply_focus_gains(&mut player.attributes, &player_focus, gain);

            // Apply fitness changes based on training focus.
            // Physical training builds fitness; non-physical days slowly decay it if peak.
            // Recovery focus gives a tiny fitness boost.
            apply_fitness_change(
                &mut player.fitness,
                &player_focus,
                intensity_mult,
                player.attributes.professionalism,
            );

            // Apply condition: deplete from training, then recover
            player.condition = player.condition.saturating_sub(condition_cost);
            let stamina_factor = player.attributes.stamina as f64 / 100.0;
            let recovery = (recovery_base
                * (0.5 + stamina_factor * 0.5)
                * age_rec
                * morale_rec
                * condition_rec
                * fitness_rec) as u8;
            player.condition = (player.condition + recovery).min(100);

            process_development_progress(player, &player_focus, intensity_mult);
        }
    }
}

/// Apply fitness changes based on training focus.
/// Physical training builds fitness (probabilistic small gains).
/// Recovery focus gives a tiny boost. Non-physical training slowly decays high fitness.
fn apply_fitness_change(
    fitness: &mut u8,
    focus: &TrainingFocus,
    intensity_mult: f64,
    professionalism: u8,
) {
    use rand::Rng;
    let mut rng = rand::thread_rng();
    let professionalism_mult = 0.75 + (professionalism as f64 / 100.0) * 0.5;
    match focus {
        TrainingFocus::Physical => {
            // Physical training is the primary way to build fitness.
            // Higher intensity → higher gain probability.
            let gain_prob = 0.015 * intensity_mult * professionalism_mult; // 0.0075–0.0281 per session
            let roll: f64 = rng.gen_range(0.0..1.0);
            if roll < gain_prob && *fitness < 100 {
                *fitness = fitness.saturating_add(1);
            }
        }
        TrainingFocus::Recovery => {
            // Recovery days give a tiny fitness nudge.
            let roll: f64 = rng.gen_range(0.0..1.0);
            if roll < 0.05 * professionalism_mult && *fitness < 100 {
                *fitness = fitness.saturating_add(1);
            }
        }
        _ => {
            // Non-physical training: very slight decay if player is already very fit
            // (fitness above 85 needs active maintenance).
            if *fitness > 85 {
                let roll: f64 = rng.gen_range(0.0..1.0);
                if roll < 0.05 {
                    *fitness = fitness.saturating_sub(1);
                }
            }
        }
    }
}

fn training_effectiveness_from_professionalism(professionalism: u8) -> f64 {
    let normalized = (professionalism as f64 / 100.0).clamp(0.0, 1.0);
    0.8 + normalized * 0.5
}

fn learning_speed_from_professionalism(professionalism: u8) -> f64 {
    let normalized = (professionalism as f64 / 100.0).clamp(0.0, 1.0);
    0.65 + normalized * 0.9
}

fn learning_success_from_professionalism(professionalism: u8) -> f64 {
    let normalized = (professionalism as f64 / 100.0).clamp(0.0, 1.0);
    0.35 + normalized * 0.55
}

fn avg_attr(values: &[u8]) -> f64 {
    if values.is_empty() {
        return 0.0;
    }
    values.iter().map(|v| *v as f64).sum::<f64>() / (values.len() as f64 * 100.0)
}

fn trait_learning_affinity(attrs: &PlayerAttributes, trait_kind: &PlayerTrait) -> f64 {
    use PlayerTrait as T;
    match trait_kind {
        T::EarlyScanner
        | T::BlindSideAwareness
        | T::TempoManipulator
        | T::DelayedPasser
        | T::RiskCalibrator
        | T::SpaceMagnet
        | T::PressBaiter
        | T::TransitionAnticipator
        | T::OneTouchSpecialist
        | T::OutsideFootPasser => avg_attr(&[
            attrs.passing,
            attrs.vision,
            attrs.decisions,
            attrs.composure,
        ]),
        T::ToePokeFinisher
        | T::AerialRedirection
        | T::HalfVolleyComfort
        | T::NearPostHunter
        | T::BlindSideRunner
        | T::FarPostGhost
        | T::ReboundInstinct
        | T::KeeperDisruptor => avg_attr(&[
            attrs.shooting,
            attrs.positioning,
            attrs.composure,
            attrs.pace,
        ]),
        T::ContainmentSpecialist
        | T::RecoverySprinter
        | T::PassingLaneThief
        | T::BodyAngleManipulator
        | T::TacticalFouler
        | T::AerialGrappler
        | T::SecondContactWinner
        | T::LineStepTrapper => avg_attr(&[
            attrs.defending,
            attrs.tackling,
            attrs.positioning,
            attrs.decisions,
        ]),
        T::DisguisedFirstTouch
        | T::BounceTechnician
        | T::RecoveryTouch
        | T::ChannelDrifter
        | T::ContactSeller
        | T::NutmegOpportunist
        | T::BounceRoomDribbler
        | T::ChaosCreator => avg_attr(&[
            attrs.dribbling,
            attrs.agility,
            attrs.composure,
            attrs.decisions,
        ]),
        T::ReboundDirector
        | T::CrossPoker
        | T::BreakawayHypnotist
        | T::LineDictator
        | T::ThrowLauncher
        | T::PenaltyReader
        | T::TrafficCommander => avg_attr(&[
            attrs.handling,
            attrs.reflexes,
            attrs.aerial,
            attrs.decisions,
        ]),
        _ => avg_attr(&[
            attrs.decisions,
            attrs.teamwork,
            attrs.composure,
            attrs.professionalism,
        ]),
    }
}

fn trait_focus_multiplier(trait_kind: &PlayerTrait, focus: &TrainingFocus) -> f64 {
    use PlayerTrait as T;
    let is_goalkeeper_trait = matches!(
        trait_kind,
        T::ReboundDirector
            | T::CrossPoker
            | T::BreakawayHypnotist
            | T::LineDictator
            | T::ThrowLauncher
            | T::PenaltyReader
            | T::TrafficCommander
    );
    if is_goalkeeper_trait {
        return match focus {
            TrainingFocus::Defending => 1.2,
            TrainingFocus::Tactical => 1.15,
            TrainingFocus::Recovery => 0.8,
            _ => 1.0,
        };
    }

    match focus {
        TrainingFocus::Technical => 1.2,
        TrainingFocus::Tactical => 1.15,
        TrainingFocus::Attacking => 1.1,
        TrainingFocus::Defending => 1.1,
        TrainingFocus::Physical => 0.95,
        TrainingFocus::Recovery => 0.75,
    }
}

fn position_learning_affinity(attrs: &PlayerAttributes, target_position: &Position) -> f64 {
    use Position as P;
    match target_position {
        P::Goalkeeper => avg_attr(&[
            attrs.handling,
            attrs.reflexes,
            attrs.aerial,
            attrs.positioning,
            attrs.decisions,
        ]),
        P::CenterBack => avg_attr(&[
            attrs.defending,
            attrs.tackling,
            attrs.positioning,
            attrs.strength,
            attrs.aerial,
        ]),
        P::RightBack | P::LeftBack | P::RightWingBack | P::LeftWingBack => avg_attr(&[
            attrs.pace,
            attrs.stamina,
            attrs.defending,
            attrs.tackling,
            attrs.passing,
        ]),
        P::DefensiveMidfielder => avg_attr(&[
            attrs.passing,
            attrs.defending,
            attrs.tackling,
            attrs.positioning,
            attrs.decisions,
        ]),
        P::CentralMidfielder => avg_attr(&[
            attrs.passing,
            attrs.vision,
            attrs.decisions,
            attrs.stamina,
            attrs.teamwork,
        ]),
        P::AttackingMidfielder => avg_attr(&[
            attrs.passing,
            attrs.vision,
            attrs.dribbling,
            attrs.decisions,
            attrs.composure,
        ]),
        P::RightMidfielder | P::LeftMidfielder | P::RightWinger | P::LeftWinger => avg_attr(&[
            attrs.pace,
            attrs.dribbling,
            attrs.passing,
            attrs.agility,
            attrs.positioning,
        ]),
        P::Striker | P::Forward => avg_attr(&[
            attrs.shooting,
            attrs.positioning,
            attrs.pace,
            attrs.dribbling,
            attrs.composure,
        ]),
        P::Defender => avg_attr(&[
            attrs.defending,
            attrs.tackling,
            attrs.positioning,
            attrs.strength,
            attrs.aerial,
        ]),
        P::Midfielder => avg_attr(&[
            attrs.passing,
            attrs.vision,
            attrs.decisions,
            attrs.stamina,
            attrs.teamwork,
        ]),
    }
}

fn position_focus_multiplier(target_position: &Position, focus: &TrainingFocus) -> f64 {
    let grouped = target_position.to_group_position();
    match grouped {
        Position::Goalkeeper => match focus {
            TrainingFocus::Defending | TrainingFocus::Tactical => 1.2,
            TrainingFocus::Recovery => 0.85,
            _ => 1.0,
        },
        Position::Defender => match focus {
            TrainingFocus::Defending => 1.2,
            TrainingFocus::Tactical => 1.1,
            TrainingFocus::Recovery => 0.85,
            _ => 1.0,
        },
        Position::Midfielder => match focus {
            TrainingFocus::Tactical => 1.2,
            TrainingFocus::Technical => 1.15,
            TrainingFocus::Recovery => 0.85,
            _ => 1.0,
        },
        Position::Forward => match focus {
            TrainingFocus::Attacking => 1.2,
            TrainingFocus::Technical => 1.1,
            TrainingFocus::Recovery => 0.85,
            _ => 1.0,
        },
        _ => 1.0,
    }
}

fn process_development_progress(player: &mut Player, focus: &TrainingFocus, intensity_mult: f64) {
    use rand::Rng;

    let professionalism_speed =
        learning_speed_from_professionalism(player.attributes.professionalism);
    let professionalism_success =
        learning_success_from_professionalism(player.attributes.professionalism);
    let mut rng = rand::thread_rng();

    if let Some(mut goal) = player.development_plan.trait_goal.clone() {
        if !trait_is_learnable(&goal.target_trait) {
            player.development_plan.trait_goal = None;
        } else {
            let focus_mult = trait_focus_multiplier(&goal.target_trait, focus);
            let affinity = trait_learning_affinity(&player.attributes, &goal.target_trait);
            let increment =
                0.45 * professionalism_speed * focus_mult * (0.55 + affinity) * intensity_mult;
            goal.progress = (goal.progress + increment as f32).min(200.0);
            goal.completed_sessions = goal.completed_sessions.saturating_add(1);

            if goal.progress >= 100.0 {
                let mut success_chance =
                    (0.2 + professionalism_success * 0.55 + affinity * 0.25).clamp(0.1, 0.97);
                if matches!(goal.mode, TraitTrainingMode::Unlearn) {
                    success_chance = (success_chance + 0.05).clamp(0.1, 0.98);
                }
                if rng.gen_range(0.0..1.0) < success_chance {
                    match goal.mode {
                        TraitTrainingMode::Learn => {
                            if !player.traits.contains(&goal.target_trait) {
                                player.traits.push(goal.target_trait.clone());
                            }
                        }
                        TraitTrainingMode::Unlearn => {
                            player
                                .traits
                                .retain(|trait_kind| trait_kind != &goal.target_trait);
                        }
                    }
                    player.development_plan.trait_goal = None;
                } else {
                    goal.progress = 70.0;
                    player.development_plan.trait_goal = Some(goal);
                }
            } else {
                player.development_plan.trait_goal = Some(goal);
            }
        }
    }

    if let Some(mut goal) = player.development_plan.position_goal.clone() {
        let already_known = player.natural_position == goal.target_position
            || player
                .alternate_positions
                .iter()
                .any(|position| position == &goal.target_position);

        if already_known {
            player.development_plan.position_goal = None;
            return;
        }

        let focus_mult = position_focus_multiplier(&goal.target_position, focus);
        let affinity = position_learning_affinity(&player.attributes, &goal.target_position);
        let increment =
            0.35 * professionalism_speed * focus_mult * (0.5 + affinity) * intensity_mult;
        goal.progress = (goal.progress + increment as f32).min(200.0);
        goal.completed_sessions = goal.completed_sessions.saturating_add(1);

        if goal.progress >= 100.0 {
            let success_chance =
                (0.18 + professionalism_success * 0.6 + affinity * 0.22).clamp(0.08, 0.95);
            if rng.gen_range(0.0..1.0) < success_chance {
                player
                    .alternate_positions
                    .push(goal.target_position.clone());
                player.development_plan.position_goal = None;
            } else {
                goal.progress = 72.0;
                player.development_plan.position_goal = Some(goal);
            }
        } else {
            player.development_plan.position_goal = Some(goal);
        }
    }
}

fn try_gain(current: &mut u8, gain: f64) {
    use rand::Rng;
    if *current >= 99 {
        return;
    }
    let mut rng = rand::thread_rng();
    let roll: f64 = rng.gen_range(0.0..1.0);
    if roll < gain {
        *current = (*current + 1).min(99);
    }
}

/// Apply attribute gains based on training focus.
fn apply_focus_gains(
    attrs: &mut domain::player::PlayerAttributes,
    focus: &TrainingFocus,
    gain: f64,
) {
    match focus {
        TrainingFocus::Physical => {
            try_gain(&mut attrs.pace, gain);
            try_gain(&mut attrs.stamina, gain);
            try_gain(&mut attrs.strength, gain);
            try_gain(&mut attrs.agility, gain);
        }
        TrainingFocus::Technical => {
            try_gain(&mut attrs.passing, gain);
            try_gain(&mut attrs.shooting, gain);
            try_gain(&mut attrs.dribbling, gain);
        }
        TrainingFocus::Tactical => {
            try_gain(&mut attrs.positioning, gain);
            try_gain(&mut attrs.vision, gain);
            try_gain(&mut attrs.decisions, gain);
            try_gain(&mut attrs.composure, gain);
        }
        TrainingFocus::Defending => {
            try_gain(&mut attrs.tackling, gain);
            try_gain(&mut attrs.defending, gain);
            try_gain(&mut attrs.strength, gain * 0.5);
            try_gain(&mut attrs.positioning, gain * 0.5);
        }
        TrainingFocus::Attacking => {
            try_gain(&mut attrs.shooting, gain);
            try_gain(&mut attrs.dribbling, gain);
            try_gain(&mut attrs.pace, gain * 0.5);
        }
        TrainingFocus::Recovery => {
            // No attribute gains on recovery days
        }
    }
}

/// Estimate player age from date_of_birth string ("YYYY-MM-DD").
fn estimate_age(dob: &str) -> u32 {
    let parts: Vec<&str> = dob.split('-').collect();
    if parts.is_empty() {
        return 25; // fallback
    }
    let birth_year: u32 = parts[0].parse().unwrap_or(2000);
    // Use a rough estimate — the game clock year would be ideal but
    // this is close enough for growth factor purposes.
    let current_year: u32 = 2025;
    current_year.saturating_sub(birth_year)
}

/// Recovery multiplier from age: younger players bounce back faster.
fn recovery_factor_from_age(age: u32) -> f64 {
    if age <= 21 {
        1.10
    } else if age <= 25 {
        1.05
    } else if age <= 29 {
        1.00
    } else if age <= 33 {
        0.85
    } else {
        0.70
    }
}

/// Recovery multiplier from morale: players in good spirits recover better.
fn recovery_factor_from_morale(morale: u8) -> f64 {
    if morale >= 70 {
        1.10
    } else if morale >= 40 {
        1.00
    } else {
        0.90
    }
}

/// Recovery multiplier from current condition: severely fatigued players recover more slowly.
fn recovery_factor_from_condition(condition: u8) -> f64 {
    if condition < 30 {
        0.80
    } else if condition < 50 {
        0.90
    } else {
        1.00
    }
}

/// Recovery multiplier from fitness: fitter players recover condition faster.
fn recovery_factor_from_fitness(fitness: u8) -> f64 {
    if fitness < 30 {
        0.75
    } else if fitness < 50 {
        0.88
    } else if fitness < 70 {
        1.00
    } else if fitness < 90 {
        1.12
    } else {
        1.20
    }
}

/// Clamp a fitness value to 0–100.
fn clamp_fitness(val: i16) -> u8 {
    val.clamp(0, 100) as u8
}
