use macroquad::prelude::*;
use std::f32::consts::PI;

use crate::ally::{
    AlliedBody, AlliedEntity, AlliedHeavyMech, AlliedMarine, OrbitingCombatDrone,
    OrbitingPlasmaSphere,
};
use crate::character::classes::{create_class_by_id, CyberMarine};
use crate::character::player::Player;
use crate::core::audio::SoundBank;
use crate::core::context::{CombatContext, WorldCommand};
use crate::core::events::{
    BattlefieldEvent, BeamArcVisualEvent, MeteorWarningStrikeEvent, ShockwaveRingEvent,
    SingularityBlackHoleEvent, SupplyPodDropEvent,
};
use crate::core::i18n::{draw_text, draw_text_centered, init_i18n_font, set_current_language, tr};
use crate::core::props::{ExplosiveBarrel, WeaponDrop, WeaponRarity};
use crate::core::structures::{
    EnergyBarricadeStructure, NanoRepairPylonStructure, RadarOutpostStructure, StructureBody,
    TacticalStructureEntity,
};
use crate::core::types::*;
use crate::enemy::spawner::EnemyFactory;
use crate::enemy::traits::EnemyEntity;
use crate::skill::deployables::Deployable;
use crate::skill::trees::{has_available_unique_upgrades, roll_upgrade_choices};
use crate::ui::hud::{draw_hud, draw_minimap};
use crate::ui::menus::*;
use crate::weapon::arsenal::{evolved_weapon_name_by_id, weapon_form_name, weapon_name_by_id};
use crate::weapon::projectile::Projectile;

pub struct GameWorld {
    pub state: GameState,
    pub settings: GameSettings,
    pub settings_open: bool,
    pub selected_class_id: usize,
    pub selected_weapon_id: usize,
    pub sound_bank: SoundBank,
    pub player: Player,
    pub player_projectiles: Vec<Projectile>,
    pub enemy_projectiles: Vec<Projectile>,
    pub enemies: Vec<EnemyEntity>,
    pub allies: Vec<AlliedEntity>,
    pub structures: Vec<TacticalStructureEntity>,
    pub deployables: Vec<Deployable>,
    pub events: Vec<Box<dyn BattlefieldEvent>>,
    pub weapon_drops: Vec<WeaponDrop>,
    pub barrels: Vec<ExplosiveBarrel>,
    pub particles: Vec<Particle>,
    pub decals: Vec<Decal>,
    pub casings: Vec<Casing>,
    pub pickups: Vec<Pickup>,
    pub exp_orbs: Vec<ExpOrb>,
    pub float_texts: Vec<FloatText>,
    pub camera_pos: Vec2,
    pub screen_shake: f32,
    pub wave: u32,
    pub score: u64,
    pub combo: u32,
    pub combo_timer: f32,
    pub total_kills: u32,
    pub shots_fired: u64,
    pub shots_hit: u64,
    pub enemies_to_spawn: u32,
    pub enemies_spawned: u32,
    pub spawn_timer: f32,
    pub current_upgrade_choices: Vec<UpgradeOption>,
    pub locked_target_pos: Option<Vec2>,
    pub tech_level: u32,
    pub tech_exp: f32,
    pub next_tech_exp: f32,
    pub pending_upgrades: u32,
    pub rerolls_left: u32,
    pub end_of_wave_upgrade: bool,
    pub ult_periodic_timer: f32,
    pub was_dashing: bool,
}

impl GameWorld {
    pub async fn new() -> Self {
        init_i18n_font();
        let start_pos = vec2(WORLD_WIDTH / 2.0, WORLD_HEIGHT / 2.0);
        let mut player = Player::new(Box::new(CyberMarine), start_pos);
        player.lock_single_weapon(0);
        let sound_bank = SoundBank::new().await;
        let settings = GameSettings::load_or_default();
        set_current_language(settings.language);
        let mut world = Self {
            state: GameState::Start,
            settings,
            settings_open: false,
            selected_class_id: 0,
            selected_weapon_id: 0,
            sound_bank,
            player,
            player_projectiles: Vec::new(),
            enemy_projectiles: Vec::new(),
            enemies: Vec::new(),
            allies: Vec::new(),
            structures: Vec::new(),
            deployables: Vec::new(),
            events: Vec::new(),
            weapon_drops: Vec::new(),
            barrels: Vec::new(),
            particles: Vec::new(),
            decals: Vec::new(),
            casings: Vec::new(),
            pickups: Vec::new(),
            exp_orbs: Vec::new(),
            float_texts: Vec::new(),
            camera_pos: start_pos,
            screen_shake: 0.0,
            wave: 1,
            score: 0,
            combo: 1,
            combo_timer: 0.0,
            total_kills: 0,
            shots_fired: 0,
            shots_hit: 0,
            enemies_to_spawn: 50,
            enemies_spawned: 0,
            spawn_timer: 0.0,
            current_upgrade_choices: Vec::new(),
            locked_target_pos: None,
            tech_level: 1,
            tech_exp: 0.0,
            next_tech_exp: 180.0,
            pending_upgrades: 0,
            rerolls_left: 3,
            end_of_wave_upgrade: false,
            ult_periodic_timer: 0.0,
            was_dashing: false,
        };
        world.populate_battlefield_props();
        world
    }

    /// Cộng độ rung màn hình đã được giảm chấn mượt theo `settings.screen_shake_intensity`
    pub fn add_screen_shake(&mut self, raw_amount: f32) {
        let scale = self.settings.screen_shake_intensity;
        if scale <= 0.001 {
            self.screen_shake = 0.0;
            return;
        }
        let max_cap = 7.5 * scale;
        self.screen_shake = (self.screen_shake + raw_amount * 0.32 * scale).min(max_cap);
    }

    pub fn populate_battlefield_props(&mut self) {
        let center = self.player.body.pos;
        // 1. Từ Wave 2 trở đi mới thả 1 Khoang Tiếp Tế nâng cấp cho đúng khẩu súng duy nhất đã chọn
        if self.wave >= 2 && self.weapon_drops.is_empty() {
            self.events.push(Box::new(SupplyPodDropEvent {
                landing_pos: center + vec2(0.0, -95.0),
                timer: 0.0,
                max_time: 0.9,
                weapon_id: self.selected_weapon_id,
                rarity: WeaponRarity::Rare,
            }));
        }

        // 2. Triển khai sẵn 2 Lính Thủy Quân Lục Chiến Đồng Minh hộ tống
        for (i, offset) in [vec2(-55.0, 50.0), vec2(55.0, 50.0)].iter().enumerate() {
            self.allies.push(AlliedEntity::new(
                AlliedBody {
                    name: "ALLIED MARINE",
                    pos: center + *offset,
                    vel: Vec2::ZERO,
                    angle: 0.0,
                    radius: 13.0,
                    speed: 285.0,
                    color: Color::new(0.15, 0.85, 1.0, 1.0),
                    hp: 160.0,
                    max_hp: 160.0,
                    life: 45.0,
                },
                Box::new(AlliedMarine {
                    formation_offset: *offset,
                    fire_timer: (i as f32) * 0.08,
                }),
            ));
        }

        // 3. Rải 16 Thùng Nhiên Liệu Nổ Dây Chuyền (Explosive Barrels) quanh chiến trường
        for _ in 0..16 {
            let bx = rand::gen_range(250.0, WORLD_WIDTH - 250.0);
            let by = rand::gen_range(250.0, WORLD_HEIGHT - 250.0);
            if (vec2(bx, by) - center).length() > 160.0 {
                self.barrels.push(ExplosiveBarrel::new(vec2(bx, by)));
            }
        }

        // 4. Khởi tạo các Công Trình & Cứ Điểm Chiến Thuật (BattlefieldStructure)
        if self.structures.is_empty() {
            // 2 Trạm Radar Chiếm Đóng ở hai cánh chiến trường
            for offset in [vec2(-460.0, -320.0), vec2(460.0, 320.0)] {
                self.structures.push(TacticalStructureEntity::new(
                    StructureBody {
                        name: "RADAR OUTPOST",
                        pos: center + offset,
                        radius: 26.0,
                        hp: 650.0,
                        max_hp: 650.0,
                        color: Color::new(0.0, 0.94, 1.0, 1.0),
                    },
                    Box::new(RadarOutpostStructure {
                        capture_radius: 130.0,
                        progress: 0.0,
                        captured: false,
                        strike_timer: 0.0,
                        radar_angle: 0.0,
                    }),
                ));
            }

            // 4 Lô Cốt Chắn Đạn Năng Lượng quanh khu vực trung tâm
            for offset in [
                vec2(-230.0, -180.0),
                vec2(230.0, -180.0),
                vec2(-230.0, 180.0),
                vec2(230.0, 180.0),
            ] {
                self.structures.push(TacticalStructureEntity::new(
                    StructureBody {
                        name: "ENERGY BUNKER",
                        pos: center + offset,
                        radius: 24.0,
                        hp: 500.0,
                        max_hp: 500.0,
                        color: Color::new(0.15, 0.85, 1.0, 1.0),
                    },
                    Box::new(EnergyBarricadeStructure { pulse: 0.0 }),
                ));
            }

            // 2 Trụ Hồi Phục Nano (Nano Repair Pylon)
            for offset in [vec2(0.0, -340.0), vec2(0.0, 340.0)] {
                self.structures.push(TacticalStructureEntity::new(
                    StructureBody {
                        name: "NANO REPAIR PYLON",
                        pos: center + offset,
                        radius: 20.0,
                        hp: 450.0,
                        max_hp: 450.0,
                        color: Color::new(0.1, 1.0, 0.55, 1.0),
                    },
                    Box::new(NanoRepairPylonStructure {
                        aura_radius: 120.0,
                        pulse: 0.0,
                    }),
                ));
            }
        }
    }

    pub fn reset(&mut self) {
        let start_pos = vec2(WORLD_WIDTH / 2.0, WORLD_HEIGHT / 2.0);
        self.player.reset(start_pos);
        self.player.lock_single_weapon(self.selected_weapon_id);
        self.player_projectiles.clear();
        self.enemy_projectiles.clear();
        self.enemies.clear();
        self.allies.clear();
        self.structures.clear();
        self.deployables.clear();
        self.events.clear();
        self.weapon_drops.clear();
        self.barrels.clear();
        self.particles.clear();
        self.decals.clear();
        self.casings.clear();
        self.pickups.clear();
        self.exp_orbs.clear();
        self.float_texts.clear();
        self.camera_pos = start_pos;
        self.screen_shake = 0.0;
        self.wave = 1;
        self.score = 0;
        self.combo = 1;
        self.combo_timer = 0.0;
        self.total_kills = 0;
        self.shots_fired = 0;
        self.shots_hit = 0;
        let base_count = 50.0 * self.settings.swarm_scale;
        self.enemies_to_spawn = (base_count.round() as u32).min(self.settings.max_ram_spawn_limit);
        self.enemies_spawned = 0;
        self.spawn_timer = 0.0;
        self.tech_level = 1;
        self.tech_exp = 0.0;
        self.next_tech_exp = 180.0;
        self.pending_upgrades = 0;
        self.rerolls_left = 3;
        self.end_of_wave_upgrade = false;
        self.ult_periodic_timer = 0.0;
        self.was_dashing = false;
        self.populate_battlefield_props();
        if self.settings.instant_full_spawn {
            self.spawn_entire_wave_instantly();
        }
        self.state = GameState::Playing;
    }

    /// Sinh toàn bộ quái của Wave trong 1 lần duy nhất (tối đa theo mức RAM, mặc định trần 10.000 con)
    pub fn spawn_entire_wave_instantly(&mut self) {
        let total_to_spawn = self.enemies_to_spawn;
        let p_pos = self.player.body.pos;
        // Cự ly sinh tối thiểu 850px, tối đa 2600px
        let min_safe_dist = 850.0;
        let max_dist = (min_safe_dist + 350.0 + (total_to_spawn as f32) * 0.25).min(2600.0);

        self.enemies.clear();
        self.enemies.reserve(total_to_spawn as usize);

        // Sinh quái theo 3 cụm hướng chiến thuật lớn (Sectors), chừa khoảng trống lớn (~120 độ) để người chơi có đường thoát hiểm
        let sector_count = 3;
        let start_angle = rand::gen_range(0.0, PI * 2.0);
        let sector_centers: [f32; 3] = [
            start_angle,
            start_angle + 1.85,
            start_angle + 3.45,
        ];

        for i in 0..total_to_spawn {
            let sector_idx = (i as usize) % sector_count;
            let center_a = sector_centers[sector_idx];
            // Phân bổ góc mở ±0.52 rad cho mỗi cụm để không bao vây kín 360 độ quanh người chơi
            let angle = center_a + rand::gen_range(-0.52, 0.52);
            let ring_step = if total_to_spawn > 1 {
                (i as f32) / ((total_to_spawn - 1) as f32)
            } else {
                0.0
            };
            let dist = min_safe_dist + ring_step * (max_dist - min_safe_dist) + rand::gen_range(-30.0, 30.0);

            let ex = (p_pos.x + angle.cos() * dist).clamp(50.0, WORLD_WIDTH - 50.0);
            let ey = (p_pos.y + angle.sin() * dist).clamp(50.0, WORLD_HEIGHT - 50.0);

            let enemy = EnemyFactory::spawn_for_wave(vec2(ex, ey), self.wave, i);
            self.enemies.push(enemy);
        }
        self.enemies_spawned = total_to_spawn;
    }

    /// Kích hoạt Khiên Bất Tử 2.5s & Đẩy lùi bảo vệ khi vừa nâng cấp xong
    pub fn trigger_upgrade_safe_pulse(&mut self) {
        let p_pos = self.player.body.pos;
        // Đẩy lùi các quái vật ở quá gần (< 360px) ra cự ly an toàn 480px và gây choáng
        for e in self.enemies.iter_mut() {
            let to_enemy = e.body.pos - p_pos;
            let dist = to_enemy.length();
            if dist < 360.0 {
                let dir = if dist > 0.001 {
                    to_enemy / dist
                } else {
                    let a = rand::gen_range(0.0, PI * 2.0);
                    vec2(a.cos(), a.sin())
                };
                e.body.pos = p_pos + dir * 480.0;
                e.body.vel = dir * 550.0;
                e.body.statuses.push(StatusEffect::Stun { duration: 0.8 });
            }
        }

        // KHIÊN BẤT TỬ 2.5s: Người chơi hoàn toàn không bị đơ/khóa phím, tự do chạy nhảy bắn súng 100%!
        self.player.body.invulnerable_timer = 2.5;

        // Hiệu ứng Sóng Xung Kích tỏa ra
        self.events.push(Box::new(ShockwaveRingEvent {
            center: p_pos,
            current_radius: 20.0,
            max_radius: 520.0,
            speed: 1200.0,
            color: Color::new(0.1, 0.95, 1.0, 0.95),
            thickness: 8.0,
        }));

        self.float_texts.push(FloatText {
            pos: p_pos - vec2(0.0, 42.0),
            text: tr("<< KHIÊN BẢO HỘ 2.5S - BẤT TỬ >>", "<< INVULNERABILITY SHIELD 2.5S >>").to_string(),
            color: Color::new(0.2, 0.95, 1.0, 1.0),
            size: 20.0,
            life: 1.6,
        });

        self.sound_bank.play_powerup(self.settings.master_volume);
    }

    /// Triệu hồi các Quả Cầu Plasma xoay quanh người chơi bảo hộ và thiêu rụi quái vật tiếp xúc
    pub fn spawn_orbital_plasma_spheres(&mut self, count: usize, radius: f32) {
        let p_pos = self.player.body.pos;
        for i in 0..count {
            let angle = (i as f32) * (PI * 2.0 / (count as f32));
            self.allies.push(AlliedEntity::new(
                AlliedBody {
                    name: "PLASMA SPHERE",
                    pos: p_pos + vec2(angle.cos(), angle.sin()) * radius,
                    vel: Vec2::ZERO,
                    angle: 0.0,
                    radius: 12.0,
                    speed: 0.0,
                    color: Color::new(0.3, 0.9, 1.0, 1.0),
                    hp: 9999.0,
                    max_hp: 9999.0,
                    life: 99999.0,
                },
                Box::new(OrbitingPlasmaSphere {
                    orbit_angle: angle,
                    orbit_radius: radius,
                    orbit_speed: 4.5,
                    pulse_timer: 0.0,
                }),
            ));
        }
    }

