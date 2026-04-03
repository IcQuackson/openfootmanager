use rand::Rng;

use crate::event::{EventType, MatchEvent};
use crate::shared::{
    ActionIntent, IntentContext, TraitContext, attack_modifier, buildup_modifier,
    choose_attacking_intent, choose_buildup_intent, choose_defensive_intent,
    choose_goalkeeper_distribution_intent, choose_midfield_intent, defense_modifier,
    formation_attack_modifier, formation_buildup_modifier, formation_midfield_modifier,
    formation_rest_defense_modifier, home_mod, midfield_attack_modifier, midfield_defense_modifier,
    press_modifier, role_carry_bias, role_clearance_bias, role_pass_bias, role_press_bias,
    role_shot_bias, trait_bonus, trait_carry_bias, trait_clearance_bias, trait_pass_bias,
    trait_press_bias, trait_shot_bias, trait_strength, transition_progress_chance,
};
use crate::types::{Position, Side, Zone};

use super::MatchContext;
use super::fouls::maybe_foul;
use super::snap_player;

pub(super) fn resolve_action<R: Rng>(ctx: &mut MatchContext, minute: u8, rng: &mut R) {
    let att_side = ctx.possession;
    let def_side = att_side.opposite();
    let zone = ctx.ball_zone;

    if zone.is_box_for(att_side) {
        resolve_shot(ctx, minute, att_side, rng);
    } else if zone == Zone::attacking_third(att_side) {
        resolve_attacking_third(ctx, minute, att_side, def_side, rng);
    } else if zone == Zone::Midfield {
        resolve_midfield(ctx, minute, att_side, def_side, rng);
    } else {
        resolve_buildup(ctx, minute, att_side, def_side, rng);
    }
}

fn resolve_buildup<R: Rng>(
    ctx: &mut MatchContext,
    minute: u8,
    att_side: Side,
    def_side: Side,
    rng: &mut R,
) {
    let passer = snap_player(ctx, att_side, Position::Defender, rng);
    let presser = snap_player(ctx, def_side, Position::Midfielder, rng);
    let press = effective_press(ctx, def_side);
    let pass_skill = (passer.passing as f64
        + passer.vision as f64
        + passer.composure as f64
        + passer.teamwork as f64)
        / 4.0
        * trait_bonus(&passer, TraitContext::Passing)
        * trait_pass_bias(&passer)
        * role_pass_bias(passer.role)
        * buildup_modifier(
            ctx.team(att_side).play_style,
            ctx.transition_side == Some(att_side),
        )
        * formation_buildup_modifier(&ctx.team(att_side).formation);
    let ball_zone = ctx.ball_zone;
    let intent_context =
        intent_context(ctx, minute, att_side, press > (pass_skill * 0.95).max(52.0));
    let intent = choose_buildup_intent(&passer, ctx.team(att_side).play_style, intent_context, rng);

    let (intent_modifier, success_zone, keeps_transition) = match intent {
        ActionIntent::SafeRecyclePass => (1.08, Zone::Midfield, false),
        ActionIntent::SplitLinePass => (
            0.95,
            if intent_context.in_transition || rng.gen_range(0.0..1.0f64) < 0.28 {
                Zone::attacking_third(att_side)
            } else {
                Zone::Midfield
            },
            intent_context.in_transition,
        ),
        ActionIntent::BaitPressTouch => (
            0.90,
            if intent_context.under_pressure || intent_context.in_transition {
                Zone::attacking_third(att_side)
            } else {
                Zone::Midfield
            },
            true,
        ),
        _ => (1.0, Zone::Midfield, false),
    };

    let success_chance =
        (pass_skill * intent_modifier * 1.3) / ((pass_skill * intent_modifier * 1.3) + press);

    if rng.gen_range(0.0..1.0f64) < success_chance {
        if intent == ActionIntent::BaitPressTouch {
            ctx.emit(
                MatchEvent::new(minute, EventType::Dribble, att_side, ball_zone)
                    .with_player(&passer.id),
            );
        }
        ctx.emit(
            MatchEvent::new(minute, EventType::PassCompleted, att_side, ball_zone)
                .with_player(&passer.id),
        );
        ctx.ball_zone = success_zone;
        if keeps_transition {
            ctx.transition_side = Some(att_side);
        }
    } else {
        if intent == ActionIntent::BaitPressTouch {
            ctx.emit(
                MatchEvent::new(minute, EventType::DribbleTackled, att_side, ball_zone)
                    .with_player(&passer.id)
                    .with_secondary(&presser.id),
            );
            ctx.emit(
                MatchEvent::new(minute, EventType::Tackle, def_side, ball_zone)
                    .with_player(&presser.id),
            );
        } else {
            ctx.emit(
                MatchEvent::new(minute, EventType::PassIntercepted, att_side, ball_zone)
                    .with_player(&passer.id),
            );
            ctx.emit(
                MatchEvent::new(minute, EventType::Interception, def_side, ball_zone)
                    .with_player(&presser.id),
            );
        }
        ctx.possession = def_side;
        ctx.transition_side = Some(def_side);
        ctx.ball_zone = if rng.gen_range(0.0..1.0f64)
            < transition_progress_chance(ctx.team(def_side).play_style)
        {
            Zone::Midfield.advance_towards(def_side)
        } else {
            Zone::Midfield
        };
    }
}

