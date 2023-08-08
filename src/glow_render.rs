use std::time::Instant;
use glow::HasContext;
use glutin::{event_loop::EventLoop, WindowedContext};
use imgui_winit_support::WinitPlatform;
use crate::{colors, gui};
use crate::colors::ImThemeBasicAccentBased;
use crate::state::{GuiAppState, UiUtilState};

const TITLE: &str = "Snooze v0.1.1";

type Window = WindowedContext<glutin::PossiblyCurrent>;

pub fn init_gui_loop() {
    // Common setup for creating a winit window and imgui context, not specifc
    // to this renderer at all except that glutin is used to create the window
    // since it will give us access to a GL context
    let (event_loop, window) = create_window();
    let (mut winit_platform, mut imgui_context) = imgui_init(&window);

    let app_state_r1 = GuiAppState::new();
    let app_state_r2 = GuiAppState::new();
    let app_state_r3 = GuiAppState::new();
    let app_state_r4 = GuiAppState::new();

    let mut ui_util_state = UiUtilState::new();
    // OpenGL context from glow
    let gl = glow_context(&window);

    // OpenGL renderer from this crate
    let mut ig_renderer = imgui_glow_renderer::AutoRenderer::initialize(gl, &mut imgui_context)
        .expect("failed to create renderer");

    unsafe {
        ig_renderer.gl_context().enable(glow::BLEND);
        ig_renderer.gl_context().blend_equation(glow::FUNC_ADD);
        ig_renderer.gl_context().blend_func_separate(glow::SRC_ALPHA, glow::ONE_MINUS_SRC_ALPHA, glow::ONE, glow::ONE_MINUS_SRC_ALPHA);
    }

    let mut last_frame = Instant::now();
    // Standard winit event loop
    event_loop.run(move |event, _, control_flow| {
        match event {
            glutin::event::Event::NewEvents(_) => {
                let now = Instant::now();
                imgui_context
                    .io_mut()
                    .update_delta_time(now.duration_since(last_frame));
                last_frame = now;
            }
            glutin::event::Event::MainEventsCleared => {
                winit_platform
                    .prepare_frame(imgui_context.io_mut(), window.window())
                    .unwrap();
                window.window().request_redraw();
            }
            glutin::event::Event::RedrawRequested(_) => {
                // The renderer assumes you'll be clearing the buffer yourself
                unsafe {
                    //clear alpha channel to 0
                    ig_renderer.gl_context().clear_color(0.0, 0.0, 0.0, 0.0);
                    ig_renderer.gl_context().clear(glow::COLOR_BUFFER_BIT | glow::DEPTH_BUFFER_BIT);
                };

                //construct a new color theme
                let theme = ImThemeBasicAccentBased{
                    main_color: ui_util_state.accent_color,
                    background_darken_factor: 0.1,
                    alpha_coefficient: 0.5,
                };
                colors::push_style_custom(&theme);

                let ui = imgui_context.new_frame();

                gui::draw_ui(&ui, &mut ui_util_state, &app_state_r1, &app_state_r2, &app_state_r3, &app_state_r4);

                colors::pop_style_custom();
                winit_platform.prepare_render(ui, window.window());
                let draw_data = imgui_context.render();

                ig_renderer
                    .render(draw_data)
                    .expect("error rendering imgui");

                window.swap_buffers().unwrap();
            }
            glutin::event::Event::WindowEvent {
                event: glutin::event::WindowEvent::CloseRequested,
                ..
            } => {
                *control_flow = glutin::event_loop::ControlFlow::Exit;
            }
            //we want to handle window resizing too

            glutin::event::Event::WindowEvent {
                event: glutin::event::WindowEvent::Resized(size),
                ..
            } => {
                window.resize(size);
                let logical_size = size.to_logical(window.window().scale_factor());
                let logical_size = winit_platform.scale_size_from_winit(window.window(), logical_size);
                imgui_context.io_mut().display_size = [logical_size.width as f32, logical_size.height as f32];
            }
            event => {
                winit_platform.handle_event(imgui_context.io_mut(), window.window(), &event);
            }
        }
    });

}

fn create_window() -> (EventLoop<()>, Window) {
    let event_loop = glutin::event_loop::EventLoop::new();
    let window = glutin::window::WindowBuilder::new()
        .with_title(TITLE)
        .with_transparent(true)
        .with_inner_size(glutin::dpi::LogicalSize::new(1280, 720));
    let window = glutin::ContextBuilder::new()
        .with_vsync(false)
        .build_windowed(window, &event_loop)
        .expect("could not create window");
    let window = unsafe {
        window
            .make_current()
            .expect("could not make window context current")
    };
    (event_loop, window)
}

fn glow_context(window: &Window) -> glow::Context {
    unsafe { glow::Context::from_loader_function(|s| window.get_proc_address(s).cast()) }
}

fn imgui_init(window: &Window) -> (WinitPlatform, imgui::Context) {
    let mut imgui_context = imgui::Context::create();
    imgui_context.set_ini_filename(None);
    imgui_context.io_mut().config_flags |= imgui::ConfigFlags::DOCKING_ENABLE;
    //p
    let mut winit_platform = WinitPlatform::init(&mut imgui_context);
    //we want to make sure we're resizing the window properly
    winit_platform.attach_window(
        imgui_context.io_mut(),
        window.window(),
        imgui_winit_support::HiDpiMode::Default,
    );
    let dpi_factor = winit_platform.hidpi_factor();
    imgui_context
        .fonts()
        //add our Roboto-Regular.ttf font
        .add_font(&[imgui::FontSource::TtfData {
            data: include_bytes!("../fonts/TASAOrbiterDisplay-Regular.otf"),
            size_pixels: (16.0 * dpi_factor) as f32, // Scale font size based on DPI factor
            config: Some(imgui::FontConfig {
                rasterizer_multiply: 1.3,
                oversample_h: 8,
                oversample_v: 8,
                glyph_ranges: imgui::FontGlyphRanges::default(),
                ..imgui::FontConfig::default()
            }),
        }]);

    imgui_context.io_mut().font_global_scale = (1.0 / winit_platform.hidpi_factor()) as f32;

    (winit_platform, imgui_context)
}