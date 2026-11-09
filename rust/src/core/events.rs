use macroquad::prelude::*;
use std::f32::consts::PI;
use crate::core::context::CombatContext;
use crate::core::props::{WeaponDrop, WeaponRarity};

/// Lớp trừu tượng cho mọi Sự kiện Chiến trường (Battlefield Events / Hazards / Tactical Strikes)
/// Mọi sự kiện có tính chất diễn biến theo thời gian và tác động lên không gian bản đồ đều cài đặt Trait này.
pub trait BattlefieldEvent: Send + Sync {
    /// Cập nhật trạng thái sự kiện mỗi frame. Trả về `true` nếu sự kiện còn tiếp diễn, `false` khi kết thúc.
    fn update(&mut self, ctx: &mut CombatContext, dt: f32) -> bool;

    /// Vẽ hiệu ứng cảnh báo (telegraph) hoặc hiệu ứng kỹ năng lên màn hình.
    fn draw(&self, cam_offset: Vec2);
}

// =========================================================
// 1. SÓNG XUNG KÍCH LAN TỎA (Shockwave Ring Event)
// =========================================================
pub struct ShockwaveRingEvent {
    pub center: Vec2,
    pub current_radius: f32,
    pub max_radius: f32,
    pub speed: f32,
    pub color: Color,
    pub thickness: f32,
}

impl BattlefieldEvent for ShockwaveRingEvent {
    fn update(&mut self, _ctx: &mut CombatContext, dt: f32) -> bool {
        self.current_radius += self.speed * dt;
        self.current_radius < self.max_radius
    }

    fn draw(&self, cam_offset: Vec2) {
        let sp = self.center + cam_offset;
        let progress = (self.current_radius / self.max_radius).clamp(0.0, 1.0);
        let alpha = (1.0 - progress) * self.color.a;
        let col = Color::new(self.color.r, self.color.g, self.color.b, alpha);
        draw_circle_lines(
            sp.x,
            sp.y,
            self.current_radius,
            self.thickness * (1.0 - progress * 0.5),
            col,
        );
    }
}

// =========================================================
// 2. HỐ ĐEN TRỌNG TRƯỜNG (Singularity Black Hole Event)
// =========================================================
pub struct SingularityBlackHoleEvent {
    pub pos: Vec2,
    pub pull_radius: f32,
    pub pull_force: f32,
    pub dps: f32,
    pub duration: f32,
    pub max_duration: f32,
    pub rotation: f32,
}

impl BattlefieldEvent for SingularityBlackHoleEvent {
    fn update(&mut self, ctx: &mut CombatContext, dt: f32) -> bool {
        self.duration -= dt;
        self.rotation += dt * 8.0;

        ctx.apply_vortex_pull(
            self.pos,
            self.pull_radius,
            self.pull_force * dt,
            self.dps * dt,
        );

        if self.duration <= 0.0 {
            ctx.trigger_explosion(self.pos, self.pull_radius * 0.9, 280.0);
            ctx.spawn_event(Box::new(ShockwaveRingEvent {
                center: self.pos,
                current_radius: 10.0,
                max_radius: self.pull_radius * 1.2,
                speed: 650.0,
                color: Color::new(0.7, 0.2, 1.0, 0.95),
                thickness: 8.0,
            }));
            return false;
        }
        true
    }

    fn draw(&self, cam_offset: Vec2) {
        let sp = self.pos + cam_offset;
        let life_ratio = (self.duration / self.max_duration.max(0.1)).clamp(0.0, 1.0);

        draw_circle(sp.x, sp.y, self.pull_radius, Color::new(0.25, 0.0, 0.45, 0.14));
        draw_circle_lines(
            sp.x,
            sp.y,
            self.pull_radius * life_ratio,
            2.0,
            Color::new(0.65, 0.2, 1.0, 0.55),
        );

        for i in 0..4 {
            let angle = self.rotation + (i as f32) * (PI / 2.0);
            let r = 28.0 + ((self.duration * 60.0 + (i as f32) * 20.0) % (self.pull_radius * 0.7));
            let p1 = sp + vec2(angle.cos(), angle.sin()) * r;
            let p2 = sp + vec2((angle + 0.6).cos(), (angle + 0.6).sin()) * (r * 0.6);
            draw_line(
                p1.x,
                p1.y,
                p2.x,
                p2.y,
                2.5,
                Color::new(0.8, 0.4, 1.0, 0.75),
            );
        }

        draw_circle(sp.x, sp.y, 26.0, Color::new(0.75, 0.2, 1.0, 0.9));
        draw_circle(sp.x, sp.y, 20.0, BLACK);
    }
}

