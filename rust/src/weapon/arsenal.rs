use macroquad::prelude::*;
use crate::core::context::CombatContext;
use crate::core::types::{PlayerModifiers, StatusEffect};
use crate::weapon::projectile::Projectile;
use crate::weapon::traits::{Weapon, WeaponBehavior, WeaponStats};

fn resolve_status(mods: &PlayerModifiers) -> Option<StatusEffect> {
    if mods.incendiary_rounds {
        Some(StatusEffect::Burn {
            dps: 18.0,
            duration: 3.0,
        })
    } else if mods.cryo_rounds {
        Some(StatusEffect::Slow {
            factor: 0.45,
            duration: 2.5,
        })
    } else {
        None
    }
}

fn calc_damage(stats: &WeaponStats, mods: &PlayerModifiers, overcharged: bool) -> f32 {
    let mut dmg = stats.damage * mods.dmg_multi;
    if overcharged {
        dmg *= 1.4;
    }
    dmg
}

fn bullet_color(stats: &WeaponStats, mods: &PlayerModifiers, overcharged: bool) -> Color {
    if overcharged {
        Color::new(0.0, 1.0, 0.4, 1.0)
    } else if mods.incendiary_rounds {
        Color::new(1.0, 0.35, 0.0, 1.0)
    } else if mods.cryo_rounds {
        Color::new(0.3, 0.85, 1.0, 1.0)
    } else {
        stats.color
    }
}

#[derive(Clone)]
pub struct StandardBallisticBehavior;

impl WeaponBehavior for StandardBallisticBehavior {
    fn fire(
        &mut self,
        origin: Vec2,
        aim_angle: f32,
        stats: &WeaponStats,
        mods: &PlayerModifiers,
        overcharged: bool,
        ctx: &mut CombatContext,
    ) {
        let angle = aim_angle + rand::gen_range(-stats.spread, stats.spread);
        let dir = vec2(angle.cos(), angle.sin());
        ctx.spawn_player_projectile(Projectile {
            pos: origin + dir * 24.0,
            vel: dir * stats.speed,
            damage: calc_damage(stats, mods, overcharged),
            color: bullet_color(stats, mods, overcharged),
            radius: 3.5,
            explosion_radius: None,
            pierce: mods.pierce_count,
            bounces: if mods.bouncing_bullets { 2 } else { 0 },
            life: 2.2,
            status_effect: resolve_status(mods),
        });
    }

    fn clone_box(&self) -> Box<dyn WeaponBehavior> {
        Box::new(self.clone())
    }
}

#[derive(Clone)]
pub struct BuckshotSpreadBehavior {
    pub pellets: u32,
}

impl WeaponBehavior for BuckshotSpreadBehavior {
    fn fire(
        &mut self,
        origin: Vec2,
        aim_angle: f32,
        stats: &WeaponStats,
        mods: &PlayerModifiers,
        overcharged: bool,
        ctx: &mut CombatContext,
    ) {
        let dmg = calc_damage(stats, mods, overcharged);
        let col = bullet_color(stats, mods, overcharged);
        for _ in 0..self.pellets {
            let angle = aim_angle + rand::gen_range(-stats.spread, stats.spread);
            let dir = vec2(angle.cos(), angle.sin());
            ctx.spawn_player_projectile(Projectile {
                pos: origin + dir * 24.0,
                vel: dir * stats.speed * rand::gen_range(0.92, 1.08),
                damage: dmg,
                color: col,
                radius: 3.0,
                explosion_radius: None,
                pierce: mods.pierce_count,
                bounces: if mods.bouncing_bullets { 2 } else { 0 },
                life: 1.4,
                status_effect: resolve_status(mods),
            });
        }
    }

    fn clone_box(&self) -> Box<dyn WeaponBehavior> {
        Box::new(self.clone())
    }
}

#[derive(Clone)]
pub struct PiercingPlasmaBehavior {
    pub base_pierce: u32,
}