fn resolve_midfield<R: Rng>(
    ctx: &mut MatchContext,
    minute: u8,
    att_side: Side,
    def_side: Side,
    rng: &mut R,
) {
    let attacker = snap_player(ctx, att_side, Position::Midfielder, rng);
    let defender = snap_player(ctx, def_side, Position::Midfielder, rng);
    let base_context = intent_context(ctx, minute, att_side, false);
    let attacker_intent =
        choose_midfield_intent(&attacker, ctx.team(att_side).play_style, base_context, rng);
    let defender_intent =
        choose_defensive_intent(&defender, ctx.team(def_side).play_style, base_context, rng);

    let att_rating = match attacker_intent {
        ActionIntent::OneTouchCombination
        | ActionIntent::DelayedRelease
        | ActionIntent::ThirdManLayoff => {
            (attacker.passing as f64
                + attacker.vision as f64
                + attacker.decisions as f64
                + attacker.teamwork as f64)
                / 4.0
                * trait_bonus(&attacker, TraitContext::Passing)
                * trait_pass_bias(&attacker)
                * role_pass_bias(attacker.role)
        }
        ActionIntent::ProgressiveCarry | ActionIntent::PressEscapeTurn => {
            (attacker.dribbling as f64
                + attacker.pace as f64
                + attacker.agility as f64
                + attacker.composure as f64)
                / 4.0
                * trait_bonus(&attacker, TraitContext::Dribbling)
                * trait_carry_bias(&attacker)
                * role_carry_bias(attacker.role)
        }
        _ => 55.0,
    } * trait_bonus(&attacker, TraitContext::Midfield);

    let def_rating = (defender.tackling as f64
        + defender.positioning as f64
        + defender.decisions as f64
        + defender.teamwork as f64)
        / 4.0
        * trait_bonus(&defender, TraitContext::Tackling)
        * trait_press_bias(&defender)
        * role_press_bias(defender.role)
        * defensive_intent_modifier(defender_intent, &defender, base_context);

    let att_mod = midfield_attack_modifier(
        ctx.team(att_side).play_style,
        ctx.transition_side == Some(att_side),
    ) * formation_midfield_modifier(&ctx.team(att_side).formation);
    let def_mod = midfield_defense_modifier(ctx.team(def_side).play_style)
        * ((formation_midfield_modifier(&ctx.team(def_side).formation) * 0.6)
            + (formation_rest_defense_modifier(&ctx.team(def_side).formation) * 0.4));
    let att_eff = att_rating * att_mod * home_mod(att_side, ctx.config);
    let def_eff = def_rating * def_mod * home_mod(def_side, ctx.config);
    let success = att_eff / (att_eff + def_eff);

    if rng.gen_range(0.0..1.0f64) < success {
        let success_event = match attacker_intent {
            ActionIntent::ProgressiveCarry | ActionIntent::PressEscapeTurn => EventType::Dribble,
            _ => EventType::PassCompleted,
        };
        ctx.emit(
            MatchEvent::new(minute, success_event, att_side, Zone::Midfield)
                .with_player(&attacker.id),
        );

        let next_zone = match attacker_intent {
            ActionIntent::ThirdManLayoff if base_context.in_transition => {
                Zone::attacking_box(att_side)
            }
            ActionIntent::PressEscapeTurn if rng.gen_range(0.0..1.0f64) < 0.22 => {
                Zone::attacking_box(att_side)
            }
            _ => Zone::attacking_third(att_side),
        };
        ctx.ball_zone = next_zone;
        if matches!(
            attacker_intent,
            ActionIntent::ThirdManLayoff | ActionIntent::PressEscapeTurn
        ) {
            ctx.transition_side = Some(att_side);
        }
    } else {
        handle_midfield_defensive_win(
            ctx,
            minute,
            att_side,
            def_side,
            &attacker,
            &defender,
            attacker_intent,
            defender_intent,
            rng,
        );
    }
}

