use std::env;
use imgui::{Condition, StyleColor, StyleVar};
use crate::colors::{MAIN_COLOR, darken_color_bg};
use crate::state::GuiAppState;

pub fn get_platform() -> String {
    let platform = env::consts::OS;
    let platform = platform.to_string();
    platform
}

// Draw function (called every frame, so don't expect anything here to persist)
pub fn draw_ui(ui: &imgui::Ui, app_state: &GuiAppState) {
    let original_window_bg_color = ui.push_style_color(StyleColor::WindowBg, darken_color_bg(app_state.ui_util_state.lock().unwrap().accent_color));
    let platform_specific_window_rounding = match get_platform().as_str() {
        "windows" => 5.0,
        "macos" => 10.0,
        _ => 5.0,
    };
    let window_rounding = ui.push_style_var(StyleVar::WindowRounding(platform_specific_window_rounding));
    let title_bar = ui.push_style_color(StyleColor::TitleBgActive, app_state.ui_util_state.lock().unwrap().accent_color);

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
                    if ui.menu_item("Change theme color") {
                        //bring up settings window
                        app_state.ui_util_state.lock().unwrap().settings_window_open = true;
                    }
                });
        });

    const MENU_BAR_HEIGHT: f32 = 22.0;

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
            ui.text(app_state.ui_util_state.lock().unwrap().main_window_has_focus.to_string());
            ui.input_text("URL", &mut app_state.request_state.lock().unwrap().url)
                .build();
        });
    //if this window has focus, set the bool in the UiUtilState to true

    if app_state.ui_util_state.lock().unwrap().settings_window_open {
        ui.window("Settings")
            .size([300.0, 200.0], Condition::FirstUseEver)
            .build(|| {
                ui.text("Change theme color");
                ui.color_edit4("Accent color", &mut app_state.ui_util_state.lock().unwrap().accent_color);
                if ui.button("Close") {
                    app_state.ui_util_state.lock().unwrap().settings_window_open = false;
                }
            });
    }
}