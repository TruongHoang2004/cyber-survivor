use macroquad::prelude::*;
use crate::ally::{
    AlliedBody, AlliedEntity, AlliedHeavyMech, AlliedMarine, OrbitingCombatDrone,
};
use crate::core::context::CombatContext;
use crate::core::events::{
    CarpetBombingRunEvent, OrbitalLaserStrikeEvent, ShockwaveRingEvent, SingularityBlackHoleEvent,
    SupplyPodDropEvent,
};
use crate::core::props::WeaponRarity;
use crate::core::types::{Particle, PlayerBody, WORLD_HEIGHT, WORLD_WIDTH};
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
        player.vel = dash_dir * 840.0;
        ctx.add_screen_shake(4.0);

        ctx.spawn_event(Box::new(ShockwaveRingEvent {
            center: player.pos,
            current_radius: 8.0,
            max_radius: 75.0,
            speed: 450.0,
            color: Color::new(1.0, 0.85, 0.1, 0.8),
            thickness: 3.5,
        }));

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
pub struct TacticalAirstrikeSkill;

impl SkillBehavior for TacticalAirstrikeSkill {
    fn name(&self) -> &'static str {
        "AIRSTRIKE + LASER"
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
        let strike_dir = if dir == Vec2::ZERO {
            vec2(1.0, 0.0)
        } else {
            dir
        };

        // 1. Không kích rải thảm 10 quả bom liên hoàn dọc theo hướng ngắm
        ctx.spawn_event(Box::new(CarpetBombingRunEvent {
            start_pos: player.pos + strike_dir * 75.0,
            dir: strike_dir,
            total_bombs: 10,
            bombs_dropped: 0,
            step_distance: 65.0,
            timer: 0.0,
            warmup_timer: 0.25,
        }));

        // 2. Tia Laser Vệ Tinh Quỹ Đạo quét mục tiêu tại vị trí con trỏ chuột
        ctx.spawn_event(Box::new(OrbitalLaserStrikeEvent {
            pos: aim_world,
            speed: 240.0,
            radius: 48.0,
            dps: 260.0,
            duration: 4.5,
            decal_timer: 0.0,
        }));

        // 3. Ném thêm Lựu Đạn Nổ Chùm tầm gần
        ctx.spawn_deployable(Deployable::ClusterGrenade {
            pos: player.pos,
            vel: strike_dir * 460.0,
            fuse: 1.0,
            radius: 6.0,
            is_cluster_child: false,
        });

        ctx.spawn_float_text(
            player.pos - vec2(0.0, 40.0),
            "TACTICAL AIRSTRIKE & ORBITAL LASER!",
            Color::new(1.0, 0.4, 0.1, 1.0),
            22.0,
        );
        true
    }

    fn clone_box(&self) -> Box<dyn SkillBehavior> {
        Box::new(self.clone())
    }
}

#[derive(Clone)]
pub struct ReinforcementsDropSkill;

