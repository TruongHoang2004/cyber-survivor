use macroquad::prelude::*;
use crate::core::context::CombatContext;
use crate::core::props::WeaponRarity;
use crate::core::types::PlayerModifiers;

#[derive(Clone)]
pub struct WeaponStats {
    pub name: &'static str,
    pub damage: f32,
    pub fire_rate: f32,
    pub spread: f32,
    pub speed: f32,
    pub clip_size: u32,
    pub current_clip: u32,
    pub reserve_ammo: i32,
    pub reload_duration: f32,
    pub color: Color,
    pub screen_shake: f32,
    pub unlocked: bool,
    pub level: u32,
    pub rarity: WeaponRarity,
}

pub trait WeaponBehavior: Send + Sync {
    fn fire(
        &mut self,
        origin: Vec2,
        aim_angle: f32,
        stats: &WeaponStats,
        mods: &PlayerModifiers,
        overcharged: bool,
        ctx: &mut CombatContext,
    );

    fn on_perfect_reload(&mut self, _stats: &mut WeaponStats, _ctx: &mut CombatContext) {}

    fn clone_box(&self) -> Box<dyn WeaponBehavior>;
}

pub struct Weapon {
    pub stats: WeaponStats,
    pub behavior: Box<dyn WeaponBehavior>,
}

impl Clone for Weapon {
    fn clone(&self) -> Self {
        Self {
            stats: self.stats.clone(),
            behavior: self.behavior.clone_box(),
        }
    }
}

impl Weapon {
    pub fn new(stats: WeaponStats, behavior: Box<dyn WeaponBehavior>) -> Self {
        Self { stats, behavior }
    }
}
