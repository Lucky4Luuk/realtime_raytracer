//NOTE: This file is meant for functions for which I have not found a home yet.
//      Most things here will move at some point!

//TODO: Dont load test data lol
// extern crate image;
use image::GenericImage;

pub fn create_texture(resolution: (i32, i32)) -> u32 {
    let mut texture = 0;

    unsafe {
        gl::GenTextures(1, &mut texture);
        gl::BindTexture(gl::TEXTURE_2D, texture);

        //tmp
        // let img = image::open(&std::path::Path::new("src/textures/cool.png")).expect("Failed to load texture");
        // let data = img.raw_pixels();

        // gl::TexImage2D(gl::TEXTURE_2D, 0, gl::RGB as i32, img.width() as i32, img.height() as i32, 0, gl::RGB, gl::UNSIGNED_BYTE, &data[0] as *const u8 as *const std::os::raw::c_void);
        gl::TexImage2D(gl::TEXTURE_2D, 0, gl::RGBA32F as i32, resolution.0, resolution.1, 0, gl::RGBA, gl::FLOAT, std::ptr::null()); //null ptr so texture is empty
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as i32); //possibly make the filtering linear?
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as i32);
        gl::BindTexture(gl::TEXTURE_2D, 0); //Unbind the texture
    }

    texture
}

pub fn create_frame_buffer(resolution: (i32, i32)) -> u32 {
    let mut fb = 0;

    unsafe {
        gl::GenFramebuffers(1, &mut fb);
        gl::BindFramebuffer(gl::FRAMEBUFFER, fb);

        let mut depth_tex = 0;
        gl::GenRenderbuffers(1, &mut depth_tex);
        gl::BindRenderbuffer(gl::RENDERBUFFER, depth_tex);
        gl::RenderbufferStorage(gl::RENDERBUFFER, gl::DEPTH_COMPONENT, resolution.0, resolution.1);

        gl::FramebufferRenderbuffer(gl::FRAMEBUFFER, gl::DEPTH_ATTACHMENT, gl::RENDERBUFFER, depth_tex);
        gl::FramebufferTexture(gl::FRAMEBUFFER, gl::COLOR_ATTACHMENT0, create_texture(resolution), 0);

        let draw_buffers = [gl::COLOR_ATTACHMENT0];
        gl::DrawBuffers(1, draw_buffers.as_ptr());

        if gl::CheckFramebufferStatus(gl::FRAMEBUFFER) != gl::FRAMEBUFFER_COMPLETE {
            panic!("aaaa framebuffer broken uwu");
        }

        gl::BindFramebuffer(gl::FRAMEBUFFER, 0);
    }

    fb
}

static quad_vertices: [f32; 12] = [
     1.0,  1.0, 0.0,
     1.0, -1.0, 0.0,
    -1.0, -1.0, 0.0,
    -1.0,  1.0, 0.0,
];

static quad_indices: [u32; 6] = [
    0,1,3,
    1,2,3,
];

pub fn create_render_quad() -> u32 {
    let mut quad_va = 0;
    let mut quad_vb = 0;
    let mut quad_ebo = 0;

    unsafe {
        //vertex array
        gl::GenVertexArrays(1, &mut quad_va);
        gl::BindVertexArray(quad_va);

        //vertex buffer
        gl::GenBuffers(1, &mut quad_vb);
        gl::BindBuffer(gl::ARRAY_BUFFER, quad_vb);
        gl::BufferData(gl::ARRAY_BUFFER, std::mem::size_of::<[f32; 12]>() as isize, quad_vertices.as_ptr() as *const std::ffi::c_void, gl::STATIC_DRAW);

        //indices
        gl::GenBuffers(1, &mut quad_ebo);
        gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, quad_ebo);
        gl::BufferData(gl::ELEMENT_ARRAY_BUFFER, std::mem::size_of::<[u32; 6]>() as isize, quad_indices.as_ptr() as *const std::ffi::c_void, gl::STATIC_DRAW);

        //vertex attribs
        gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, 3 * std::mem::size_of::<f32>() as i32, std::ptr::null());
        gl::EnableVertexAttribArray(0);

        //unbind
        gl::BindVertexArray(0);
        gl::BindBuffer(gl::ARRAY_BUFFER, 0);
    }

    quad_va
}
