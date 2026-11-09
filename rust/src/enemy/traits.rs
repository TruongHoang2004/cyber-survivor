use macroquad::prelude::*;
use crate::core::context::CombatContext;
use crate::core::types::StatusEffect;

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
    pub statuses: Vec<StatusEffect>,
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

    pub fn take_damage(
        &mut self,
        dmg: f32,
        knockback: Vec2,
        status: Option<StatusEffect>,
        ctx: &mut CombatContext,
    ) {
        self.body.hp -= dmg;
        self.body.vel += knockback;
        if let Some(st) = status {
            self.apply_status(st);
        }
        self.behavior.on_damaged(&mut self.body, dmg, ctx);
    }

    pub fn tick(&mut self, ctx: &mut CombatContext, dt: f32) {
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
        }
    }

    pub fn draw(&self, cam_offset: Vec2) {
        let sp = self.body.pos + cam_offset;

        // Custom overlays (e.g. sniper laser sight, charge indicator)
        self.behavior.draw_custom(&self.body, sp, cam_offset);

        // Status visual aura
        for st in &self.body.statuses {
            match st {
                StatusEffect::Burn { .. } => {
                    draw_circle(sp.x, sp.y, self.body.radius + 3.5, Color::new(1.0, 0.4, 0.0, 0.45));
                }
                StatusEffect::Slow { .. } => {
                    draw_circle(sp.x, sp.y, self.body.radius + 3.5, Color::new(0.2, 0.8, 1.0, 0.45));
                }
                StatusEffect::Stun { .. } => {
                    draw_circle_lines(sp.x, sp.y, self.body.radius + 5.0, 2.0, YELLOW);
                }
            }
        }

        // Core body
        draw_circle(sp.x, sp.y, self.body.radius, self.body.color);
        draw_circle(sp.x, sp.y, self.body.radius * 0.42, BLACK);

        // Health bar
        if self.body.hp < self.body.max_hp && !self.body.is_boss {
            let bw = self.body.radius * 2.0;
            let pct = (self.body.hp / self.body.max_hp).clamp(0.0, 1.0);
            draw_rectangle(
                sp.x - bw / 2.0,
                sp.y - self.body.radius - 8.0,
                bw,
                3.5,
                Color::new(0.0, 0.0, 0.0, 0.65),
            );
            draw_rectangle(
                sp.x - bw / 2.0,
                sp.y - self.body.radius - 8.0,
                bw * pct,
                3.5,
                Color::new(0.0, 1.0, 0.4, 1.0),
            );
        }
    }
}
