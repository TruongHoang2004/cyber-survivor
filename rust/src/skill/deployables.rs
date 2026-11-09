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
                draw_circle(sp.x, sp.y, 14.0, Color::new(0.12, 0.16, 0.24, 1.0));
                draw_circle_lines(sp.x, sp.y, 14.0, 2.0, Color::new(0.0, 0.94, 1.0, 1.0));
                let dir = vec2(angle.cos(), angle.sin());
                draw_line(
                    sp.x,
                    sp.y,
                    sp.x + dir.x * 18.0,
                    sp.y + dir.y * 18.0,
                    4.0,
                    Color::new(0.6, 0.7, 0.8, 1.0),
                );
            }
            Deployable::ClusterGrenade { pos, radius, .. } => {
                let sp = *pos + cam_offset;
                draw_circle(sp.x, sp.y, *radius, Color::new(1.0, 0.2, 0.27, 1.0));
                draw_circle_lines(sp.x, sp.y, *radius, 1.5, WHITE);
            }
        }
    }
}
