use macroquad::prelude::*;
use crate::core::context::CombatContext;
use crate::core::types::{Particle, PlayerBody};
use crate::skill::deployables::Deployable;
use crate::skill::traits::{SkillBehavior, SkillSlot};

#[derive(Clone)]
pub struct TacticalDashSkill;

impl SkillBehavior for TacticalDashSkill {
    fn name(&self) -> &'static str {
        "DASH"
    }

    fn key_label(&self) -> &'static str {
        "SPACE"
    }

    fn activate(
        &mut self,
        player: &mut PlayerBody,
        _aim_world: Vec2,
        move_dir: Vec2,
        ctx: &mut CombatContext,
    ) -> bool {
        if player.is_dashing {
            return false;
        }

        let dash_dir = if move_dir != Vec2::ZERO {
            move_dir.normalize()
        } else {
            vec2(player.angle.cos(), player.angle.sin())
        };

        player.is_dashing = true;
        player.dash_duration = 0.22;
        player.vel = dash_dir * 820.0;
        ctx.add_screen_shake(4.0);

        for _ in 0..14 {
            ctx.spawn_particle(Particle {
                pos: player.pos,
                vel: -dash_dir * 200.0
                    + vec2(
                        rand::gen_range(-100.0, 100.0),
                        rand::gen_range(-100.0, 100.0),
                    ),
                color: Color::new(1.0, 0.8, 0.0, 1.0),
                radius: rand::gen_range(2.0, 5.0),
                life: 0.3,
                max_life: 0.3,
            });
        }
        true
    }

    fn clone_box(&self) -> Box<dyn SkillBehavior> {
        Box::new(self.clone())
    }
}

#[derive(Clone)]
pub struct ClusterGrenadeSkill;

impl SkillBehavior for ClusterGrenadeSkill {
    fn name(&self) -> &'static str {
        "GRENADE"
    }

    fn key_label(&self) -> &'static str {
        "Q"
    }

    fn activate(
        &mut self,
        player: &mut PlayerBody,
        aim_world: Vec2,
        _move_dir: Vec2,
        ctx: &mut CombatContext,
    ) -> bool {
        let dir = (aim_world - player.pos).normalize_or_zero();
        ctx.spawn_deployable(Deployable::ClusterGrenade {
            pos: player.pos,
            vel: dir * 480.0,
            fuse: 1.25,
            radius: 6.0,
            is_cluster_child: false,
        });
        true
    }

    fn clone_box(&self) -> Box<dyn SkillBehavior> {
        Box::new(self.clone())
    }
}

#[derive(Clone)]
pub struct AutoTurretSkill;

impl SkillBehavior for AutoTurretSkill {
    fn name(&self) -> &'static str {
        "TURRET"
    }

    fn key_label(&self) -> &'static str {
        "E"
    }

    fn activate(
        &mut self,
        player: &mut PlayerBody,
        _aim_world: Vec2,
        _move_dir: Vec2,
        ctx: &mut CombatContext,
    ) -> bool {
        let fire_interval = if player.modifiers.turret_overclock {
            0.11
        } else {
            0.22
        };
        let life = if player.modifiers.turret_overclock {
            35.0
        } else {
            20.0
        };
        ctx.spawn_deployable(Deployable::AutoTurret {
            pos: player.pos,
            angle: player.angle,
            fire_timer: 0.0,
            fire_interval,
            life,
        });
        true
    }

    fn clone_box(&self) -> Box<dyn SkillBehavior> {
        Box::new(self.clone())
    }
}

pub fn default_skills() -> Vec<SkillSlot> {
    vec![
        SkillSlot::new(KeyCode::Space, 1.2, Box::new(TacticalDashSkill)),
        SkillSlot::new(KeyCode::Q, 8.0, Box::new(ClusterGrenadeSkill)),
        SkillSlot::new(KeyCode::E, 16.0, Box::new(AutoTurretSkill)),
    ]
}
