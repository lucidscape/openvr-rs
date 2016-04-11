#[link(name = "openvr_api", kind="static")]
extern "C" {}

#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[allow(non_upper_case_globals)]
pub mod ffi;

use std::mem;
use ffi::*;

const MAX_TRACKED_DEVICE_COUNT: usize = 16;

pub type Vector3    = (f32, f32, f32);
pub type Vector4    = (f32, f32, f32, f32);
pub type Quaternion = (f32, f32, f32, f32);
pub type Matrix44   = [[f32; 4]; 4];
pub type Matrix34   = [[f32; 4]; 3];
pub type Color      = (f32, f32, f32, f32);

pub struct VRCompositor {
    ptr:    *mut Struct_IVRCompositor
}

impl VRCompositor {
    /// Updated scene texture to display. If bounds is NULL the entire texture will be used.
    /// https://github.com/ValveSoftware/openvr/wiki/IVRCompositor::Submit
    pub fn submit(
        &mut self,
        eye:            Eye,
        texture:        *const Struct_Texture_t,
        bounds:         *const Struct_VRTextureBounds_t,
        submit_flags:   EVRSubmitFlags
    ) -> EVRCompositorError {
        unsafe {
            VR_IVRCompositor_Submit(self.ptr as i64, eye.into(), texture, bounds, submit_flags)
        }
    }

