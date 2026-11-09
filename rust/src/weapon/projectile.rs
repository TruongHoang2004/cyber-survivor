use macroquad::prelude::*;
use crate::core::types::StatusEffect;

#[derive(Clone)]
pub struct Projectile {
    pub pos: Vec2,
    pub vel: Vec2,
    pub damage: f32,
    pub color: Color,
    pub radius: f32,
    pub explosion_radius: Option<f32>,
    pub spawns_black_hole: bool,
    pub pierce: u32,
    pub bounces: u32,
    pub life: f32,
    pub status_effect: Option<StatusEffect>,
}

impl Projectile {
    pub fn basic(pos: Vec2, vel: Vec2, damage: f32, color: Color, radius: f32, life: f32) -> Self {
        Self {
            pos,
            vel,
            damage,
            color,
            radius,
            explosion_radius: None,
            spawns_black_hole: false,
            pierce: 0,
            bounces: 0,
            life,
            status_effect: None,
        }
    }
}