// =========================================================
// 3. KHÔNG KÍCH RẢI THẢM (Carpet Bombing Run Event)
// =========================================================
pub struct CarpetBombingRunEvent {
    pub start_pos: Vec2,
    pub dir: Vec2,
    pub total_bombs: u32,
    pub bombs_dropped: u32,
    pub step_distance: f32,
    pub timer: f32,
    pub warmup_timer: f32,
}

impl BattlefieldEvent for CarpetBombingRunEvent {
    fn update(&mut self, ctx: &mut CombatContext, dt: f32) -> bool {
        if self.warmup_timer > 0.0 {
            self.warmup_timer -= dt;
            return true;
        }

        self.timer += dt;
        if self.timer >= 0.07 && self.bombs_dropped < self.total_bombs {
            self.timer = 0.0;
            let drop_pos = self.start_pos
                + self.dir * ((self.bombs_dropped as f32) * self.step_distance)
                + vec2(rand::gen_range(-25.0, 25.0), rand::gen_range(-25.0, 25.0));

            ctx.trigger_explosion(drop_pos, 145.0, 220.0);
            ctx.spawn_event(Box::new(ShockwaveRingEvent {
                center: drop_pos,
                current_radius: 12.0,
                max_radius: 155.0,
                speed: 520.0,
                color: Color::new(1.0, 0.55, 0.1, 0.9),
                thickness: 5.0,
            }));

            self.bombs_dropped += 1;
        }

        self.bombs_dropped < self.total_bombs
    }

    fn draw(&self, cam_offset: Vec2) {
        let length = (self.total_bombs as f32) * self.step_distance;
        let p1 = self.start_pos + cam_offset;
        let p2 = self.start_pos + self.dir * length + cam_offset;
        draw_line(
            p1.x,
            p1.y,
            p2.x,
            p2.y,
            56.0,
            Color::new(1.0, 0.15, 0.1, 0.18),
        );
        draw_line(
            p1.x,
            p1.y,
            p2.x,
            p2.y,
            2.0,
            Color::new(1.0, 0.3, 0.1, 0.75),
        );
    }
}

// =========================================================
// 4. TIA LASER VỆ TINH QUỸ ĐẠO (Orbital Laser Strike Event)
// =========================================================
pub struct OrbitalLaserStrikeEvent {
    pub pos: Vec2,
    pub speed: f32,
    pub radius: f32,
    pub dps: f32,
    pub duration: f32,
    pub decal_timer: f32,
}

impl BattlefieldEvent for OrbitalLaserStrikeEvent {
    fn update(&mut self, ctx: &mut CombatContext, dt: f32) -> bool {
        self.duration -= dt;
        self.decal_timer += dt;

        if let Some(target) = ctx.find_nearest_enemy(self.pos, 900.0) {
            let dir = (target - self.pos).normalize_or_zero();
            self.pos += dir * self.speed * dt;
        }

        ctx.apply_vortex_pull(self.pos, self.radius, -120.0 * dt, self.dps * dt);
        ctx.add_screen_shake(3.0 * dt);

        if self.decal_timer >= 0.08 {
            self.decal_timer = 0.0;
            ctx.spawn_particle_burst(
                self.pos,
                Color::new(0.0, 0.95, 1.0, 1.0),
                6,
                220.0,
                0.25,
            );
        }

        self.duration > 0.0
    }

    fn draw(&self, cam_offset: Vec2) {
        let sp = self.pos + cam_offset;
        draw_line(
            sp.x,
            0.0,
            sp.x,
            sp.y,
            self.radius * 1.4,
            Color::new(0.0, 0.85, 1.0, 0.28),
        );
        draw_line(
            sp.x,
            0.0,
            sp.x,
            sp.y,
            self.radius * 0.55,
            Color::new(0.8, 0.98, 1.0, 0.85),
        );

        draw_circle(sp.x, sp.y, self.radius, Color::new(0.0, 0.9, 1.0, 0.45));
        draw_circle(sp.x, sp.y, self.radius * 0.5, WHITE);
    }
}

// =========================================================
// 5. PHÁO KÍCH / THIÊN THẠCH CẢNH BÁO (Meteor Telegraph Event)
// =========================================================
pub struct MeteorWarningStrikeEvent {
    pub target_pos: Vec2,
    pub radius: f32,
    pub damage: f32,
    pub timer: f32,
    pub max_time: f32,
    pub harms_player: bool,
}

