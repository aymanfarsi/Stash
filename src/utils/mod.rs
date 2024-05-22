pub mod enums;

pub fn calc_btn_size_from_text(text: &str) -> f32 {
    text.len() as f32 * 10.0
}
