use macroquad::prelude::*;
use crate::character::traits::CharacterClass;
use crate::core::context::CombatContext;
use crate::core::i18n::tr;
use crate::core::types::{PlayerBody, PlayerModifiers};
use crate::skill::abilities::{default_skills, tech_engineer_skills, void_stalker_skills};
use crate::skill::traits::SkillSlot;
use crate::weapon::arsenal::default_arsenal;
use crate::weapon::traits::Weapon;

pub fn create_class_by_id(id: usize) -> Box<dyn CharacterClass> {
    match id {
        1 => Box::new(VoidStalker),
        2 => Box::new(TechEngineer),
        _ => Box::new(CyberMarine),
    }
}

// =========================================================
// 1. CYBER MARINE (Chiến Binh Khung Giáp Nano Cân Bằng)
// =========================================================
pub struct CyberMarine;

impl CharacterClass for CyberMarine {
    fn name(&self) -> &'static str {
        "CYBER MARINE"
    }

    fn description(&self) -> &'static str {
        tr(
            "Chiến binh cân bằng: Giáp Nano phục hồi nhanh, Không kích & Tiểu đội tiếp viện.",
            "Balanced soldier: Fast Nano Shield regen, Orbital Strike & Marine Squad.",
        )
    }

    fn initial_body(&self, start_pos: Vec2) -> PlayerBody {
        PlayerBody {
            pos: start_pos,
            vel: Vec2::ZERO,
            radius: 18.0,
            angle: 0.0,
            speed: 330.0,
            hp: 110.0,
            max_hp: 110.0,
            shield: 60.0,
            max_shield: 60.0,
            shield_regen_timer: 0.0,
            stamina: 100.0,
            max_stamina: 100.0,
            is_dashing: false,
            dash_duration: 0.0,
            invulnerable_timer: 0.0,
            bullet_time: false,
            active_overcharge: false,
            modifiers: PlayerModifiers::default(),
        }
    }

    fn starting_weapons(&self) -> Vec<Weapon> {
        default_arsenal()
    }

    fn starting_skills(&self) -> Vec<SkillSlot> {
        default_skills()
    }

    fn on_enemy_killed(&mut self, player: &mut PlayerBody, _ctx: &mut CombatContext) {
        if player.modifiers.vampiric && player.hp < player.max_hp {
            player.hp = (player.hp + 2.5).min(player.max_hp);
        }
    }
}

// =========================================================
// 2. VOID STALKER (Sát Thủ Không Gian - Blink & Hố Đen)
// =========================================================
pub struct VoidStalker;

impl CharacterClass for VoidStalker {
    fn name(&self) -> &'static str {
        "VOID STALKER"
    }

    fn description(&self) -> &'static str {
        tr(
            "Sát thủ tốc độ cao (+25% DMG, Dịch chuyển Phase Blink tạo Hố Đen & Bão Sét).",
            "High-speed assassin (+25% DMG, Phase Blink spawns Black Hole & Storm).",
        )
    }

    fn initial_body(&self, start_pos: Vec2) -> PlayerBody {
        let mut mods = PlayerModifiers::default();
        mods.dmg_multi = 1.25;
        mods.pierce_count = 1;
        mods.vampiric = true;
        PlayerBody {
            pos: start_pos,
            vel: Vec2::ZERO,
            radius: 16.5,
            angle: 0.0,
            speed: 375.0,
            hp: 90.0,
            max_hp: 90.0,
            shield: 45.0,
            max_shield: 45.0,
            shield_regen_timer: 0.0,
            stamina: 130.0,
            max_stamina: 130.0,
            is_dashing: false,
            dash_duration: 0.0,
            invulnerable_timer: 0.0,
            bullet_time: false,
            active_overcharge: false,
            modifiers: mods,
        }
    }

    fn starting_weapons(&self) -> Vec<Weapon> {
        default_arsenal()
    }

    fn starting_skills(&self) -> Vec<SkillSlot> {
        void_stalker_skills()
    }

    fn on_enemy_killed(&mut self, player: &mut PlayerBody, _ctx: &mut CombatContext) {
        // Nội tại Void: Hạ gục quái hồi ngay +2.5 HP và +4 Stamina Bullet Time
        player.hp = (player.hp + 2.5).min(player.max_hp);
        player.stamina = (player.stamina + 4.0).min(player.max_stamina);
    }
}

// =========================================================
// 3. TECH ENGINEER (Chỉ Huy Binh Đoàn Cơ Khí & Trụ Súng)
// =========================================================
pub struct TechEngineer;

impl CharacterClass for TechEngineer {
    fn name(&self) -> &'static str {
        "TECH ENGINEER"
    }

    fn description(&self) -> &'static str {
        tr(
            "Kỹ sư Cơ khí: Giáp khiên 100, gọi 2 Đại Robot Titan Mech + 2 Trụ Súng [E].",
            "Mecha Engineer: 100 Shield, deploys 2 Titan Mechs + 2 Sentry Turrets [E].",
        )
    }

    fn initial_body(&self, start_pos: Vec2) -> PlayerBody {
        let mut mods = PlayerModifiers::default();
        mods.turret_overclock = true;
        mods.bouncing_bullets = true;
        PlayerBody {
            pos: start_pos,
            vel: Vec2::ZERO,
            radius: 19.0,
            angle: 0.0,
            speed: 310.0,
            hp: 125.0,
            max_hp: 125.0,
            shield: 100.0,
            max_shield: 100.0,
            shield_regen_timer: 0.0,
            stamina: 100.0,
            max_stamina: 100.0,
            is_dashing: false,
            dash_duration: 0.0,
            invulnerable_timer: 0.0,
            bullet_time: false,
            active_overcharge: false,
            modifiers: mods,
        }
    }

    fn starting_weapons(&self) -> Vec<Weapon> {
        default_arsenal()
    }

    fn starting_skills(&self) -> Vec<SkillSlot> {
        tech_engineer_skills()
    }

    fn on_enemy_killed(&mut self, player: &mut PlayerBody, _ctx: &mut CombatContext) {
        // Nội tại Kỹ sư: Hạ gục quái nạp lại +3.0 Khiên năng lượng
        player.shield = (player.shield + 3.0).min(player.max_shield);
        if player.modifiers.vampiric && player.hp < player.max_hp {
            player.hp = (player.hp + 2.0).min(player.max_hp);
        }
    }
}
