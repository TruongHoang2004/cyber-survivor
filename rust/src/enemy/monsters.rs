use macroquad::prelude::*;
use std::f32::consts::PI;
use crate::core::context::CombatContext;
use crate::core::events::{MeteorWarningStrikeEvent, ShockwaveRingEvent};
use crate::core::props::{WeaponDrop, WeaponRarity};
use crate::core::types::StatusEffect;
use crate::enemy::traits::{BossPartBehavior, EnemyBehavior, EnemyBody, EnemyEntity};
use crate::weapon::projectile::Projectile;

// ==========================================
// 1. SWARM RUNNER (Quái bầy đàn áp sát)
// ==========================================
pub struct SwarmRunner;

impl EnemyBehavior for SwarmRunner {
    fn update(&mut self, body: &mut EnemyBody, ctx: &mut CombatContext, dt: f32) {
        let to_player = ctx.player_pos - body.pos;
        let dist = to_player.length();
        let dir = to_player.normalize_or_zero();

        body.pos += dir * body.speed * dt;

        if dist < ctx.player_radius + body.radius {
            ctx.damage_player(14.0);
            body.vel -= dir * 200.0;
        }
    }
}

// ==========================================
// 2. ACID SPITTER (Quái phun độc giữ khoảng cách)
// ==========================================
pub struct AcidSpitter {
    pub shoot_timer: f32,
}

impl EnemyBehavior for AcidSpitter {
    fn update(&mut self, body: &mut EnemyBody, ctx: &mut CombatContext, dt: f32) {
        let to_player = ctx.player_pos - body.pos;
        let dist = to_player.length();
        let dir = to_player.normalize_or_zero();

        if dist < 260.0 {
            body.pos -= dir * body.speed * dt;
        } else if dist > 400.0 {
            body.pos += dir * body.speed * dt;
        }

        self.shoot_timer += dt;
        if self.shoot_timer >= 2.0 {
            self.shoot_timer = 0.0;
            ctx.spawn_enemy_projectile(Projectile {
                pos: body.pos,
                vel: dir * 380.0,
                damage: 18.0,
                color: Color::new(0.0, 1.0, 0.4, 1.0),
                radius: 6.0,
                explosion_radius: None,
                spawns_black_hole: false,
                pierce: 0,
                bounces: 0,
                life: 3.0,
                status_effect: Some(StatusEffect::Slow {
                    factor: 0.6,
                    duration: 1.5,
                }),
            });
        }
    }
}

// ==========================================
// 3. KAMIKAZE DRONE (Drone cảm tử phát nổ)
// ==========================================
pub struct KamikazeDrone {
    pub pulse: f32,
}

impl EnemyBehavior for KamikazeDrone {
    fn update(&mut self, body: &mut EnemyBody, ctx: &mut CombatContext, dt: f32) {
        self.pulse += dt * 10.0;
        let to_player = ctx.player_pos - body.pos;
        let dist = to_player.length();
        let dir = to_player.normalize_or_zero();

        body.pos += dir * body.speed * dt;

        if dist < 42.0 {
            body.hp = 0.0;
        }
    }

    fn on_death(&mut self, body: &EnemyBody, ctx: &mut CombatContext) {
        ctx.trigger_explosion(body.pos, 85.0, 35.0);
        if (ctx.player_pos - body.pos).length() < 85.0 + ctx.player_radius {
            ctx.damage_player(30.0);
        }
    }

    fn draw_custom(&self, body: &EnemyBody, screen_pos: Vec2, _cam_offset: Vec2) {
        let r = body.radius + 4.0 + self.pulse.sin().abs() * 5.0;
        draw_circle_lines(
            screen_pos.x,
            screen_pos.y,
            r,
            1.5,
            Color::new(1.0, 0.5, 0.0, 0.6),
        );
    }
}

// ==========================================
// 4. BRUTE CHARGER (Quái húc hạng nặng - Rơi Súng khi hạ!)
// ==========================================
pub struct BruteCharger {
    pub charge_timer: f32,
    pub is_charging: bool,
    pub charge_angle: f32,
}

