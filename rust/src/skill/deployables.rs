use macroquad::prelude::*;
use std::f32::consts::PI;
use crate::core::context::CombatContext;
use crate::weapon::projectile::Projectile;

pub enum Deployable {
    AutoTurret {
        pos: Vec2,
        angle: f32,
        fire_timer: f32,
        fire_interval: f32,
        life: f32,
    },
    ClusterGrenade {
        pos: Vec2,
        vel: Vec2,
        fuse: f32,
        radius: f32,
        is_cluster_child: bool,
    },
}

impl Deployable {
    pub fn update(&mut self, ctx: &mut CombatContext, dt: f32) -> bool {
        match self {
            Deployable::AutoTurret {
                pos,
                angle,
                fire_timer,
                fire_interval,
                life,
            } => {
                *life -= dt;
                *fire_timer += dt;

                if let Some(target) = ctx.find_nearest_enemy(*pos, 460.0) {
                    *angle = (target.y - pos.y).atan2(target.x - pos.x);
                    if *fire_timer >= *fire_interval {
                        *fire_timer = 0.0;
                        let dir = vec2(angle.cos(), angle.sin());
                        ctx.spawn_player_projectile(Projectile {
                            pos: *pos + dir * 18.0,
                            vel: dir * 1150.0,
                            damage: 22.0,
                            color: Color::new(0.0, 0.94, 1.0, 1.0),
                            radius: 3.0,
                            explosion_radius: None,
                            spawns_black_hole: false,
                            pierce: 0,
                            bounces: 0,
                            life: 1.5,
                            status_effect: None,
                        });
                    }
                }

                *life > 0.0
            }
            Deployable::ClusterGrenade {
                pos,
                vel,
                fuse,
                is_cluster_child,
                ..
            } => {
                *pos += *vel * dt;
                *vel *= 0.93;
                *fuse -= dt;

                if *fuse <= 0.0 {
                    let blast_rad = if *is_cluster_child { 75.0 } else { 135.0 };
                    let blast_dmg = if *is_cluster_child { 85.0 } else { 160.0 };
                    ctx.trigger_explosion(*pos, blast_rad, blast_dmg);

                    // Spawn 4 mini cluster bombs if main grenade
                    if !*is_cluster_child {
                        for k in 0..4 {
                            let a = (PI * 2.0 * (k as f32)) / 4.0 + rand::gen_range(-0.3, 0.3);
                            let dir = vec2(a.cos(), a.sin());
                            ctx.spawn_deployable(Deployable::ClusterGrenade {
                                pos: *pos,
                                vel: dir * rand::gen_range(160.0, 240.0),
                                fuse: 0.35,
                                radius: 4.0,
                                is_cluster_child: true,
                            });
                        }
                    }
                    return false;
                }
                true
            }
        }
    }

    pub fn draw(&self, cam_offset: Vec2) {
        match self {
            Deployable::AutoTurret { pos, angle, .. } => {
                let sp = *pos + cam_offset;
                // Chân đế chữ X cố định trên mặt đất
                draw_line(sp.x - 13.0, sp.y - 13.0, sp.x + 13.0, sp.y + 13.0, 3.5, Color::new(0.3, 0.4, 0.55, 1.0));
                draw_line(sp.x + 13.0, sp.y - 13.0, sp.x - 13.0, sp.y + 13.0, 3.5, Color::new(0.3, 0.4, 0.55, 1.0));

                // Tháp pháo vuông xoay + 2 nòng súng song song
                let dir = vec2(angle.cos(), angle.sin());
                let perp = vec2(-dir.y, dir.x) * 4.5;
                for side in [-1.0f32, 1.0] {
                    let b_start = sp + perp * side;
                    let b_end = b_start + dir * 20.0;
                    draw_line(b_start.x, b_start.y, b_end.x, b_end.y, 3.5, Color::new(0.75, 0.88, 1.0, 1.0));
                }
                draw_poly(sp.x, sp.y, 4, 12.0, angle * 57.3 + 45.0, Color::new(0.1, 0.18, 0.28, 1.0));
                draw_poly_lines(sp.x, sp.y, 4, 12.0, angle * 57.3 + 45.0, 2.0, Color::new(0.0, 0.94, 1.0, 1.0));
            }
            Deployable::ClusterGrenade { pos, radius, .. } => {
                let sp = *pos + cam_offset;
                draw_poly(sp.x, sp.y, 6, *radius + 2.0, 0.0, Color::new(1.0, 0.25, 0.2, 1.0));
                draw_poly_lines(sp.x, sp.y, 6, *radius + 2.0, 0.0, 1.5, YELLOW);
            }
        }
    }
}