    /// Tự động cộng chỉ số toàn năng (Omni-Stat Bonus) khi đã cày xong toàn bộ buff độc nhất
    pub fn apply_omni_stat_upgrade(&mut self) {
        let p_pos = self.player.body.pos;
        self.player.body.modifiers.dmg_multi += 0.08;
        self.player.body.modifiers.fire_rate_multi *= 1.05;
        self.player.body.max_hp += 15.0;
        self.player.body.hp = (self.player.body.hp + 35.0).min(self.player.body.max_hp);
        self.player.body.max_shield += 15.0;
        self.player.body.shield = (self.player.body.shield + 35.0).min(self.player.body.max_shield);
        self.player.body.speed *= 1.03;
        self.player.body.modifiers.magnet_radius_multi += 0.12;
        self.player.body.modifiers.bullet_size_multi *= 1.03;

        self.sound_bank.play_powerup(self.settings.master_volume);

        self.float_texts.push(FloatText {
            pos: p_pos - vec2(0.0, 48.0),
            text: tr(
                "★ CƯỜNG HÓA TOÀN NĂNG (+TẤT CẢ CHỈ SỐ)! ★",
                "★ OMNI-STAT LEVEL UP (+ALL STATS BOOSTED)! ★",
            )
            .to_string(),
            color: Color::new(1.0, 0.85, 0.15, 1.0),
            size: 22.0,
            life: 1.8,
        });

        for _ in 0..16 {
            let a = rand::gen_range(0.0, PI * 2.0);
            let spd = rand::gen_range(120.0, 320.0);
            self.particles.push(Particle {
                pos: p_pos,
                vel: vec2(a.cos(), a.sin()) * spd,
                color: Color::new(1.0, 0.85, 0.2, 1.0),
                radius: rand::gen_range(3.0, 6.0),
                life: 0.8,
                max_life: 0.8,
            });
        }
    }

    pub fn make_context(&self) -> CombatContext {
        CombatContext::new(
            self.player.body.pos,
            self.player.body.radius,
            self.player.body.is_dashing,
            Vec::new(),
        )
    }

    pub fn flush_commands(&mut self, commands: Vec<WorldCommand>) {
        let mut follow_up_commands: Vec<WorldCommand> = Vec::new();

        for cmd in commands {
            match cmd {
                WorldCommand::SpawnPlayerProjectile(proj) => {
                    self.player_projectiles.push(proj);
                }
                WorldCommand::SpawnEnemyProjectile(proj) => {
                    self.enemy_projectiles.push(proj);
                }
                WorldCommand::SpawnEnemy(enemy) => {
                    self.enemies.push(enemy);
                }
                WorldCommand::SpawnAlly(ally) => {
                    self.allies.push(ally);
                }
                WorldCommand::SpawnDeployable(dep) => {
                    self.deployables.push(dep);
                }
                WorldCommand::SpawnEvent(ev) => {
                    self.events.push(ev);
                }
                WorldCommand::SpawnWeaponDrop(mut drop) => {
                    // Mỗi trận chỉ dùng 1 vũ khí duy nhất: mọi Lõi Vũ Khí rơi ra đều dùng để Cường Hóa / Tiến Hóa vũ khí đã chọn!
                    drop.weapon_id = self.selected_weapon_id;
                    self.weapon_drops.push(drop);
                }
                WorldCommand::TriggerExplosion {
                    pos,
                    radius,
                    damage,
                } => {
                    self.sound_bank.play_explosion(self.settings.master_volume);
                    self.add_screen_shake(7.0);
                    self.decals.push(Decal {
                        pos,
                        radius: radius * 0.72,
                        color: Color::new(0.08, 0.08, 0.1, 0.72),
                        life: 12.0,
                        is_damage_zone: false,
                    });

                    let count = (26.0 * self.settings.particle_density).round() as usize;
                    for _ in 0..count {
                        let a = rand::gen_range(0.0, PI * 2.0);
                        let spd = rand::gen_range(60.0, 340.0);
                        self.particles.push(Particle {
                            pos,
                            vel: vec2(a.cos(), a.sin()) * spd,
                            color: if rand::gen_range(0, 2) == 0 {
                                Color::new(1.0, 0.25, 0.15, 1.0)
                            } else {
                                Color::new(1.0, 0.75, 0.0, 1.0)
                            },
                            radius: rand::gen_range(3.0, 7.0),
                            life: 0.45,
                            max_life: 0.45,
                        });
                    }

                    // Kích hoạt nổ dây chuyền các Thùng Nhiên Liệu (Explosive Barrels) gần đó!
                    for barrel in self.barrels.iter_mut() {
                        if barrel.fuse_timer.is_none()
                            && (barrel.pos - pos).length() < radius + barrel.radius
                        {
                            barrel.fuse_timer = Some(0.12);
                        }
                    }

                    let mut sub_ctx = CombatContext::new(
                        self.player.body.pos,
                        self.player.body.radius,
                        self.player.body.is_dashing,
                        Vec::new(),
                    );
                    let max_box = radius + 60.0;
                    for e in self.enemies.iter_mut() {
                        if (e.body.pos.x - pos.x).abs() > max_box || (e.body.pos.y - pos.y).abs() > max_box {
                            continue;
                        }
                        let dist = (e.body.pos - pos).length();
                        if dist < radius + e.body.radius + 50.0 {
                            let falloff = (1.0 - (dist / (radius + e.body.radius + 50.0)) * 0.5)
                                .clamp(0.2, 1.0);
                            let total_dmg = damage * falloff;
                            let kb = (e.body.pos - pos).normalize_or_zero() * 520.0;
                            let dealt = e.take_damage_at(pos, total_dmg, kb, None, &mut sub_ctx);
                            if self.settings.show_damage_numbers && self.float_texts.len() < 120 {
                                self.float_texts.push(FloatText {
                                    pos: e.body.pos,
                                    text: format!("{:.0}", dealt),
                                    color: Color::new(1.0, 0.35, 0.15, 1.0),
                                    size: 20.0,
                                    life: 0.75,
                                });
                            }
                        }
                    }
                    follow_up_commands.append(&mut sub_ctx.commands);
                }
                WorldCommand::ApplyVortexPull {
                    center,
                    radius,
                    pull_step,
                    damage_step,
                } => {
                    let max_box = radius + 30.0;
                    for e in self.enemies.iter_mut() {
                        if (e.body.pos.x - center.x).abs() > max_box || (e.body.pos.y - center.y).abs() > max_box {
                            continue;
                        }
                        let to_center = center - e.body.pos;
                        let dist = to_center.length();
                        if dist < radius + e.body.radius {
                            if pull_step > 0.0 && dist > 14.0 {
                                e.body.pos += to_center.normalize_or_zero() * pull_step;
                            }
                            e.body.hp -= damage_step;
                        }
                    }
                }
                WorldCommand::RailgunStrike {
                    start,
                    end,
                    width,
                    damage,
                } => {
                    self.add_screen_shake(8.0);
                    let seg = end - start;
                    let seg_len_sq = seg.length_squared().max(1.0);

                    let mut sub_ctx = CombatContext::new(
                        self.player.body.pos,
                        self.player.body.radius,
                        self.player.body.is_dashing,
                        Vec::new(),
                    );

                    let min_x = start.x.min(end.x) - width - 35.0;
                    let max_x = start.x.max(end.x) + width + 35.0;
                    let min_y = start.y.min(end.y) - width - 35.0;
                    let max_y = start.y.max(end.y) + width + 35.0;

                    for e in self.enemies.iter_mut() {
                        if e.body.pos.x < min_x || e.body.pos.x > max_x || e.body.pos.y < min_y || e.body.pos.y > max_y {
                            continue;
                        }
                        let t = ((e.body.pos - start).dot(seg) / seg_len_sq).clamp(0.0, 1.0);
                        let closest = start + seg * t;
                        let dist = (e.body.pos - closest).length();
                        if dist < width + e.body.radius {
                            let kb = seg.normalize_or_zero() * 350.0;
                            let dealt = e.take_damage_at(closest, damage, kb, None, &mut sub_ctx);
                            if self.settings.show_damage_numbers && self.float_texts.len() < 120 {
                                self.float_texts.push(FloatText {
                                    pos: e.body.pos,
                                    text: format!("{:.0} RAIL!", dealt),
                                    color: Color::new(0.0, 1.0, 0.9, 1.0),
                                    size: 22.0,
                                    life: 0.85,
                                });
                            }
                        }
                    }
                    follow_up_commands.append(&mut sub_ctx.commands);

                    // Hiệu ứng tia Railgun và các vòng sóng xung kích dọc đường đạn
                    self.events.push(Box::new(BeamArcVisualEvent {
                        points: vec![start, end],
                        color: Color::new(0.0, 0.95, 1.0, 1.0),
                        thickness: 6.5,
                        life: 0.28,
                        max_life: 0.28,
                    }));
                    for step in (120..1200).step_by(240) {
                        let ring_pos = start + seg.normalize_or_zero() * (step as f32);
                        self.events.push(Box::new(ShockwaveRingEvent {
                            center: ring_pos,
                            current_radius: 6.0,
                            max_radius: 65.0,
                            speed: 380.0,
                            color: Color::new(0.0, 1.0, 0.9, 0.8),
                            thickness: 3.0,
                        }));
                    }
                }
                WorldCommand::ChainLightning {
                    origin,
                    aim_dir,
                    max_jumps,
                    jump_range,
                    damage,
                } => {
                    let mut chain_points = vec![origin];
                    let mut hit_indices: Vec<usize> = Vec::new();
                    let mut current_pos = origin + aim_dir * 60.0;

                    let mut sub_ctx = CombatContext::new(
                        self.player.body.pos,
                        self.player.body.radius,
                        self.player.body.is_dashing,
                        Vec::new(),
                    );

                    for _ in 0..max_jumps {
                        let mut best_idx = None;
                        let mut best_dist = jump_range;
                        for (idx, e) in self.enemies.iter().enumerate() {
                            if hit_indices.contains(&idx) || e.body.hp <= 0.0 {
                                continue;
                            }
                            let d = (e.body.pos - current_pos).length();
                            if d < best_dist {
                                best_dist = d;
                                best_idx = Some(idx);
                            }
                        }

                        if let Some(idx) = best_idx {
                            hit_indices.push(idx);
                            let target_pos = self.enemies[idx].body.pos;
                            chain_points.push(target_pos);
                            let dealt = self.enemies[idx].take_damage_at(
                                target_pos,
                                damage,
                                Vec2::ZERO,
                                Some(StatusEffect::Stun { duration: 0.18 }),
                                &mut sub_ctx,
                            );
                            if self.settings.show_damage_numbers {
                                self.float_texts.push(FloatText {
                                    pos: target_pos,
                                    text: format!("{:.0} ZAP", dealt),
                                    color: Color::new(0.3, 0.95, 1.0, 1.0),
                                    size: 16.0,
                                    life: 0.65,
                                });
                            }
                            current_pos = target_pos;
                        } else {
                            break;
                        }
                    }

                    if chain_points.len() == 1 {
                        chain_points.push(origin + aim_dir * 240.0);
                    }
                    follow_up_commands.append(&mut sub_ctx.commands);

                    self.events.push(Box::new(BeamArcVisualEvent {
                        points: chain_points,
                        color: Color::new(0.25, 0.9, 1.0, 1.0),
                        thickness: 3.5,
                        life: 0.18,
                        max_life: 0.18,
                    }));
                }
                WorldCommand::DamagePlayer(dmg) => {
                    let shake = self.player.take_damage(dmg);
                    self.add_screen_shake(shake);
                    if self.player.body.modifiers.shield_retaliation {
                        let p_pos = self.player.body.pos;
                        follow_up_commands.push(WorldCommand::TriggerExplosion {
                            pos: p_pos,
                            radius: 135.0,
                            damage: 90.0,
                        });
                        follow_up_commands.push(WorldCommand::ChainLightning {
                            origin: p_pos,
                            aim_dir: vec2(1.0, 0.0),
                            max_jumps: 4,
                            jump_range: 300.0,
                            damage: 75.0,
                        });
                    }
                }
                WorldCommand::AddScreenShake(s) => {
                    self.add_screen_shake(s);
                }
                WorldCommand::SpawnParticle(p) => {
                    if rand::gen_range(0.0, 1.0) <= self.settings.particle_density {
                        self.particles.push(p);
                    }
                }
                WorldCommand::SpawnDecal(d) => {
                    self.decals.push(d);
                }
                WorldCommand::SpawnFloatText(f) => {
                    self.float_texts.push(f);
                }
            }
        }

        if !follow_up_commands.is_empty() {
            self.flush_commands(follow_up_commands);
        }
    }

    pub fn step(&mut self, dt: f32) {
        set_current_language(self.settings.language);

        // Phím tắt [L]: Chuyển đổi nhanh giữa Tiếng Việt (Có dấu) và Tiếng Anh ở bất kỳ đâu!
        if is_key_pressed(KeyCode::L) {
            self.settings.language = self.settings.language.toggle();
            set_current_language(self.settings.language);
            self.settings.save_to_disk();
            self.sound_bank.play_powerup(self.settings.master_volume);
            if self.state == GameState::Playing {
                self.float_texts.push(FloatText {
                    pos: self.player.body.pos - vec2(0.0, 44.0),
                    text: format!(
                        "{}: {}",
                        tr("NGÔN NGỮ", "LANGUAGE"),
                        self.settings.language.label()
                    ),
                    color: Color::new(1.0, 0.85, 0.15, 1.0),
                    size: 21.0,
                    life: 1.4,
                });
            }
        }

        if self.settings_open {
            if self.state == GameState::Paused {
                let _ = self.render_world();
            } else {
                clear_background(Color::new(0.03, 0.04, 0.07, 1.0));
            }
            if draw_settings_modal(&mut self.settings) {
                self.settings_open = false;
            }
            return;
        }

        if is_key_pressed(KeyCode::Escape) || is_key_pressed(KeyCode::P) {
            if self.state == GameState::Playing {
                self.state = GameState::Paused;
            } else if self.state == GameState::Paused {
                self.state = GameState::Playing;
            }
        } else if is_key_pressed(KeyCode::O) && self.state == GameState::Playing {
            self.state = GameState::Paused;
            self.settings_open = true;
            return;
        }

        match self.state {
            GameState::Start => {
                match draw_start_menu(
                    self.selected_class_id,
                    self.selected_weapon_id,
                    self.player.class.name(),
                    self.player.class.description(),
                    &self.settings,
                ) {
                    StartMenuAction::SelectClass(cid) => {
                        self.selected_class_id = cid;
                        let start_pos = vec2(WORLD_WIDTH / 2.0, WORLD_HEIGHT / 2.0);
                        self.player.set_class(create_class_by_id(cid), start_pos);
                        self.player.lock_single_weapon(self.selected_weapon_id);
                        self.sound_bank.play_powerup(self.settings.master_volume);
                    }
                    StartMenuAction::SelectWeapon(wid) => {
                        self.selected_weapon_id = wid;
                        self.player.lock_single_weapon(wid);
                        self.sound_bank.play_powerup(self.settings.master_volume);
                    }
                    StartMenuAction::StartGame => {
                        self.player.lock_single_weapon(self.selected_weapon_id);
                        let base_count = 50.0 * self.settings.swarm_scale;
                        self.enemies_to_spawn = (base_count.round() as u32).min(self.settings.max_ram_spawn_limit);
                        self.enemies_spawned = 0;
                        if self.settings.instant_full_spawn {
                            self.spawn_entire_wave_instantly();
                        }
                        self.state = GameState::Playing;
                    }
                    StartMenuAction::OpenSettings => {
                        self.settings_open = true;
                    }
                    StartMenuAction::ToggleLanguage => {
                        self.settings.language = self.settings.language.toggle();
                        set_current_language(self.settings.language);
                        self.settings.save_to_disk();
                        self.sound_bank.play_powerup(self.settings.master_volume);
                    }
                    StartMenuAction::None => {}
                }
            }
            GameState::Upgrade => {
                match draw_upgrade_menu(
                    &self.current_upgrade_choices,
                    &self.player.body.modifiers,
                    self.rerolls_left,
                    self.pending_upgrades.max(1),
                ) {
                    UpgradeMenuAction::Pick(chosen_id) => {
                        self.sound_bank.play_powerup(self.settings.master_volume);
                        self.apply_upgrade(chosen_id);
                        self.pending_upgrades = self.pending_upgrades.saturating_sub(1);
                        if self.pending_upgrades > 0 {
                            if has_available_unique_upgrades(&self.player.body.modifiers) {
                                self.current_upgrade_choices =
                                    roll_upgrade_choices(&self.player.body.modifiers, 4);
                            } else {
                                while self.pending_upgrades > 0 {
                                    self.apply_omni_stat_upgrade();
                                    self.pending_upgrades -= 1;
                                }
                                if self.end_of_wave_upgrade {
                                    self.end_of_wave_upgrade = false;
                                    self.wave += 1;
                                    self.rerolls_left = (self.rerolls_left + 2).min(5);
                                    let base_count = 50.0 + (self.wave as f32) * 22.0 + ((self.wave as f32).powf(1.3)) * 6.0;
                                    self.enemies_to_spawn = ((base_count * self.settings.swarm_scale).round() as u32)
                                        .min(self.settings.max_ram_spawn_limit);
                                    self.enemies_spawned = 0;
                                    self.populate_battlefield_props();
                                    if self.settings.instant_full_spawn {
                                        self.spawn_entire_wave_instantly();
                                    }
                                }
                                self.trigger_upgrade_safe_pulse();
                                self.state = GameState::Playing;
                            }
                        } else {
                            if self.end_of_wave_upgrade {
                                self.end_of_wave_upgrade = false;
                                self.wave += 1;
                                self.rerolls_left = (self.rerolls_left + 2).min(5);
                                let base_count = 50.0 + (self.wave as f32) * 22.0 + ((self.wave as f32).powf(1.3)) * 6.0;
                                self.enemies_to_spawn = ((base_count * self.settings.swarm_scale).round() as u32)
                                    .min(self.settings.max_ram_spawn_limit);
                                self.enemies_spawned = 0;
                                self.populate_battlefield_props();
                                if self.settings.instant_full_spawn {
                                    self.spawn_entire_wave_instantly();
                                }
                            }
                            self.trigger_upgrade_safe_pulse();
                            self.state = GameState::Playing;
                        }
                    }
                    UpgradeMenuAction::Reroll => {
                        if self.rerolls_left > 0 {
                            self.rerolls_left -= 1;
                            self.sound_bank.play_powerup(self.settings.master_volume);
                            self.current_upgrade_choices =
                                roll_upgrade_choices(&self.player.body.modifiers, 4);
                        }
                    }
                    UpgradeMenuAction::None => {}
                }
            }
            GameState::GameOver => {
                if draw_gameover_menu(
                    self.wave,
                    self.score,
                    self.total_kills,
                    self.shots_fired,
                    self.shots_hit,
                ) {
                    self.reset();
                }
            }
            GameState::Paused => {
                let _ = self.render_world();
                match draw_pause_menu() {
                    PauseMenuAction::Resume => {
                        self.state = GameState::Playing;
                    }
                    PauseMenuAction::OpenSettings => {
                        self.settings_open = true;
                    }
                    PauseMenuAction::Restart => {
                        self.reset();
                    }
                    PauseMenuAction::MainMenu => {
                        self.reset();
                        self.state = GameState::Start;
                    }
                    PauseMenuAction::None => {}
                }
            }
            GameState::Playing => {
                self.update_playing(dt);
                if self.render_world() {
                    self.state = GameState::Paused;
                    self.settings_open = true;
                }
            }
        }
    }

