use egui::{vec2, Button, Rounding};

use crate::utils::calc_btn_size_from_text;

pub fn custom_button(ui: &mut egui::Ui, text: &str, width: Option<f32>, on_press: impl FnOnce()) {
    let min_size = match width {
        Some(width) => vec2(width, 30.0),
        None => vec2(calc_btn_size_from_text(text), 30.0),
    };
    let button = Button::new(text)
        .frame(true)
        .min_size(min_size)
        .rounding(Rounding::same(7.0));
    if ui.add(button).clicked() {
        on_press();
    }
}

pub fn custom_header(ui: &mut egui::Ui, text: &str, on_press: impl FnOnce()) {
    let button = Button::new(format!("{} ⏷", text))
        .frame(true)
        .min_size(vec2(calc_btn_size_from_text(text), 30.0))
        .rounding(Rounding::same(7.0));
    if ui.add(button).clicked() {
        on_press();
    }
}

pub fn custom_tile(ui: &mut egui::Ui, text: &str, on_press: impl FnOnce()) {
    let button = Button::new(format!("|  {}", text))
        .frame(true)
        .min_size(vec2(calc_btn_size_from_text(text), 30.0))
        .rounding(Rounding::same(7.0));
    if ui.add(button).clicked() {
        on_press();
    }
}
