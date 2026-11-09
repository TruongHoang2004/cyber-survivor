use macroquad::prelude::*;
use crate::core::context::CombatContext;
use crate::core::props::{WeaponDrop, WeaponRarity};
use crate::core::types::StatusEffect;

/// Lớp trừu tượng cho từng Bộ phận độc lập của Đại Boss (Multi-Part Boss System)
pub trait BossPartBehavior: Send + Sync {
    fn update_part(
        &mut self,
        boss_pos: Vec2,
        part_world_pos: Vec2,
        ctx: &mut CombatContext,
        dt: f32,
    );

    fn is_shield_generator(&self) -> bool {
        false
    }
}

pub struct BossPart {
    pub name: &'static str,
    pub offset: Vec2,
    pub radius: f32,
    pub hp: f32,
    pub max_hp: f32,
    pub color: Color,
    pub hit_flash: f32,
    pub behavior: Box<dyn BossPartBehavior>,
}

pub struct EnemyBody {
    pub name: &'static str,
    pub pos: Vec2,
    pub vel: Vec2,
    pub radius: f32,
    pub speed: f32,
    pub color: Color,
    pub hp: f32,
    pub max_hp: f32,
    pub is_boss: bool,
    pub score_reward: u64,
    pub hit_flash: f32,
    pub statuses: Vec<StatusEffect>,
    pub parts: Vec<BossPart>,
}

pub trait EnemyBehavior: Send + Sync {
    fn update(&mut self, body: &mut EnemyBody, ctx: &mut CombatContext, dt: f32);

    fn on_damaged(&mut self, _body: &mut EnemyBody, _dmg: f32, _ctx: &mut CombatContext) {}

    fn on_death(&mut self, _body: &EnemyBody, _ctx: &mut CombatContext) {}

    fn draw_custom(&self, _body: &EnemyBody, _screen_pos: Vec2, _cam_offset: Vec2) {}
}

pub struct EnemyEntity {
    pub body: EnemyBody,
    pub behavior: Box<dyn EnemyBehavior>,
}

impl EnemyEntity {
    pub fn new(body: EnemyBody, behavior: Box<dyn EnemyBehavior>) -> Self {
        Self { body, behavior }
    }

    pub fn apply_status(&mut self, effect: StatusEffect) {
        self.body.statuses.push(effect);
    }

    /// Kiểm tra xem đạn có bắn trúng bộ phận phụ (BossPart) hoặc thân chính của quái không
    pub fn check_hit(&self, proj_pos: Vec2, proj_radius: f32) -> bool {
        for part in &self.body.parts {
            let p_pos = self.body.pos + part.offset;
            if (p_pos - proj_pos).length() < part.radius + proj_radius {
                return true;
            }
        }
        (self.body.pos - proj_pos).length() < self.body.radius + proj_radius
    }

