use macroquad::prelude::*;
use crate::core::types::UpgradeOption;

pub fn draw_start_menu(class_name: &str, class_desc: &str) -> bool {
    clear_background(Color::new(0.04, 0.05, 0.08, 1.0));
    let cx = screen_width() / 2.0;
    let cy = screen_height() / 2.0;

    draw_text(
        "CYBER SURVIVOR",
        cx - 240.0,
        cy - 140.0,
        56.0,
        Color::new(0.0, 0.94, 1.0, 1.0),
    );
    draw_text(
        &format!("NHAN VAT: {} - {}", class_name, class_desc),
        cx - 310.0,
        cy - 95.0,
        18.0,
        Color::new(0.7, 0.85, 1.0, 1.0),
    );

    draw_text(
        "BAN PHIM: [W][A][S][D] Di chuyen  |  [SPACE] Dash ne don (I-frames)",
        cx - 280.0,
        cy - 30.0,
        20.0,
        WHITE,
    );
    draw_text(
        "[R] Active Reload (Nhan dung vach de tang 40% dmg) | [SHIFT] Bullet Time",
        cx - 280.0,
        cy + 5.0,
        20.0,
        Color::new(0.0, 1.0, 0.4, 1.0),
    );
    draw_text(
        "[1..5] Doi vu khi  |  [Q] Luu dan no chum  |  [E] Dat tru sung tu dong",
        cx - 280.0,
        cy + 40.0,
        20.0,
        Color::new(1.0, 0.85, 0.0, 1.0),
    );
    draw_text(
        "CHUOT: [Chuot Trai] Ban 360  |  [Chuot Phai] Chem dao day lui",
        cx - 280.0,
        cy + 75.0,
        20.0,
        Color::new(1.0, 0.4, 0.4, 1.0),
    );

    let btn_hover = mouse_position().0 > cx - 140.0
        && mouse_position().0 < cx + 140.0
        && mouse_position().1 > cy + 125.0
        && mouse_position().1 < cy + 180.0;
    let btn_col = if btn_hover {
        Color::new(0.0, 1.0, 0.8, 1.0)
    } else {
        Color::new(0.0, 0.8, 1.0, 1.0)
    };
    draw_rectangle(cx - 140.0, cy + 125.0, 280.0, 55.0, btn_col);
    draw_text("BAT DAU CHIEN DAU [ENTER]", cx - 120.0, cy + 160.0, 20.0, BLACK);

    is_key_pressed(KeyCode::Enter)
        || is_key_pressed(KeyCode::Space)
        || (btn_hover && is_mouse_button_pressed(MouseButton::Left))
}

pub fn draw_upgrade_menu(choices: &[UpgradeOption]) -> Option<&'static str> {
    clear_background(Color::new(0.06, 0.08, 0.12, 1.0));
    let cx = screen_width() / 2.0;
    let cy = screen_height() / 2.0;

    draw_text(
        "NANG CAP CONG NGHE ROGUELITE",
        cx - 230.0,
        cy - 140.0,
        36.0,
        Color::new(0.0, 0.94, 1.0, 1.0),
    );
    draw_text(
        "Chon 1 trong 3 mo-dun de tiep tuc Wave tiep theo:",
        cx - 210.0,
        cy - 100.0,
        18.0,
        Color::new(0.7, 0.8, 0.9, 1.0),
    );

    let card_w = 270.0;
    let card_h = 200.0;
    let start_x = cx - (3.0 * card_w + 2.0 * 24.0) / 2.0;

    for (i, opt) in choices.iter().enumerate() {
        let rx = start_x + (i as f32) * (card_w + 24.0);
        let ry = cy - 60.0;
        let hover = mouse_position().0 > rx
            && mouse_position().0 < rx + card_w
            && mouse_position().1 > ry
            && mouse_position().1 < ry + card_h;

        let bg_col = if hover {
            Color::new(0.0, 0.3, 0.45, 0.9)
        } else {
            Color::new(0.08, 0.12, 0.18, 0.9)
        };
        let border_col = if hover {
            Color::new(0.0, 1.0, 0.8, 1.0)
        } else {
            Color::new(0.0, 0.6, 0.8, 0.5)
        };

        draw_rectangle(rx, ry, card_w, card_h, bg_col);
        draw_rectangle_lines(rx, ry, card_w, card_h, 2.0, border_col);

        draw_text(opt.icon, rx + 16.0, ry + 40.0, 32.0, WHITE);
        draw_text(
            opt.title,
            rx + 16.0,
            75.0 + ry,
            20.0,
            Color::new(1.0, 0.84, 0.0, 1.0),
        );
        draw_text(
            opt.desc,
            rx + 16.0,
            110.0 + ry,
            14.0,
            Color::new(0.8, 0.9, 1.0, 1.0),
        );

        if hover && is_mouse_button_pressed(MouseButton::Left) {
            return Some(opt.id);
        }
    }
    None
}

