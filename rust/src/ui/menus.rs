use macroquad::prelude::*;
use crate::core::i18n::{
    draw_text, draw_text_centered, draw_text_fitted, draw_wrapped_text, measure_text,
    set_current_language, tr,
};
use crate::core::types::{GameSettings, PlayerModifiers, UpgradeOption};
use crate::skill::trees::{tree_meta_localized, TREE_META};

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum UpgradeMenuAction {
    None,
    Pick(&'static str),
    Reroll,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum StartMenuAction {
    None,
    SelectClass(usize),
    SelectWeapon(usize),
    StartGame,
    OpenSettings,
    ToggleLanguage,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum PauseMenuAction {
    None,
    Resume,
    OpenSettings,
    Restart,
    MainMenu,
}

/// Vẽ nút bấm chiến thuật tương tác chuột với khả năng co giãn chữ thông minh
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
    draw_rectangle_lines(x, y, w, h, if hover { 2.2 } else { 1.3 }, border);

    // Điểm nhấn thanh dọc bên trái nút
    draw_rectangle(x, y, 4.0, h, accent);

    let max_tw = w - 16.0;
    let mut fs = (h * 0.42).clamp(11.0, 18.0);
    while fs > 9.5 {
        let dims = measure_text(label, None, (fs.round() as u16).max(8), 1.0);
        if dims.width <= max_tw {
            break;
        }
        fs -= 0.8;
    }
    let dims = measure_text(label, None, (fs.round() as u16).max(8), 1.0);
    draw_text(
        label,
        x + (w - dims.width) / 2.0,
        y + h / 2.0 + (fs * 0.35),
        fs,
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
    draw_rectangle(x, y, w, 48.0, Color::new(0.06, 0.09, 0.14, 0.92));
    draw_rectangle_lines(x, y, w, 48.0, 1.0, Color::new(0.0, 0.85, 1.0, 0.25));

    let right_controls_w = if quick_zero_label.is_some() { 365.0 } else { 285.0 };
    let max_text_w = (w - right_controls_w - 20.0).max(180.0);

    draw_text_fitted(title, x + 14.0, y + 19.0, max_text_w, 14.5, WHITE);
    draw_text_fitted(
        subtitle,
        x + 14.0,
        y + 37.0,
        max_text_w,
        11.0,
        Color::new(0.6, 0.72, 0.85, 1.0),
    );

    let (mx, my) = mouse_position();

    // Nút tắt nhanh (ví dụ: [TẮT 0%])
    if let Some(q_label) = quick_zero_label {
        let qx = x + w - 355.0;
        let qy = y + 10.0;
        let qw = 70.0;
        let qh = 28.0;
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
        draw_text_fitted(q_label, qx + 6.0, qy + 19.0, qw - 12.0, 11.5, WHITE);
        if q_hover && is_mouse_button_pressed(MouseButton::Left) {
            *value = min;
        }
    }

    // Nút [-]
    let minus_x = x + w - 275.0;
    let btn_y = y + 10.0;
    let btn_sz = 28.0;
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
    draw_text_centered("-", minus_x + btn_sz / 2.0, btn_y + 19.0, 18.0, WHITE);
    if hover_minus && is_mouse_button_pressed(MouseButton::Left) {
        *value = (*value - step).clamp(min, max);
    }

    // Thanh kéo (Slider Bar)
    let bar_x = minus_x + btn_sz + 8.0;
    let bar_w = 130.0;
    let bar_y = y + 17.0;
    let bar_h = 14.0;

    let hover_bar =
        mx >= bar_x - 6.0 && mx <= bar_x + bar_w + 6.0 && my >= y + 6.0 && my <= y + 42.0;
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
    draw_rectangle(knob_x - 4.0, bar_y - 3.0, 8.0, bar_h + 6.0, WHITE);

    // Nút [+]
    let plus_x = bar_x + bar_w + 8.0;
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
    draw_text_centered("+", plus_x + btn_sz / 2.0, btn_y + 19.0, 18.0, WHITE);
    if hover_plus && is_mouse_button_pressed(MouseButton::Left) {
        *value = (*value + step).clamp(min, max);
    }

    // Hiển thị phần trăm
    draw_text_fitted(
        &format!("{:.0}%", *value * 100.0),
        plus_x + btn_sz + 8.0,
        y + 30.0,
        55.0,
        15.0,
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
    draw_rectangle(x, y, w, 44.0, Color::new(0.06, 0.09, 0.14, 0.92));
    draw_rectangle_lines(x, y, w, 44.0, 1.0, Color::new(0.0, 0.85, 1.0, 0.25));

    let btn_w = 145.0;
    let max_text_w = (w - btn_w - 24.0).max(120.0);

    draw_text_fitted(title, x + 12.0, y + 17.0, max_text_w, 13.5, WHITE);
    draw_text_fitted(
        subtitle,
        x + 12.0,
        y + 33.0,
        max_text_w,
        10.5,
        Color::new(0.6, 0.72, 0.85, 1.0),
    );

    let btn_h = 28.0;
    let bx = x + w - btn_w - 10.0;
    let by = y + 8.0;

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
    draw_rectangle_lines(bx, by, btn_w, btn_h, if hover { 2.0 } else { 1.2 }, border);

    let label = if *value { on_text } else { off_text };
    let dims = measure_text(label, None, 12, 1.0);
    draw_text(
        label,
        bx + (btn_w - dims.width) / 2.0,
        by + 19.0,
        12.0,
        WHITE,
    );

    if hover && is_mouse_button_pressed(MouseButton::Left) {
        *value = !*value;
    }
}

/// Màn hình chính (Start Menu) với Chọn Nhân Vật (3 Classes), Chọn 1 Vũ Khí Duy Nhất (7 Weapons), Ngôn Ngữ và Cài đặt
pub fn draw_start_menu(
    selected_class_id: usize,
    selected_weapon_id: usize,
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

    // Khung chính mở rộng lên 1140x664 để bố trí thông thoáng, cân bằng trên độ phân giải 1280x720
    let panel_w = 1140.0;
    let panel_h = 664.0;
    let px = cx - panel_w / 2.0;
    let py = cy - panel_h / 2.0;

    draw_rectangle(px, py, panel_w, panel_h, Color::new(0.05, 0.07, 0.12, 0.95));
    draw_rectangle_lines(px, py, panel_w, panel_h, 2.0, Color::new(0.0, 0.9, 1.0, 0.7));

    // Tiêu đề game ở góc trên trái
    draw_text_fitted(
        "CYBER SURVIVOR: GRAND BATTLE",
        px + 28.0,
        py + 34.0,
        panel_w - 480.0,
        28.0,
        Color::new(0.0, 0.95, 1.0, 1.0),
    );

    // Kỷ lục điểm số căn gọn gàng bên cạnh nút ngôn ngữ
    draw_text_fitted(
        &format!("{}: {}", tr("KỶ LỤC", "RECORD"), settings.high_score),
        px + panel_w - 430.0,
        py + 32.0,
        200.0,
        15.5,
        Color::new(1.0, 0.85, 0.1, 1.0),
    );

    // Nút chuyển nhanh ngôn ngữ ở góc trên phải
    let lang_btn_w = 210.0;
    let lang_btn_x = px + panel_w - lang_btn_w - 24.0;
    let lang_label = format!(
        "{}: {} [L]",
        tr("NGÔN NGỮ", "LANG"),
        settings.language.short_code()
    );
    if draw_tactical_button(
        lang_btn_x,
        py + 14.0,
        lang_btn_w,
        32.0,
        &lang_label,
        Color::new(1.0, 0.85, 0.15, 1.0),
    ) {
        return StartMenuAction::ToggleLanguage;
    }

    // 1. CHỌN CHIẾN BINH (Lớp Nhân Vật)
    let class_sec_y = py + 56.0;
    draw_text_fitted(
        &format!(
            "{}: {} — {}",
            tr("1. CHỌN CHIẾN BINH", "1. SELECT CLASS"),
            class_name,
            class_desc
        ),
        px + 28.0,
        class_sec_y,
        panel_w - 56.0,
        13.5,
        Color::new(0.75, 0.9, 1.0, 1.0),
    );

    let classes = [
        (
            0usize,
            "[1] CYBER MARINE",
            tr(
                "HP 110 | Giáp 60 | Không Kích & Lính Thủy Quân",
                "HP 110 | Shield 60 | Orbital Strike & Marines",
            ),
            Color::new(0.0, 0.95, 1.0, 1.0),
        ),
        (
            1usize,
            "[2] VOID STALKER",
            tr(
                "+25% DMG | Dịch Chuyển Tạo Hố Đen & Bão Sét",
                "+25% DMG | Phase Blink Black Hole & Storm",
            ),
            Color::new(0.8, 0.35, 1.0, 1.0),
        ),
        (
            2usize,
            "[3] TECH ENGINEER",
            tr(
                "HP 125 | Giáp 100 | Gọi 2 Đại Robot Titan Mech",
                "HP 125 | Shield 100 | Deploys 2 Titan Mechs",
            ),
            Color::new(1.0, 0.8, 0.15, 1.0),
        ),
    ];

    let class_gap = 14.0;
    let class_card_w = (panel_w - 56.0 - 2.0 * class_gap) / 3.0;
    let class_card_y = py + 66.0;
    let class_card_h = 58.0;
    let (mx, my) = mouse_position();

    for (idx, title, sub, col) in classes {
        let card_x = px + 28.0 + (idx as f32) * (class_card_w + class_gap);
        let is_sel = selected_class_id == idx;
        let hover = mx >= card_x && mx <= card_x + class_card_w && my >= class_card_y && my <= class_card_y + class_card_h;

        draw_rectangle(
            card_x,
            class_card_y,
            class_card_w,
            class_card_h,
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
            class_card_y,
            class_card_w,
            class_card_h,
            if is_sel { 2.5 } else { 1.0 },
            if is_sel { col } else { Color::new(col.r, col.g, col.b, 0.4) },
        );
        draw_text_fitted(title, card_x + 12.0, class_card_y + 22.0, class_card_w - 24.0, 15.0, col);
        draw_text_fitted(sub, card_x + 12.0, class_card_y + 43.0, class_card_w - 24.0, 11.5, WHITE);

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

    // 2. CHỌN 1 VŨ KHÍ DUY NHẤT CHO TRẬN ĐẤU (7 Weapons)
    let w_sec_y = py + 142.0;
    draw_text_fitted(
        tr(
            "2. CHỌN 1 VŨ KHÍ DUY NHẤT CHO TRẬN ĐẤU (Nhặt Lõi Nâng Cấp đến Lv.100 để Tiến Hóa [EVO]):",
            "2. SELECT 1 WEAPON FOR THE MATCH (Collect Upgrade Cores to Lv.100 to Evolve [EVO]):",
        ),
        px + 28.0,
        w_sec_y,
        panel_w - 56.0,
        13.5,
        Color::new(1.0, 0.85, 0.15, 1.0),
    );

    let weapons = [
        (
            0usize,
            "PISTOL",
            tr("Cân bằng / Cơ động", "Balanced / Agile"),
            Color::new(0.0, 0.94, 1.0, 1.0),
        ),
        (
            1usize,
            "SHOTGUN",
            tr("Chùm 8 viên cận chiến", "8-Pellet Buckshot"),
            Color::new(1.0, 0.66, 0.0, 1.0),
        ),
        (
            2usize,
            "MINIGUN",
            tr("Xả đạn cực nhanh", "Extreme Fire Rate"),
            Color::new(1.0, 0.88, 0.15, 1.0),
        ),
        (
            3usize,
            "TESLA GUN",
            tr("Phóng sét dây chuyền", "Chain Lightning"),
            Color::new(0.25, 0.9, 1.0, 1.0),
        ),
        (
            4usize,
            "RAILGUN",
            tr("Tia xuyên toàn bản đồ", "Piercing Rail Beam"),
            Color::new(0.0, 1.0, 0.85, 1.0),
        ),
        (
            5usize,
            "SINGULARITY",
            tr("Bắn Hố Đen hút quái", "Gravity Black Hole"),
            Color::new(0.75, 0.2, 1.0, 1.0),
        ),
        (
            6usize,
            "CLUSTER RPG",
            tr("Tên lửa nổ diện rộng", "Heavy AoE Rockets"),
            Color::new(1.0, 0.25, 0.3, 1.0),
        ),
    ];

    let w_gap = 8.0;
    let w_card_w = (panel_w - 56.0 - 6.0 * w_gap) / 7.0;
    let w_card_y = py + 152.0;
    let w_card_h = 56.0;

    for (w_id, w_name, w_sub, w_col) in weapons {
        let wx = px + 28.0 + (w_id as f32) * (w_card_w + w_gap);
        let is_w_sel = selected_weapon_id == w_id;
        let w_hover = mx >= wx && mx <= wx + w_card_w && my >= w_card_y && my <= w_card_y + w_card_h;

        draw_rectangle(
            wx,
            w_card_y,
            w_card_w,
            w_card_h,
            if is_w_sel {
                Color::new(w_col.r * 0.28, w_col.g * 0.28, w_col.b * 0.28, 0.96)
            } else if w_hover {
                Color::new(0.12, 0.18, 0.28, 0.95)
            } else {
                Color::new(0.04, 0.06, 0.1, 0.9)
            },
        );
        draw_rectangle_lines(
            wx,
            w_card_y,
            w_card_w,
            w_card_h,
            if is_w_sel { 2.5 } else { 1.0 },
            if is_w_sel {
                w_col
            } else {
                Color::new(w_col.r, w_col.g, w_col.b, 0.4)
            },
        );
        draw_text_fitted(w_name, wx + 6.0, w_card_y + 21.0, w_card_w - 12.0, 13.5, w_col);
        draw_text_fitted(w_sub, wx + 6.0, w_card_y + 42.0, w_card_w - 12.0, 10.5, WHITE);

        if w_hover && is_mouse_button_pressed(MouseButton::Left) {
            return StartMenuAction::SelectWeapon(w_id);
        }
    }

    // KHU VỰC DƯỚI: Chia 2 cột sạch đẹp
    let lower_y = py + 224.0;
    let left_x = px + 28.0;
    let left_w = 460.0;
    let right_x = left_x + left_w + 18.0;
    let right_w = panel_w - 56.0 - left_w - 18.0;
    let lower_h = panel_h - 248.0;

    // Cột trái: 2 Nút lớn + Bảng tóm tắt cơ chế EVO
    let btn_w = left_w;
    if draw_tactical_button(
        left_x,
        lower_y,
        btn_w,
        48.0,
        tr("BẮT ĐẦU CHIẾN ĐẤU [ENTER]", "START BATTLE [ENTER]"),
        Color::new(0.0, 1.0, 0.65, 1.0),
    ) || is_key_pressed(KeyCode::Enter)
    {
        return StartMenuAction::StartGame;
    }

    if draw_tactical_button(
        left_x,
        lower_y + 56.0,
        btn_w,
        42.0,
        tr("TUỲ CHỈNH CÀI ĐẶT [PHÍM O]", "SETTINGS & LANGUAGE [KEY O]"),
        Color::new(0.0, 0.88, 1.0, 1.0),
    ) || is_key_pressed(KeyCode::O)
    {
        return StartMenuAction::OpenSettings;
    }

    // Khung tóm tắt cơ chế EVO & Nâng cấp
    let info_box_y = lower_y + 108.0;
    let info_box_h = lower_h - 108.0;
    draw_rectangle(
        left_x,
        info_box_y,
        btn_w,
        info_box_h,
        Color::new(0.04, 0.06, 0.1, 0.9),
    );
    draw_rectangle_lines(
        left_x,
        info_box_y,
        btn_w,
        info_box_h,
        1.0,
        Color::new(0.0, 0.8, 1.0, 0.3),
    );

    draw_text_fitted(
        tr("CƠ CHẾ TIẾN HÓA SÚNG 100 CẤP:", "100-LEVEL WEAPON EVOLUTION:"),
        left_x + 14.0,
        info_box_y + 24.0,
        btn_w - 28.0,
        14.5,
        Color::new(1.0, 0.84, 0.0, 1.0),
    );

    let evo_notes = [
        (
            tr(
                "• RƠI LÕI NÂNG CẤP SÚNG: Quái & Boss rơi Lõi (+1..+8 Lv) thay vì rơi súng!",
                "• WEAPON UPGRADE CORES: Enemies & Bosses drop Cores (+1..+8 Lv) instead of guns!",
            ),
            Color::new(0.0, 1.0, 0.65, 1.0),
        ),
        (
            tr(
                "• TIẾN TRÌNH: Lv.1-24 Thường -> Lv.25-49 Hiếm -> Lv.50-74 Sử Thi -> Lv.75-99 Huyền Thoại.",
                "• PROGRESSION: Lv.1-24 Common -> Lv.25-49 Rare -> Lv.50-74 Epic -> Lv.75-99 Legendary.",
            ),
            WHITE,
        ),
        (
            tr(
                "• TIẾN HÓA [EVO]: Đạt Lv.100 mở khóa Vũ Khí Thức Tỉnh sức mạnh tối thượng!",
                "• EVOLUTION [EVO]: Reach Lv.100 to awaken ultimate weapon form!",
            ),
            Color::new(1.0, 0.5, 0.2, 1.0),
        ),
        (
            tr(
                "• HÚT TINH THỂ EXP: Nhặt kinh nghiệm rơi; nâng Hệ Từ Trường để hút xa!",
                "• EXP CRYSTALS: Collect EXP orbs; upgrade Magnetism tree to pull from afar!",
            ),
            Color::new(0.0, 0.95, 1.0, 1.0),
        ),
        (
            tr(
                "• PHÍM TẮT TRONG TRẬN: Bấm [ESC] hoặc [O] mở Cài Đặt; [L] đổi Ngôn Ngữ.",
                "• IN-GAME HOTKEYS: Press [ESC] or [O] for Settings; [L] to switch Language.",
            ),
            Color::new(0.85, 0.75, 1.0, 1.0),
        ),
    ];

    let mut ny = info_box_y + 48.0;
    for (note, col) in evo_notes {
        ny = draw_wrapped_text(note, left_x + 14.0, ny, btn_w - 28.0, 16.0, 11.5, col);
        ny += 4.0;
    }

    // Cột phải: Bảng Hướng Dẫn Chiến Trường & Phím Tắt
    draw_rectangle(
        right_x,
        lower_y,
        right_w,
        lower_h,
        Color::new(0.04, 0.06, 0.1, 0.9),
    );
    draw_rectangle_lines(
        right_x,
        lower_y,
        right_w,
        lower_h,
        1.0,
        Color::new(0.0, 0.8, 1.0, 0.3),
    );

    draw_text_fitted(
        tr(
            "CẨM NANG TÁC CHIẾN CHI TIẾT:",
            "TACTICAL FIELD MANUAL:",
        ),
        right_x + 16.0,
        lower_y + 24.0,
        right_w - 32.0,
        16.0,
        Color::new(0.0, 0.95, 1.0, 1.0),
    );

    let guides = [
        (
            tr("[CHUỘT / 1..3]", "[MOUSE / 1..3]"),
            tr(
                "Chọn Chiến Binh & 1 Vũ Khí Duy Nhất trước trận",
                "Select Class & 1 Match Weapon before deployment",
            ),
        ),
        (
            "[W][A][S][D]",
            tr(
                "Di chuyển & Thu thập Tinh Thể Kinh Nghiệm (EXP)",
                "Move & Collect dropped EXP Crystals",
            ),
        ),
        (
            tr("[CHUỘT TRÁI / PHẢI]", "[LMB / RMB]"),
            tr(
                "Bắn vũ khí / Chém Đao Năng Lượng đẩy lùi",
                "Fire Weapon / Plasma Melee Slash knockback",
            ),
        ),
        (
            "[TAB] / [V]",
            tr(
                "Chuyển Chế Độ Ngắm Thông Minh / Tự Động Bắn",
                "Cycle Smart Aim Mode / Toggle Auto-Fire",
            ),
        ),
        (
            "[SPACE] / [SHIFT]",
            tr(
                "Lướt Dash / Dịch Chuyển / Làm Chậm Thời Gian",
                "Dash / Phase Blink / Bullet-Time Slow-Mo",
            ),
        ),
        (
            "[R]",
            tr(
                "Nạp Đạn Nhanh (Canh vạch xanh trên đầu +40% DMG)",
                "Active Reload (Hit overhead green zone +40% DMG)",
            ),
        ),
        (
            "[F]",
            tr(
                "Nhặt Lõi Nâng Cấp Súng (+1..+8 Lv -> Lv.100 [EVO])",
                "Collect Weapon Upgrade Cores (+1..+8 Lv -> Lv.100 [EVO])",
            ),
        ),
        (
            "[Q] / [E]",
            tr(
                "Kỹ năng Hủy Diệt & Gọi Binh Đoàn Tiếp Viện",
                "Ultimate Ability & Call Allied Reinforcements",
            ),
        ),
        (
            tr("[EVO CẤP 100]", "[LV.100 EVO]"),
            tr(
                "Đạt Lv.100 mở khóa Vũ Khí Thức Tỉnh Tối Thượng",
                "Reach Lv.100 to awaken Awakened Ultimate Form",
            ),
        ),
        (
            "[L] / [ESC] / [O]",
            tr(
                "Đổi Ngôn Ngữ [L] / Tạm dừng & Mở Cài Đặt [ESC/O]",
                "Switch Language [L] / Pause & Open Settings [ESC/O]",
            ),
        ),
    ];

    let row_spacing = ((lower_h - 48.0) / guides.len() as f32).min(32.0);
    for (i, (key, desc)) in guides.iter().enumerate() {
        let gy = lower_y + 48.0 + (i as f32) * row_spacing;
        draw_text_fitted(key, right_x + 14.0, gy, 145.0, 12.5, Color::new(1.0, 0.85, 0.1, 1.0));
        draw_text_fitted(desc, right_x + 165.0, gy, right_w - 180.0, 12.0, WHITE);
    }

    StartMenuAction::None
}

/// Menu Tạm Dừng (Pause Menu)
pub fn draw_pause_menu() -> PauseMenuAction {
    draw_rectangle(
        0.0,
        0.0,
        screen_width(),
        screen_height(),
        Color::new(0.02, 0.03, 0.06, 0.72),
    );

    let cx = screen_width() / 2.0;
    let cy = screen_height() / 2.0;
    let pw = 480.0;
    let ph = 360.0;
    let px = cx - pw / 2.0;
    let py = cy - ph / 2.0;

    draw_rectangle(px, py, pw, ph, Color::new(0.05, 0.08, 0.14, 0.96));
    draw_rectangle_lines(px, py, pw, ph, 2.0, Color::new(0.0, 0.94, 1.0, 0.9));

    draw_text_centered(
        tr("TẠM DỪNG CHIẾN TRƯỜNG", "BATTLEFIELD PAUSED"),
        cx,
        py + 44.0,
        24.0,
        Color::new(0.0, 0.94, 1.0, 1.0),
    );

    let bw = pw - 64.0;
    let bx = px + 32.0;

    if draw_tactical_button(
        bx,
        py + 74.0,
        bw,
        46.0,
        tr("TIẾP TỤC TRẬN ĐẤU [ESC / P]", "RESUME BATTLE [ESC / P]"),
        Color::new(0.0, 1.0, 0.6, 1.0),
    ) {
        return PauseMenuAction::Resume;
    }

    if draw_tactical_button(
        bx,
        py + 134.0,
        bw,
        46.0,
        tr("TUỲ CHỈNH CÀI ĐẶT & NGÔN NGỮ [O]", "SETTINGS & LANGUAGE [KEY O]"),
        Color::new(0.0, 0.88, 1.0, 1.0),
    ) || is_key_pressed(KeyCode::O)
    {
        return PauseMenuAction::OpenSettings;
    }

    if draw_tactical_button(
        bx,
        py + 194.0,
        bw,
        46.0,
        tr("CHƠI LẠI TỪ ĐẦU [RESTART]", "RESTART MATCH [RESTART]"),
        Color::new(1.0, 0.8, 0.1, 1.0),
    ) {
        return PauseMenuAction::Restart;
    }

    if draw_tactical_button(
        bx,
        py + 254.0,
        bw,
        46.0,
        tr("VỀ MÀN HÌNH CHÍNH [MAIN MENU]", "RETURN TO MAIN MENU"),
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
    let mw = 940.0;
    let mh = 680.0;
    let mx = cx - mw / 2.0;
    let my = cy - mh / 2.0;

    draw_rectangle(mx, my, mw, mh, Color::new(0.04, 0.06, 0.11, 0.98));
    draw_rectangle_lines(mx, my, mw, mh, 2.0, Color::new(0.0, 0.94, 1.0, 0.9));

    let row_x = mx + 24.0;
    let row_w = mw - 48.0;

    draw_text_fitted(
        tr(
            "TUỲ CHỈNH CÀI ĐẶT NGÔN NGỮ, ÂM THANH & CHIẾN TRƯỜNG",
            "LANGUAGE, AUDIO, VISUAL & BATTLEFIELD SETTINGS",
        ),
        row_x,
        my + 28.0,
        row_w,
        20.0,
        Color::new(0.0, 0.95, 1.0, 1.0),
    );
    draw_text_fitted(
        tr(
            "Kéo thanh trượt hoặc bấm [-] / [+] (Tự động lưu cấu hình khi đóng):",
            "Drag sliders or click [-] / [+] (Automatically saved on close):",
        ),
        row_x,
        my + 46.0,
        row_w,
        12.0,
        Color::new(0.7, 0.82, 0.95, 1.0),
    );

    let mut cur_y = my + 56.0;

    // 0. Ngôn ngữ hiển thị (Language Toggle: Tiếng Việt có dấu / English)
    let mut lang_clicked = false;
    draw_toggle_row(
        row_x,
        cur_y,
        row_w,
        tr(
            "0. NGÔN NGỮ HIỂN THỊ (LANGUAGE) [PHÍM L]",
            "0. DISPLAY LANGUAGE [KEY L]",
        ),
        tr(
            "Chuyển đổi giữa Tiếng Việt có dấu đầy đủ và Tiếng Anh (English)",
            "Switch between full accented Vietnamese and English",
        ),
        &mut lang_clicked,
        settings.language.label(),
        settings.language.label(),
    );
    if lang_clicked {
        settings.language = settings.language.toggle();
        set_current_language(settings.language);
        settings.save_to_disk();
    }
    cur_y += 50.0;

    let off_zero = tr("TẮT 0%", "OFF 0%");

    // 1. Âm lượng tổng (Master Volume)
    draw_slider_row(
        row_x,
        cur_y,
        row_w,
        tr(
            "1. ÂM LƯỢNG TỔNG HỢP (MASTER SYNTH VOLUME)",
            "1. MASTER SYNTH AUDIO VOLUME",
        ),
        tr(
            "Điều chỉnh âm lượng tiếng súng, tia sét Tesla, bom nổ & nâng cấp",
            "Adjust volume of gunfire, Tesla arcs, explosions & level-ups",
        ),
        &mut settings.master_volume,
        0.0,
        1.0,
        0.10,
        Some(off_zero),
    );
    cur_y += 50.0;

    // 2. Độ rung màn hình (Screen Shake)
    draw_slider_row(
        row_x,
        cur_y,
        row_w,
        tr(
            "2. ĐỘ RUNG MÀN HÌNH (SCREEN SHAKE)",
            "2. CAMERA SCREEN SHAKE INTENSITY",
        ),
        tr(
            "Giảm hoặc tắt hẳn rung chấn camera khi nổ bom, bắn Railgun, Laser",
            "Reduce or disable camera shake from explosions, Railgun & Orbital strikes",
        ),
        &mut settings.screen_shake_intensity,
        0.0,
        1.0,
        0.05,
        Some(off_zero),
    );
    cur_y += 50.0;

    // 3. Mật độ hiệu ứng nổ (Particle Density)
    draw_slider_row(
        row_x,
        cur_y,
        row_w,
        tr(
            "3. MẬT ĐỘ HIỆU ỨNG HẠT CHÁY NỔ (VFX DENSITY)",
            "3. PARTICLE & EXPLOSION VFX DENSITY",
        ),
        tr(
            "Giảm bớt tia lửa để nhìn rõ đường đạn và quái vật trong đại chiến",
            "Lower spark density for clearer bullet & enemy visibility in large battles",
        ),
        &mut settings.particle_density,
        0.1,
        1.0,
        0.10,
        None,
    );
    cur_y += 50.0;

    // 4. Độ sáng lưới nền (Arena Grid)
    draw_slider_row(
        row_x,
        cur_y,
        row_w,
        tr(
            "4. ĐỘ SÁNG LƯỚI CHIẾN TRƯỜNG (GRID OPACITY)",
            "4. TACTICAL ARENA GRID OPACITY",
        ),
        tr(
            "Điều chỉnh độ rõ của lưới tọa độ dưới mặt đất",
            "Adjust brightness of the tactical coordinate grid on the ground",
        ),
        &mut settings.grid_opacity,
        0.0,
        1.0,
        0.10,
        Some(off_zero),
    );
    cur_y += 50.0;

    // 5. Quy mô đàn quái (Swarm Scale)
    draw_slider_row(
        row_x,
        cur_y,
        row_w,
        tr(
            "5. QUY MÔ ĐÀN QUÁI (BATTLE SWARM SCALE)",
            "5. ENEMY SWARM DENSITY SCALE",
        ),
        tr(
            "Tăng/giảm số lượng quái vật xuất hiện mỗi đợt (50% Nhẹ -> 200% Đại Chiến)",
            "Scale enemy wave size (50% Light Skirmish -> 200% Grand Swarm)",
        ),
        &mut settings.swarm_scale,
        0.5,
        2.0,
        0.25,
        None,
    );
    cur_y += 52.0;

    // Hai cột cho 6 nút Toggle bên dưới
    let half_w = (row_w - 14.0) / 2.0;

    // Nút chọn chế độ Hỗ Trợ Nhắm Bắn
    let mut smart_aim_active = settings.aim_assist_mode > 0;
    let aim_btn_label = settings.aim_mode_label();
    let prev_smart = smart_aim_active;
    draw_toggle_row(
        row_x,
        cur_y,
        half_w,
        tr("HỖ TRỢ NHẮM BẮN [TAB]", "SMART AIM ASSIST [TAB]"),
        tr(
            "Khóa mục tiêu & đạn tự bẻ lái vào quái",
            "Target lock-on & magnetic bullet homing",
        ),
        &mut smart_aim_active,
        aim_btn_label,
        aim_btn_label,
    );
    if smart_aim_active != prev_smart {
        settings.aim_assist_mode = (settings.aim_assist_mode + 1) % 3;
    }

    draw_toggle_row(
        row_x + half_w + 14.0,
        cur_y,
        half_w,
        tr("TỰ ĐỘNG BÓP CÒ [PHÍM V]", "AUTO-FIRE TRIGGER [KEY V]"),
        tr(
            "Tự động xả đạn khi đã khóa mục tiêu",
            "Automatically fire weapon when target is in sight",
        ),
        &mut settings.auto_fire,
        tr("ĐANG BẬT [V]", "ENABLED [V]"),
        tr("ĐANG TẮT [V]", "DISABLED [V]"),
    );
    cur_y += 46.0;

    draw_toggle_row(
        row_x,
        cur_y,
        half_w,
        tr("HIỆN SỐ SÁT THƯƠNG", "FLOATING DAMAGE NUMBERS"),
        tr(
            "Số sát thương bay lên khi bắn trúng",
            "Show floating damage numbers on enemy hit",
        ),
        &mut settings.show_damage_numbers,
        tr("ĐANG BẬT", "ENABLED"),
        tr("ĐANG TẮT", "DISABLED"),
    );
    draw_toggle_row(
        row_x + half_w + 14.0,
        cur_y,
        half_w,
        tr("TỰ ĐỘNG NHẶT LÕI SÚNG", "AUTO-PICKUP UPGRADE CORES"),
        tr(
            "Tắt để chỉ nhặt Lõi Nâng Cấp khi bấm [F]",
            "Disable to require pressing [F] on Upgrade Cores",
        ),
        &mut settings.auto_pickup_weapons,
        tr("TỰ ĐỘNG + [F]", "AUTO + [F]"),
        tr("CHỈ BẤM [F]", "PRESS [F] ONLY"),
    );
    cur_y += 46.0;

    draw_toggle_row(
        row_x,
        cur_y,
        half_w,
        tr("SPAWN QUÁI 1 LẦN DUY NHẤT", "INSTANT FULL SWARM SPAWN"),
        tr(
            "Toàn bộ quái xuất hiện cùng 1 lúc ngay đầu đợt",
            "Spawn entire monster horde all at once at wave start",
        ),
        &mut settings.instant_full_spawn,
        tr("TOÀN BỘ 1 LẦN", "INSTANT ALL"),
        tr("SINH TỪ TỪ", "SPAWN OVER TIME"),
    );

    let ram_str = format!("{} CON", settings.max_ram_spawn_limit);
    let mut ram_clicked = false;
    draw_toggle_row(
        row_x + half_w + 14.0,
        cur_y,
        half_w,
        tr("MỨC RAM / GIỚI HẠN QUÁI", "RAM LIMIT / SWARM CAP"),
        tr(
            "Bấm để đổi: 2000 -> 5000 -> 10000 -> 15000 -> 20000",
            "Click to cycle: 2K -> 5K -> 10K (Def) -> 15K -> 20K",
        ),
        &mut ram_clicked,
        &ram_str,
        &ram_str,
    );
    if ram_clicked {
        settings.max_ram_spawn_limit = match settings.max_ram_spawn_limit {
            2000 => 5000,
            5000 => 10000,
            10000 => 15000,
            15000 => 20000,
            _ => 2000,
        };
        settings.save_to_disk();
    }
    cur_y += 46.0;

    draw_toggle_row(
        row_x,
        cur_y,
        half_w,
        tr("BẢN ĐỒ RADAR MINIMAP", "TACTICAL RADAR MINIMAP"),
        tr(
            "Hiện Radar vị trí Boss, Lõi Súng & Trạm",
            "Show minimap with Boss, Upgrade Cores & Structures",
        ),
        &mut settings.show_minimap,
        tr("ĐANG BẬT", "ENABLED"),
        tr("ĐANG TẮT", "DISABLED"),
    );
    draw_toggle_row(
        row_x + half_w + 14.0,
        cur_y,
        half_w,
        tr("THANH MÁU ĐỒNG MINH", "ALLY HEALTH BARS"),
        tr(
            "Hiển thị tên và máu lính hỗ trợ / Turret",
            "Show health bars on allied bots & turrets",
        ),
        &mut settings.show_ally_bars,
        tr("ĐANG BẬT", "ENABLED"),
        tr("ĐANG TẮT", "DISABLED"),
    );
    cur_y += 50.0;

    // Hàng nút dưới cùng: Khôi phục mặc định & Lưu/Đóng
    if draw_tactical_button(
        row_x,
        cur_y,
        280.0,
        40.0,
        tr("KHÔI PHỤC MẶC ĐỊNH [RESET]", "RESET TO DEFAULTS"),
        Color::new(1.0, 0.75, 0.1, 1.0),
    ) {
        let hs = settings.high_score;
        let lang = settings.language;
        *settings = GameSettings::default();
        settings.high_score = hs;
        settings.language = lang;
        settings.save_to_disk();
    }

    let close_clicked = draw_tactical_button(
        row_x + row_w - 320.0,
        cur_y,
        320.0,
        40.0,
        tr("LƯU & QUAY LẠI [ESC]", "SAVE & RETURN [ESC]"),
        Color::new(0.0, 1.0, 0.65, 1.0),
    );

    if close_clicked || is_key_pressed(KeyCode::Escape) {
        settings.save_to_disk();
        true
    } else {
        false
    }
}

pub fn draw_upgrade_menu(
    choices: &[UpgradeOption],
    mods: &PlayerModifiers,
    rerolls_left: u32,
    pending_picks: u32,
) -> UpgradeMenuAction {
    clear_background(Color::new(0.04, 0.06, 0.10, 1.0));
    let cx = screen_width() / 2.0;
    let cy = screen_height() / 2.0;

    // Tiêu đề chính căn giữa
    draw_text_centered(
        tr(
            "HỆ THỐNG THĂNG CẤP CÔNG NGHỆ & BỘ TỐI THƯỢNG",
            "TECH TREE UPGRADES & ULTIMATE SYNERGY SETS",
        ),
        cx,
        cy - 275.0,
        25.0,
        Color::new(0.0, 0.95, 1.0, 1.0),
    );
    draw_text_centered(
        &format!(
            "{}: {} | {}",
            tr("Lượt chọn còn lại", "Picks remaining"),
            pending_picks,
            tr(
                "Đạt Cấp 3 của 2-3 Hệ để mở khóa 12 BỘ KỸ NĂNG TỐI THƯỢNG!",
                "Reach Tier 3 in 2-3 Trees to unlock 12 ULTIMATE SYNERGY SETS!",
            )
        ),
        cx,
        cy - 248.0,
        14.0,
        Color::new(1.0, 0.85, 0.2, 1.0),
    );

    // 1. Thanh Tiến Độ 8 Hệ Nâng Cấp (Skill Tree Mastery Tracker)
    let tree_gap = 8.0;
    let tree_w = ((1180.0 - ((TREE_META.len() - 1) as f32) * tree_gap) / (TREE_META.len() as f32)).min(158.0);
    let total_tree_w = (TREE_META.len() as f32) * tree_w + ((TREE_META.len() - 1) as f32) * tree_gap;
    let tree_start_x = cx - total_tree_w / 2.0;
    let tree_y = cy - 228.0;

    for tid in 0..TREE_META.len() {
        let (t_name, t_sub, t_col) = tree_meta_localized(tid);
        let tx = tree_start_x + (tid as f32) * (tree_w + tree_gap);
        let tier = mods.tree_tier(tid);
        let is_mastered = tier >= 3;

        draw_rectangle(
            tx,
            tree_y,
            tree_w,
            58.0,
            if is_mastered {
                Color::new(t_col.r * 0.28, t_col.g * 0.28, t_col.b * 0.28, 0.95)
            } else {
                Color::new(0.06, 0.09, 0.15, 0.92)
            },
        );
        draw_rectangle_lines(
            tx,
            tree_y,
            tree_w,
            58.0,
            if is_mastered { 2.2 } else { 1.1 },
            if is_mastered { t_col } else { Color::new(t_col.r, t_col.g, t_col.b, 0.45) },
        );

        draw_text_fitted(t_name, tx + 6.0, tree_y + 17.0, tree_w - 12.0, 12.0, t_col);
        draw_text_fitted(
            &format!("{} [{}/3]", t_sub, tier),
            tx + 6.0,
            tree_y + 32.0,
            tree_w - 12.0,
            10.5,
            WHITE,
        );

        // 3 ô tiến độ Cấp 1 -> Cấp 2 -> Cấp 3
        let step_w = (tree_w - 12.0 - 2.0 * 6.0) / 3.0;
        for step in 1..=3 {
            let bx = tx + 6.0 + ((step - 1) as f32) * (step_w + 6.0);
            let by = tree_y + 40.0;
            let active = tier >= step;
            draw_rectangle(
                bx,
                by,
                step_w,
                8.0,
                if active {
                    t_col
                } else {
                    Color::new(0.15, 0.2, 0.28, 1.0)
                },
            );
        }
    }

    // 2. Vẽ 4 Thẻ Lựa Chọn Nâng Cấp (4 Upgrade Cards)
    let count = choices.len().max(1);
    let card_w = 278.0;
    let card_h = 300.0;
    let gap = 16.0;
    let total_cards_w = (count as f32) * card_w + ((count - 1) as f32) * gap;
    let start_x = cx - total_cards_w / 2.0;
    let ry = cy - 146.0;

    for (i, opt) in choices.iter().enumerate() {
        let rx = start_x + (i as f32) * (card_w + gap);
        let (mx, my) = mouse_position();
        let hover = mx >= rx && mx <= rx + card_w && my >= ry && my <= ry + card_h;

        let is_ultimate = opt.tier == 4;
        let is_mastery = opt.tier == 3;

        let accent = if is_ultimate {
            Color::new(1.0, 0.25, 0.65, 1.0)
        } else if is_mastery {
            Color::new(1.0, 0.84, 0.0, 1.0)
        } else if opt.tree_id < TREE_META.len() {
            TREE_META[opt.tree_id].2
        } else {
            Color::new(0.0, 0.94, 1.0, 1.0)
        };

        let bg_col = if is_ultimate {
            if hover {
                Color::new(0.32, 0.08, 0.24, 0.98)
            } else {
                Color::new(0.20, 0.04, 0.16, 0.96)
            }
        } else if hover {
            Color::new(0.08, 0.22, 0.34, 0.96)
        } else {
            Color::new(0.06, 0.09, 0.15, 0.95)
        };

        draw_rectangle(rx, ry, card_w, card_h, bg_col);
        draw_rectangle_lines(
            rx,
            ry,
            card_w,
            card_h,
            if is_ultimate || hover { 2.8 } else { 1.5 },
            if hover { WHITE } else { accent },
        );

        // Dải băng tiêu đề phân cấp Hệ / Bộ Tối Thượng
        draw_rectangle(
            rx,
            ry,
            card_w,
            30.0,
            Color::new(accent.r * 0.3, accent.g * 0.3, accent.b * 0.3, 0.95),
        );
        draw_text_fitted(
            opt.category_tag,
            rx + 8.0,
            ry + 20.0,
            card_w - 76.0,
            12.0,
            if is_ultimate { YELLOW } else { accent },
        );

        // Phím tắt [1]..[4]
        draw_text(
            &format!("{} [{}]", tr("PHÍM", "KEY"), i + 1),
            rx + card_w - 64.0,
            ry + 20.0,
            12.0,
            Color::new(0.8, 0.9, 1.0, 1.0),
        );

        // Icon nâng cấp
        draw_text(opt.icon, rx + 14.0, ry + 62.0, 26.0, WHITE);

        // Tên nâng cấp (co dãn tự động không bao giờ tràn)
        draw_text_fitted(
            opt.title,
            rx + 14.0,
            ry + 96.0,
            card_w - 28.0,
            15.5,
            if is_ultimate {
                Color::new(1.0, 0.9, 0.2, 1.0)
            } else {
                accent
            },
        );

        // Đường kẻ phân cách
        draw_line(
            rx + 14.0,
            ry + 106.0,
            rx + card_w - 14.0,
            ry + 106.0,
            1.0,
            Color::new(accent.r, accent.g, accent.b, 0.3),
        );

        // Mô tả chi tiết — TỰ ĐỘNG XUỐNG DÒNG (WRAPPED TEXT) ĐỂ KHÔNG BAO GIỜ BỊ ĐÈ CHỮ!
        draw_wrapped_text(
            opt.desc,
            rx + 14.0,
            ry + 126.0,
            card_w - 28.0,
            18.0,
            12.0,
            WHITE,
        );

        // Điều kiện / gợi ý ghép Bộ Tối Thượng ở chân thẻ
        draw_line(
            rx + 12.0,
            ry + card_h - 48.0,
            rx + card_w - 12.0,
            ry + card_h - 48.0,
            1.0,
            Color::new(accent.r, accent.g, accent.b, 0.35),
        );
        draw_wrapped_text(
            opt.req_desc,
            rx + 14.0,
            ry + card_h - 28.0,
            card_w - 28.0,
            14.0,
            11.0,
            Color::new(0.85, 0.9, 0.6, 1.0),
        );

        let key_pick = match i {
            0 => is_key_pressed(KeyCode::Key1),
            1 => is_key_pressed(KeyCode::Key2),
            2 => is_key_pressed(KeyCode::Key3),
            3 => is_key_pressed(KeyCode::Key4),
            _ => false,
        };

        if (hover && is_mouse_button_pressed(MouseButton::Left)) || key_pick {
            return UpgradeMenuAction::Pick(opt.id);
        }
    }

    // 3. Nút Đổi Bộ Thẻ Khác (Reroll [R])
    let reroll_y = ry + card_h + 18.0;
    if rerolls_left > 0 {
        let reroll_label = format!(
            "{} ({}: {}) [R]",
            tr("ĐỔI BỘ THẺ KHÁC", "REROLL UPGRADE CARDS"),
            tr("CÒN", "LEFT"),
            rerolls_left
        );
        if draw_tactical_button(
            cx - 200.0,
            reroll_y,
            400.0,
            46.0,
            &reroll_label,
            Color::new(1.0, 0.82, 0.1, 1.0),
        ) || is_key_pressed(KeyCode::R)
        {
            return UpgradeMenuAction::Reroll;
        }
    } else {
        draw_text_centered(
            tr(
                "Đã dùng hết lượt Reroll trong Đợt này (Hồi lại mỗi Wave)",
                "No Rerolls remaining this Wave (Refreshes next Wave)",
            ),
            cx,
            reroll_y + 24.0,
            14.5,
            GRAY,
        );
    }

    UpgradeMenuAction::None
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

    let panel_w = 560.0;
    let panel_h = 420.0;
    let px = cx - panel_w / 2.0;
    let py = cy - panel_h / 2.0;

    draw_rectangle(px, py, panel_w, panel_h, Color::new(0.08, 0.03, 0.06, 0.96));
    draw_rectangle_lines(px, py, panel_w, panel_h, 2.0, Color::new(1.0, 0.2, 0.3, 0.8));

    draw_text_centered(
        tr("CHIẾN DỊCH THẤT BẠI", "MISSION FAILED"),
        cx,
        py + 54.0,
        38.0,
        Color::new(1.0, 0.15, 0.25, 1.0),
    );

    let stats = [
        (
            tr("ĐỢT ĐẠT ĐƯỢC (WAVE):", "WAVE REACHED:"),
            format!("{}", wave),
            WHITE,
        ),
        (
            tr("TỔNG ĐIỂM SỐ:", "TOTAL SCORE:"),
            format!("{}", score),
            Color::new(1.0, 0.84, 0.0, 1.0),
        ),
        (
            tr("QUÁI ĐÃ TIÊU DIỆT:", "ENEMIES ELIMINATED:"),
            format!("{}", total_kills),
            Color::new(0.0, 0.94, 1.0, 1.0),
        ),
        (
            tr("ĐỘ CHÍNH XÁC XẢ ĐẠN:", "SHOOTING ACCURACY:"),
            if shots_fired > 0 {
                format!("{:.1}%", shots_hit as f32 / shots_fired as f32 * 100.0)
            } else {
                "0%".to_string()
            },
            Color::new(0.0, 1.0, 0.5, 1.0),
        ),
    ];

    let mut sy = py + 104.0;
    for (label, val, val_col) in stats {
        draw_text_fitted(label, px + 50.0, sy, 240.0, 16.0, Color::new(0.7, 0.8, 0.9, 1.0));
        draw_text_fitted(&val, px + 310.0, sy, 200.0, 18.0, val_col);
        sy += 40.0;
    }

    let clicked = draw_tactical_button(
        cx - 180.0,
        py + panel_h - 76.0,
        360.0,
        50.0,
        tr("CHƠI LẠI TỪ ĐẦU [ENTER / R]", "PLAY AGAIN [ENTER / R]"),
        Color::new(0.0, 1.0, 0.65, 1.0),
    );

    clicked || is_key_pressed(KeyCode::Enter) || is_key_pressed(KeyCode::R)
}
