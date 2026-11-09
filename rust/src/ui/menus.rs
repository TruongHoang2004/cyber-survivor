use macroquad::prelude::*;
use crate::core::types::{GameSettings, UpgradeOption};

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum StartMenuAction {
    None,
    SelectClass(usize),
    StartGame,
    OpenSettings,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum PauseMenuAction {
    None,
    Resume,
    OpenSettings,
    Restart,
    MainMenu,
}

/// Vẽ nút bấm chiến thuật tương tác chuột
fn draw_tactical_button(x: f32, y: f32, w: f32, h: f32, label: &str, accent: Color) -> bool {
    let (mx, my) = mouse_position();
    let hover = mx >= x && mx <= x + w && my >= y && my <= y + h;

    let bg = if hover {
        Color::new(accent.r * 0.35, accent.g * 0.35, accent.b * 0.35, 0.95)
    } else {
        Color::new(0.06, 0.09, 0.15, 0.92)
    };
    let border = if hover {
        WHITE
    } else {
        Color::new(accent.r, accent.g, accent.b, 0.8)
    };

    draw_rectangle(x, y, w, h, bg);
    draw_rectangle_lines(x, y, w, h, if hover { 2.5 } else { 1.5 }, border);

    // Điểm nhấn thanh dọc bên trái nút
    draw_rectangle(x, y, 5.0, h, accent);

    let text_dims = measure_text(label, None, 18, 1.0);
    draw_text(
        label,
        x + (w - text_dims.width) / 2.0,
        y + h / 2.0 + 6.0,
        18.0,
        if hover { WHITE } else { accent },
    );

    hover && is_mouse_button_pressed(MouseButton::Left)
}

/// Vẽ thanh trượt (Slider) có thể kéo trực tiếp bằng chuột + nút [-] [+] + nút tắt nhanh
fn draw_slider_row(
    x: f32,
    y: f32,
    w: f32,
    title: &str,
    subtitle: &str,
    value: &mut f32,
    min: f32,
    max: f32,
    step: f32,
    quick_zero_label: Option<&str>,
) {
    draw_rectangle(x, y, w, 62.0, Color::new(0.06, 0.09, 0.14, 0.92));
    draw_rectangle_lines(x, y, w, 62.0, 1.0, Color::new(0.0, 0.85, 1.0, 0.25));

    draw_text(title, x + 16.0, y + 25.0, 18.0, WHITE);
    draw_text(
        subtitle,
        x + 16.0,
        y + 46.0,
        13.0,
        Color::new(0.6, 0.72, 0.85, 1.0),
    );

    let (mx, my) = mouse_position();

    // Nút tắt nhanh (ví dụ: [TẮT 0%])
    if let Some(q_label) = quick_zero_label {
        let qx = x + w - 395.0;
        let qy = y + 15.0;
        let qw = 82.0;
        let qh = 32.0;
        let q_hover = mx >= qx && mx <= qx + qw && my >= qy && my <= qy + qh;
        let is_zero = *value <= min + 0.001;
        draw_rectangle(
            qx,
            qy,
            qw,
            qh,
            if is_zero {
                Color::new(1.0, 0.25, 0.3, 0.85)
            } else if q_hover {
                Color::new(0.2, 0.3, 0.4, 0.9)
            } else {
                Color::new(0.1, 0.14, 0.2, 0.9)
            },
        );
        draw_rectangle_lines(qx, qy, qw, qh, 1.0, Color::new(1.0, 0.4, 0.4, 0.7));
        draw_text(q_label, qx + 10.0, qy + 21.0, 14.0, WHITE);
        if q_hover && is_mouse_button_pressed(MouseButton::Left) {
            *value = min;
        }
    }

    // Nút [-]
    let minus_x = x + w - 300.0;
    let btn_y = y + 15.0;
    let btn_sz = 32.0;
    let hover_minus =
        mx >= minus_x && mx <= minus_x + btn_sz && my >= btn_y && my <= btn_y + btn_sz;
    draw_rectangle(
        minus_x,
        btn_y,
        btn_sz,
        btn_sz,
        if hover_minus {
            Color::new(0.0, 0.45, 0.65, 0.95)
        } else {
            Color::new(0.1, 0.16, 0.24, 0.95)
        },
    );
    draw_rectangle_lines(minus_x, btn_y, btn_sz, btn_sz, 1.0, SKYBLUE);
    draw_text("-", minus_x + 11.0, btn_y + 22.0, 22.0, WHITE);
    if hover_minus && is_mouse_button_pressed(MouseButton::Left) {
        *value = (*value - step).clamp(min, max);
    }

    // Thanh kéo (Slider Bar)
    let bar_x = minus_x + btn_sz + 10.0;
    let bar_w = 145.0;
    let bar_y = y + 23.0;
    let bar_h = 16.0;

    let hover_bar =
        mx >= bar_x - 6.0 && mx <= bar_x + bar_w + 6.0 && my >= y + 10.0 && my <= y + 52.0;
    if hover_bar && is_mouse_button_down(MouseButton::Left) {
        let ratio = ((mx - bar_x) / bar_w).clamp(0.0, 1.0);
        *value = min + ratio * (max - min);
    }

    let pct = ((*value - min) / (max - min)).clamp(0.0, 1.0);
    draw_rectangle(bar_x, bar_y, bar_w, bar_h, Color::new(0.03, 0.05, 0.08, 1.0));
    draw_rectangle(
        bar_x,
        bar_y,
        bar_w * pct,
        bar_h,
        Color::new(0.0, 0.9, 1.0, 0.85),
    );
    draw_rectangle_lines(bar_x, bar_y, bar_w, bar_h, 1.0, Color::new(0.0, 0.8, 1.0, 0.6));

    // Đầu kéo (Knob)
    let knob_x = bar_x + bar_w * pct;
    draw_rectangle(knob_x - 5.0, bar_y - 4.0, 10.0, bar_h + 8.0, WHITE);

    // Nút [+]
    let plus_x = bar_x + bar_w + 10.0;
    let hover_plus = mx >= plus_x && mx <= plus_x + btn_sz && my >= btn_y && my <= btn_y + btn_sz;
    draw_rectangle(
        plus_x,
        btn_y,
        btn_sz,
        btn_sz,
        if hover_plus {
            Color::new(0.0, 0.45, 0.65, 0.95)
        } else {
            Color::new(0.1, 0.16, 0.24, 0.95)
        },
    );
    draw_rectangle_lines(plus_x, btn_y, btn_sz, btn_sz, 1.0, SKYBLUE);
    draw_text("+", plus_x + 9.0, btn_y + 22.0, 22.0, WHITE);
    if hover_plus && is_mouse_button_pressed(MouseButton::Left) {
        *value = (*value + step).clamp(min, max);
    }

    // Hiển thị phần trăm
    draw_text(
        &format!("{:.0}%", *value * 100.0),
        plus_x + btn_sz + 12.0,
        y + 37.0,
        18.0,
        Color::new(1.0, 0.85, 0.0, 1.0),
    );
}

/// Vẽ công tắc Bật/Tắt (Toggle Row)
fn draw_toggle_row(
    x: f32,
    y: f32,
    w: f32,
    title: &str,
    subtitle: &str,
    value: &mut bool,
    on_text: &str,
    off_text: &str,
) {
    draw_rectangle(x, y, w, 56.0, Color::new(0.06, 0.09, 0.14, 0.92));
    draw_rectangle_lines(x, y, w, 56.0, 1.0, Color::new(0.0, 0.85, 1.0, 0.25));

    draw_text(title, x + 16.0, y + 23.0, 17.0, WHITE);
    draw_text(
        subtitle,
        x + 16.0,
        y + 43.0,
        13.0,
        Color::new(0.6, 0.72, 0.85, 1.0),
    );

    let btn_w = 175.0;
    let btn_h = 34.0;
    let bx = x + w - btn_w - 18.0;
    let by = y + 11.0;

    let (mx, my) = mouse_position();
    let hover = mx >= bx && mx <= bx + btn_w && my >= by && my <= by + btn_h;

    let bg = if *value {
        Color::new(0.0, 0.45, 0.3, 0.92)
    } else {
        Color::new(0.22, 0.1, 0.14, 0.92)
    };
    let border = if *value {
        Color::new(0.0, 1.0, 0.55, 1.0)
    } else {
        Color::new(1.0, 0.35, 0.4, 0.85)
    };

    draw_rectangle(bx, by, btn_w, btn_h, bg);
    draw_rectangle_lines(bx, by, btn_w, btn_h, if hover { 2.5 } else { 1.5 }, border);

    let label = if *value { on_text } else { off_text };
    let dims = measure_text(label, None, 15, 1.0);
    draw_text(
        label,
        bx + (btn_w - dims.width) / 2.0,
        by + 22.0,
        15.0,
        WHITE,
    );

    if hover && is_mouse_button_pressed(MouseButton::Left) {
        *value = !*value;
    }
}

/// Màn hình chính (Start Menu) với Chọn Nhân Vật (3 Classes), vào Trận và Cài đặt
pub fn draw_start_menu(
    selected_class_id: usize,
    class_name: &str,
    class_desc: &str,
    settings: &GameSettings,
) -> StartMenuAction {
    clear_background(Color::new(0.03, 0.04, 0.07, 1.0));

    // Lưới chiến thuật trang trí nền
    let grid_size = 64.0;
    let mut gx = 0.0;
    while gx < screen_width() {
        draw_line(gx, 0.0, gx, screen_height(), 1.0, Color::new(0.0, 0.8, 1.0, 0.04));
        gx += grid_size;
    }
    let mut gy = 0.0;
    while gy < screen_height() {
        draw_line(0.0, gy, screen_width(), gy, 1.0, Color::new(0.0, 0.8, 1.0, 0.04));
        gy += grid_size;
    }

    let cx = screen_width() / 2.0;
    let cy = screen_height() / 2.0;

    // Khung chính
    let panel_w = 920.0;
    let panel_h = 610.0;
    let px = cx - panel_w / 2.0;
    let py = cy - panel_h / 2.0;

    draw_rectangle(px, py, panel_w, panel_h, Color::new(0.05, 0.07, 0.12, 0.95));
    draw_rectangle_lines(px, py, panel_w, panel_h, 2.0, Color::new(0.0, 0.9, 1.0, 0.7));

    // Tiêu đề & High Score
    draw_text(
        "CYBER SURVIVOR: GRAND BATTLE",
        px + 34.0,
        py + 48.0,
        38.0,
        Color::new(0.0, 0.95, 1.0, 1.0),
    );
    draw_text(
        &format!("KY LUC (HIGH SCORE): {}", settings.high_score),
        px + panel_w - 310.0,
        py + 46.0,
        20.0,
        Color::new(1.0, 0.85, 0.1, 1.0),
    );
    draw_text(
        &format!("DANG CHON: {} — {}", class_name, class_desc),
        px + 34.0,
        py + 78.0,
        16.0,
        Color::new(0.75, 0.9, 1.0, 1.0),
    );

    // Hàng chọn 3 Lớp Nhân Vật (CharacterClass)
    let classes = [
        (
            0usize,
            "[1] CYBER MARINE",
            "HP 110 | Shield 60 | Khong Kich & Linh Thuy Quan",
            Color::new(0.0, 0.95, 1.0, 1.0),
        ),
        (
            1usize,
            "[2] VOID STALKER",
            "+25% DMG | Phase Blink Tao Ho Den & Bao Set 360",
            Color::new(0.8, 0.35, 1.0, 1.0),
        ),
        (
            2usize,
            "[3] TECH ENGINEER",
            "HP 125 | Shield 100 | Goi 2 Dai Robot Titan Mech",
            Color::new(1.0, 0.8, 0.15, 1.0),
        ),
    ];

    let card_w = (panel_w - 68.0 - 28.0) / 3.0;
    let card_y = py + 95.0;
    let card_h = 74.0;
    let (mx, my) = mouse_position();

    for (idx, title, sub, col) in classes {
        let card_x = px + 34.0 + (idx as f32) * (card_w + 14.0);
        let is_sel = selected_class_id == idx;
        let hover = mx >= card_x && mx <= card_x + card_w && my >= card_y && my <= card_y + card_h;

        draw_rectangle(
            card_x,
            card_y,
            card_w,
            card_h,
            if is_sel {
                Color::new(col.r * 0.25, col.g * 0.25, col.b * 0.25, 0.95)
            } else if hover {
                Color::new(0.1, 0.15, 0.24, 0.95)
            } else {
                Color::new(0.04, 0.06, 0.1, 0.9)
            },
        );
        draw_rectangle_lines(
            card_x,
            card_y,
            card_w,
            card_h,
            if is_sel { 2.5 } else { 1.0 },
            if is_sel { col } else { Color::new(col.r, col.g, col.b, 0.4) },
        );
        draw_text(title, card_x + 12.0, card_y + 28.0, 17.0, col);
        draw_text(sub, card_x + 12.0, card_y + 52.0, 12.0, WHITE);

        if hover && is_mouse_button_pressed(MouseButton::Left) {
            return StartMenuAction::SelectClass(idx);
        }
    }
    if is_key_pressed(KeyCode::Key1) {
        return StartMenuAction::SelectClass(0);
    }
    if is_key_pressed(KeyCode::Key2) {
        return StartMenuAction::SelectClass(1);
    }
    if is_key_pressed(KeyCode::Key3) {
        return StartMenuAction::SelectClass(2);
    }

    // Cột trái: Các nút điều khiển Menu
    let left_x = px + 34.0;
    let btn_w = 355.0;

    if draw_tactical_button(
        left_x,
        py + 188.0,
        btn_w,
        56.0,
        "BAT DAU CHIEN DAU [ENTER]",
        Color::new(0.0, 1.0, 0.65, 1.0),
    ) || is_key_pressed(KeyCode::Enter)
    {
        return StartMenuAction::StartGame;
    }

    if draw_tactical_button(
        left_x,
        py + 258.0,
        btn_w,
        52.0,
        "TUY CHINH CAI DAT [PHIM O]",
        Color::new(0.0, 0.88, 1.0, 1.0),
    ) || is_key_pressed(KeyCode::O)
    {
        return StartMenuAction::OpenSettings;
    }

    // Tóm tắt trạng thái cài đặt hiện tại ở góc dưới cột trái
    draw_rectangle(
        left_x,
        py + 325.0,
        btn_w,
        250.0,
        Color::new(0.04, 0.06, 0.1, 0.9),
    );
    draw_rectangle_lines(
        left_x,
        py + 325.0,
        btn_w,
        250.0,
        1.0,
        Color::new(0.0, 0.8, 1.0, 0.3),
    );
    draw_text(
        "THONG SO CAI DAT & TIEN HOA:",
        left_x + 16.0,
        py + 352.0,
        16.0,
        Color::new(1.0, 0.84, 0.0, 1.0),
    );
    draw_text(
        &format!(
            "- Am luong Synth: {:.0}% | Rung: {:.0}%",
            settings.master_volume * 100.0,
            settings.screen_shake_intensity * 100.0
        ),
        left_x + 16.0,
        py + 382.0,
        14.0,
        WHITE,
    );
    draw_text(
        &format!(
            "- Mat do VFX: {:.0}% | Dan quai: {:.0}%",
            settings.particle_density * 100.0,
            settings.swarm_scale * 100.0
        ),
        left_x + 16.0,
        py + 408.0,
        14.0,
        WHITE,
    );
    draw_text(
        "- TIEN HOA VU KHI: Nhat trung sung len Lv.3",
        left_x + 16.0,
        py + 436.0,
        14.0,
        Color::new(0.0, 1.0, 0.65, 1.0),
    );
    draw_text(
        "  se Tien Hoa thanh Sieu Vu Khi [EVO]!",
        left_x + 16.0,
        py + 456.0,
        14.0,
        Color::new(0.0, 1.0, 0.65, 1.0),
    );
    draw_text(
        "- TRAM RADAR & LO COT: Dung giu vung Radar",
        left_x + 16.0,
        py + 484.0,
        14.0,
        Color::new(1.0, 0.85, 0.2, 1.0),
    );
    draw_text(
        "  de goi Ve Tinh ban set & nhan Sung Huyen Thoai!",
        left_x + 16.0,
        py + 504.0,
        14.0,
        Color::new(1.0, 0.85, 0.2, 1.0),
    );
    draw_text(
        "Bam [ESC] hoac [O] trong tran de chinh ngay!",
        left_x + 16.0,
        py + 548.0,
        13.0,
        Color::new(0.0, 0.95, 1.0, 1.0),
    );

    // Cột phải: Bảng Hướng Dẫn Chiến Trường & Phím Tắt
    let right_x = px + 410.0;
    let right_w = 476.0;
    draw_rectangle(
        right_x,
        py + 188.0,
        right_w,
        387.0,
        Color::new(0.04, 0.06, 0.1, 0.9),
    );
    draw_rectangle_lines(
        right_x,
        py + 188.0,
        right_w,
        387.0,
        1.0,
        Color::new(0.0, 0.8, 1.0, 0.3),
    );

    draw_text(
        "CAM NANG TAC CHIEN QUY MO LON:",
        right_x + 18.0,
        py + 218.0,
        18.0,
        Color::new(0.0, 0.95, 1.0, 1.0),
    );

    let guides = [
        ("[1][2][3] (Menu)", "Chon 1 trong 3 Chuyen Gia truoc tran"),
        ("[W][A][S][D]", "Di chuyen chien thuat 8 huong"),
        ("[CHUOT TRAI / PHAI]", "Ban vu khi / Chem dao day lui"),
        ("[SPACE] / [SHIFT]", "Dash / Phase Blink / Bullet Time"),
        ("[1] .. [7]", "Chuyen 7 loai vu khi (Lv.3 -> Tien Hoa EVO)"),
        ("[F]", "Nhat hinh chieu Sung roi tren mat dat"),
        ("[R]", "Active Reload (Bam dung vach xanh +40% DMG)"),
        ("[Q] / [E]", "Ky nang Huy Diet & Goi Binh Doan Tiep Vien"),
        ("[G] / [B]", "Tha Hom Sung ngay / Trieu hoi Dai Boss"),
        ("[ESC] / [O]", "Tam dung & Mo Menu Cai Dat bat cu luc nao"),
    ];

    for (i, (key, desc)) in guides.iter().enumerate() {
        let gy = py + 252.0 + (i as f32) * 31.0;
        draw_text(key, right_x + 18.0, gy, 15.0, Color::new(1.0, 0.85, 0.1, 1.0));
        draw_text(desc, right_x + 182.0, gy, 15.0, WHITE);
    }

    StartMenuAction::None
}

/// Menu Tạm Dừng (Pause Menu)
pub fn draw_pause_menu() -> PauseMenuAction {
    // Lớp phủ mờ toàn màn hình
    draw_rectangle(
        0.0,
        0.0,
        screen_width(),
        screen_height(),
        Color::new(0.02, 0.03, 0.06, 0.72),
    );

    let cx = screen_width() / 2.0;
    let cy = screen_height() / 2.0;
    let pw = 440.0;
    let ph = 360.0;
    let px = cx - pw / 2.0;
    let py = cy - ph / 2.0;

    draw_rectangle(px, py, pw, ph, Color::new(0.05, 0.08, 0.14, 0.96));
    draw_rectangle_lines(px, py, pw, ph, 2.0, Color::new(0.0, 0.94, 1.0, 0.9));

    draw_text(
        "TAM DUNG CHIEN TRUONG",
        px + 68.0,
        py + 48.0,
        28.0,
        Color::new(0.0, 0.94, 1.0, 1.0),
    );

    let bw = pw - 64.0;
    let bx = px + 32.0;

    if draw_tactical_button(
        bx,
        py + 78.0,
        bw,
        48.0,
        "TIEP TUC TRAN DAU [ESC / P]",
        Color::new(0.0, 1.0, 0.6, 1.0),
    ) {
        return PauseMenuAction::Resume;
    }

    if draw_tactical_button(
        bx,
        py + 140.0,
        bw,
        48.0,
        "TUY CHINH CAI DAT [PHIM O]",
        Color::new(0.0, 0.88, 1.0, 1.0),
    ) || is_key_pressed(KeyCode::O)
    {
        return PauseMenuAction::OpenSettings;
    }

    if draw_tactical_button(
        bx,
        py + 202.0,
        bw,
        48.0,
        "CHOI LAI TU DAU [RESTART]",
        Color::new(1.0, 0.8, 0.1, 1.0),
    ) {
        return PauseMenuAction::Restart;
    }

    if draw_tactical_button(
        bx,
        py + 264.0,
        bw,
        48.0,
        "VE MAN HINH CHINH [MAIN MENU]",
        Color::new(1.0, 0.35, 0.4, 1.0),
    ) {
        return PauseMenuAction::MainMenu;
    }

    PauseMenuAction::None
}

/// Bảng Tuỳ Chỉnh Cài Đặt (Settings Modal) — Trả về `true` khi đóng bảng
pub fn draw_settings_modal(settings: &mut GameSettings) -> bool {
    draw_rectangle(
        0.0,
        0.0,
        screen_width(),
        screen_height(),
        Color::new(0.01, 0.02, 0.05, 0.82),
    );

    let cx = screen_width() / 2.0;
    let cy = screen_height() / 2.0;
    let mw = 840.0;
    let mh = 670.0;
    let mx = cx - mw / 2.0;
    let my = cy - mh / 2.0;

    draw_rectangle(mx, my, mw, mh, Color::new(0.04, 0.06, 0.11, 0.98));
    draw_rectangle_lines(mx, my, mw, mh, 2.0, Color::new(0.0, 0.94, 1.0, 0.9));

    draw_text(
        "TUY CHINH CAI DAT AM THANH, HINH ANH & CHIEN TRUONG",
        mx + 28.0,
        my + 38.0,
        24.0,
        Color::new(0.0, 0.95, 1.0, 1.0),
    );
    draw_text(
        "Keo thanh truot hoac bam [-] / [+] (Tu dong luu ra file khi dong):",
        mx + 28.0,
        my + 60.0,
        14.0,
        Color::new(0.7, 0.82, 0.95, 1.0),
    );

    let row_x = mx + 28.0;
    let row_w = mw - 56.0;
    let mut cur_y = my + 72.0;

    // 1. Âm lượng tổng (Master Volume)
    draw_slider_row(
        row_x,
        cur_y,
        row_w,
        "1. AM LUONG TONG HOP (MASTER SYNTH VOLUME)",
        "Dieu chinh am luong tieng sung, tia set Tesla, bom no & nang cap",
        &mut settings.master_volume,
        0.0,
        1.0,
        0.10,
        Some("TAT 0%"),
    );
    cur_y += 65.0;

    // 2. Độ rung màn hình (Screen Shake)
    draw_slider_row(
        row_x,
        cur_y,
        row_w,
        "2. DO RUNG MAN HINH (SCREEN SHAKE)",
        "Giam hoac tat han rung cam khi no bom, ban Railgun, Laser Quy Dao",
        &mut settings.screen_shake_intensity,
        0.0,
        1.0,
        0.05,
        Some("TAT 0%"),
    );
    cur_y += 65.0;

    // 3. Mật độ hiệu ứng nổ (Particle Density)
    draw_slider_row(
        row_x,
        cur_y,
        row_w,
        "3. MAT DO HIEU UNG HAT CHAY NO (VFX DENSITY)",
        "Giam bot tia lua de nhin ro duong dan va quai vat trong dai chien",
        &mut settings.particle_density,
        0.1,
        1.0,
        0.10,
        None,
    );
    cur_y += 65.0;

    // 4. Độ sáng lưới nền (Arena Grid)
    draw_slider_row(
        row_x,
        cur_y,
        row_w,
        "4. DO SANG LUOI CHIEN TRUONG (GRID OPACITY)",
        "Dieu chinh do ro cua luoi toa do duoi mat dat",
        &mut settings.grid_opacity,
        0.0,
        1.0,
        0.10,
        Some("TAT 0%"),
    );
    cur_y += 65.0;

    // 5. Quy mô đàn quái (Swarm Scale)
    draw_slider_row(
        row_x,
        cur_y,
        row_w,
        "5. QUY MO DAN QUAI (BATTLE SWARM SCALE)",
        "Tang/giam so luong quai vat xuat hien moi dot (50% Nhe -> 200% Dai Chien)",
        &mut settings.swarm_scale,
        0.5,
        2.0,
        0.25,
        None,
    );
    cur_y += 67.0;

    // Hai cột cho 4 nút Toggle bên dưới
    let half_w = (row_w - 16.0) / 2.0;

    draw_toggle_row(
        row_x,
        cur_y,
        half_w,
        "HIEN SO SAT THUONG",
        "So DMG bay len khi ban trung",
        &mut settings.show_damage_numbers,
        "DANG BAT",
        "DANG TAT",
    );
    draw_toggle_row(
        row_x + half_w + 16.0,
        cur_y,
        half_w,
        "TU DONG NHAT SUNG",
        "Tat de chi nhat khi bam [F]",
        &mut settings.auto_pickup_weapons,
        "TU DONG + [F]",
        "CHI BAM [F]",
    );
    cur_y += 61.0;

    draw_toggle_row(
        row_x,
        cur_y,
        half_w,
        "TU DONG CAM SUNG MOI",
        "Chuyen ngay sang sung vua mo khoa",
        &mut settings.auto_equip_new_weapon,
        "DANG BAT",
        "DANG TAT",
    );
    draw_toggle_row(
        row_x + half_w + 16.0,
        cur_y,
        half_w,
        "BAN DO RADAR MINIMAP",
        "Hien Radar vi tri Boss, Sung & Tram",
        &mut settings.show_minimap,
        "DANG BAT",
        "DANG TAT",
    );
    cur_y += 66.0;

    // Hàng nút dưới cùng: Khôi phục mặc định & Lưu/Đóng
    if draw_tactical_button(
        row_x,
        cur_y,
        320.0,
        46.0,
        "KHOI PHUC MAC DINH [RESET]",
        Color::new(1.0, 0.75, 0.1, 1.0),
    ) {
        let hs = settings.high_score;
        *settings = GameSettings::default();
        settings.high_score = hs;
        settings.save_to_disk();
    }

    let close_clicked = draw_tactical_button(
        row_x + row_w - 360.0,
        cur_y,
        360.0,
        46.0,
        "LUU & QUAY LAI [ESC]",
        Color::new(0.0, 1.0, 0.65, 1.0),
    );

    if close_clicked || is_key_pressed(KeyCode::Escape) {
        settings.save_to_disk();
        true
    } else {
        false
    }
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
        cy - 120.0,
        52.0,
        Color::new(1.0, 0.1, 0.2, 1.0),
    );
    draw_text(&format!("WAVE DAT DUOC: {}", wave), cx - 120.0, cy - 45.0, 24.0, WHITE);
    draw_text(
        &format!("TONG DIEM: {}", score),
        cx - 120.0,
        cy - 10.0,
        24.0,
        Color::new(1.0, 0.84, 0.0, 1.0),
    );
    draw_text(
        &format!("QUAI DA HA: {}", total_kills),
        cx - 120.0,
        cy + 25.0,
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
        cy + 60.0,
        24.0,
        Color::new(0.0, 1.0, 0.4, 1.0),
    );

    let clicked = draw_tactical_button(
        cx - 170.0,
        cy + 100.0,
        340.0,
        52.0,
        "CHOI LAI TU DAU [ENTER / R]",
        Color::new(0.0, 1.0, 0.65, 1.0),
    );

    clicked || is_key_pressed(KeyCode::Enter) || is_key_pressed(KeyCode::R)
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

    for i in (1..pool.len()).rev() {
        let j = rand::gen_range(0, i + 1);
        pool.swap(i, j);
    }
    pool.truncate(3);
    pool
}