fn resolve_attacking_third<R: Rng>(
    ctx: &mut MatchContext,
    minute: u8,
    att_side: Side,
    def_side: Side,
    rng: &mut R,
) {
    let attacker = snap_player(ctx, att_side, Position::Forward, rng);
    let defender = snap_player(ctx, def_side, Position::Defender, rng);
    let base_context = intent_context(ctx, minute, att_side, false);
    let attacker_intent =
        choose_attacking_intent(&attacker, ctx.team(att_side).play_style, base_context, rng);
    let defender_intent =
        choose_defensive_intent(&defender, ctx.team(def_side).play_style, base_context, rng);

    let att_rating = match attacker_intent {
        ActionIntent::LinkPlaySlip | ActionIntent::LateBoxDelivery => {
            (attacker.passing as f64
                + attacker.vision as f64
                + attacker.teamwork as f64
                + attacker.composure as f64)
                / 4.0
                * trait_bonus(&attacker, TraitContext::Passing)
                * trait_pass_bias(&attacker)
                * role_pass_bias(attacker.role)
        }
        ActionIntent::BlindSideRun | ActionIntent::NearPostAttack => {
            (attacker.pace as f64
                + attacker.positioning as f64
                + attacker.agility as f64
                + attacker.shooting as f64)
                / 4.0
                * trait_bonus(&attacker, TraitContext::Shooting)
                * trait_shot_bias(&attacker)
                * role_shot_bias(attacker.role)
        }
        ActionIntent::DirectDribble => {
            (attacker.dribbling as f64
                + attacker.pace as f64
                + attacker.agility as f64
                + attacker.composure as f64)
                / 4.0
                * trait_bonus(&attacker, TraitContext::Dribbling)
                * trait_carry_bias(&attacker)
                * role_carry_bias(attacker.role)
        }
        _ => 55.0,
    };
    let def_rating = (defender.defending as f64
        + defender.tackling as f64
        + defender.positioning as f64
        + defender.aerial as f64)
        / 4.0
        * trait_bonus(&defender, TraitContext::Tackling)
        * trait_clearance_bias(&defender)
        * role_clearance_bias(defender.role)
        * defensive_intent_modifier(defender_intent, &defender, base_context);

    let att_mod = attack_modifier(
        ctx.team(att_side).play_style,
        ctx.transition_side == Some(att_side),
    ) * formation_attack_modifier(&ctx.team(att_side).formation);
    let def_mod = defense_modifier(ctx.team(def_side).play_style)
        * formation_rest_defense_modifier(&ctx.team(def_side).formation);
    let att_eff = att_rating * att_mod * home_mod(att_side, ctx.config);
    let def_eff = def_rating * def_mod * home_mod(def_side, ctx.config);
    let success = att_eff / (att_eff + def_eff);
    let zone = Zone::attacking_third(att_side);

    if rng.gen_range(0.0..1.0f64) < success {
        let success_event = match attacker_intent {
            ActionIntent::LateBoxDelivery => EventType::Cross,
            ActionIntent::LinkPlaySlip => EventType::PassCompleted,
            _ => EventType::Dribble,
        };
        ctx.emit(MatchEvent::new(minute, success_event, att_side, zone).with_player(&attacker.id));
        ctx.ball_zone = Zone::attacking_box(att_side);
        if matches!(
            attacker_intent,
            ActionIntent::BlindSideRun
                | ActionIntent::NearPostAttack
                | ActionIntent::LateBoxDelivery
        ) {
            ctx.transition_side = Some(att_side);
        }
    } else {
        handle_attacking_third_defensive_win(
            ctx,
            minute,
            att_side,
            def_side,
            &attacker,
            &defender,
            attacker_intent,
            defender_intent,
            rng,
        );
    }
}

