use macroquad::prelude::*;
use crate::enemy::monsters::*;
use crate::enemy::traits::{EnemyBody, EnemyEntity};

pub struct EnemyFactory;

impl EnemyFactory {
    pub fn create_runner(pos: Vec2, wave: u32) -> EnemyEntity {
        let hp = 50.0 + (wave as f32) * 8.0;
        EnemyEntity::new(
            EnemyBody {
                name: "SWARM RUNNER",
                pos,
                vel: Vec2::ZERO,
                radius: 14.0,
                speed: 195.0,
                color: Color::new(1.0, 0.2, 0.27, 1.0),
                hp,
                max_hp: hp,
                is_boss: false,
                score_reward: 100,
                statuses: Vec::new(),
            },
            Box::new(SwarmRunner),
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
                speed: 165.0,
                color: Color::new(0.0, 0.88, 0.65, 1.0),
                hp,
                max_hp: hp,
                is_boss: false,
                score_reward: 140,
                statuses: Vec::new(),
            },
            Box::new(SplitterSlime { generation: 1 }),
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
                speed: 140.0,
                color: Color::new(0.0, 1.0, 0.4, 1.0),
                hp,
                max_hp: hp,
                is_boss: false,
                score_reward: 130,
                statuses: Vec::new(),
            },
            Box::new(AcidSpitter {
                shoot_timer: rand::gen_range(0.5, 1.8),
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
                speed: 265.0,
                color: Color::new(1.0, 0.66, 0.0, 1.0),
                hp,
                max_hp: hp,
                is_boss: false,
                score_reward: 120,
                statuses: Vec::new(),
            },
            Box::new(KamikazeDrone { pulse: 0.0 }),
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
                statuses: Vec::new(),
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
                speed: 112.0,
                color: Color::new(0.66, 0.33, 0.97, 1.0),
                hp,
                max_hp: hp,
                is_boss: false,
                score_reward: 250,
                statuses: Vec::new(),
            },
            Box::new(BruteCharger {
                charge_timer: 0.0,
                is_charging: false,
                charge_angle: 0.0,
            }),
        )
    }

    pub fn create_boss(pos: Vec2, wave: u32) -> EnemyEntity {
        let hp = 1500.0 + (wave as f32) * 450.0;
        EnemyEntity::new(
            EnemyBody {
                name: "CYBER TITAN BOSS",
                pos,
                vel: Vec2::ZERO,
                radius: 45.0,
                speed: 120.0,
                color: Color::new(1.0, 0.0, 0.35, 1.0),
                hp,
                max_hp: hp,
                is_boss: true,
                score_reward: 5000,
                statuses: Vec::new(),
            },
            Box::new(CyberTitanBoss {
                attack_timer: 0.0,
                phase: 0,
                wave_level: wave,
            }),
        )
    }

    pub fn spawn_for_wave(pos: Vec2, wave: u32, spawn_index: u32) -> EnemyEntity {
        if wave % 5 == 0 && spawn_index == 0 {
            return Self::create_boss(pos, wave);
        }

        let r = rand::gen_range(0.0, 1.0);
        if wave >= 4 && r < 0.16 {
            Self::create_brute(pos, wave)
        } else if wave >= 3 && r < 0.32 {
            Self::create_sniper(pos, wave)
        } else if wave >= 2 && r < 0.50 {
            Self::create_kamikaze(pos, wave)
        } else if wave >= 2 && r < 0.68 {
            Self::create_spitter(pos, wave)
        } else if r < 0.85 {
            Self::create_splitter_slime(pos, wave)
        } else {
            Self::create_runner(pos, wave)
        }
    }
}
