use macroquad::prelude::*;
use crate::core::context::CombatContext;
use crate::core::events::{ShockwaveRingEvent, SupplyPodDropEvent};
use crate::core::i18n::{draw_text, tr};
use crate::core::props::WeaponRarity;
use crate::core::types::PlayerBody;

pub struct StructureBody {
    pub name: &'static str,
    pub pos: Vec2,
    pub radius: f32,
    pub hp: f32,
    pub max_hp: f32,
    pub color: Color,
}

/// Lớp trừu tượng cho mọi Công Trình / Cứ Điểm Chiến Thuật trên bản đồ
pub trait BattlefieldStructure: Send + Sync {
    fn update(
        &mut self,
        body: &mut StructureBody,
        player: &mut PlayerBody,
        ctx: &mut CombatContext,
        dt: f32,
    ) -> bool;

    fn draw_custom(&self, body: &StructureBody, screen_pos: Vec2);

    fn blocks_enemy_projectiles(&self) -> bool {
        false
    }
}

pub struct TacticalStructureEntity {
    pub body: StructureBody,
    pub behavior: Box<dyn BattlefieldStructure>,
}

impl TacticalStructureEntity {
    pub fn new(body: StructureBody, behavior: Box<dyn BattlefieldStructure>) -> Self {
        Self { body, behavior }
    }

    pub fn tick(&mut self, player: &mut PlayerBody, ctx: &mut CombatContext, dt: f32) -> bool {
        let alive = self.behavior.update(&mut self.body, player, ctx, dt);
        alive && self.body.hp > 0.0
    }

    pub fn draw(&self, cam_offset: Vec2) {
        let sp = self.body.pos + cam_offset;
        self.behavior.draw_custom(&self.body, sp);

        // Thanh máu / trạng thái của công trình
        let bw = self.body.radius * 2.2;
        let pct = (self.body.hp / self.body.max_hp.max(1.0)).clamp(0.0, 1.0);
        draw_rectangle(
            sp.x - bw / 2.0,
            sp.y - self.body.radius - 14.0,
            bw,
            4.0,
            Color::new(0.1, 0.12, 0.18, 0.85),
        );
        draw_rectangle(
            sp.x - bw / 2.0,
            sp.y - self.body.radius - 14.0,
            bw * pct,
            4.0,
            self.body.color,
        );
        draw_text(
            self.body.name,
            sp.x - bw * 0.65,
            sp.y - self.body.radius - 18.0,
            12.0,
            self.body.color,
        );
    }
}

// =========================================================
// 1. LÔ CỐT TƯỜNG CHẮN NĂNG LƯỢNG (Energy Barricade Bunker)
// =========================================================
pub struct EnergyBarricadeStructure {
    pub pulse: f32,
}

impl BattlefieldStructure for EnergyBarricadeStructure {
    fn update(
        &mut self,
        body: &mut StructureBody,
        _player: &mut PlayerBody,
        _ctx: &mut CombatContext,
        dt: f32,
    ) -> bool {
        self.pulse += dt * 3.0;
        // Tự hồi nhẹ khiên chắn theo thời gian
        if body.hp < body.max_hp {
            body.hp = (body.hp + 8.0 * dt).min(body.max_hp);
        }
        true
    }

    fn draw_custom(&self, body: &StructureBody, sp: Vec2) {
        let pulse_w = (self.pulse.sin()) * 2.0;
        // Lô cốt tường chắn chữ H bọc thép (Armored Bunker Bastion)
        let w = body.radius * 2.1;
        let h = body.radius * 1.45;
        let x = sp.x - w / 2.0;
        let y = sp.y - h / 2.0;
        draw_rectangle(
            x - 4.0 - pulse_w,
            y - 4.0 - pulse_w,
            w + 8.0 + pulse_w * 2.0,
            h + 8.0 + pulse_w * 2.0,
            Color::new(0.0, 0.78, 1.0, 0.18),
        );
        draw_rectangle(x, y, w, h, Color::new(0.08, 0.16, 0.26, 0.96));
        draw_rectangle_lines(x, y, w, h, 2.5, body.color);
        // Hai trụ góc lô cốt & khe bắn năng lượng
        draw_rectangle(x - 3.0, y - 3.0, 9.0, h + 6.0, body.color);
        draw_rectangle(x + w - 6.0, y - 3.0, 9.0, h + 6.0, body.color);
        draw_line(x + 8.0, sp.y, x + w - 8.0, sp.y, 3.0, WHITE);
    }

    fn blocks_enemy_projectiles(&self) -> bool {
        true
    }
}

// =========================================================
// 2. TRẠM RADAR VỆ TINH CHIẾM ĐÓNG (Radar Outpost Capture Zone)
// =========================================================
pub struct RadarOutpostStructure {
    pub capture_radius: f32,
    pub progress: f32,
    pub captured: bool,
    pub strike_timer: f32,
    pub radar_angle: f32,
}