fn resolve_shot<R: Rng>(ctx: &mut MatchContext, minute: u8, att_side: Side, rng: &mut R) {
    let def_side = att_side.opposite();
    let shooter = snap_player(ctx, att_side, Position::Forward, rng);
    let assister = snap_player(ctx, att_side, Position::Midfielder, rng);
    let goalkeeper = snap_player(ctx, def_side, Position::Goalkeeper, rng);

    let shoot_rating =
        (shooter.shooting as f64 + shooter.composure as f64 + shooter.decisions as f64) / 3.0
            * trait_bonus(&shooter, TraitContext::Shooting)
            * trait_shot_bias(&shooter)
            * role_shot_bias(shooter.role);
    let gk_rating =
        (goalkeeper.handling as f64 + goalkeeper.reflexes as f64 + goalkeeper.positioning as f64)
            / 3.0
            * trait_bonus(&goalkeeper, TraitContext::Goalkeeping);

    let accuracy = (ctx.config.shot_accuracy_base
        + (shoot_rating - 50.0) / 200.0
        + (formation_attack_modifier(&ctx.team(att_side).formation)
            - formation_rest_defense_modifier(&ctx.team(def_side).formation))
            * 0.04
        + trait_strength(&shooter, "BlindSideRunner") * 0.03
        + trait_strength(&shooter, "NearPostHunter") * 0.02)
        .clamp(0.15, 0.88);
    let zone = Zone::attacking_box(att_side);

    if rng.gen_range(0.0..1.0f64) > accuracy {
        if rng.gen_range(0.0..1.0f64) < 0.38 {
            ctx.emit(
                MatchEvent::new(minute, EventType::ShotBlocked, att_side, zone)
                    .with_player(&shooter.id),
            );
            if maybe_rebound_follow_up(ctx, minute, att_side, def_side, rng) {
                return;
            }
            if rng.gen_range(0.0..1.0f64) < 0.28 {
                ctx.emit(MatchEvent::new(minute, EventType::Corner, att_side, zone));
                ctx.ball_zone = Zone::attacking_box(att_side);
                return;
            }
        } else {
            ctx.emit(
                MatchEvent::new(minute, EventType::ShotOffTarget, att_side, zone)
                    .with_player(&shooter.id),
            );
            if rng.gen_range(0.0..1.0f64) < 0.55 {
                ctx.emit(MatchEvent::new(minute, EventType::GoalKick, def_side, zone));
            }
        }
        ctx.ball_zone = Zone::defensive_third(def_side);
        ctx.possession = def_side;
        return;
    }

    let conversion =
        (ctx.config.goal_conversion_base + (shoot_rating - gk_rating) / 150.0).clamp(0.10, 0.70);

    if rng.gen_range(0.0..1.0f64) < conversion {
        ctx.emit(
            MatchEvent::new(minute, EventType::Goal, att_side, zone)
                .with_player(&shooter.id)
                .with_secondary(&assister.id),
        );
        ctx.add_goal(att_side);
        ctx.ball_zone = Zone::Midfield;
        ctx.possession = def_side;
    } else {
        ctx.emit(
            MatchEvent::new(minute, EventType::ShotSaved, att_side, zone).with_player(&shooter.id),
        );
        if maybe_rebound_follow_up(ctx, minute, att_side, def_side, rng) {
            return;
        }
        settle_after_goalkeeper_control(ctx, minute, def_side, &goalkeeper, rng);
    }
}