impl BattlefieldEvent for MeteorWarningStrikeEvent {
    fn update(&mut self, ctx: &mut CombatContext, dt: f32) -> bool {
        self.timer += dt;
        if self.timer >= self.max_time {
            if self.harms_player {
                let dist = (ctx.player_pos - self.target_pos).length();
                if dist < self.radius + ctx.player_radius {
                    ctx.damage_player(self.damage);
                }
                ctx.add_screen_shake(8.0);
                ctx.spawn_particle_burst(
                    self.target_pos,
                    Color::new(1.0, 0.2, 0.1, 1.0),
                    24,
                    280.0,
                    0.4,
                );
            } else {
                ctx.trigger_explosion(self.target_pos, self.radius, self.damage);
            }
            ctx.spawn_event(Box::new(ShockwaveRingEvent {
                center: self.target_pos,
                current_radius: 8.0,
                max_radius: self.radius * 1.15,
                speed: 450.0,
                color: Color::new(1.0, 0.15, 0.25, 0.9),
                thickness: 4.5,
            }));
            return false;
        }
        true
    }

    fn draw(&self, cam_offset: Vec2) {
        let sp = self.target_pos + cam_offset;
        let pct = (self.timer / self.max_time).clamp(0.0, 1.0);

        draw_circle(sp.x, sp.y, self.radius, Color::new(1.0, 0.05, 0.15, 0.16));
        draw_circle(
            sp.x,
            sp.y,
            self.radius * pct,
            Color::new(1.0, 0.1, 0.2, 0.35),
        );
        draw_circle_lines(
            sp.x,
            sp.y,
            self.radius,
            2.0,
            Color::new(1.0, 0.15, 0.25, 0.85),
        );

        let drop_y = sp.y - (1.0 - pct) * 420.0;
        draw_circle(sp.x, drop_y, 7.0, Color::new(1.0, 0.5, 0.1, 0.9));
    }
}

// =========================================================
// 6. TIA SÉT LIÊN HOÀN & TIA RAILGUN (Lightning / Beam Visual Event)
// =========================================================
pub struct BeamArcVisualEvent {
    pub points: Vec<Vec2>,
    pub color: Color,
    pub thickness: f32,
    pub life: f32,
    pub max_life: f32,
}

impl BattlefieldEvent for BeamArcVisualEvent {
    fn update(&mut self, _ctx: &mut CombatContext, dt: f32) -> bool {
        self.life -= dt;
        self.life > 0.0
    }

    fn draw(&self, cam_offset: Vec2) {
        let alpha = (self.life / self.max_life).clamp(0.0, 1.0);
        let col = Color::new(self.color.r, self.color.g, self.color.b, alpha);
        let core_col = Color::new(1.0, 1.0, 1.0, alpha);

        for pair in self.points.windows(2) {
            let p1 = pair[0] + cam_offset;
            let p2 = pair[1] + cam_offset;
            draw_line(p1.x, p1.y, p2.x, p2.y, self.thickness * 2.2, col);
            draw_line(p1.x, p1.y, p2.x, p2.y, self.thickness * 0.8, core_col);
        }
    }
}

// =========================================================
// 7. HÒM TIẾP TẾ THẢ DÙ TỪ QUỸ ĐẠO (Supply Pod Drop Event)
// =========================================================
pub struct SupplyPodDropEvent {
    pub landing_pos: Vec2,
    pub timer: f32,
    pub max_time: f32,
    pub weapon_id: usize,
    pub rarity: WeaponRarity,
}

impl BattlefieldEvent for SupplyPodDropEvent {
    fn update(&mut self, ctx: &mut CombatContext, dt: f32) -> bool {
        self.timer += dt;
        if self.timer >= self.max_time {
            ctx.trigger_explosion(self.landing_pos, 110.0, 180.0);
            ctx.spawn_event(Box::new(ShockwaveRingEvent {
                center: self.landing_pos,
                current_radius: 10.0,
                max_radius: 130.0,
                speed: 480.0,
                color: self.rarity.color(),
                thickness: 5.0,
            }));
            ctx.spawn_weapon_drop(WeaponDrop::new(
                self.landing_pos,
                self.weapon_id,
                self.rarity,
            ));
            ctx.spawn_float_text(
                self.landing_pos - vec2(0.0, 35.0),
                format!("SUPPLY DROP: [{}]", self.rarity.label()),
                self.rarity.color(),
                20.0,
            );
            return false;
        }
        true
    }

    fn draw(&self, cam_offset: Vec2) {
        let sp = self.landing_pos + cam_offset;
        let pct = (self.timer / self.max_time).clamp(0.0, 1.0);
        let col = self.rarity.color();

        draw_circle_lines(sp.x, sp.y, 44.0, 2.0, col);
        draw_circle(
            sp.x,
            sp.y,
            44.0 * pct,
            Color::new(col.r, col.g, col.b, 0.22),
        );

        let pod_y = sp.y - (1.0 - pct) * 550.0;
        draw_rectangle(
            sp.x - 14.0,
            pod_y - 22.0,
            28.0,
            32.0,
            Color::new(0.18, 0.24, 0.34, 1.0),
        );
        draw_rectangle_lines(sp.x - 14.0, pod_y - 22.0, 28.0, 32.0, 2.0, col);
    }
}
