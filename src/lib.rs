extern crate libloading;
#[macro_use]
extern crate lazy_static;

#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[allow(non_upper_case_globals)]
pub mod ffi;

use std::os::raw::{c_char};
use std::ffi::{CString, CStr};
use std::ptr;
use std::mem;
use std::slice;
use libloading::Library;

use ffi::{
    IVRCompositor,
    IVRSystem,
    IVRChaperone,
    IVRRenderModels,
    IVRCompositor_FnTable,
    IVRSystem_FnTable,
    IVRRenderModels_FnTable,
    // IVRChaperone_FnTable,
};

pub use ffi::{
    Texture,
    TextureID,
    TrackedDevicePose,
    HmdMatrix34,
    HmdMatrix44,
    EVREye as Eye,
    EVRInitError as InitError,
    EVRApplicationType as ApplicationType,
    EGraphicsAPIConvention as GraphicsAPIConvention,
    EVRSubmitFlags as SubmitFlags,
    EVRCompositorError as CompositorError,
    ETrackingUniverseOrigin as TrackingUniverseOrigin,
    EColorSpace as ColorSpace,
    VRTextureBounds as TextureBounds,
    ETrackedDeviceClass as TrackedDeviceClass,
    ETrackedControllerRole as TrackedControllerRole,
    ETrackedDeviceProperty as TrackedDeviceProperty,
    ETrackedPropertyError as TrackedPropertyError,
    EVRRenderModelError as RenderModelError,
    EVRControllerAxisType as ControllerAxisType,
    VRControllerState as ControllerState,
    EVRButtonId as ButtonId,
    k_unMaxTrackedDeviceCount as MAX_TRACKED_DEVICE_COUNT
};

impl ButtonId {
    pub fn mask(self) -> u64 {
        1 << self as usize
    }
}

#[allow(non_snake_case)]
#[allow(dead_code)]
struct ApiFunctions {
    VR_InitInternal:            extern fn (peError: *mut InitError, eType: ApplicationType) -> usize,
    VR_ShutdownInternal:        extern fn (),
    VR_IsHmdPresent:            extern fn () -> ::std::os::raw::c_char,
    VR_GetStringForHmdError:    extern fn (error: InitError) -> *mut ::std::os::raw::c_char,
    VR_GetGenericInterface:     extern fn (pchInterfaceVersion: *const ::std::os::raw::c_char, peError: *mut InitError) -> usize,
    VR_IsRuntimeInstalled:      extern fn () -> ::std::os::raw::c_char,
    VR_GetVRInitErrorAsSymbol:  extern fn (error: InitError) -> *const ::std::os::raw::c_char,
    VR_GetVRInitErrorAsEnglishDescription:  extern fn (error: InitError) -> *const ::std::os::raw::c_char,
    lib: Library,
}

unsafe impl Sync for ApiFunctions {}

lazy_static! {
    static ref API: ApiFunctions = {
        let lib = Library::new("openvr_api.dll").unwrap();

        unsafe {
            ApiFunctions {
                VR_InitInternal:            *lib.get(b"VR_InitInternal\0").unwrap(),
                VR_ShutdownInternal:        *lib.get(b"VR_ShutdownInternal\0").unwrap(),
                VR_IsHmdPresent:            *lib.get(b"VR_IsHmdPresent\0").unwrap(),
                VR_GetStringForHmdError:    *lib.get(b"VR_GetStringForHmdError\0").unwrap(),
                VR_GetGenericInterface:     *lib.get(b"VR_GetGenericInterface\0").unwrap(),
                VR_IsRuntimeInstalled:      *lib.get(b"VR_IsRuntimeInstalled\0").unwrap(),
                VR_GetVRInitErrorAsSymbol:  *lib.get(b"VR_GetVRInitErrorAsSymbol\0").unwrap(),
                VR_GetVRInitErrorAsEnglishDescription: *lib.get(b"VR_GetVRInitErrorAsEnglishDescription\0").unwrap(),
                lib: lib,
            }
        }
    };
}

pub struct VRCompositor {
    i: *mut IVRCompositor
}

impl VRCompositor {
    fn new(vr_compositor: *mut IVRCompositor) -> Self {
        assert!(vr_compositor as *const _ != ptr::null());
        VRCompositor {
            i:  vr_compositor
        }
    }

    /// Updated scene texture to display. If bounds is NULL the entire texture will be used.
    /// https://github.com/ValveSoftware/openvr/wiki/IVRCompositor::Submit
    pub fn submit(
        &mut self,
        eye:            Eye,
        texture:        *mut Texture,
        bounds:         *mut TextureBounds,
        submit_flags:   SubmitFlags
    ) -> CompositorError {
        unsafe {
            ((*self.i).Submit)(eye, texture, bounds, submit_flags)
        }
    }