impl EnemyBehavior for BruteCharger {
    fn update(&mut self, body: &mut EnemyBody, ctx: &mut CombatContext, dt: f32) {
        let to_player = ctx.player_pos - body.pos;
        let dist = to_player.length();
        let dir = to_player.normalize_or_zero();

        self.charge_timer += dt;
        if self.is_charging {
            let cdir = vec2(self.charge_angle.cos(), self.charge_angle.sin());
            body.pos += cdir * 540.0 * dt;
            if dist < ctx.player_radius + body.radius {
                ctx.damage_player(35.0);
                self.is_charging = false;
            }
            if self.charge_timer > 0.75 {
                self.is_charging = false;
                self.charge_timer = 0.0;
            }
        } else {
            body.pos += dir * body.speed * dt;
            if self.charge_timer >= 3.2 && dist < 440.0 {
                self.is_charging = true;
                self.charge_angle = to_player.y.atan2(to_player.x);
                self.charge_timer = 0.0;
                ctx.spawn_float_text(body.pos - vec2(0.0, 20.0), "CHARGE!", RED, 16.0);
            }
        }
    }

    fn on_death(&mut self, body: &EnemyBody, ctx: &mut CombatContext) {
        // Quái Tinh Anh (Brute) có 45% tỉ lệ rơi thẳng Súng xuống đất!
        if rand::gen_range(0.0, 1.0) < 0.45 {
            ctx.spawn_weapon_drop(WeaponDrop::new(
                body.pos,
                rand::gen_range(1, 7),
                WeaponRarity::random(),
            ));
        }
    }

    fn draw_custom(&self, body: &EnemyBody, screen_pos: Vec2, _cam_offset: Vec2) {
        if self.is_charging {
            let dir = vec2(self.charge_angle.cos(), self.charge_angle.sin());
            draw_line(
                screen_pos.x,
                screen_pos.y,
                screen_pos.x + dir.x * 120.0,
                screen_pos.y + dir.y * 120.0,
                3.0,
                Color::new(1.0, 0.1, 0.3, 0.6),
            );
        } else {
            draw_circle_lines(
                screen_pos.x,
                screen_pos.y,
                body.radius + 3.0,
                2.5,
                Color::new(0.8, 0.4, 1.0, 0.8),
            );
        }
    }
}

// ==========================================
// 5. SPLITTER SLIME (Quái phân tách thành 2 khi chết)
// ==========================================
pub struct SplitterSlime {
    pub generation: u8,
}

impl EnemyBehavior for SplitterSlime {
    fn update(&mut self, body: &mut EnemyBody, ctx: &mut CombatContext, dt: f32) {
        let to_player = ctx.player_pos - body.pos;
        let dist = to_player.length();
        let dir = to_player.normalize_or_zero();

        body.pos += dir * body.speed * dt;

        if dist < ctx.player_radius + body.radius {
            ctx.damage_player(if self.generation > 0 { 16.0 } else { 9.0 });
            body.vel -= dir * 220.0;
        }
    }

    fn on_death(&mut self, body: &EnemyBody, ctx: &mut CombatContext) {
        ctx.spawn_particle_burst(body.pos, body.color, 14, 180.0, 0.35);
        ctx.spawn_decal(crate::core::types::Decal {
            pos: body.pos,
            radius: body.radius * 1.1,
            color: Color::new(0.0, 0.85, 0.6, 0.4),
            life: 10.0,
            is_damage_zone: false,
        });

        if self.generation > 0 {
            for offset_angle in [-1.2f32, 1.2f32] {
                let burst_dir = vec2(offset_angle.cos(), offset_angle.sin());
                let mini_body = EnemyBody {
                    name: "MINI SLIME",
                    pos: body.pos + burst_dir * 12.0,
                    vel: burst_dir * 260.0,
                    radius: 10.0,
                    speed: 235.0,
                    color: Color::new(0.1, 0.95, 0.75, 1.0),
                    hp: body.max_hp * 0.4,
                    max_hp: body.max_hp * 0.4,
                    is_boss: false,
                    score_reward: 60,
                    hit_flash: 0.0,
                    statuses: Vec::new(),
                    parts: Vec::new(),
                };
                ctx.spawn_enemy(EnemyEntity::new(
                    mini_body,
                    Box::new(SplitterSlime { generation: 0 }),
                ));
            }
        }
    }
}

