use std::os::raw::c_void;

pub struct Window {
    pub sdl: sdl2::Sdl,
    pub video: sdl2::VideoSubsystem,
    pub window: sdl2::video::Window,
}

impl Window {
    //TODO: Return a proper error type
    pub fn new(gl_version: (u8, u8), window_title: &str, window_size: (u32, u32), vsync: bool) -> Result<Self, &'static str> {
        debug!("Initializing sdl...");
        let sdl = sdl2::init().expect("Failed to load SDL!");
        let video = sdl.video().expect("Failed to load video subsystem!");
        debug!("sdl initialized!");

        //Set opengl settings
        {
            let gl_attr = video.gl_attr();
            gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
            gl_attr.set_context_version(gl_version.0, gl_version.1);
            gl_attr.set_double_buffer(true);
        }

        debug!("Opening a window...");
        let window = video.window(window_title, window_size.0, window_size.1)
                        .position_centered()
                        .opengl()
                        .build()
                        .map_err(|e| e.to_string()).expect("Failed to open window!");
        debug!("Window opened!");

        let _gl_context = window.gl_create_context().expect("Failed to create GL context!");
        gl::load_with(|s| video.gl_get_proc_address(s) as *const c_void);

        let swap_interval = if vsync {
            sdl2::video::SwapInterval::VSync
        } else {
            sdl2::video::SwapInterval::Immediate
        };

        video.gl_set_swap_interval(swap_interval).expect("Failed to set swap interval!");

        Ok(Self {
            sdl: sdl,
            video: video,
            window: window,
        })
    }

    pub fn size(&self) -> (u32, u32) {
        self.window.drawable_size()
    }

    pub fn width(&self) -> u32 {
        self.window.drawable_size().0
    }

    pub fn height(&self) -> u32 {
        self.window.drawable_size().1
    }

    // pub fn back_buffer(&mut self) -> Result<Framebuffer<Flat, Dim2, (), ()>, SDL2SurfaceError> {
    //     Ok(Framebuffer::back_buffer(self, self.size_array()))
    // }

    pub fn swap_buffer(&self) {
        self.window.gl_swap_window();
    }

    pub fn set_title(&mut self, title: &str) {
        self.window.set_title(title); //TODO: Return the error?
    }
}
