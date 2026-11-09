use macroquad::prelude::*;
use crate::character::traits::CharacterClass;
use crate::core::types::PlayerBody;
use crate::skill::traits::SkillSlot;
use crate::weapon::traits::Weapon;

pub struct ActiveReload {
    pub is_reloading: bool,
    pub timer: f32,
    pub duration: f32,
}

pub struct Player {
    pub body: PlayerBody,
    pub class: Box<dyn CharacterClass>,
    pub weapons: Vec<Weapon>,
    pub current_weapon_idx: usize,
    pub skills: Vec<SkillSlot>,
    pub reload_state: ActiveReload,
    pub fire_timer: f32,
}

impl Player {
    pub fn new(class: Box<dyn CharacterClass>, start_pos: Vec2) -> Self {
        let body = class.initial_body(start_pos);
        let weapons = class.starting_weapons();
        let skills = class.starting_skills();
        Self {
            body,
            class,
            weapons,
            current_weapon_idx: 0,
            skills,
            reload_state: ActiveReload {
                is_reloading: false,
                timer: 0.0,
                duration: 1.0,
            },
            fire_timer: 0.0,
        }
    }

    pub fn set_class(&mut self, class: Box<dyn CharacterClass>, start_pos: Vec2) {
        self.class = class;
        self.reset(start_pos);
    }

    pub fn reset(&mut self, start_pos: Vec2) {
        self.body = self.class.initial_body(start_pos);
        self.weapons = self.class.starting_weapons();
        self.skills = self.class.starting_skills();
        self.current_weapon_idx = 0;
        self.reload_state.is_reloading = false;
        self.fire_timer = 0.0;
    }

    pub fn current_weapon(&self) -> &Weapon {
        &self.weapons[self.current_weapon_idx]
    }

    pub fn current_weapon_mut(&mut self) -> &mut Weapon {
        &mut self.weapons[self.current_weapon_idx]
    }

    pub fn dash_cd_ratio(&self) -> f32 {
        for s in &self.skills {
            if s.key == KeyCode::Space {
                if s.cooldown <= 0.0 {
                    return 1.0;
                }
                let max_cd = s.effective_max_cd(&self.body).max(0.01);
                return (1.0 - s.cooldown / max_cd).clamp(0.0, 1.0);
            }
        }
        1.0
    }

    pub fn take_damage(&mut self, mut amount: f32) -> f32 {
        if self.body.is_dashing {
            return 0.0;
        }
        self.body.shield_regen_timer = 0.0;
        if self.body.shield > 0.0 {
            if self.body.shield >= amount {
                self.body.shield -= amount;
                amount = 0.0;
            } else {
                amount -= self.body.shield;
                self.body.shield = 0.0;
            }
        }
        self.body.hp = (self.body.hp - amount).max(0.0);
        6.0
    }
}