    /// Returns pose(s) to use to render scene.
    /// https://github.com/ValveSoftware/openvr/wiki/IVRCompositor::WaitGetPoses
    pub fn wait_get_poses(
        &mut self,
    ) -> [TrackedDevicePose; ffi::k_unMaxTrackedDeviceCount as usize] {
        unsafe {
            let mut render_poses = [TrackedDevicePose::default(); ffi::k_unMaxTrackedDeviceCount as usize];
            ((*self.i).WaitGetPoses)(
                (&mut render_poses[..]).as_mut_ptr(),
                render_poses.len() as u32,
                std::ptr::null_mut(),
                0
            );

            render_poses
        }
    }
}

pub struct VRChaperone {
    _i: *mut IVRChaperone
}

pub struct RenderModel {
    ptr: *mut ffi::RenderModel
}

impl RenderModel {
    pub fn vertices(&self) -> &[ffi::RenderModel_Vertex] {
        unsafe {
            slice::from_raw_parts((*self.ptr).rVertexData, (*self.ptr).unVertexCount as usize)
        }
    }

    pub fn indices(&self) -> &[u16] {
        unsafe {
            slice::from_raw_parts((*self.ptr).rIndexData, (*self.ptr).unTriangleCount as usize * 3)
        }
    }

    pub fn triangle_count(&self) -> usize {
        unsafe {
            (*self.ptr).unTriangleCount as usize
        }
    }

    pub fn texture_id(&self) -> TextureID {
        unsafe {
            (*self.ptr).diffuseTextureId
        }
    }
}

pub struct RenderModelTexture {
    ptr: *mut ffi::RenderModel_TextureMap
}

pub struct VRRenderModels {
    i: *mut IVRRenderModels
}

impl VRRenderModels {
    fn new(render_models: *mut IVRRenderModels) -> Self {
        assert!(render_models as *const _ != ptr::null());
        VRRenderModels {
            i:  render_models
        }
    }

    pub fn load_render_model_async(
        &mut self,
        render_model_name: String
    ) -> Result<RenderModel, RenderModelError> {
        unsafe {
            let render_model_name = CString::from_vec_unchecked(render_model_name.into());
            let mut render_model: *mut ffi::RenderModel = mem::zeroed();
            let error = ((*self.i).LoadRenderModel_Async)(render_model_name.as_ptr(), &mut render_model);
            if error == RenderModelError::None {
                Ok(RenderModel {
                    ptr:    render_model
                })
            } else {
                Err(error)
            }
        }
    }

    pub fn free_render_model(&mut self, render_model: RenderModel) {
        unsafe {
            ((*self.i).FreeRenderModel)(render_model.ptr)
        }
    }

    pub fn load_texture_async(
        &mut self,
        texture_id: TextureID
    ) -> Result<RenderModelTexture, RenderModelError> {
        unsafe {
            let texture: *mut *mut ffi::RenderModel_TextureMap = mem::zeroed();
            let error = ((*self.i).LoadTexture_Async)(texture_id, texture);
            if error == RenderModelError::None {
                Ok(RenderModelTexture {
                    ptr: *texture
                })
            } else {
                Err(error)
            }
        }
    }

    pub fn free_texture(&mut self, texture: RenderModelTexture) {
        unsafe {
            ((*self.i).FreeTexture)(texture.ptr)
        }
    }
}

pub struct VRSystem {
    i: *mut IVRSystem
}

impl VRSystem {
    fn new(vr_system: *mut IVRSystem) -> Self {
        assert!(vr_system as *const _ != ptr::null());
        VRSystem {
            i:  vr_system
        }
    }

    /// Provides the game with the minimum size that it should use for its offscreen render
    /// target to minimize pixel stretching. This size is matched with the projection matrix
    // and distortion function and will change from display to display depending on resolution,
    // distortion, and field of view.
    pub fn get_recommended_render_target_size(&self) -> (u32, u32) {
        unsafe {
            let mut width = 0;
            let mut height = 0;
            ((*self.i).GetRecommendedRenderTargetSize)(&mut width, &mut height);
            (width, height)
        }
    }

    /// Returns the projection matrix to use for the specified eye.
    pub fn get_projection_matrix(
        &self,
        eye:                Eye,
        near_z:             f32,
        far_z:              f32,
        api:                GraphicsAPIConvention
    ) -> HmdMatrix44 {
        unsafe {
            ((*self.i).GetProjectionMatrix)(
                eye,
                near_z,
                far_z,
                api
            )
        }
    }

    /// Returns the transform between the view space and eye space.
    pub fn get_eye_to_head_transform(&self, eye: Eye) -> HmdMatrix34 {
        unsafe {
            ((*self.i).GetEyeToHeadTransform)(eye)
        }
    }

    /// Calculates updated poses for all devices.
    /// https://github.com/ValveSoftware/openvr/wiki/IVRSystem::GetDeviceToAbsoluteTrackingPose
    pub fn get_device_to_absolute_tracking_pose(
        &self,
        origin: TrackingUniverseOrigin,
        predicted_seconds_to_photons_from_now:  f32,
        tracked_device_pose_array: &mut [TrackedDevicePose]
    ) {
        unsafe {
            ((*self.i).GetDeviceToAbsoluteTrackingPose)(
                origin,
                predicted_seconds_to_photons_from_now,
                tracked_device_pose_array.as_mut_ptr(),
                tracked_device_pose_array.len() as u32
            )
        }
    }

