use std::env;
use std::sync::{Arc, Mutex};
use imgui::{Condition, StyleColor, StyleVar};
use crate::colors::{MAIN_COLOR, darken_color_bg};

pub fn get_platform() -> String {
    let platform = env::consts::OS;
    let platform = platform.to_string();
    platform
}

//we'll use an arc mutex to store the text buffer
pub static mut url_buffer: String = String::new();

// Draw function (called every frame, so don't expect anything here to persist)
pub fn draw_ui(ui: &imgui::Ui) {
    let original_window_bg_color = ui.push_style_color(StyleColor::WindowBg, darken_color_bg(MAIN_COLOR));
    let platform_specific_window_rounding = match get_platform().as_str() {
        "windows" => 0.0,
        "macos" => 10.0,
        _ => 5.0,
    };
    let window_rounding = ui.push_style_var(StyleVar::WindowRounding(platform_specific_window_rounding));
    let title_bar = ui.push_style_color(StyleColor::TitleBgActive, MAIN_COLOR);

    ui.window(" Main window")
        //make the window fill the main window
        .position([0.0, 0.0], Condition::Always)
        //glow is a little different, so we need to get the current window size
        .size(
            [
                ui.io().display_size[0],
                ui.io().display_size[1],
            ],
            Condition::Always,
        )
        //get rid of the resize handle
        .resizable(false)
        .build(|| {
            unsafe { ui.input_text("URL", &mut url_buffer)
                .build();}

        });
}