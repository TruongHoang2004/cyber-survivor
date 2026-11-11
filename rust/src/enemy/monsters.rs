use macroquad::prelude::*;
use std::f32::consts::PI;
use crate::core::context::CombatContext;
use crate::core::events::{MeteorWarningStrikeEvent, ShockwaveRingEvent};
use crate::core::props::{WeaponDrop, WeaponRarity};
use crate::core::types::StatusEffect;
use crate::enemy::traits::{BossPartBehavior, EnemyBehavior, EnemyBody, EnemyEntity};
use crate::weapon::projectile::Projectile;

// ==========================================
// 1. SWARM RUNNER (Bọ Cạp Máy Cận Chiến - Hình dáng Mũi Tên 6 Chân + Càng Kẹp)
// ==========================================
pub struct SwarmRunner {
    pub facing: f32,
    pub leg_phase: f32,
}

impl EnemyBehavior for SwarmRunner {
    fn update(&mut self, body: &mut EnemyBody, ctx: &mut CombatContext, dt: f32) {
        let to_player = ctx.player_pos - body.pos;
        let dist = to_player.length();
        let dir = to_player.normalize_or_zero();
        self.facing = to_player.y.atan2(to_player.x);
        self.leg_phase += dt * 18.0;

        body.pos += dir * body.speed * dt;

        if dist < ctx.player_radius + body.radius {
            ctx.damage_player(24.0);
            body.vel -= dir * 200.0;
        }
    }

    fn draw_custom(&self, body: &EnemyBody, sp: Vec2, _cam_offset: Vec2) {
        let col = if body.hit_flash > 0.0 { WHITE } else { body.color };
        let fwd = vec2(self.facing.cos(), self.facing.sin());
        let right = vec2(-fwd.y, fwd.x);
        let r = body.radius;

        // 6 chân nhện máy cử động hai bên sườn
        for i in [-1.0f32, 0.0, 1.0] {
            let swing = (self.leg_phase + i * 1.5).sin() * 4.0;
            let base = sp + fwd * (i * 5.0);
            for side in [-1.0f32, 1.0] {
                let tip = base + right * (side * (r + 6.0)) + fwd * swing;
                draw_line(
                    base.x,
                    base.y,
                    tip.x,
                    tip.y,
                    2.2,
                    Color::new(0.85, 0.25, 0.3, 0.9),
                );
            }
        }

        // Thân hình mũi giáo tam giác nhọn lao về phía trước
        let nose = sp + fwd * (r * 1.35);
        let tail_l = sp - fwd * (r * 0.95) + right * (r * 0.85);
        let tail_r = sp - fwd * (r * 0.95) - right * (r * 0.85);
        let back_notch = sp - fwd * (r * 0.45);
        draw_triangle(nose, tail_l, back_notch, col);
        draw_triangle(nose, back_notch, tail_r, col);
        draw_triangle_lines(nose, tail_l, tail_r, 2.0, Color::new(1.0, 0.65, 0.7, 0.95));

        // 2 càng kẹp sắc nhọn (Mandibles) ở mũi
        let p_l = nose + fwd * 5.0 + right * 4.5;
        let p_r = nose + fwd * 5.0 - right * 4.5;
        draw_line(nose.x - right.x * 3.0, nose.y - right.y * 3.0, p_r.x, p_r.y, 2.2, WHITE);
        draw_line(nose.x + right.x * 3.0, nose.y + right.y * 3.0, p_l.x, p_l.y, 2.2, WHITE);
    }
}

// ==========================================
// 2. ACID SPITTER (Bọ Phun Axit Sinh Học - Hình Thoi Cánh Cứng + 2 Túi Độc & Vòi Phun)
// ==========================================
pub struct AcidSpitter {
    pub shoot_timer: f32,
    pub facing: f32,
}

impl EnemyBehavior for AcidSpitter {
    fn update(&mut self, body: &mut EnemyBody, ctx: &mut CombatContext, dt: f32) {
        let to_player = ctx.player_pos - body.pos;
        let dist = to_player.length();
        let dir = to_player.normalize_or_zero();
        self.facing = to_player.y.atan2(to_player.x);

        if dist < 260.0 {
            body.pos -= dir * body.speed * dt;
        } else if dist > 400.0 {
            body.pos += dir * body.speed * dt;
        }

        self.shoot_timer += dt;
        if self.shoot_timer >= 1.35 {
            self.shoot_timer = 0.0;
            ctx.spawn_enemy_projectile(Projectile {
                pos: body.pos + dir * (body.radius + 6.0),
                vel: dir * 440.0,
                damage: 26.0,
                color: Color::new(0.15, 1.0, 0.35, 1.0),
                radius: 6.5,
                explosion_radius: None,
                spawns_black_hole: false,
                pierce: 0,
                bounces: 0,
                life: 3.0,
                status_effect: Some(StatusEffect::Slow {
                    factor: 0.5,
                    duration: 1.8,
                }),
            });
        }
    }

    fn draw_custom(&self, body: &EnemyBody, sp: Vec2, _cam_offset: Vec2) {
        let col = if body.hit_flash > 0.0 { WHITE } else { body.color };
        let fwd = vec2(self.facing.cos(), self.facing.sin());
        let right = vec2(-fwd.y, fwd.x);
        let r = body.radius;
        let sac_pulse = (self.shoot_timer * 4.0).sin().abs() * 2.5;

        // 2 túi chứa dịch độc phát sáng phồng xẹp hai bên hông
        for side in [-1.0f32, 1.0] {
            let sac_pos = sp - fwd * 2.0 + right * (side * (r * 0.85));
            draw_poly(
                sac_pos.x,
                sac_pos.y,
                5,
                6.5 + sac_pulse,
                0.0,
                Color::new(0.45, 1.0, 0.15, 0.9),
            );
        }

        // Thân hình thoi (Diamond Rhombus) bọc giáp tối màu
        let front = sp + fwd * (r * 1.15);
        let back = sp - fwd * (r * 1.25);
        let side_l = sp + right * (r * 0.75);
        let side_r = sp - right * (r * 0.75);
        draw_triangle(front, side_l, side_r, Color::new(0.05, 0.28, 0.14, 0.98));
        draw_triangle(back, side_l, side_r, col);
        draw_line(front.x, front.y, side_l.x, side_l.y, 2.0, col);
        draw_line(front.x, front.y, side_r.x, side_r.y, 2.0, col);
        draw_line(back.x, back.y, side_l.x, side_l.y, 2.0, WHITE);
        draw_line(back.x, back.y, side_r.x, side_r.y, 2.0, WHITE);

        // Vòi phun axit kép phía trước
        let noz_end = sp + fwd * (r + 9.0);
        draw_line(sp.x, sp.y, noz_end.x, noz_end.y, 5.0, Color::new(0.2, 0.95, 0.3, 1.0));
    }
}