    fn apply_upgrade(&mut self, id: &'static str) {
        let p_pos = self.player.body.pos;
        if !self.player.body.modifiers.has(id) {
            self.player.body.modifiers.acquired_ids.push(id);
        }

        match id {
            // HỆ 0: TRỌNG PHÁO (HEAVY CALIBER)
            "heavy_1" => {
                self.player.body.modifiers.bullet_size_multi *= 1.35;
                self.player.body.modifiers.dmg_multi += 0.20;
            }
            "heavy_2a" => {
                self.player.body.modifiers.pierce_count += 2;
            }
            "heavy_2b" => {
                self.player.body.modifiers.impact_blast_radius =
                    self.player.body.modifiers.impact_blast_radius.max(55.0);
            }
            "heavy_3" => {
                self.player.body.modifiers.bullet_size_multi *= 1.65;
                self.player.body.modifiers.dmg_multi += 0.40;
                self.player.body.modifiers.pierce_count += 2;
            }

            // HỆ 1: LIÊN THANH (RAPID FIRE)
            "rapid_1" => {
                self.player.body.modifiers.fire_rate_multi *= 1.28;
            }
            "rapid_2a" => {
                self.player.body.modifiers.multishot_bonus += 1;
            }
            "rapid_2b" => {
                self.player.body.modifiers.bouncing_bullets = true;
            }
            "rapid_3" => {
                self.player.body.modifiers.fire_rate_multi *= 1.45;
                self.player.body.modifiers.multishot_bonus += 1;
            }

            // HỆ 2: TIẾP VẬN (FAST RELOAD)
            "reload_1" => {
                self.player.body.modifiers.reload_speed_multi *= 1.25;
                for w in self.player.weapons.iter_mut() {
                    w.stats.clip_size = ((w.stats.clip_size as f32) * 1.5).round() as u32;
                    w.stats.current_clip = w.stats.clip_size;
                }
            }
            "reload_2a" => {
                self.player.body.modifiers.reload_speed_multi *= 1.45;
                self.player.body.modifiers.wide_perfect_reload = true;
            }
            "reload_2b" => {
                self.player.body.modifiers.ammo_on_kill += 2;
            }
            "reload_3" => {
                self.player.body.modifiers.reload_speed_multi *= 1.35;
                self.player.body.modifiers.free_ammo_timer = 4.0;
                self.player.body.active_overcharge = true;
            }

            // HỆ 3: NGUYÊN TỐ (ELEMENTAL)
            "elem_1" => {
                self.player.body.modifiers.incendiary_rounds = true;
            }
            "elem_2a" => {
                self.player.body.modifiers.chain_on_hit =
                    self.player.body.modifiers.chain_on_hit.max(2);
            }
            "elem_2b" => {
                self.player.body.modifiers.cryo_rounds = true;
            }
            "elem_3" => {
                self.player.body.modifiers.incendiary_rounds = true;
                self.player.body.modifiers.cryo_rounds = true;
                self.player.body.modifiers.chain_on_hit =
                    self.player.body.modifiers.chain_on_hit.max(4);
                self.player.body.modifiers.dmg_multi += 0.25;
            }

            // HỆ 4: CƠ ĐỘNG & HƯ KHÔNG (PHANTOM MOBILITY)
            "mob_1" => {
                self.player.body.speed *= 1.20;
                self.player.body.modifiers.dash_cd_multi *= 0.75;
                self.player.body.modifiers.magnet_radius_multi += 0.40;
            }
            "mob_2a" => {
                self.player.body.modifiers.dash_fire_trail = true;
            }
            "mob_2b" => {
                self.player.body.max_stamina *= 1.60;
                self.player.body.stamina = self.player.body.max_stamina;
                self.player.body.modifiers.dmg_multi += 0.20;
            }
            "mob_3" => {
                self.player.body.modifiers.dash_blades = true;
                self.player.body.modifiers.dash_cd_multi *= 0.75;
            }

            // HỆ 5: CÔNG NGHỆ & SINH TỒN (CYBER AEGIS)
            "aegis_1" => {
                self.player.body.modifiers.vampiric = true;
                self.player.body.max_shield += 45.0;
                self.player.body.shield = self.player.body.max_shield;
            }
            "aegis_2a" => {
                self.player.body.modifiers.turret_overclock = true;
                self.deployables.push(Deployable::AutoTurret {
                    pos: p_pos + vec2(40.0, 0.0),
                    angle: 0.0,
                    fire_timer: 0.0,
                    fire_interval: 0.08,
                    life: 45.0,
                });
            }
            "aegis_2b" => {
                self.player.body.modifiers.shield_retaliation = true;
            }
            "aegis_3" => {
                self.player.body.max_hp += 60.0;
                self.player.body.hp = self.player.body.max_hp;
                self.player.body.max_shield += 60.0;
                self.player.body.shield = self.player.body.max_shield;
                for i in 0..2 {
                    self.allies.push(AlliedEntity::new(
                        AlliedBody {
                            name: "OMEGA DRONE",
                            pos: p_pos,
                            vel: Vec2::ZERO,
                            angle: 0.0,
                            radius: 9.0,
                            speed: 0.0,
                            color: Color::new(0.4, 1.0, 0.4, 1.0),
                            hp: 250.0,
                            max_hp: 250.0,
                            life: 999.0,
                        },
                        Box::new(OrbitingCombatDrone {
                            orbit_angle: (i as f32) * PI,
                            orbit_radius: 75.0,
                            fire_timer: 0.0,
                        }),
                    ));
                }
            }

            // HỆ 6: TỪ TRƯỜNG THU HỒI (MAGNETIC HARVESTER - Hút EXP Rộng)
            "magnet_1" => {
                self.player.body.modifiers.magnet_radius_multi += 0.85;
                self.player.body.modifiers.exp_gain_multi += 0.15;
            }
            "magnet_2a" => {
                self.player.body.modifiers.magnet_radius_multi += 1.20;
                self.player.body.modifiers.magnet_shield_on_orb = true;
            }
            "magnet_2b" => {
                self.player.body.modifiers.magnet_radius_multi += 0.75;
                self.player.body.modifiers.exp_gain_multi += 0.30;
                self.player.body.modifiers.magnet_arc_on_orb = true;
            }
            "magnet_3" => {
                self.player.body.modifiers.magnet_radius_multi += 2.20;
                self.player.body.modifiers.dmg_multi += 0.35;
                for orb in self.exp_orbs.iter_mut() {
                    orb.magnetized = true;
                }
            }

            // =====================================================
            // 12 BỘ KỸ NĂNG TỐI THƯỢNG (12 ULTIMATE SYNERGY SETS)
            // =====================================================
            "ult_apocalypse_gatling" => {
                self.player.body.modifiers.ult_apocalypse_gatling = true;
                self.player.body.modifiers.infinite_ammo = true;
                self.player.body.modifiers.bullet_size_multi *= 1.65;
                self.player.body.modifiers.fire_rate_multi *= 1.65;
                self.player.body.modifiers.impact_blast_radius =
                    self.player.body.modifiers.impact_blast_radius.max(85.0);
            }
            "ult_cataclysm_barrage" => {
                self.player.body.modifiers.ult_cataclysm_barrage = true;
                self.player.body.modifiers.dmg_multi += 0.45;
            }
            "ult_chrono_legion" => {
                self.player.body.modifiers.ult_chrono_legion = true;
                self.player.body.modifiers.dash_cd_multi *= 0.45;
                self.spawn_heavy_mech_ally(p_pos + vec2(60.0, 0.0));
            }
            "ult_hellfire_artillery" => {
                self.player.body.modifiers.ult_hellfire_artillery = true;
                self.player.body.modifiers.impact_blast_radius =
                    self.player.body.modifiers.impact_blast_radius.max(110.0);
            }
            "ult_volt_overdrive" => {
                self.player.body.modifiers.ult_volt_overdrive = true;
                self.player.body.modifiers.chain_on_hit =
                    self.player.body.modifiers.chain_on_hit.max(8);
            }
            "ult_bullet_hurricane" => {
                self.player.body.modifiers.ult_bullet_hurricane = true;
                self.player.body.modifiers.multishot_bonus += 2;
            }
            "ult_dreadnought_titan" => {
                self.player.body.modifiers.ult_dreadnought_titan = true;
                self.player.body.max_hp += 120.0;
                self.player.body.hp = self.player.body.max_hp;
                self.player.body.max_shield += 120.0;
                self.player.body.shield = self.player.body.max_shield;
            }
            "ult_phantom_trigger" => {
                self.player.body.modifiers.ult_phantom_trigger = true;
                self.player.body.speed *= 1.35;
            }
            "ult_hyper_reactor" => {
                self.player.body.modifiers.ult_hyper_reactor = true;
            }
            "ult_autonomous_arsenal" => {
                self.player.body.modifiers.ult_autonomous_arsenal = true;
                for side in [-1.0f32, 1.0] {
                    self.deployables.push(Deployable::AutoTurret {
                        pos: p_pos + vec2(side * 65.0, -35.0),
                        angle: 0.0,
                        fire_timer: 0.0,
                        fire_interval: 0.07,
                        life: 120.0,
                    });
                }
            }
            "ult_singularity_walker" => {
                self.player.body.modifiers.ult_singularity_walker = true;
            }
            "ult_valkyrie_impact" => {
                self.player.body.modifiers.ult_valkyrie_impact = true;
                self.player.body.modifiers.dmg_multi += 0.65;
            }

            // HỆ 7: ĐA HƯỚNG & HỎA LỰC PHÒNG THỦ (OMNI-FIRE)
            "omni_1" => {
                self.player.body.modifiers.multi_rear_shot = true;
                self.player.body.modifiers.dmg_multi += 0.15;
            }
            "omni_2a" => {
                self.player.body.modifiers.multi_flank_shot = true;
                self.player.body.modifiers.fire_rate_multi *= 1.20;
            }
            "omni_2b" => {
                self.player.body.modifiers.orbital_plasma_shield = true;
                self.spawn_orbital_plasma_spheres(3, 85.0);
            }
            "omni_3" => {
                self.player.body.modifiers.multi_omni_spiral = true;
                self.player.body.modifiers.dmg_multi += 0.25;
            }

            // CÁC BỘ TỐI THƯỢNG MỚI (13, 14, 15, 16)
            "ult_orbital_supernova" => {
                self.player.body.modifiers.ult_orbital_supernova = true;
                self.spawn_orbital_plasma_spheres(3, 125.0);
            }
            "ult_omni_annihilation" => {
                self.player.body.modifiers.ult_omni_annihilation = true;
                self.player.body.modifiers.dmg_multi += 0.40;
                self.player.body.modifiers.multishot_bonus += 2;
            }
            "ult_chronos_singularity" => {
                self.player.body.modifiers.ult_chronos_singularity = true;
            }
            "ult_divine_wrath" => {
                self.player.body.modifiers.ult_divine_wrath = true;
                self.player.body.modifiers.shield_retaliation = true;
            }

            // Nâng cấp cộng dồn dự phòng
            "boost_dmg" => self.player.body.modifiers.dmg_multi += 0.25,
            "boost_rof" => self.player.body.modifiers.fire_rate_multi *= 1.20,
            "boost_hp" => {
                self.player.body.max_hp += 50.0;
                self.player.body.hp = self.player.body.max_hp;
                self.player.body.max_shield += 50.0;
                self.player.body.shield = self.player.body.max_shield;
            }
            "boost_spd" => {
                self.player.body.speed *= 1.15;
                self.player.body.stamina = self.player.body.max_stamina;
            }
            "boost_magnet" => {
                self.player.body.modifiers.magnet_radius_multi += 0.60;
                self.player.body.modifiers.exp_gain_multi += 0.15;
            }
            _ => {}
        }

        if id.starts_with("ult_") {
            self.float_texts.push(FloatText {
                pos: p_pos - vec2(0.0, 48.0),
                text: tr(
                    "** ĐÃ KÍCH HOẠT BỘ KỸ NĂNG TỐI THƯỢNG! **",
                    "** ULTIMATE SYNERGY SET ACTIVATED! **",
                )
                .to_string(),
                color: Color::new(1.0, 0.85, 0.1, 1.0),
                size: 25.0,
                life: 2.0,
            });
        }
    }

    fn spawn_heavy_mech_ally(&mut self, pos: Vec2) {
        self.allies.push(AlliedEntity::new(
            AlliedBody {
                name: "CHRONO TITAN MECH",
                pos,
                vel: Vec2::ZERO,
                angle: 0.0,
                radius: 21.0,
                speed: 225.0,
                color: Color::new(1.0, 0.84, 0.15, 1.0),
                hp: 480.0,
                max_hp: 480.0,
                life: 40.0,
            },
            Box::new(AlliedHeavyMech {
                fire_timer: 0.0,
                rocket_timer: 0.5,
            }),
        ));
    }

