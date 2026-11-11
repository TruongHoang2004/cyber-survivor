use macroquad::prelude::*;
use crate::core::i18n::{tr, Language};

pub const WORLD_WIDTH: f32 = 3600.0;
pub const WORLD_HEIGHT: f32 = 3600.0;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum GameState {
    Start,
    Playing,
    Upgrade,
    Paused,
    GameOver,
}

#[derive(Clone)]
pub struct GameSettings {
    /// Ngôn ngữ hiển thị: Tiếng Việt có dấu (`Language::Vi`) hoặc Tiếng Anh (`Language::En`)
    pub language: Language,
    /// Âm lượng tổng: 0.0 -> 1.0. Mặc định: 0.70
    pub master_volume: f32,
    /// Độ rung màn hình: 0.0 (Tắt hẳn) -> 1.0 (Tối đa). Mặc định: 0.15 (15% rất êm)
    pub screen_shake_intensity: f32,
    /// Mật độ hiệu ứng hạt cháy nổ: 0.2 -> 1.0. Mặc định: 0.65
    pub particle_density: f32,
    /// Độ sáng lưới nền chiến trường: 0.0 -> 1.0. Mặc định: 0.50
    pub grid_opacity: f32,
    /// Quy mô số lượng quái: 0.5 -> 2.0. Mặc định: 1.0
    pub swarm_scale: f32,
    /// Sinh toàn bộ quái trong 1 lần duy nhất ngay đầu Wave (Instant Full Horde)
    pub instant_full_spawn: bool,
    /// Mức giới hạn quái tối đa tuỳ theo mức RAM (Mặc định: 10,000 con)
    pub max_ram_spawn_limit: u32,
    /// Hiển thị số sát thương bay lên
    pub show_damage_numbers: bool,
    /// Tự động nhặt Lõi Nâng Cấp Vũ Khí khi chạm vào (nếu tắt thì bấm [F] để nhặt)
    pub auto_pickup_weapons: bool,
    /// Tự động chuyển sang súng mới vừa mở khóa
    pub auto_equip_new_weapon: bool,
    /// Hiển thị tên và thanh máu của lính đồng minh
    pub show_ally_bars: bool,
    /// Hiển thị Bản đồ nhỏ Radar (Minimap)
    pub show_minimap: bool,
    /// Chế độ Hỗ Trợ Nhắm Bắn: 0 = Thủ công (Manual), 1 = Khóa tâm thông minh (Smart Assist), 2 = Tự động 360° (Full Auto-Aim)
    pub aim_assist_mode: u8,
    /// Tự động bóp cò bắn khi có mục tiêu (Auto-Fire, phím tắt [V])
    pub auto_fire: bool,
    /// Điểm kỷ lục cao nhất được lưu lại
    pub high_score: u64,
}

impl Default for GameSettings {
    fn default() -> Self {
        Self {
            language: Language::Vi,
            master_volume: 0.70,
            screen_shake_intensity: 0.15,
            particle_density: 0.65,
            grid_opacity: 0.50,
            swarm_scale: 1.0,
            instant_full_spawn: true,
            max_ram_spawn_limit: 10000,
            show_damage_numbers: true,
            auto_pickup_weapons: true,
            auto_equip_new_weapon: true,
            show_ally_bars: true,
            show_minimap: true,
            aim_assist_mode: 1,
            auto_fire: false,
            high_score: 0,
        }
    }
}

impl GameSettings {
    const CONFIG_PATH: &'static str = "cyber_survivor_settings.cfg";

