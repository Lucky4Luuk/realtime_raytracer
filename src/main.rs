#[macro_use] extern crate log;
#[macro_use] extern crate imgui;

use std::os::raw::c_void;
use std::time::Instant;

use sdl2::event::Event;

mod window;
use window::Window;

pub mod compute;
pub mod gl_util;
pub mod shader;

fn main() {
    // let level_filter = log::LevelFilter::max();
    let level_filter = log::LevelFilter::Debug;

    pretty_env_logger::formatted_builder()
        .filter_level(level_filter)
        .init();

    debug!("Hello, world!");

    let window = Window::new((4,5), "Realtime Raytracer", (1280, 720), false).unwrap();
    let _gl_context = window.window.gl_create_context().expect("Failed to create GL context!");
    gl::load_with(|s| window.video.gl_get_proc_address(s) as _);

    let mut imgui = imgui::Context::create();
    imgui.set_ini_filename(None);
    // ui::style_ui(imgui.style_mut());

    let mut imgui_sdl2 = imgui_sdl2::ImguiSdl2::new(&mut imgui, &window.window);

    let ui_renderer = imgui_opengl_renderer::Renderer::new(&mut imgui, |s| window.video.gl_get_proc_address(s) as *const c_void);

    let mut event_pump = window.sdl.event_pump().expect("Failed to get event pump!");

    let mut last_frame = Instant::now();
    let mut delta_s: f32 = 1.0;

    //SETUP
    let quad_vs = shader::Shader::from_vertex(include_str!("shaders/passthrough_vertex.glsl"));
    let quad_fs = shader::Shader::from_fragment(include_str!("shaders/passthrough_fragment_textured.glsl"));
    let quad_program = shader::Program::from_shaders(vec![quad_vs, quad_fs]);
    let quad_va = gl_util::create_render_quad();
    let render_tex = gl_util::create_texture((1280, 720));

    'main: loop {
        for event in event_pump.poll_iter() {
            imgui_sdl2.handle_event(&mut imgui, &event);
            if imgui_sdl2.ignore_event(&event) { continue; }

            match event {
                Event::Quit{..} => {
                    break 'main;
                },
                _ => {},
            }
        }

        unsafe {
            gl::ClearColor(127.0 / 255.0, 103.0 / 255.0, 181.0 / 255.0, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }

        imgui_sdl2.prepare_frame(imgui.io_mut(), &window.window, &event_pump.mouse_state());

        //Render textured quad
        unsafe {
            gl::UseProgram(quad_program.handle);
            gl::BindVertexArray(quad_va);
            gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, std::ptr::null());
            gl::BindVertexArray(0);
            gl::UseProgram(0);
        }

        let ui = imgui.frame();

        let debug_window = imgui::Window::new(im_str!("Debug window"))
            .position([10.0, 10.0], imgui::Condition::Appearing)
            .size([320.0, 120.0], imgui::Condition::Appearing)
            .focused(false)
            .collapsible(true);

        debug_window.build(&ui, || {
            ui.text(format!("fps: {:.2}", 1.0 / delta_s));
            // ui.separator();
            // ui.text(format!("cam pos: {:?}", camera.position));
        });

        imgui_sdl2.prepare_render(&ui, &window.window);
        ui_renderer.render(ui);

        let now = Instant::now();
        let delta = now - last_frame;
        delta_s = delta.as_secs() as f32 + delta.subsec_nanos() as f32 / 1_000_000_000.0;
        last_frame = now;
        imgui.io_mut().delta_time = delta_s;

        // window.set_title(&format!("FPS: {}", 1.0 / delta_s));

        window.swap_buffer();
    }
}
