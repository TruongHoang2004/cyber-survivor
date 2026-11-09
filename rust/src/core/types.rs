use macroquad::prelude::*;

pub const WORLD_WIDTH: f32 = 2400.0;
pub const WORLD_HEIGHT: f32 = 2400.0;

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
    /// Hiển thị số sát thương bay lên
    pub show_damage_numbers: bool,
    /// Tự động nhặt vũ khí khi chạm vào (nếu tắt thì bấm [F] để nhặt)
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
            master_volume: 0.70,
            screen_shake_intensity: 0.15,
            particle_density: 0.65,
            grid_opacity: 0.50,
            swarm_scale: 1.0,
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
            0 => "THU CONG (MANUAL)",
            1 => "NAM CHAM THONG MINH (SMART)",
            _ => "TU DONG 360 (FULL AUTO)",
        }
    }

    pub fn load_or_default() -> Self {
        let mut s = Self::default();
        if let Ok(content) = std::fs::read_to_string(Self::CONFIG_PATH) {
            for line in content.lines() {
                if let Some((k, v)) = line.split_once('=') {
                    match k.trim() {
                        "master_volume" => s.master_volume = v.parse().unwrap_or(s.master_volume),
                        "screen_shake_intensity" => {
                            s.screen_shake_intensity = v.parse().unwrap_or(s.screen_shake_intensity)
                        }
                        "particle_density" => {
                            s.particle_density = v.parse().unwrap_or(s.particle_density)
                        }
                        "grid_opacity" => s.grid_opacity = v.parse().unwrap_or(s.grid_opacity),
                        "swarm_scale" => s.swarm_scale = v.parse().unwrap_or(s.swarm_scale),
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
            "master_volume={:.2}\n\
             screen_shake_intensity={:.2}\n\
             particle_density={:.2}\n\
             grid_opacity={:.2}\n\
             swarm_scale={:.2}\n\
             show_damage_numbers={}\n\
             auto_pickup_weapons={}\n\
             auto_equip_new_weapon={}\n\
             show_ally_bars={}\n\
             show_minimap={}\n\
             aim_assist_mode={}\n\
             auto_fire={}\n\
             high_score={}\n",
            self.master_volume,
            self.screen_shake_intensity,
            self.particle_density,
            self.grid_opacity,
            self.swarm_scale,
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
    pub bouncing_bullets: bool,
    pub pierce_count: u32,
    pub vampiric: bool,
    pub dash_fire_trail: bool,
    pub dmg_multi: f32,
    pub dash_cd_multi: f32,
    pub turret_overclock: bool,
    pub incendiary_rounds: bool,
    pub cryo_rounds: bool,
}

impl Default for PlayerModifiers {
    fn default() -> Self {
        Self {
            bouncing_bullets: false,
            pierce_count: 0,
            vampiric: false,
            dash_fire_trail: false,
            dmg_multi: 1.0,
            dash_cd_multi: 1.0,
            turret_overclock: false,
            incendiary_rounds: false,
            cryo_rounds: false,
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
    pub title: &'static str,
    pub icon: &'static str,
    pub desc: &'static str,
}
