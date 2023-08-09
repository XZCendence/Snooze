use imgui::{Condition, TabItem};
use crate::docking::UiDocking;
use crate::state::{GuiAppState, UiUtilState, DualFontData};

// Draw a request window (we can have up to 4)
fn draw_request_window(num: u8, ui: &imgui::Ui, state: &GuiAppState, font_data: &DualFontData) {
    let window_name = format!("Request {}", num);
    // if it's request 1, it should span the whole window
    let size = if num == 1 {
        [ui.io().display_size[0], ui.io().display_size[1]]
    } else {
        [300.0, 110.0]
    };
    ui.window(&window_name)
        .size(size, Condition::FirstUseEver)
        .build(|| {
            ui.columns(2, &window_name, true);

            ui.input_text("URL", &mut state.get_request_state_mut().url)
                .build();

            /// # HTTP Method Buttons
            let get_n = ui.push_style_color(imgui::StyleColor::Button, [0.3, 0.8, 0.3, 0.7]); //normal
            let get_h = ui.push_style_color(imgui::StyleColor::ButtonHovered, [0.3, 1.0, 0.3, 0.7]); //hovered (slightly lighter)
            let get_a = ui.push_style_color(imgui::StyleColor::ButtonActive, [0.3, 0.8, 0.3, 0.7]); //active (slightly darker)
            // border color should be the same as the hovered color
            let get_b = ui.push_style_color(imgui::StyleColor::Border, [0.3, 1.0, 0.3, 0.7]);
            if ui.button("GET") {
                println!("GET");
            }
            get_n.pop();
            get_h.pop();
            get_a.pop();
            get_b.pop();
            let post_n = ui.push_style_color(imgui::StyleColor::Button, [0.3, 0.2, 0.8, 0.7]); //normal
            let post_h = ui.push_style_color(imgui::StyleColor::ButtonHovered, [0.4, 0.2, 1.0, 0.7]); //hovered (slightly lighter)
            let post_a = ui.push_style_color(imgui::StyleColor::ButtonActive, [0.3, 0.2, 0.8, 0.7]); //active (slightly darker)
            // border color should be the same as the hovered color
            let post_b = ui.push_style_color(imgui::StyleColor::Border, [0.4, 0.2, 1.0, 0.7]);
            ui.same_line();
            if ui.button("POST") {
                println!("POST");
            }
            post_n.pop();
            post_h.pop();
            post_a.pop();
            post_b.pop();
            ui.same_line();
            let put_n = ui.push_style_color(imgui::StyleColor::Button, [0.8, 0.8, 0.2, 0.7]); //normal
            let put_h = ui.push_style_color(imgui::StyleColor::ButtonHovered, [1.0, 1.0, 0.2, 0.7]); //hovered (slightly lighter)
            let put_a = ui.push_style_color(imgui::StyleColor::ButtonActive, [0.8, 0.8, 0.2, 0.7]); //active (slightly darker)
            // border color should be the same as the hovered color
            let put_b = ui.push_style_color(imgui::StyleColor::Border, [1.0, 1.0, 0.2, 0.7]);
            if ui.button("PUT") {
                println!("PUT");
            }
            put_n.pop();
            put_h.pop();
            put_a.pop();
            put_b.pop();
            ui.same_line();
            let delete_n = ui.push_style_color(imgui::StyleColor::Button, [0.8, 0.2, 0.2, 0.7]); //normal
            let delete_h = ui.push_style_color(imgui::StyleColor::ButtonHovered, [1.0, 0.2, 0.2, 0.7]); //hovered (slightly lighter)
            let delete_a = ui.push_style_color(imgui::StyleColor::ButtonActive, [0.8, 0.2, 0.2, 0.7]); //active (slightly darker)
            // border color should be the same as the hovered color
            let delete_b = ui.push_style_color(imgui::StyleColor::Border, [1.0, 0.2, 0.2, 0.7]);
            if ui.button("DELETE") {
                println!("DELETE");
            }
            delete_n.pop();
            delete_h.pop();
            delete_a.pop();
            delete_b.pop();
            ui.same_line();
            let patch_n = ui.push_style_color(imgui::StyleColor::Button, [0.2, 0.8, 0.8, 0.7]); //normal
            let patch_h = ui.push_style_color(imgui::StyleColor::ButtonHovered, [0.2, 1.0, 1.0, 0.7]); //hovered (slightly lighter)
            let patch_a = ui.push_style_color(imgui::StyleColor::ButtonActive, [0.2, 0.8, 0.8, 0.7]); //active (slightly darker)
            // border color should be the same as the hovered color
            let patch_b = ui.push_style_color(imgui::StyleColor::Border, [0.2, 1.0, 1.0, 0.7]);
            if ui.button("PATCH") {
                println!("PATCH");
            }
            patch_n.pop();
            patch_h.pop();
            patch_a.pop();
            patch_b.pop();
            ui.same_line();
            let head_n = ui.push_style_color(imgui::StyleColor::Button, [0.8, 0.2, 0.8, 0.7]); //normal
            let head_h = ui.push_style_color(imgui::StyleColor::ButtonHovered, [1.0, 0.2, 1.0, 0.7]); //hovered (slightly lighter)
            let head_a = ui.push_style_color(imgui::StyleColor::ButtonActive, [0.8, 0.2, 0.8, 0.7]); //active (slightly darker)
            // border color should be the same as the hovered color
            let head_b = ui.push_style_color(imgui::StyleColor::Border, [1.0, 0.2, 1.0, 0.7]);
            if ui.button("HEAD") {
                println!("HEAD");
            }
            head_n.pop();
            head_h.pop();
            head_a.pop();
            head_b.pop();
            ui.same_line();
            let options_n = ui.push_style_color(imgui::StyleColor::Button, [0.8, 0.3, 0.2, 0.7]); //normal
            let options_h = ui.push_style_color(imgui::StyleColor::ButtonHovered, [1.0, 0.4, 0.2, 0.7]); //hovered (slightly lighter)
            let options_a = ui.push_style_color(imgui::StyleColor::ButtonActive, [0.8, 0.3, 0.2, 0.7]); //active (slightly darker)
            // border color should be the same as the hovered color
            let options_b = ui.push_style_color(imgui::StyleColor::Border, [1.0, 0.4, 0.2, 0.7]);
            if ui.button("OPTIONS") {
                println!("OPTIONS");
            }
            options_n.pop();
            options_h.pop();
            options_a.pop();
            options_b.pop();
            let tab_bar = ui.tab_bar("Request Editor Tabs");
            TabItem::new("Headers")
                .build(&ui, || {
                    ui.show_demo_window(&mut true);
                    ui.text("Headers");
                });

            TabItem::new("Body")
                .build(&ui, || {
                    let font = ui.push_font(font_data.font_beta);
                    let free_space = ui.content_region_avail();



                            ui.input_text_multiline("Body", &mut state.get_request_state_mut().body, free_space)
                                .allow_tab_input(true)
                                .build();

                    font.pop();
                });

            TabItem::new("Query")
                .build(&ui, || {
                    ui.text("Query");
                });

            ui.next_column();

            ui.text("Response");
        });
}

    pub fn draw_ui(
        ui: &imgui::Ui,
        ui_util_state: &mut UiUtilState,
        one_state: &GuiAppState,
        two_state: &GuiAppState,
        three_state: &GuiAppState,
        four_state: &GuiAppState,
        font_data: &DualFontData,
    ){
    let flags =
        imgui::WindowFlags::NO_DECORATION | imgui::WindowFlags::NO_MOVE
            | imgui::WindowFlags::MENU_BAR
            | imgui::WindowFlags::NO_BRING_TO_FRONT_ON_FOCUS | imgui::WindowFlags::NO_NAV_FOCUS
            | imgui::WindowFlags::NO_DOCKING
        ;

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
                        ui_util_state.settings_window_open = true;
                    }
                    if ui.menu_item("Output Log") {
                        ui_util_state.log_open = true;
                    }
                    if ui.menu_item_config("Request 2").build() {
                        ui_util_state.request_2_open = true;
                    }

                });
        });

    const MENU_BAR_HEIGHT: f32 = 22.0;
    ui.window("Main Window")
        .flags(flags)
        .position([0.0, 0.0], imgui::Condition::Always)
        .size(ui.io().display_size, imgui::Condition::Always)
        .build(||{

            // Create top-level docking area, needs to be made early (before docked windows)
            let ui_d = UiDocking {};
            let space = ui_d.dockspace("MainDockArea");

            //we want it do only dock the request 1 window once

            if ui.frame_count() < 10 {
                space.dock_window("Request 1");
            }

            draw_request_window(1, ui, one_state, font_data);

            if (ui_util_state.request_2_open) {
                draw_request_window(2, ui, two_state, font_data);
            }
            if (ui_util_state.request_3_open) {
                draw_request_window(3, ui, three_state, font_data);
            }
            if (ui_util_state.request_4_open) {
                draw_request_window(4, ui, four_state, font_data);
            }

        });

    //ui.show_demo_window(&mut true);

    if ui_util_state.settings_window_open {
        ui.window("Settings")
            .opened(&mut ui_util_state.settings_window_open)
            .size([300.0, 100.0], Condition::FirstUseEver)
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

    std::thread::sleep(std::time::Duration::from_millis(12));
}