impl SkillBehavior for ReinforcementsDropSkill {
    fn name(&self) -> &'static str {
        "SQUAD + SUPPLY"
    }

    fn key_label(&self) -> &'static str {
        "E"
    }

    fn activate(
        &mut self,
        player: &mut PlayerBody,
        aim_world: Vec2,
        _move_dir: Vec2,
        ctx: &mut CombatContext,
    ) -> bool {
        // 1. Triển khai 2 Lính Thủy Quân Lục Chiến Đồng Minh đi theo đội hình
        for (i, offset) in [vec2(-55.0, -45.0), vec2(-55.0, 45.0)].iter().enumerate() {
            ctx.spawn_ally(AlliedEntity::new(
                AlliedBody {
                    name: "ALLIED MARINE",
                    pos: player.pos + *offset,
                    vel: Vec2::ZERO,
                    angle: player.angle,
                    radius: 13.0,
                    speed: 285.0,
                    color: Color::new(0.15, 0.85, 1.0, 1.0),
                    hp: 150.0,
                    max_hp: 150.0,
                    life: 30.0,
                },
                Box::new(AlliedMarine {
                    formation_offset: *offset,
                    fire_timer: (i as f32) * 0.09,
                }),
            ));
        }

        // 2. Triển khai 1 Robot Chiến Đấu Hạng Nặng (Heavy Mech)
        ctx.spawn_ally(AlliedEntity::new(
            AlliedBody {
                name: "HEAVY MECH",
                pos: player.pos + vec2(45.0, 0.0),
                vel: Vec2::ZERO,
                angle: player.angle,
                radius: 22.0,
                speed: 195.0,
                color: Color::new(0.2, 1.0, 0.55, 1.0),
                hp: 400.0,
                max_hp: 400.0,
                life: 32.0,
            },
            Box::new(AlliedHeavyMech {
                fire_timer: 0.0,
                rocket_timer: 0.5,
            }),
        ));

        // 3. Triển khai 1 Drone Hộ Tống xoay quanh người chơi
        ctx.spawn_ally(AlliedEntity::new(
            AlliedBody {
                name: "ORBIT DRONE",
                pos: player.pos,
                vel: Vec2::ZERO,
                angle: 0.0,
                radius: 9.0,
                speed: 300.0,
                color: Color::new(0.75, 0.35, 1.0, 1.0),
                hp: 100.0,
                max_hp: 100.0,
                life: 35.0,
            },
            Box::new(OrbitingCombatDrone {
                orbit_angle: rand::gen_range(0.0, 6.28),
                orbit_radius: 56.0,
                fire_timer: 0.0,
            }),
        ));

        // 4. Đặt 1 Ụ Súng Tự Động tại chỗ
        let fire_interval = if player.modifiers.turret_overclock {
            0.10
        } else {
            0.20
        };
        ctx.spawn_deployable(Deployable::AutoTurret {
            pos: player.pos,
            angle: player.angle,
            fire_timer: 0.0,
            fire_interval,
            life: 28.0,
        });

        // 5. Gọi 1 Hòm Tiếp Tế Vũ Khí Hạng Nặng rơi từ quỹ đạo xuống vị trí chuột
        let weapon_id = rand::gen_range(1, 7);
        ctx.spawn_event(Box::new(SupplyPodDropEvent {
            landing_pos: aim_world,
            timer: 0.0,
            max_time: 1.1,
            weapon_id,
            rarity: WeaponRarity::random(),
        }));

        ctx.spawn_float_text(
            player.pos - vec2(0.0, 40.0),
            "REINFORCEMENTS & SUPPLY POD INBOUND!",
            Color::new(0.2, 1.0, 0.6, 1.0),
            22.0,
        );
        true
    }

    fn clone_box(&self) -> Box<dyn SkillBehavior> {
        Box::new(self.clone())
    }
}

pub fn default_skills() -> Vec<SkillSlot> {
    vec![
        SkillSlot::new(KeyCode::Space, 1.1, Box::new(TacticalDashSkill)),
        SkillSlot::new(KeyCode::Q, 7.5, Box::new(TacticalAirstrikeSkill)),
        SkillSlot::new(KeyCode::E, 14.0, Box::new(ReinforcementsDropSkill)),
    ]
}

// =========================================================
// VOID STALKER SPECIAL SKILLS (Phase Blink & Void Storm)
// =========================================================
#[derive(Clone)]
pub struct PhaseBlinkSkill;

impl SkillBehavior for PhaseBlinkSkill {
    fn name(&self) -> &'static str {
        "PHASE BLINK"
    }

    fn key_label(&self) -> &'static str {
        "SPACE"
    }

    fn activate(
        &mut self,
        player: &mut PlayerBody,
        aim_world: Vec2,
        move_dir: Vec2,
        ctx: &mut CombatContext,
    ) -> bool {
        let old_pos = player.pos;
        let blink_dir = if move_dir != Vec2::ZERO {
            move_dir.normalize()
        } else {
            (aim_world - player.pos).normalize_or_zero()
        };
        let target = (player.pos + blink_dir * 235.0).clamp(
            vec2(player.radius, player.radius),
            vec2(WORLD_WIDTH - player.radius, WORLD_HEIGHT - player.radius),
        );

        // Để lại Hố Đen Không Gian nhỏ tại điểm biến mất
        ctx.spawn_event(Box::new(SingularityBlackHoleEvent {
            pos: old_pos,
            pull_radius: 175.0,
            pull_force: 320.0,
            dps: 85.0,
            duration: 1.5,
            max_duration: 1.5,
            rotation: 0.0,
        }));

        player.pos = target;
        player.is_dashing = true;
        player.dash_duration = 0.16;
        player.vel = Vec2::ZERO;

        ctx.spawn_event(Box::new(ShockwaveRingEvent {
            center: target,
            current_radius: 10.0,
            max_radius: 110.0,
            speed: 560.0,
            color: Color::new(0.75, 0.25, 1.0, 0.9),
            thickness: 4.5,
        }));
        true
    }

    fn clone_box(&self) -> Box<dyn SkillBehavior> {
        Box::new(self.clone())
    }
}

#[derive(Clone)]
pub struct VoidTempestSkill;

impl SkillBehavior for VoidTempestSkill {
    fn name(&self) -> &'static str {
        "VOID TEMPEST"
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
        // Phóng 6 luồng sét không gian tỏa 360 độ quanh nhân vật
        for i in 0..6 {
            let a = (i as f32) * std::f32::consts::PI / 3.0;
            let dir = vec2(a.cos(), a.sin());
            ctx.chain_lightning(player.pos, dir, 5, 340.0, 95.0);
        }

