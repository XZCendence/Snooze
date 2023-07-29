use std::env;
use imgui::sys;

pub struct ImThemeBasicAccentBased {
    pub main_color: [f32; 4],
    pub background_darken_factor: f32,
    pub alpha_coefficient: f32,
}

pub fn get_platform() -> String {
    let platform = env::consts::OS;
    let platform = platform.to_string();
    platform
}

//unsafe function to cal the sys api and push our theme
pub fn push_style_custom(theme: &ImThemeBasicAccentBased){
    unsafe {
        let platform_specific_window_rounding = match get_platform().as_str() {
            "windows" => 5.0,
            "macos" => 10.0,
            _ => 5.0,
        };
        // COLORS
        //let _title_bar = ui.push_style_color(StyleColor::TitleBgActive, ui_util_state.accent_color);
        //let _original_window_bg_color = ui.push_style_color(StyleColor::WindowBg, darken_color_bg(ui_util_state.accent_color));
        //let _frame_rounding = ui.push_style_var(StyleVar::FrameRounding(5.0));
        //let _child_rounding = ui.push_style_var(StyleVar::ChildRounding(5.0));
        sys::igPushStyleVar_Float(sys::ImGuiStyleVar::from(3), platform_specific_window_rounding);
        sys::igPushStyleVar_Float(sys::ImGuiStyleVar::from(7), platform_specific_window_rounding);
        sys::igPushStyleColor_Vec4(sys::ImGuiCol::from(11), sys::ImVec4 { x: theme.main_color[0], y: theme.main_color[1], z: theme.main_color[2], w: theme.main_color[3] });
        sys::igPushStyleColor_Vec4(sys::ImGuiCol::from(2), sys::ImVec4 { x: theme.main_color[0], y: theme.main_color[1], z: theme.main_color[2], w: theme.main_color[3] });
    }
}

pub fn darken_color_bg(color: [f32; 4]) -> [f32; 4] {
    [
        color[0] * 0.1,
        color[1] * 0.1,
        color[2] * 0.1,
        color[3] * 0.5,
    ]
}