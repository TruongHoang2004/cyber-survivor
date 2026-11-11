use macroquad::prelude::*;
use crate::core::context::CombatContext;
use crate::weapon::projectile::Projectile;

pub struct AlliedBody {
    pub name: &'static str,
    pub pos: Vec2,
    pub vel: Vec2,
    pub angle: f32,
    pub radius: f32,
    pub speed: f32,
    pub color: Color,
    pub hp: f32,
    pub max_hp: f32,
    pub life: f32,
}

/// Lớp trừu tượng cho mọi Đơn vị Đồng minh chiến đấu trên chiến trường
pub trait AlliedBehavior: Send + Sync {
    fn update(&mut self, body: &mut AlliedBody, ctx: &mut CombatContext, dt: f32);
    fn draw_custom(&self, _body: &AlliedBody, _screen_pos: Vec2) {}
}

pub struct AlliedEntity {
    pub body: AlliedBody,
    pub behavior: Box<dyn AlliedBehavior>,
}

impl AlliedEntity {
    pub fn new(body: AlliedBody, behavior: Box<dyn AlliedBehavior>) -> Self {
        Self { body, behavior }
    }

    pub fn tick(&mut self, ctx: &mut CombatContext, dt: f32) -> bool {
        self.body.life -= dt;
        self.body.pos += self.body.vel * dt;
        self.body.vel *= 0.88;
        self.behavior.update(&mut self.body, ctx, dt);
        self.body.life > 0.0 && self.body.hp > 0.0
    }

    pub fn draw(&self, cam_offset: Vec2, show_bars: bool) {
        let sp = self.body.pos + cam_offset;
        self.behavior.draw_custom(&self.body, sp);

        if show_bars {
            let bw = self.body.radius * 2.0;
            let pct = (self.body.hp / self.body.max_hp.max(1.0)).clamp(0.0, 1.0);
            draw_rectangle(
                sp.x - bw / 2.0,
                sp.y - self.body.radius - 7.0,
                bw * pct,
                3.0,
                Color::new(0.2, 1.0, 0.6, 0.85),
            );
            draw_text(
                self.body.name,
                sp.x - bw * 0.7,
                sp.y - self.body.radius - 10.0,
                11.0,
                Color::new(0.6, 0.95, 1.0, 0.85),
            );
        }
    }
}

// =========================================================
// 1. LÍNH THỦY QUÂN LỤC CHIẾN ĐỒNG MINH (Allied Marine)
// =========================================================
pub struct AlliedMarine {
    pub formation_offset: Vec2,
    pub fire_timer: f32,
}

impl AlliedBehavior for AlliedMarine {
    fn update(&mut self, body: &mut AlliedBody, ctx: &mut CombatContext, dt: f32) {
        let desired_pos = ctx.player_pos + self.formation_offset;
        let to_desired = desired_pos - body.pos;
        if to_desired.length() > 25.0 {
            body.pos += to_desired.normalize_or_zero() * body.speed * dt;
        }

        self.fire_timer += dt;
        if let Some(target) = ctx.find_nearest_enemy(body.pos, 520.0) {
            body.angle = (target.y - body.pos.y).atan2(target.x - body.pos.x);
            if self.fire_timer >= 0.18 {
                self.fire_timer = 0.0;
                let spread = rand::gen_range(-0.05, 0.05);
                let a = body.angle + spread;
                let dir = vec2(a.cos(), a.sin());
                ctx.spawn_player_projectile(Projectile::basic(
                    body.pos + dir * 16.0,
                    dir * 1150.0,
                    22.0,
                    Color::new(0.2, 0.95, 1.0, 1.0),
                    3.0,
                    1.5,
                ));
            }
        }
    }

