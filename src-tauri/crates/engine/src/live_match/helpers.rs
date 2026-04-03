use rand::Rng;

use crate::shared::{
    PlayerSnap, fatigue_modifier, formation_midfield_modifier, formation_rest_defense_modifier,
    home_mod, midfield_attack_modifier, player_action_weight, press_modifier, weighted_index,
};
use crate::types::{PlayerData, Position, Side, TeamData};

use super::{LiveMatchState, SetPieceTakers};

// ---------------------------------------------------------------------------
// Stamina system
// ---------------------------------------------------------------------------

impl LiveMatchState {
    pub(super) fn deplete_stamina_tick(&mut self) {
        let fatigue_rate = self.config.fatigue_per_minute;
        // Iterate over all on-pitch players
        for (players, style) in [
            (&self.home.players, self.home.play_style),
            (&self.away.players, self.away.play_style),
        ] {
            let fatigue_style = fatigue_modifier(style);
            for p in players {
                if self.sent_off.contains(&p.id) {
                    continue;
                }
                let stamina_factor = p.stamina as f64 / 100.0;
                let fitness_factor = p.fitness as f64 / 100.0;
                // Higher stamina → less depletion; higher fitness → less depletion.
                // Fitness scales the base depletion more aggressively (unfit players tire much faster).
                let depletion = fatigue_rate
                    * fatigue_style
                    * (1.0 - stamina_factor * 0.5)
                    * (1.3 - fitness_factor * 0.6);
                if let Some(cond) = self.player_conditions.get_mut(&p.id) {
                    *cond = (*cond - depletion).max(5.0);
                }
            }
        }
    }

    /// Adjust a skill value based on the player's current in-match condition.
    pub(super) fn condition_adjusted_skill(&self, player_id: &str, base_skill: f64) -> f64 {
        let condition = self
            .player_conditions
            .get(player_id)
            .copied()
            .unwrap_or(50.0);
        // At 100% condition: full skill. At 50%: ~80% skill. At 0%: ~60% skill.
        let factor = 0.6 + 0.4 * (condition / 100.0);
        base_skill * factor
    }

    // -----------------------------------------------------------------------
    // Player selection helpers
    // -----------------------------------------------------------------------

    pub(super) fn snap_player<R: Rng>(
        &self,
        side: Side,
        preferred: Position,
        rng: &mut R,
    ) -> PlayerSnap {
        let team = self.team_ref(side);
        let available: Vec<&PlayerData> = team
            .players
            .iter()
            .filter(|p| !self.sent_off.contains(&p.id))
            .collect();

        let candidates: Vec<&PlayerData> = available
            .iter()
            .filter(|p| p.position == preferred)
            .copied()
            .collect();

        let pool = if candidates.is_empty() {
            &available
        } else {
            &candidates
        };
        if pool.is_empty() {
            return PlayerSnap::from(&team.players[0]);
        }
        let weights = pool
            .iter()
            .map(|player| {
                let condition = self
                    .player_conditions
                    .get(&player.id)
                    .copied()
                    .unwrap_or(player.condition as f64)
                    / 100.0;
                player_action_weight(player, preferred, &team.formation) * (0.6 + condition * 0.4)
            })
            .collect::<Vec<_>>();
        PlayerSnap::from(pool[weighted_index(&weights, rng)])
    }

    pub(super) fn snap_player_by_id(&self, player_id: &str, side: Side) -> PlayerSnap {
        let team = self.team_ref(side);
        if let Some(p) = team.players.iter().find(|p| p.id == player_id) {
            PlayerSnap::from(p)
        } else {
            PlayerSnap::from(&team.players[0])
        }
    }

    pub(super) fn pick_penalty_taker<R: Rng>(&self, side: Side, rng: &mut R) -> PlayerSnap {
        // Use designated taker if set
        if let Some(ref id) = self.set_pieces_ref(side).penalty_taker {
            let team = self.team_ref(side);
            if let Some(p) = team
                .players
                .iter()
                .find(|p| p.id == *id && !self.sent_off.contains(&p.id))
            {
                return PlayerSnap::from(p);
            }
        }
        // Fallback: pick the forward with highest shooting
        let team = self.team_ref(side);
        let mut candidates: Vec<&PlayerData> = team
            .players
            .iter()
            .filter(|p| !self.sent_off.contains(&p.id))
            .collect();
        candidates.sort_by(|a, b| b.shooting.cmp(&a.shooting));
        if let Some(p) = candidates.first() {
            PlayerSnap::from(p)
        } else {
            self.snap_player(side, Position::Forward, rng)
        }
    }

    pub(super) fn pick_goalkeeper(&self, side: Side) -> PlayerSnap {
        let team = self.team_ref(side);
        for p in &team.players {
            if p.position == Position::Goalkeeper && !self.sent_off.contains(&p.id) {
                return PlayerSnap::from(p);
            }
        }
        // No goalkeeper available — pick first available
        for p in &team.players {
            if !self.sent_off.contains(&p.id) {
                return PlayerSnap::from(p);
            }
        }
        PlayerSnap::from(&team.players[0])
    }

    // -----------------------------------------------------------------------
    // Rating helpers
    // -----------------------------------------------------------------------

    pub(super) fn effective_midfield(&self, side: Side) -> f64 {
        let base = self.team_ref(side).midfield_rating();
        let modifier = midfield_attack_modifier(self.team_ref(side).play_style, false)
            * formation_midfield_modifier(&self.team_ref(side).formation);
        base * modifier * home_mod(side, &self.config)
    }

    pub(super) fn effective_press(&self, pressing_side: Side) -> f64 {
        let team = self.team_ref(pressing_side);
        let base = team.position_attr_avg(Position::Midfielder, |p| {
            ((p.stamina as u16 + p.tackling as u16 + p.pace as u16) / 3) as u8
        });
        let modifier = press_modifier(team.play_style)
            * ((formation_midfield_modifier(&team.formation) * 0.65)
                + (formation_rest_defense_modifier(&team.formation) * 0.35));
        base * modifier * home_mod(pressing_side, &self.config)
    }

    // -----------------------------------------------------------------------
    // Internal accessors
    // -----------------------------------------------------------------------

    pub(super) fn team_ref(&self, side: Side) -> &TeamData {
        match side {
            Side::Home => &self.home,
            Side::Away => &self.away,
        }
    }

    pub(super) fn team_mut(&mut self, side: Side) -> &mut TeamData {
        match side {
            Side::Home => &mut self.home,
            Side::Away => &mut self.away,
        }
    }

    pub(super) fn set_pieces_ref(&self, side: Side) -> &SetPieceTakers {
        match side {
            Side::Home => &self.home_set_pieces,
            Side::Away => &self.away_set_pieces,
        }
    }

    pub(super) fn set_pieces_mut(&mut self, side: Side) -> &mut SetPieceTakers {
        match side {
            Side::Home => &mut self.home_set_pieces,
            Side::Away => &mut self.away_set_pieces,
        }
    }

    pub(super) fn add_goal(&mut self, side: Side) {
        match side {
            Side::Home => self.home_score += 1,
            Side::Away => self.away_score += 1,
        }
    }
}