    pub fn take_damage_at(
        &mut self,
        hit_pos: Vec2,
        dmg: f32,
        knockback: Vec2,
        status: Option<StatusEffect>,
        ctx: &mut CombatContext,
    ) -> f32 {
        // 1. Ưu tiên kiểm tra xem đạn có trúng Bộ Phận Phụ (BossPart) của Đại Boss không
        for i in 0..self.body.parts.len() {
            let part_world_pos = self.body.pos + self.body.parts[i].offset;
            if (part_world_pos - hit_pos).length() <= self.body.parts[i].radius + 12.0 {
                let part = &mut self.body.parts[i];
                part.hp -= dmg;
                part.hit_flash = 0.08;
                if part.hp <= 0.0 {
                    let destroyed = self.body.parts.swap_remove(i);
                    let drop_pos = self.body.pos + destroyed.offset;
                    ctx.trigger_explosion(drop_pos, 95.0, 120.0);
                    ctx.spawn_float_text(
                        drop_pos - vec2(0.0, 25.0),
                        format!("BROKEN: {}!", destroyed.name),
                        Color::new(1.0, 0.85, 0.0, 1.0),
                        22.0,
                    );
                    // Có 30% tỉ lệ rơi súng hiếm khi bắn nổ 1 bộ phận Boss
                    if rand::gen_range(0.0, 1.0) < 0.30 {
                        ctx.spawn_weapon_drop(WeaponDrop::new(
                            drop_pos,
                            rand::gen_range(1, 7),
                            WeaponRarity::Epic,
                        ));
                    }
                }
                return dmg;
            }
        }

        // 2. Nếu bắn vào thân chính: kiểm tra xem Boss còn Bộ Phát Khiên (Shield Generator) không
        let has_shield_gen = self
            .body
            .parts
            .iter()
            .any(|p| p.behavior.is_shield_generator());
        let actual_dmg = if has_shield_gen { dmg * 0.35 } else { dmg };

        self.body.hp -= actual_dmg;
        self.body.hit_flash = 0.07;
        if !self.body.is_boss {
            self.body.vel += knockback;
        } else {
            self.body.vel += knockback * 0.15;
        }
        if let Some(st) = status {
            self.apply_status(st);
        }
        self.behavior.on_damaged(&mut self.body, actual_dmg, ctx);
        actual_dmg
    }

    pub fn tick(&mut self, ctx: &mut CombatContext, dt: f32) {
        if self.body.hit_flash > 0.0 {
            self.body.hit_flash = (self.body.hit_flash - dt).max(0.0);
        }
        for p in self.body.parts.iter_mut() {
            if p.hit_flash > 0.0 {
                p.hit_flash = (p.hit_flash - dt).max(0.0);
            }
        }

        let mut speed_mult = 1.0f32;
        let mut stunned = false;

        for st in self.body.statuses.iter_mut() {
            match st {
                StatusEffect::Burn { dps, duration } => {
                    self.body.hp -= *dps * dt;
                    *duration -= dt;
                }
                StatusEffect::Slow { factor, duration } => {
                    speed_mult = speed_mult.min(*factor);
                    *duration -= dt;
                }
                StatusEffect::Stun { duration } => {
                    stunned = true;
                    *duration -= dt;
                }
            }
        }

        self.body.statuses.retain(|st| match st {
            StatusEffect::Burn { duration, .. }
            | StatusEffect::Slow { duration, .. }
            | StatusEffect::Stun { duration } => *duration > 0.0,
        });

        self.body.pos += self.body.vel * dt;
        self.body.vel *= 0.88;

        if !stunned {
            let orig_speed = self.body.speed;
            self.body.speed *= speed_mult;
            self.behavior.update(&mut self.body, ctx, dt);
            self.body.speed = orig_speed;

            // Cập nhật từng bộ phận còn sống của Boss
            let boss_pos = self.body.pos;
            for part in self.body.parts.iter_mut() {
                let part_pos = boss_pos + part.offset;
                part.behavior.update_part(boss_pos, part_pos, ctx, dt);
            }
        }
    }