    fn update_playing(&mut self, dt: f32) {
        self.player.body.bullet_time =
            is_key_down(KeyCode::LeftShift) || is_key_down(KeyCode::RightShift);
        let time_scale = if self.player.body.bullet_time && self.player.body.stamina > 0.0 {
            self.player.body.stamina = (self.player.body.stamina - dt * 45.0).max(0.0);
            0.4
        } else {
            self.player.body.bullet_time = false;
            self.player.body.stamina =
                (self.player.body.stamina + dt * 18.0).min(self.player.body.max_stamina);
            1.0
        };
        let sim_dt = dt * time_scale;

        if self.screen_shake > 0.0 {
            self.screen_shake = (self.screen_shake - dt * 26.0).max(0.0);
        }

        // Aiming + Hệ thống Hỗ Trợ Nhắm Bắn Thông Minh (Smart Aim Assist & Auto-Lock)
        if is_key_pressed(KeyCode::Tab) {
            self.settings.aim_assist_mode = (self.settings.aim_assist_mode + 1) % 3;
            self.settings.save_to_disk();
            self.float_texts.push(FloatText {
                pos: self.player.body.pos - vec2(0.0, 42.0),
                text: format!(
                    "{} [TAB]: {}",
                    tr("CHẾ ĐỘ NGẮM", "AIM MODE"),
                    self.settings.aim_mode_label()
                ),
                color: Color::new(0.0, 1.0, 0.75, 1.0),
                size: 21.0,
                life: 1.4,
            });
        }
        if is_key_pressed(KeyCode::V) {
            self.settings.auto_fire = !self.settings.auto_fire;
            self.settings.save_to_disk();
            self.float_texts.push(FloatText {
                pos: self.player.body.pos - vec2(0.0, 42.0),
                text: format!(
                    "{} [V]: {}",
                    tr("TỰ ĐỘNG BẮN", "AUTO-FIRE"),
                    if self.settings.auto_fire {
                        tr("BẬT (ON)", "ENABLED")
                    } else {
                        tr("TẮT (OFF)", "DISABLED")
                    }
                ),
                color: Color::new(1.0, 0.85, 0.1, 1.0),
                size: 21.0,
                life: 1.3,
            });
        }

        let mouse_screen = vec2(mouse_position().0, mouse_position().1);
        let mouse_world = mouse_screen + self.camera_pos
            - vec2(screen_width() / 2.0, screen_height() / 2.0);
        let (assisted_angle, locked_pos) = self.compute_smart_aim(mouse_world);
        self.player.body.angle = assisted_angle;
        self.locked_target_pos = locked_pos;

        // Shield Regen
        self.player.body.shield_regen_timer += dt;
        if self.player.body.shield_regen_timer > 4.0
            && self.player.body.shield < self.player.body.max_shield
        {
            self.player.body.shield =
                (self.player.body.shield + dt * 25.0).min(self.player.body.max_shield);
        }

        // Đảm bảo trận đấu luôn cố định đúng 1 loại vũ khí duy nhất đã chọn và đồng bộ tên theo ngôn ngữ
        self.player.current_weapon_idx = self.selected_weapon_id;
        {
            let wid = self.selected_weapon_id;
            let w = self.player.current_weapon_mut();
            w.stats.name = if w.stats.level >= 100 {
                evolved_weapon_name_by_id(wid)
            } else {
                weapon_name_by_id(wid)
            };
        }

        if self.player.body.modifiers.free_ammo_timer > 0.0 {
            self.player.body.modifiers.free_ammo_timer =
                (self.player.body.modifiers.free_ammo_timer - dt).max(0.0);
        }

        // Movement Input
        let mut move_dir = Vec2::ZERO;
        if is_key_down(KeyCode::W) || is_key_down(KeyCode::Up) {
            move_dir.y -= 1.0;
        }
        if is_key_down(KeyCode::S) || is_key_down(KeyCode::Down) {
            move_dir.y += 1.0;
        }
        if is_key_down(KeyCode::A) || is_key_down(KeyCode::Left) {
            move_dir.x -= 1.0;
        }
        if is_key_down(KeyCode::D) || is_key_down(KeyCode::Right) {
            move_dir.x += 1.0;
        }

        // Skills
        let mut ctx = self.make_context();
        for skill in self.player.skills.iter_mut() {
            if skill.cooldown > 0.0 {
                skill.cooldown -= dt;
            }
            if is_key_pressed(skill.key) {
                skill.try_activate(&mut self.player.body, mouse_world, move_dir, &mut ctx);
            }
        }

        // Kích hoạt hiệu ứng khi vừa bắt đầu Lướt (Dash Start Triggers: Dash Blades & Ultimate Sets)
        if self.player.body.is_dashing && !self.was_dashing {
            let p_pos = self.player.body.pos;
            if self.player.body.modifiers.dash_blades {
                for i in 0..12 {
                    let a = (i as f32) * (PI * 2.0 / 12.0);
                    ctx.spawn_player_projectile(Projectile {
                        pos: p_pos,
                        vel: vec2(a.cos(), a.sin()) * 780.0,
                        radius: 6.5,
                        damage: 58.0 * self.player.body.modifiers.dmg_multi,
                        life: 0.7,
                        color: Color::new(0.0, 0.95, 1.0, 1.0),
                        pierce: 2,
                        bounces: 1,
                        explosion_radius: None,
                        spawns_black_hole: false,
                        status_effect: None,
                    });
                }
            }
            if self.player.body.modifiers.ult_phantom_trigger {
                self.player.body.active_overcharge = true;
                for i in 0..16 {
                    let a = (i as f32) * (PI * 2.0 / 16.0);
                    ctx.spawn_player_projectile(Projectile {
                        pos: p_pos,
                        vel: vec2(a.cos(), a.sin()) * 860.0,
                        radius: 7.0,
                        damage: 72.0 * self.player.body.modifiers.dmg_multi,
                        life: 0.85,
                        color: Color::new(0.85, 0.35, 1.0, 1.0),
                        pierce: 3,
                        bounces: 1,
                        explosion_radius: None,
                        spawns_black_hole: false,
                        status_effect: None,
                    });
                }
            }
            if self.player.body.modifiers.ult_singularity_walker {
                ctx.spawn_event(Box::new(SingularityBlackHoleEvent {
                    pos: p_pos,
                    pull_radius: 260.0,
                    pull_force: 380.0,
                    dps: 110.0,
                    duration: 2.6,
                    max_duration: 2.6,
                    rotation: 0.0,
                }));
            }
            if self.player.body.modifiers.ult_valkyrie_impact {
                ctx.trigger_explosion(p_pos, 175.0, 290.0 * self.player.body.modifiers.dmg_multi);
                ctx.spawn_event(Box::new(ShockwaveRingEvent {
                    center: p_pos,
                    current_radius: 12.0,
                    max_radius: 185.0,
                    speed: 620.0,
                    color: Color::new(1.0, 0.75, 0.1, 0.95),
                    thickness: 7.0,
                }));
            }
        }
        self.was_dashing = self.player.body.is_dashing;

        // Chu kỳ kích hoạt tự động của các Bộ Kỹ Năng Tối Thượng (Ultimate Periodic Triggers)
        self.ult_periodic_timer += sim_dt;
        if self.ult_periodic_timer >= 2.2 {
            self.ult_periodic_timer = 0.0;
            let p_pos = self.player.body.pos;
            let dmg_m = self.player.body.modifiers.dmg_multi;
            if self.player.body.modifiers.ult_bullet_hurricane {
                let base_a = (get_time() as f32) * 2.5;
                for i in 0..16 {
                    let a = base_a + (i as f32) * (PI * 2.0 / 16.0);
                    ctx.spawn_player_projectile(Projectile {
                        pos: p_pos,
                        vel: vec2(a.cos(), a.sin()) * 760.0,
                        radius: 6.5 * self.player.body.modifiers.bullet_size_multi,
                        damage: 55.0 * dmg_m,
                        life: 1.1,
                        color: Color::new(0.0, 1.0, 0.85, 1.0),
                        pierce: 2 + self.player.body.modifiers.pierce_count,
                        bounces: if self.player.body.modifiers.bouncing_bullets {
                            4
                        } else {
                            2
                        },
                        explosion_radius: None,
                        spawns_black_hole: false,
                        status_effect: None,
                    });
                }
            }
            if self.player.body.modifiers.ult_dreadnought_titan {
                for side in [-1.0f32, 1.0] {
                    let a = self.player.body.angle + side * 0.35;
                    ctx.spawn_player_projectile(Projectile {
                        pos: p_pos,
                        vel: vec2(a.cos(), a.sin()) * 680.0,
                        radius: 10.0,
                        damage: 185.0 * dmg_m,
                        life: 1.8,
                        color: Color::new(1.0, 0.45, 0.0, 1.0),
                        pierce: 0,
                        bounces: 0,
                        explosion_radius: Some(150.0),
                        spawns_black_hole: false,
                        status_effect: Some(StatusEffect::Burn {
                            dps: 50.0,
                            duration: 3.0,
                        }),
                    });
                }
            }
            if self.player.body.modifiers.ult_hyper_reactor {
                ctx.trigger_explosion(p_pos, 220.0, 220.0 * dmg_m);
                ctx.spawn_event(Box::new(ShockwaveRingEvent {
                    center: p_pos,
                    current_radius: 15.0,
                    max_radius: 230.0,
                    speed: 650.0,
                    color: Color::new(0.0, 0.95, 1.0, 0.95),
                    thickness: 8.0,
                }));
            }
            if self.player.body.modifiers.ult_orbital_supernova {
                ctx.trigger_explosion(p_pos, 350.0, 360.0 * dmg_m);
                ctx.spawn_event(Box::new(ShockwaveRingEvent {
                    center: p_pos,
                    current_radius: 20.0,
                    max_radius: 360.0,
                    speed: 850.0,
                    color: Color::new(1.0, 0.45, 0.9, 0.95),
                    thickness: 12.0,
                }));
            }
            if self.player.body.modifiers.ult_chronos_singularity {
                let target = ctx.find_nearest_enemy(p_pos, 600.0).unwrap_or(p_pos + vec2(150.0, 0.0));
                ctx.spawn_event(Box::new(SingularityBlackHoleEvent {
                    pos: target,
                    pull_radius: 450.0,
                    pull_force: 520.0,
                    dps: 180.0 * dmg_m,
                    duration: 3.2,
                    max_duration: 3.2,
                    rotation: 0.0,
                }));
            }
            if self.player.body.modifiers.ult_divine_wrath {
                for _ in 0..6 {
                    let aim_dir = vec2(rand::gen_range(-1.0, 1.0), rand::gen_range(-1.0, 1.0)).normalize_or_zero();
                    ctx.chain_lightning(p_pos, aim_dir, 5, 450.0, 140.0 * dmg_m);
                }
            }
        }
        self.flush_commands(ctx.commands);

        if self.player.body.invulnerable_timer > 0.0 {
            self.player.body.invulnerable_timer = (self.player.body.invulnerable_timer - dt).max(0.0);
        }

        // Player Movement & Dash
        if self.player.body.is_dashing {
            self.player.body.dash_duration -= dt;
            self.player.body.pos += self.player.body.vel * dt;
            if self.player.body.modifiers.dash_fire_trail {
                self.decals.push(Decal {
                    pos: self.player.body.pos,
                    radius: 16.0,
                    color: Color::new(1.0, 0.3, 0.0, 0.6),
                    life: 3.0,
                    is_damage_zone: true,
                });
            }
            if self.player.body.dash_duration <= 0.0 {
                self.player.body.is_dashing = false;
                self.player.body.vel = Vec2::ZERO;
            }
        } else if move_dir != Vec2::ZERO {
            self.player.body.pos += move_dir.normalize() * self.player.body.speed * dt;
        }

        let r = self.player.body.radius;
        self.player.body.pos.x = self.player.body.pos.x.clamp(r, WORLD_WIDTH - r);
        self.player.body.pos.y = self.player.body.pos.y.clamp(r, WORLD_HEIGHT - r);

        // Active Reload (R)
        self.handle_reload(dt);

        // Melee Slash (Right Click)
        if is_mouse_button_pressed(MouseButton::Right) {
            self.perform_melee();
        }

        // Weapon Firing (Left Click hoặc Tự động bắn Auto-Fire [V] khi đã khóa mục tiêu)
        if self.player.fire_timer > 0.0 {
            self.player.fire_timer -= dt;
        }
        let wants_fire = is_mouse_button_down(MouseButton::Left)
            || (self.settings.auto_fire && self.locked_target_pos.is_some());
        if wants_fire
            && self.player.fire_timer <= 0.0
            && !self.player.reload_state.is_reloading
        {
            self.fire_current_weapon();
        }

        // Update Battlefield Events (Trait BattlefieldEvent)
        let mut ev_ctx = self.make_context();
        let mut active_events = std::mem::take(&mut self.events);
        active_events.retain_mut(|ev| ev.update(&mut ev_ctx, sim_dt));
        self.events.append(&mut active_events);
        self.flush_commands(ev_ctx.commands);

        // Update Allied Units (Trait AlliedBehavior)
        let mut ally_ctx = self.make_context();
        self.allies.retain_mut(|a| a.tick(&mut ally_ctx, sim_dt));
        self.flush_commands(ally_ctx.commands);

        // Update Tactical Battlefield Structures (Trait BattlefieldStructure)
        let mut st_ctx = self.make_context();
        let mut active_structures = std::mem::take(&mut self.structures);
        active_structures.retain_mut(|st| st.tick(&mut self.player.body, &mut st_ctx, sim_dt));
        self.structures.append(&mut active_structures);
        self.flush_commands(st_ctx.commands);

        // Update Deployables
        let mut dep_ctx = self.make_context();
        self.deployables
            .retain_mut(|d| d.update(&mut dep_ctx, sim_dt));
        self.flush_commands(dep_ctx.commands);

        // Update Explosive Barrels (Domino Chain Reactions)
        let mut barrel_ctx = self.make_context();
        for barrel in self.barrels.iter_mut() {
            if let Some(ref mut fuse) = barrel.fuse_timer {
                *fuse -= sim_dt;
                if *fuse <= 0.0 {
                    barrel.hp = 0.0;
                    barrel_ctx.trigger_explosion(barrel.pos, 165.0, 240.0);
                    barrel_ctx.spawn_event(Box::new(ShockwaveRingEvent {
                        center: barrel.pos,
                        current_radius: 10.0,
                        max_radius: 175.0,
                        speed: 540.0,
                        color: Color::new(1.0, 0.5, 0.0, 0.9),
                        thickness: 6.0,
                    }));
                }
            }
        }
        self.barrels.retain(|b| b.hp > 0.0);
        self.flush_commands(barrel_ctx.commands);

        // Update Player Projectiles (kèm cơ chế Bullet Magnetism tự bẻ lái nhẹ vào quái khi bật Aim Assist)
        let mut proj_ctx = self.make_context();
        let assist_magnet = self.settings.aim_assist_mode >= 1;
        for b in self.player_projectiles.iter_mut() {
            if assist_magnet && !b.spawns_black_hole {
                let spd = b.vel.length();
                if spd > 10.0 {
                    let b_dir = b.vel / spd;
                    let mut best_dir = None;
                    let mut best_dist = 95.0f32;
                    for e in &self.enemies {
                        let to_e = e.body.pos - b.pos;
                        let d = to_e.length();
                        if d < best_dist && d > 1.0 {
                            let dir_e = to_e / d;
                            if b_dir.dot(dir_e) > 0.72 {
                                best_dist = d;
                                best_dir = Some(dir_e);
                            }
                        }
                    }
                    if let Some(target_dir) = best_dir {
                        let new_dir = (b_dir + target_dir * (6.5 * sim_dt)).normalize_or_zero();
                        b.vel = new_dir * spd;
                    }
                }
            }
            b.pos += b.vel * sim_dt;
            b.life -= sim_dt;

            if b.bounces > 0 {
                if b.pos.x < 0.0 || b.pos.x > WORLD_WIDTH {
                    b.vel.x *= -1.0;
                    b.bounces -= 1;
                }
                if b.pos.y < 0.0 || b.pos.y > WORLD_HEIGHT {
                    b.vel.y *= -1.0;
                    b.bounces -= 1;
                }
            }

            // Check hit against Explosive Barrels
            for barrel in self.barrels.iter_mut() {
                if (barrel.pos - b.pos).length() < barrel.radius + b.radius && b.life > 0.0 {
                    barrel.hp -= b.damage;
                    if barrel.hp <= 0.0 && barrel.fuse_timer.is_none() {
                        barrel.fuse_timer = Some(0.02);
                    }
                    if b.pierce == 0 {
                        b.life = 0.0;
                    }
                }
            }

            // Check hit against Enemies & Boss Parts (AABB Pre-check tối ưu cho 10,000 quái)
            for e in self.enemies.iter_mut() {
                let dx = (e.body.pos.x - b.pos.x).abs();
                let dy = (e.body.pos.y - b.pos.y).abs();
                if dx > 48.0 || dy > 48.0 {
                    continue;
                }
                if e.check_hit(b.pos, b.radius) && b.life > 0.0 {
                    self.shots_hit += 1;
                    if b.spawns_black_hole {
                        b.life = 0.0;
                        proj_ctx.spawn_event(Box::new(SingularityBlackHoleEvent {
                            pos: b.pos,
                            pull_radius: 260.0,
                            pull_force: 360.0,
                            dps: 95.0,
                            duration: 2.8,
                            max_duration: 2.8,
                            rotation: 0.0,
                        }));
                        break;
                    } else if let Some(blast_rad) = b.explosion_radius {
                        b.life = 0.0;
                        proj_ctx.trigger_explosion(b.pos, blast_rad, b.damage);
                        if self.player.body.modifiers.ult_hellfire_artillery {
                            self.decals.push(Decal {
                                pos: b.pos,
                                radius: 38.0,
                                color: Color::new(1.0, 0.35, 0.0, 0.65),
                                life: 3.5,
                                is_damage_zone: true,
                            });
                        }
                        break;
                    } else {
                        let knockback = b.vel.normalize_or_zero() * 65.0;
                        let dealt =
                            e.take_damage_at(b.pos, b.damage, knockback, b.status_effect, &mut proj_ctx);
                        if self.player.body.modifiers.chain_on_hit > 0 {
                            proj_ctx.chain_lightning(
                                b.pos,
                                b.vel.normalize_or_zero(),
                                self.player.body.modifiers.chain_on_hit as usize,
                                230.0,
                                dealt * 0.45,
                            );
                        }
                        if self.player.body.modifiers.ult_hellfire_artillery {
                            self.decals.push(Decal {
                                pos: b.pos,
                                radius: 26.0,
                                color: Color::new(1.0, 0.35, 0.0, 0.55),
                                life: 2.8,
                                is_damage_zone: true,
                            });
                        }
                        if self.player.body.modifiers.ult_cataclysm_barrage
                            && rand::gen_range(0.0, 1.0) < 0.25
                        {
                            proj_ctx.spawn_event(Box::new(MeteorWarningStrikeEvent {
                                target_pos: b.pos,
                                radius: 135.0,
                                timer: 0.45,
                                max_time: 0.45,
                                damage: 220.0 * self.player.body.modifiers.dmg_multi,
                                harms_player: false,
                            }));
                        }
                        if self.settings.show_damage_numbers && self.float_texts.len() < 120 {
                            self.float_texts.push(FloatText {
                                pos: b.pos,
                                text: format!("{:.0}", dealt),
                                color: WHITE,
                                size: 14.0,
                                life: 0.55,
                            });
                        }
                        if b.pierce > 0 {
                            b.pierce -= 1;
                        } else {
                            b.life = 0.0;
                            break;
                        }
                    }
                }
            }
        }
        self.player_projectiles.retain(|b| {
            b.life > 0.0
                && b.pos.x >= 0.0
                && b.pos.x <= WORLD_WIDTH
                && b.pos.y >= 0.0
                && b.pos.y <= WORLD_HEIGHT
        });
        self.flush_commands(proj_ctx.commands);

        // Incremental Spawner (Chỉ chạy khi tắt chế độ Instant Full Spawn)
        if !self.settings.instant_full_spawn {
            self.spawn_timer += sim_dt;
            let spawn_interval = (0.45 - (self.wave as f32) * 0.03).max(0.15);
            if self.spawn_timer >= spawn_interval && self.enemies_spawned < self.enemies_to_spawn {
                self.spawn_timer = 0.0;
                let squad_size = rand::gen_range(4, 9).min(self.enemies_to_spawn - self.enemies_spawned);
                let base_angle = rand::gen_range(0.0, PI * 2.0);
                let base_dist = rand::gen_range(880.0, 1250.0);

                for s in 0..squad_size {
                    let angle = base_angle + (s as f32) * 0.09;
                    let ex = (self.player.body.pos.x + angle.cos() * base_dist)
                        .clamp(60.0, WORLD_WIDTH - 60.0);
                    let ey = (self.player.body.pos.y + angle.sin() * base_dist)
                        .clamp(60.0, WORLD_HEIGHT - 60.0);

                    let enemy =
                        EnemyFactory::spawn_for_wave(vec2(ex, ey), self.wave, self.enemies_spawned);
                    self.enemies.push(enemy);
                    self.enemies_spawned += 1;
                }
            }
        }

        // Update Enemies (Tối ưu hóa kiểm tra bãi độc cho 10,000 quái)
        let mut enemy_ctx = self.make_context();
        let damage_decals: Vec<_> = self.decals.iter().filter(|d| d.is_damage_zone).collect();
        for e in self.enemies.iter_mut() {
            if !damage_decals.is_empty() {
                for d in &damage_decals {
                    let max_r = d.radius + e.body.radius;
                    if (e.body.pos.x - d.pos.x).abs() < max_r
                        && (e.body.pos.y - d.pos.y).abs() < max_r
                        && (e.body.pos - d.pos).length() < max_r
                    {
                        e.body.hp -= 35.0 * sim_dt;
                    }
                }
            }
            e.tick(&mut enemy_ctx, sim_dt);
        }
        self.flush_commands(enemy_ctx.commands);

        self.cleanup_dead_enemies();
        self.update_exp_orbs(dt);
        self.update_weapon_drops(dt);
        self.update_pickups(dt);

        // Update Enemy Projectiles (Including Energy Barricade Bunker Interception!)
        let mut hit_shake = 0.0;
        let mut shield_retaliate = false;
        for eb in self.enemy_projectiles.iter_mut() {
            eb.pos += eb.vel * sim_dt;
            eb.life -= sim_dt;

            for st in self.structures.iter_mut() {
                if st.behavior.blocks_enemy_projectiles()
                    && (eb.pos - st.body.pos).length() < eb.radius + st.body.radius
                    && eb.life > 0.0
                {
                    st.body.hp -= eb.damage * 0.5;
                    eb.life = 0.0;
                    break;
                }
            }

            if eb.life > 0.0
                && (eb.pos - self.player.body.pos).length() < eb.radius + self.player.body.radius
                && !self.player.body.is_dashing
            {
                let prev_shield = self.player.body.shield;
                hit_shake += self.player.take_damage(eb.damage);
                if prev_shield > 0.0 && self.player.body.modifiers.shield_retaliation {
                    shield_retaliate = true;
                }
                eb.life = 0.0;
            }
        }
        if shield_retaliate {
            let mut ret_ctx = self.make_context();
            ret_ctx.trigger_explosion(self.player.body.pos, 180.0, 145.0);
            ret_ctx.spawn_event(Box::new(ShockwaveRingEvent {
                center: self.player.body.pos,
                current_radius: 14.0,
                max_radius: 185.0,
                speed: 620.0,
                color: Color::new(0.0, 0.95, 1.0, 0.95),
                thickness: 6.0,
            }));
            self.flush_commands(ret_ctx.commands);
        }
        if hit_shake > 0.0 {
            self.add_screen_shake(hit_shake);
        }
        self.enemy_projectiles.retain(|eb| {
            eb.life > 0.0
                && eb.pos.x >= 0.0
                && eb.pos.x <= WORLD_WIDTH
                && eb.pos.y >= 0.0
                && eb.pos.y <= WORLD_HEIGHT
        });

        // Visual FX Updates
        for d in self.decals.iter_mut() {
            d.life -= dt;
        }
        self.decals.retain(|d| d.life > 0.0);

        for c in self.casings.iter_mut() {
            c.pos += c.vel * dt;
            c.vel *= 0.9;
            c.rotation += c.v_rot * dt;
            c.life -= dt;
        }
        self.casings.retain(|c| c.life > 0.0);

        for p in self.particles.iter_mut() {
            p.pos += p.vel * dt;
            p.life -= dt;
        }
        self.particles.retain(|p| p.life > 0.0);

        for f in self.float_texts.iter_mut() {
            f.pos.y -= 32.0 * dt;
            f.life -= dt;
        }
        self.float_texts.retain(|f| f.life > 0.0);

        if self.combo_timer > 0.0 {
            self.combo_timer -= dt;
            if self.combo_timer <= 0.0 {
                self.combo = 1;
            }
        }

        if self.enemies_spawned >= self.enemies_to_spawn && self.enemies.is_empty() {
            self.end_of_wave_upgrade = true;
            self.pending_upgrades += 1;
        }

        if self.pending_upgrades > 0 {
            if has_available_unique_upgrades(&self.player.body.modifiers) {
                self.current_upgrade_choices =
                    roll_upgrade_choices(&self.player.body.modifiers, 4);
                self.state = GameState::Upgrade;
            } else {
                // Đã cày xong hết toàn bộ buff độc nhất: Tự động cộng chỉ số toàn năng mà không ngắt quãng trận chiến!
                while self.pending_upgrades > 0 {
                    self.apply_omni_stat_upgrade();
                    self.pending_upgrades -= 1;
                }
                if self.end_of_wave_upgrade {
                    self.end_of_wave_upgrade = false;
                    self.wave += 1;
                    self.rerolls_left = (self.rerolls_left + 2).min(5);
                    let base_count = 50.0 + (self.wave as f32) * 22.0 + ((self.wave as f32).powf(1.3)) * 6.0;
                    self.enemies_to_spawn = ((base_count * self.settings.swarm_scale).round() as u32)
                        .min(self.settings.max_ram_spawn_limit);
                    self.enemies_spawned = 0;
                    self.populate_battlefield_props();
                    if self.settings.instant_full_spawn {
                        self.spawn_entire_wave_instantly();
                    }
                }
                self.trigger_upgrade_safe_pulse();
            }
        }

        if self.score > self.settings.high_score {
            self.settings.high_score = self.score;
        }

        if self.player.body.hp <= 0.0 {
            self.settings.save_to_disk();
            self.state = GameState::GameOver;
        }

        self.camera_pos += (self.player.body.pos - self.camera_pos) * 10.0 * dt;
    }

