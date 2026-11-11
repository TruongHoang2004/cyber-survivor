use macroquad::prelude::*;
use crate::core::i18n::{draw_text, tr};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum WeaponRarity {
    Common,    // Dạng 1 (Lv 1 - 99)
    Rare,      // Dạng 2 (Lv 100 - 199)
    Epic,      // Dạng 3 (Lv 200 - 299)
    Legendary, // Dạng 4 (Lv 300 - 399)
    Mythic,    // Dạng 5 (Lv 400 - 499)
    Ultimate,  // Dạng 6 (Lv 500 MAX - Tối Thượng)
}

impl WeaponRarity {
    pub fn from_level(level: u32) -> Self {
        if level >= 500 {
            WeaponRarity::Ultimate
        } else if level >= 400 {
            WeaponRarity::Mythic
        } else if level >= 300 {
            WeaponRarity::Legendary
        } else if level >= 200 {
            WeaponRarity::Epic
        } else if level >= 100 {
            WeaponRarity::Rare
        } else {
            WeaponRarity::Common
        }
    }

    pub fn random() -> Self {
        let r = rand::gen_range(0.0, 1.0);
        if r < 0.05 {
            WeaponRarity::Ultimate
        } else if r < 0.16 {
            WeaponRarity::Mythic
        } else if r < 0.38 {
            WeaponRarity::Legendary
        } else if r < 0.68 {
            WeaponRarity::Epic
        } else if r < 0.86 {
            WeaponRarity::Rare
        } else {
            WeaponRarity::Common
        }
    }

    pub fn color(&self) -> Color {
        match self {
            WeaponRarity::Common => Color::new(0.75, 0.85, 0.95, 1.0),
            WeaponRarity::Rare => Color::new(0.0, 0.88, 1.0, 1.0),
            WeaponRarity::Epic => Color::new(0.78, 0.28, 1.0, 1.0),
            WeaponRarity::Legendary => Color::new(1.0, 0.82, 0.0, 1.0),
            WeaponRarity::Mythic => Color::new(1.0, 0.22, 0.35, 1.0),
            WeaponRarity::Ultimate => Color::new(0.2, 1.0, 0.85, 1.0),
        }
    }

    pub fn label(&self) -> &'static str {
        match self {
            WeaponRarity::Common => tr("DẠNG 1 [CƠ BẢN]", "FORM 1 [BASE]"),
            WeaponRarity::Rare => tr("DẠNG 2 [CẢI TIẾN]", "FORM 2 [ENHANCED]"),
            WeaponRarity::Epic => tr("DẠNG 3 [CƯỜNG HÓA]", "FORM 3 [OVERCHARGED]"),
            WeaponRarity::Legendary => tr("DẠNG 4 [SIÊU CẤP]", "FORM 4 [HYPER]"),
            WeaponRarity::Mythic => tr("DẠNG 5 [THẦN THOẠI]", "FORM 5 [MYTHIC]"),
            WeaponRarity::Ultimate => tr("DẠNG 6 [TỐI THƯỢNG MAX]", "FORM 6 [ULTIMATE MAX]"),
        }
    }

    pub fn damage_bonus(&self) -> f32 {
        match self {
            WeaponRarity::Common => 1.0,
            WeaponRarity::Rare => 1.35,
            WeaponRarity::Epic => 1.75,
            WeaponRarity::Legendary => 2.30,
            WeaponRarity::Mythic => 3.10,
            WeaponRarity::Ultimate => 4.50,
        }
    }

    /// Số cấp độ vũ khí cộng thêm khi nhặt Đồ Nâng Cấp Súng
    pub fn upgrade_levels(&self) -> u32 {
        match self {
            WeaponRarity::Common => 3,
            WeaponRarity::Rare => 8,
            WeaponRarity::Epic => 18,
            WeaponRarity::Legendary => 35,
            WeaponRarity::Mythic => 60,
            WeaponRarity::Ultimate => 100,
        }
    }
}

/// Đồ Nâng Cấp Súng (Weapon Upgrade Module) rơi ra trong trận để tích đủ 100 Lv tiến hóa thành súng [EVO]
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
        let lv_gain = self.rarity.upgrade_levels();

        // Cột sáng Hologram chiếu thẳng lên từ Đồ Nâng Cấp Súng
        let beam_h = match self.rarity {
            WeaponRarity::Common => 45.0,
            WeaponRarity::Rare => 65.0,
            WeaponRarity::Epic => 90.0,
            WeaponRarity::Legendary => 125.0,
            WeaponRarity::Mythic => 165.0,
            WeaponRarity::Ultimate => 230.0,
        };
        draw_rectangle(
            sp.x - 9.0,
            sp.y - beam_h,
            18.0,
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

        // Bệ Module Công Nghệ Lục Giác dưới đất
        draw_poly(sp.x, sp.y, 6, 18.0, 0.0, Color::new(0.07, 0.11, 0.18, 0.92));
        draw_poly_lines(sp.x, sp.y, 6, 18.0, 0.0, 2.2, col);

        // Vẽ Hộp Linh Kiện Nâng Cấp Vũ Khí (Weapon Upgrade Core + Mũi tên thăng cấp)
        let gy = sp.y - 5.0 + bob;
        draw_poly(sp.x, gy, 8, 12.5, self.bob_phase * 25.0, Color::new(0.12, 0.18, 0.28, 0.96));
        draw_poly_lines(sp.x, gy, 8, 12.5, self.bob_phase * 25.0, 2.0, col);
        // Mũi tên thăng cấp kép hướng lên [▲]
        draw_triangle(
            vec2(sp.x, gy - 8.0),
            vec2(sp.x - 6.5, gy + 1.0),
            vec2(sp.x + 6.5, gy + 1.0),
            col,
        );
        draw_triangle(
            vec2(sp.x, gy - 2.0),
            vec2(sp.x - 5.5, gy + 6.0),
            vec2(sp.x + 5.5, gy + 6.0),
            WHITE,
        );

        // Hiển thị nhãn Đồ Nâng Cấp Súng
        let dist = (self.pos - player_pos).length();
        let label = format!(
            "{} (+{} LV)",
            tr("LÕI NÂNG CẤP SÚNG", "WEAPON UPGRADE"),
            lv_gain
        );
        draw_text(&label, sp.x - 68.0, sp.y - 24.0 + bob, 14.0, col);
        if dist < 95.0 {
            draw_text(
                tr("CHẠM VÀO HOẶC BẤM [F] ĐỂ NÂNG CẤP SÚNG!", "TOUCH OR PRESS [F] TO UPGRADE WEAPON!"),
                sp.x - 110.0,
                sp.y + 30.0,
                13.0,
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
