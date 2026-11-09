use macroquad::prelude::*;
use crate::enemy::monsters::*;
use crate::enemy::traits::{BossPart, EnemyBody, EnemyEntity};

pub struct EnemyFactory;

impl EnemyFactory {
    pub fn create_runner(pos: Vec2, wave: u32) -> EnemyEntity {
        let hp = 48.0 + (wave as f32) * 8.0;
        EnemyEntity::new(
            EnemyBody {
                name: "SWARM RUNNER",
                pos,
                vel: Vec2::ZERO,
                radius: 14.0,
                speed: 205.0,
                color: Color::new(1.0, 0.2, 0.27, 1.0),
                hp,
                max_hp: hp,
                is_boss: false,
                score_reward: 100,
                hit_flash: 0.0,
                statuses: Vec::new(),
                parts: Vec::new(),
            },
            Box::new(SwarmRunner {
                facing: 0.0,
                leg_phase: rand::gen_range(0.0, 6.28),
            }),
        )
    }

    pub fn create_splitter_slime(pos: Vec2, wave: u32) -> EnemyEntity {
        let hp = 68.0 + (wave as f32) * 10.0;
        EnemyEntity::new(
            EnemyBody {
                name: "SPLITTER SLIME",
                pos,
                vel: Vec2::ZERO,
                radius: 17.0,
                speed: 170.0,
                color: Color::new(0.0, 0.88, 0.65, 1.0),
                hp,
                max_hp: hp,
                is_boss: false,
                score_reward: 140,
                hit_flash: 0.0,
                statuses: Vec::new(),
                parts: Vec::new(),
            },
            Box::new(SplitterSlime {
                generation: 1,
                rot_angle: rand::gen_range(0.0, 6.28),
            }),
        )
    }

    pub fn create_spitter(pos: Vec2, wave: u32) -> EnemyEntity {
        let hp = 65.0 + (wave as f32) * 10.0;
        EnemyEntity::new(
            EnemyBody {
                name: "ACID SPITTER",
                pos,
                vel: Vec2::ZERO,
                radius: 16.0,
                speed: 145.0,
                color: Color::new(0.0, 1.0, 0.4, 1.0),
                hp,
                max_hp: hp,
                is_boss: false,
                score_reward: 130,
                hit_flash: 0.0,
                statuses: Vec::new(),
                parts: Vec::new(),
            },
            Box::new(AcidSpitter {
                shoot_timer: rand::gen_range(0.5, 1.8),
                facing: 0.0,
            }),
        )
    }

    pub fn create_kamikaze(pos: Vec2, _wave: u32) -> EnemyEntity {
        let hp = 36.0;
        EnemyEntity::new(
            EnemyBody {
                name: "KAMIKAZE DRONE",
                pos,
                vel: Vec2::ZERO,
                radius: 13.0,
                speed: 270.0,
                color: Color::new(1.0, 0.66, 0.0, 1.0),
                hp,
                max_hp: hp,
                is_boss: false,
                score_reward: 120,
                hit_flash: 0.0,
                statuses: Vec::new(),
                parts: Vec::new(),
            },
            Box::new(KamikazeDrone {
                pulse: 0.0,
                facing: 0.0,
            }),
        )
    }

    pub fn create_sniper(pos: Vec2, wave: u32) -> EnemyEntity {
        let hp = 72.0 + (wave as f32) * 12.0;
        EnemyEntity::new(
            EnemyBody {
                name: "SNIPER BOT",
                pos,
                vel: Vec2::ZERO,
                radius: 15.0,
                speed: 135.0,
                color: Color::new(0.95, 0.15, 0.55, 1.0),
                hp,
                max_hp: hp,
                is_boss: false,
                score_reward: 160,
                hit_flash: 0.0,
                statuses: Vec::new(),
                parts: Vec::new(),
            },
            Box::new(SniperBot {
                aim_timer: rand::gen_range(0.0, 1.0),
                target_pos: pos,
            }),
        )
    }

