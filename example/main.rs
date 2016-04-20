extern crate gl;
extern crate glutin;
extern crate openvr;

use std::os::raw::c_void;
use openvr::{Eye, SubmitFlags, Texture, TextureBounds, GraphicsAPIConvention, ColorSpace, CompositorError};

fn main() {
    unsafe {
        let window =
                glutin::WindowBuilder::new()
                .with_title("OpenVR Example".to_string())
                .with_gl_profile(glutin::GlProfile::Core)
                .with_dimensions(800, 600)
                .build()
                .expect("create window");

        window.make_current().expect("make current");

        gl::load_with(|symbol| window.get_proc_address(symbol) as *mut c_void);

        // init OpenVR ----------------------------------------------------------------------------
        let mut context = openvr::initialize().expect("OpenVR init");

        let (tex_width, tex_height) = context.system.get_recommended_render_target_size();

        let mut textures = [0; 2];
        gl::GenTextures(2, textures.as_mut_ptr());

        for i in 0..2 {
            gl::BindTexture(gl::TEXTURE_2D, textures[i]);

            // Not generating mips
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAX_LEVEL, 0);

            let pixel =
                if i == 0 {
                    0xff0000ffu32
                } else {
                    0xff00ff00u32
                };

            let texture_data = vec![pixel; (tex_width * tex_height) as usize];

            gl::TexImage2D(
                gl::TEXTURE_2D,
                0,
                gl::RGBA8 as i32,
                tex_width as i32,
                tex_height as i32,
                0,
                gl::RGBA,
                gl::UNSIGNED_BYTE,
                texture_data.as_ptr() as *const _
            );
        }

        'main_loop: loop {
            // get poses --------------------------------------------------------------------------
            let _poses = context.compositor.wait_get_poses();

            // submit each eye --------------------------------------------------------------------
            for eye in &[Eye::Eye_Left, Eye::Eye_Right] {
                let eye_index = *eye as usize;

                // "render" the scene for each eye


                let mut desc = Texture {
                    handle:         textures[eye_index] as *mut ::std::os::raw::c_void,
                    eType:          GraphicsAPIConvention::API_OpenGL,
                    eColorSpace:    ColorSpace::Gamma
                };

                // submit to the compositor
                let error = context.compositor.submit(
                    *eye,
                    &mut desc,
                    &mut TextureBounds {
                        uMin:   0.0,
                        vMin:   0.0,
                        uMax:   1.0,
                        vMax:   1.0,
                    },
                    SubmitFlags::Submit_Default
                );

                if error != CompositorError::None {
                    println!("[openvr] compositor error: {:?}", error);
                }
            }

            // process window events --------------------------------------------------------------
            for event in window.poll_events() {
                match event {
                    glutin::Event::KeyboardInput(glutin::ElementState::Pressed, _, Some(glutin::VirtualKeyCode::Escape))
                    | glutin::Event::Closed => break 'main_loop,
                    _ => {}
                }
            }

            match window.swap_buffers() {
                Ok(()) => (),
                Err(_) => panic!("Failed to swap buffers")
            }
        }
    }
}
