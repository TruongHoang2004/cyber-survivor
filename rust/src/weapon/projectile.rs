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
    pub pierce: u32,
    pub bounces: u32,
    pub life: f32,
    pub status_effect: Option<StatusEffect>,
}
