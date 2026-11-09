use macroquad::prelude::*;
use crate::core::context::CombatContext;
use crate::core::types::PlayerBody;

pub trait SkillBehavior: Send + Sync {
    fn name(&self) -> &'static str;
    fn key_label(&self) -> &'static str;

    fn activate(
        &mut self,
        player: &mut PlayerBody,
        aim_world: Vec2,
        move_dir: Vec2,
        ctx: &mut CombatContext,
    ) -> bool;

    fn clone_box(&self) -> Box<dyn SkillBehavior>;
}

pub struct SkillSlot {
    pub key: KeyCode,
    pub cooldown: f32,
    pub max_cooldown: f32,
    pub behavior: Box<dyn SkillBehavior>,
}

impl Clone for SkillSlot {
    fn clone(&self) -> Self {
        Self {
            key: self.key,
            cooldown: self.cooldown,
            max_cooldown: self.max_cooldown,
            behavior: self.behavior.clone_box(),
        }
    }
}

impl SkillSlot {
    pub fn new(key: KeyCode, max_cooldown: f32, behavior: Box<dyn SkillBehavior>) -> Self {
        Self {
            key,
            cooldown: 0.0,
            max_cooldown,
            behavior,
        }
    }

    pub fn effective_max_cd(&self, player: &PlayerBody) -> f32 {
        if self.key == KeyCode::Space {
            self.max_cooldown * player.modifiers.dash_cd_multi
        } else {
            self.max_cooldown
        }
    }

    pub fn try_activate(
        &mut self,
        player: &mut PlayerBody,
        aim_world: Vec2,
        move_dir: Vec2,
        ctx: &mut CombatContext,
    ) {
        if self.cooldown <= 0.0 {
            if self.behavior.activate(player, aim_world, move_dir, ctx) {
                self.cooldown = self.effective_max_cd(player);
            }
        }
    }
}
