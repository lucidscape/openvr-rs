
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[allow(non_upper_case_globals)]
pub mod ffi;

use std::mem;
use ffi::*;
use std::ffi::CStr;

const MAX_TRACKED_DEVICE_COUNT: usize = 16;

#[link(name = "openvr_api", kind="static")]
extern "C" {
    pub fn VR_InitInternal(peError: *mut EVRInitError, eType: EVRApplicationType) -> usize;
    pub fn VR_ShutdownInternal();
    pub fn VR_IsHmdPresent() -> ::std::os::raw::c_char;
    pub fn VR_GetStringForHmdError(error: EVRInitError) -> *mut ::std::os::raw::c_char;
    pub fn VR_GetGenericInterface(pchInterfaceVersion: *const ::std::os::raw::c_char, peError: *mut EVRInitError) -> usize;
    pub fn VR_IsRuntimeInstalled() -> ::std::os::raw::c_char;
    pub fn VR_GetVRInitErrorAsSymbol(error: EVRInitError) -> *const ::std::os::raw::c_char;
    pub fn VR_GetVRInitErrorAsEnglishDescription(error: EVRInitError) -> *const ::std::os::raw::c_char;
}

pub struct VRCompositor {
    i: *mut IVRCompositor
}

impl VRCompositor {
    /// Updated scene texture to display. If bounds is NULL the entire texture will be used.
    /// https://github.com/ValveSoftware/openvr/wiki/IVRCompositor::Submit
    pub fn submit(
        &mut self,
        eye:            Eye,
        texture:        *mut Texture,
        bounds:         *mut VRTextureBounds,
        submit_flags:   EVRSubmitFlags
    ) -> EVRCompositorError {
        unsafe {
            ((*self.i).Submit)(eye.into(), texture, bounds, submit_flags)
        }
    }

    /// Returns pose(s) to use to render scene.
    /// https://github.com/ValveSoftware/openvr/wiki/IVRCompositor::WaitGetPoses
    pub fn wait_get_poses(
        &mut self,
    ) -> [TrackedDevicePose; MAX_TRACKED_DEVICE_COUNT] {
        unsafe {
            let mut render_poses = [TrackedDevicePose::default(); MAX_TRACKED_DEVICE_COUNT];
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

pub struct VRSystem {
    i: *mut IVRSystem
}

impl VRSystem {
    // fn new(vr_system: IVRSystem) -> Self {
    //     VRSystem {
    //         i:  vr_system
    //     }
    // }

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


    // /// Returns the viewport in pixels in display space that the game should render into for
    // /// the specified eye - (x, y, width, height).
    // pub fn get_eye_output_viewport(&self, eye: Eye) -> (u32, u32, u32, u32) {
    //     unsafe {
    //         let mut x = 0;
    //         let mut y = 0;
    //         let mut width = 0;
    //         let mut height = 0;
    //         self.i.GetEyeOutputViewport(
    //             eye.into(),
    //             &mut x,
    //             &mut y,
    //             &mut width,
    //             &mut height);
    //
    //         (x, y, width, height)
    //     }
    // }

    /// Returns the projection matrix to use for the specified eye.
    pub fn get_projection_matrix(
        &self,
        eye:                Eye,
        near_z:             f32,
        far_z:              f32,
        api:                EGraphicsAPIConvention
    ) -> HmdMatrix44 {
        unsafe {
            ((*self.i).GetProjectionMatrix)(
                eye.into(),
                near_z,
                far_z,
                api
            )
        }
    }

    /// Returns the transform between the view space and eye space.
    pub fn get_eye_to_head_transform(&self, eye: Eye) -> HmdMatrix34 {
        unsafe {
            ((*self.i).GetEyeToHeadTransform)(eye.into())
        }
    }

    /// Calculates updated poses for all devices.
    /// https://github.com/ValveSoftware/openvr/wiki/IVRSystem::GetDeviceToAbsoluteTrackingPose
    pub fn get_device_to_absolute_tracking_pose(
        &self,
        origin: ETrackingUniverseOrigin,
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
}

#[derive(Clone, Copy, PartialEq)]
#[repr(i32)]
pub enum Eye {
    Left = 0,
    Right = 1,
}

impl Into<EVREye> for Eye {
    fn into(self) -> EVREye {
        unsafe {
            mem::transmute(self)
        }
    }
}

#[derive(Clone, Copy)]
#[repr(i32)]
pub enum TrackedDeviceType {
    Invalid = 0,
    HMD = 1,
    Controller = 2,
    TrackingReference = 4,
    Other = 1000,
}

impl Into<TrackedDeviceClass> for TrackedDeviceType {
    fn into(self) -> TrackedDeviceClass {
        unsafe {
            mem::transmute(self)
        }
    }
}

impl std::fmt::Display for EVRInitError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        unsafe {
            let err = CStr::from_ptr(VR_GetVRInitErrorAsEnglishDescription(*self));
            write!(f, "{}", err.to_string_lossy())
        }
    }
}

impl std::error::Error for EVRInitError {
    fn description(&self) -> &str {
        "init error"
    }
}

pub struct VRContext {
    pub system:            VRSystem,
    // chaperone:         VRChaperone,
    // chaperone_setup:    IVRChaperoneSetup,
    // compositor:         IVRCompositor,
    // overlay:            IVROverlay,
    // render_models:      IVRRenderModels,
    // IVRExtendedDisplay: *mut IVRExtendedDisplay,
    // IVRSettings: *mut IVRSettings,
    // IVRApplications: *mut IVRApplications,
}

impl Drop for VRContext {
    fn drop(&mut self) {
        unsafe {
            VR_ShutdownInternal();
        }
    }
}

pub fn initialize() -> Result<VRContext, EVRInitError> {
    unsafe {
        let mut err = EVRInitError::None;
        let _hmd = VR_InitInternal(&mut err, EVRApplicationType::VRApplication_Scene);
        if err != EVRInitError::None {
            Err(err)
        } else {

            let system = VRSystem {
                i: VR_GetGenericInterface(IVRSystem_Version.as_ptr() as *const i8, &mut err) as *mut _
            };

            Ok(VRContext {
                system: system
            })
        }
    }
}

/// Returns true if the system believes that an HMD is present on the system.
pub fn is_hmd_present() -> bool {
    unsafe {
        VR_IsHmdPresent() != 0
    }
}

/// Returns true if the SteamVR runtime is installed.
pub fn is_runtime_installed() -> bool {
    unsafe {
        VR_IsRuntimeInstalled() != 0
    }
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
