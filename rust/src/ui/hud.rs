use macroquad::prelude::*;
use crate::ally::AlliedEntity;
use crate::character::player::Player;
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

    // 2. Vũ khí rơi dưới đất (Chấm sáng theo độ hiếm)
    for wd in weapon_drops {
        let mp = to_map(wd.pos);
        draw_circle(mp.x, mp.y, 2.8, wd.rarity.color());
    }

    // 3. Đồng minh (Chấm xanh lục)
    for a in allies {
        let mp = to_map(a.body.pos);
        draw_circle(mp.x, mp.y, 2.2, Color::new(0.2, 1.0, 0.6, 0.95));
    }

    // 4. Quái vật & Boss
    for e in enemies {
        let mp = to_map(e.body.pos);
        if e.body.is_boss {
            draw_circle(mp.x, mp.y, 5.0, Color::new(1.0, 0.1, 0.35, 1.0));
            draw_circle_lines(mp.x, mp.y, 7.0, 1.2, WHITE);
        } else {
            draw_circle(mp.x, mp.y, 1.8, Color::new(1.0, 0.25, 0.3, 0.85));
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
    active_boss: Option<(&str, f32, f32, usize)>,
    settings: &GameSettings,
) -> bool {
    // Top Left: Wave, Score, Enemies, Allies
    draw_rectangle(20.0, 20.0, 115.0, 50.0, Color::new(0.05, 0.07, 0.12, 0.85));
    draw_rectangle_lines(20.0, 20.0, 115.0, 50.0, 1.0, Color::new(0.0, 0.94, 1.0, 0.3));
    draw_text("WAVE", 30.0, 38.0, 15.0, Color::new(0.6, 0.7, 0.8, 1.0));
    draw_text(&format!("{}", wave), 30.0, 61.0, 22.0, Color::new(0.0, 0.94, 1.0, 1.0));

    draw_rectangle(145.0, 20.0, 210.0, 50.0, Color::new(0.05, 0.07, 0.12, 0.85));
    draw_rectangle_lines(145.0, 20.0, 210.0, 50.0, 1.0, Color::new(0.0, 0.94, 1.0, 0.3));
    draw_text("SCORE", 155.0, 38.0, 15.0, Color::new(0.6, 0.7, 0.8, 1.0));
    draw_text(&format!("{}", score), 155.0, 61.0, 22.0, Color::new(1.0, 0.84, 0.0, 1.0));
    draw_text(&format!("x{}", combo), 295.0, 61.0, 18.0, Color::new(0.0, 1.0, 0.4, 1.0));

    draw_rectangle(365.0, 20.0, 130.0, 50.0, Color::new(0.05, 0.07, 0.12, 0.85));
    draw_rectangle_lines(365.0, 20.0, 130.0, 50.0, 1.0, Color::new(0.0, 0.94, 1.0, 0.3));
    draw_text("ENEMIES", 375.0, 38.0, 15.0, Color::new(0.6, 0.7, 0.8, 1.0));
    draw_text(&format!("{}", enemies_left), 375.0, 61.0, 22.0, Color::new(1.0, 0.2, 0.27, 1.0));

    draw_rectangle(505.0, 20.0, 120.0, 50.0, Color::new(0.05, 0.07, 0.12, 0.85));
    draw_rectangle_lines(505.0, 20.0, 120.0, 50.0, 1.0, Color::new(0.2, 1.0, 0.6, 0.3));
    draw_text("ALLIES", 515.0, 38.0, 15.0, Color::new(0.6, 0.7, 0.8, 1.0));
    draw_text(&format!("{}", allies_count), 515.0, 61.0, 22.0, Color::new(0.2, 1.0, 0.6, 1.0));

    // Top Right: Commander Hotkeys + Settings Quick Button
    let settings_btn_w = 185.0;
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
    draw_text(
        "[ESC / O] CAI DAT",
        settings_btn_x + 14.0,
        40.0,
        15.0,
        Color::new(0.0, 0.95, 1.0, 1.0),
    );
    draw_text(
        &format!("Rung: {:.0}% | VFX: {:.0}%", settings.screen_shake_intensity * 100.0, settings.particle_density * 100.0),
        settings_btn_x + 14.0,
        60.0,
        13.0,
        Color::new(0.75, 0.88, 1.0, 1.0),
    );

    let cmd_w = 275.0;
    let rx = settings_btn_x - cmd_w - 10.0;
    draw_rectangle(rx, 20.0, cmd_w, 50.0, Color::new(0.05, 0.07, 0.12, 0.85));
    draw_rectangle_lines(rx, 20.0, cmd_w, 50.0, 1.0, Color::new(1.0, 0.8, 0.0, 0.4));
    draw_text("[G] GOI HOM TIEP TE SUNG", rx + 12.0, 40.0, 13.0, Color::new(1.0, 0.85, 0.1, 1.0));
    draw_text("[B] TRIEU HOI DAI BOSS PHAO DAI", rx + 12.0, 60.0, 13.0, Color::new(1.0, 0.3, 0.4, 1.0));

    // Boss HP Bar
    if let Some((boss_name, hp, max_hp, parts_left)) = active_boss {
        let bw = 460.0;
        let bx = screen_width() / 2.0 - bw / 2.0;
        let by = 82.0;
        draw_rectangle(bx, by, bw, 44.0, Color::new(0.12, 0.02, 0.05, 0.92));
        draw_rectangle_lines(bx, by, bw, 44.0, 1.5, Color::new(1.0, 0.0, 0.35, 0.85));
        draw_text(
            &format!("{} (MODULES ACTIVE: {})", boss_name, parts_left),
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
        &format!("CLASS: {}", player.class.name()),
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
            "SHIELD: {:.0} / {:.0}",
            player.body.shield, player.body.max_shield
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

    // Bottom Center: Weapon Hotbar [1..7] & Skills
    let slot_w = 72.0;
    let total_w = (player.weapons.len() as f32) * (slot_w + 6.0);
    let start_wx = screen_width() / 2.0 - total_w / 2.0;
    let wy = screen_height() - 95.0;

    for (idx, w) in player.weapons.iter().enumerate() {
        let sx = start_wx + (idx as f32) * (slot_w + 6.0);
        let is_selected = idx == player.current_weapon_idx;
        let bg = if !w.stats.unlocked {
            Color::new(0.04, 0.05, 0.08, 0.5)
        } else if is_selected {
            Color::new(0.0, 0.32, 0.45, 0.92)
        } else {
            Color::new(0.06, 0.09, 0.14, 0.85)
        };
        let border = if !w.stats.unlocked {
            Color::new(0.2, 0.25, 0.3, 0.3)
        } else if is_selected {
            w.stats.rarity.color()
        } else {
            Color::new(0.3, 0.4, 0.5, 0.5)
        };

        draw_rectangle(sx, wy, slot_w, 46.0, bg);
        draw_rectangle_lines(sx, wy, slot_w, 46.0, if is_selected { 2.0 } else { 1.0 }, border);
        draw_text(&format!("[{}]", idx + 1), sx + 5.0, wy + 14.0, 12.0, LIGHTGRAY);

        if w.stats.unlocked {
            let short_name: String = w.stats.name.chars().take(7).collect();
            draw_text(&short_name, sx + 5.0, wy + 29.0, 12.0, w.stats.rarity.color());
            draw_text(
                &format!("Lv.{}", w.stats.level),
                sx + 5.0,
                wy + 42.0,
                12.0,
                WHITE,
            );
        } else {
            draw_text("LOCKED", sx + 10.0, wy + 31.0, 12.0, DARKGRAY);
        }
    }

    // Skills bar below weapon slots
    let mut sk_x = screen_width() / 2.0 - 230.0;
    for s in &player.skills {
        let status = if s.cooldown <= 0.0 {
            "READY".to_string()
        } else {
            format!("{:.1}s", s.cooldown)
        };
        draw_text(
            &format!("[{}] {}: {}", s.behavior.key_label(), s.behavior.name(), status),
            sk_x,
            screen_height() - 22.0,
            14.0,
            Color::new(0.0, 0.94, 1.0, 1.0),
        );
        sk_x += 175.0;
    }

    // Bottom Right: Current Weapon Details & Active Reload
    let w = player.current_weapon();
    let reload = &player.reload_state;
    let bx = screen_width() - 255.0;
    draw_rectangle(bx, by, 235.0, 102.0, Color::new(0.05, 0.07, 0.12, 0.85));
    draw_rectangle_lines(bx, by, 235.0, 102.0, 1.5, w.stats.rarity.color());

    draw_text(
        &format!("{} Lv.{}", w.stats.name, w.stats.level),
        bx + 14.0,
        by + 24.0,
        18.0,
        w.stats.rarity.color(),
    );
    let reserve_str = if w.stats.reserve_ammo < 0 {
        "INF".to_string()
    } else {
        format!("{}", w.stats.reserve_ammo)
    };
    draw_text(
        &format!("{} / {}", w.stats.current_clip, reserve_str),
        bx + 14.0,
        by + 56.0,
        28.0,
        WHITE,
    );

    if reload.is_reloading {
        let rw = 204.0;
        let progress = (reload.timer / reload.duration).clamp(0.0, 1.0);
        draw_rectangle(bx + 14.0, by + 72.0, rw, 12.0, Color::new(0.1, 0.15, 0.2, 1.0));
        draw_rectangle(
            bx + 14.0 + rw * 0.58,
            by + 72.0,
            rw * 0.24,
            12.0,
            Color::new(0.0, 1.0, 0.4, 0.5),
        );
        draw_rectangle(bx + 14.0 + rw * progress, by + 70.0, 4.0, 16.0, WHITE);
    } else {
        draw_text(
            "[R] ACTIVE RELOAD | [F] NHAT SUNG",
            bx + 14.0,
            by + 84.0,
            12.0,
            Color::new(0.6, 0.7, 0.8, 1.0),
        );
    }

    hover_settings && is_mouse_button_pressed(MouseButton::Left)
}