    fn update_weapon_drops(&mut self, dt: f32) {
        let p_pos = self.player.body.pos;
        let press_f = is_key_pressed(KeyCode::F);
        let module_magnet_rad = 85.0 * self.player.body.modifiers.magnet_radius_multi;

        let mut i = 0;
        while i < self.weapon_drops.len() {
            self.weapon_drops[i].bob_phase += dt * 4.5;
            let to_p = p_pos - self.weapon_drops[i].pos;
            let dist = to_p.length();

            // Từ trường hút các Lõi Nâng Cấp Súng về phía người chơi khi vào vùng hút
            if self.settings.auto_pickup_weapons && dist < module_magnet_rad && dist > 1.0 {
                self.weapon_drops[i].pos += to_p.normalize_or_zero() * 390.0 * dt;
            }

            let dist_now = (self.weapon_drops[i].pos - p_pos).length();
            let should_pickup = (self.settings.auto_pickup_weapons && dist_now < 36.0)
                || (dist_now < 95.0 && press_f);

            if should_pickup {
                self.sound_bank.play_powerup(self.settings.master_volume);
                let drop = self.weapon_drops.swap_remove(i);
                if drop.weapon_id < self.player.weapons.len() {
                    let w = &mut self.player.weapons[drop.weapon_id];
                    let prev_lvl = w.stats.level;
                    let lv_gain = drop.rarity.upgrade_levels();
                    w.stats.unlocked = true;
                    w.stats.level = (w.stats.level + lv_gain).min(500);

                    let prev_form_tier = (prev_lvl / 100).min(5);
                    let new_form_tier = (w.stats.level / 100).min(5);
                    let just_new_form = new_form_tier > prev_form_tier;
                    let just_ultimate = prev_lvl < 500 && w.stats.level >= 500;

                    // Cập nhật tên và độ hiếm theo Form cấp độ (1-99: Dạng 1, 100-199: Dạng 2, 200-299: Dạng 3, 300-399: Dạng 4, 400-499: Dạng 5, 500: Tối Thượng)
                    w.stats.name = weapon_form_name(drop.weapon_id, w.stats.level);
                    w.stats.rarity = WeaponRarity::from_level(w.stats.level);

                    w.stats.current_clip = w.stats.clip_size;
                    if w.stats.reserve_ammo >= 0 {
                        w.stats.reserve_ammo += (w.stats.clip_size * 5) as i32;
                    }
                    let w_name = w.stats.name;
                    let w_lvl = w.stats.level;
                    let w_rarity = w.stats.rarity.label();
                    let w_col = w.stats.rarity.color();

                    if just_ultimate {
                        self.screen_shake = (self.screen_shake + 12.0).min(20.0);
                        self.events.push(Box::new(crate::core::events::ShockwaveRingEvent {
                            center: p_pos,
                            current_radius: 15.0,
                            max_radius: 240.0,
                            speed: 550.0,
                            color: Color::new(0.2, 1.0, 0.85, 0.9),
                            thickness: 6.0,
                        }));
                    }

                    self.float_texts.push(FloatText {
                        pos: p_pos - vec2(0.0, 38.0),
                        text: if just_ultimate {
                            format!(
                                "{} {} (MAX Lv.500)!",
                                tr("★ ĐẠT DẠNG TỐI THƯỢNG:", "★ ULTIMATE FORM REACHED:"),
                                w_name
                            )
                        } else if just_new_form {
                            format!(
                                "{} {} (Lv.{}/500)!",
                                tr("MỞ KHÓA DẠNG MỚI:", "UNLOCKED NEW FORM:"),
                                w_name,
                                w_lvl
                            )
                        } else if w_lvl < 500 {
                            format!(
                                "+{} LV -> {} Lv.{}/500 [{}]!",
                                lv_gain, w_name, w_lvl, w_rarity
                            )
                        } else {
                            format!("{} Lv.500 [MAX TỐI THƯỢNG]!", w_name)
                        },
                        color: if just_ultimate {
                            Color::new(0.2, 1.0, 0.85, 1.0)
                        } else if just_new_form {
                            Color::new(1.0, 0.85, 0.1, 1.0)
                        } else {
                            w_col
                        },
                        size: if just_ultimate { 28.0 } else if just_new_form { 24.0 } else { 20.0 },
                        life: if just_ultimate { 2.4 } else if just_new_form { 1.8 } else { 1.35 },
                    });
                }
            } else {
                i += 1;
            }
        }
    }

    fn handle_reload(&mut self, dt: f32) {
        let clip = self.player.current_weapon().stats.current_clip;
        let max_clip = self.player.current_weapon().stats.clip_size;
        let reserve = self.player.current_weapon().stats.reserve_ammo;
        let reload_spd = self.player.body.modifiers.reload_speed_multi.max(0.2);
        let reload_dur = (self.player.current_weapon().stats.reload_duration / reload_spd).max(0.18);
        let sweet_range = if self.player.body.modifiers.wide_perfect_reload {
            0.40..=0.88
        } else {
            0.58..=0.82
        };

        if is_key_pressed(KeyCode::R) {
            if !self.player.reload_state.is_reloading && clip < max_clip && reserve != 0 {
                self.player.reload_state.is_reloading = true;
                self.player.reload_state.timer = 0.0;
                self.player.reload_state.duration = reload_dur;
            } else if self.player.reload_state.is_reloading {
                let progress =
                    self.player.reload_state.timer / self.player.reload_state.duration.max(0.01);
                if sweet_range.contains(&progress) {
                    self.player.body.active_overcharge = true;
                    if self.player.body.modifiers.has("reload_3") {
                        self.player.body.modifiers.free_ammo_timer = 3.5;
                    }
                    let mut ctx = self.make_context();
                    let w = self.player.current_weapon_mut();
                    let needed = w.stats.clip_size - w.stats.current_clip;
                    let can_load = if w.stats.reserve_ammo < 0 {
                        needed
                    } else {
                        needed.min(w.stats.reserve_ammo as u32)
                    };
                    w.stats.current_clip += can_load;
                    if w.stats.reserve_ammo > 0 {
                        w.stats.reserve_ammo -= can_load as i32;
                    }
                    w.behavior.on_perfect_reload(&mut w.stats, &mut ctx);
                    if self.player.body.modifiers.ult_hyper_reactor {
                        ctx.trigger_explosion(self.player.body.pos, 230.0, 260.0);
                    }
                    self.player.reload_state.is_reloading = false;
                    self.flush_commands(ctx.commands);
                    if self.player.body.modifiers.ult_chrono_legion {
                        self.spawn_heavy_mech_ally(self.player.body.pos + vec2(45.0, 0.0));
                    }
                    self.float_texts.push(FloatText {
                        pos: self.player.body.pos - vec2(0.0, 48.0),
                        text: tr("NẠP ĐẠN HOÀN HẢO! +40% DMG", "PERFECT RELOAD! +40% DMG")
                            .to_string(),
                        color: Color::new(0.0, 1.0, 0.4, 1.0),
                        size: 24.0,
                        life: 1.2,
                    });
                } else {
                    self.player.reload_state.timer =
                        (self.player.reload_state.timer - 0.35).max(0.0);
                    self.float_texts.push(FloatText {
                        pos: self.player.body.pos - vec2(0.0, 48.0),
                        text: tr("TRƯỢT NHỊP NẠP ĐẠN!", "RELOAD MISSED!").to_string(),
                        color: Color::new(1.0, 0.25, 0.25, 1.0),
                        size: 20.0,
                        life: 0.9,
                    });
                }
            }
        }

        if self.player.reload_state.is_reloading {
            self.player.reload_state.timer += dt;
            if self.player.reload_state.timer >= self.player.reload_state.duration {
                let mut ctx = self.make_context();
                let w = self.player.current_weapon_mut();
                let needed = w.stats.clip_size - w.stats.current_clip;
                let can_load = if w.stats.reserve_ammo < 0 {
                    needed
                } else {
                    needed.min(w.stats.reserve_ammo as u32)
                };
                w.stats.current_clip += can_load;
                if w.stats.reserve_ammo > 0 {
                    w.stats.reserve_ammo -= can_load as i32;
                }
                self.player.reload_state.is_reloading = false;
                if self.player.body.modifiers.has("reload_3") {
                    self.player.body.modifiers.free_ammo_timer = 3.5;
                    self.player.body.active_overcharge = true;
                } else {
                    self.player.body.active_overcharge = false;
                }
                if self.player.body.modifiers.ult_hyper_reactor {
                    ctx.trigger_explosion(self.player.body.pos, 210.0, 210.0);
                }
                self.flush_commands(ctx.commands);
                if self.player.body.modifiers.ult_chrono_legion {
                    self.spawn_heavy_mech_ally(self.player.body.pos + vec2(45.0, 0.0));
                }
            }
        }
    }

