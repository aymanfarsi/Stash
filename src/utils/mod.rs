pub mod enums;
pub mod run_main_app;
pub mod run_first_error_app;

pub fn calc_btn_size_from_text(text: &str) -> f32 {
    text.len() as f32 * 10.0
}
