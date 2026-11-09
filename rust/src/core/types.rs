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
}

impl Default for GameSettings {
    fn default() -> Self {
        Self {
            screen_shake_intensity: 0.15,
            particle_density: 0.65,
            grid_opacity: 0.50,
            swarm_scale: 1.0,
            show_damage_numbers: true,
            auto_pickup_weapons: true,
            auto_equip_new_weapon: true,
            show_ally_bars: true,
        }
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