fn handle_midfield_defensive_win<R: Rng>(
    ctx: &mut MatchContext,
    minute: u8,
    att_side: Side,
    def_side: Side,
    attacker: &crate::shared::PlayerSnap,
    defender: &crate::shared::PlayerSnap,
    attacker_intent: ActionIntent,
    defender_intent: ActionIntent,
    rng: &mut R,
) {
    match defender_intent {
        ActionIntent::StepInInterception => {
            if matches!(
                attacker_intent,
                ActionIntent::OneTouchCombination
                    | ActionIntent::DelayedRelease
                    | ActionIntent::ThirdManLayoff
            ) {
                ctx.emit(
                    MatchEvent::new(minute, EventType::PassIntercepted, att_side, Zone::Midfield)
                        .with_player(&attacker.id),
                );
            }
            ctx.emit(
                MatchEvent::new(minute, EventType::Interception, def_side, Zone::Midfield)
                    .with_player(&defender.id),
            );
        }
        ActionIntent::TacticalFoulStop => {
            ctx.emit(
                MatchEvent::new(minute, EventType::Tackle, def_side, Zone::Midfield)
                    .with_player(&defender.id),
            );
            maybe_foul(
                ctx,
                minute,
                def_side,
                attacker,
                defender,
                Zone::Midfield,
                rng,
            );
        }
        ActionIntent::ContainAndShowWide => {
            ctx.emit(
                MatchEvent::new(minute, EventType::Tackle, def_side, Zone::Midfield)
                    .with_player(&defender.id),
            );
        }
        ActionIntent::AerialClearance => {
            ctx.emit(
                MatchEvent::new(minute, EventType::Clearance, def_side, Zone::Midfield)
                    .with_player(&defender.id),
            );
        }
        _ => {}
    }

    ctx.possession = def_side;
    ctx.transition_side = Some(def_side);
    ctx.ball_zone =
        if rng.gen_range(0.0..1.0f64) < transition_progress_chance(ctx.team(def_side).play_style) {
            Zone::attacking_third(def_side)
        } else {
            Zone::Midfield
        };
}

