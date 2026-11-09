use macroquad::prelude::*;
use crate::core::context::CombatContext;
use crate::core::types::PlayerBody;
use crate::skill::traits::SkillSlot;
use crate::weapon::traits::Weapon;

pub trait CharacterClass: Send + Sync {
    fn name(&self) -> &'static str;
    fn description(&self) -> &'static str;
    fn initial_body(&self, start_pos: Vec2) -> PlayerBody;
    fn starting_weapons(&self) -> Vec<Weapon>;
    fn starting_skills(&self) -> Vec<SkillSlot>;

    fn on_enemy_killed(&mut self, _player: &mut PlayerBody, _ctx: &mut CombatContext) {}
}
