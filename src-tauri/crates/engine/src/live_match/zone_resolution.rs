use rand::Rng;

use crate::event::{EventType, MatchEvent};
use crate::shared::{
    ActionIntent, IntentContext, PlayerSnap, TraitContext, attack_modifier, buildup_modifier,
    choose_attacking_intent, choose_buildup_intent, choose_defensive_intent,
    choose_goalkeeper_distribution_intent, choose_midfield_intent, defense_modifier,
    formation_attack_modifier, formation_buildup_modifier, formation_midfield_modifier,
    formation_rest_defense_modifier, midfield_attack_modifier, midfield_defense_modifier,
    role_carry_bias, role_clearance_bias, role_pass_bias, role_press_bias, role_shot_bias,
    trait_bonus, trait_carry_bias, trait_clearance_bias, trait_pass_bias, trait_press_bias,
    trait_shot_bias, trait_strength, transition_progress_chance,
};
use crate::types::{Position, Side, Zone};

use super::LiveMatchState;

impl LiveMatchState {
    pub(super) fn resolve_action<R: Rng>(&mut self, minute: u8, rng: &mut R) -> Vec<MatchEvent> {
        let att_side = self.possession;
        let def_side = att_side.opposite();
        let zone = self.ball_zone;

        if zone.is_box_for(att_side) {
            self.resolve_shot(minute, att_side, rng)
        } else if zone == Zone::attacking_third(att_side) {
            self.resolve_attacking_third(minute, att_side, def_side, rng)
        } else if zone == Zone::Midfield {
            self.resolve_midfield(minute, att_side, def_side, rng)
        } else {
            self.resolve_buildup(minute, att_side, def_side, rng)
        }
    }

