use macroquad::prelude::*;
use crate::enemy::monsters::*;
use crate::enemy::traits::{BossPart, EnemyBody, EnemyEntity};

pub struct EnemyFactory;

impl EnemyFactory {
    pub fn create_runner(pos: Vec2, wave: u32) -> EnemyEntity {
        let hp = 55.0 + (wave as f32) * 14.0 + ((wave as f32).powf(1.35)) * 3.5;
        let speed = (210.0 + (wave as f32) * 4.0).min(280.0);
        EnemyEntity::new(
            EnemyBody {
                name: "SWARM RUNNER",
                pos,
                vel: Vec2::ZERO,
                radius: 14.0,
                speed,
                color: Color::new(1.0, 0.2, 0.27, 1.0),
                hp,
                max_hp: hp,
                is_boss: false,
                score_reward: 100 + (wave as u64) * 15,
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
        let hp = 80.0 + (wave as f32) * 16.0 + ((wave as f32).powf(1.35)) * 4.0;
        let speed = (175.0 + (wave as f32) * 3.0).min(225.0);
        EnemyEntity::new(
            EnemyBody {
                name: "SPLITTER SLIME",
                pos,
                vel: Vec2::ZERO,
                radius: 17.0,
                speed,
                color: Color::new(0.0, 0.88, 0.65, 1.0),
                hp,
                max_hp: hp,
                is_boss: false,
                score_reward: 140 + (wave as u64) * 20,
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
        let hp = 75.0 + (wave as f32) * 16.0 + ((wave as f32).powf(1.35)) * 4.0;
        let speed = (155.0 + (wave as f32) * 2.5).min(205.0);
        let max_timer = (1.5 - (wave as f32) * 0.06).max(0.65);
        EnemyEntity::new(
            EnemyBody {
                name: "ACID SPITTER",
                pos,
                vel: Vec2::ZERO,
                radius: 16.0,
                speed,
                color: Color::new(0.0, 1.0, 0.4, 1.0),
                hp,
                max_hp: hp,
                is_boss: false,
                score_reward: 130 + (wave as u64) * 20,
                hit_flash: 0.0,
                statuses: Vec::new(),
                parts: Vec::new(),
            },
            Box::new(AcidSpitter {
                shoot_timer: rand::gen_range(0.4, max_timer),
                facing: 0.0,
            }),
        )
    }

    pub fn create_kamikaze(pos: Vec2, wave: u32) -> EnemyEntity {
        let hp = 45.0 + (wave as f32) * 10.0 + ((wave as f32).powf(1.3)) * 3.0;
        let speed = (270.0 + (wave as f32) * 4.5).min(340.0);
        EnemyEntity::new(
            EnemyBody {
                name: "KAMIKAZE DRONE",
                pos,
                vel: Vec2::ZERO,
                radius: 13.0,
                speed,
                color: Color::new(1.0, 0.66, 0.0, 1.0),
                hp,
                max_hp: hp,
                is_boss: false,
                score_reward: 120 + (wave as u64) * 18,
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
        let hp = 95.0 + (wave as f32) * 18.0 + ((wave as f32).powf(1.35)) * 4.5;
        let speed = (145.0 + (wave as f32) * 2.5).min(195.0);
        let max_aim = (0.95 - (wave as f32) * 0.05).max(0.35);
        EnemyEntity::new(
            EnemyBody {
                name: "SNIPER BOT",
                pos,
                vel: Vec2::ZERO,
                radius: 15.0,
                speed,
                color: Color::new(0.95, 0.15, 0.55, 1.0),
                hp,
                max_hp: hp,
                is_boss: false,
                score_reward: 160 + (wave as u64) * 25,
                hit_flash: 0.0,
                statuses: Vec::new(),
                parts: Vec::new(),
            },
            Box::new(SniperBot {
                aim_timer: rand::gen_range(0.0, max_aim),
                target_pos: pos,
            }),
        )
    }

    pub fn create_brute(pos: Vec2, wave: u32) -> EnemyEntity {
        let hp = 280.0 + (wave as f32) * 55.0 + ((wave as f32).powf(1.45)) * 12.0;
        let speed = (125.0 + (wave as f32) * 3.0).min(175.0);
        EnemyEntity::new(
            EnemyBody {
                name: "BRUTE CHARGER",
                pos,
                vel: Vec2::ZERO,
                radius: 24.0,
                speed,
                color: Color::new(0.66, 0.33, 0.97, 1.0),
                hp,
                max_hp: hp,
                is_boss: false,
                score_reward: 250 + (wave as u64) * 35,
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

    pub fn create_shadow_assassin(pos: Vec2, wave: u32) -> EnemyEntity {
        let hp = 95.0 + (wave as f32) * 18.0 + ((wave as f32).powf(1.35)) * 4.0;
        let speed = (220.0 + (wave as f32) * 4.0).min(285.0);
        EnemyEntity::new(
            EnemyBody {
                name: "SHADOW ASSASSIN",
                pos,
                vel: Vec2::ZERO,
                radius: 15.0,
                speed,
                color: Color::new(0.7, 0.15, 0.95, 1.0),
                hp,
                max_hp: hp,
                is_boss: false,
                score_reward: 180 + (wave as u64) * 30,
                hit_flash: 0.0,
                statuses: Vec::new(),
                parts: Vec::new(),
            },
            Box::new(ShadowAssassin {
                facing: 0.0,
                cloak_timer: rand::gen_range(0.5, 2.0),
                telegraph_timer: 0.0,
                is_teleporting: false,
                slash_timer: 0.0,
            }),
        )
    }

    pub fn create_phalanx_golem(pos: Vec2, wave: u32) -> EnemyEntity {
        let hp = 220.0 + (wave as f32) * 45.0 + ((wave as f32).powf(1.4)) * 9.0;
        let speed = (115.0 + (wave as f32) * 2.5).min(155.0);
        EnemyEntity::new(
            EnemyBody {
                name: "PHALANX GOLEM",
                pos,
                vel: Vec2::ZERO,
                radius: 22.0,
                speed,
                color: Color::new(0.25, 0.7, 0.9, 1.0),
                hp,
                max_hp: hp,
                is_boss: false,
                score_reward: 240 + (wave as u64) * 35,
                hit_flash: 0.0,
                statuses: Vec::new(),
                parts: Vec::new(),
            },
            Box::new(PhalanxGolem {
                facing: 0.0,
                bash_timer: rand::gen_range(0.0, 1.5),
                is_bashing: false,
                bash_duration: 0.0,
            }),
        )
    }

    pub fn create_void_summoner(pos: Vec2, wave: u32) -> EnemyEntity {
        let hp = 140.0 + (wave as f32) * 28.0 + ((wave as f32).powf(1.35)) * 6.0;
        let speed = (130.0 + (wave as f32) * 2.5).min(175.0);
        EnemyEntity::new(
            EnemyBody {
                name: "VOID SUMMONER",
                pos,
                vel: Vec2::ZERO,
                radius: 18.0,
                speed,
                color: Color::new(0.65, 0.1, 0.85, 1.0),
                hp,
                max_hp: hp,
                is_boss: false,
                score_reward: 260 + (wave as u64) * 40,
                hit_flash: 0.0,
                statuses: Vec::new(),
                parts: Vec::new(),
            },
            Box::new(VoidSummoner {
                summon_timer: rand::gen_range(0.5, 2.0),
                orb_timer: 0.0,
                rot: 0.0,
                wave_level: wave,
            }),
        )
    }

    pub fn create_mortar_juggernaut(pos: Vec2, wave: u32) -> EnemyEntity {
        let hp = 260.0 + (wave as f32) * 55.0 + ((wave as f32).powf(1.45)) * 12.0;
        let speed = (85.0 + (wave as f32) * 2.0).min(125.0);
        EnemyEntity::new(
            EnemyBody {
                name: "MORTAR JUGGERNAUT",
                pos,
                vel: Vec2::ZERO,
                radius: 25.0,
                speed,
                color: Color::new(0.85, 0.45, 0.15, 1.0),
                hp,
                max_hp: hp,
                is_boss: false,
                score_reward: 300 + (wave as u64) * 45,
                hit_flash: 0.0,
                statuses: Vec::new(),
                parts: Vec::new(),
            },
            Box::new(MortarJuggernaut {
                facing: 0.0,
                fire_timer: rand::gen_range(0.5, 1.8),
                recoil: 0.0,
            }),
        )
    }

    pub fn create_tesla_overlord(pos: Vec2, wave: u32) -> EnemyEntity {
        let hp = 320.0 + (wave as f32) * 65.0 + ((wave as f32).powf(1.45)) * 14.0;
        let speed = (140.0 + (wave as f32) * 3.0).min(195.0);
        EnemyEntity::new(
            EnemyBody {
                name: "TESLA OVERLORD",
                pos,
                vel: Vec2::ZERO,
                radius: 24.0,
                speed,
                color: Color::new(0.2, 0.85, 1.0, 1.0),
                hp,
                max_hp: hp,
                is_boss: false,
                score_reward: 380 + (wave as u64) * 55,
                hit_flash: 0.0,
                statuses: Vec::new(),
                parts: Vec::new(),
            },
            Box::new(TeslaOverlord {
                facing: 0.0,
                zap_timer: rand::gen_range(0.3, 1.2),
                emp_timer: rand::gen_range(1.0, 3.0),
                coil_angle: 0.0,
            }),
        )
    }

    pub fn create_mutant_biogiant(pos: Vec2, wave: u32) -> EnemyEntity {
        let hp = 420.0 + (wave as f32) * 85.0 + ((wave as f32).powf(1.5)) * 18.0;
        let speed = (95.0 + (wave as f32) * 2.5).min(140.0);
        EnemyEntity::new(
            EnemyBody {
                name: "MUTANT BIO-GIANT",
                pos,
                vel: Vec2::ZERO,
                radius: 30.0,
                speed,
                color: Color::new(0.35, 0.85, 0.2, 1.0),
                hp,
                max_hp: hp,
                is_boss: false,
                score_reward: 480 + (wave as u64) * 70,
                hit_flash: 0.0,
                statuses: Vec::new(),
                parts: Vec::new(),
            },
            Box::new(MutantBioGiant {
                facing: 0.0,
                spine_timer: rand::gen_range(0.5, 2.0),
                leg_phase: 0.0,
            }),
        )
    }

    pub fn create_boss(pos: Vec2, wave: u32) -> EnemyEntity {
        let hp = 1800.0 + (wave as f32) * 750.0 + ((wave as f32).powf(1.6)) * 160.0;
        let part_hp = 340.0 + (wave as f32) * 140.0 + ((wave as f32).powf(1.4)) * 35.0;

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
                speed: 95.0,
                color: Color::new(1.0, 0.0, 0.35, 1.0),
                hp,
                max_hp: hp,
                is_boss: true,
                score_reward: 6000 + (wave as u64) * 600,
                hit_flash: 0.0,
                statuses: Vec::new(),
                parts,
            },
            Box::new(CyberTitanBoss { attack_timer: 0.0 }),
        )
    }

    /// Sinh kẻ địch theo tiến trình độ khó & độ phức tạp tăng dần qua từng Wave
    pub fn spawn_for_wave(pos: Vec2, wave: u32, spawn_index: u32) -> EnemyEntity {
        // Boss xuất hiện mỗi 3 Wave (Wave 3, 6, 9, 12, ...) tại kẻ địch đầu tiên của Wave
        if wave >= 3 && wave % 3 == 0 && spawn_index == 0 {
            return Self::create_boss(pos, wave);
        }

        let mut enemy = match wave {
            1 => {
                // Wave 1: Khởi đầu cận chiến & phân bào
                // 70% Swarm Runner, 30% Splitter Slime
                let r = rand::gen_range(0.0, 1.0);
                if r < 0.30 {
                    Self::create_splitter_slime(pos, wave)
                } else {
                    Self::create_runner(pos, wave)
                }
            }
            2 => {
                // Wave 2: Xuất hiện quái bắn xa Acid Spitter (bắn đạn axit độc làm chậm)
                // 40% Runner, 25% Splitter Slime, 35% Acid Spitter
                let r = rand::gen_range(0.0, 1.0);
                if r < 0.35 {
                    Self::create_spitter(pos, wave)
                } else if r < 0.60 {
                    Self::create_splitter_slime(pos, wave)
                } else {
                    Self::create_runner(pos, wave)
                }
            }
            3 => {
                // Wave 3: Đại Boss Colossus xuất hiện + Drone tự sát Kamikaze bắt đầu bay vào!
                // 25% Runner, 25% Kamikaze Drone, 25% Acid Spitter, 25% Splitter Slime
                let r = rand::gen_range(0.0, 1.0);
                if r < 0.25 {
                    Self::create_kamikaze(pos, wave)
                } else if r < 0.50 {
                    Self::create_spitter(pos, wave)
                } else if r < 0.75 {
                    Self::create_splitter_slime(pos, wave)
                } else {
                    Self::create_runner(pos, wave)
                }
            }
            4 => {
                // Wave 4: Xuất hiện Phalanx Golem (chặn đòn trước mặt) & Brute Charger (húc xe tăng)
                let r = rand::gen_range(0.0, 1.0);
                if r < 0.20 {
                    Self::create_phalanx_golem(pos, wave)
                } else if r < 0.38 {
                    Self::create_brute(pos, wave)
                } else if r < 0.56 {
                    Self::create_kamikaze(pos, wave)
                } else if r < 0.74 {
                    Self::create_spitter(pos, wave)
                } else if r < 0.88 {
                    Self::create_splitter_slime(pos, wave)
                } else {
                    Self::create_runner(pos, wave)
                }
            }
            5 => {
                // Wave 5: Xuất hiện Shadow Assassin (tàng hình dịch chuyển sau lưng) & Sniper Bot (bắn tỉa laze)
                let r = rand::gen_range(0.0, 1.0);
                if r < 0.18 {
                    Self::create_shadow_assassin(pos, wave)
                } else if r < 0.34 {
                    Self::create_sniper(pos, wave)
                } else if r < 0.50 {
                    Self::create_phalanx_golem(pos, wave)
                } else if r < 0.64 {
                    Self::create_brute(pos, wave)
                } else if r < 0.78 {
                    Self::create_kamikaze(pos, wave)
                } else if r < 0.90 {
                    Self::create_spitter(pos, wave)
                } else {
                    Self::create_runner(pos, wave)
                }
            }
            6 => {
                // Wave 6: Đại Boss hồi quy + Xuất hiện Void Summoner (triệu hồi ký sinh trùng) & Mortar Juggernaut (pháo kích vòng đỏ)
                let r = rand::gen_range(0.0, 1.0);
                if r < 0.16 {
                    Self::create_mortar_juggernaut(pos, wave)
                } else if r < 0.32 {
                    Self::create_void_summoner(pos, wave)
                } else if r < 0.47 {
                    Self::create_shadow_assassin(pos, wave)
                } else if r < 0.61 {
                    Self::create_phalanx_golem(pos, wave)
                } else if r < 0.75 {
                    Self::create_sniper(pos, wave)
                } else if r < 0.88 {
                    Self::create_brute(pos, wave)
                } else {
                    Self::create_kamikaze(pos, wave)
                }
            }
            7 => {
                // Wave 7: Xuất hiện Tesla Overlord (phóng sấm sét chuỗi & xung chấn EMP)
                let r = rand::gen_range(0.0, 1.0);
                if r < 0.16 {
                    Self::create_tesla_overlord(pos, wave)
                } else if r < 0.31 {
                    Self::create_mortar_juggernaut(pos, wave)
                } else if r < 0.46 {
                    Self::create_void_summoner(pos, wave)
                } else if r < 0.60 {
                    Self::create_shadow_assassin(pos, wave)
                } else if r < 0.74 {
                    Self::create_phalanx_golem(pos, wave)
                } else if r < 0.87 {
                    Self::create_sniper(pos, wave)
                } else {
                    Self::create_brute(pos, wave)
                }
            }
            _ => {
                // Wave 8+: Mở khóa toàn diện 12 chủng loại quái — Xuất hiện Mutant Bio-Giant (khổng lồ bão gai độc & bãi axit)
                let r = rand::gen_range(0.0, 1.0);
                if r < 0.13 {
                    Self::create_mutant_biogiant(pos, wave)
                } else if r < 0.25 {
                    Self::create_tesla_overlord(pos, wave)
                } else if r < 0.37 {
                    Self::create_mortar_juggernaut(pos, wave)
                } else if r < 0.49 {
                    Self::create_void_summoner(pos, wave)
                } else if r < 0.61 {
                    Self::create_shadow_assassin(pos, wave)
                } else if r < 0.72 {
                    Self::create_phalanx_golem(pos, wave)
                } else if r < 0.82 {
                    Self::create_sniper(pos, wave)
                } else if r < 0.90 {
                    Self::create_brute(pos, wave)
                } else if r < 0.95 {
                    Self::create_kamikaze(pos, wave)
                } else {
                    Self::create_spitter(pos, wave)
                }
            }
        };

        // Từ Wave 4 trở đi, xuất hiện biến thể Quái Tinh Anh (Elite Champion) với tỷ lệ tăng dần theo Wave (tối đa 35%)!
        if wave >= 4 {
            let elite_chance = (((wave - 3) as f32) * 0.045).min(0.35);
            if rand::gen_range(0.0, 1.0) < elite_chance {
                enemy.body.hp *= 1.85;
                enemy.body.max_hp = enemy.body.hp;
                enemy.body.radius *= 1.25;
                enemy.body.speed = (enemy.body.speed * 1.10).min(295.0);
                enemy.body.score_reward *= 2;
                // Ánh sắc hào quang Tinh Anh (tím / vàng rực sáng)
                enemy.body.color = Color::new(
                    (enemy.body.color.r * 0.8 + 0.35).min(1.0),
                    (enemy.body.color.g * 0.7).min(1.0),
                    (enemy.body.color.b * 0.9 + 0.45).min(1.0),
                    1.0,
                );
            }
        }

        enemy
    }
}
