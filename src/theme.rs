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
        let _window_rounding = sys::igPushStyleVar_Float(sys::ImGuiStyleVar::from(3), platform_specific_window_rounding);
        let _child_rounding = sys::igPushStyleVar_Float(sys::ImGuiStyleVar::from(7), platform_specific_window_rounding);
        let _frame_rounding = sys::igPushStyleVar_Float(sys::ImGuiStyleVar::from(12), 5.0);
        let _frame_border_size = sys::igPushStyleVar_Float(sys::ImGuiStyleVar::from(13), 1.0);
        let _frame_padding = sys::igPushStyleVar_Vec2(sys::ImGuiStyleVar::from(11), sys::ImVec2 { x: 4.0, y: 4.0 });

        // Colors
        let darkened_title_bg = [
            theme.main_color[0] * 0.6,
            theme.main_color[1] * 0.6,
            theme.main_color[2] * 0.6,
            theme.main_color[3],
        ];
        let darkened_title_bg_imvec4 = sys::ImVec4 { x: darkened_title_bg[0], y: darkened_title_bg[1], z: darkened_title_bg[2], w: darkened_title_bg[3] };
        let _title_bg_active = sys::igPushStyleColor_Vec4(sys::ImGuiCol::from(11), darkened_title_bg_imvec4);
        let darkened_bg = [
            theme.main_color[0] * theme.background_darken_factor,
            theme.main_color[1] * theme.background_darken_factor,
            theme.main_color[2] * theme.background_darken_factor,
            theme.main_color[3] * theme.alpha_coefficient,
        ];
        let darkened_bg_imvec4 = sys::ImVec4 { x: darkened_bg[0], y: darkened_bg[1], z: darkened_bg[2], w: darkened_bg[3] };
        let _window_bg = sys::igPushStyleColor_Vec4(sys::ImGuiCol::from(2), darkened_bg_imvec4);

        // TABS -----
        let _tab = sys::igPushStyleColor_Vec4(sys::ImGuiCol::from(33),
                                              sys::ImVec4 { x: theme.main_color[0] * 0.7, y: theme.main_color[1] * 0.7, z: theme.main_color[2] * 0.7, w: theme.main_color[3]});
        let _tab_hovered = sys::igPushStyleColor_Vec4(sys::ImGuiCol::from(34),
                                                      sys::ImVec4 { x: theme.main_color[0], y: theme.main_color[1], z: theme.main_color[2], w: theme.main_color[3] });
        let _tab_active = sys::igPushStyleColor_Vec4(sys::ImGuiCol::from(35),
                                                     sys::ImVec4 { x: theme.main_color[0], y: theme.main_color[1], z: theme.main_color[2], w: theme.main_color[3] });
        let _tab_unfocused = sys::igPushStyleColor_Vec4(sys::ImGuiCol::from(36),
                                                        sys::ImVec4 { x: theme.main_color[0] * 0.35, y: theme.main_color[1] * 0.35, z: theme.main_color[2] * 0.35, w: theme.main_color[3]});
        let _tab_unfocused_active = sys::igPushStyleColor_Vec4(sys::ImGuiCol::from(37),
                                                               sys::ImVec4 { x: theme.main_color[0] * 0.5, y: theme.main_color[1] * 0.5, z: theme.main_color[2] * 0.5, w: theme.main_color[3]});

        let _frame_bg = sys::igPushStyleColor_Vec4(sys::ImGuiCol::from(7),
                                                   sys::ImVec4 { x: theme.main_color[0] * 0.6, y: theme.main_color[1] * 0.6, z: theme.main_color[2] * 0.6, w: theme.main_color[3] * 0.5});
        let _border = sys::igPushStyleColor_Vec4(sys::ImGuiCol::from(5),
                                                 sys::ImVec4 { x: theme.main_color[0] * 0.7, y: theme.main_color[1] * 0.7, z: theme.main_color[2] * 0.7, w: theme.main_color[3] * 0.7 });
        let _menu_bar_bg = sys::igPushStyleColor_Vec4(sys::ImGuiCol::from(13),
                                                      sys::ImVec4 { x: theme.main_color[0] * 0.4, y: theme.main_color[1] * 0.4, z: theme.main_color[2] * 0.4, w: theme.main_color[3]});
    }
}

//safe function to quicklu push a button color

pub fn pop_button_color(){
    unsafe {
        sys::igPopStyleColor(3);
    }
}

pub fn pop_style_custom(){
    unsafe {
        sys::igPopStyleColor(10);
        sys::igPopStyleVar(5);
    }
}
