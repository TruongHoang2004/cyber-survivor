use macroquad::prelude::*;
use crate::ally::AlliedEntity;
use crate::character::player::Player;
use crate::core::i18n::{draw_text, draw_text_fitted, tr};
use crate::core::props::WeaponDrop;
use crate::core::structures::TacticalStructureEntity;
use crate::core::types::{GameSettings, WORLD_HEIGHT, WORLD_WIDTH};
use crate::enemy::traits::EnemyEntity;

/// Vẽ Bản đồ nhỏ chiến thuật (Tactical Radar Minimap) ở góc trên bên phải
pub fn draw_minimap(
    player_pos: Vec2,
    enemies: &[EnemyEntity],
    allies: &[AlliedEntity],
    structures: &[TacticalStructureEntity],
    weapon_drops: &[WeaponDrop],
) {
    let map_sz = 142.0;
    let mx = screen_width() - map_sz - 20.0;
    let my = 80.0;

    draw_rectangle(mx, my, map_sz, map_sz, Color::new(0.03, 0.05, 0.09, 0.88));
    draw_rectangle_lines(mx, my, map_sz, map_sz, 1.5, Color::new(0.0, 0.9, 1.0, 0.55));

    // Đường chữ thập tọa độ tâm Radar
    draw_line(
        mx + map_sz / 2.0,
        my,
        mx + map_sz / 2.0,
        my + map_sz,
        1.0,
        Color::new(0.0, 0.8, 1.0, 0.12),
    );
    draw_line(
        mx,
        my + map_sz / 2.0,
        mx + map_sz,
        my + map_sz / 2.0,
        1.0,
        Color::new(0.0, 0.8, 1.0, 0.12),
    );

    let to_map = |p: Vec2| -> Vec2 {
        vec2(
            mx + (p.x / WORLD_WIDTH).clamp(0.0, 1.0) * map_sz,
            my + (p.y / WORLD_HEIGHT).clamp(0.0, 1.0) * map_sz,
        )
    };

    // 1. Công trình / Trạm Radar (Hình vuông xanh/vàng)
    for st in structures {
        let mp = to_map(st.body.pos);
        draw_rectangle(mp.x - 3.0, mp.y - 3.0, 6.0, 6.0, st.body.color);
    }

    // 2. Lõi nâng cấp súng rơi dưới đất (Chấm sáng theo độ hiếm)
    for wd in weapon_drops {
        let mp = to_map(wd.pos);
        draw_circle(mp.x, mp.y, 2.8, wd.rarity.color());
    }

    // 3. Đồng minh (Chấm xanh lục)
    for a in allies {
        let mp = to_map(a.body.pos);
        draw_circle(mp.x, mp.y, 2.2, Color::new(0.2, 1.0, 0.6, 0.95));
    }

    // 4. Quái vật & Boss (Tự động lấy mẫu mượt mà khi có đàn quái 10,000 con)
    let step = if enemies.len() > 1000 {
        enemies.len() / 320
    } else {
        1
    };
    for (idx, e) in enemies.iter().enumerate() {
        if e.body.is_boss {
            let mp = to_map(e.body.pos);
            draw_circle(mp.x, mp.y, 5.0, Color::new(1.0, 0.1, 0.35, 1.0));
            draw_circle_lines(mp.x, mp.y, 7.0, 1.2, WHITE);
        } else if idx % step == 0 {
            let mp = to_map(e.body.pos);
            draw_circle(mp.x, mp.y, 1.6, Color::new(1.0, 0.25, 0.3, 0.85));
        }
    }

    // 5. Người chơi (Chấm tròn trắng viền Cyan)
    let pp = to_map(player_pos);
    draw_circle(pp.x, pp.y, 3.6, Color::new(0.0, 0.95, 1.0, 1.0));
    draw_circle_lines(pp.x, pp.y, 5.2, 1.2, WHITE);

    draw_text("RADAR", mx + 6.0, my + 13.0, 11.0, Color::new(0.0, 0.94, 1.0, 0.8));
}

