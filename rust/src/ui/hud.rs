use macroquad::prelude::*;
use crate::character::player::Player;

pub fn draw_hud(
    player: &Player,
    wave: u32,
    score: u64,
    combo: u32,
    enemies_left: u32,
    active_boss: Option<(&str, f32, f32)>,
) {
    // Top Left: Wave, Score, Enemies
    draw_rectangle(20.0, 20.0, 140.0, 50.0, Color::new(0.05, 0.07, 0.12, 0.85));
    draw_rectangle_lines(20.0, 20.0, 140.0, 50.0, 1.0, Color::new(0.0, 0.94, 1.0, 0.3));
    draw_text("WAVE", 30.0, 40.0, 16.0, Color::new(0.6, 0.7, 0.8, 1.0));
    draw_text(&format!("{}", wave), 30.0, 62.0, 22.0, Color::new(0.0, 0.94, 1.0, 1.0));

    draw_rectangle(170.0, 20.0, 220.0, 50.0, Color::new(0.05, 0.07, 0.12, 0.85));
    draw_rectangle_lines(170.0, 20.0, 220.0, 50.0, 1.0, Color::new(0.0, 0.94, 1.0, 0.3));
    draw_text("SCORE", 180.0, 40.0, 16.0, Color::new(0.6, 0.7, 0.8, 1.0));
    draw_text(&format!("{}", score), 180.0, 62.0, 22.0, Color::new(1.0, 0.84, 0.0, 1.0));
    draw_text(&format!("x{}", combo), 330.0, 62.0, 18.0, Color::new(0.0, 1.0, 0.4, 1.0));

    draw_rectangle(400.0, 20.0, 150.0, 50.0, Color::new(0.05, 0.07, 0.12, 0.85));
    draw_rectangle_lines(400.0, 20.0, 150.0, 50.0, 1.0, Color::new(0.0, 0.94, 1.0, 0.3));
    draw_text("ENEMIES", 410.0, 40.0, 16.0, Color::new(0.6, 0.7, 0.8, 1.0));
    draw_text(&format!("{}", enemies_left), 410.0, 62.0, 22.0, Color::new(1.0, 0.2, 0.27, 1.0));

    // Boss HP Bar
    if let Some((boss_name, hp, max_hp)) = active_boss {
        let bw = 420.0;
        let bx = screen_width() / 2.0 - bw / 2.0;
        let by = 84.0;
        draw_rectangle(bx, by, bw, 40.0, Color::new(0.12, 0.02, 0.05, 0.9));
        draw_rectangle_lines(bx, by, bw, 40.0, 1.5, Color::new(1.0, 0.0, 0.35, 0.8));
        draw_text(
            &format!("BOSS: {}", boss_name),
            bx + 14.0,
            by + 18.0,
            16.0,
            Color::new(1.0, 0.2, 0.35, 1.0),
        );
        let pct = (hp / max_hp.max(1.0)).clamp(0.0, 1.0);
        draw_rectangle(bx + 14.0, by + 24.0, bw - 28.0, 10.0, Color::new(0.2, 0.05, 0.1, 1.0));
        draw_rectangle(
            bx + 14.0,
            by + 24.0,
            (bw - 28.0) * pct,
            10.0,
            Color::new(1.0, 0.0, 0.35, 1.0),
        );
    }

    // Bottom Left: Vitals
    let by = screen_height() - 122.0;
    draw_rectangle(20.0, by, 280.0, 102.0, Color::new(0.05, 0.07, 0.12, 0.85));
    draw_rectangle_lines(20.0, by, 280.0, 102.0, 1.0, Color::new(0.0, 0.94, 1.0, 0.3));

    draw_text(
        &format!("CLASS: {}", player.class.name()),
        30.0,
        by + 16.0,
        13.0,
        Color::new(0.0, 0.94, 1.0, 1.0),
    );

    // HP Bar
    draw_text(
        &format!("HP: {:.0} / {:.0}", player.body.hp, player.body.max_hp),
        30.0,
        by + 34.0,
        14.0,
        WHITE,
    );
    draw_rectangle(30.0, by + 40.0, 260.0, 8.0, Color::new(0.2, 0.2, 0.2, 1.0));
    draw_rectangle(
        30.0,
        by + 40.0,
        260.0 * (player.body.hp / player.body.max_hp).clamp(0.0, 1.0),
        8.0,
        Color::new(1.0, 0.2, 0.27, 1.0),
    );

    // Shield Bar
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
    draw_rectangle(30.0, by + 68.0, 260.0, 8.0, Color::new(0.2, 0.2, 0.2, 1.0));
    draw_rectangle(
        30.0,
        by + 68.0,
        260.0 * (player.body.shield / player.body.max_shield).clamp(0.0, 1.0),
        8.0,
        Color::new(0.0, 0.94, 1.0, 1.0),
    );

    // Dash & Bullet Time
    let dash_pct = player.dash_cd_ratio();
    draw_text("DASH [SPACE]", 30.0, by + 90.0, 12.0, Color::new(1.0, 0.8, 0.0, 1.0));
    draw_rectangle(30.0, by + 94.0, 120.0, 5.0, Color::new(0.2, 0.2, 0.2, 1.0));
    draw_rectangle(30.0, by + 94.0, 120.0 * dash_pct, 5.0, Color::new(1.0, 0.8, 0.0, 1.0));

    draw_text("TIME [SHIFT]", 165.0, by + 90.0, 12.0, Color::new(0.7, 0.4, 1.0, 1.0));
    draw_rectangle(165.0, by + 94.0, 125.0, 5.0, Color::new(0.2, 0.2, 0.2, 1.0));
    draw_rectangle(
        165.0,
        by + 94.0,
        125.0 * (player.body.stamina / player.body.max_stamina).clamp(0.0, 1.0),
        5.0,
        Color::new(0.7, 0.4, 1.0, 1.0),
    );

    // Bottom Right: Ammo & Active Reload
    let w = player.current_weapon();
    let reload = &player.reload_state;
    let bx = screen_width() - 250.0;
    draw_rectangle(bx, by, 230.0, 90.0, Color::new(0.05, 0.07, 0.12, 0.85));
    draw_rectangle_lines(bx, by, 230.0, 90.0, 1.0, Color::new(0.0, 0.94, 1.0, 0.3));

    draw_text(
        w.stats.name,
        bx + 16.0,
        by + 26.0,
        20.0,
        Color::new(0.0, 0.94, 1.0, 1.0),
    );
    let reserve_str = if w.stats.reserve_ammo < 0 {
        "INF".to_string()
    } else {
        format!("{}", w.stats.reserve_ammo)
    };
    draw_text(
        &format!("{} / {}", w.stats.current_clip, reserve_str),
        bx + 16.0,
        by + 56.0,
        28.0,
        WHITE,
    );

    if reload.is_reloading {
        let rw = 198.0;
        let progress = (reload.timer / reload.duration).clamp(0.0, 1.0);
        draw_rectangle(bx + 16.0, by + 68.0, rw, 12.0, Color::new(0.1, 0.15, 0.2, 1.0));
        draw_rectangle(
            bx + 16.0 + rw * 0.58,
            by + 68.0,
            rw * 0.24,
            12.0,
            Color::new(0.0, 1.0, 0.4, 0.5),
        );
        draw_rectangle(bx + 16.0 + rw * progress, by + 66.0, 4.0, 16.0, WHITE);
    } else {
        draw_text(
            "[R] ACTIVE RELOAD",
            bx + 16.0,
            by + 80.0,
            13.0,
            Color::new(0.6, 0.7, 0.8, 1.0),
        );
    }

    // Center Skills (Dynamic via SkillBehavior trait)
    let mut sx = screen_width() / 2.0 - 210.0;
    for s in &player.skills {
        let status = if s.cooldown <= 0.0 {
            "READY".to_string()
        } else {
            format!("{:.1}s", s.cooldown)
        };
        draw_text(
            &format!("[{}] {}: {}", s.behavior.key_label(), s.behavior.name(), status),
            sx,
            screen_height() - 30.0,
            14.0,
            Color::new(0.0, 0.94, 1.0, 1.0),
        );
        sx += 155.0;
    }
}
