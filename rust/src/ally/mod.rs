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

    pub fn draw(&self, cam_offset: Vec2) {
        let sp = self.body.pos + cam_offset;
        self.behavior.draw_custom(&self.body, sp);

        draw_circle(sp.x, sp.y, self.body.radius, self.body.color);
        draw_circle(sp.x, sp.y, self.body.radius * 0.45, Color::new(0.05, 0.1, 0.18, 1.0));

        let dir = vec2(self.body.angle.cos(), self.body.angle.sin());
        draw_line(
            sp.x,
            sp.y,
            sp.x + dir.x * (self.body.radius + 6.0),
            sp.y + dir.y * (self.body.radius + 6.0),
            3.5,
            WHITE,
        );

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

    fn draw_custom(&self, body: &AlliedBody, screen_pos: Vec2) {
        draw_circle_lines(
            screen_pos.x,
            screen_pos.y,
            body.radius + 4.0,
            2.5,
            Color::new(0.2, 1.0, 0.6, 0.75),
        );
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
}