/// Vẽ HUD và trả về `true` nếu người chơi bấm vào nút [CÀI ĐẶT / MENU] trên góc phải màn hình
pub fn draw_hud(
    player: &Player,
    wave: u32,
    score: u64,
    combo: u32,
    enemies_left: u32,
    allies_count: usize,
    tech_level: u32,
    exp_ratio: f32,
    active_boss: Option<(&str, f32, f32, usize)>,
    settings: &GameSettings,
) -> bool {
    // Top Left: Wave, Score, Enemies, Allies
    draw_rectangle(20.0, 20.0, 115.0, 50.0, Color::new(0.05, 0.07, 0.12, 0.85));
    draw_rectangle_lines(20.0, 20.0, 115.0, 50.0, 1.0, Color::new(0.0, 0.94, 1.0, 0.3));
    draw_text(
        tr("ĐỢT (WAVE)", "WAVE"),
        30.0,
        38.0,
        14.0,
        Color::new(0.6, 0.7, 0.8, 1.0),
    );
    draw_text(&format!("{}", wave), 30.0, 61.0, 22.0, Color::new(0.0, 0.94, 1.0, 1.0));

    draw_rectangle(145.0, 20.0, 210.0, 50.0, Color::new(0.05, 0.07, 0.12, 0.85));
    draw_rectangle_lines(145.0, 20.0, 210.0, 50.0, 1.0, Color::new(0.0, 0.94, 1.0, 0.3));
    draw_text(
        tr("ĐIỂM SỐ", "SCORE"),
        155.0,
        38.0,
        14.0,
        Color::new(0.6, 0.7, 0.8, 1.0),
    );
    draw_text(&format!("{}", score), 155.0, 61.0, 22.0, Color::new(1.0, 0.84, 0.0, 1.0));
    draw_text(&format!("x{}", combo), 295.0, 61.0, 18.0, Color::new(0.0, 1.0, 0.4, 1.0));

    draw_rectangle(365.0, 20.0, 130.0, 50.0, Color::new(0.05, 0.07, 0.12, 0.85));
    draw_rectangle_lines(365.0, 20.0, 130.0, 50.0, 1.0, Color::new(0.0, 0.94, 1.0, 0.3));
    draw_text(
        tr("QUÁI CÒN LẠI", "ENEMIES"),
        375.0,
        38.0,
        14.0,
        Color::new(0.6, 0.7, 0.8, 1.0),
    );
    draw_text(&format!("{}", enemies_left), 375.0, 61.0, 22.0, Color::new(1.0, 0.2, 0.27, 1.0));

    draw_rectangle(505.0, 20.0, 120.0, 50.0, Color::new(0.05, 0.07, 0.12, 0.85));
    draw_rectangle_lines(505.0, 20.0, 120.0, 50.0, 1.0, Color::new(0.2, 1.0, 0.6, 0.3));
    draw_text(
        tr("ĐỒNG MINH", "ALLIES"),
        515.0,
        38.0,
        14.0,
        Color::new(0.6, 0.7, 0.8, 1.0),
    );
    draw_text(&format!("{}", allies_count), 515.0, 61.0, 22.0, Color::new(0.2, 1.0, 0.6, 1.0));

    // Thanh Kinh Nghiệm Nâng Cấp Công Nghệ (Tech Upgrade Progress Bar) ngay dưới hàng trên
    draw_rectangle(20.0, 76.0, 605.0, 20.0, Color::new(0.04, 0.06, 0.11, 0.88));
    draw_rectangle(
        22.0,
        78.0,
        601.0 * exp_ratio.clamp(0.0, 1.0),
        16.0,
        Color::new(0.0, 0.85, 0.55, 0.75),
    );
    draw_rectangle_lines(20.0, 76.0, 605.0, 20.0, 1.0, Color::new(0.0, 1.0, 0.65, 0.55));
    draw_text_fitted(
        &format!(
            "{} LV.{} — {}: {:.0}%",
            tr("CẤP CÔNG NGHỆ", "TECH LEVEL"),
            tech_level,
            tr(
                "TIẾN ĐỘ MỞ THẺ KỸ NĂNG TIẾP THEO",
                "NEXT SKILL UPGRADE PROGRESS"
            ),
            exp_ratio.clamp(0.0, 1.0) * 100.0
        ),
        28.0,
        90.0,
        588.0,
        12.0,
        WHITE,
    );

    // Top Right: Commander Hotkeys + Settings Quick Button
    let settings_btn_w = 195.0;
    let settings_btn_x = screen_width() - settings_btn_w - 20.0;
    let (mx, my) = mouse_position();
    let hover_settings = mx >= settings_btn_x
        && mx <= settings_btn_x + settings_btn_w
        && my >= 20.0
        && my <= 70.0;

    draw_rectangle(
        settings_btn_x,
        20.0,
        settings_btn_w,
        50.0,
        if hover_settings {
            Color::new(0.0, 0.35, 0.5, 0.95)
        } else {
            Color::new(0.05, 0.09, 0.16, 0.9)
        },
    );
    draw_rectangle_lines(
        settings_btn_x,
        20.0,
        settings_btn_w,
        50.0,
        1.5,
        if hover_settings {
            Color::new(0.0, 1.0, 0.85, 1.0)
        } else {
            Color::new(0.0, 0.85, 1.0, 0.5)
        },
    );
    draw_text_fitted(
        tr("[ESC / O] CÀI ĐẶT", "[ESC / O] SETTINGS"),
        settings_btn_x + 10.0,
        39.0,
        settings_btn_w - 20.0,
        14.0,
        Color::new(0.0, 0.95, 1.0, 1.0),
    );
    draw_text_fitted(
        &format!(
            "[L] {} | {}: {:.0}%",
            settings.language.short_code(),
            tr("Rung", "Shake"),
            settings.screen_shake_intensity * 100.0
        ),
        settings_btn_x + 10.0,
        59.0,
        settings_btn_w - 20.0,
        12.0,
        Color::new(0.75, 0.88, 1.0, 1.0),
    );

    let cmd_w = 340.0;
    let rx = settings_btn_x - cmd_w - 10.0;
    draw_rectangle(rx, 20.0, cmd_w, 50.0, Color::new(0.05, 0.07, 0.12, 0.85));
    draw_rectangle_lines(rx, 20.0, cmd_w, 50.0, 1.0, Color::new(0.0, 0.95, 0.7, 0.55));
    draw_text_fitted(
        &format!("[TAB] {}: {}", tr("NGẮM", "AIM"), settings.aim_mode_label()),
        rx + 10.0,
        38.0,
        cmd_w - 20.0,
        13.0,
        Color::new(0.0, 1.0, 0.75, 1.0),
    );
    draw_text_fitted(
        &format!(
            "[V] {}: {} | [F] {}",
            tr("TỰ BẮN", "AUTO"),
            if settings.auto_fire { "ON" } else { "OFF" },
            tr("NHẶT LÕI SÚNG", "PICKUP CORE")
        ),
        rx + 10.0,
        58.0,
        cmd_w - 20.0,
        12.0,
        Color::new(1.0, 0.85, 0.15, 1.0),
    );

    // Boss HP Bar
    if let Some((boss_name, hp, max_hp, parts_left)) = active_boss {
        let bw = 460.0;
        let bx = screen_width() / 2.0 - bw / 2.0;
        let by = 82.0;
        draw_rectangle(bx, by, bw, 44.0, Color::new(0.12, 0.02, 0.05, 0.92));
        draw_rectangle_lines(bx, by, bw, 44.0, 1.5, Color::new(1.0, 0.0, 0.35, 0.85));
        draw_text(
            &format!(
                "{} ({}: {})",
                boss_name,
                tr("GIÁP PHỤ CÒN", "MODULES"),
                parts_left
            ),
            bx + 14.0,
            by + 18.0,
            15.0,
            Color::new(1.0, 0.3, 0.4, 1.0),
        );
        let pct = (hp / max_hp.max(1.0)).clamp(0.0, 1.0);
        draw_rectangle(
            bx + 14.0,
            by + 26.0,
            bw - 28.0,
            10.0,
            Color::new(0.2, 0.05, 0.1, 1.0),
        );
        draw_rectangle(
            bx + 14.0,
            by + 26.0,
            (bw - 28.0) * pct,
            10.0,
            Color::new(1.0, 0.0, 0.35, 1.0),
        );
    }

    // Bottom Left: Vitals
    let by = screen_height() - 122.0;
    draw_rectangle(20.0, by, 270.0, 102.0, Color::new(0.05, 0.07, 0.12, 0.85));
    draw_rectangle_lines(20.0, by, 270.0, 102.0, 1.0, Color::new(0.0, 0.94, 1.0, 0.3));

    draw_text(
        &format!("{}: {}", tr("LỚP", "CLASS"), player.class.name()),
        30.0,
        by + 16.0,
        13.0,
        Color::new(0.0, 0.94, 1.0, 1.0),
    );

    draw_text(
        &format!("HP: {:.0} / {:.0}", player.body.hp, player.body.max_hp),
        30.0,
        by + 34.0,
        14.0,
        WHITE,
    );
    draw_rectangle(30.0, by + 40.0, 250.0, 8.0, Color::new(0.2, 0.2, 0.2, 1.0));
    draw_rectangle(
        30.0,
        by + 40.0,
        250.0 * (player.body.hp / player.body.max_hp).clamp(0.0, 1.0),
        8.0,
        Color::new(1.0, 0.2, 0.27, 1.0),
    );

    draw_text(
        &format!(
            "{}: {:.0} / {:.0}",
            tr("GIÁP", "SHIELD"),
            player.body.shield,
            player.body.max_shield
        ),
        30.0,
        by + 62.0,
        14.0,
        WHITE,
    );
    draw_rectangle(30.0, by + 68.0, 250.0, 8.0, Color::new(0.2, 0.2, 0.2, 1.0));
    draw_rectangle(
        30.0,
        by + 68.0,
        250.0 * (player.body.shield / player.body.max_shield).clamp(0.0, 1.0),
        8.0,
        Color::new(0.0, 0.94, 1.0, 1.0),
    );

    let dash_pct = player.dash_cd_ratio();
    draw_text("DASH [SPACE]", 30.0, by + 90.0, 12.0, Color::new(1.0, 0.8, 0.0, 1.0));
    draw_rectangle(30.0, by + 94.0, 115.0, 5.0, Color::new(0.2, 0.2, 0.2, 1.0));
    draw_rectangle(30.0, by + 94.0, 115.0 * dash_pct, 5.0, Color::new(1.0, 0.8, 0.0, 1.0));

    draw_text("TIME [SHIFT]", 160.0, by + 90.0, 12.0, Color::new(0.7, 0.4, 1.0, 1.0));
    draw_rectangle(160.0, by + 94.0, 120.0, 5.0, Color::new(0.2, 0.2, 0.2, 1.0));
    draw_rectangle(
        160.0,
        by + 94.0,
        120.0 * (player.body.stamina / player.body.max_stamina).clamp(0.0, 1.0),
        5.0,
        Color::new(0.7, 0.4, 1.0, 1.0),
    );

    // Bottom Center: Tiến Độ Tiến Hóa Vũ Khí 500 Cấp (Mỗi 100 Cấp 1 Form Mới -> Lv.500 Tối Thượng) & Phạm Vi Hút EXP
    let w_cur = player.current_weapon();
    let panel_w = 540.0;
    let panel_h = 58.0;
    let start_wx = screen_width() / 2.0 - panel_w / 2.0;
    let wy = screen_height() - 104.0;
    draw_rectangle(start_wx, wy, panel_w, panel_h, Color::new(0.05, 0.08, 0.14, 0.92));
    draw_rectangle_lines(start_wx, wy, panel_w, panel_h, 1.5, w_cur.stats.rarity.color());

    let form_ratio = (w_cur.stats.level as f32 / 500.0).clamp(0.0, 1.0);
    let cur_tier = ((w_cur.stats.level / 100).min(5) + 1).min(6);
    let next_bound = (((w_cur.stats.level / 100) + 1) * 100).min(500);
    let form_status = if w_cur.stats.level >= 500 {
        format!(
            "[{}] - {}",
            w_cur.stats.rarity.label(),
            tr("ĐẠT ĐỈNH PHONG TỐI THƯỢNG [MAX]!", "MAX ULTIMATE APEX REACHED!")
        )
    } else {
        format!(
            "[{}] ({} +{} Lv -> Form {})",
            w_cur.stats.rarity.label(),
            tr("Cần", "Need"),
            next_bound.saturating_sub(w_cur.stats.level),
            cur_tier + 1
        )
    };

    // Hàng 1: Tên Form súng + Cấp độ (trái) và Trạng thái Tiến Hóa Form (phải)
    let w_title = format!("{} Lv.{}/500", w_cur.stats.name, w_cur.stats.level);
    draw_text_fitted(
        &w_title,
        start_wx + 12.0,
        wy + 17.0,
        270.0,
        13.5,
        w_cur.stats.rarity.color(),
    );
    draw_text_fitted(
        &form_status,
        start_wx + 290.0,
        wy + 17.0,
        panel_w - 302.0,
        12.0,
        if w_cur.stats.level >= 500 {
            Color::new(0.2, 1.0, 0.85, 1.0)
        } else if w_cur.stats.level >= 400 {
            Color::new(1.0, 0.35, 0.45, 1.0)
        } else if w_cur.stats.level >= 100 {
            YELLOW
        } else {
            Color::new(0.85, 0.9, 1.0, 1.0)
        },
    );

    // Hàng 2: Thanh tiến trình 500 cấp tới [TỐI THƯỢNG] (có 5 vạch chia từng 100 cấp)
    draw_rectangle(
        start_wx + 12.0,
        wy + 24.0,
        panel_w - 24.0,
        6.0,
        Color::new(0.12, 0.16, 0.24, 1.0),
    );
    draw_rectangle(
        start_wx + 12.0,
        wy + 24.0,
        (panel_w - 24.0) * form_ratio,
        6.0,
        w_cur.stats.rarity.color(),
    );
    // Vạch đánh dấu các mốc Form 100, 200, 300, 400
    for m in 1..=4 {
        let mark_x = start_wx + 12.0 + (panel_w - 24.0) * ((m as f32) / 5.0);
        draw_line(mark_x, wy + 23.0, mark_x, wy + 31.0, 1.5, Color::new(0.6, 0.7, 0.9, 0.7));
    }

    // Hàng 3: Tầm hút tinh thể EXP & Hệ số kinh nghiệm
    draw_text_fitted(
        &format!(
            "{}: {:.0}% ({:.0}px) | EXP: {:.0}%",
            tr("TẦM HÚT TINH THỂ EXP", "EXP MAGNET RANGE"),
            player.body.modifiers.magnet_radius_multi * 100.0,
            95.0 * player.body.modifiers.magnet_radius_multi,
            player.body.modifiers.exp_gain_multi * 100.0
        ),
        start_wx + 12.0,
        wy + 47.0,
        panel_w - 24.0,
        11.5,
        Color::new(0.0, 1.0, 0.85, 1.0),
    );

    // Skills bar below weapon panel
    let mut sk_x = screen_width() / 2.0 - 230.0;
    for s in &player.skills {
        let status = if s.cooldown <= 0.0 {
            tr("SẴN SÀNG", "READY").to_string()
        } else {
            format!("{:.1}s", s.cooldown)
        };
        draw_text_fitted(
            &format!("[{}] {}: {}", s.behavior.key_label(), s.behavior.name(), status),
            sk_x,
            screen_height() - 20.0,
            170.0,
            12.5,
            Color::new(0.0, 0.94, 1.0, 1.0),
        );
        sk_x += 175.0;
    }

    // Bottom Right: Current Weapon Details & Active Reload
    let w = player.current_weapon();
    let reload = &player.reload_state;
    let bx = screen_width() - 265.0;
    draw_rectangle(bx, by, 245.0, 102.0, Color::new(0.05, 0.07, 0.12, 0.85));
    draw_rectangle_lines(bx, by, 245.0, 102.0, 1.5, w.stats.rarity.color());

    draw_text(
        &format!("{} Lv.{}/500", w.stats.name, w.stats.level),
        bx + 12.0,
        by + 24.0,
        14.5,
        w.stats.rarity.color(),
    );
    let reserve_str = if w.stats.reserve_ammo < 0 {
        "INF".to_string()
    } else {
        format!("{}", w.stats.reserve_ammo)
    };
    draw_text(
        &format!("{} / {}", w.stats.current_clip, reserve_str),
        bx + 12.0,
        by + 56.0,
        28.0,
        WHITE,
    );

    if reload.is_reloading {
        let rw = 216.0;
        let progress = (reload.timer / reload.duration).clamp(0.0, 1.0);
        draw_rectangle(bx + 12.0, by + 72.0, rw, 12.0, Color::new(0.1, 0.15, 0.2, 1.0));
        draw_rectangle(
            bx + 12.0 + rw * 0.58,
            by + 72.0,
            rw * 0.24,
            12.0,
            Color::new(0.0, 1.0, 0.4, 0.5),
        );
        draw_rectangle(bx + 12.0 + rw * progress, by + 70.0, 4.0, 16.0, WHITE);
    } else {
        draw_text(
            tr(
                "[R] CANH VẠCH XANH TRÊN ĐẦU",
                "[R] HIT OVERHEAD GREEN ZONE",
            ),
            bx + 12.0,
            by + 84.0,
            12.0,
            Color::new(0.6, 0.7, 0.8, 1.0),
        );
    }

    hover_settings && is_mouse_button_pressed(MouseButton::Left)
}
