use macroquad::prelude::*;
use crate::character::traits::CharacterClass;
use crate::core::context::CombatContext;
use crate::core::types::{PlayerBody, PlayerModifiers};
use crate::skill::abilities::default_skills;
use crate::skill::traits::SkillSlot;
use crate::weapon::arsenal::default_arsenal;
use crate::weapon::traits::Weapon;

pub struct CyberMarine;

impl CharacterClass for CyberMarine {
    fn name(&self) -> &'static str {
        "CYBER MARINE"
    }

    fn description(&self) -> &'static str {
        "Chien binh can bang voi giap Nano tu phuc hoi va kho vu khi 5 sung."
    }

    fn initial_body(&self, start_pos: Vec2) -> PlayerBody {
        PlayerBody {
            pos: start_pos,
            vel: Vec2::ZERO,
            radius: 18.0,
            angle: 0.0,
            speed: 325.0,
            hp: 100.0,
            max_hp: 100.0,
            shield: 50.0,
            max_shield: 50.0,
            shield_regen_timer: 0.0,
            stamina: 100.0,
            max_stamina: 100.0,
            is_dashing: false,
            dash_duration: 0.0,
            bullet_time: false,
            active_overcharge: false,
            modifiers: PlayerModifiers::default(),
        }
    }

    fn starting_weapons(&self) -> Vec<Weapon> {
        default_arsenal()
    }

    fn starting_skills(&self) -> Vec<SkillSlot> {
        default_skills()
    }

    fn on_enemy_killed(&mut self, player: &mut PlayerBody, _ctx: &mut CombatContext) {
        if player.modifiers.vampiric && player.hp < player.max_hp {
            player.hp = (player.hp + 2.5).min(player.max_hp);
        }
    }
}