        // Kèm 1 Hố Đen Vũ Trụ ngay tại tâm con trỏ chuột
        ctx.spawn_event(Box::new(SingularityBlackHoleEvent {
            pos: aim_world,
            pull_radius: 280.0,
            pull_force: 400.0,
            dps: 120.0,
            duration: 3.2,
            max_duration: 3.2,
            rotation: 0.0,
        }));

        ctx.spawn_float_text(
            player.pos - vec2(0.0, 40.0),
            "VOID SINGULARITY TEMPEST!",
            Color::new(0.8, 0.3, 1.0, 1.0),
            22.0,
        );
        true
    }

    fn clone_box(&self) -> Box<dyn SkillBehavior> {
        Box::new(self.clone())
    }
}

// =========================================================
// TECH ENGINEER SPECIAL SKILLS (Mecha Legion Drop)
// =========================================================
#[derive(Clone)]
pub struct MechaLegionDropSkill;

impl SkillBehavior for MechaLegionDropSkill {
    fn name(&self) -> &'static str {
        "MECHA LEGION"
    }

    fn key_label(&self) -> &'static str {
        "E"
    }

    fn activate(
        &mut self,
        player: &mut PlayerBody,
        aim_world: Vec2,
        _move_dir: Vec2,
        ctx: &mut CombatContext,
    ) -> bool {
        // Triển khai cùng lúc 2 Đại Robot Heavy Mech!
        for offset in [vec2(-55.0, 0.0), vec2(55.0, 0.0)] {
            ctx.spawn_ally(AlliedEntity::new(
                AlliedBody {
                    name: "TITAN MECH",
                    pos: player.pos + offset,
                    vel: Vec2::ZERO,
                    angle: player.angle,
                    radius: 23.0,
                    speed: 205.0,
                    color: Color::new(1.0, 0.78, 0.15, 1.0),
                    hp: 520.0,
                    max_hp: 520.0,
                    life: 38.0,
                },
                Box::new(AlliedHeavyMech {
                    fire_timer: 0.0,
                    rocket_timer: 0.2,
                }),
            ));
        }

        // Triển khai 2 Drone Hộ Vệ xoay quanh
        for i in 0..2 {
            ctx.spawn_ally(AlliedEntity::new(
                AlliedBody {
                    name: "AEGIS DRONE",
                    pos: player.pos,
                    vel: Vec2::ZERO,
                    angle: 0.0,
                    radius: 9.5,
                    speed: 320.0,
                    color: Color::new(0.0, 0.95, 1.0, 1.0),
                    hp: 130.0,
                    max_hp: 130.0,
                    life: 40.0,
                },
                Box::new(OrbitingCombatDrone {
                    orbit_angle: (i as f32) * std::f32::consts::PI,
                    orbit_radius: 62.0,
                    fire_timer: 0.0,
                }),
            ));
        }

        // Đặt 2 Trụ Súng Ép Xung
        for offset in [vec2(0.0, -48.0), vec2(0.0, 48.0)] {
            ctx.spawn_deployable(Deployable::AutoTurret {
                pos: player.pos + offset,
                angle: player.angle,
                fire_timer: 0.0,
                fire_interval: 0.12,
                life: 32.0,
            });
        }

        // Thả 1 Khoang Tiếp Tế Súng Epic/Legendary
        ctx.spawn_event(Box::new(SupplyPodDropEvent {
            landing_pos: aim_world,
            timer: 0.0,
            max_time: 0.95,
            weapon_id: rand::gen_range(2, 7),
            rarity: WeaponRarity::Epic,
        }));

        ctx.spawn_float_text(
            player.pos - vec2(0.0, 40.0),
            "MECHA LEGION & TURRET MATRIX DEPLOYED!",
            Color::new(1.0, 0.85, 0.1, 1.0),
            22.0,
        );
        true
    }

    fn clone_box(&self) -> Box<dyn SkillBehavior> {
        Box::new(self.clone())
    }
}

pub fn void_stalker_skills() -> Vec<SkillSlot> {
    vec![
        SkillSlot::new(KeyCode::Space, 0.9, Box::new(PhaseBlinkSkill)),
        SkillSlot::new(KeyCode::Q, 6.5, Box::new(VoidTempestSkill)),
        SkillSlot::new(KeyCode::E, 13.0, Box::new(ReinforcementsDropSkill)),
    ]
}

pub fn tech_engineer_skills() -> Vec<SkillSlot> {
    vec![
        SkillSlot::new(KeyCode::Space, 1.1, Box::new(TacticalDashSkill)),
        SkillSlot::new(KeyCode::Q, 7.0, Box::new(TacticalAirstrikeSkill)),
        SkillSlot::new(KeyCode::E, 12.0, Box::new(MechaLegionDropSkill)),
    ]
}