// ==========================================
// 6. SNIPER BOT (Robot bắn tỉa có tia Laser đỏ)
// ==========================================
pub struct SniperBot {
    pub aim_timer: f32,
    pub target_pos: Vec2,
}

impl EnemyBehavior for SniperBot {
    fn update(&mut self, body: &mut EnemyBody, ctx: &mut CombatContext, dt: f32) {
        self.target_pos = ctx.player_pos;
        let to_player = ctx.player_pos - body.pos;
        let dist = to_player.length();
        let dir = to_player.normalize_or_zero();

        if dist < 380.0 {
            body.pos -= dir * body.speed * dt;
        } else if dist > 580.0 {
            body.pos += dir * body.speed * dt;
        }

        self.aim_timer += dt;
        if self.aim_timer >= 2.4 {
            self.aim_timer = 0.0;
            ctx.spawn_enemy_projectile(Projectile {
                pos: body.pos,
                vel: dir * 820.0,
                damage: 30.0,
                color: Color::new(1.0, 0.1, 0.2, 1.0),
                radius: 4.5,
                explosion_radius: None,
                spawns_black_hole: false,
                pierce: 0,
                bounces: 0,
                life: 2.5,
                status_effect: None,
            });
        }
    }

    fn draw_custom(&self, _body: &EnemyBody, screen_pos: Vec2, cam_offset: Vec2) {
        if self.aim_timer > 1.0 {
            let target_sp = self.target_pos + cam_offset;
            let alpha = ((self.aim_timer - 1.0) / 1.4).clamp(0.15, 0.85);
            draw_line(
                screen_pos.x,
                screen_pos.y,
                target_sp.x,
                target_sp.y,
                1.5,
                Color::new(1.0, 0.05, 0.15, alpha),
            );
        }
    }
}

// =========================================================
// 7. CÁC BỘ PHẬN ĐỘC LẬP CỦA ĐẠI BOSS (Multi-Part Boss Modules)
// =========================================================

/// Bộ phận 1: Cánh tay Pháo Cối (Bắn đạn pháo có Vòng Cảnh Báo Đỏ Telegraph)
pub struct MortarArtilleryArmPart {
    pub timer: f32,
}

impl BossPartBehavior for MortarArtilleryArmPart {
    fn update_part(
        &mut self,
        _boss_pos: Vec2,
        _part_world_pos: Vec2,
        ctx: &mut CombatContext,
        dt: f32,
    ) {
        self.timer += dt;
        if self.timer >= 3.2 {
            self.timer = 0.0;
            for _ in 0..3 {
                let offset = vec2(
                    rand::gen_range(-170.0, 170.0),
                    rand::gen_range(-170.0, 170.0),
                );
                ctx.spawn_event(Box::new(MeteorWarningStrikeEvent {
                    target_pos: ctx.player_pos + offset,
                    radius: 85.0,
                    damage: 34.0,
                    timer: 0.0,
                    max_time: 1.35,
                    harms_player: true,
                }));
            }
        }
    }
}

/// Bộ phận 2: Ụ Súng Laser Liên Thanh (Xả chùm đạn năng lượng tốc độ cao)
pub struct GatlingLaserTurretPart {
    pub timer: f32,
}

impl BossPartBehavior for GatlingLaserTurretPart {
    fn update_part(
        &mut self,
        _boss_pos: Vec2,
        part_world_pos: Vec2,
        ctx: &mut CombatContext,
        dt: f32,
    ) {
        self.timer += dt;
        if self.timer >= 1.5 {
            self.timer = 0.0;
            let to_player = ctx.player_pos - part_world_pos;
            let base_angle = to_player.y.atan2(to_player.x);
            for offset in [-0.28f32, -0.14, 0.0, 0.14, 0.28] {
                let a = base_angle + offset;
                ctx.spawn_enemy_projectile(Projectile {
                    pos: part_world_pos,
                    vel: vec2(a.cos(), a.sin()) * 450.0,
                    damage: 22.0,
                    color: Color::new(1.0, 0.85, 0.0, 1.0),
                    radius: 6.5,
                    explosion_radius: None,
                    spawns_black_hole: false,
                    pierce: 0,
                    bounces: 0,
                    life: 3.0,
                    status_effect: None,
                });
            }
        }
    }
}