impl WeaponBehavior for PiercingPlasmaBehavior {
    fn fire(
        &mut self,
        origin: Vec2,
        aim_angle: f32,
        stats: &WeaponStats,
        mods: &PlayerModifiers,
        overcharged: bool,
        ctx: &mut CombatContext,
    ) {
        let angle = aim_angle + rand::gen_range(-stats.spread, stats.spread);
        let dir = vec2(angle.cos(), angle.sin());
        ctx.spawn_player_projectile(Projectile {
            pos: origin + dir * 24.0,
            vel: dir * stats.speed,
            damage: calc_damage(stats, mods, overcharged),
            color: bullet_color(stats, mods, overcharged),
            radius: 6.5,
            explosion_radius: None,
            pierce: self.base_pierce + mods.pierce_count,
            bounces: if mods.bouncing_bullets { 2 } else { 0 },
            life: 2.4,
            status_effect: resolve_status(mods),
        });
    }

    fn clone_box(&self) -> Box<dyn WeaponBehavior> {
        Box::new(self.clone())
    }
}

#[derive(Clone)]
pub struct ExplosiveRocketBehavior {
    pub blast_radius: f32,
}

impl WeaponBehavior for ExplosiveRocketBehavior {
    fn fire(
        &mut self,
        origin: Vec2,
        aim_angle: f32,
        stats: &WeaponStats,
        mods: &PlayerModifiers,
        overcharged: bool,
        ctx: &mut CombatContext,
    ) {
        let angle = aim_angle + rand::gen_range(-stats.spread, stats.spread);
        let dir = vec2(angle.cos(), angle.sin());
        ctx.spawn_player_projectile(Projectile {
            pos: origin + dir * 24.0,
            vel: dir * stats.speed,
            damage: calc_damage(stats, mods, overcharged),
            color: bullet_color(stats, mods, overcharged),
            radius: 8.0,
            explosion_radius: Some(self.blast_radius),
            pierce: 0,
            bounces: 0,
            life: 2.5,
            status_effect: resolve_status(mods),
        });
    }

    fn clone_box(&self) -> Box<dyn WeaponBehavior> {
        Box::new(self.clone())
    }
}

pub fn default_arsenal() -> Vec<Weapon> {
    vec![
        Weapon::new(
            WeaponStats {
                name: "PISTOL",
                damage: 28.0,
                fire_rate: 0.16,
                spread: 0.04,
                speed: 1150.0,
                clip_size: 15,
                current_clip: 15,
                reserve_ammo: -1,
                reload_duration: 1.1,
                color: Color::new(0.0, 0.94, 1.0, 1.0),
                screen_shake: 2.0,
            },
            Box::new(StandardBallisticBehavior),
        ),
        Weapon::new(
            WeaponStats {
                name: "SHOTGUN",
                damage: 19.0,
                fire_rate: 0.65,
                spread: 0.28,
                speed: 980.0,
                clip_size: 8,
                current_clip: 8,
                reserve_ammo: 48,
                reload_duration: 1.8,
                color: Color::new(1.0, 0.66, 0.0, 1.0),
                screen_shake: 6.0,
            },
            Box::new(BuckshotSpreadBehavior { pellets: 7 }),
        ),
        Weapon::new(
            WeaponStats {
                name: "RIFLE",
                damage: 23.0,
                fire_rate: 0.09,
                spread: 0.08,
                speed: 1300.0,
                clip_size: 30,
                current_clip: 30,
                reserve_ammo: 180,
                reload_duration: 1.4,
                color: Color::new(1.0, 0.84, 0.0, 1.0),
                screen_shake: 2.5,
            },
            Box::new(StandardBallisticBehavior),
        ),
        Weapon::new(
            WeaponStats {
                name: "PLASMA",
                damage: 52.0,
                fire_rate: 0.28,
                spread: 0.02,
                speed: 850.0,
                clip_size: 18,
                current_clip: 18,
                reserve_ammo: 72,
                reload_duration: 1.6,
                color: Color::new(0.66, 0.33, 0.97, 1.0),
                screen_shake: 4.0,
            },
            Box::new(PiercingPlasmaBehavior { base_pierce: 2 }),
        ),
        Weapon::new(
            WeaponStats {
                name: "RPG",
                damage: 200.0,
                fire_rate: 0.9,
                spread: 0.01,
                speed: 680.0,
                clip_size: 4,
                current_clip: 4,
                reserve_ammo: 12,
                reload_duration: 2.2,
                color: Color::new(1.0, 0.2, 0.27, 1.0),
                screen_shake: 14.0,
            },
            Box::new(ExplosiveRocketBehavior {
                blast_radius: 140.0,
            }),
        ),
    ]
}
