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
            ctx.damage_player(14.0);
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
        if self.shoot_timer >= 2.0 {
            self.shoot_timer = 0.0;
            ctx.spawn_enemy_projectile(Projectile {
                pos: body.pos + dir * (body.radius + 6.0),
                vel: dir * 380.0,
                damage: 18.0,
                color: Color::new(0.15, 1.0, 0.35, 1.0),
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
        ctx.trigger_explosion(body.pos, 85.0, 35.0);
        if (ctx.player_pos - body.pos).length() < 85.0 + ctx.player_radius {
            ctx.damage_player(30.0);
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
        if self.aim_timer >= 2.4 {
            self.aim_timer = 0.0;
            ctx.spawn_enemy_projectile(Projectile {
                pos: body.pos + dir * (body.radius + 12.0),
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