    fn perform_melee(&mut self) {
        self.add_screen_shake(4.0);
        let slash_angle = self.player.body.angle;
        let p_pos = self.player.body.pos;
        let mut ctx = self.make_context();

        ctx.spawn_event(Box::new(ShockwaveRingEvent {
            center: p_pos + vec2(slash_angle.cos(), slash_angle.sin()) * 32.0,
            current_radius: 10.0,
            max_radius: 90.0,
            speed: 550.0,
            color: Color::new(0.0, 0.95, 1.0, 0.85),
            thickness: 4.5,
        }));

        for e in self.enemies.iter_mut() {
            let to_enemy = e.body.pos - p_pos;
            let dist = to_enemy.length();
            if dist < 95.0 + e.body.radius {
                let angle_diff = (slash_angle - to_enemy.y.atan2(to_enemy.x)).abs();
                if angle_diff < 0.95 || angle_diff > (PI * 2.0 - 0.95) {
                    let dealt = e.take_damage_at(
                        e.body.pos,
                        85.0,
                        to_enemy.normalize_or_zero() * 560.0,
                        Some(StatusEffect::Stun { duration: 0.45 }),
                        &mut ctx,
                    );
                    if self.settings.show_damage_numbers {
                        self.float_texts.push(FloatText {
                            pos: e.body.pos,
                            text: format!("{:.0} SLASH!", dealt),
                            color: Color::new(1.0, 0.84, 0.0, 1.0),
                            size: 20.0,
                            life: 0.8,
                        });
                    }
                }
            }
        }
        self.flush_commands(ctx.commands);
    }

    fn fire_current_weapon(&mut self) {
        let clip = self.player.current_weapon().stats.current_clip;
        let reserve = self.player.current_weapon().stats.reserve_ammo;
        let reload_spd = self.player.body.modifiers.reload_speed_multi.max(0.2);
        let reload_dur = (self.player.current_weapon().stats.reload_duration / reload_spd).max(0.18);

        if clip > 0 {
            let origin = self.player.body.pos;
            let aim = self.player.body.angle;
            let mods = self.player.body.modifiers.clone();
            let overcharged = self.player.body.active_overcharge;

            let mut ctx = self.make_context();
            let w = self.player.current_weapon_mut();
            if !mods.infinite_ammo && mods.free_ammo_timer <= 0.0 {
                w.stats.current_clip -= 1;
            }
            let level_rof_bonus = 1.0 + (w.stats.level.saturating_sub(1) as f32) * 0.025;
            let fire_rate = (w.stats.fire_rate / (mods.fire_rate_multi * level_rof_bonus).max(0.2))
                .max(0.02);
            let shake = w.stats.screen_shake;
            let stats_clone = w.stats.clone();
            w.behavior
                .fire(origin, aim, &stats_clone, &mods, overcharged, &mut ctx);

            // 1. Pháo Đuôi Phản Lực (Rear Cannons - 180° sau lưng đẩy lùi kẻ bám đuôi)
            if mods.multi_rear_shot {
                let rear_aim = aim + PI;
                let rear_dir = vec2(rear_aim.cos(), rear_aim.sin());
                ctx.spawn_player_projectile(Projectile {
                    pos: origin + rear_dir * 26.0,
                    vel: rear_dir * (stats_clone.speed * 0.95),
                    damage: stats_clone.damage * 0.85 * mods.dmg_multi,
                    color: Color::new(1.0, 0.45, 0.85, 1.0),
                    radius: 7.0 * mods.bullet_size_multi,
                    explosion_radius: Some(40.0),
                    spawns_black_hole: false,
                    pierce: 2 + mods.pierce_count,
                    bounces: if mods.bouncing_bullets { 2 } else { 0 },
                    life: 1.8,
                    status_effect: None,
                });
            }

            // 2. Pháo Mạn Cánh Tả Hữu (Twin Flank Batteries - ±90° hai bên sườn)
            if mods.multi_flank_shot {
                for side_angle in [aim + PI * 0.5, aim - PI * 0.5] {
                    let flank_dir = vec2(side_angle.cos(), side_angle.sin());
                    ctx.spawn_player_projectile(Projectile {
                        pos: origin + flank_dir * 24.0,
                        vel: flank_dir * (stats_clone.speed * 0.9),
                        damage: stats_clone.damage * 0.75 * mods.dmg_multi,
                        color: Color::new(0.4, 0.9, 1.0, 1.0),
                        radius: 6.0 * mods.bullet_size_multi,
                        explosion_radius: None,
                        spawns_black_hole: false,
                        pierce: 1 + mods.pierce_count,
                        bounces: if mods.bouncing_bullets { 2 } else { 0 },
                        life: 1.6,
                        status_effect: None,
                    });
                }
            }

            // 3. Bão Đạn Hoa Sen 360° Mỗi 4 Phát Bắn (Omni Starburst Matrix)
            if mods.multi_omni_spiral && self.shots_fired % 4 == 0 {
                for i in 0..8 {
                    let a = aim + (i as f32) * (PI * 2.0 / 8.0);
                    let star_dir = vec2(a.cos(), a.sin());
                    ctx.spawn_player_projectile(Projectile {
                        pos: origin + star_dir * 28.0,
                        vel: star_dir * (stats_clone.speed * 1.05),
                        damage: stats_clone.damage * 0.9 * mods.dmg_multi,
                        color: Color::new(1.0, 0.85, 0.2, 1.0),
                        radius: 7.5 * mods.bullet_size_multi,
                        explosion_radius: Some(55.0),
                        spawns_black_hole: false,
                        pierce: 3 + mods.pierce_count,
                        bounces: 2,
                        life: 2.0,
                        status_effect: None,
                    });
                }
            }

            // 4. Ma Trận Hủy Diệt Toàn Diện (Omni Annihilation Matrix - 16 Hướng 360° Mọi Phát Bắn)
            if mods.ult_omni_annihilation {
                for i in 0..16 {
                    let a = aim + (i as f32) * (PI * 2.0 / 16.0);
                    let dir = vec2(a.cos(), a.sin());
                    ctx.spawn_player_projectile(Projectile {
                        pos: origin + dir * 30.0,
                        vel: dir * 880.0,
                        damage: stats_clone.damage * 1.25 * mods.dmg_multi,
                        color: Color::new(1.0, 0.25, 0.75, 1.0),
                        radius: 8.5 * mods.bullet_size_multi,
                        explosion_radius: Some(75.0),
                        spawns_black_hole: false,
                        pierce: 4 + mods.pierce_count,
                        bounces: 3,
                        life: 2.2,
                        status_effect: None,
                    });
                }
            }

            if mods.ult_volt_overdrive && self.shots_fired % 4 == 0 {
                let aim_dir = vec2(aim.cos(), aim.sin());
                let bolt_pos = origin + aim_dir * 40.0;
                ctx.chain_lightning(bolt_pos, aim_dir, 8, 340.0, 95.0 * mods.dmg_multi);
            }

            self.player.fire_timer = fire_rate;
            self.add_screen_shake(shake * 0.5);
            self.sound_bank
                .play_weapon_fire(self.player.current_weapon_idx, self.settings.master_volume);
            self.shots_fired += 1;

            self.casings.push(Casing {
                pos: origin,
                vel: vec2((aim - PI / 2.0).cos(), (aim - PI / 2.0).sin())
                    * rand::gen_range(80.0, 160.0),
                rotation: rand::gen_range(0.0, PI * 2.0),
                v_rot: rand::gen_range(-10.0, 10.0),
                life: 5.0,
            });

            self.flush_commands(ctx.commands);
        } else if self.player.body.modifiers.ult_autonomous_arsenal {
            // Kho Vũ Khí Tự Hành: Tự động nạp đầy băng đạn tức thì không cần chờ!
            let w = self.player.current_weapon_mut();
            w.stats.current_clip = w.stats.clip_size;
            self.player.body.active_overcharge = true;
        } else if !self.player.reload_state.is_reloading && reserve != 0 {
            self.player.reload_state.is_reloading = true;
            self.player.reload_state.timer = 0.0;
            self.player.reload_state.duration = reload_dur;
        }
    }

    fn cleanup_dead_enemies(&mut self) {
        let mut death_ctx = self.make_context();
        let mut i = 0;
        while i < self.enemies.len() {
            if self.enemies[i].body.hp <= 0.0 {
                let mut dead = self.enemies.swap_remove(i);
                self.total_kills += 1;
                self.combo += 1;
                self.combo_timer = 3.5;
                self.score += dead.body.score_reward * (self.combo as u64);

                // Hút máu Vampiric & Nạp đạn khi hạ quái (Ammo on Kill)
                if self.player.body.modifiers.vampiric {
                    self.player.body.hp =
                        (self.player.body.hp + 3.0).min(self.player.body.max_hp);
                    self.player.body.shield =
                        (self.player.body.shield + 3.0).min(self.player.body.max_shield);
                }
                let ammo_gain = self.player.body.modifiers.ammo_on_kill;
                if ammo_gain > 0 {
                    let w = self.player.current_weapon_mut();
                    w.stats.current_clip =
                        (w.stats.current_clip + ammo_gain).min(w.stats.clip_size);
                }

                // Quái chết rơi ra Tinh Thể Kinh Nghiệm (ExpOrb) để người chơi di chuyển tới nhặt!
                if dead.body.is_boss {
                    for k in 0..5 {
                        let a = (k as f32) * (PI * 2.0 / 5.0) + rand::gen_range(-0.2, 0.2);
                        self.exp_orbs.push(ExpOrb {
                            pos: dead.body.pos,
                            vel: vec2(a.cos(), a.sin()) * rand::gen_range(80.0, 150.0),
                            value: 25.0,
                            tier: 2,
                            pulse: rand::gen_range(0.0, PI * 2.0),
                            magnetized: false,
                        });
                    }
                } else if dead.body.max_hp >= 120.0 {
                    let a = rand::gen_range(0.0, PI * 2.0);
                    self.exp_orbs.push(ExpOrb {
                        pos: dead.body.pos,
                        vel: vec2(a.cos(), a.sin()) * rand::gen_range(35.0, 75.0),
                        value: 14.0,
                        tier: 1,
                        pulse: rand::gen_range(0.0, PI * 2.0),
                        magnetized: false,
                    });
                } else {
                    let a = rand::gen_range(0.0, PI * 2.0);
                    self.exp_orbs.push(ExpOrb {
                        pos: dead.body.pos,
                        vel: vec2(a.cos(), a.sin()) * rand::gen_range(25.0, 55.0),
                        value: 5.0,
                        tier: 0,
                        pulse: rand::gen_range(0.0, PI * 2.0),
                        magnetized: false,
                    });
                }

                self.player
                    .class
                    .on_enemy_killed(&mut self.player.body, &mut death_ctx);
                dead.behavior.on_death(&dead.body, &mut death_ctx);

                // Rơi Lõi Nâng Cấp Súng để người chơi tích lũy mở khóa các Form tới Lv.500 [Tối Thượng]!
                if dead.body.is_boss {
                    death_ctx.spawn_weapon_drop(WeaponDrop::new(
                        dead.body.pos,
                        self.selected_weapon_id,
                        WeaponRarity::Ultimate,
                    ));
                    death_ctx.spawn_weapon_drop(WeaponDrop::new(
                        dead.body.pos + vec2(25.0, 0.0),
                        self.selected_weapon_id,
                        WeaponRarity::Mythic,
                    ));
                } else if self.weapon_drops.len() < 8 {
                    if dead.body.max_hp >= 120.0 && rand::gen_range(0.0, 1.0) < 0.14 {
                        let roll = rand::gen_range(0.0, 1.0);
                        let r = if roll < 0.20 {
                            WeaponRarity::Legendary
                        } else if roll < 0.60 {
                            WeaponRarity::Epic
                        } else {
                            WeaponRarity::Rare
                        };
                        death_ctx.spawn_weapon_drop(WeaponDrop::new(
                            dead.body.pos,
                            self.selected_weapon_id,
                            r,
                        ));
                    } else if rand::gen_range(0.0, 1.0) < 0.045 {
                        let r = if rand::gen_range(0.0, 1.0) < 0.30 {
                            WeaponRarity::Rare
                        } else {
                            WeaponRarity::Common
                        };
                        death_ctx.spawn_weapon_drop(WeaponDrop::new(
                            dead.body.pos,
                            self.selected_weapon_id,
                            r,
                        ));
                    }
                }

                self.decals.push(Decal {
                    pos: dead.body.pos,
                    radius: dead.body.radius * 0.9,
                    color: Color::new(
                        dead.body.color.r,
                        dead.body.color.g,
                        dead.body.color.b,
                        0.32,
                    ),
                    life: 14.0,
                    is_damage_zone: false,
                });

                // Cân bằng tỷ lệ rơi đồ cứu viện (Hồi máu Medkit, Hộp đạn Ammo, Nam châm Magnet):
                // Boss rơi chắc chắn 2 món; quái Tinh Anh 18%; quái thường 6.5%; tối đa 10 món trên sàn
                let pickup_roll = rand::gen_range(0.0, 1.0);
                let should_spawn_pickup = self.pickups.len() < 10
                    && (dead.body.is_boss
                        || (dead.body.max_hp >= 120.0 && pickup_roll < 0.18)
                        || pickup_roll < 0.065);

                if should_spawn_pickup {
                    let hp_ratio = self.player.body.hp / self.player.body.max_hp;
                    let roll = rand::gen_range(0, 100);

                    // Thuật toán nhạy thông minh: Nếu người chơi sắp cạn máu (<45%), ưu tiên cao rơi Hồi Máu Medkit!
                    let ptype = if hp_ratio < 0.45 {
                        if roll < 45 {
                            PickupType::Medkit
                        } else if roll < 70 {
                            PickupType::Ammo
                        } else if roll < 85 {
                            PickupType::Magnet
                        } else if roll < 93 {
                            PickupType::Overcharge
                        } else {
                            PickupType::Nuke
                        }
                    } else {
                        // Tỷ lệ phân bổ chuẩn: Đảm bảo Hồi máu, Hộp đạn và Nam châm luôn phong phú
                        if roll < 28 {
                            PickupType::Medkit
                        } else if roll < 56 {
                            PickupType::Ammo
                        } else if roll < 78 {
                            PickupType::Magnet
                        } else if roll < 89 {
                            PickupType::Overcharge
                        } else {
                            PickupType::Nuke
                        }
                    };

                    self.pickups.push(Pickup {
                        pos: dead.body.pos,
                        pickup_type: ptype,
                        radius: 12.0,
                        pulse: 0.0,
                    });

                    // Nếu là Boss bị hạ gục, rơi thêm 1 hộp trang bị bổ trợ thứ 2!
                    if dead.body.is_boss && self.pickups.len() < 10 {
                        self.pickups.push(Pickup {
                            pos: dead.body.pos + vec2(-25.0, 15.0),
                            pickup_type: if rand::gen_range(0.0, 1.0) < 0.5 {
                                PickupType::Magnet
                            } else {
                                PickupType::Ammo
                            },
                            radius: 12.0,
                            pulse: 0.0,
                        });
                    }
                }
            } else {
                i += 1;
            }
        }
        if !death_ctx.commands.is_empty() {
            self.flush_commands(death_ctx.commands);
        }
    }

