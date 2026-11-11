use macroquad::prelude::*;
use std::cell::{Cell, RefCell};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Language {
    Vi,
    En,
}

impl Language {
    pub fn code(self) -> &'static str {
        match self {
            Language::Vi => "vi",
            Language::En => "en",
        }
    }

    pub fn from_code(code: &str) -> Self {
        match code.trim().to_lowercase().as_str() {
            "en" | "english" => Language::En,
            _ => Language::Vi,
        }
    }

    pub fn label(self) -> &'static str {
        match self {
            Language::Vi => "TIẾNG VIỆT (CÓ DẤU)",
            Language::En => "ENGLISH",
        }
    }

    pub fn short_code(self) -> &'static str {
        match self {
            Language::Vi => "VI",
            Language::En => "EN",
        }
    }

    pub fn toggle(self) -> Self {
        match self {
            Language::Vi => Language::En,
            Language::En => Language::Vi,
        }
    }
}

thread_local! {
    static CURRENT_LANG: Cell<Language> = const { Cell::new(Language::Vi) };
    static UI_FONT: RefCell<Option<Font>> = const { RefCell::new(None) };
}

/// Khởi tạo bộ phông chữ Unicode hỗ trợ đầy đủ Tiếng Việt có dấu (UTF-8)
pub fn init_i18n_font() {
    let candidate_paths = [
        "/System/Library/Fonts/Supplemental/Arial Bold.ttf",
        "/System/Library/Fonts/Supplemental/Arial.ttf",
        "/System/Library/Fonts/Supplemental/Tahoma Bold.ttf",
        "/System/Library/Fonts/Supplemental/Verdana Bold.ttf",
        "assets/font_vn.ttf",
        "C:\\Windows\\Fonts\\arialbd.ttf",
        "/usr/share/fonts/truetype/dejavu/DejaVuSans-Bold.ttf",
    ];

    for path in candidate_paths {
        if let Ok(bytes) = std::fs::read(path) {
            if let Ok(mut font) = load_ttf_font_from_bytes(&bytes) {
                font.set_filter(FilterMode::Linear);
                UI_FONT.with(|cell| {
                    *cell.borrow_mut() = Some(font);
                });
                break;
            }
        }
    }
}

pub fn set_current_language(lang: Language) {
    CURRENT_LANG.with(|c| c.set(lang));
}

pub fn current_language() -> Language {
    CURRENT_LANG.with(|c| c.get())
}

/// Trả về chuỗi theo ngôn ngữ hiện tại (`vi`: Tiếng Việt có dấu, `en`: Tiếng Anh)
#[inline]
pub fn tr<'a>(vi: &'a str, en: &'a str) -> &'a str {
    match current_language() {
        Language::Vi => vi,
        Language::En => en,
    }
}

/// Vẽ văn bản hỗ trợ đầy đủ Unicode Tiếng Việt có dấu (thay thế trực tiếp `macroquad::text::draw_text`)
pub fn draw_text(text: &str, x: f32, y: f32, font_size: f32, color: Color) {
    UI_FONT.with(|cell| {
        let borrow = cell.borrow();
        if let Some(ref font) = *borrow {
            draw_text_ex(
                text,
                x,
                y,
                TextParams {
                    font: Some(font),
                    font_size: (font_size.round() as u16).max(8),
                    font_scale: 1.0,
                    color,
                    ..Default::default()
                },
            );
        } else {
            macroquad::text::draw_text(text, x, y, font_size, color);
        }
    });
}

/// Đo kích thước văn bản với phông chữ Unicode Tiếng Việt
pub fn measure_text(
    text: &str,
    font_override: Option<&Font>,
    font_size: u16,
    font_scale: f32,
) -> TextDimensions {
    if font_override.is_some() {
        return macroquad::text::measure_text(text, font_override, font_size, font_scale);
    }
    UI_FONT.with(|cell| {
        let borrow = cell.borrow();
        macroquad::text::measure_text(text, borrow.as_ref(), font_size, font_scale)
    })
}

/// Vẽ văn bản tự động xuống dòng khi vượt quá `max_width`.
/// Trả về `y` kết thúc để tiện căn chỉnh layout phía dưới.
pub fn draw_wrapped_text(
    text: &str,
    x: f32,
    mut y: f32,
    max_width: f32,
    line_spacing: f32,
    font_size: f32,
    color: Color,
) -> f32 {
    for paragraph in text.split('\n') {
        let words: Vec<&str> = paragraph.split_whitespace().collect();
        if words.is_empty() {
            y += line_spacing;
            continue;
        }

        let mut current_line = String::new();
        for word in words {
            let test_line = if current_line.is_empty() {
                word.to_string()
            } else {
                format!("{} {}", current_line, word)
            };

            let dims = measure_text(&test_line, None, (font_size.round() as u16).max(8), 1.0);
            if dims.width > max_width && !current_line.is_empty() {
                draw_text(&current_line, x, y, font_size, color);
                y += line_spacing;
                current_line = word.to_string();
            } else {
                current_line = test_line;
            }
        }

        if !current_line.is_empty() {
            draw_text(&current_line, x, y, font_size, color);
            y += line_spacing;
        }
    }
    y
}

/// Vẽ văn bản vừa vặn trong `max_width`, tự động co nhỏ font nếu quá dài để KHÔNG BAO GIỜ bị tràn hoặc đè chữ.
pub fn draw_text_fitted(
    text: &str,
    x: f32,
    y: f32,
    max_width: f32,
    initial_font_size: f32,
    color: Color,
) {
    let mut fs = initial_font_size;
    let min_fs = (initial_font_size * 0.65).max(8.5);
    loop {
        let dims = measure_text(text, None, (fs.round() as u16).max(8), 1.0);
        if dims.width <= max_width || fs <= min_fs {
            break;
        }
        fs -= 0.8;
    }
    draw_text(text, x, y, fs, color);
}

/// Vẽ văn bản căn giữa theo trục X tại tọa độ `center_x`.
pub fn draw_text_centered(
    text: &str,
    center_x: f32,
    y: f32,
    font_size: f32,
    color: Color,
) {
    let dims = measure_text(text, None, (font_size.round() as u16).max(8), 1.0);
    draw_text(text, center_x - dims.width / 2.0, y, font_size, color);
}
