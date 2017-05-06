// This file is auto-generated

use std::os::raw::{c_char, c_void, c_int};
use std::mem;

// Opaque structs
pub struct VkDevice_T(());
pub struct VkPhysicalDevice_T(());
pub struct VkInstance_T(());
pub struct VkQueue_T(());
pub struct ID3D12Resource(());
pub struct ID3D12CommandQueue(());

// Manually-maintained unions
#[derive(Clone, Copy)]
pub union VROverlayIntersectionMaskPrimitive_Data {
    rectangle: IntersectionMaskRectangle,
    circle: IntersectionMaskCircle,
}

#[derive(Clone, Copy)]
pub union VREvent_Data {
    reserved: VREvent_Reserved,
    controller: VREvent_Controller,
    mouse: VREvent_Mouse,
    scroll: VREvent_Scroll,
    process: VREvent_Process,
    notification: VREvent_Notification,
    overlay: VREvent_Overlay,
    status: VREvent_Status,
    keyboard: VREvent_Keyboard,
    ipd: VREvent_Ipd,
    chaperone: VREvent_Chaperone,
    performanceTest: VREvent_PerformanceTest,
    touchPadMove: VREvent_TouchPadMove,
    seatedZeroPoseReset: VREvent_SeatedZeroPoseReset,
    screenshot: VREvent_Screenshot,
    screenshotProgress: VREvent_ScreenshotProgress,
    applicationLaunch: VREvent_ApplicationLaunch,
    cameraSurface: VREvent_EditingCameraSurface,
    messageOverlay: VREvent_MessageOverlay,
    property: VREvent_Property,
}