fn handle_attacking_third_defensive_win<R: Rng>(
    ctx: &mut MatchContext,
    minute: u8,
    att_side: Side,
    def_side: Side,
    attacker: &crate::shared::PlayerSnap,
    defender: &crate::shared::PlayerSnap,
    attacker_intent: ActionIntent,
    defender_intent: ActionIntent,
    rng: &mut R,
) {
    let zone = Zone::attacking_third(att_side);
    match defender_intent {
        ActionIntent::StepInInterception
            if matches!(
                attacker_intent,
                ActionIntent::LinkPlaySlip | ActionIntent::LateBoxDelivery
            ) =>
        {
            let event_type = if attacker_intent == ActionIntent::LateBoxDelivery {
                EventType::Cross
            } else {
                EventType::PassIntercepted
            };
            ctx.emit(MatchEvent::new(minute, event_type, att_side, zone).with_player(&attacker.id));
            ctx.emit(
                MatchEvent::new(minute, EventType::Interception, def_side, zone)
                    .with_player(&defender.id),
            );
        }
        ActionIntent::TacticalFoulStop => {
            ctx.emit(
                MatchEvent::new(minute, EventType::Tackle, def_side, zone)
                    .with_player(&defender.id),
            );
            maybe_foul(ctx, minute, def_side, attacker, defender, zone, rng);
        }
        ActionIntent::ContainAndShowWide => {
            ctx.emit(
                MatchEvent::new(minute, EventType::DribbleTackled, att_side, zone)
                    .with_player(&attacker.id)
                    .with_secondary(&defender.id),
            );
            ctx.emit(
                MatchEvent::new(minute, EventType::Tackle, def_side, zone)
                    .with_player(&defender.id),
            );
        }
        _ => {
            ctx.emit(
                MatchEvent::new(minute, EventType::Clearance, def_side, zone)
                    .with_player(&defender.id),
            );
        }
    }

    if rng.gen_range(0.0..1.0f64)
        < if attacker_intent == ActionIntent::LateBoxDelivery {
            0.32
        } else {
            0.24
        }
    {
        ctx.emit(MatchEvent::new(minute, EventType::Corner, att_side, zone));
        if rng.gen_range(0.0..1.0f64) < 0.30 {
            ctx.ball_zone = Zone::attacking_box(att_side);
            return;
        }
    }

    ctx.possession = def_side;
    ctx.transition_side = Some(def_side);
    ctx.ball_zone = Zone::defensive_third(def_side);
}

fn maybe_rebound_follow_up<R: Rng>(
    ctx: &mut MatchContext,
    minute: u8,
    att_side: Side,
    def_side: Side,
    rng: &mut R,
) -> bool {
    let rebounder = snap_player(ctx, att_side, Position::Forward, rng);
    let rebound_window = 0.08
        + trait_strength(&rebounder, "ReboundInstinct") * 0.28
        + trait_strength(&rebounder, "SecondBallPredator") * 0.18
        + trait_strength(&rebounder, "LateBoxArriver") * 0.12
        + trait_strength(&rebounder, "FarPostGhost") * 0.08;
    if rng.gen_range(0.0..1.0f64) >= rebound_window.clamp(0.05, 0.42) {
        return false;
    }

    let goalkeeper = snap_player(ctx, def_side, Position::Goalkeeper, rng);
    let finish_rating = (rebounder.shooting as f64
        + rebounder.positioning as f64
        + rebounder.composure as f64
        + rebounder.aggression as f64)
        / 4.0
        * (1.0 + trait_strength(&rebounder, "ReboundInstinct") * 0.12)
        * trait_shot_bias(&rebounder)
        * role_shot_bias(rebounder.role);
    let gk_rating =
        (goalkeeper.handling as f64 + goalkeeper.reflexes as f64 + goalkeeper.positioning as f64)
            / 3.0
            * trait_bonus(&goalkeeper, TraitContext::Goalkeeping);
    let zone = Zone::attacking_box(att_side);

    if rng.gen_range(0.0..1.0f64) < (0.22 + (finish_rating - gk_rating) / 220.0).clamp(0.08, 0.55) {
        ctx.emit(
            MatchEvent::new(minute, EventType::Goal, att_side, zone).with_player(&rebounder.id),
        );
        ctx.add_goal(att_side);
        ctx.ball_zone = Zone::Midfield;
        ctx.possession = def_side;
        return true;
    }

    if rng.gen_range(0.0..1.0f64) < 0.58 {
        ctx.emit(
            MatchEvent::new(minute, EventType::ShotSaved, att_side, zone)
                .with_player(&rebounder.id),
        );
        settle_after_goalkeeper_control(ctx, minute, def_side, &goalkeeper, rng);
    } else {
        ctx.emit(
            MatchEvent::new(minute, EventType::ShotOffTarget, att_side, zone)
                .with_player(&rebounder.id),
        );
        ctx.emit(MatchEvent::new(minute, EventType::GoalKick, def_side, zone));
        ctx.ball_zone = Zone::defensive_third(def_side);
        ctx.possession = def_side;
    }

    true
}