    /// Returns pose(s) to use to render scene.
    /// https://github.com/ValveSoftware/openvr/wiki/IVRCompositor::WaitGetPoses
    pub fn wait_get_poses(
        &mut self,
    ) -> [Struct_TrackedDevicePose_t; MAX_TRACKED_DEVICE_COUNT] {
        unsafe {
            let mut render_poses = [Struct_TrackedDevicePose_t::default(); MAX_TRACKED_DEVICE_COUNT];
            VR_IVRCompositor_WaitGetPoses(
                self.ptr as i64,
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
    _ptr:    *mut Struct_IVRChaperone
}

pub struct VRSystem {
    vr_system: *mut Struct_IVRSystem
}

impl VRSystem {
    fn new(vr_system: *mut Struct_IVRSystem) -> Self {
        VRSystem {
            vr_system:  vr_system
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
            VR_IVRSystem_GetRecommendedRenderTargetSize(self.vr_system as i64, &mut width, &mut height);
            (width, height)
        }
    }


    /// Returns the viewport in pixels in display space that the game should render into for
    // the specified eye - (x, y, width, height).
    pub fn get_eye_output_viewport(&self, eye: Eye) -> (u32, u32, u32, u32) {
        unsafe {
            let mut x = 0;
            let mut y = 0;
            let mut width = 0;
            let mut height = 0;
            VR_IVRExtendedDisplay_GetEyeOutputViewport(
                self.vr_system as i64,
                eye.into(),
                &mut x,
                &mut y,
                &mut width,
                &mut height);

            (x, y, width, height)
        }
    }

    /// Returns the projection matrix to use for the specified eye.
    pub fn get_projection_matrix(
        &self,
        eye:                Eye,
        near_z:             f32,
        far_z:              f32,
        api:                EGraphicsAPIConvention
    ) -> Matrix44 {
        unsafe {
            mem::transmute(
                VR_IVRSystem_GetProjectionMatrix(
                    self.vr_system as i64,
                    eye.into(),
                    near_z,
                    far_z,
                    api
                )
            )
        }
    }

    /// Returns the transform between the view space and eye space.
    pub fn get_eye_to_head_transform(&self, eye: Eye) -> Matrix34 {
        unsafe {
            mem::transmute(
                VR_IVRSystem_GetEyeToHeadTransform(
                    self.vr_system as i64,
                    eye.into(),
                )
            )
        }
    }

    /// Calculates updated poses for all devices.
    /// https://github.com/ValveSoftware/openvr/wiki/IVRSystem::GetDeviceToAbsoluteTrackingPose
    pub fn get_device_to_absolute_tracking_pose(
        &self,
        origin: ETrackingUniverseOrigin,
        predicted_seconds_to_photons_from_now:  f32,
        tracked_device_pose_array: &mut [Struct_TrackedDevicePose_t]
    ) {
        unsafe {
            VR_IVRSystem_GetDeviceToAbsoluteTrackingPose(
                self.vr_system as i64,
                origin,
                predicted_seconds_to_photons_from_now,
                tracked_device_pose_array.as_mut_ptr(),
                tracked_device_pose_array.len() as u32
            )
        }
    }
}

impl Drop for VRSystem {
    fn drop(&mut self) {
        unsafe {
            VR_ShutdownInternal();
        }
    }
}

pub struct VRContext {
    // pub compositor: VRCompositor,
    pub system:     VRSystem
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

#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(i32)]
pub enum InitError {
    None = 0,
    Unknown = 1,
    InitInstallationNotFound = 100,
    InitInstallationCorrupt = 101,
    InitVRClientDLLNotFound = 102,
    InitFileNotFound = 103,
    InitFactoryNotFound = 104,
    InitInterfaceNotFound = 105,
    InitInvalidInterface = 106,
    InitUserConfigDirectoryInvalid = 107,
    InitHmdNotFound = 108,
    InitNotInitialized = 109,
    InitPathRegistryNotFound = 110,
    InitNoConfigPath = 111,
    InitNoLogPath = 112,
    InitPathRegistryNotWritable = 113,
    InitAppInfoInitFailed = 114,
    InitRetry = 115,
    InitInitCanceledByUser = 116,
    InitAnotherAppLaunching = 117,
    InitSettingsInitFailed = 118,
    InitShuttingDown = 119,
    InitTooManyObjects = 120,
    InitNoServerForBackgroundApp = 121,
    InitNotSupportedWithCompositor = 122,
    InitNotAvailableToUtilityApps = 123,
    DriverFailed = 200,
    DriverUnknown = 201,
    DriverHmdUnknown = 202,
    DriverNotLoaded = 203,
    DriverRuntimeOutOfDate = 204,
    DriverHmdInUse = 205,
    DriverNotCalibrated = 206,
    DriverCalibrationInvalid = 207,
    DriverHmdDisplayNotFound = 208,
    IPCServerInitFailed = 300,
    IPCConnectFailed = 301,
    IPCSharedStateInitFailed = 302,
    IPCCompositorInitFailed = 303,
    IPCMutexInitFailed = 304,
    IPCFailed = 305,
    CompositorFailed = 400,
    CompositorD3D11HardwareRequired = 401,
    VendorSpecificUnableToConnectToOculusRuntime = 1000,
    VendorSpecificHmdFoundCantOpenDevice = 1101,
    VendorSpecificHmdFoundUnableToRequestConfigStart = 1102,
    VendorSpecificHmdFoundNoStoredConfig = 1103,
    VendorSpecificHmdFoundConfigTooBig = 1104,
    VendorSpecificHmdFoundConfigTooSmall = 1105,
    VendorSpecificHmdFoundUnableToInitZLib = 1106,
    VendorSpecificHmdFoundCantReadFirmwareVersion = 1107,
    VendorSpecificHmdFoundUnableToSendUserDataStart = 1108,
    VendorSpecificHmdFoundUnableToGetUserDataStart = 1109,
    VendorSpecificHmdFoundUnableToGetUserDataNext = 1110,
    VendorSpecificHmdFoundUserDataAddressRange = 1111,
    VendorSpecificHmdFoundUserDataError = 1112,
    VendorSpecificHmdFoundConfigFailedSanityCheck = 1113,
    SteamSteamInstallationNotFound = 2000,
}

impl std::fmt::Display for InitError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for InitError {
    fn description(&self) -> &str {
        match *self {
            InitError::InitPathRegistryNotFound => "Path registry not found, ensure that SteamVR is installed.",
            _ => "Unknown error"
        }
    }
}

pub fn init() -> Result<VRContext, InitError> {
    unsafe {
        let mut err = Enum_EVRInitError::EVRInitError_VRInitError_None;
        let hmd = VR_InitInternal(&mut err, Enum_EVRApplicationType::EVRApplicationType_VRApplication_Scene);
        let err: InitError = mem::transmute(err);
        if err != InitError::None {
            Err(err)
        } else {

            let system = VRSystem::new(hmd as *mut Struct_IVRSystem);

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
        match init() {
            Err(err) => {
                println!("init failed: {} - {}", err, err.description());
                assert!(false);
            }
            Ok(context) => {
                context.system.get_recommended_render_target_size();
            }
        }
    }
}