impl BattlefieldStructure for RadarOutpostStructure {
    fn update(
        &mut self,
        body: &mut StructureBody,
        player: &mut PlayerBody,
        ctx: &mut CombatContext,
        dt: f32,
    ) -> bool {
        self.radar_angle += dt * 2.4;
        let dist_player = (player.pos - body.pos).length();

        if !self.captured {
            if dist_player <= self.capture_radius {
                self.progress = (self.progress + dt * 0.24).min(1.0);
                if self.progress >= 1.0 {
                    self.captured = true;
                    body.name = "RADAR [ONLINE]";
                    body.color = Color::new(1.0, 0.84, 0.0, 1.0);

                    // Thưởng ngay 1 Khoang Tiếp Tế Súng Epic/Legendary!
                    let rarity = if rand::gen_range(0, 2) == 0 {
                        WeaponRarity::Legendary
                    } else {
                        WeaponRarity::Epic
                    };
                    ctx.spawn_event(Box::new(SupplyPodDropEvent {
                        landing_pos: body.pos + vec2(0.0, 68.0),
                        timer: 0.0,
                        max_time: 0.85,
                        weapon_id: rand::gen_range(2, 7),
                        rarity,
                    }));
                    ctx.spawn_event(Box::new(ShockwaveRingEvent {
                        center: body.pos,
                        current_radius: 12.0,
                        max_radius: 180.0,
                        speed: 520.0,
                        color: Color::new(1.0, 0.85, 0.1, 0.9),
                        thickness: 5.0,
                    }));
                    ctx.spawn_float_text(
                        body.pos - vec2(0.0, 36.0),
                        tr(
                            "ĐÃ CHIẾM TRẠM RADAR! VỆ TINH TIẾP TẾ LÕI SÚNG!",
                            "RADAR OUTPOST CAPTURED! SATELLITE ONLINE!",
                        ),
                        Color::new(1.0, 0.85, 0.0, 1.0),
                        21.0,
                    );
                }
            }
        } else {
            // Khi đã chiếm đóng: Tự động phóng sét vệ tinh đánh quái xung quanh mỗi 1.35s
            self.strike_timer += dt;
            if self.strike_timer >= 1.35 {
                if let Some(target) = ctx.find_nearest_enemy(body.pos, 520.0) {
                    self.strike_timer = 0.0;
                    let dir = (target - body.pos).normalize_or_zero();
                    ctx.chain_lightning(body.pos, dir, 4, 280.0, 68.0);
                }
            }
        }
        true
    }

    fn draw_custom(&self, body: &StructureBody, sp: Vec2) {
        let ring_col = if self.captured {
            Color::new(1.0, 0.84, 0.0, 0.4)
        } else {
            Color::new(0.0, 0.9, 1.0, 0.32)
        };

        // Vùng chiếm đóng hình Bát Giác Chiến Thuật
        draw_poly(
            sp.x,
            sp.y,
            8,
            self.capture_radius * self.progress.max(0.12),
            22.5,
            Color::new(ring_col.r, ring_col.g, ring_col.b, 0.08),
        );
        draw_poly_lines(sp.x, sp.y, 8, self.capture_radius, 22.5, 2.0, ring_col);

        if !self.captured {
            draw_text(
                &format!(
                    "{}: {:.0}%",
                    tr("GIỮ VÙNG RADAR", "CAPTURING RADAR"),
                    self.progress * 100.0
                ),
                sp.x - 62.0,
                sp.y + body.radius + 22.0,
                14.0,
                Color::new(0.0, 0.95, 1.0, 0.95),
            );
        }

        // Bệ tháp Radar hình vuông có 4 chân trụ góc
        let r = body.radius;
        draw_rectangle(sp.x - r, sp.y - r, r * 2.0, r * 2.0, Color::new(0.08, 0.12, 0.2, 0.96));
        draw_rectangle_lines(sp.x - r, sp.y - r, r * 2.0, r * 2.0, 2.5, body.color);

        // Chảo thu phát sóng Vệ Tinh chữ V xoay tròn
        let dish_dir = vec2(self.radar_angle.cos(), self.radar_angle.sin());
        let perp = vec2(-dish_dir.y, dish_dir.x);
        let tip = sp + dish_dir * (r + 9.0);
        let dish_l = sp + dish_dir * 4.0 + perp * 12.0;
        let dish_r = sp + dish_dir * 4.0 - perp * 12.0;
        draw_line(dish_l.x, dish_l.y, dish_r.x, dish_r.y, 3.5, body.color);
        draw_line(sp.x, sp.y, tip.x, tip.y, 3.0, WHITE);
    }
}

// =========================================================
// 3. TRỤ HỒI PHỤC NANO CHIẾN TRƯỜNG (Nano Repair Pylon)
// =========================================================
pub struct NanoRepairPylonStructure {
    pub aura_radius: f32,
    pub pulse: f32,
}

impl BattlefieldStructure for NanoRepairPylonStructure {
    fn update(
        &mut self,
        body: &mut StructureBody,
        player: &mut PlayerBody,
        _ctx: &mut CombatContext,
        dt: f32,
    ) -> bool {
        self.pulse += dt * 3.5;
        if (player.pos - body.pos).length() <= self.aura_radius {
            player.hp = (player.hp + 12.0 * dt).min(player.max_hp);
            player.shield = (player.shield + 20.0 * dt).min(player.max_shield);
        }
        true
    }

    fn draw_custom(&self, body: &StructureBody, sp: Vec2) {
        let wave_r = self.aura_radius * (0.88 + 0.12 * self.pulse.sin());
        draw_poly(sp.x, sp.y, 6, wave_r, self.pulse * 12.0, Color::new(0.1, 1.0, 0.55, 0.07));
        draw_poly_lines(sp.x, sp.y, 6, self.aura_radius, 0.0, 1.8, Color::new(0.1, 1.0, 0.55, 0.45));

        // Đài hồi phục hình Chữ Thập Lớn bọc viền lục giác (không dùng hình tròn)
        draw_poly(sp.x, sp.y, 6, body.radius + 2.0, 30.0, Color::new(0.05, 0.2, 0.14, 0.96));
        draw_poly_lines(sp.x, sp.y, 6, body.radius + 2.0, 30.0, 2.2, body.color);
        draw_rectangle(sp.x - 4.5, sp.y - 13.0, 9.0, 26.0, body.color);
        draw_rectangle(sp.x - 13.0, sp.y - 4.5, 26.0, 9.0, body.color);
    }
}
