use macroquad::prelude::*;
use crate::core::types::{Decal, FloatText, Particle};
use crate::enemy::traits::EnemyEntity;
use crate::skill::deployables::Deployable;
use crate::weapon::projectile::Projectile;

pub enum WorldCommand {
    SpawnPlayerProjectile(Projectile),
    SpawnEnemyProjectile(Projectile),
    SpawnEnemy(EnemyEntity),
    SpawnDeployable(Deployable),
    TriggerExplosion {
        pos: Vec2,
        radius: f32,
        damage: f32,
    },
    DamagePlayer(f32),
    AddScreenShake(f32),
    SpawnParticle(Particle),
    SpawnDecal(Decal),
    SpawnFloatText(FloatText),
}

pub struct CombatContext {
    pub player_pos: Vec2,
    pub player_radius: f32,
    pub player_is_dashing: bool,
    pub enemy_positions: Vec<Vec2>,
    pub commands: Vec<WorldCommand>,
}

impl CombatContext {
    pub fn new(
        player_pos: Vec2,
        player_radius: f32,
        player_is_dashing: bool,
        enemy_positions: Vec<Vec2>,
    ) -> Self {
        Self {
            player_pos,
            player_radius,
            player_is_dashing,
            enemy_positions,
            commands: Vec::with_capacity(32),
        }
    }

    pub fn spawn_player_projectile(&mut self, proj: Projectile) {
        self.commands.push(WorldCommand::SpawnPlayerProjectile(proj));
    }

    pub fn spawn_enemy_projectile(&mut self, proj: Projectile) {
        self.commands.push(WorldCommand::SpawnEnemyProjectile(proj));
    }

    pub fn spawn_enemy(&mut self, enemy: EnemyEntity) {
        self.commands.push(WorldCommand::SpawnEnemy(enemy));
    }

    pub fn spawn_deployable(&mut self, dep: Deployable) {
        self.commands.push(WorldCommand::SpawnDeployable(dep));
    }

    pub fn trigger_explosion(&mut self, pos: Vec2, radius: f32, damage: f32) {
        self.commands.push(WorldCommand::TriggerExplosion {
            pos,
            radius,
            damage,
        });
    }

    pub fn damage_player(&mut self, amount: f32) {
        if !self.player_is_dashing {
            self.commands.push(WorldCommand::DamagePlayer(amount));
        }
    }

    pub fn add_screen_shake(&mut self, amount: f32) {
        self.commands.push(WorldCommand::AddScreenShake(amount));
    }

    pub fn spawn_particle(&mut self, p: Particle) {
        self.commands.push(WorldCommand::SpawnParticle(p));
    }

    pub fn spawn_particle_burst(&mut self, pos: Vec2, color: Color, count: usize, max_speed: f32, life: f32) {
        for _ in 0..count {
            let angle = rand::gen_range(0.0, std::f32::consts::PI * 2.0);
            let spd = rand::gen_range(max_speed * 0.2, max_speed);
            self.commands.push(WorldCommand::SpawnParticle(Particle {
                pos,
                vel: vec2(angle.cos(), angle.sin()) * spd,
                color,
                radius: rand::gen_range(2.0, 5.0),
                life,
                max_life: life,
            }));
        }
    }

    pub fn spawn_decal(&mut self, decal: Decal) {
        self.commands.push(WorldCommand::SpawnDecal(decal));
    }

    pub fn spawn_float_text(&mut self, pos: Vec2, text: impl Into<String>, color: Color, size: f32) {
        self.commands.push(WorldCommand::SpawnFloatText(FloatText {
            pos,
            text: text.into(),
            color,
            size,
            life: 0.9,
        }));
    }

    pub fn find_nearest_enemy(&self, from: Vec2, max_range: f32) -> Option<Vec2> {
        let mut nearest = None;
        let mut min_dist = max_range;
        for &ep in &self.enemy_positions {
            let d = (ep - from).length();
            if d < min_dist {
                min_dist = d;
                nearest = Some(ep);
            }
        }
        nearest
    }
}
