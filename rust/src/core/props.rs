use macroquad::prelude::*;
use crate::weapon::arsenal::weapon_name_by_id;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum WeaponRarity {
    Common,
    Rare,
    Epic,
    Legendary,
}

impl WeaponRarity {
    pub fn random() -> Self {
        let r = rand::gen_range(0.0, 1.0);
        if r < 0.14 {
            WeaponRarity::Legendary
        } else if r < 0.38 {
            WeaponRarity::Epic
        } else if r < 0.72 {
            WeaponRarity::Rare
        } else {
            WeaponRarity::Common
        }
    }

    pub fn color(&self) -> Color {
        match self {
            WeaponRarity::Common => Color::new(0.75, 0.85, 0.95, 1.0),
            WeaponRarity::Rare => Color::new(0.0, 0.85, 1.0, 1.0),
            WeaponRarity::Epic => Color::new(0.75, 0.25, 1.0, 1.0),
            WeaponRarity::Legendary => Color::new(1.0, 0.82, 0.0, 1.0),
        }
    }

    pub fn label(&self) -> &'static str {
        match self {
            WeaponRarity::Common => "COMMON",
            WeaponRarity::Rare => "RARE",
            WeaponRarity::Epic => "EPIC",
            WeaponRarity::Legendary => "LEGENDARY",
        }
    }

    pub fn damage_bonus(&self) -> f32 {
        match self {
            WeaponRarity::Common => 1.0,
            WeaponRarity::Rare => 1.25,
            WeaponRarity::Epic => 1.55,
            WeaponRarity::Legendary => 2.0,
        }
    }
}

pub struct WeaponDrop {
    pub pos: Vec2,
    pub weapon_id: usize,
    pub rarity: WeaponRarity,
    pub bob_phase: f32,
}

impl WeaponDrop {
    pub fn new(pos: Vec2, weapon_id: usize, rarity: WeaponRarity) -> Self {
        Self {
            pos,
            weapon_id,
            rarity,
            bob_phase: rand::gen_range(0.0, 6.28),
        }
    }

    pub fn draw(&self, cam_offset: Vec2, player_pos: Vec2) {
        let sp = self.pos + cam_offset;
        let col = self.rarity.color();
        let bob = (self.bob_phase.sin()) * 4.0;

        // Cột sáng Hologram chiếu thẳng lên từ vũ khí rơi dưới đất
        let beam_h = match self.rarity {
            WeaponRarity::Common => 45.0,
            WeaponRarity::Rare => 65.0,
            WeaponRarity::Epic => 90.0,
            WeaponRarity::Legendary => 125.0,
        };
        draw_rectangle(
            sp.x - 10.0,
            sp.y - beam_h,
            20.0,
            beam_h,
            Color::new(col.r, col.g, col.b, 0.16),
        );
        draw_line(
            sp.x,
            sp.y - beam_h,
            sp.x,
            sp.y,
            2.0,
            Color::new(col.r, col.g, col.b, 0.75),
        );

        // Khung bệ hòm tiếp tế lục giác dưới đất (không dùng hình tròn)
        draw_poly(sp.x, sp.y, 6, 20.0, 0.0, Color::new(0.08, 0.12, 0.18, 0.9));
        draw_poly_lines(sp.x, sp.y, 6, 20.0, 0.0, 2.2, col);

        // Vẽ hình dáng khẩu súng chi tiết (báng súng, thân súng, băng đạn, nòng súng)
        let gy = sp.y - 6.0 + bob;
        // Báng súng
        draw_rectangle(sp.x - 16.0, gy - 2.0, 7.0, 6.0, Color::new(0.22, 0.26, 0.34, 1.0));
        // Thân súng chính
        draw_rectangle(sp.x - 10.0, gy - 5.0, 20.0, 8.0, Color::new(0.15, 0.2, 0.28, 1.0));
        draw_rectangle_lines(sp.x - 10.0, gy - 5.0, 20.0, 8.0, 1.5, col);
        // Nòng súng & băng đạn
        draw_rectangle(sp.x + 10.0, gy - 3.5, 10.0, 4.5, col);
        draw_rectangle(sp.x - 2.0, gy + 3.0, 5.0, 6.0, col);
        draw_rectangle(sp.x - 11.0, gy + 3.0, 4.5, 6.0, Color::new(0.3, 0.35, 0.42, 1.0));

        // Hiển thị tên súng & hướng dẫn nhặt khi đứng gần
        let dist = (self.pos - player_pos).length();
        let w_name = weapon_name_by_id(self.weapon_id);
        draw_text(
            &format!("[{}] {}", self.rarity.label(), w_name),
            sp.x - 48.0,
            sp.y - 26.0 + bob,
            15.0,
            col,
        );
        if dist < 95.0 {
            draw_text(
                "CHAM VAO HOAC BAM [F] DE NHAT!",
                sp.x - 75.0,
                sp.y + 30.0,
                14.0,
                WHITE,
            );
        }
    }
}

pub struct ExplosiveBarrel {
    pub pos: Vec2,
    pub radius: f32,
    pub hp: f32,
    pub fuse_timer: Option<f32>,
}

impl ExplosiveBarrel {
    pub fn new(pos: Vec2) -> Self {
        Self {
            pos,
            radius: 16.0,
            hp: 30.0,
            fuse_timer: None,
        }
    }

    pub fn draw(&self, cam_offset: Vec2) {
        let sp = self.pos + cam_offset;
        let flashing = self.fuse_timer.is_some();
        let body_col = if flashing {
            Color::new(1.0, 0.55, 0.0, 1.0)
        } else {
            Color::new(0.82, 0.14, 0.14, 1.0)
        };

        // Thùng phuy nhiên liệu nổ hình trụ chữ nhật đứng có đai thép cảnh báo
        let w = 24.0;
        let h = 30.0;
        let x = sp.x - w / 2.0;
        let y = sp.y - h / 2.0;
        draw_rectangle(x, y, w, h, body_col);
        draw_rectangle(x, y + 6.0, w, 4.0, Color::new(1.0, 0.85, 0.0, 0.95));
        draw_rectangle(x, y + h - 10.0, w, 4.0, Color::new(1.0, 0.85, 0.0, 0.95));
        draw_rectangle_lines(x, y, w, h, 2.0, YELLOW);
        draw_text("TNT", sp.x - 10.5, sp.y + 4.0, 12.0, WHITE);
    }
}
