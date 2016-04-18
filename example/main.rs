extern crate gl;
extern crate glutin;
extern crate openvr;

use std::os::raw::c_void;
use std::ptr;
use openvr::ffi;

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

        'main_loop: loop {
            // get poses --------------------------------------------------------------------------
            let _ = context.compositor.wait_get_poses();

            // submit each eye --------------------------------------------------------------------
            for eye in &[ffi::EVREye::Eye_Left, ffi::EVREye::Eye_Right] {
                // "render" the sceen for each eye
                let pixel =
                    if *eye == ffi::EVREye::Eye_Left {
                        0xff0000ffu32
                    } else {
                        0xff00ff00u32
                    };

                let texture_data = vec![pixel; (tex_width * tex_height) as usize];

                let mut texture = 0;

                gl::GenTextures(1, &mut texture);
                gl::BindTexture(gl::TEXTURE_2D, texture);

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

                let mut desc = ffi::Texture {
                    handle:         texture as *mut ::std::os::raw::c_void,
                    eType:          ffi::EGraphicsAPIConvention::API_OpenGL,
                    eColorSpace:    ffi::EColorSpace::Gamma
                };

                // submit to the compositor
                let error = context.compositor.submit(
                    *eye,
                    &mut desc,
                    ptr::null::<ffi::VRTextureBounds>() as *mut _,
                    ffi::EVRSubmitFlags::Submit_Default
                );

                if error != ffi::EVRCompositorError::None {
                    println!("[openvr] compositor error: {:?}", error);
                }

                gl::DeleteTextures(1, &mut texture);
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