    pub fn aim_mode_label(&self) -> &'static str {
        match self.aim_assist_mode {
            0 => tr("THỦ CÔNG (MANUAL)", "MANUAL AIM"),
            1 => tr("NAM CHÂM THÔNG MINH", "SMART MAGNETIC"),
            _ => tr("TỰ ĐỘNG 360°", "FULL 360° AUTO"),
        }
    }

    pub fn load_or_default() -> Self {
        let mut s = Self::default();
        if let Ok(content) = std::fs::read_to_string(Self::CONFIG_PATH) {
            for line in content.lines() {
                if let Some((k, v)) = line.split_once('=') {
                    match k.trim() {
                        "language" => s.language = Language::from_code(v),
                        "master_volume" => s.master_volume = v.parse().unwrap_or(s.master_volume),
                        "screen_shake_intensity" => {
                            s.screen_shake_intensity = v.parse().unwrap_or(s.screen_shake_intensity)
                        }
                        "particle_density" => {
                            s.particle_density = v.parse().unwrap_or(s.particle_density)
                        }
                        "grid_opacity" => s.grid_opacity = v.parse().unwrap_or(s.grid_opacity),
                        "swarm_scale" => s.swarm_scale = v.parse().unwrap_or(s.swarm_scale),
                        "instant_full_spawn" => s.instant_full_spawn = v == "true",
                        "max_ram_spawn_limit" => {
                            s.max_ram_spawn_limit = v.parse().unwrap_or(s.max_ram_spawn_limit)
                        }
                        "show_damage_numbers" => s.show_damage_numbers = v == "true",
                        "auto_pickup_weapons" => s.auto_pickup_weapons = v == "true",
                        "auto_equip_new_weapon" => s.auto_equip_new_weapon = v == "true",
                        "show_ally_bars" => s.show_ally_bars = v == "true",
                        "show_minimap" => s.show_minimap = v == "true",
                        "aim_assist_mode" => {
                            s.aim_assist_mode = v.parse::<u8>().unwrap_or(1).min(2)
                        }
                        "auto_fire" => s.auto_fire = v == "true",
                        "high_score" => s.high_score = v.parse().unwrap_or(0),
                        _ => {}
                    }
                }
            }
        }
        s
    }

    pub fn save_to_disk(&self) {
        let data = format!(
            "language={}\n\
             master_volume={:.2}\n\
             screen_shake_intensity={:.2}\n\
             particle_density={:.2}\n\
             grid_opacity={:.2}\n\
             swarm_scale={:.2}\n\
             instant_full_spawn={}\n\
             max_ram_spawn_limit={}\n\
             show_damage_numbers={}\n\
             auto_pickup_weapons={}\n\
             auto_equip_new_weapon={}\n\
             show_ally_bars={}\n\
             show_minimap={}\n\
             aim_assist_mode={}\n\
             auto_fire={}\n\
             high_score={}\n",
            self.language.code(),
            self.master_volume,
            self.screen_shake_intensity,
            self.particle_density,
            self.grid_opacity,
            self.swarm_scale,
            self.instant_full_spawn,
            self.max_ram_spawn_limit,
            self.show_damage_numbers,
            self.auto_pickup_weapons,
            self.auto_equip_new_weapon,
            self.show_ally_bars,
            self.show_minimap,
            self.aim_assist_mode,
            self.auto_fire,
            self.high_score
        );
        let _ = std::fs::write(Self::CONFIG_PATH, data);
    }
}

#[derive(Clone, Copy)]
pub enum StatusEffect {
    Burn { dps: f32, duration: f32 },
    Slow { factor: f32, duration: f32 },
    Stun { duration: f32 },
}

#[derive(Clone)]
pub struct PlayerModifiers {
    pub acquired_ids: Vec<&'static str>,
    pub bouncing_bullets: bool,
    pub pierce_count: u32,
    pub vampiric: bool,
    pub dash_fire_trail: bool,
    pub dmg_multi: f32,
    pub bullet_size_multi: f32,
    pub fire_rate_multi: f32,
    pub reload_speed_multi: f32,
    pub wide_perfect_reload: bool,
    pub ammo_on_kill: u32,
    pub infinite_ammo: bool,
    pub free_ammo_timer: f32,
    pub multishot_bonus: u32,
    pub impact_blast_radius: f32,
    pub chain_on_hit: u32,
    pub dash_cd_multi: f32,
    pub dash_blades: bool,
    pub turret_overclock: bool,
    pub shield_retaliation: bool,
    pub incendiary_rounds: bool,
    pub cryo_rounds: bool,
    // Thuộc tính Từ Trường Thu Hồi Kinh Nghiệm (EXP Magnet & Gain)
    pub magnet_radius_multi: f32,
    pub exp_gain_multi: f32,
    pub magnet_shield_on_orb: bool,
    pub magnet_arc_on_orb: bool,
    // Hệ 8: Đa Hướng & Hỏa Lực Phòng Thủ (Omni-Directional & Defensive Matrix)
    pub multi_rear_shot: bool,
    pub multi_flank_shot: bool,
    pub orbital_plasma_shield: bool,
    pub multi_omni_spiral: bool,
    // 16 Bộ Kỹ Năng Tối Thượng (Ultimate Synergy Sets)
    pub ult_apocalypse_gatling: bool,
    pub ult_cataclysm_barrage: bool,
    pub ult_chrono_legion: bool,
    pub ult_hellfire_artillery: bool,
    pub ult_volt_overdrive: bool,
    pub ult_bullet_hurricane: bool,
    pub ult_dreadnought_titan: bool,
    pub ult_phantom_trigger: bool,
    pub ult_hyper_reactor: bool,
    pub ult_autonomous_arsenal: bool,
    pub ult_singularity_walker: bool,
    pub ult_valkyrie_impact: bool,
    pub ult_orbital_supernova: bool,
    pub ult_omni_annihilation: bool,
    pub ult_chronos_singularity: bool,
    pub ult_divine_wrath: bool,
}