    pub fn create_brute(pos: Vec2, wave: u32) -> EnemyEntity {
        let hp = 210.0 + (wave as f32) * 32.0;
        EnemyEntity::new(
            EnemyBody {
                name: "BRUTE CHARGER",
                pos,
                vel: Vec2::ZERO,
                radius: 24.0,
                speed: 118.0,
                color: Color::new(0.66, 0.33, 0.97, 1.0),
                hp,
                max_hp: hp,
                is_boss: false,
                score_reward: 250,
                hit_flash: 0.0,
                statuses: Vec::new(),
                parts: Vec::new(),
            },
            Box::new(BruteCharger {
                charge_timer: 0.0,
                is_charging: false,
                charge_angle: 0.0,
                facing: 0.0,
            }),
        )
    }

    pub fn create_boss(pos: Vec2, wave: u32) -> EnemyEntity {
        let hp = 1400.0 + (wave as f32) * 400.0;
        let part_hp = 260.0 + (wave as f32) * 75.0;

        let parts = vec![
            BossPart {
                name: "MORTAR CANNON",
                offset: vec2(-62.0, -25.0),
                radius: 20.0,
                hp: part_hp,
                max_hp: part_hp,
                color: Color::new(1.0, 0.45, 0.1, 1.0),
                hit_flash: 0.0,
                behavior: Box::new(MortarArtilleryArmPart { timer: 1.0 }),
            },
            BossPart {
                name: "LASER TURRET",
                offset: vec2(62.0, -25.0),
                radius: 20.0,
                hp: part_hp,
                max_hp: part_hp,
                color: Color::new(1.0, 0.85, 0.1, 1.0),
                hit_flash: 0.0,
                behavior: Box::new(GatlingLaserTurretPart { timer: 0.3 }),
            },
            BossPart {
                name: "SHIELD GENERATOR",
                offset: vec2(0.0, 62.0),
                radius: 18.0,
                hp: part_hp * 0.85,
                max_hp: part_hp * 0.85,
                color: Color::new(0.0, 0.92, 1.0, 1.0),
                hit_flash: 0.0,
                behavior: Box::new(ShieldGeneratorPart),
            },
            BossPart {
                name: "DRONE HATCHERY",
                offset: vec2(0.0, -64.0),
                radius: 19.0,
                hp: part_hp,
                max_hp: part_hp,
                color: Color::new(0.75, 0.25, 1.0, 1.0),
                hit_flash: 0.0,
                behavior: Box::new(DroneHatcheryBayPart {
                    timer: 1.5,
                    wave_level: wave,
                }),
            },
        ];

        EnemyEntity::new(
            EnemyBody {
                name: "COLOSSUS FORTRESS",
                pos,
                vel: Vec2::ZERO,
                radius: 44.0,
                speed: 92.0,
                color: Color::new(1.0, 0.0, 0.35, 1.0),
                hp,
                max_hp: hp,
                is_boss: true,
                score_reward: 5000,
                hit_flash: 0.0,
                statuses: Vec::new(),
                parts,
            },
            Box::new(CyberTitanBoss { attack_timer: 0.0 }),
        )
    }

    pub fn spawn_for_wave(pos: Vec2, wave: u32, spawn_index: u32) -> EnemyEntity {
        // Boss xuất hiện mỗi 3 Wave (hoặc ngay Wave 2 trở đi)
        if wave % 3 == 0 && spawn_index == 0 {
            return Self::create_boss(pos, wave);
        }

        let r = rand::gen_range(0.0, 1.0);
        if r < 0.16 {
            Self::create_brute(pos, wave)
        } else if r < 0.30 {
            Self::create_sniper(pos, wave)
        } else if r < 0.48 {
            Self::create_kamikaze(pos, wave)
        } else if r < 0.65 {
            Self::create_spitter(pos, wave)
        } else if r < 0.82 {
            Self::create_splitter_slime(pos, wave)
        } else {
            Self::create_runner(pos, wave)
        }
    }
}
