use ratatui::{
    layout::Rect,
    style::{Color, Style, palette::material::WHITE},
};

use crate::tui::ColorType;

const PALE_CYAN: Color = Color::Rgb(155, 255, 255);
const CYAN: Color = Color::Rgb(0, 255, 255);
const DARK_CYAN: Color = Color::Rgb(0, 155, 155);
// const MODERATE_CYAN: Color = Color::Rgb(75, 233, 233);
const GRAY155: Color = Color::Rgb(155, 155, 155);
// const GRAY195: Color = Color::Rgb(195, 195, 195);

pub fn text_color(focused: bool, color_type: &ColorType, selected: bool) -> Color {
    if focused && selected {
        match color_type {
            ColorType::Primary => WHITE,
            ColorType::Secondary => CYAN,
        }
    } else {
        match color_type {
            ColorType::Primary => WHITE,
            ColorType::Secondary => GRAY155,
        }
    }
}

pub fn border_color(focused: bool) -> Color {
    if focused { PALE_CYAN } else { WHITE }
}

pub fn line_style(focused: bool, selected: bool) -> Style {
    if focused && selected {
        Style::new().bg(DARK_CYAN)
    } else {
        Style::new()
    }
}

pub fn inside_area(area: &Rect) -> Rect {
    Rect {
        x: area.x + 2,
        y: area.y + 1,
        width: area.width.saturating_sub(4),
        height: area.height.saturating_sub(2),
    }
}