pub fn draw_pause_menu() {
    let cx = screen_width() / 2.0;
    let cy = screen_height() / 2.0;
    draw_rectangle(
        cx - 200.0,
        cy - 80.0,
        400.0,
        160.0,
        Color::new(0.05, 0.07, 0.12, 0.95),
    );
    draw_rectangle_lines(
        cx - 200.0,
        cy - 80.0,
        400.0,
        160.0,
        2.0,
        Color::new(0.0, 0.94, 1.0, 1.0),
    );
    draw_text(
        "TAM DUNG TRAN DAU",
        cx - 130.0,
        cy - 20.0,
        28.0,
        Color::new(0.0, 0.94, 1.0, 1.0),
    );
    draw_text("Bam [ESC] hoac [P] de tiep tuc", cx - 110.0, cy + 25.0, 18.0, WHITE);
}

pub fn draw_gameover_menu(
    wave: u32,
    score: u64,
    total_kills: u32,
    shots_fired: u64,
    shots_hit: u64,
) -> bool {
    clear_background(Color::new(0.08, 0.02, 0.04, 1.0));
    let cx = screen_width() / 2.0;
    let cy = screen_height() / 2.0;

    draw_text(
        "MISSION FAILED",
        cx - 190.0,
        cy - 100.0,
        52.0,
        Color::new(1.0, 0.1, 0.2, 1.0),
    );
    draw_text(&format!("WAVE DAT DUOC: {}", wave), cx - 120.0, cy - 30.0, 24.0, WHITE);
    draw_text(
        &format!("TONG DIEM: {}", score),
        cx - 120.0,
        cy + 5.0,
        24.0,
        Color::new(1.0, 0.84, 0.0, 1.0),
    );
    draw_text(
        &format!("QUAI DA HA: {}", total_kills),
        cx - 120.0,
        cy + 40.0,
        24.0,
        Color::new(0.0, 0.94, 1.0, 1.0),
    );

    let acc = if shots_fired > 0 {
        (shots_hit as f32 / shots_fired as f32 * 100.0) as u32
    } else {
        0
    };
    draw_text(
        &format!("DO CHINH XAC: {}%", acc),
        cx - 120.0,
        cy + 75.0,
        24.0,
        Color::new(0.0, 1.0, 0.4, 1.0),
    );

    draw_text("Bam [ENTER] hoac [R] de choi lai", cx - 140.0, cy + 140.0, 22.0, WHITE);

    is_key_pressed(KeyCode::Enter) || is_key_pressed(KeyCode::R)
}

pub fn get_random_upgrades() -> Vec<UpgradeOption> {
    let mut pool = vec![
        UpgradeOption {
            id: "bounce",
            title: "DAN PHAN XA",
            icon: "[O]",
            desc: "Dan nay lai 2 lan khi va vao tuong.",
        },
        UpgradeOption {
            id: "pierce",
            title: "DAN XUYEN THAU",
            icon: "[>]",
            desc: "Tang them +1 lan ban xuyen muc tieu.",
        },
        UpgradeOption {
            id: "vamp",
            title: "HUT NANG LUONG",
            icon: "[+]",
            desc: "Hoi +2.5 HP moi khi ha guc quai vat.",
        },
        UpgradeOption {
            id: "burn",
            title: "DAN NHIET PLASMA",
            icon: "[*]",
            desc: "Dan gay hieu ung dot chay (Burn DPS).",
        },
        UpgradeOption {
            id: "cryo",
            title: "DAN BANG CRYO",
            icon: "[~]",
            desc: "Dan lam cham 45% toc do di chuyen quai.",
        },
        UpgradeOption {
            id: "heavy",
            title: "DAN HANG NANG",
            icon: "[!]",
            desc: "Tang vinh vien +25% sat thuong vu khi.",
        },
        UpgradeOption {
            id: "dash_fire",
            title: "LUOT LUA",
            icon: "[^]",
            desc: "Luot (Space) de lai vet lua thieu dot.",
        },
        UpgradeOption {
            id: "overclock",
            title: "EP XUNG TRU SUNG",
            icon: "[#]",
            desc: "Tru sung [E] ban nhanh gap doi & ben hon.",
        },
    ];

    // Fisher-Yates shuffle
    for i in (1..pool.len()).rev() {
        let j = rand::gen_range(0, i + 1);
        pool.swap(i, j);
    }
    pool.truncate(3);
    pool
}