// ==========================================
// 3. KAMIKAZE DRONE (Tên Lửa Cảm Tử Cánh Dơi - Hình Phi Cơ Delta-Wing + Lửa Đuôi)
// ==========================================
pub struct KamikazeDrone {
    pub pulse: f32,
    pub facing: f32,
}

impl EnemyBehavior for KamikazeDrone {
    fn update(&mut self, body: &mut EnemyBody, ctx: &mut CombatContext, dt: f32) {
        self.pulse += dt * 14.0;
        let to_player = ctx.player_pos - body.pos;
        let dist = to_player.length();
        let dir = to_player.normalize_or_zero();
        self.facing = to_player.y.atan2(to_player.x);

        body.pos += dir * body.speed * dt;

        if dist < 42.0 {
            body.hp = 0.0;
        }
    }

    fn on_death(&mut self, body: &EnemyBody, ctx: &mut CombatContext) {
        ctx.trigger_explosion(body.pos, 95.0, 48.0);
        if (ctx.player_pos - body.pos).length() < 95.0 + ctx.player_radius {
            ctx.damage_player(48.0);
        }
    }

    fn draw_custom(&self, body: &EnemyBody, sp: Vec2, _cam_offset: Vec2) {
        let col = if body.hit_flash > 0.0 { WHITE } else { body.color };
        let fwd = vec2(self.facing.cos(), self.facing.sin());
        let right = vec2(-fwd.y, fwd.x);
        let r = body.radius;

        // Ngọn lửa phản lực phụt phía đuôi
        let jet_len = 8.0 + self.pulse.sin().abs() * 8.0;
        let tail = sp - fwd * (r * 0.8);
        let flame_tip = tail - fwd * jet_len;
        draw_triangle(
            tail + right * 4.0,
            tail - right * 4.0,
            flame_tip,
            Color::new(1.0, 0.35, 0.0, 0.9),
        );

        // Cánh phi cơ cảm tử xuôi về sau (Swept Delta Jet)
        let nose = sp + fwd * (r * 1.45);
        let wing_l = sp - fwd * (r * 0.7) + right * (r * 1.35);
        let wing_r = sp - fwd * (r * 0.7) - right * (r * 1.35);
        draw_triangle(nose, wing_l, wing_r, col);
        draw_triangle_lines(nose, wing_l, wing_r, 2.0, YELLOW);

        // Đèn cảnh báo bom nhấp nháy đỏ-trắng ở giữa thân
        let blink = if (self.pulse as i32) % 2 == 0 {
            Color::new(1.0, 0.1, 0.1, 1.0)
        } else {
            WHITE
        };
        draw_rectangle(sp.x - 3.5, sp.y - 3.5, 7.0, 7.0, blink);
    }
}

// ==========================================
// 4. BRUTE CHARGER (Xe Tăng Ủi Hạng Nặng - Khối Chữ Nhật Bánh Xích + Lưỡi Ủi Gai Chữ V)
// ==========================================
pub struct BruteCharger {
    pub charge_timer: f32,
    pub is_charging: bool,
    pub charge_angle: f32,
    pub facing: f32,
}

impl EnemyBehavior for BruteCharger {
    fn update(&mut self, body: &mut EnemyBody, ctx: &mut CombatContext, dt: f32) {
        let to_player = ctx.player_pos - body.pos;
        let dist = to_player.length();
        let dir = to_player.normalize_or_zero();

        self.charge_timer += dt;
        if self.is_charging {
            self.facing = self.charge_angle;
            let cdir = vec2(self.charge_angle.cos(), self.charge_angle.sin());
            body.pos += cdir * 580.0 * dt;
            if dist < ctx.player_radius + body.radius {
                ctx.damage_player(55.0);
                self.is_charging = false;
            }
            if self.charge_timer > 0.75 {
                self.is_charging = false;
                self.charge_timer = 0.0;
            }
        } else {
            self.facing = to_player.y.atan2(to_player.x);
            body.pos += dir * body.speed * dt;
            if self.charge_timer >= 3.2 && dist < 440.0 {
                self.is_charging = true;
                self.charge_angle = self.facing;
                self.charge_timer = 0.0;
                ctx.spawn_float_text(body.pos - vec2(0.0, 20.0), "CHARGE!", RED, 16.0);
            }
        }
    }

    fn on_death(&mut self, body: &EnemyBody, ctx: &mut CombatContext) {
        // Giảm tỉ lệ rơi súng của Brute từ 45% xuống 12% để súng quý hiếm và đỡ loạn màn hình
        if rand::gen_range(0.0, 1.0) < 0.12 {
            ctx.spawn_weapon_drop(WeaponDrop::new(
                body.pos,
                rand::gen_range(1, 7),
                WeaponRarity::random(),
            ));
        }
    }