fn settle_after_goalkeeper_control<R: Rng>(
    ctx: &mut MatchContext,
    minute: u8,
    possession_side: Side,
    goalkeeper: &crate::shared::PlayerSnap,
    rng: &mut R,
) {
    let distribution_context = intent_context(ctx, minute, possession_side, false);
    let distribution_intent = choose_goalkeeper_distribution_intent(
        goalkeeper,
        ctx.team(possession_side).play_style,
        IntentContext {
            in_transition: true,
            chasing_game: distribution_context.chasing_game,
            protecting_lead: distribution_context.protecting_lead,
            late_game: distribution_context.late_game,
            under_pressure: false,
        },
        rng,
    );

    ctx.possession = possession_side;
    match distribution_intent {
        ActionIntent::LaunchThrow => {
            ctx.emit(
                MatchEvent::new(
                    minute,
                    EventType::PassCompleted,
                    possession_side,
                    Zone::defensive_third(possession_side),
                )
                .with_player(&goalkeeper.id),
            );
            ctx.transition_side = Some(possession_side);
            ctx.ball_zone = if rng.gen_range(0.0..1.0f64)
                < transition_progress_chance(ctx.team(possession_side).play_style) + 0.12
            {
                Zone::attacking_third(possession_side)
            } else {
                Zone::Midfield
            };
        }
        _ => {
            ctx.ball_zone = Zone::defensive_third(possession_side);
        }
    }
}

fn defensive_intent_modifier(
    intent: ActionIntent,
    defender: &crate::shared::PlayerSnap,
    context: IntentContext,
) -> f64 {
    match intent {
        ActionIntent::StepInInterception => {
            1.0 + trait_strength(defender, "PassingLaneThief") * 0.12
        }
        ActionIntent::ContainAndShowWide => {
            1.0 + trait_strength(defender, "ContainmentSpecialist") * 0.10
                + if context.protecting_lead { 0.04 } else { 0.0 }
        }
        ActionIntent::TacticalFoulStop => 1.0 + trait_strength(defender, "TacticalFouler") * 0.08,
        ActionIntent::AerialClearance => 1.0 + trait_strength(defender, "AerialGrappler") * 0.10,
        _ => 1.0,
    }
}

fn intent_context(
    ctx: &MatchContext,
    minute: u8,
    side: Side,
    under_pressure: bool,
) -> IntentContext {
    let side_score = match side {
        Side::Home => ctx.home_score,
        Side::Away => ctx.away_score,
    };
    let opp_score = match side.opposite() {
        Side::Home => ctx.home_score,
        Side::Away => ctx.away_score,
    };

    IntentContext {
        under_pressure,
        in_transition: ctx.transition_side == Some(side),
        late_game: minute >= 75,
        protecting_lead: side_score > opp_score && minute >= 60,
        chasing_game: side_score < opp_score && minute >= 55,
    }
}

pub(super) fn effective_midfield(ctx: &MatchContext, side: Side) -> f64 {
    let base = ctx.team(side).midfield_rating();
    let modifier = midfield_attack_modifier(ctx.team(side).play_style, false)
        * formation_midfield_modifier(&ctx.team(side).formation);
    base * modifier * home_mod(side, ctx.config)
}

fn effective_press(ctx: &MatchContext, pressing_side: Side) -> f64 {
    let team = ctx.team(pressing_side);
    let base = team.position_attr_avg(Position::Midfielder, |p| {
        ((p.stamina as u16 + p.tackling as u16 + p.pace as u16) / 3) as u8
    });
    let modifier = press_modifier(team.play_style)
        * ((formation_midfield_modifier(&team.formation) * 0.65)
            + (formation_rest_defense_modifier(&team.formation) * 0.35));
    base * modifier * home_mod(pressing_side, ctx.config)
}