    fn draw_custom(&self, body: &AlliedBody, sp: Vec2) {
        let fwd = vec2(body.angle.cos(), body.angle.sin());
        let right = vec2(-fwd.y, fwd.x);
        let r = body.radius;

        // Hai cầu vai giáp lính thủy quân lục chiến
        for side in [-1.0f32, 1.0] {
            let sh = sp + right * (side * r * 0.78);
            draw_poly(sh.x, sh.y, 4, 6.5, body.angle * 57.3, body.color);
        }
        // Thân giáp lục giác + mũ chiến thuật
        draw_poly(sp.x, sp.y, 6, r * 0.78, body.angle * 57.3, Color::new(0.08, 0.25, 0.35, 1.0));
        draw_poly_lines(sp.x, sp.y, 6, r * 0.78, body.angle * 57.3, 2.0, body.color);
        // Súng trường xung kích trên tay
        let gun_start = sp + right * 4.0;
        let gun_end = gun_start + fwd * (r + 9.0);
        draw_line(gun_start.x, gun_start.y, gun_end.x, gun_end.y, 3.8, WHITE);
    }
}

// =========================================================
// 2. ROBOT CHIẾN ĐẤU HẠNG NẶNG (Allied Heavy Mech)
// =========================================================
pub struct AlliedHeavyMech {
    pub fire_timer: f32,
    pub rocket_timer: f32,
}

impl AlliedBehavior for AlliedHeavyMech {
    fn update(&mut self, body: &mut AlliedBody, ctx: &mut CombatContext, dt: f32) {
        self.fire_timer += dt;
        self.rocket_timer += dt;

        if let Some(target) = ctx.find_nearest_enemy(body.pos, 650.0) {
            let to_target = target - body.pos;
            body.angle = to_target.y.atan2(to_target.x);

            if to_target.length() > 180.0 {
                body.pos += to_target.normalize_or_zero() * body.speed * dt;
            }

            // Súng máy đôi liên thanh
            if self.fire_timer >= 0.11 {
                self.fire_timer = 0.0;
                let dir = vec2(body.angle.cos(), body.angle.sin());
                let perp = vec2(-dir.y, dir.x) * 8.0;
                for side in [-1.0f32, 1.0] {
                    ctx.spawn_player_projectile(Projectile {
                        pos: body.pos + dir * 20.0 + perp * side,
                        vel: dir * 1200.0,
                        damage: 26.0,
                        color: Color::new(1.0, 0.85, 0.1, 1.0),
                        radius: 3.5,
                        explosion_radius: None,
                        spawns_black_hole: false,
                        pierce: 1,
                        bounces: 0,
                        life: 1.6,
                        status_effect: None,
                    });
                }
            }

            // Phóng tên lửa tầm nhiệt mỗi 2.2s
            if self.rocket_timer >= 2.2 {
                self.rocket_timer = 0.0;
                let dir = vec2(body.angle.cos(), body.angle.sin());
                ctx.spawn_player_projectile(Projectile {
                    pos: body.pos + dir * 24.0,
                    vel: dir * 700.0,
                    damage: 140.0,
                    color: Color::new(1.0, 0.3, 0.1, 1.0),
                    radius: 7.0,
                    explosion_radius: Some(115.0),
                    spawns_black_hole: false,
                    pierce: 0,
                    bounces: 0,
                    life: 2.0,
                    status_effect: None,
                });
            }
        } else {
            // Đi theo bảo vệ người chơi nếu không có quái gần
            let to_player = ctx.player_pos - body.pos;
            if to_player.length() > 90.0 {
                body.pos += to_player.normalize_or_zero() * body.speed * dt;
            }
        }
    }

    fn draw_custom(&self, body: &AlliedBody, sp: Vec2) {
        let fwd = vec2(body.angle.cos(), body.angle.sin());
        let right = vec2(-fwd.y, fwd.x);
        let r = body.radius;

        // 2 dàn pháo đôi gắn trên hai vai Robot Mech
        for side in [-1.0f32, 1.0] {
            let arm = sp + right * (side * (r * 0.92));
            let barrel_tip = arm + fwd * (r + 10.0);
            draw_line(arm.x, arm.y, barrel_tip.x, barrel_tip.y, 5.5, Color::new(0.75, 0.85, 0.95, 1.0));
        }

        // Thân Robot hình hộp bát giác bọc thép xanh ngọc
        draw_poly(sp.x, sp.y, 4, r, body.angle * 57.3 + 45.0, Color::new(0.08, 0.25, 0.2, 0.98));
        draw_poly_lines(sp.x, sp.y, 4, r, body.angle * 57.3 + 45.0, 2.8, body.color);
        draw_poly(sp.x, sp.y, 4, r * 0.48, body.angle * 57.3, Color::new(1.0, 0.85, 0.1, 1.0));
    }
}

