use std::env;
use std::os::raw::c_char;
use imgui::{Condition, dear_imgui_version, Direction, StyleColor, StyleVar, sys};
use imgui::sys::{ImGuiCol, ImGuiStyleVar, ImGuiStyleVar_, ImGuiStyleVar_WindowRounding, ImVec4};
use crate::colors::{MAIN_COLOR, darken_color_bg};
use crate::state::{GuiAppState, UiUtilState};

//docking boilerplate

pub struct DockNode {
    id: u32,
}

impl DockNode {
    fn new(id: u32) -> Self {
        Self { id }
    }

    pub fn is_split(&self) -> bool {
        unsafe {
            let node = sys::igDockBuilderGetNode(self.id);
            if node.is_null() {
                false
            } else {
                sys::ImGuiDockNode_IsSplitNode(node)
            }
        }
    }
    /// Dock window into this dockspace
    #[doc(alias = "DockBuilder::DockWindow")]
    pub fn dock_window(&self, window: &str) {
        let window = imgui::ImString::from(window.to_string());
        unsafe {
            sys::igDockBuilderDockWindow(window.as_ptr(), self.id);
        }
    }

    #[doc(alias = "DockBuilder::SplitNode")]
    pub fn split<D, O>(&self, split_dir: Direction, size_ratio: f32, dir: D, opposite_dir: O)
        where
            D: FnOnce(DockNode),
            O: FnOnce(DockNode),
    {
        if self.is_split() {
            // Can't split an already split node (need to split the
            // node within)
            return;
        }

        let mut out_id_at_dir: sys::ImGuiID = 0;
        let mut out_id_at_opposite_dir: sys::ImGuiID = 0;
        unsafe {
            sys::igDockBuilderSplitNode(
                self.id,
                split_dir as i32,
                size_ratio,
                &mut out_id_at_dir,
                &mut out_id_at_opposite_dir,
            );
        }

        dir(DockNode::new(out_id_at_dir));
        opposite_dir(DockNode::new(out_id_at_opposite_dir));
    }
}

/// # Docking

pub struct UiDocking {}

impl UiDocking {
    #[doc(alias = "IsWindowDocked")]
    pub fn is_window_docked(&self) -> bool {
        unsafe { sys::igIsWindowDocked() }
    }
    /// Create dockspace with given label. Returns a handle to the
    /// dockspace which can be used to, say, programatically split or
    /// dock windows into it
    #[doc(alias = "DockSpace")]
    pub fn dockspace(&self, label: &str) -> DockNode {
        let label = imgui::ImString::from(label.to_string());
        unsafe {
            let id = sys::igGetID_Str(label.as_ptr() as *const c_char);
            sys::igDockSpace(
                id,
                [0.0, 0.0].into(),
                0,
                ::std::ptr::null::<sys::ImGuiWindowClass>(),
            );
            DockNode { id }
        }
    }

    #[doc(alias = "DockSpaceOverViewport")]
    pub fn dockspace_over_viewport(&self) {
        unsafe {
            sys::igDockSpaceOverViewport(
                sys::igGetMainViewport(),
                0,
                ::std::ptr::null::<sys::ImGuiWindowClass>(),
            );
        }
    }
}

pub fn get_platform() -> String {
    let platform = env::consts::OS;
    let platform = platform.to_string();
    platform
}

// Draw function (called every frame, so don't expect anything here to persist)
pub fn draw_ui(ui: &imgui::Ui, app_state: &GuiAppState, ui_util_state: &mut UiUtilState) {

    let flags =
        imgui::WindowFlags::NO_DECORATION | imgui::WindowFlags::NO_MOVE
            | imgui::WindowFlags::MENU_BAR
            | imgui::WindowFlags::NO_BRING_TO_FRONT_ON_FOCUS | imgui::WindowFlags::NO_NAV_FOCUS
            | imgui::WindowFlags::NO_DOCKING
        ;

    let platform_specific_window_rounding = match get_platform().as_str() {
        "windows" => 5.0,
        "macos" => 10.0,
        _ => 5.0,
    };
    let _window_rounding = ui.push_style_var(StyleVar::WindowRounding(platform_specific_window_rounding));
    // COLORS
    let _title_bar = ui.push_style_color(StyleColor::TitleBgActive, ui_util_state.accent_color);
    let _original_window_bg_color = ui.push_style_color(StyleColor::WindowBg, darken_color_bg(ui_util_state.accent_color));
    let _frame_rounding = ui.push_style_var(StyleVar::FrameRounding(5.0));
    let _child_rounding = ui.push_style_var(StyleVar::ChildRounding(5.0));

        ui.main_menu_bar(|| {
            ui.menu("File", ||
                {
                    ui.menu_item("Open Workspace");
                    ui.menu_item("Save Request As");
                    ui.separator();
                    if ui.menu_item("Exit") {
                        std::process::exit(0);
                    }
                });
            ui.menu("Window", ||
                {
                    if ui.menu_item("Preferences") {
                        //bring up settings window
                        ui_util_state.settings_window_open = true;
                    }
                    if ui.menu_item("Output Log") {
                        //bring up log window
                        ui_util_state.log_open = true;
                    }
                });
        });

    const MENU_BAR_HEIGHT: f32 = 22.0;
    ui.window("Main Window")
        .flags(flags)
        .position([0.0, 0.0], imgui::Condition::Always)
        .size(ui.io().display_size, imgui::Condition::Always)
        .build(|| unsafe {

            // Create top-level docking area, needs to be made early (before docked windows)
            let ui_d = UiDocking {};
            let space = ui_d.dockspace("MainDockArea");

            // Set up splits, docking windows. This can be done conditionally,
            // or calling it every time is also mostly fine
            space.split(
                imgui::Direction::Left,
                0.7,
                |left| {
                    left.dock_window("Request");
                },
                |right| {
                    right.dock_window("Response");
                },
            );
            //ui.show_metrics_window(&mut true);
            // Create application windows as normal
            ui.window("Request")
                .size([300.0, 110.0], Condition::FirstUseEver)
                .build(|| {
                    ui.input_text("URL", &mut app_state.request_state.lock().unwrap().url)
                        .build();
                    //button for GET, POST, PUT, DELETE, PATCH, OPTIONS, HEAD
                    //each is a different color

                });
            ui.window("Response")
                .size([300.0, 110.0], Condition::FirstUseEver)
                .build(|| {
                    ui.text("Window 2");
                });
        });

    ui.show_metrics_window(&mut true);

    if ui_util_state.settings_window_open {
        ui.window("Settings")
            .opened(&mut ui_util_state.settings_window_open)
            .size([300.0, 200.0], Condition::FirstUseEver)
            .build(|| {
                ui.text("Change theme color");
                ui.color_edit4("Accent color", &mut ui_util_state.accent_color);
            });
    }

    if ui_util_state.log_open {
        ui.window("Output Log")
            .opened(&mut ui_util_state.log_open)
            .size([300.0, 200.0], Condition::FirstUseEver)
            .build(|| {
                ui.text("Log");
            });
    }
    //log the FPS every 100 frames
    if ui.frame_count() % 100 == 0 {
        println!("FPS: {}", ui.io().framerate);
    }
    //try to get the refresh rate of the monitor
    //println!("Refresh rate: {}", ui.io().display_framebuffer_scale[0]);


    std::thread::sleep(std::time::Duration::from_millis(12));
}