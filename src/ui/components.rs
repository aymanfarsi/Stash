use egui::{vec2, Button, Rounding};

pub fn custom_button(ui: &mut egui::Ui, text: &str, on_press: impl FnOnce()) {
    let button = Button::new(text)
        .frame(true)
        .min_size(vec2(calc_btn_size_from_text(text), 30.0))
        .rounding(Rounding::same(7.0));
    if ui.add(button).clicked() {
        on_press();
    }
}

pub fn calc_btn_size_from_text(text: &str) -> f32 {
    text.len() as f32 * 10.0
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