impl Default for PlayerModifiers {
    fn default() -> Self {
        Self {
            acquired_ids: Vec::new(),
            bouncing_bullets: false,
            pierce_count: 0,
            vampiric: false,
            dash_fire_trail: false,
            dmg_multi: 1.0,
            bullet_size_multi: 1.0,
            fire_rate_multi: 1.0,
            reload_speed_multi: 1.0,
            wide_perfect_reload: false,
            ammo_on_kill: 0,
            infinite_ammo: false,
            free_ammo_timer: 0.0,
            multishot_bonus: 0,
            impact_blast_radius: 0.0,
            chain_on_hit: 0,
            dash_cd_multi: 1.0,
            dash_blades: false,
            turret_overclock: false,
            shield_retaliation: false,
            incendiary_rounds: false,
            cryo_rounds: false,
            magnet_radius_multi: 1.0,
            exp_gain_multi: 1.0,
            magnet_shield_on_orb: false,
            magnet_arc_on_orb: false,
            multi_rear_shot: false,
            multi_flank_shot: false,
            orbital_plasma_shield: false,
            multi_omni_spiral: false,
            ult_apocalypse_gatling: false,
            ult_cataclysm_barrage: false,
            ult_chrono_legion: false,
            ult_hellfire_artillery: false,
            ult_volt_overdrive: false,
            ult_bullet_hurricane: false,
            ult_dreadnought_titan: false,
            ult_phantom_trigger: false,
            ult_hyper_reactor: false,
            ult_autonomous_arsenal: false,
            ult_singularity_walker: false,
            ult_valkyrie_impact: false,
            ult_orbital_supernova: false,
            ult_omni_annihilation: false,
            ult_chronos_singularity: false,
            ult_divine_wrath: false,
        }
    }
}

impl PlayerModifiers {
    pub fn has(&self, id: &str) -> bool {
        self.acquired_ids.iter().any(|&x| x == id)
    }

    /// Trả về cấp độ cao nhất đã đạt được (0..=3) của 1 trong 8 Hệ Nâng Cấp (`tree_id`: 0..8)
    pub fn tree_tier(&self, tree_id: usize) -> u8 {
        let (t1, t2a, t2b, t3) = match tree_id {
            0 => ("heavy_1", "heavy_2a", "heavy_2b", "heavy_3"),
            1 => ("rapid_1", "rapid_2a", "rapid_2b", "rapid_3"),
            2 => ("reload_1", "reload_2a", "reload_2b", "reload_3"),
            3 => ("elem_1", "elem_2a", "elem_2b", "elem_3"),
            4 => ("mob_1", "mob_2a", "mob_2b", "mob_3"),
            5 => ("aegis_1", "aegis_2a", "aegis_2b", "aegis_3"),
            6 => ("magnet_1", "magnet_2a", "magnet_2b", "magnet_3"),
            7 => ("omni_1", "omni_2a", "omni_2b", "omni_3"),
            _ => return 0,
        };
        if self.has(t3) {
            3
        } else if self.has(t2a) || self.has(t2b) {
            2
        } else if self.has(t1) {
            1
        } else {
            0
        }
    }
}

pub struct PlayerBody {
    pub pos: Vec2,
    pub vel: Vec2,
    pub radius: f32,
    pub angle: f32,
    pub speed: f32,
    pub hp: f32,
    pub max_hp: f32,
    pub shield: f32,
    pub max_shield: f32,
    pub shield_regen_timer: f32,
    pub stamina: f32,
    pub max_stamina: f32,
    pub is_dashing: bool,
    pub dash_duration: f32,
    pub invulnerable_timer: f32,
    pub bullet_time: bool,
    pub active_overcharge: bool,
    pub modifiers: PlayerModifiers,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum PickupType {
    Medkit,
    Ammo,
    Nuke,
    Overcharge,
    Magnet,
}

#[derive(Clone, Copy)]
pub struct ExpOrb {
    pub pos: Vec2,
    pub vel: Vec2,
    pub value: f32,
    pub tier: u8,
    pub pulse: f32,
    pub magnetized: bool,
}

pub struct Pickup {
    pub pos: Vec2,
    pub pickup_type: PickupType,
    pub radius: f32,
    pub pulse: f32,
}

pub struct Particle {
    pub pos: Vec2,
    pub vel: Vec2,
    pub color: Color,
    pub radius: f32,
    pub life: f32,
    pub max_life: f32,
}

pub struct Decal {
    pub pos: Vec2,
    pub radius: f32,
    pub color: Color,
    pub life: f32,
    pub is_damage_zone: bool,
}

pub struct Casing {
    pub pos: Vec2,
    pub vel: Vec2,
    pub rotation: f32,
    pub v_rot: f32,
    pub life: f32,
}

pub struct FloatText {
    pub pos: Vec2,
    pub text: String,
    pub color: Color,
    pub size: f32,
    pub life: f32,
}

#[derive(Clone)]
pub struct UpgradeOption {
    pub id: &'static str,
    pub tree_id: usize,
    pub tier: u8,
    pub category_tag: &'static str,
    pub title: &'static str,
    pub icon: &'static str,
    pub desc: &'static str,
    pub req_desc: &'static str,
}