    fn update_exp_orbs(&mut self, dt: f32) {
        let p_pos = self.player.body.pos;
        let p_rad = self.player.body.radius;
        let magnet_rad = 95.0 * self.player.body.modifiers.magnet_radius_multi;
        let exp_multi = self.player.body.modifiers.exp_gain_multi;
        let shield_on_orb = self.player.body.modifiers.magnet_shield_on_orb;
        let arc_on_orb = self.player.body.modifiers.magnet_arc_on_orb;
        let dmg_multi = self.player.body.modifiers.dmg_multi;

        let mut zap_count = 0usize;
        let mut i = 0;
        while i < self.exp_orbs.len() {
            let orb_vel = self.exp_orbs[i].vel;
            self.exp_orbs[i].pulse += dt * 6.5;
            self.exp_orbs[i].pos += orb_vel * dt;
            self.exp_orbs[i].vel *= 0.88;

            let to_p = p_pos - self.exp_orbs[i].pos;
            let dist = to_p.length();

            if dist <= magnet_rad || self.exp_orbs[i].magnetized {
                self.exp_orbs[i].magnetized = true;
                let pull_spd = 520.0 + 180.0 * self.player.body.modifiers.magnet_radius_multi;
                self.exp_orbs[i].pos += to_p.normalize_or_zero() * pull_spd * dt;
            }

            if (p_pos - self.exp_orbs[i].pos).length() <= p_rad + 14.0 {
                let orb = self.exp_orbs.swap_remove(i);
                let gain = orb.value * exp_multi;
                self.tech_exp += gain;

                if shield_on_orb {
                    self.player.body.shield =
                        (self.player.body.shield + 2.0).min(self.player.body.max_shield);
                }
                if arc_on_orb && rand::gen_range(0.0, 1.0) < 0.35 {
                    zap_count += 1;
                }

                // Lên cấp với đường cong kinh nghiệm tăng dần: càng lên cấp cao càng tốn nhiều EXP!
                while self.tech_exp >= self.next_tech_exp {
                    self.tech_exp -= self.next_tech_exp;
                    self.tech_level += 1;
                    let lv = self.tech_level as f32;
                    self.next_tech_exp = 180.0 + lv * 110.0 + lv * lv * 32.0 + (lv * 0.15).powf(3.0) * 120.0;
                    self.pending_upgrades += 1;
                }
            } else {
                i += 1;
            }
        }

        if zap_count > 0 && !self.enemies.is_empty() {
            let mut ctx = self.make_context();
            for _ in 0..zap_count.min(3) {
                let aim_dir = vec2(self.player.body.angle.cos(), self.player.body.angle.sin());
                ctx.chain_lightning(p_pos, aim_dir, 3, 280.0, 52.0 * dmg_multi);
            }
            self.flush_commands(ctx.commands);
        }
    }

    fn update_pickups(&mut self, dt: f32) {
        let p_pos = self.player.body.pos;
        let p_rad = self.player.body.radius;
        let item_magnet_rad = 70.0 * self.player.body.modifiers.magnet_radius_multi;
        let mut nuke_shake = 0.0;
        let mut vacuum_all_orbs = false;

        for p in self.pickups.iter_mut() {
            p.pulse += dt * 5.0;
            let to_p = p_pos - p.pos;
            let dist = to_p.length();
            if dist < item_magnet_rad && dist > 1.0 {
                p.pos += to_p.normalize_or_zero() * 360.0 * dt;
            }
            if (p.pos - p_pos).length() < p.radius + p_rad {
                match p.pickup_type {
                    PickupType::Medkit => {
                        self.player.body.hp =
                            (self.player.body.hp + 35.0).min(self.player.body.max_hp);
                        self.float_texts.push(FloatText {
                            pos: p_pos,
                            text: "+35 HP".to_string(),
                            color: GREEN,
                            size: 20.0,
                            life: 1.0,
                        });
                    }
                    PickupType::Ammo => {
                        let w = self.player.current_weapon_mut();
                        w.stats.current_clip = w.stats.clip_size;
                        self.player.reload_state.is_reloading = false;
                        self.float_texts.push(FloatText {
                            pos: p_pos,
                            text: tr("NẠP ĐẦY BĂNG ĐẠN TỨC THÌ!", "INSTANT FULL AMMO!")
                                .to_string(),
                            color: GOLD,
                            size: 20.0,
                            life: 1.0,
                        });
                    }
                    PickupType::Nuke => {
                        nuke_shake += 10.0;
                        for e in self.enemies.iter_mut() {
                            if !e.body.is_boss {
                                e.body.hp = 0.0;
                            } else {
                                e.body.hp -= 500.0;
                            }
                        }
                        self.float_texts.push(FloatText {
                            pos: p_pos,
                            text: tr("KÍCH HOẠT ĐẦU ĐẠN HẠT NHÂN!", "NUKE ACTIVATED!")
                                .to_string(),
                            color: RED,
                            size: 26.0,
                            life: 1.5,
                        });
                    }
                    PickupType::Overcharge => {
                        self.player.body.active_overcharge = true;
                        self.float_texts.push(FloatText {
                            pos: p_pos,
                            text: tr("CƯỜNG HÓA OVERCHARGE!", "OVERCHARGE BUFF!").to_string(),
                            color: SKYBLUE,
                            size: 23.0,
                            life: 1.2,
                        });
                    }
                    PickupType::Magnet => {
                        vacuum_all_orbs = true;
                        self.float_texts.push(FloatText {
                            pos: p_pos - vec2(0.0, 28.0),
                            text: tr(
                                "SIÊU NAM CHÂM: HÚT TOÀN BỘ EXP!",
                                "SUPER MAGNET: VACUUM ALL EXP!",
                            )
                            .to_string(),
                            color: Color::new(0.0, 1.0, 0.9, 1.0),
                            size: 23.0,
                            life: 1.4,
                        });
                    }
                }
            }
        }
        if vacuum_all_orbs {
            for orb in self.exp_orbs.iter_mut() {
                orb.magnetized = true;
            }
        }
        if nuke_shake > 0.0 {
            self.add_screen_shake(nuke_shake);
        }
        self.pickups
            .retain(|p| (p.pos - p_pos).length() >= p.radius + p_rad);
    }

    /// Tính toán góc ngắm thông minh (Aim Assist / Auto-Aim) và vị trí mục tiêu đang bị khóa tâm
    fn compute_smart_aim(&self, mouse_world: Vec2) -> (f32, Option<Vec2>) {
        let p_pos = self.player.body.pos;
        let raw_angle = (mouse_world.y - p_pos.y).atan2(mouse_world.x - p_pos.x);

        if self.settings.aim_assist_mode == 0 || self.enemies.is_empty() {
            return (raw_angle, None);
        }

        let raw_dir = vec2(raw_angle.cos(), raw_angle.sin());
        let max_range = 720.0f32;
        let mut best_target: Option<(Vec2, Vec2)> = None;
        let mut best_score = f32::MAX;

        for e in &self.enemies {
            // Ưu tiên kiểm tra cả các Bộ Phận Boss (BossPart) lẫn thân quái
            let mut candidates = vec![(e.body.pos, e.body.vel)];
            for part in &e.body.parts {
                candidates.push((e.body.pos + part.offset, e.body.vel));
            }

            for (t_pos, t_vel) in candidates {
                let to_t = t_pos - p_pos;
                let dist = to_t.length();
                if dist > max_range || dist < 1.0 {
                    continue;
                }
                let dir_t = to_t / dist;
                let dot = raw_dir.dot(dir_t);
                let cursor_dist = (t_pos - mouse_world).length();

                if self.settings.aim_assist_mode == 1 {
                    // Chế độ 1 (SMART MAGNETIC AIM): Khóa mục tiêu trong góc nón ~55 độ quanh hướng chuột,
                    // hoặc gần con trỏ chuột (<170px), hoặc quái áp sát nguy hiểm (<190px)
                    let in_cone = dot >= 0.57;
                    let near_cursor = cursor_dist <= 170.0;
                    let close_danger = dist <= 190.0 && dot >= -0.15;

                    if in_cone || near_cursor || close_danger {
                        let angle_penalty = (1.0 - dot).max(0.0) * 420.0;
                        let score = dist * 0.55 + cursor_dist * 0.45 + angle_penalty;
                        if score < best_score {
                            best_score = score;
                            best_target = Some((t_pos, t_vel));
                        }
                    }
                } else {
                    // Chế độ 2 (FULL 360 AUTO-AIM): Tự động khóa mục tiêu nguy hiểm/gần nhất 360 độ
                    let dir_bonus = (1.0 - dot) * 85.0;
                    let score = dist + dir_bonus;
                    if score < best_score {
                        best_score = score;
                        best_target = Some((t_pos, t_vel));
                    }
                }
            }
        }

        if let Some((t_pos, t_vel)) = best_target {
            let dist = (t_pos - p_pos).length();
            // Dự đoán hướng di chuyển của mục tiêu (Target Velocity Leading)
            let lead_pos = t_pos + t_vel * (dist / 1150.0).min(0.24);
            let aim_angle = (lead_pos.y - p_pos.y).atan2(lead_pos.x - p_pos.x);
            (aim_angle, Some(t_pos))
        } else {
            (raw_angle, None)
        }
    }