    fn draw_custom(&self, body: &EnemyBody, sp: Vec2, _cam_offset: Vec2) {
        let col = if body.hit_flash > 0.0 { WHITE } else { body.color };
        let fwd = vec2(self.facing.cos(), self.facing.sin());
        let right = vec2(-fwd.y, fwd.x);
        let r = body.radius;

        if self.is_charging {
            draw_line(
                sp.x,
                sp.y,
                sp.x + fwd.x * 135.0,
                sp.y + fwd.y * 135.0,
                4.0,
                Color::new(1.0, 0.15, 0.3, 0.7),
            );
        }

        // 2 bánh xích xe tăng hạng nặng hai bên
        for side in [-1.0f32, 1.0] {
            let track_start = sp - fwd * (r * 0.9) + right * (side * r * 0.85);
            let track_end = sp + fwd * (r * 0.9) + right * (side * r * 0.85);
            draw_line(
                track_start.x,
                track_start.y,
                track_end.x,
                track_end.y,
                8.0,
                Color::new(0.18, 0.15, 0.25, 1.0),
            );
        }

        // Khung xe tăng hình hộp vuông vức bọc thép
        let fl = sp + fwd * (r * 0.85) + right * (r * 0.75);
        let fr = sp + fwd * (r * 0.85) - right * (r * 0.75);
        let bl = sp - fwd * (r * 0.95) + right * (r * 0.75);
        let br = sp - fwd * (r * 0.95) - right * (r * 0.75);
        draw_triangle(fl, fr, bl, col);
        draw_triangle(fr, br, bl, col);
        draw_line(fl.x, fl.y, fr.x, fr.y, 2.5, WHITE);
        draw_line(bl.x, bl.y, br.x, br.y, 2.5, WHITE);

        // Lưỡi ủi gai thép chữ V (Spiked Ramming Plow) ở mũi xe
        let plow_tip = sp + fwd * (r * 1.45);
        let plow_l = fl + right * 5.0;
        let plow_r = fr - right * 5.0;
        draw_triangle(
            plow_tip,
            plow_l,
            plow_r,
            if self.is_charging {
                Color::new(1.0, 0.3, 0.2, 1.0)
            } else {
                Color::new(0.88, 0.75, 1.0, 1.0)
            },
        );
    }
}

// ==========================================
// 5. SPLITTER SLIME (Tinh Thể Phân Bào - Cấu Trúc 3 Thùy Tinh Thể Xoay)
// ==========================================
pub struct SplitterSlime {
    pub generation: u8,
    pub rot_angle: f32,
}

impl EnemyBehavior for SplitterSlime {
    fn update(&mut self, body: &mut EnemyBody, ctx: &mut CombatContext, dt: f32) {
        self.rot_angle += dt * 3.5;
        let to_player = ctx.player_pos - body.pos;
        let dist = to_player.length();
        let dir = to_player.normalize_or_zero();

        body.pos += dir * body.speed * dt;

        if dist < ctx.player_radius + body.radius {
            ctx.damage_player(if self.generation > 0 { 22.0 } else { 12.0 });
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
                    Box::new(SplitterSlime {
                        generation: 0,
                        rot_angle: 0.0,
                    }),
                ));
            }
        }
    }

    fn draw_custom(&self, body: &EnemyBody, sp: Vec2, _cam_offset: Vec2) {
        let col = if body.hit_flash > 0.0 { WHITE } else { body.color };
        let r = body.radius;

        // Vẽ 3 khối tinh thể tam giác liên kết xoay quanh tâm (nhìn rõ cấu trúc sẽ tách đôi!)
        let lobes = if self.generation > 0 { 3 } else { 2 };
        for i in 0..lobes {
            let a = self.rot_angle + (i as f32) * (PI * 2.0 / (lobes as f32));
            let offset = vec2(a.cos(), a.sin()) * (r * 0.48);
            let lp = sp + offset;
            draw_poly(lp.x, lp.y, 4, r * 0.62, a * 57.3, col);
            draw_poly_lines(lp.x, lp.y, 4, r * 0.62, a * 57.3, 2.0, WHITE);
        }
    }
}

// ==========================================
// 6. SNIPER BOT (Pháo Kích Bắn Tỉa Chữ X - 4 Chân Nhện Chữ X + Nòng Railgun Dài)
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
        if self.aim_timer >= 1.6 {
            self.aim_timer = 0.0;
            ctx.spawn_enemy_projectile(Projectile {
                pos: body.pos + dir * (body.radius + 12.0),
                vel: dir * 880.0,
                damage: 52.0,
                color: Color::new(1.0, 0.1, 0.2, 1.0),
                radius: 5.0,
                explosion_radius: None,
                spawns_black_hole: false,
                pierce: 0,
                bounces: 0,
                life: 2.5,
                status_effect: None,
            });
        }
    }

    fn draw_custom(&self, body: &EnemyBody, sp: Vec2, cam_offset: Vec2) {
        let col = if body.hit_flash > 0.0 { WHITE } else { body.color };
        let to_target = (self.target_pos + cam_offset) - sp;
        let fwd = to_target.normalize_or_zero();
        let right = vec2(-fwd.y, fwd.x);
        let r = body.radius;

        if self.aim_timer > 1.0 {
            let target_sp = self.target_pos + cam_offset;
            let alpha = ((self.aim_timer - 1.0) / 1.4).clamp(0.15, 0.85);
            draw_line(
                sp.x,
                sp.y,
                target_sp.x,
                target_sp.y,
                1.8,
                Color::new(1.0, 0.05, 0.2, alpha),
            );
        }

        // 4 chân chống chữ X (X-Strider Stabilizers)
        let d1 = (fwd + right).normalize_or_zero() * (r * 1.25);
        let d2 = (fwd - right).normalize_or_zero() * (r * 1.25);
        draw_line(sp.x - d1.x, sp.y - d1.y, sp.x + d1.x, sp.y + d1.y, 3.5, Color::new(0.4, 0.15, 0.3, 1.0));
        draw_line(sp.x - d2.x, sp.y - d2.y, sp.x + d2.x, sp.y + d2.y, 3.5, Color::new(0.4, 0.15, 0.3, 1.0));

        // Thân tháp pháo lục giác dẹp
        draw_poly(sp.x, sp.y, 4, r * 0.85, 45.0, col);
        draw_poly_lines(sp.x, sp.y, 4, r * 0.85, 45.0, 2.0, WHITE);

        // Nòng súng bắn tỉa siêu dài (Long Sniper Barrel)
        let barrel_tip = sp + fwd * (r + 16.0);
        draw_line(sp.x, sp.y, barrel_tip.x, barrel_tip.y, 4.5, Color::new(0.85, 0.88, 0.95, 1.0));
        draw_circle(sp.x + fwd.x * 4.0, sp.y + fwd.y * 4.0, 4.0, RED);
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
                Box::new(KamikazeDrone {
                    pulse: 0.0,
                    facing: 0.0,
                }),
            ));
        }
    }
}

