use std::env;
use std::sync::{Arc, Mutex};
use imgui::{Condition, StyleColor, StyleVar};
use imgui::StyleColor::Text;
use crate::colors::{MAIN_COLOR, darken_color_bg};

pub fn get_platform() -> String {
    let platform = env::consts::OS;
    let platform = platform.to_string();
    platform
}

// Draw function (called every frame, so don't expect anything here to persist)
pub fn draw_ui(ui: &imgui::Ui) {
    let original_window_bg_color = ui.push_style_color(StyleColor::WindowBg, darken_color_bg(MAIN_COLOR));
    let platform_specific_window_rounding = match get_platform().as_str() {
        "windows" => 5.0,
        "macos" => 10.0,
        _ => 5.0,
    };
    let window_rounding = ui.push_style_var(StyleVar::WindowRounding(platform_specific_window_rounding));
    let title_bar = ui.push_style_color(StyleColor::TitleBgActive, MAIN_COLOR);

        ui.main_menu_bar(|| {
            ui.menu("File", ||
                {
                    ui.menu_item("Open workspace");
                    ui.menu_item("Save request as");
                    ui.separator();
                    if ui.menu_item("Exit") {
                        std::process::exit(0);
                    }
                });
            ui.menu("View", ||
                {
                    ui.menu_item("Change accent color");
                });
        });

    const MENU_BAR_HEIGHT: f32 = 20.0;

    ui.window(" Main window")
        //make the window fill the main window
        .position([0.0, MENU_BAR_HEIGHT], Condition::Always)
        //glow is a little different, so we need to get the current window size
        .size(
            [
                ui.io().display_size[0],
                ui.io().display_size[1]-MENU_BAR_HEIGHT,
            ],
            Condition::Always,
        )
        //get rid of the resize handle
        .resizable(false)
        .build(|| {


        });
}