    pub fn draw(&self, cam_offset: Vec2) {
        let sp = self.body.pos + cam_offset;

        // Nếu Boss còn Shield Generator, vẽ khung khiên lục giác năng lượng xanh bảo vệ thân chính
        if self
            .body
            .parts
            .iter()
            .any(|p| p.behavior.is_shield_generator())
        {
            draw_poly(
                sp.x,
                sp.y,
                6,
                self.body.radius + 14.0,
                15.0,
                Color::new(0.0, 0.85, 1.0, 0.16),
            );
            draw_poly_lines(
                sp.x,
                sp.y,
                6,
                self.body.radius + 14.0,
                15.0,
                3.0,
                Color::new(0.0, 0.95, 1.0, 0.85),
            );
        }

        // Vẽ các khớp xương cơ khí nối từ thân chính tới các bộ phận phụ (BossPart)
        for part in &self.body.parts {
            let p_sp = sp + part.offset;
            draw_line(
                sp.x,
                sp.y,
                p_sp.x,
                p_sp.y,
                8.0,
                Color::new(0.22, 0.26, 0.34, 1.0),
            );
            draw_line(
                sp.x,
                sp.y,
                p_sp.x,
                p_sp.y,
                3.0,
                Color::new(0.55, 0.65, 0.78, 0.9),
            );
        }

        // Status aura (Hiệu ứng cháy / đóng băng / choáng)
        for st in &self.body.statuses {
            match st {
                StatusEffect::Burn { .. } => {
                    draw_poly_lines(
                        sp.x,
                        sp.y,
                        5,
                        self.body.radius + 5.0,
                        (get_time() as f32) * 90.0,
                        2.5,
                        Color::new(1.0, 0.45, 0.0, 0.85),
                    );
                }
                StatusEffect::Slow { .. } => {
                    draw_poly_lines(
                        sp.x,
                        sp.y,
                        6,
                        self.body.radius + 5.0,
                        0.0,
                        2.5,
                        Color::new(0.2, 0.85, 1.0, 0.85),
                    );
                }
                StatusEffect::Stun { .. } => {
                    draw_poly_lines(
                        sp.x,
                        sp.y,
                        3,
                        self.body.radius + 7.0,
                        (get_time() as f32) * 180.0,
                        2.5,
                        YELLOW,
                    );
                }
            }
        }

        // Mỗi loại quái tự vẽ hình dáng đặc trưng riêng trong `draw_custom` (không dùng hình tròn chung nữa)
        self.behavior.draw_custom(&self.body, sp, cam_offset);

        // Vẽ từng bộ phận phụ (BossPart) với hình khối cơ khí riêng biệt và thanh máu riêng
        for (idx, part) in self.body.parts.iter().enumerate() {
            let p_sp = sp + part.offset;
            let p_col = if part.hit_flash > 0.0 {
                WHITE
            } else {
                part.color
            };
            let sides = match idx % 4 {
                0 => 4, // Mortar Cannon: Khối pháo vuông
                1 => 3, // Laser Turret: Tháp tam giác
                2 => 6, // Shield Generator: Lõi lục giác
                _ => 5, // Drone Hatchery: Khoang ngũ giác
            };
            let rot = (get_time() as f32) * 35.0 * if idx % 2 == 0 { 1.0 } else { -1.0 };
            draw_poly(p_sp.x, p_sp.y, sides, part.radius, rot, Color::new(0.12, 0.15, 0.22, 0.96));
            draw_poly(p_sp.x, p_sp.y, sides, part.radius * 0.72, rot, p_col);
            draw_poly_lines(p_sp.x, p_sp.y, sides, part.radius, rot, 2.5, WHITE);

            let bw = part.radius * 2.2;
            let pct = (part.hp / part.max_hp).clamp(0.0, 1.0);
            draw_rectangle(
                p_sp.x - bw / 2.0,
                p_sp.y - part.radius - 10.0,
                bw,
                4.0,
                Color::new(0.0, 0.0, 0.0, 0.8),
            );
            draw_rectangle(
                p_sp.x - bw / 2.0,
                p_sp.y - part.radius - 10.0,
                bw * pct,
                4.0,
                Color::new(1.0, 0.75, 0.0, 1.0),
            );
        }

        // Health bar cho quái thường
        if self.body.hp < self.body.max_hp && !self.body.is_boss {
            let bw = self.body.radius * 2.1;
            let pct = (self.body.hp / self.body.max_hp).clamp(0.0, 1.0);
            draw_rectangle(
                sp.x - bw / 2.0,
                sp.y - self.body.radius - 10.0,
                bw,
                4.0,
                Color::new(0.0, 0.0, 0.0, 0.75),
            );
            draw_rectangle(
                sp.x - bw / 2.0,
                sp.y - self.body.radius - 10.0,
                bw * pct,
                4.0,
                Color::new(0.0, 1.0, 0.45, 1.0),
            );
        }
    }
}
