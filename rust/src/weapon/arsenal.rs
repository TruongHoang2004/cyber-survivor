use macroquad::prelude::*;
use crate::core::context::CombatContext;
use crate::core::props::WeaponRarity;
use crate::core::types::{PlayerModifiers, StatusEffect};
use crate::weapon::projectile::Projectile;
use crate::weapon::traits::{Weapon, WeaponBehavior, WeaponStats};

pub fn weapon_name_by_id(id: usize) -> &'static str {
    match id {
        0 => "PISTOL",
        1 => "SHOTGUN",
        2 => "VULCAN MINIGUN",
        3 => "TESLA CHAIN-GUN",
        4 => "ORBITAL RAILGUN",
        5 => "SINGULARITY CANNON",
        6 => "CLUSTER RPG",
        _ => "UNKNOWN WEAPON",
    }
}

fn resolve_status(mods: &PlayerModifiers) -> Option<StatusEffect> {
    if mods.incendiary_rounds {
        Some(StatusEffect::Burn {
            dps: 22.0,
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
    let level_bonus = 1.0 + ((stats.level.saturating_sub(1)) as f32) * 0.25;
    let rarity_bonus = stats.rarity.damage_bonus();
    let mut dmg = stats.damage * mods.dmg_multi * level_bonus * rarity_bonus;
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

// =========================================================
// 1. STANDARD / MINIGUN BALLISTIC BEHAVIOR
// =========================================================
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
        let streams = if stats.level >= 3 { 2 } else { 1 };
        let dmg = calc_damage(stats, mods, overcharged);
        let col = bullet_color(stats, mods, overcharged);

        for s in 0..streams {
            let offset_angle = if streams > 1 {
                ((s as f32) - 0.5) * 0.06
            } else {
                0.0
            };
            let angle = aim_angle + offset_angle + rand::gen_range(-stats.spread, stats.spread);
            let dir = vec2(angle.cos(), angle.sin());
            ctx.spawn_player_projectile(Projectile {
                pos: origin + dir * 24.0,
                vel: dir * stats.speed,
                damage: dmg,
                color: col,
                radius: 3.8,
                explosion_radius: None,
                spawns_black_hole: false,
                pierce: mods.pierce_count,
                bounces: if mods.bouncing_bullets { 2 } else { 0 },
                life: 2.2,
                status_effect: resolve_status(mods),
            });
        }
    }

    fn clone_box(&self) -> Box<dyn WeaponBehavior> {
        Box::new(self.clone())
    }
}

// =========================================================
// 2. BUCKSHOT SPREAD BEHAVIOR (Shotgun)
// =========================================================
#[derive(Clone)]
pub struct BuckshotSpreadBehavior {
    pub base_pellets: u32,
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
        let pellets = self.base_pellets + (stats.level.saturating_sub(1)) * 2;
        let dmg = calc_damage(stats, mods, overcharged);
        let col = bullet_color(stats, mods, overcharged);

        for _ in 0..pellets {
            let angle = aim_angle + rand::gen_range(-stats.spread, stats.spread);
            let dir = vec2(angle.cos(), angle.sin());
            ctx.spawn_player_projectile(Projectile {
                pos: origin + dir * 24.0,
                vel: dir * stats.speed * rand::gen_range(0.9, 1.1),
                damage: dmg,
                color: col,
                radius: 3.2,
                explosion_radius: None,
                spawns_black_hole: false,
                pierce: mods.pierce_count,
                bounces: if mods.bouncing_bullets { 2 } else { 0 },
                life: 1.3,
                status_effect: resolve_status(mods),
            });
        }
    }

    fn clone_box(&self) -> Box<dyn WeaponBehavior> {
        Box::new(self.clone())
    }
}

// =========================================================
// 3. TESLA CHAIN-LIGHTNING BEHAVIOR (Sét Liên Hoàn)
// =========================================================
#[derive(Clone)]
pub struct TeslaChainBehavior {
    pub base_jumps: usize,
}

impl WeaponBehavior for TeslaChainBehavior {
    fn fire(
        &mut self,
        origin: Vec2,
        aim_angle: f32,
        stats: &WeaponStats,
        mods: &PlayerModifiers,
        overcharged: bool,
        ctx: &mut CombatContext,
    ) {
        let dir = vec2(aim_angle.cos(), aim_angle.sin());
        let jumps = self.base_jumps + ((stats.level as usize).saturating_sub(1)) * 3;
        let dmg = calc_damage(stats, mods, overcharged);
        ctx.chain_lightning(origin + dir * 20.0, dir, jumps, 320.0, dmg);
    }

    fn clone_box(&self) -> Box<dyn WeaponBehavior> {
        Box::new(self.clone())
    }
}

// =========================================================
// 4. ORBITAL RAILGUN BEHAVIOR (Tia Điện Từ Xuyên Bản Đồ)
// =========================================================
#[derive(Clone)]
pub struct RailgunBeamBehavior;

impl WeaponBehavior for RailgunBeamBehavior {
    fn fire(
        &mut self,
        origin: Vec2,
        aim_angle: f32,
        stats: &WeaponStats,
        mods: &PlayerModifiers,
        overcharged: bool,
        ctx: &mut CombatContext,
    ) {
        let dir = vec2(aim_angle.cos(), aim_angle.sin());
        let start = origin + dir * 22.0;
        let end = origin + dir * 1500.0;
        let dmg = calc_damage(stats, mods, overcharged);
        let width = 26.0 + (stats.level as f32) * 4.0;
        ctx.railgun_strike(start, end, width, dmg);
    }

    fn clone_box(&self) -> Box<dyn WeaponBehavior> {
        Box::new(self.clone())
    }
}

// =========================================================
// 5. SINGULARITY CANNON BEHAVIOR (Súng Hố Đen)
// =========================================================
#[derive(Clone)]
pub struct SingularityCannonBehavior;

impl WeaponBehavior for SingularityCannonBehavior {
    fn fire(
        &mut self,
        origin: Vec2,
        aim_angle: f32,
        stats: &WeaponStats,
        mods: &PlayerModifiers,
        overcharged: bool,
        ctx: &mut CombatContext,
    ) {
        let dir = vec2(aim_angle.cos(), aim_angle.sin());
        let dmg = calc_damage(stats, mods, overcharged);
        ctx.spawn_player_projectile(Projectile {
            pos: origin + dir * 24.0,
            vel: dir * stats.speed,
            damage: dmg,
            color: Color::new(0.75, 0.2, 1.0, 1.0),
            radius: 10.0,
            explosion_radius: Some(120.0),
            spawns_black_hole: true,
            pierce: 0,
            bounces: 0,
            life: 2.2,
            status_effect: None,
        });
    }

    fn clone_box(&self) -> Box<dyn WeaponBehavior> {
        Box::new(self.clone())
    }
}

// =========================================================
// 6. EXPLOSIVE ROCKET BEHAVIOR (RPG)
// =========================================================
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
            explosion_radius: Some(self.blast_radius + (stats.level as f32) * 15.0),
            spawns_black_hole: false,
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
        // [1] PISTOL - Mở khóa sẵn từ đầu
        Weapon::new(
            WeaponStats {
                name: "PISTOL",
                damage: 30.0,
                fire_rate: 0.15,
                spread: 0.04,
                speed: 1180.0,
                clip_size: 16,
                current_clip: 16,
                reserve_ammo: -1,
                reload_duration: 1.0,
                color: Color::new(0.0, 0.94, 1.0, 1.0),
                screen_shake: 2.0,
                unlocked: true,
                level: 1,
                rarity: WeaponRarity::Common,
            },
            Box::new(StandardBallisticBehavior),
        ),
        // [2] SHOTGUN - Nhặt trên chiến trường
        Weapon::new(
            WeaponStats {
                name: "SHOTGUN",
                damage: 22.0,
                fire_rate: 0.58,
                spread: 0.30,
                speed: 1020.0,
                clip_size: 10,
                current_clip: 10,
                reserve_ammo: 60,
                reload_duration: 1.5,
                color: Color::new(1.0, 0.66, 0.0, 1.0),
                screen_shake: 6.5,
                unlocked: false,
                level: 1,
                rarity: WeaponRarity::Common,
            },
            Box::new(BuckshotSpreadBehavior { base_pellets: 8 }),
        ),
        // [3] VULCAN MINIGUN - Nhặt trên chiến trường (Xả đạn cực nhanh)
        Weapon::new(
            WeaponStats {
                name: "VULCAN MINIGUN",
                damage: 19.0,
                fire_rate: 0.045,
                spread: 0.14,
                speed: 1350.0,
                clip_size: 80,
                current_clip: 80,
                reserve_ammo: 400,
                reload_duration: 1.8,
                color: Color::new(1.0, 0.88, 0.15, 1.0),
                screen_shake: 2.2,
                unlocked: false,
                level: 1,
                rarity: WeaponRarity::Rare,
            },
            Box::new(StandardBallisticBehavior),
        ),
        // [4] TESLA CHAIN-GUN - Phóng sét nhảy qua 6-15 mục tiêu
        Weapon::new(
            WeaponStats {
                name: "TESLA CHAIN-GUN",
                damage: 46.0,
                fire_rate: 0.32,
                spread: 0.02,
                speed: 1000.0,
                clip_size: 18,
                current_clip: 18,
                reserve_ammo: 90,
                reload_duration: 1.5,
                color: Color::new(0.25, 0.9, 1.0, 1.0),
                screen_shake: 4.5,
                unlocked: false,
                level: 1,
                rarity: WeaponRarity::Epic,
            },
            Box::new(TeslaChainBehavior { base_jumps: 6 }),
        ),
        // [5] ORBITAL RAILGUN - Tia chớp xuyên thủng toàn bản đồ
        Weapon::new(
            WeaponStats {
                name: "ORBITAL RAILGUN",
                damage: 240.0,
                fire_rate: 0.75,
                spread: 0.0,
                speed: 2000.0,
                clip_size: 6,
                current_clip: 6,
                reserve_ammo: 30,
                reload_duration: 1.8,
                color: Color::new(0.0, 1.0, 0.85, 1.0),
                screen_shake: 12.0,
                unlocked: false,
                level: 1,
                rarity: WeaponRarity::Epic,
            },
            Box::new(RailgunBeamBehavior),
        ),
        // [6] SINGULARITY CANNON - Bắn ra Hố Đen hút hàng trăm quái vật
        Weapon::new(
            WeaponStats {
                name: "SINGULARITY",
                damage: 120.0,
                fire_rate: 1.1,
                spread: 0.01,
                speed: 560.0,
                clip_size: 3,
                current_clip: 3,
                reserve_ammo: 12,
                reload_duration: 2.1,
                color: Color::new(0.75, 0.2, 1.0, 1.0),
                screen_shake: 14.0,
                unlocked: false,
                level: 1,
                rarity: WeaponRarity::Legendary,
            },
            Box::new(SingularityCannonBehavior),
        ),
        // [7] CLUSTER RPG - Tên lửa hạng nặng
        Weapon::new(
            WeaponStats {
                name: "CLUSTER RPG",
                damage: 220.0,
                fire_rate: 0.85,
                spread: 0.01,
                speed: 720.0,
                clip_size: 4,
                current_clip: 4,
                reserve_ammo: 16,
                reload_duration: 2.0,
                color: Color::new(1.0, 0.2, 0.27, 1.0),
                screen_shake: 15.0,
                unlocked: false,
                level: 1,
                rarity: WeaponRarity::Rare,
            },
            Box::new(ExplosiveRocketBehavior {
                blast_radius: 155.0,
            }),
        ),
    ]
}