    fn resolve_buildup<R: Rng>(
        &mut self,
        minute: u8,
        att_side: Side,
        def_side: Side,
        rng: &mut R,
    ) -> Vec<MatchEvent> {
        let mut events = Vec::new();
        let passer = self.snap_player(att_side, Position::Defender, rng);
        let presser = self.snap_player(def_side, Position::Midfielder, rng);
        let pass_raw = (passer.passing as f64
            + passer.vision as f64
            + passer.composure as f64
            + passer.teamwork as f64)
            / 4.0;
        let press = self.effective_press(def_side);
        let pass_skill = self.condition_adjusted_skill(&passer.id, pass_raw)
            * trait_bonus(&passer, TraitContext::Passing)
            * trait_pass_bias(&passer)
            * role_pass_bias(passer.role)
            * buildup_modifier(
                self.team_ref(att_side).play_style,
                self.transition_side == Some(att_side),
            )
            * formation_buildup_modifier(&self.team_ref(att_side).formation);
        let ball_zone = self.ball_zone;
        let intent_context =
            self.intent_context(minute, att_side, press > (pass_skill * 0.95).max(52.0));
        let intent = choose_buildup_intent(
            &passer,
            self.team_ref(att_side).play_style,
            intent_context,
            rng,
        );

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
                let evt = MatchEvent::new(minute, EventType::Dribble, att_side, ball_zone)
                    .with_player(&passer.id);
                self.events.push(evt.clone());
                events.push(evt);
            }
            let evt = MatchEvent::new(minute, EventType::PassCompleted, att_side, ball_zone)
                .with_player(&passer.id);
            self.events.push(evt.clone());
            events.push(evt);
            self.ball_zone = success_zone;
            if keeps_transition {
                self.transition_side = Some(att_side);
            }
        } else {
            if intent == ActionIntent::BaitPressTouch {
                let evt1 = MatchEvent::new(minute, EventType::DribbleTackled, att_side, ball_zone)
                    .with_player(&passer.id)
                    .with_secondary(&presser.id);
                let evt2 = MatchEvent::new(minute, EventType::Tackle, def_side, ball_zone)
                    .with_player(&presser.id);
                self.events.push(evt1.clone());
                self.events.push(evt2.clone());
                events.push(evt1);
                events.push(evt2);
            } else {
                let evt1 = MatchEvent::new(minute, EventType::PassIntercepted, att_side, ball_zone)
                    .with_player(&passer.id);
                let evt2 = MatchEvent::new(minute, EventType::Interception, def_side, ball_zone)
                    .with_player(&presser.id);
                self.events.push(evt1.clone());
                self.events.push(evt2.clone());
                events.push(evt1);
                events.push(evt2);
            }
            self.possession = def_side;
            self.transition_side = Some(def_side);
            self.ball_zone = if rng.gen_range(0.0..1.0f64)
                < transition_progress_chance(self.team_ref(def_side).play_style)
            {
                Zone::Midfield.advance_towards(def_side)
            } else {
                Zone::Midfield
            };
        }
        events
    }

    fn resolve_midfield<R: Rng>(
        &mut self,
        minute: u8,
        att_side: Side,
        def_side: Side,
        rng: &mut R,
    ) -> Vec<MatchEvent> {
        let mut events = Vec::new();
        let attacker = self.snap_player(att_side, Position::Midfielder, rng);
        let defender = self.snap_player(def_side, Position::Midfielder, rng);
        let base_context = self.intent_context(minute, att_side, false);
        let attacker_intent = choose_midfield_intent(
            &attacker,
            self.team_ref(att_side).play_style,
            base_context,
            rng,
        );
        let defender_intent = choose_defensive_intent(
            &defender,
            self.team_ref(def_side).play_style,
            base_context,
            rng,
        );

        let att_raw = match attacker_intent {
            ActionIntent::OneTouchCombination
            | ActionIntent::DelayedRelease
            | ActionIntent::ThirdManLayoff => {
                (attacker.passing as f64
                    + attacker.vision as f64
                    + attacker.decisions as f64
                    + attacker.teamwork as f64)
                    / 4.0
            }
            ActionIntent::ProgressiveCarry | ActionIntent::PressEscapeTurn => {
                (attacker.dribbling as f64
                    + attacker.pace as f64
                    + attacker.agility as f64
                    + attacker.composure as f64)
                    / 4.0
            }
            _ => 55.0,
        };
        let att_rating = self.condition_adjusted_skill(&attacker.id, att_raw)
            * match attacker_intent {
                ActionIntent::OneTouchCombination
                | ActionIntent::DelayedRelease
                | ActionIntent::ThirdManLayoff => {
                    trait_bonus(&attacker, TraitContext::Passing)
                        * trait_pass_bias(&attacker)
                        * role_pass_bias(attacker.role)
                }
                ActionIntent::ProgressiveCarry | ActionIntent::PressEscapeTurn => {
                    trait_bonus(&attacker, TraitContext::Dribbling)
                        * trait_carry_bias(&attacker)
                        * role_carry_bias(attacker.role)
                }
                _ => 1.0,
            }
            * trait_bonus(&attacker, TraitContext::Midfield);

        let def_raw = (defender.tackling as f64
            + defender.positioning as f64
            + defender.decisions as f64
            + defender.teamwork as f64)
            / 4.0;
        let def_rating = self.condition_adjusted_skill(&defender.id, def_raw)
            * trait_bonus(&defender, TraitContext::Tackling)
            * trait_press_bias(&defender)
            * role_press_bias(defender.role)
            * defensive_intent_modifier(defender_intent, &defender, base_context);

        let att_mod = midfield_attack_modifier(
            self.team_ref(att_side).play_style,
            self.transition_side == Some(att_side),
        ) * formation_midfield_modifier(&self.team_ref(att_side).formation);
        let def_mod = midfield_defense_modifier(self.team_ref(def_side).play_style)
            * ((formation_midfield_modifier(&self.team_ref(def_side).formation) * 0.6)
                + (formation_rest_defense_modifier(&self.team_ref(def_side).formation) * 0.4));
        let att_eff = att_rating * att_mod * crate::shared::home_mod(att_side, &self.config);
        let def_eff = def_rating * def_mod * crate::shared::home_mod(def_side, &self.config);
        let success = att_eff / (att_eff + def_eff);

        if rng.gen_range(0.0..1.0f64) < success {
            let success_event = match attacker_intent {
                ActionIntent::ProgressiveCarry | ActionIntent::PressEscapeTurn => {
                    EventType::Dribble
                }
                _ => EventType::PassCompleted,
            };
            let evt = MatchEvent::new(minute, success_event, att_side, Zone::Midfield)
                .with_player(&attacker.id);
            self.events.push(evt.clone());
            events.push(evt);

            let next_zone = match attacker_intent {
                ActionIntent::ThirdManLayoff if base_context.in_transition => {
                    Zone::attacking_box(att_side)
                }
                ActionIntent::PressEscapeTurn if rng.gen_range(0.0..1.0f64) < 0.22 => {
                    Zone::attacking_box(att_side)
                }
                _ => Zone::attacking_third(att_side),
            };
            self.ball_zone = next_zone;
            if matches!(
                attacker_intent,
                ActionIntent::ThirdManLayoff | ActionIntent::PressEscapeTurn
            ) {
                self.transition_side = Some(att_side);
            }
        } else {
            events.extend(self.handle_midfield_defensive_win(
                minute,
                att_side,
                def_side,
                &attacker,
                &defender,
                attacker_intent,
                defender_intent,
                rng,
            ));
        }
        events
    }

    fn resolve_attacking_third<R: Rng>(
        &mut self,
        minute: u8,
        att_side: Side,
        def_side: Side,
        rng: &mut R,
    ) -> Vec<MatchEvent> {
        let mut events = Vec::new();
        let attacker = self.snap_player(att_side, Position::Forward, rng);
        let defender = self.snap_player(def_side, Position::Defender, rng);
        let base_context = self.intent_context(minute, att_side, false);
        let attacker_intent = choose_attacking_intent(
            &attacker,
            self.team_ref(att_side).play_style,
            base_context,
            rng,
        );
        let defender_intent = choose_defensive_intent(
            &defender,
            self.team_ref(def_side).play_style,
            base_context,
            rng,
        );

        let att_raw = match attacker_intent {
            ActionIntent::LinkPlaySlip | ActionIntent::LateBoxDelivery => {
                (attacker.passing as f64
                    + attacker.vision as f64
                    + attacker.teamwork as f64
                    + attacker.composure as f64)
                    / 4.0
            }
            ActionIntent::BlindSideRun | ActionIntent::NearPostAttack => {
                (attacker.pace as f64
                    + attacker.positioning as f64
                    + attacker.agility as f64
                    + attacker.shooting as f64)
                    / 4.0
            }
            ActionIntent::DirectDribble => {
                (attacker.dribbling as f64
                    + attacker.pace as f64
                    + attacker.agility as f64
                    + attacker.composure as f64)
                    / 4.0
            }
            _ => 55.0,
        };
        let att_rating = self.condition_adjusted_skill(&attacker.id, att_raw)
            * match attacker_intent {
                ActionIntent::LinkPlaySlip | ActionIntent::LateBoxDelivery => {
                    trait_bonus(&attacker, TraitContext::Passing)
                        * trait_pass_bias(&attacker)
                        * role_pass_bias(attacker.role)
                }
                ActionIntent::BlindSideRun | ActionIntent::NearPostAttack => {
                    trait_bonus(&attacker, TraitContext::Shooting)
                        * trait_shot_bias(&attacker)
                        * role_shot_bias(attacker.role)
                }
                ActionIntent::DirectDribble => {
                    trait_bonus(&attacker, TraitContext::Dribbling)
                        * trait_carry_bias(&attacker)
                        * role_carry_bias(attacker.role)
                }
                _ => 1.0,
            };
        let def_raw = (defender.defending as f64
            + defender.tackling as f64
            + defender.positioning as f64
            + defender.aerial as f64)
            / 4.0;
        let def_rating = self.condition_adjusted_skill(&defender.id, def_raw)
            * trait_bonus(&defender, TraitContext::Tackling)
            * trait_clearance_bias(&defender)
            * role_clearance_bias(defender.role)
            * defensive_intent_modifier(defender_intent, &defender, base_context);

        let att_mod = attack_modifier(
            self.team_ref(att_side).play_style,
            self.transition_side == Some(att_side),
        ) * formation_attack_modifier(&self.team_ref(att_side).formation);
        let def_mod = defense_modifier(self.team_ref(def_side).play_style)
            * formation_rest_defense_modifier(&self.team_ref(def_side).formation);
        let att_eff = att_rating * att_mod * crate::shared::home_mod(att_side, &self.config);
        let def_eff = def_rating * def_mod * crate::shared::home_mod(def_side, &self.config);
        let success = att_eff / (att_eff + def_eff);
        let zone = Zone::attacking_third(att_side);

        if rng.gen_range(0.0..1.0f64) < success {
            let success_event = match attacker_intent {
                ActionIntent::LateBoxDelivery => EventType::Cross,
                ActionIntent::LinkPlaySlip => EventType::PassCompleted,
                _ => EventType::Dribble,
            };
            let evt =
                MatchEvent::new(minute, success_event, att_side, zone).with_player(&attacker.id);
            self.events.push(evt.clone());
            events.push(evt);
            self.ball_zone = Zone::attacking_box(att_side);
            if matches!(
                attacker_intent,
                ActionIntent::BlindSideRun
                    | ActionIntent::NearPostAttack
                    | ActionIntent::LateBoxDelivery
            ) {
                self.transition_side = Some(att_side);
            }
        } else {
            events.extend(self.handle_attacking_third_defensive_win(
                minute,
                att_side,
                def_side,
                &attacker,
                &defender,
                attacker_intent,
                defender_intent,
                rng,
            ));
        }
        events
    }

    fn resolve_shot<R: Rng>(&mut self, minute: u8, att_side: Side, rng: &mut R) -> Vec<MatchEvent> {
        let mut events = Vec::new();
        let def_side = att_side.opposite();
        let shooter = self.snap_player(att_side, Position::Forward, rng);
        let assister = self.snap_player(att_side, Position::Midfielder, rng);
        let goalkeeper = self.snap_player(def_side, Position::Goalkeeper, rng);

        let shoot_raw =
            (shooter.shooting as f64 + shooter.composure as f64 + shooter.decisions as f64) / 3.0;
        let shoot_rating = self.condition_adjusted_skill(&shooter.id, shoot_raw)
            * trait_bonus(&shooter, TraitContext::Shooting)
            * trait_shot_bias(&shooter)
            * role_shot_bias(shooter.role);
        let gk_raw = (goalkeeper.handling as f64
            + goalkeeper.reflexes as f64
            + goalkeeper.positioning as f64)
            / 3.0;
        let gk_rating = self.condition_adjusted_skill(&goalkeeper.id, gk_raw)
            * trait_bonus(&goalkeeper, TraitContext::Goalkeeping);

        let accuracy = (self.config.shot_accuracy_base
            + (shoot_rating - 50.0) / 200.0
            + (formation_attack_modifier(&self.team_ref(att_side).formation)
                - formation_rest_defense_modifier(&self.team_ref(def_side).formation))
                * 0.04
            + trait_strength(&shooter, "BlindSideRunner") * 0.03
            + trait_strength(&shooter, "NearPostHunter") * 0.02)
            .clamp(0.15, 0.88);
        let zone = Zone::attacking_box(att_side);

        if rng.gen_range(0.0..1.0f64) > accuracy {
            if rng.gen_range(0.0..1.0f64) < 0.38 {
                let evt = MatchEvent::new(minute, EventType::ShotBlocked, att_side, zone)
                    .with_player(&shooter.id);
                self.events.push(evt.clone());
                events.push(evt);
                if self.maybe_rebound_follow_up(minute, att_side, def_side, rng, &mut events) {
                    return events;
                }
                if rng.gen_range(0.0..1.0f64) < 0.28 {
                    let evt = MatchEvent::new(minute, EventType::Corner, att_side, zone);
                    self.events.push(evt.clone());
                    events.push(evt);
                    self.ball_zone = Zone::attacking_box(att_side);
                    return events;
                }
            } else {
                let evt = MatchEvent::new(minute, EventType::ShotOffTarget, att_side, zone)
                    .with_player(&shooter.id);
                self.events.push(evt.clone());
                events.push(evt);
                if rng.gen_range(0.0..1.0f64) < 0.55 {
                    let evt = MatchEvent::new(minute, EventType::GoalKick, def_side, zone);
                    self.events.push(evt.clone());
                    events.push(evt);
                }
            }
            self.ball_zone = Zone::defensive_third(def_side);
            self.possession = def_side;
            return events;
        }

        let conversion = (self.config.goal_conversion_base + (shoot_rating - gk_rating) / 150.0)
            .clamp(0.10, 0.70);

        if rng.gen_range(0.0..1.0f64) < conversion {
            let evt = MatchEvent::new(minute, EventType::Goal, att_side, zone)
                .with_player(&shooter.id)
                .with_secondary(&assister.id);
            self.events.push(evt.clone());
            events.push(evt);
            self.add_goal(att_side);
            self.ball_zone = Zone::Midfield;
            self.possession = def_side;
        } else {
            let evt = MatchEvent::new(minute, EventType::ShotSaved, att_side, zone)
                .with_player(&shooter.id);
            self.events.push(evt.clone());
            events.push(evt);
            if self.maybe_rebound_follow_up(minute, att_side, def_side, rng, &mut events) {
                return events;
            }
            self.settle_after_goalkeeper_control(minute, def_side, &goalkeeper, rng, &mut events);
        }

        events
    }

    fn handle_midfield_defensive_win<R: Rng>(
        &mut self,
        minute: u8,
        att_side: Side,
        def_side: Side,
        attacker: &PlayerSnap,
        defender: &PlayerSnap,
        attacker_intent: ActionIntent,
        defender_intent: ActionIntent,
        rng: &mut R,
    ) -> Vec<MatchEvent> {
        let mut events = Vec::new();
        match defender_intent {
            ActionIntent::StepInInterception => {
                if matches!(
                    attacker_intent,
                    ActionIntent::OneTouchCombination
                        | ActionIntent::DelayedRelease
                        | ActionIntent::ThirdManLayoff
                ) {
                    let evt = MatchEvent::new(
                        minute,
                        EventType::PassIntercepted,
                        att_side,
                        Zone::Midfield,
                    )
                    .with_player(&attacker.id);
                    self.events.push(evt.clone());
                    events.push(evt);
                }
                let evt =
                    MatchEvent::new(minute, EventType::Interception, def_side, Zone::Midfield)
                        .with_player(&defender.id);
                self.events.push(evt.clone());
                events.push(evt);
            }
            ActionIntent::TacticalFoulStop => {
                let evt = MatchEvent::new(minute, EventType::Tackle, def_side, Zone::Midfield)
                    .with_player(&defender.id);
                self.events.push(evt.clone());
                events.push(evt);
                let foul_events =
                    self.maybe_foul(minute, def_side, attacker, defender, Zone::Midfield, rng);
                events.extend(foul_events);
            }
            ActionIntent::ContainAndShowWide => {
                let evt = MatchEvent::new(minute, EventType::Tackle, def_side, Zone::Midfield)
                    .with_player(&defender.id);
                self.events.push(evt.clone());
                events.push(evt);
            }
            ActionIntent::AerialClearance => {
                let evt = MatchEvent::new(minute, EventType::Clearance, def_side, Zone::Midfield)
                    .with_player(&defender.id);
                self.events.push(evt.clone());
                events.push(evt);
            }
            _ => {}
        }

        self.possession = def_side;
        self.transition_side = Some(def_side);
        self.ball_zone = if rng.gen_range(0.0..1.0f64)
            < transition_progress_chance(self.team_ref(def_side).play_style)
        {
            Zone::attacking_third(def_side)
        } else {
            Zone::Midfield
        };
        events
    }

    fn handle_attacking_third_defensive_win<R: Rng>(
        &mut self,
        minute: u8,
        att_side: Side,
        def_side: Side,
        attacker: &PlayerSnap,
        defender: &PlayerSnap,
        attacker_intent: ActionIntent,
        defender_intent: ActionIntent,
        rng: &mut R,
    ) -> Vec<MatchEvent> {
        let mut events = Vec::new();
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
                let evt1 =
                    MatchEvent::new(minute, event_type, att_side, zone).with_player(&attacker.id);
                let evt2 = MatchEvent::new(minute, EventType::Interception, def_side, zone)
                    .with_player(&defender.id);
                self.events.push(evt1.clone());
                self.events.push(evt2.clone());
                events.push(evt1);
                events.push(evt2);
            }
            ActionIntent::TacticalFoulStop => {
                let evt = MatchEvent::new(minute, EventType::Tackle, def_side, zone)
                    .with_player(&defender.id);
                self.events.push(evt.clone());
                events.push(evt);
                let foul_events = self.maybe_foul(minute, def_side, attacker, defender, zone, rng);
                events.extend(foul_events);
            }
            ActionIntent::ContainAndShowWide => {
                let evt1 = MatchEvent::new(minute, EventType::DribbleTackled, att_side, zone)
                    .with_player(&attacker.id)
                    .with_secondary(&defender.id);
                let evt2 = MatchEvent::new(minute, EventType::Tackle, def_side, zone)
                    .with_player(&defender.id);
                self.events.push(evt1.clone());
                self.events.push(evt2.clone());
                events.push(evt1);
                events.push(evt2);
            }
            _ => {
                let evt = MatchEvent::new(minute, EventType::Clearance, def_side, zone)
                    .with_player(&defender.id);
                self.events.push(evt.clone());
                events.push(evt);
            }
        }

        if rng.gen_range(0.0..1.0f64)
            < if attacker_intent == ActionIntent::LateBoxDelivery {
                0.32
            } else {
                0.24
            }
        {
            let evt = MatchEvent::new(minute, EventType::Corner, att_side, zone);
            self.events.push(evt.clone());
            events.push(evt);
            if rng.gen_range(0.0..1.0f64) < 0.30 {
                self.ball_zone = Zone::attacking_box(att_side);
                return events;
            }
        }

        self.possession = def_side;
        self.transition_side = Some(def_side);
        self.ball_zone = Zone::defensive_third(def_side);
        events
    }

    fn maybe_rebound_follow_up<R: Rng>(
        &mut self,
        minute: u8,
        att_side: Side,
        def_side: Side,
        rng: &mut R,
        events: &mut Vec<MatchEvent>,
    ) -> bool {
        let rebounder = self.snap_player(att_side, Position::Forward, rng);
        let rebound_window = 0.08
            + trait_strength(&rebounder, "ReboundInstinct") * 0.28
            + trait_strength(&rebounder, "SecondBallPredator") * 0.18
            + trait_strength(&rebounder, "LateBoxArriver") * 0.12
            + trait_strength(&rebounder, "FarPostGhost") * 0.08;
        if rng.gen_range(0.0..1.0f64) >= rebound_window.clamp(0.05, 0.42) {
            return false;
        }

        let goalkeeper = self.snap_player(def_side, Position::Goalkeeper, rng);
        let finish_raw = (rebounder.shooting as f64
            + rebounder.positioning as f64
            + rebounder.composure as f64
            + rebounder.aggression as f64)
            / 4.0;
        let finish_rating = self.condition_adjusted_skill(&rebounder.id, finish_raw)
            * (1.0 + trait_strength(&rebounder, "ReboundInstinct") * 0.12)
            * trait_shot_bias(&rebounder)
            * role_shot_bias(rebounder.role);
        let gk_raw = (goalkeeper.handling as f64
            + goalkeeper.reflexes as f64
            + goalkeeper.positioning as f64)
            / 3.0;
        let gk_rating = self.condition_adjusted_skill(&goalkeeper.id, gk_raw)
            * trait_bonus(&goalkeeper, TraitContext::Goalkeeping);
        let zone = Zone::attacking_box(att_side);

        if rng.gen_range(0.0..1.0f64)
            < (0.22 + (finish_rating - gk_rating) / 220.0).clamp(0.08, 0.55)
        {
            let evt =
                MatchEvent::new(minute, EventType::Goal, att_side, zone).with_player(&rebounder.id);
            self.events.push(evt.clone());
            events.push(evt);
            self.add_goal(att_side);
            self.ball_zone = Zone::Midfield;
            self.possession = def_side;
            return true;
        }

        if rng.gen_range(0.0..1.0f64) < 0.58 {
            let evt = MatchEvent::new(minute, EventType::ShotSaved, att_side, zone)
                .with_player(&rebounder.id);
            self.events.push(evt.clone());
            events.push(evt);
            self.settle_after_goalkeeper_control(minute, def_side, &goalkeeper, rng, events);
        } else {
            let evt1 = MatchEvent::new(minute, EventType::ShotOffTarget, att_side, zone)
                .with_player(&rebounder.id);
            let evt2 = MatchEvent::new(minute, EventType::GoalKick, def_side, zone);
            self.events.push(evt1.clone());
            self.events.push(evt2.clone());
            events.push(evt1);
            events.push(evt2);
            self.ball_zone = Zone::defensive_third(def_side);
            self.possession = def_side;
        }

        true
    }

    fn settle_after_goalkeeper_control<R: Rng>(
        &mut self,
        minute: u8,
        possession_side: Side,
        goalkeeper: &PlayerSnap,
        rng: &mut R,
        events: &mut Vec<MatchEvent>,
    ) {
        let distribution_context = self.intent_context(minute, possession_side, false);
        let distribution_intent = choose_goalkeeper_distribution_intent(
            goalkeeper,
            self.team_ref(possession_side).play_style,
            IntentContext {
                in_transition: true,
                chasing_game: distribution_context.chasing_game,
                protecting_lead: distribution_context.protecting_lead,
                late_game: distribution_context.late_game,
                under_pressure: false,
            },
            rng,
        );

        self.possession = possession_side;
        match distribution_intent {
            ActionIntent::LaunchThrow => {
                let evt = MatchEvent::new(
                    minute,
                    EventType::PassCompleted,
                    possession_side,
                    Zone::defensive_third(possession_side),
                )
                .with_player(&goalkeeper.id);
                self.events.push(evt.clone());
                events.push(evt);
                self.transition_side = Some(possession_side);
                self.ball_zone = if rng.gen_range(0.0..1.0f64)
                    < transition_progress_chance(self.team_ref(possession_side).play_style) + 0.12
                {
                    Zone::attacking_third(possession_side)
                } else {
                    Zone::Midfield
                };
            }
            _ => {
                self.ball_zone = Zone::defensive_third(possession_side);
            }
        }
    }

    pub(super) fn maybe_foul<R: Rng>(
        &mut self,
        minute: u8,
        fouling_side: Side,
        fouled: &PlayerSnap,
        fouler: &PlayerSnap,
        zone: Zone,
        rng: &mut R,
    ) -> Vec<MatchEvent> {
        let mut events = Vec::new();

        let aggression_mod = fouler.aggression as f64 / 100.0;
        let foul_chance = self.config.foul_probability
            * (0.6 + aggression_mod * 0.8)
            * trait_bonus(fouler, TraitContext::Foul);
        if rng.gen_range(0.0..1.0f64) >= foul_chance {
            return events;
        }

        let evt = MatchEvent::new(minute, EventType::Foul, fouling_side, zone)
            .with_player(&fouler.id)
            .with_secondary(&fouled.id);
        self.events.push(evt.clone());
        events.push(evt);

        let att_side = fouling_side.opposite();

        if zone.is_box_for(att_side) && rng.gen_range(0.0..1.0f64) < self.config.penalty_probability
        {
            let evt = MatchEvent::new(minute, EventType::PenaltyAwarded, att_side, zone);
            self.events.push(evt.clone());
            events.push(evt);
            let pen_events = self.resolve_in_match_penalty(minute, att_side, rng);
            events.extend(pen_events);
        } else {
            let evt = MatchEvent::new(minute, EventType::FreeKick, att_side, zone);
            self.events.push(evt.clone());
            events.push(evt);
        }

        let card_events = self.maybe_card(minute, fouling_side, &fouler.id, zone, rng);
        events.extend(card_events);

        if rng.gen_range(0.0..1.0f64) < self.config.injury_probability {
            let evt =
                MatchEvent::new(minute, EventType::Injury, att_side, zone).with_player(&fouled.id);
            self.events.push(evt.clone());
            events.push(evt);
        }

        events
    }

    fn maybe_card<R: Rng>(
        &mut self,
        minute: u8,
        side: Side,
        fouler_id: &str,
        zone: Zone,
        rng: &mut R,
    ) -> Vec<MatchEvent> {
        let mut events = Vec::new();

        if rng.gen_range(0.0..1.0f64) >= self.config.yellow_card_probability {
            return events;
        }

        if rng.gen_range(0.0..1.0f64) < self.config.red_card_probability {
            let evt =
                MatchEvent::new(minute, EventType::RedCard, side, zone).with_player(fouler_id);
            self.events.push(evt.clone());
            events.push(evt);
            self.sent_off.insert(fouler_id.to_string());
            return events;
        }

        let current_yellows = self.yellows.entry(fouler_id.to_string()).or_insert(0);
        *current_yellows += 1;

        if *current_yellows >= 2 {
            let evt =
                MatchEvent::new(minute, EventType::SecondYellow, side, zone).with_player(fouler_id);
            self.events.push(evt.clone());
            events.push(evt);
            self.sent_off.insert(fouler_id.to_string());
        } else {
            let evt =
                MatchEvent::new(minute, EventType::YellowCard, side, zone).with_player(fouler_id);
            self.events.push(evt.clone());
            events.push(evt);
        }

        events
    }

    fn intent_context(&self, minute: u8, side: Side, under_pressure: bool) -> IntentContext {
        let side_score = match side {
            Side::Home => self.home_score,
            Side::Away => self.away_score,
        };
        let opp_score = match side.opposite() {
            Side::Home => self.home_score,
            Side::Away => self.away_score,
        };

        IntentContext {
            under_pressure,
            in_transition: self.transition_side == Some(side),
            late_game: minute >= 75,
            protecting_lead: side_score > opp_score && minute >= 60,
            chasing_game: side_score < opp_score && minute >= 55,
        }
    }
}

fn defensive_intent_modifier(
    intent: ActionIntent,
    defender: &PlayerSnap,
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