// ==========================================
// 8. COLOSSUS FORTRESS BOSS (Pháo Đài Cơ Khí Bát Giác Khổng Lồ)
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

        // Vòng đạn xoáy 16 hướng từ Lõi Trung Tâm
        if self.attack_timer >= 2.0 {
            self.attack_timer = 0.0;
            ctx.spawn_event(Box::new(ShockwaveRingEvent {
                center: body.pos,
                current_radius: body.radius,
                max_radius: 220.0,
                speed: 460.0,
                color: Color::new(1.0, 0.0, 0.4, 0.85),
                thickness: 5.5,
            }));

            for a in 0..16 {
                let angle = (a as f32) * (PI * 2.0 / 16.0);
                ctx.spawn_enemy_projectile(Projectile {
                    pos: body.pos,
                    vel: vec2(angle.cos(), angle.sin()) * 340.0,
                    damage: 32.0,
                    color: Color::new(1.0, 0.0, 0.35, 1.0),
                    radius: 7.5,
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
            ctx.damage_player(65.0);
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

    fn draw_custom(&self, body: &EnemyBody, sp: Vec2, _cam_offset: Vec2) {
        let col = if body.hit_flash > 0.0 { WHITE } else { body.color };
        let r = body.radius;
        let t = get_time() as f32;

        // Khung pháo đài bát giác ngoài + lõi sao xoay ngược chiều
        draw_poly(sp.x, sp.y, 8, r, t * 18.0, Color::new(0.14, 0.04, 0.09, 0.98));
        draw_poly_lines(sp.x, sp.y, 8, r, t * 18.0, 3.5, col);
        draw_poly(sp.x, sp.y, 4, r * 0.72, -t * 36.0, col);
        draw_poly(sp.x, sp.y, 4, r * 0.42, t * 48.0, Color::new(1.0, 0.85, 0.1, 1.0));
    }
}

// ==========================================
// 9. SHADOW ASSASSIN (Sát Thủ Hư Không - Bọ Ngựa Tàng Hình & Dịch Chuyển Trảm)
// ==========================================
pub struct ShadowAssassin {
    pub facing: f32,
    pub cloak_timer: f32,
    pub telegraph_timer: f32,
    pub is_teleporting: bool,
    pub slash_timer: f32,
}

impl EnemyBehavior for ShadowAssassin {
    fn update(&mut self, body: &mut EnemyBody, ctx: &mut CombatContext, dt: f32) {
        let to_player = ctx.player_pos - body.pos;
        let dist = to_player.length();
        let dir = to_player.normalize_or_zero();
        self.facing = to_player.y.atan2(to_player.x);

        if self.is_teleporting {
            self.telegraph_timer -= dt;
            if self.telegraph_timer <= 0.0 {
                self.is_teleporting = false;
                self.slash_timer = 0.25;
            }
        } else if self.slash_timer > 0.0 {
            self.slash_timer -= dt;
            let slash_dir = vec2(self.facing.cos(), self.facing.sin());
            body.pos += slash_dir * 580.0 * dt;
            if dist < ctx.player_radius + body.radius + 15.0 {
                ctx.damage_player(42.0);
                self.slash_timer = 0.0;
                ctx.spawn_particle_burst(body.pos, Color::new(0.9, 0.1, 0.3, 1.0), 12, 180.0, 0.3);
            }
        } else {
            body.pos += dir * body.speed * dt;
            self.cloak_timer += dt;

            // Khi ở gần (dưới 340px) và hồi chiêu xong (3.2s): Dịch chuyển ra ngay sau lưng người chơi!
            if self.cloak_timer >= 3.2 && dist < 340.0 {
                self.cloak_timer = 0.0;
                self.is_teleporting = true;
                self.telegraph_timer = 0.42;

                ctx.spawn_particle_burst(body.pos, Color::new(0.6, 0.1, 0.9, 0.8), 10, 160.0, 0.25);
                // Vị trí dịch chuyển: phía sau lưng người chơi
                let behind_dir = if dir.length_squared() > 0.01 { -dir } else { vec2(0.0, 1.0) };
                body.pos = ctx.player_pos + behind_dir * 85.0;
                self.facing = (-behind_dir).y.atan2((-behind_dir).x);
                ctx.spawn_particle_burst(body.pos, Color::new(0.9, 0.1, 0.6, 1.0), 14, 220.0, 0.3);
                ctx.spawn_float_text(body.pos - vec2(0.0, 24.0), "BLINK!", Color::new(0.8, 0.2, 1.0, 1.0), 18.0);
            }

            if dist < ctx.player_radius + body.radius {
                ctx.damage_player(28.0);
                body.vel -= dir * 180.0;
            }
        }
    }

    fn draw_custom(&self, body: &EnemyBody, sp: Vec2, _cam_offset: Vec2) {
        let is_cloaked = !self.is_teleporting && self.slash_timer <= 0.0;
        let alpha = if is_cloaked { 0.28 } else { 1.0 };
        let col = if body.hit_flash > 0.0 {
            WHITE
        } else {
            Color::new(0.7, 0.15, 0.95, alpha)
        };
        let fwd = vec2(self.facing.cos(), self.facing.sin());
        let right = vec2(-fwd.y, fwd.x);
        let r = body.radius;

        // Vạch cảnh báo chém chéo đỏ rực khi đang chuẩn bị ra đòn (Telegraph)
        if self.is_teleporting {
            let t1 = sp + fwd * 70.0 + right * 35.0;
            let t2 = sp + fwd * 70.0 - right * 35.0;
            draw_line(sp.x, sp.y, t1.x, t1.y, 2.5, Color::new(1.0, 0.1, 0.2, 0.85));
            draw_line(sp.x, sp.y, t2.x, t2.y, 2.5, Color::new(1.0, 0.1, 0.2, 0.85));
        }

        // Cặp lưỡi hái song sát (Dual Mantis Scythes)
        let scythe_len = if self.slash_timer > 0.0 { r * 2.2 } else { r * 1.5 };
        for side in [-1.0f32, 1.0] {
            let arm_base = sp + right * (side * r * 0.7) + fwd * 4.0;
            let elbow = arm_base + fwd * (scythe_len * 0.6) + right * (side * 12.0);
            let blade_tip = sp + fwd * scythe_len + right * (side * 4.0);
            draw_line(arm_base.x, arm_base.y, elbow.x, elbow.y, 3.0, col);
            draw_line(
                elbow.x,
                elbow.y,
                blade_tip.x,
                blade_tip.y,
                3.5,
                if self.slash_timer > 0.0 {
                    Color::new(1.0, 0.2, 0.4, 1.0)
                } else {
                    Color::new(0.9, 0.4, 1.0, alpha)
                },
            );
        }

        // Thân bọ ngựa hư không thon nhọn góc cạnh
        let head = sp + fwd * (r * 1.3);
        let left_hip = sp - fwd * (r * 0.8) + right * (r * 0.6);
        let right_hip = sp - fwd * (r * 0.8) - right * (r * 0.6);
        let tail = sp - fwd * (r * 1.4);
        draw_triangle(head, left_hip, tail, col);
        draw_triangle(head, right_hip, tail, col);
        draw_line(head.x, head.y, tail.x, tail.y, 2.0, Color::new(0.3, 0.9, 1.0, alpha));
    }
}

// ==========================================
// 10. PHALANX GOLEM (Người Đá Khiên Năng Lượng - Chặn 80% Đòn Chính Diện & Đẩy Khiên)
// ==========================================
pub struct PhalanxGolem {
    pub facing: f32,
    pub bash_timer: f32,
    pub is_bashing: bool,
    pub bash_duration: f32,
}

impl EnemyBehavior for PhalanxGolem {
    fn update(&mut self, body: &mut EnemyBody, ctx: &mut CombatContext, dt: f32) {
        let to_player = ctx.player_pos - body.pos;
        let dist = to_player.length();
        let dir = to_player.normalize_or_zero();

        if self.is_bashing {
            self.bash_duration -= dt;
            let bdir = vec2(self.facing.cos(), self.facing.sin());
            body.pos += bdir * 380.0 * dt;
            if dist < ctx.player_radius + body.radius + 10.0 {
                ctx.damage_player(34.0);
                self.is_bashing = false;
                ctx.spawn_event(Box::new(ShockwaveRingEvent {
                    center: body.pos,
                    current_radius: 15.0,
                    max_radius: 110.0,
                    speed: 360.0,
                    color: Color::new(0.2, 0.8, 1.0, 0.85),
                    thickness: 4.5,
                }));
            }
            if self.bash_duration <= 0.0 {
                self.is_bashing = false;
            }
        } else {
            self.facing = to_player.y.atan2(to_player.x);
            body.pos += dir * body.speed * dt;
            self.bash_timer += dt;

            if self.bash_timer >= 2.8 && dist < 150.0 {
                self.bash_timer = 0.0;
                self.is_bashing = true;
                self.bash_duration = 0.32;
                ctx.spawn_float_text(body.pos - vec2(0.0, 26.0), "SHIELD BASH!", Color::new(0.2, 0.8, 1.0, 1.0), 16.0);
            }

            if dist < ctx.player_radius + body.radius {
                ctx.damage_player(26.0);
                body.vel -= dir * 140.0;
            }
        }
    }

    fn modify_incoming_damage(&self, body: &EnemyBody, hit_pos: Vec2, raw_dmg: f32) -> f32 {
        let to_hit = hit_pos - body.pos;
        let hit_angle = to_hit.y.atan2(to_hit.x);
        let mut diff = (hit_angle - self.facing).abs();
        while diff > PI {
            diff = (diff - 2.0 * PI).abs();
        }
        // Góc đón đỡ chính diện 70 độ (+- 1.2 rad): Khiên giảm 80% sát thương!
        if diff < 1.20 {
            raw_dmg * 0.20
        } else {
            raw_dmg
        }
    }

    fn draw_custom(&self, body: &EnemyBody, sp: Vec2, _cam_offset: Vec2) {
        let col = if body.hit_flash > 0.0 { WHITE } else { body.color };
        let fwd = vec2(self.facing.cos(), self.facing.sin());
        let right = vec2(-fwd.y, fwd.x);
        let r = body.radius;

        // Thân người đá lục giác giáp titan
        draw_poly(sp.x, sp.y, 6, r, self.facing * 57.3, col);
        draw_poly_lines(sp.x, sp.y, 6, r, self.facing * 57.3, 2.5, Color::new(0.4, 0.85, 1.0, 1.0));

        // Tấm Khiên Năng Lượng Hình Vòng Cung Cỡ Lớn Phía Trước (Phalanx Energy Shield)
        let shield_center = sp + fwd * (r + 10.0);
        let p_left = shield_center + right * (r * 1.5) - fwd * 4.0;
        let p_right = shield_center - right * (r * 1.5) - fwd * 4.0;
        let p_tip = shield_center + fwd * 8.0;

        // Vẽ màng chắn năng lượng phát quang
        draw_triangle(p_tip, p_left, p_right, Color::new(0.1, 0.65, 1.0, 0.35));
        draw_line(p_left.x, p_left.y, p_tip.x, p_tip.y, 4.5, Color::new(0.3, 0.9, 1.0, 0.95));
        draw_line(p_tip.x, p_tip.y, p_right.x, p_right.y, 4.5, Color::new(0.3, 0.9, 1.0, 0.95));

        // Lõi năng lượng trung tâm ngực phát sáng
        draw_circle(sp.x, sp.y, 5.0, Color::new(0.0, 1.0, 0.9, 1.0));
    }
}

// ==========================================
// 11. VOID SUMMONER (Phù Thủy Triệu Hồi Hư Không - Đẻ Bọ Ký Sinh & Bắn Cầu Hắc Ám)
// ==========================================
pub struct VoidSummoner {
    pub summon_timer: f32,
    pub orb_timer: f32,
    pub rot: f32,
    pub wave_level: u32,
}

impl EnemyBehavior for VoidSummoner {
    fn update(&mut self, body: &mut EnemyBody, ctx: &mut CombatContext, dt: f32) {
        self.rot += dt * 2.5;
        let to_player = ctx.player_pos - body.pos;
        let dist = to_player.length();
        let dir = to_player.normalize_or_zero();

        // Giữ cự ly tầm xa 380 - 540px
        if dist < 380.0 {
            body.pos -= dir * body.speed * dt;
        } else if dist > 540.0 {
            body.pos += dir * body.speed * dt;
        }

        // 1. Triệu hồi đàn bọ Ký Sinh Trùng Hư Không (Void Parasite) mỗi 3.6s
        self.summon_timer += dt;
        if self.summon_timer >= 3.6 {
            self.summon_timer = 0.0;
            ctx.spawn_float_text(body.pos - vec2(0.0, 25.0), "SUMMON!", Color::new(0.9, 0.3, 1.0, 1.0), 16.0);
            for offset_angle in [-1.0f32, 1.0] {
                let off = vec2(offset_angle.cos(), offset_angle.sin()) * 28.0;
                let parasite_hp = 38.0 + (self.wave_level as f32) * 8.0;
                ctx.spawn_enemy(EnemyEntity::new(
                    EnemyBody {
                        name: "VOID PARASITE",
                        pos: body.pos + off,
                        vel: off.normalize_or_zero() * 120.0,
                        radius: 11.0,
                        speed: 285.0 + (self.wave_level as f32) * 2.0,
                        color: Color::new(0.85, 0.15, 0.9, 1.0),
                        hp: parasite_hp,
                        max_hp: parasite_hp,
                        is_boss: false,
                        score_reward: 75,
                        hit_flash: 0.0,
                        statuses: Vec::new(),
                        parts: Vec::new(),
                    },
                    Box::new(VoidParasite {
                        facing: 0.0,
                        wiggle: rand::gen_range(0.0, 6.28),
                    }),
                ));
            }
        }

        // 2. Bắn 3 cầu hắc ám ma thuật hư không mỗi 2.0s
        self.orb_timer += dt;
        if self.orb_timer >= 2.0 {
            self.orb_timer = 0.0;
            let base_angle = to_player.y.atan2(to_player.x);
            for spread in [-0.22f32, 0.0, 0.22] {
                let a = base_angle + spread;
                ctx.spawn_enemy_projectile(Projectile {
                    pos: body.pos,
                    vel: vec2(a.cos(), a.sin()) * 340.0,
                    damage: 26.0,
                    color: Color::new(0.7, 0.1, 0.95, 1.0),
                    radius: 7.0,
                    explosion_radius: None,
                    spawns_black_hole: false,
                    pierce: 0,
                    bounces: 0,
                    life: 3.2,
                    status_effect: Some(StatusEffect::Slow {
                        factor: 0.55,
                        duration: 1.6,
                    }),
                });
            }
        }

        if dist < ctx.player_radius + body.radius {
            ctx.damage_player(20.0);
        }
    }

    fn draw_custom(&self, body: &EnemyBody, sp: Vec2, _cam_offset: Vec2) {
        let col = if body.hit_flash > 0.0 { WHITE } else { body.color };
        let r = body.radius;

        // 3 Tinh thể ma thuật Hư Không xoay tròn xung quanh
        for i in 0..3 {
            let a = self.rot + (i as f32) * (PI * 2.0 / 3.0);
            let orb_pos = sp + vec2(a.cos(), a.sin()) * (r * 1.35);
            draw_poly(orb_pos.x, orb_pos.y, 4, 6.5, a * 57.3, Color::new(0.9, 0.2, 1.0, 0.9));
            draw_line(sp.x, sp.y, orb_pos.x, orb_pos.y, 1.5, Color::new(0.6, 0.1, 0.8, 0.5));
        }

        // Tháp tinh thể kim cương hắc diện thạch
        draw_poly(sp.x, sp.y, 5, r, self.rot * 30.0, col);
        draw_poly_lines(sp.x, sp.y, 5, r, self.rot * 30.0, 2.5, WHITE);
        draw_poly(sp.x, sp.y, 5, r * 0.5, -self.rot * 45.0, Color::new(0.2, 0.05, 0.35, 1.0));
    }
}

// ==========================================
// 12. VOID PARASITE (Ký Sinh Trùng Hư Không - Bọ Bò Nhanh Cắn Rỉa)
// ==========================================
pub struct VoidParasite {
    pub facing: f32,
    pub wiggle: f32,
}

impl EnemyBehavior for VoidParasite {
    fn update(&mut self, body: &mut EnemyBody, ctx: &mut CombatContext, dt: f32) {
        let to_player = ctx.player_pos - body.pos;
        let dist = to_player.length();
        let dir = to_player.normalize_or_zero();
        self.facing = to_player.y.atan2(to_player.x);
        self.wiggle += dt * 22.0;

        body.pos += dir * body.speed * dt;

        if dist < ctx.player_radius + body.radius {
            ctx.damage_player(18.0);
            body.vel -= dir * 210.0;
        }
    }

    fn draw_custom(&self, body: &EnemyBody, sp: Vec2, _cam_offset: Vec2) {
        let col = if body.hit_flash > 0.0 { WHITE } else { body.color };
        let fwd = vec2(self.facing.cos(), self.facing.sin());
        let right = vec2(-fwd.y, fwd.x);
        let r = body.radius;

        // 4 chân bọ bò ngoe nguẩy tốc độ cao
        for i in [-0.6f32, 0.6] {
            let leg_swing = (self.wiggle + i * 2.0).sin() * 5.0;
            for side in [-1.0f32, 1.0] {
                let base = sp + fwd * (i * 6.0);
                let tip = base + right * (side * (r + 7.0)) + fwd * leg_swing;
                draw_line(base.x, base.y, tip.x, tip.y, 2.0, Color::new(0.9, 0.2, 0.8, 0.95));
            }
        }

        // Thân bọ hình thoi dẹp
        let front = sp + fwd * (r * 1.2);
        let back = sp - fwd * (r * 1.1);
        let left = sp + right * (r * 0.75);
        let right_pt = sp - right * (r * 0.75);
        draw_triangle(front, left, right_pt, col);
        draw_triangle(back, left, right_pt, col);
        // Mắt đơn phát sáng màu tím
        draw_circle(sp.x + fwd.x * 3.0, sp.y + fwd.y * 3.0, 3.0, Color::new(1.0, 0.8, 1.0, 1.0));
    }
}

// ==========================================
// 13. MORTAR JUGGERNAUT (Pháo Tự Hành Hạng Nặng - Bắn Đạn Pháo Kích Vòng Đỏ Telegraph)
// ==========================================
pub struct MortarJuggernaut {
    pub facing: f32,
    pub fire_timer: f32,
    pub recoil: f32,
}

impl EnemyBehavior for MortarJuggernaut {
    fn update(&mut self, body: &mut EnemyBody, ctx: &mut CombatContext, dt: f32) {
        if self.recoil > 0.0 {
            self.recoil = (self.recoil - dt * 4.0).max(0.0);
        }
        let to_player = ctx.player_pos - body.pos;
        let dist = to_player.length();
        let dir = to_player.normalize_or_zero();
        self.facing = to_player.y.atan2(to_player.x);

        // Duy trì khoảng cách tầm pháo kích 420 - 620px
        if dist < 420.0 {
            body.pos -= dir * body.speed * dt;
        } else if dist > 620.0 {
            body.pos += dir * body.speed * dt;
        }

        self.fire_timer += dt;
        if self.fire_timer >= 2.8 {
            self.fire_timer = 0.0;
            self.recoil = 1.0;
            ctx.spawn_float_text(body.pos - vec2(0.0, 26.0), "BOMBARDMENT!", Color::new(1.0, 0.4, 0.1, 1.0), 16.0);

            // Bắn 2 quả đạn cối cảnh báo vùng đỏ xuống khu vực người chơi
            for offset in [Vec2::ZERO, vec2(rand::gen_range(-110.0, 110.0), rand::gen_range(-110.0, 110.0))] {
                ctx.spawn_event(Box::new(MeteorWarningStrikeEvent {
                    target_pos: ctx.player_pos + offset,
                    radius: 95.0,
                    damage: 46.0,
                    timer: 0.0,
                    max_time: 1.25,
                    harms_player: true,
                }));
            }
        }

        if dist < ctx.player_radius + body.radius {
            ctx.damage_player(30.0);
            body.vel -= dir * 120.0;
        }
    }

    fn draw_custom(&self, body: &EnemyBody, sp: Vec2, _cam_offset: Vec2) {
        let col = if body.hit_flash > 0.0 { WHITE } else { body.color };
        let fwd = vec2(self.facing.cos(), self.facing.sin());
        let right = vec2(-fwd.y, fwd.x);
        let r = body.radius;

        // 2 khối bánh xích xe bọc thép hai bên
        for side in [-1.0f32, 1.0] {
            let p1 = sp - fwd * (r * 0.95) + right * (side * r * 0.9);
            let p2 = sp + fwd * (r * 0.95) + right * (side * r * 0.9);
            draw_line(p1.x, p1.y, p2.x, p2.y, 9.0, Color::new(0.2, 0.22, 0.26, 1.0));
        }

        // Thân xe thiết giáp chữ nhật kiên cố
        let fl = sp + fwd * (r * 0.8) + right * (r * 0.75);
        let fr = sp + fwd * (r * 0.8) - right * (r * 0.75);
        let bl = sp - fwd * (r * 0.85) + right * (r * 0.75);
        let br = sp - fwd * (r * 0.85) - right * (r * 0.75);
        draw_triangle(fl, fr, bl, col);
        draw_triangle(fr, br, bl, col);
        draw_line(fl.x, fl.y, fr.x, fr.y, 2.5, WHITE);
        draw_line(bl.x, bl.y, br.x, br.y, 2.5, WHITE);

        // Nòng pháo cối cỡ đại vươn cao (có hiệu ứng giật lùi recoil)
        let barrel_len = (r + 14.0) - self.recoil * 7.0;
        let muzzle = sp + fwd * barrel_len;
        draw_line(sp.x, sp.y, muzzle.x, muzzle.y, 8.0, Color::new(0.85, 0.45, 0.1, 1.0));
        draw_circle(muzzle.x, muzzle.y, 4.5, Color::new(1.0, 0.7, 0.2, 1.0));
    }
}

// ==========================================
// 14. TESLA OVERLORD (Lãnh Chúa Sấm Sét Cyber - Phóng Tia Điện Dây Chuyền & Sóng Xung Kích EMP)
// ==========================================
pub struct TeslaOverlord {
    pub facing: f32,
    pub zap_timer: f32,
    pub emp_timer: f32,
    pub coil_angle: f32,
}

impl EnemyBehavior for TeslaOverlord {
    fn update(&mut self, body: &mut EnemyBody, ctx: &mut CombatContext, dt: f32) {
        self.coil_angle += dt * 4.0;
        let to_player = ctx.player_pos - body.pos;
        let dist = to_player.length();
        let dir = to_player.normalize_or_zero();
        self.facing = to_player.y.atan2(to_player.x);

        // Di chuyển lơ lửng tầm trung
        if dist < 290.0 {
            body.pos -= dir * body.speed * dt;
        } else if dist > 440.0 {
            body.pos += dir * body.speed * dt;
        }

        // 1. Phóng chùm sét 3 tia điện cao thế mỗi 2.1s
        self.zap_timer += dt;
        if self.zap_timer >= 2.1 {
            self.zap_timer = 0.0;
            let base_angle = self.facing;
            for offset in [-0.25f32, 0.0, 0.25] {
                let a = base_angle + offset;
                ctx.spawn_enemy_projectile(Projectile {
                    pos: body.pos,
                    vel: vec2(a.cos(), a.sin()) * 520.0,
                    damage: 30.0,
                    color: Color::new(0.2, 0.9, 1.0, 1.0),
                    radius: 5.5,
                    explosion_radius: None,
                    spawns_black_hole: false,
                    pierce: 0,
                    bounces: 0,
                    life: 2.8,
                    status_effect: Some(StatusEffect::Slow {
                        factor: 0.45,
                        duration: 1.5,
                    }),
                });
            }
        }

        // 2. Phát xung sóng chấn điện từ EMP Shockwave mỗi 4.2s
        self.emp_timer += dt;
        if self.emp_timer >= 4.2 {
            self.emp_timer = 0.0;
            ctx.spawn_float_text(body.pos - vec2(0.0, 28.0), "EMP DISCHARGE!", Color::new(0.3, 0.95, 1.0, 1.0), 17.0);
            ctx.spawn_event(Box::new(ShockwaveRingEvent {
                center: body.pos,
                current_radius: 20.0,
                max_radius: 210.0,
                speed: 440.0,
                color: Color::new(0.25, 0.9, 1.0, 0.9),
                thickness: 5.0,
            }));
            if dist < 210.0 {
                ctx.damage_player(26.0);
            }
        }

        if dist < ctx.player_radius + body.radius {
            ctx.damage_player(28.0);
        }
    }

    fn draw_custom(&self, body: &EnemyBody, sp: Vec2, _cam_offset: Vec2) {
        let col = if body.hit_flash > 0.0 { WHITE } else { body.color };
        let r = body.radius;

        // 3 cuộn cảm Tesla quay xung quanh phóng hồ quang điện
        let mut coil_positions = [Vec2::ZERO; 3];
        for i in 0..3 {
            let a = self.coil_angle + (i as f32) * (PI * 2.0 / 3.0);
            let cp = sp + vec2(a.cos(), a.sin()) * (r * 1.35);
            coil_positions[i] = cp;
            draw_circle(cp.x, cp.y, 5.0, Color::new(0.2, 0.9, 1.0, 1.0));
            draw_circle_lines(cp.x, cp.y, 5.0, 2.0, WHITE);
        }
        // Phóng tia điện liên kết giữa 3 cuộn cảm
        for i in 0..3 {
            let p1 = coil_positions[i];
            let p2 = coil_positions[(i + 1) % 3];
            draw_line(p1.x, p1.y, p2.x, p2.y, 2.0, Color::new(0.4, 0.95, 1.0, 0.75));
        }

        // Lõi năng lượng Plasma điện từ hình bát giác
        draw_poly(sp.x, sp.y, 8, r, -self.coil_angle * 20.0, col);
        draw_poly_lines(sp.x, sp.y, 8, r, -self.coil_angle * 20.0, 2.5, WHITE);
        draw_circle(sp.x, sp.y, r * 0.45, Color::new(0.9, 1.0, 0.3, 1.0));
    }
}

// ==========================================
// 15. MUTANT BIO-GIANT (Khổng Lồ Gai Độc Đột Biến - Bão Gai Xoáy 16 Hướng & Bãi Độc Tử Thần)
// ==========================================
pub struct MutantBioGiant {
    pub facing: f32,
    pub spine_timer: f32,
    pub leg_phase: f32,
}

impl EnemyBehavior for MutantBioGiant {
    fn update(&mut self, body: &mut EnemyBody, ctx: &mut CombatContext, dt: f32) {
        let to_player = ctx.player_pos - body.pos;
        let dist = to_player.length();
        let dir = to_player.normalize_or_zero();
        self.facing = to_player.y.atan2(to_player.x);
        self.leg_phase += dt * 10.0;

        body.pos += dir * body.speed * dt;

        // Bão phóng 16 gai độc xoáy tròn mỗi 3.0s
        self.spine_timer += dt;
        if self.spine_timer >= 3.0 {
            self.spine_timer = 0.0;
            ctx.spawn_float_text(body.pos - vec2(0.0, 32.0), "TOXIC BURST!", Color::new(0.3, 1.0, 0.2, 1.0), 18.0);
            for i in 0..16 {
                let a = (i as f32) * (PI * 2.0 / 16.0);
                ctx.spawn_enemy_projectile(Projectile {
                    pos: body.pos,
                    vel: vec2(a.cos(), a.sin()) * 320.0,
                    damage: 24.0,
                    color: Color::new(0.4, 1.0, 0.15, 1.0),
                    radius: 6.0,
                    explosion_radius: None,
                    spawns_black_hole: false,
                    pierce: 0,
                    bounces: 0,
                    life: 3.2,
                    status_effect: Some(StatusEffect::Slow {
                        factor: 0.5,
                        duration: 2.0,
                    }),
                });
            }
        }

        if dist < ctx.player_radius + body.radius {
            ctx.damage_player(45.0);
            body.vel -= dir * 150.0;
        }
    }

    fn on_damaged(&mut self, body: &mut EnemyBody, dmg: f32, ctx: &mut CombatContext) {
        // Phản đòn gai nhọn khi bị dính sát thương lớn (trên 35 sát thương)
        if dmg > 35.0 && rand::gen_range(0.0, 1.0) < 0.28 {
            for i in 0..8 {
                let a = (i as f32) * (PI * 2.0 / 8.0);
                ctx.spawn_enemy_projectile(Projectile {
                    pos: body.pos,
                    vel: vec2(a.cos(), a.sin()) * 360.0,
                    damage: 18.0,
                    color: Color::new(0.5, 1.0, 0.1, 1.0),
                    radius: 5.0,
                    explosion_radius: None,
                    spawns_black_hole: false,
                    pierce: 0,
                    bounces: 0,
                    life: 2.2,
                    status_effect: None,
                });
            }
        }
    }

    fn on_death(&mut self, body: &EnemyBody, ctx: &mut CombatContext) {
        ctx.trigger_explosion(body.pos, 110.0, 50.0);
        // Để lại vũng axit độc ăn mòn kéo dài 8 giây trên chiến trường!
        ctx.spawn_decal(crate::core::types::Decal {
            pos: body.pos,
            radius: 80.0,
            color: Color::new(0.2, 0.95, 0.15, 0.45),
            life: 8.0,
            is_damage_zone: true,
        });
    }

    fn draw_custom(&self, body: &EnemyBody, sp: Vec2, _cam_offset: Vec2) {
        let col = if body.hit_flash > 0.0 { WHITE } else { body.color };
        let fwd = vec2(self.facing.cos(), self.facing.sin());
        let right = vec2(-fwd.y, fwd.x);
        let r = body.radius;

        // Các gai độc khổng lồ mọc sau lưng (Dorsal Spine Rack)
        for i in [-1.0f32, -0.3, 0.3, 1.0] {
            let spine_base = sp - fwd * (r * 0.4) + right * (i * r * 0.7);
            let spine_tip = spine_base - fwd * 18.0 + right * (i * 12.0);
            draw_line(spine_base.x, spine_base.y, spine_tip.x, spine_tip.y, 4.0, Color::new(0.5, 1.0, 0.2, 0.95));
        }

        // Thân sinh học đột biến hình đa giác gồ ghề
        draw_poly(sp.x, sp.y, 7, r, self.facing * 57.3, col);
        draw_poly_lines(sp.x, sp.y, 7, r, self.facing * 57.3, 3.5, Color::new(0.3, 1.0, 0.3, 1.0));

        // 3 bọc độc phát sáng xanh nõn chuối trên lưng
        for angle_offset in [-1.2f32, 0.0, 1.2] {
            let blister_pos = sp + vec2(angle_offset.cos(), angle_offset.sin()) * (r * 0.45);
            let pulse = (self.leg_phase + angle_offset).sin().abs() * 2.0;
            draw_circle(blister_pos.x, blister_pos.y, 6.0 + pulse, Color::new(0.3, 1.0, 0.1, 0.9));
        }
    }
}