    pub fn render_world(&self) -> bool {
        clear_background(Color::new(0.05, 0.06, 0.08, 1.0));

        // Rung màn hình dạng sóng điều hòa mượt (không nhảy giật ngẫu nhiên gây hoa mắt)
        let shake_offset =
            if self.screen_shake > 0.01 && self.settings.screen_shake_intensity > 0.001 {
                let t = get_time() as f32;
                vec2(
                    (t * 42.0).sin() * self.screen_shake,
                    (t * 53.0).cos() * self.screen_shake,
                )
            } else {
                Vec2::ZERO
            };

        let cam_offset =
            vec2(screen_width() / 2.0, screen_height() / 2.0) - self.camera_pos + shake_offset;

        // 1. Arena Grid
        if self.settings.grid_opacity > 0.01 {
            let grid_size = 80.0;
            let start_x = (cam_offset.x % grid_size) - grid_size;
            let start_y = (cam_offset.y % grid_size) - grid_size;
            let grid_col = Color::new(0.0, 0.8, 1.0, 0.08 * self.settings.grid_opacity);
            let mut x = start_x;
            while x < screen_width() + grid_size {
                draw_line(x, 0.0, x, screen_height(), 1.0, grid_col);
                x += grid_size;
            }
            let mut y = start_y;
            while y < screen_height() + grid_size {
                draw_line(0.0, y, screen_width(), y, 1.0, grid_col);
                y += grid_size;
            }
        }

        // Outer Boundary
        draw_rectangle_lines(
            cam_offset.x,
            cam_offset.y,
            WORLD_WIDTH,
            WORLD_HEIGHT,
            4.0,
            Color::new(0.0, 0.94, 1.0, 0.8),
        );

        // Decals
        for d in &self.decals {
            let sp = d.pos + cam_offset;
            draw_circle(sp.x, sp.y, d.radius, d.color);
        }

        // Casings
        for c in &self.casings {
            let sp = c.pos + cam_offset;
            draw_circle(sp.x, sp.y, 2.0, Color::new(0.85, 0.7, 0.2, 0.8));
        }

        // Explosive Barrels
        for b in &self.barrels {
            b.draw(cam_offset);
        }

        // Tactical Battlefield Structures (Radar Outposts, Energy Bunkers, Repair Pylons)
        for st in &self.structures {
            st.draw(cam_offset);
        }

        // Weapon Drops (Ground Loot with Hologram Beams)
        for wd in &self.weapon_drops {
            wd.draw(cam_offset, self.player.body.pos);
        }

        // Deployables
        for dep in &self.deployables {
            dep.draw(cam_offset);
        }

        // Allied Units
        for ally in &self.allies {
            ally.draw(cam_offset, self.settings.show_ally_bars);
        }

        // Pickups (Vẽ biểu tượng đặc trưng rõ ràng cho từng loại vật phẩm - Không dùng hình tròn!)
        for p in &self.pickups {
            let sp = p.pos + cam_offset;
            let bob = p.pulse.sin() * 2.5;
            let py = sp.y + bob;
            match p.pickup_type {
                PickupType::Medkit => {
                    // Hộp Cứu Thương Quân Y: Hộp vuông trắng viền xanh lá + Chữ thập Đỏ/Xanh nổi bật
                    draw_rectangle(sp.x - 11.0, py - 9.0, 22.0, 18.0, Color::new(0.92, 0.98, 0.94, 0.96));
                    draw_rectangle_lines(sp.x - 11.0, py - 9.0, 22.0, 18.0, 2.0, Color::new(0.0, 0.95, 0.45, 1.0));
                    draw_rectangle(sp.x - 2.5, py - 6.0, 5.0, 12.0, Color::new(0.95, 0.15, 0.25, 1.0));
                    draw_rectangle(sp.x - 6.0, py - 2.5, 12.0, 5.0, Color::new(0.95, 0.15, 0.25, 1.0));
                }
                PickupType::Ammo => {
                    // Hòm Đạn Tiếp Tế: Thùng chữ nhật vàng đồng + 3 viên đạn nhọn xếp dọc
                    draw_rectangle(sp.x - 12.0, py - 9.0, 24.0, 18.0, Color::new(0.24, 0.18, 0.05, 0.96));
                    draw_rectangle_lines(sp.x - 12.0, py - 9.0, 24.0, 18.0, 2.0, GOLD);
                    for bx_off in [-5.5f32, 0.0, 5.5] {
                        draw_rectangle(sp.x + bx_off - 1.5, py - 3.0, 3.0, 8.0, GOLD);
                        draw_triangle(
                            vec2(sp.x + bx_off, py - 7.0),
                            vec2(sp.x + bx_off - 1.8, py - 3.0),
                            vec2(sp.x + bx_off + 1.8, py - 3.0),
                            WHITE,
                        );
                    }
                }
                PickupType::Nuke => {
                    // Đầu Đạn Hạt Nhân: Hình quả tên lửa cánh đuôi hướng lên + nhãn NUKE
                    draw_triangle(
                        vec2(sp.x, py - 14.0),
                        vec2(sp.x - 8.0, py + 6.0),
                        vec2(sp.x + 8.0, py + 6.0),
                        Color::new(1.0, 0.45, 0.0, 1.0),
                    );
                    draw_triangle_lines(
                        vec2(sp.x, py - 14.0),
                        vec2(sp.x - 8.0, py + 6.0),
                        vec2(sp.x + 8.0, py + 6.0),
                        2.0,
                        YELLOW,
                    );
                    draw_rectangle(sp.x - 11.0, py + 5.0, 22.0, 4.0, RED);
                }
                PickupType::Overcharge => {
                    // Lõi Năng Lượng Cường Hóa: Ngôi sao 4 cánh pha lê điện xoay
                    draw_poly(sp.x, py, 4, 13.0, p.pulse * 25.0, Color::new(0.0, 0.94, 1.0, 0.92));
                    draw_poly(sp.x, py, 4, 10.0, p.pulse * 25.0 + 45.0, WHITE);
                }
                PickupType::Magnet => {
                    // Siêu Nam Châm Hút EXP: Hình chữ U hai cực Đỏ - Xanh Lam phát tia từ trường
                    draw_rectangle(sp.x - 10.0, py - 9.0, 6.0, 16.0, Color::new(1.0, 0.2, 0.3, 1.0));
                    draw_rectangle(sp.x + 4.0, py - 9.0, 6.0, 16.0, Color::new(0.1, 0.65, 1.0, 1.0));
                    draw_rectangle(sp.x - 10.0, py + 3.0, 20.0, 6.0, Color::new(0.75, 0.82, 0.9, 1.0));
                    draw_rectangle(sp.x - 10.0, py - 10.0, 6.0, 4.0, WHITE);
                    draw_rectangle(sp.x + 4.0, py - 10.0, 6.0, 4.0, WHITE);
                }
            }
        }

        // Tinh Thể Kinh Nghiệm (EXP Orbs) rơi ra từ xác quái (Hình Pha Lê Kim Cương / Lục Giác / Sao Vàng)
        for orb in &self.exp_orbs {
            let sp = orb.pos + cam_offset;
            let rot = orb.pulse * 24.0;
            match orb.tier {
                2 => {
                    // Lõi Kinh Nghiệm Đại Boss (Sao Vàng Hoàng Kim 8 cạnh)
                    draw_poly(sp.x, sp.y, 4, 11.0, rot, Color::new(1.0, 0.85, 0.1, 0.96));
                    draw_poly(sp.x, sp.y, 4, 11.0, rot + 45.0, Color::new(1.0, 0.55, 0.0, 0.92));
                    draw_poly(sp.x, sp.y, 4, 5.5, rot, WHITE);
                }
                1 => {
                    // Tinh Thể Tinh Anh (Lục Giác Tím Neon)
                    draw_poly(sp.x, sp.y, 6, 8.5, rot, Color::new(0.82, 0.3, 1.0, 0.95));
                    draw_poly_lines(sp.x, sp.y, 6, 9.5, rot, 1.5, WHITE);
                    draw_poly(sp.x, sp.y, 6, 4.2, -rot, Color::new(0.95, 0.85, 1.0, 1.0));
                }
                _ => {
                    // Mảnh Pha Lê Kinh Nghiệm Thường (Hình Thoi Lam Ngọc Sắc Nét)
                    draw_poly(sp.x, sp.y, 4, 6.8, rot, Color::new(0.0, 0.95, 0.88, 0.94));
                    draw_poly(sp.x, sp.y, 4, 3.2, rot, WHITE);
                }
            }
        }

        // Battlefield Events (Black Holes, Orbital Lasers, Carpet Bombs, Telegraphs, Lightning Arcs)
        for ev in &self.events {
            ev.draw(cam_offset);
        }

        // Particles
        for p in &self.particles {
            let sp = p.pos + cam_offset;
            let alpha = (p.life / p.max_life).clamp(0.0, 1.0);
            draw_poly(
                sp.x,
                sp.y,
                4,
                p.radius,
                p.life * 180.0,
                Color::new(p.color.r, p.color.g, p.color.b, alpha),
            );
        }

        // Enemies & Multi-Part Bosses (Frustum culling tối ưu cho bầy quái 10,000 con)
        let scr_w = screen_width();
        let scr_h = screen_height();
        let cull_margin = 65.0;
        for e in &self.enemies {
            let sp = e.body.pos + cam_offset;
            if sp.x < -cull_margin
                || sp.x > scr_w + cull_margin
                || sp.y < -cull_margin
                || sp.y > scr_h + cull_margin
            {
                continue;
            }
            e.draw(cam_offset);
        }

        // Vẽ Tia Laser Ngắm & Khung Khóa Mục Tiêu (Target Lock-On Reticle) khi Aim Assist khóa mục tiêu!
        let p_sp = self.player.body.pos + cam_offset;
        let gun_dir = vec2(self.player.body.angle.cos(), self.player.body.angle.sin());
        let gun_right = vec2(-gun_dir.y, gun_dir.x);

        if let Some(lock_world) = self.locked_target_pos {
            let lock_sp = lock_world + cam_offset;
            // Tia Laser dẫn đường từ mũi súng tới mục tiêu đang khóa
            draw_line(
                p_sp.x + gun_dir.x * 22.0,
                p_sp.y + gun_dir.y * 22.0,
                lock_sp.x,
                lock_sp.y,
                1.6,
                Color::new(0.0, 1.0, 0.65, 0.35),
            );

            // Khung khóa mục tiêu 4 góc [ + ] xoay quanh mục tiêu
            let bracket_r = 22.0 + ((get_time() as f32) * 8.0).sin() * 3.0;
            let b_col = Color::new(0.0, 1.0, 0.65, 0.95);
            let corner_len = 8.0;
            for (sx, sy) in [(-1.0f32, -1.0f32), (1.0, -1.0), (-1.0, 1.0), (1.0, 1.0)] {
                let cx = lock_sp.x + sx * bracket_r;
                let cy = lock_sp.y + sy * bracket_r;
                draw_line(cx, cy, cx - sx * corner_len, cy, 2.5, b_col);
                draw_line(cx, cy, cx, cy - sy * corner_len, 2.5, b_col);
            }
            draw_poly_lines(
                lock_sp.x,
                lock_sp.y,
                4,
                12.0,
                (get_time() as f32) * 90.0,
                1.8,
                Color::new(1.0, 0.9, 0.1, 0.9),
            );
        } else {
            // Tia ngắm laser ngắn khi ngắm thủ công
            draw_line(
                p_sp.x + gun_dir.x * 22.0,
                p_sp.y + gun_dir.y * 22.0,
                p_sp.x + gun_dir.x * 160.0,
                p_sp.y + gun_dir.y * 160.0,
                1.2,
                Color::new(0.0, 0.94, 1.0, 0.22),
            );
        }

        // Vòng Từ Trường Thu Hồi Kinh Nghiệm (Magnetic Pickup Field Ring) quanh người chơi
        let magnet_r = 95.0 * self.player.body.modifiers.magnet_radius_multi;
        draw_poly_lines(
            p_sp.x,
            p_sp.y,
            24,
            magnet_r,
            (get_time() as f32) * 18.0,
            if self.player.body.modifiers.magnet_radius_multi > 1.05 {
                1.4
            } else {
                1.0
            },
            Color::new(0.0, 1.0, 0.85, 0.16),
        );

        // Player (Mô hình Chiến Binh Giáp Cơ Khí Top-Down có Ba-lô Phản Lực, Cầu Vai Giáp, Mũ Visor & Vũ Khí)
        if self.player.body.shield > 0.0 {
            draw_poly_lines(
                p_sp.x,
                p_sp.y,
                6,
                self.player.body.radius + 6.0,
                self.player.body.angle * 57.3,
                2.0,
                Color::new(0.0, 0.94, 1.0, 0.55),
            );
        }
        // Khiên Bảo Hộ Bất Tử (Invulnerability Shield Aura) khi vừa nâng cấp xong
        if self.player.body.invulnerable_timer > 0.0 {
            let t = (get_time() as f32) * 8.0;
            let pulse_r = self.player.body.radius + 10.0 + t.sin() * 2.5;
            draw_poly_lines(
                p_sp.x,
                p_sp.y,
                8,
                pulse_r,
                t * 35.0,
                2.5,
                Color::new(0.0, 1.0, 0.85, 0.9),
            );
            draw_poly(
                p_sp.x,
                p_sp.y,
                8,
                pulse_r * 0.92,
                t * 35.0,
                Color::new(0.0, 1.0, 0.85, 0.18),
            );
        }
        let class_accent = match self.selected_class_id {
            1 => Color::new(0.82, 0.35, 1.0, 1.0),
            2 => Color::new(1.0, 0.82, 0.15, 1.0),
            _ => Color::new(0.0, 0.94, 1.0, 1.0),
        };
        // Ba-lô phản lực (Jetpack Thrusters) phía lưng
        let back_center = p_sp - gun_dir * 11.0;
        for side in [-1.0f32, 1.0] {
            let jet = back_center + gun_right * (side * 6.5);
            draw_poly(jet.x, jet.y, 4, 4.5, self.player.body.angle * 57.3, Color::new(0.2, 0.28, 0.38, 1.0));
            if self.player.body.is_dashing {
                let flame = jet - gun_dir * 12.0;
                draw_line(jet.x, jet.y, flame.x, flame.y, 4.0, Color::new(0.0, 0.95, 1.0, 0.9));
            }
        }
        // Hai cầu vai thiết giáp mở rộng hai bên
        for side in [-1.0f32, 1.0] {
            let shoulder = p_sp + gun_right * (side * 12.5) - gun_dir * 1.5;
            draw_poly(
                shoulder.x,
                shoulder.y,
                4,
                7.5,
                self.player.body.angle * 57.3 + 45.0,
                class_accent,
            );
            draw_poly_lines(
                shoulder.x,
                shoulder.y,
                4,
                7.5,
                self.player.body.angle * 57.3 + 45.0,
                1.5,
                WHITE,
            );
        }
        // Thân giáp chính hình lục giác & Mũ Visor chữ V hướng về phía trước
        draw_poly(
            p_sp.x,
            p_sp.y,
            6,
            11.5,
            self.player.body.angle * 57.3,
            Color::new(0.08, 0.14, 0.24, 1.0),
        );
        draw_poly_lines(
            p_sp.x,
            p_sp.y,
            6,
            11.5,
            self.player.body.angle * 57.3,
            2.0,
            class_accent,
        );
        let visor_tip = p_sp + gun_dir * 9.0;
        let visor_l = p_sp + gun_dir * 2.0 + gun_right * 5.5;
        let visor_r = p_sp + gun_dir * 2.0 - gun_right * 5.5;
        draw_triangle(visor_tip, visor_l, visor_r, WHITE);

        // Khẩu súng trên tay thay đổi theo vũ khí đang cầm
        let w_col = self.player.current_weapon().stats.rarity.color();
        let gun_base = p_sp + gun_right * 6.0 + gun_dir * 4.0;
        let gun_tip = gun_base + gun_dir * 22.0;
        draw_line(gun_base.x, gun_base.y, gun_tip.x, gun_tip.y, 5.5, Color::new(0.2, 0.25, 0.32, 1.0));
        draw_line(
            gun_base.x + gun_dir.x * 4.0,
            gun_base.y + gun_dir.y * 4.0,
            gun_tip.x,
            gun_tip.y,
            3.2,
            w_col,
        );

        // THANH CANH NHỊP NẠP ĐẠN HOÀN HẢO (OVERHEAD ACTIVE RELOAD TIMING BAR) NGAY TRÊN ĐẦU NHÂN VẬT!
        if self.player.reload_state.is_reloading {
            let bar_w = 88.0;
            let bar_h = 11.0;
            let bar_x = p_sp.x - bar_w / 2.0;
            let bar_y = p_sp.y - 44.0;
            let progress = (self.player.reload_state.timer
                / self.player.reload_state.duration.max(0.01))
            .clamp(0.0, 1.0);
            let (sweet_start, sweet_end) = if self.player.body.modifiers.wide_perfect_reload {
                (0.40f32, 0.88f32)
            } else {
                (0.58f32, 0.82f32)
            };
            let in_sweet = (sweet_start..=sweet_end).contains(&progress);

            // Khung nền tối nổi bật
            draw_rectangle(
                bar_x - 2.0,
                bar_y - 2.0,
                bar_w + 4.0,
                bar_h + 4.0,
                Color::new(0.02, 0.04, 0.08, 0.94),
            );
            // Tiến trình đã nạp mờ phía dưới
            draw_rectangle(
                bar_x,
                bar_y,
                bar_w * progress,
                bar_h,
                Color::new(0.18, 0.45, 0.65, 0.65),
            );
            // Vùng Xanh Lá Canh Nhịp Hoàn Hảo (Sweet Spot Zone)
            let sx = bar_x + bar_w * sweet_start;
            let sw = bar_w * (sweet_end - sweet_start);
            draw_rectangle(
                sx,
                bar_y,
                sw,
                bar_h,
                if in_sweet {
                    Color::new(0.1, 1.0, 0.45, 0.98)
                } else {
                    Color::new(0.0, 0.88, 0.38, 0.82)
                },
            );
            draw_rectangle_lines(sx, bar_y, sw, bar_h, 1.5, WHITE);
            draw_rectangle_lines(
                bar_x - 2.0,
                bar_y - 2.0,
                bar_w + 4.0,
                bar_h + 4.0,
                1.8,
                if in_sweet {
                    Color::new(0.2, 1.0, 0.5, 1.0)
                } else {
                    Color::new(0.0, 0.85, 1.0, 0.85)
                },
            );

            // Kim trượt canh nhịp (Moving Timing Cursor)
            let cur_x = bar_x + bar_w * progress;
            draw_rectangle(cur_x - 2.0, bar_y - 4.0, 4.0, bar_h + 8.0, WHITE);
            draw_triangle(
                vec2(cur_x, bar_y - 1.5),
                vec2(cur_x - 4.5, bar_y - 6.5),
                vec2(cur_x + 4.5, bar_y - 6.5),
                YELLOW,
            );

            // Nhãn nhắc bấm [R] ngay phía trên thanh nạp đạn (căn giữa hoàn hảo)
            if in_sweet {
                draw_text_centered(
                    tr("[BẤM R! PERFECT]", "[PRESS R! PERFECT]"),
                    p_sp.x,
                    bar_y - 8.0,
                    14.0,
                    Color::new(0.15, 1.0, 0.45, 1.0),
                );
            } else {
                draw_text_centered(
                    tr("NẠP ĐẠN [R]", "RELOAD [R]"),
                    p_sp.x,
                    bar_y - 8.0,
                    12.5,
                    Color::new(0.85, 0.92, 1.0, 0.9),
                );
            }
        }

        // Projectiles (Vẽ đạn của người chơi dạng Tia Dài / Đầu Đạn Nhọn; đạn quái dạng Thoi Gai Đỏ)
        for b in &self.player_projectiles {
            let sp = b.pos + cam_offset;
            if b.spawns_black_hole {
                draw_poly(sp.x, sp.y, 6, b.radius + 3.0, (get_time() as f32) * 180.0, b.color);
                draw_poly_lines(sp.x, sp.y, 6, b.radius + 5.0, -(get_time() as f32) * 140.0, 2.0, WHITE);
            } else if b.explosion_radius.is_some() {
                // Đạn tên lửa RPG có đầu nhọn và đuôi lửa
                let dir = b.vel.normalize_or_zero();
                let right = vec2(-dir.y, dir.x);
                let nose = sp + dir * 10.0;
                let tail_l = sp - dir * 7.0 + right * 4.5;
                let tail_r = sp - dir * 7.0 - right * 4.5;
                draw_triangle(nose, tail_l, tail_r, b.color);
                draw_line(sp.x - dir.x * 7.0, sp.y - dir.y * 7.0, sp.x - dir.x * 15.0, sp.y - dir.y * 15.0, 3.5, YELLOW);
            } else {
                // Đạn laser / đạn xuyên giáp kéo vệt dài sắc nét
                let dir = b.vel.normalize_or_zero();
                let tail = sp - dir * (b.radius * 3.4).max(11.0);
                let head = sp + dir * 4.0;
                draw_line(tail.x, tail.y, head.x, head.y, b.radius * 1.45, b.color);
                draw_line(sp.x - dir.x * 4.0, sp.y - dir.y * 4.0, head.x, head.y, (b.radius * 0.75).max(1.8), WHITE);
            }
        }
        for eb in &self.enemy_projectiles {
            let sp = eb.pos + cam_offset;
            let dir = eb.vel.normalize_or_zero();
            let right = vec2(-dir.y, dir.x);
            let nose = sp + dir * (eb.radius * 1.45);
            let back = sp - dir * (eb.radius * 1.1);
            let side_l = sp + right * (eb.radius * 0.95);
            let side_r = sp - right * (eb.radius * 0.95);
            draw_triangle(nose, side_l, side_r, eb.color);
            draw_triangle(back, side_l, side_r, eb.color);
            draw_poly(sp.x, sp.y, 4, eb.radius * 0.45, 45.0, WHITE);
        }

        // Float Texts
        for f in &self.float_texts {
            let sp = f.pos + cam_offset;
            draw_text(&f.text, sp.x, sp.y, f.size, f.color);
        }

        let active_boss = self
            .enemies
            .iter()
            .find(|e| e.body.is_boss)
            .map(|b| (b.body.name, b.body.hp, b.body.max_hp, b.body.parts.len()));

        if self.settings.show_minimap {
            draw_minimap(
                self.player.body.pos,
                &self.enemies,
                &self.allies,
                &self.structures,
                &self.weapon_drops,
            );
        }

        draw_hud(
            &self.player,
            self.wave,
            self.score,
            self.combo,
            (self.enemies_to_spawn - self.enemies_spawned) + (self.enemies.len() as u32),
            self.allies.len(),
            self.tech_level,
            self.tech_exp / self.next_tech_exp.max(1.0),
            active_boss,
            &self.settings,
        )
    }
}