    pub fn reset_seated_zero_pose(&self) {
        unsafe {
            ((*self.i).ResetSeatedZeroPose)()
        }
    }

    pub fn get_tracked_device_class(&self, device_index: usize) -> TrackedDeviceClass {
        unsafe {
            ((*self.i).GetTrackedDeviceClass)(device_index as ffi::TrackedDeviceIndex)
        }
    }

    pub fn get_controller_role_for_tracked_device_index(&self, device_index: usize) -> TrackedControllerRole {
        unsafe {
            ((*self.i).GetControllerRoleForTrackedDeviceIndex)(device_index as ffi::TrackedDeviceIndex)
        }
    }

    pub fn get_controller_state(&self, device_index: usize) -> ControllerState {
        unsafe {
            let mut state = ControllerState::default();
            ((*self.i).GetControllerState)(device_index as ffi::TrackedDeviceIndex, &mut state);
            state
        }
    }

    pub fn get_string_tracked_device_property(
        &self,
        device_index:   usize,
        prop:           TrackedDeviceProperty
    ) -> Result<String, TrackedPropertyError> {
        unsafe {
            let mut error = TrackedPropertyError::TrackedProp_Success;

            let buffer_length = ((*self.i).GetStringTrackedDeviceProperty)(
                device_index as ffi::TrackedDeviceIndex,
                prop,
                ptr::null::<c_char>() as *mut _,
                0,
                &mut error
            );

            assert!(buffer_length > 0);

            let mut str_data = Vec::<u8>::with_capacity(buffer_length as usize);

            ((*self.i).GetStringTrackedDeviceProperty)(
                device_index as ffi::TrackedDeviceIndex,
                prop,
                str_data.as_mut_ptr() as *mut _,
                buffer_length,
                &mut error
            );

            str_data.set_len(buffer_length as usize - 1);

            if error == TrackedPropertyError::TrackedProp_Success {
                Ok(String::from_utf8_unchecked(str_data))
            } else {
                Err(error)
            }
        }
    }
}

impl std::fmt::Display for InitError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        unsafe {
            let err = CStr::from_ptr((API.VR_GetVRInitErrorAsEnglishDescription)(*self));
            write!(f, "{}", err.to_string_lossy())
        }
    }
}

impl std::error::Error for InitError {
    fn description(&self) -> &str {
        "init error"
    }
}

pub struct VRContext {
    pub system:         VRSystem,
    pub compositor:     VRCompositor,
    pub render_models:  VRRenderModels

    // chaperone:         VRChaperone,
    // chaperone_setup:    IVRChaperoneSetup,
    // overlay:            IVROverlay,
    // IVRExtendedDisplay: *mut IVRExtendedDisplay,
    // IVRSettings: *mut IVRSettings,
    // IVRApplications: *mut IVRApplications,
}

impl Drop for VRContext {
    fn drop(&mut self) {
        // VR_ShutdownInternal();
    }
}

pub fn initialize() -> Result<VRContext, InitError> {
    let mut err = InitError::None;
    let _hmd = (API.VR_InitInternal)(&mut err, ApplicationType::VRApplication_Scene);
    if err != InitError::None {
        Err(err)
    } else {

        let system = (API.VR_GetGenericInterface)(CString::new(IVRSystem_FnTable).unwrap().as_ptr(), &mut err) as *mut IVRSystem;
        if err != InitError::None {
            return Err(err);
        }
        let system = VRSystem::new(system);

        let compositor = (API.VR_GetGenericInterface)(CString::new(IVRCompositor_FnTable).unwrap().as_ptr(), &mut err) as *mut IVRCompositor;
        if err != InitError::None {
            return Err(err);
        }
        let compositor = VRCompositor::new(compositor);


        let render_models = (API.VR_GetGenericInterface)(CString::new(IVRRenderModels_FnTable).unwrap().as_ptr(), &mut err) as *mut IVRRenderModels;
        if err != InitError::None {
            return Err(err);
        }
        let render_models = VRRenderModels::new(render_models);


        Ok(VRContext {
            system:         system,
            compositor:     compositor,
            render_models:  render_models
        })
    }
}

/// Returns true if the system believes that an HMD is present on the system.
pub fn is_hmd_present() -> bool {
    (API.VR_IsHmdPresent)() != 0
}

/// Returns true if the SteamVR runtime is installed.
pub fn is_runtime_installed() -> bool {
    (API.VR_IsRuntimeInstalled)() != 0
}

#[cfg(test)]
mod prerequisites {
    use super::*;

    #[test]
    fn runtime_installed() {
        assert!(is_runtime_installed());
    }

    #[test]
    fn hmd_present() {
        assert!(is_hmd_present());
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::error::Error;

    #[test]
    fn vr_init() {
        // NOTE: initializing seems to fail if a window has not been created

        // match init() {
        //     Err(err) => {
        //         println!("init failed: {} - {}", err, err.description());
        //         assert!(false);
        //     }
        //     Ok(context) => {
        //         context.system.get_recommended_render_target_size();
        //     }
        // }
    }
}