/// Bộ phận 3: Lõi Phát Khiên Năng Lượng (Giảm 65% sát thương vào Boss cho đến khi bị bắn vỡ!)
pub struct ShieldGeneratorPart;

impl BossPartBehavior for ShieldGeneratorPart {
    fn update_part(
        &mut self,
        _boss_pos: Vec2,
        _part_world_pos: Vec2,
        _ctx: &mut CombatContext,
        _dt: f32,
    ) {
    }

    fn is_shield_generator(&self) -> bool {
        true
    }
}

/// Bộ phận 4: Khoang Đẻ Drone Cảm Tử (Triệu hồi Kamikaze Drone liên tục)
pub struct DroneHatcheryBayPart {
    pub timer: f32,
    pub wave_level: u32,
}

impl BossPartBehavior for DroneHatcheryBayPart {
    fn update_part(
        &mut self,
        _boss_pos: Vec2,
        part_world_pos: Vec2,
        ctx: &mut CombatContext,
        dt: f32,
    ) {
        self.timer += dt;
        if self.timer >= 4.2 {
            self.timer = 0.0;
            ctx.spawn_enemy(EnemyEntity::new(
                EnemyBody {
                    name: "MINION DRONE",
                    pos: part_world_pos,
                    vel: Vec2::ZERO,
                    radius: 12.0,
                    speed: 255.0,
                    color: Color::new(1.0, 0.66, 0.0, 1.0),
                    hp: 35.0 + (self.wave_level as f32) * 4.0,
                    max_hp: 35.0 + (self.wave_level as f32) * 4.0,
                    is_boss: false,
                    score_reward: 80,
                    hit_flash: 0.0,
                    statuses: Vec::new(),
                    parts: Vec::new(),
                },
                Box::new(KamikazeDrone { pulse: 0.0 }),
            ));
        }
    }
}

// ==========================================
// 8. COLOSSUS FORTRESS BOSS (Thân chính của Đại Boss)
// ==========================================
pub struct CyberTitanBoss {
    pub attack_timer: f32,
}

impl EnemyBehavior for CyberTitanBoss {
    fn update(&mut self, body: &mut EnemyBody, ctx: &mut CombatContext, dt: f32) {
        let to_player = ctx.player_pos - body.pos;
        let dist = to_player.length();
        let dir = to_player.normalize_or_zero();

        body.pos += dir * body.speed * dt;
        self.attack_timer += dt;

        // Vòng đạn xoáy 12 hướng từ Lõi Trung Tâm
        if self.attack_timer >= 2.4 {
            self.attack_timer = 0.0;
            ctx.spawn_event(Box::new(ShockwaveRingEvent {
                center: body.pos,
                current_radius: body.radius,
                max_radius: 190.0,
                speed: 420.0,
                color: Color::new(1.0, 0.0, 0.4, 0.85),
                thickness: 5.0,
            }));

            for a in 0..12 {
                let angle = (a as f32) * (PI * 2.0 / 12.0);
                ctx.spawn_enemy_projectile(Projectile {
                    pos: body.pos,
                    vel: vec2(angle.cos(), angle.sin()) * 320.0,
                    damage: 24.0,
                    color: Color::new(1.0, 0.0, 0.35, 1.0),
                    radius: 7.0,
                    explosion_radius: None,
                    spawns_black_hole: false,
                    pierce: 0,
                    bounces: 0,
                    life: 3.5,
                    status_effect: None,
                });
            }
        }

        if dist < ctx.player_radius + body.radius {
            ctx.damage_player(40.0);
        }
    }

    fn on_death(&mut self, body: &EnemyBody, ctx: &mut CombatContext) {
        ctx.trigger_explosion(body.pos, 220.0, 500.0);
        // Chắc chắn rơi Súng Huyền Thoại (Legendary) khi tiêu diệt Đại Boss!
        ctx.spawn_weapon_drop(WeaponDrop::new(
            body.pos,
            rand::gen_range(2, 7),
            WeaponRarity::Legendary,
        ));
    }

    fn draw_custom(&self, body: &EnemyBody, screen_pos: Vec2, _cam_offset: Vec2) {
        draw_circle_lines(
            screen_pos.x,
            screen_pos.y,
            body.radius + 6.0,
            3.0,
            Color::new(1.0, 0.0, 0.4, 0.7),
        );
    }
}