// =========================================================
// 3. DRONE HỘ TỐNG XOAY QUANH NGƯỜI CHƠI (Orbiting Drone)
// =========================================================
pub struct OrbitingCombatDrone {
    pub orbit_angle: f32,
    pub orbit_radius: f32,
    pub fire_timer: f32,
}

impl AlliedBehavior for OrbitingCombatDrone {
    fn update(&mut self, body: &mut AlliedBody, ctx: &mut CombatContext, dt: f32) {
        self.orbit_angle += dt * 3.2;
        body.pos = ctx.player_pos
            + vec2(self.orbit_angle.cos(), self.orbit_angle.sin()) * self.orbit_radius;

        self.fire_timer += dt;
        if let Some(target) = ctx.find_nearest_enemy(body.pos, 420.0) {
            body.angle = (target.y - body.pos.y).atan2(target.x - body.pos.x);
            if self.fire_timer >= 0.24 {
                self.fire_timer = 0.0;
                let dir = vec2(body.angle.cos(), body.angle.sin());
                ctx.spawn_player_projectile(Projectile {
                    pos: body.pos,
                    vel: dir * 1050.0,
                    damage: 24.0,
                    color: Color::new(0.6, 0.3, 1.0, 1.0),
                    radius: 4.0,
                    explosion_radius: None,
                    spawns_black_hole: false,
                    pierce: 1,
                    bounces: 0,
                    life: 1.4,
                    status_effect: None,
                });
            }
        }
    }

    fn draw_custom(&self, body: &AlliedBody, sp: Vec2) {
        let fwd = vec2(body.angle.cos(), body.angle.sin());
        let right = vec2(-fwd.y, fwd.x);
        let r = body.radius;

        // Hình dáng phi cơ cánh én (Chevron Stealth Wing)
        let nose = sp + fwd * (r * 1.3);
        let wing_l = sp - fwd * (r * 0.6) + right * (r * 1.25);
        let wing_r = sp - fwd * (r * 0.6) - right * (r * 1.25);
        draw_triangle(nose, wing_l, wing_r, body.color);
        draw_triangle_lines(nose, wing_l, wing_r, 1.8, WHITE);
    }
}

// =========================================================
// 5. ORBITING PLASMA SPHERE (Cầu Plasma Xoay Quanh Bảo Hộ)
// =========================================================
pub struct OrbitingPlasmaSphere {
    pub orbit_angle: f32,
    pub orbit_radius: f32,
    pub orbit_speed: f32,
    pub pulse_timer: f32,
}

impl AlliedBehavior for OrbitingPlasmaSphere {
    fn update(&mut self, body: &mut AlliedBody, ctx: &mut CombatContext, dt: f32) {
        self.orbit_angle += dt * self.orbit_speed;
        body.pos = ctx.player_pos
            + vec2(self.orbit_angle.cos(), self.orbit_angle.sin()) * self.orbit_radius;

        self.pulse_timer += dt;
        if self.pulse_timer >= 0.10 {
            self.pulse_timer = 0.0;
            // Xung kích plasma thiêu rụi quái vật tiếp xúc
            ctx.trigger_explosion(body.pos, 38.0, 85.0);
        }
    }

    fn draw_custom(&self, body: &AlliedBody, sp: Vec2) {
        let t = get_time() as f32 * 6.0;
        let pulse = (t.sin() * 0.2 + 1.0) * body.radius;
        draw_circle(sp.x, sp.y, pulse * 1.7, Color::new(1.0, 0.3, 0.85, 0.28));
        draw_circle(sp.x, sp.y, pulse * 1.15, Color::new(0.2, 0.9, 1.0, 0.75));
        draw_circle(sp.x, sp.y, pulse * 0.6, WHITE);
    }
}
