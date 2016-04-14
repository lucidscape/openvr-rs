// This file is auto-generated

use std::os::raw::{c_char, c_void, c_int};
use std::mem;

pub type glSharedTextureHandle = *mut c_void;
pub type glInt = i32;
pub type glUInt = u32;
pub type TrackedDeviceIndex = u32;
pub type VRControllerState = VRControllerState001;
pub type VROverlayHandle = u64;
pub type VRComponentProperties = u32;
pub type TextureID = i32;
pub type VRNotificationId = u32;
pub type HmdError = EVRInitError;
pub type Hmd_Eye = EVREye;
pub type GraphicsAPIConvention = EGraphicsAPIConvention;
pub type ColorSpace = EColorSpace;
pub type HmdTrackingResult = ETrackingResult;
pub type TrackedDeviceClass = ETrackedDeviceClass;
pub type TrackingUniverseOrigin = ETrackingUniverseOrigin;
pub type TrackedDeviceProperty = ETrackedDeviceProperty;
pub type TrackedPropertyError = ETrackedPropertyError;
pub type VRSubmitFlags = EVRSubmitFlags;
pub type VRState = EVRState;
pub type CollisionBoundsStyle = ECollisionBoundsStyle;
pub type VROverlayError = EVROverlayError;
pub type VRFirmwareError = EVRFirmwareError;
pub type VRCompositorError = EVRCompositorError;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EVREye {
    Eye_Left = 0,
    Eye_Right = 1,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EGraphicsAPIConvention {
    API_DirectX = 0,
    API_OpenGL = 1,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EColorSpace {
    Auto = 0,
    Gamma = 1,
    Linear = 2,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ETrackingResult {
    Uninitialized = 1,
    Calibrating_InProgress = 100,
    Calibrating_OutOfRange = 101,
    Running_OK = 200,
    Running_OutOfRange = 201,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ETrackedDeviceClass {
    Invalid = 0,
    HMD = 1,
    Controller = 2,
    TrackingReference = 4,
    Other = 1000,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ETrackedControllerRole {
    Invalid = 0,
    LeftHand = 1,
    RightHand = 2,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ETrackingUniverseOrigin {
    TrackingUniverseSeated = 0,
    TrackingUniverseStanding = 1,
    TrackingUniverseRawAndUncalibrated = 2,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ETrackedDeviceProperty {
    Prop_TrackingSystemName_String = 1000,
    Prop_ModelNumber_String = 1001,
    Prop_SerialNumber_String = 1002,
    Prop_RenderModelName_String = 1003,
    Prop_WillDriftInYaw_Bool = 1004,
    Prop_ManufacturerName_String = 1005,
    Prop_TrackingFirmwareVersion_String = 1006,
    Prop_HardwareRevision_String = 1007,
    Prop_AllWirelessDongleDescriptions_String = 1008,
    Prop_ConnectedWirelessDongle_String = 1009,
    Prop_DeviceIsWireless_Bool = 1010,
    Prop_DeviceIsCharging_Bool = 1011,
    Prop_DeviceBatteryPercentage_Float = 1012,
    Prop_StatusDisplayTransform_Matrix34 = 1013,
    Prop_Firmware_UpdateAvailable_Bool = 1014,
    Prop_Firmware_ManualUpdate_Bool = 1015,
    Prop_Firmware_ManualUpdateURL_String = 1016,
    Prop_HardwareRevision_Uint64 = 1017,
    Prop_FirmwareVersion_Uint64 = 1018,
    Prop_FPGAVersion_Uint64 = 1019,
    Prop_VRCVersion_Uint64 = 1020,
    Prop_RadioVersion_Uint64 = 1021,
    Prop_DongleVersion_Uint64 = 1022,
    Prop_BlockServerShutdown_Bool = 1023,
    Prop_CanUnifyCoordinateSystemWithHmd_Bool = 1024,
    Prop_ContainsProximitySensor_Bool = 1025,
    Prop_DeviceProvidesBatteryStatus_Bool = 1026,
    Prop_DeviceCanPowerOff_Bool = 1027,
    Prop_Firmware_ProgrammingTarget_String = 1028,
    Prop_DeviceClass_Int32 = 1029,
    Prop_HasCamera_Bool = 1030,
    Prop_ReportsTimeSinceVSync_Bool = 2000,
    Prop_SecondsFromVsyncToPhotons_Float = 2001,
    Prop_DisplayFrequency_Float = 2002,
    Prop_UserIpdMeters_Float = 2003,
    Prop_CurrentUniverseId_Uint64 = 2004,
    Prop_PreviousUniverseId_Uint64 = 2005,
    Prop_DisplayFirmwareVersion_Uint64 = 2006,
    Prop_IsOnDesktop_Bool = 2007,
    Prop_DisplayMCType_Int32 = 2008,
    Prop_DisplayMCOffset_Float = 2009,
    Prop_DisplayMCScale_Float = 2010,
    Prop_EdidVendorID_Int32 = 2011,
    Prop_DisplayMCImageLeft_String = 2012,
    Prop_DisplayMCImageRight_String = 2013,
    Prop_DisplayGCBlackClamp_Float = 2014,
    Prop_EdidProductID_Int32 = 2015,
    Prop_CameraToHeadTransform_Matrix34 = 2016,
    Prop_DisplayGCType_Int32 = 2017,
    Prop_DisplayGCOffset_Float = 2018,
    Prop_DisplayGCScale_Float = 2019,
    Prop_DisplayGCPrescale_Float = 2020,
    Prop_DisplayGCImage_String = 2021,
    Prop_LensCenterLeftU_Float = 2022,
    Prop_LensCenterLeftV_Float = 2023,
    Prop_LensCenterRightU_Float = 2024,
    Prop_LensCenterRightV_Float = 2025,
    Prop_UserHeadToEyeDepthMeters_Float = 2026,
    Prop_CameraFirmwareVersion_Uint64 = 2027,
    Prop_CameraFirmwareDescription_String = 2028,
    Prop_DisplayFPGAVersion_Uint64 = 2029,
    Prop_DisplayBootloaderVersion_Uint64 = 2030,
    Prop_DisplayHardwareVersion_Uint64 = 2031,
    Prop_AudioFirmwareVersion_Uint64 = 2032,
    Prop_CameraCompatibilityMode_Int32 = 2033,
    Prop_AttachedDeviceId_String = 3000,
    Prop_SupportedButtons_Uint64 = 3001,
    Prop_Axis0Type_Int32 = 3002,
    Prop_Axis1Type_Int32 = 3003,
    Prop_Axis2Type_Int32 = 3004,
    Prop_Axis3Type_Int32 = 3005,
    Prop_Axis4Type_Int32 = 3006,
    Prop_FieldOfViewLeftDegrees_Float = 4000,
    Prop_FieldOfViewRightDegrees_Float = 4001,
    Prop_FieldOfViewTopDegrees_Float = 4002,
    Prop_FieldOfViewBottomDegrees_Float = 4003,
    Prop_TrackingRangeMinimumMeters_Float = 4004,
    Prop_TrackingRangeMaximumMeters_Float = 4005,
    Prop_ModeLabel_String = 4006,
    Prop_VendorSpecific_Reserved_Start = 10000,
    Prop_VendorSpecific_Reserved_End = 10999,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ETrackedPropertyError {
    TrackedProp_Success = 0,
    TrackedProp_WrongDataType = 1,
    TrackedProp_WrongDeviceClass = 2,
    TrackedProp_BufferTooSmall = 3,
    TrackedProp_UnknownProperty = 4,
    TrackedProp_InvalidDevice = 5,
    TrackedProp_CouldNotContactServer = 6,
    TrackedProp_ValueNotProvidedByDevice = 7,
    TrackedProp_StringExceedsMaximumLength = 8,
    TrackedProp_NotYetAvailable = 9,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EVRSubmitFlags {
    Submit_Default = 0,
    Submit_LensDistortionAlreadyApplied = 1,
    Submit_GlRenderBuffer = 2,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EVRState {
    Undefined = -1,
    Off = 0,
    Searching = 1,
    Searching_Alert = 2,
    Ready = 3,
    Ready_Alert = 4,
    NotReady = 5,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EVREventType {
    VREvent_None = 0,
    VREvent_TrackedDeviceActivated = 100,
    VREvent_TrackedDeviceDeactivated = 101,
    VREvent_TrackedDeviceUpdated = 102,
    VREvent_TrackedDeviceUserInteractionStarted = 103,
    VREvent_TrackedDeviceUserInteractionEnded = 104,
    VREvent_IpdChanged = 105,
    VREvent_EnterStandbyMode = 106,
    VREvent_LeaveStandbyMode = 107,
    VREvent_TrackedDeviceRoleChanged = 108,
    VREvent_ButtonPress = 200,
    VREvent_ButtonUnpress = 201,
    VREvent_ButtonTouch = 202,
    VREvent_ButtonUntouch = 203,
    VREvent_MouseMove = 300,
    VREvent_MouseButtonDown = 301,
    VREvent_MouseButtonUp = 302,
    VREvent_FocusEnter = 303,
    VREvent_FocusLeave = 304,
    VREvent_Scroll = 305,
    VREvent_TouchPadMove = 306,
    VREvent_InputFocusCaptured = 400,
    VREvent_InputFocusReleased = 401,
    VREvent_SceneFocusLost = 402,
    VREvent_SceneFocusGained = 403,
    VREvent_SceneApplicationChanged = 404,
    VREvent_SceneFocusChanged = 405,
    VREvent_HideRenderModels = 410,
    VREvent_ShowRenderModels = 411,
    VREvent_OverlayShown = 500,
    VREvent_OverlayHidden = 501,
    VREvent_DashboardActivated = 502,
    VREvent_DashboardDeactivated = 503,
    VREvent_DashboardThumbSelected = 504,
    VREvent_DashboardRequested = 505,
    VREvent_ResetDashboard = 506,
    VREvent_RenderToast = 507,
    VREvent_ImageLoaded = 508,
    VREvent_ShowKeyboard = 509,
    VREvent_HideKeyboard = 510,
    VREvent_OverlayGamepadFocusGained = 511,
    VREvent_OverlayGamepadFocusLost = 512,
    VREvent_OverlaySharedTextureChanged = 513,
    VREvent_Notification_Shown = 600,
    VREvent_Notification_Hidden = 601,
    VREvent_Notification_BeginInteraction = 602,
    VREvent_Notification_Destroyed = 603,
    VREvent_Quit = 700,
    VREvent_ProcessQuit = 701,
    VREvent_QuitAborted_UserPrompt = 702,
    VREvent_QuitAcknowledged = 703,
    VREvent_ChaperoneDataHasChanged = 800,
    VREvent_ChaperoneUniverseHasChanged = 801,
    VREvent_ChaperoneTempDataHasChanged = 802,
    VREvent_ChaperoneSettingsHaveChanged = 803,
    VREvent_SeatedZeroPoseReset = 804,
    VREvent_BackgroundSettingHasChanged = 850,
    VREvent_CameraSettingsHaveChanged = 851,
    VREvent_StatusUpdate = 900,
    VREvent_MCImageUpdated = 1000,
    VREvent_FirmwareUpdateStarted = 1100,
    VREvent_FirmwareUpdateFinished = 1101,
    VREvent_KeyboardClosed = 1200,
    VREvent_KeyboardCharInput = 1201,
    VREvent_KeyboardDone = 1202,
    VREvent_ApplicationTransitionStarted = 1300,
    VREvent_ApplicationTransitionAborted = 1301,
    VREvent_ApplicationTransitionNewAppStarted = 1302,
    VREvent_Compositor_MirrorWindowShown = 1400,
    VREvent_Compositor_MirrorWindowHidden = 1401,
    VREvent_Compositor_ChaperoneBoundsShown = 1410,
    VREvent_Compositor_ChaperoneBoundsHidden = 1411,
    VREvent_TrackedCamera_StartVideoStream = 1500,
    VREvent_TrackedCamera_StopVideoStream = 1501,
    VREvent_TrackedCamera_PauseVideoStream = 1502,
    VREvent_TrackedCamera_ResumeVideoStream = 1503,
    VREvent_PerformanceTest_EnableCapture = 1600,
    VREvent_PerformanceTest_DisableCapture = 1601,
    VREvent_PerformanceTest_FidelityLevel = 1602,
    VREvent_VendorSpecific_Reserved_Start = 10000,
    VREvent_VendorSpecific_Reserved_End = 19999,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EDeviceActivityLevel {
    k_EDeviceActivityLevel_Unknown = -1,
    k_EDeviceActivityLevel_Idle = 0,
    k_EDeviceActivityLevel_UserInteraction = 1,
    k_EDeviceActivityLevel_UserInteraction_Timeout = 2,
    k_EDeviceActivityLevel_Standby = 3,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EVRButtonId {
    k_EButton_System = 0,
    k_EButton_ApplicationMenu = 1,
    k_EButton_Grip = 2,
    k_EButton_DPad_Left = 3,
    k_EButton_DPad_Up = 4,
    k_EButton_DPad_Right = 5,
    k_EButton_DPad_Down = 6,
    k_EButton_A = 7,
    k_EButton_Axis0 = 32,
    k_EButton_Axis1 = 33,
    k_EButton_Axis2 = 34,
    k_EButton_Axis3 = 35,
    k_EButton_Axis4 = 36,
    k_EButton_Max = 64,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EVRMouseButton {
    Left = 1,
    Right = 2,
    Middle = 4,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EVRControllerAxisType {
    k_eControllerAxis_None = 0,
    k_eControllerAxis_TrackPad = 1,
    k_eControllerAxis_Joystick = 2,
    k_eControllerAxis_Trigger = 3,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EVRControllerEventOutputType {
    ControllerEventOutput_OSEvents = 0,
    ControllerEventOutput_VREvents = 1,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ECollisionBoundsStyle {
    COLLISION_BOUNDS_STYLE_BEGINNER = 0,
    COLLISION_BOUNDS_STYLE_INTERMEDIATE = 1,
    COLLISION_BOUNDS_STYLE_SQUARES = 2,
    COLLISION_BOUNDS_STYLE_ADVANCED = 3,
    COLLISION_BOUNDS_STYLE_NONE = 4,
    COLLISION_BOUNDS_STYLE_COUNT = 5,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EVROverlayError {
    None = 0,
    UnknownOverlay = 10,
    InvalidHandle = 11,
    PermissionDenied = 12,
    OverlayLimitExceeded = 13,
    WrongVisibilityType = 14,
    KeyTooLong = 15,
    NameTooLong = 16,
    KeyInUse = 17,
    WrongTransformType = 18,
    InvalidTrackedDevice = 19,
    InvalidParameter = 20,
    ThumbnailCantBeDestroyed = 21,
    ArrayTooSmall = 22,
    RequestFailed = 23,
    InvalidTexture = 24,
    UnableToLoadFile = 25,
    VROVerlayError_KeyboardAlreadyInUse = 26,
    NoNeighbor = 27,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EVRApplicationType {
    VRApplication_Other = 0,
    VRApplication_Scene = 1,
    VRApplication_Overlay = 2,
    VRApplication_Background = 3,
    VRApplication_Utility = 4,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EVRFirmwareError {
    None = 0,
    Success = 1,
    Fail = 2,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EVRNotificationError {
    OK = 0,
    InvalidNotificationId = 100,
    NotificationQueueFull = 101,
    InvalidOverlayHandle = 102,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EVRInitError {
    None = 0,
    Unknown = 1,
    Init_InstallationNotFound = 100,
    Init_InstallationCorrupt = 101,
    Init_VRClientDLLNotFound = 102,
    Init_FileNotFound = 103,
    Init_FactoryNotFound = 104,
    Init_InterfaceNotFound = 105,
    Init_InvalidInterface = 106,
    Init_UserConfigDirectoryInvalid = 107,
    Init_HmdNotFound = 108,
    Init_NotInitialized = 109,
    Init_PathRegistryNotFound = 110,
    Init_NoConfigPath = 111,
    Init_NoLogPath = 112,
    Init_PathRegistryNotWritable = 113,
    Init_AppInfoInitFailed = 114,
    Init_Retry = 115,
    Init_InitCanceledByUser = 116,
    Init_AnotherAppLaunching = 117,
    Init_SettingsInitFailed = 118,
    Init_ShuttingDown = 119,
    Init_TooManyObjects = 120,
    Init_NoServerForBackgroundApp = 121,
    Init_NotSupportedWithCompositor = 122,
    Init_NotAvailableToUtilityApps = 123,
    Driver_Failed = 200,
    Driver_Unknown = 201,
    Driver_HmdUnknown = 202,
    Driver_NotLoaded = 203,
    Driver_RuntimeOutOfDate = 204,
    Driver_HmdInUse = 205,
    Driver_NotCalibrated = 206,
    Driver_CalibrationInvalid = 207,
    Driver_HmdDisplayNotFound = 208,
    IPC_ServerInitFailed = 300,
    IPC_ConnectFailed = 301,
    IPC_SharedStateInitFailed = 302,
    IPC_CompositorInitFailed = 303,
    IPC_MutexInitFailed = 304,
    IPC_Failed = 305,
    Compositor_Failed = 400,
    Compositor_D3D11HardwareRequired = 401,
    VendorSpecific_UnableToConnectToOculusRuntime = 1000,
    VendorSpecific_HmdFound_CantOpenDevice = 1101,
    VendorSpecific_HmdFound_UnableToRequestConfigStart = 1102,
    VendorSpecific_HmdFound_NoStoredConfig = 1103,
    VendorSpecific_HmdFound_ConfigTooBig = 1104,
    VendorSpecific_HmdFound_ConfigTooSmall = 1105,
    VendorSpecific_HmdFound_UnableToInitZLib = 1106,
    VendorSpecific_HmdFound_CantReadFirmwareVersion = 1107,
    VendorSpecific_HmdFound_UnableToSendUserDataStart = 1108,
    VendorSpecific_HmdFound_UnableToGetUserDataStart = 1109,
    VendorSpecific_HmdFound_UnableToGetUserDataNext = 1110,
    VendorSpecific_HmdFound_UserDataAddressRange = 1111,
    VendorSpecific_HmdFound_UserDataError = 1112,
    VendorSpecific_HmdFound_ConfigFailedSanityCheck = 1113,
    Steam_SteamInstallationNotFound = 2000,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EVRApplicationError {
    None = 0,
    AppKeyAlreadyExists = 100,
    NoManifest = 101,
    NoApplication = 102,
    InvalidIndex = 103,
    UnknownApplication = 104,
    IPCFailed = 105,
    ApplicationAlreadyRunning = 106,
    InvalidManifest = 107,
    InvalidApplication = 108,
    LaunchFailed = 109,
    ApplicationAlreadyStarting = 110,
    LaunchInProgress = 111,
    OldApplicationQuitting = 112,
    TransitionAborted = 113,
    IsTemplate = 114,
    BufferTooSmall = 200,
    PropertyNotSet = 201,
    UnknownProperty = 202,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EVRApplicationProperty {
    Name_String = 0,
    LaunchType_String = 11,
    WorkingDirectory_String = 12,
    BinaryPath_String = 13,
    Arguments_String = 14,
    URL_String = 15,
    Description_String = 50,
    NewsURL_String = 51,
    ImagePath_String = 52,
    Source_String = 53,
    IsDashboardOverlay_Bool = 60,
    IsTemplate_Bool = 61,
    IsInstanced_Bool = 62,
    LastLaunchTime_Uint64 = 70,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EVRApplicationTransitionState {
    VRApplicationTransition_None = 0,
    VRApplicationTransition_OldAppQuitSent = 10,
    VRApplicationTransition_WaitingForExternalLaunch = 11,
    VRApplicationTransition_NewAppLaunched = 20,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ChaperoneCalibrationState {
    ChaperoneCalibrationState_OK = 1,
    ChaperoneCalibrationState_Warning = 100,
    ChaperoneCalibrationState_Warning_BaseStationMayHaveMoved = 101,
    ChaperoneCalibrationState_Warning_BaseStationRemoved = 102,
    ChaperoneCalibrationState_Warning_SeatedBoundsInvalid = 103,
    ChaperoneCalibrationState_Error = 200,
    ChaperoneCalibrationState_Error_BaseStationUninitalized = 201,
    ChaperoneCalibrationState_Error_BaseStationConflict = 202,
    ChaperoneCalibrationState_Error_PlayAreaInvalid = 203,
    ChaperoneCalibrationState_Error_CollisionBoundsInvalid = 204,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EChaperoneConfigFile {
    EChaperoneConfigFile_Live = 1,
    EChaperoneConfigFile_Temp = 2,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EChaperoneImportFlags {
    EChaperoneImport_BoundsOnly = 1,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EVRCompositorError {
    None = 0,
    IncompatibleVersion = 100,
    DoNotHaveFocus = 101,
    InvalidTexture = 102,
    IsNotSceneApplication = 103,
    TextureIsOnWrongDevice = 104,
    TextureUsesUnsupportedFormat = 105,
    SharedTexturesNotSupported = 106,
    IndexOutOfRange = 107,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VROverlayInputMethod {
    VROverlayInputMethod_None = 0,
    VROverlayInputMethod_Mouse = 1,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VROverlayTransformType {
    VROverlayTransform_Absolute = 0,
    VROverlayTransform_TrackedDeviceRelative = 1,
    VROverlayTransform_SystemOverlay = 2,
    VROverlayTransform_TrackedComponent = 3,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VROverlayFlags {
    VROverlayFlags_None = 0,
    VROverlayFlags_Curved = 1,
    VROverlayFlags_RGSS4X = 2,
    VROverlayFlags_NoDashboardTab = 3,
    VROverlayFlags_AcceptsGamepadEvents = 4,
    VROverlayFlags_ShowGamepadFocus = 5,
    VROverlayFlags_SendVRScrollEvents = 6,
    VROverlayFlags_SendVRTouchpadEvents = 7,
    VROverlayFlags_ShowTouchPadScrollWheel = 8,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EGamepadTextInputMode {
    k_EGamepadTextInputModeNormal = 0,
    k_EGamepadTextInputModePassword = 1,
    k_EGamepadTextInputModeSubmit = 2,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EGamepadTextInputLineMode {
    k_EGamepadTextInputLineModeSingleLine = 0,
    k_EGamepadTextInputLineModeMultipleLines = 1,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EOverlayDirection {
    Up = 0,
    Down = 1,
    Left = 2,
    Right = 3,
    Count = 4,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EVRRenderModelError {
    None = 0,
    Loading = 100,
    NotSupported = 200,
    InvalidArg = 300,
    InvalidModel = 301,
    NoShapes = 302,
    MultipleShapes = 303,
    TooManyIndices = 304,
    MultipleTextures = 305,
    InvalidTexture = 400,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EVRComponentProperty {
    IsStatic = 1,
    IsVisible = 2,
    IsTouched = 4,
    IsPressed = 8,
    IsScrolled = 16,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EVRNotificationType {
    EVRNotificationType_Transient = 0,
    EVRNotificationType_Persistent = 1,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EVRNotificationStyle {
    EVRNotificationStyle_None = 0,
    EVRNotificationStyle_Application = 100,
    EVRNotificationStyle_Contact_Disabled = 200,
    EVRNotificationStyle_Contact_Enabled = 201,
    EVRNotificationStyle_Contact_Active = 202,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EVRSettingsError {
    None = 0,
    IPCFailed = 1,
    WriteFailed = 2,
    ReadFailed = 3,
}

pub const k_unTrackingStringSize: u32 = 32;
pub const k_unMaxDriverDebugResponseSize: u32 = 32768;
pub const k_unTrackedDeviceIndex_Hmd: u32 = 0;
pub const k_unMaxTrackedDeviceCount: u32 = 16;
pub const k_unTrackedDeviceIndexInvalid: u32 = 4294967295;
pub const k_unMaxPropertyStringSize: u32 = 32768;
pub const k_unControllerStateAxisCount: u32 = 5;
pub const k_ulOverlayHandleInvalid: VROverlayHandle = 0;
pub const IVRSystem_Version: &'static str = "IVRSystem_012";
pub const IVRExtendedDisplay_Version: &'static str = "IVRExtendedDisplay_001";
pub const k_unMaxApplicationKeyLength: u32 = 128;
pub const IVRApplications_Version: &'static str = "IVRApplications_005";
pub const IVRChaperone_Version: &'static str = "IVRChaperone_003";
pub const IVRChaperoneSetup_Version: &'static str = "IVRChaperoneSetup_005";
pub const IVRCompositor_Version: &'static str = "IVRCompositor_013";
pub const k_unVROverlayMaxKeyLength: u32 = 128;
pub const k_unVROverlayMaxNameLength: u32 = 128;
pub const k_unMaxOverlayCount: u32 = 32;
pub const IVROverlay_Version: &'static str = "IVROverlay_011";
pub const k_pch_Controller_Component_GDC2015: &'static str = "gdc2015";
pub const k_pch_Controller_Component_Base: &'static str = "base";
pub const k_pch_Controller_Component_Tip: &'static str = "tip";
pub const k_pch_Controller_Component_HandGrip: &'static str = "handgrip";
pub const k_pch_Controller_Component_Status: &'static str = "status";
pub const IVRRenderModels_Version: &'static str = "IVRRenderModels_005";
pub const k_unNotificationTextMaxSize: u32 = 256;
pub const IVRNotifications_Version: &'static str = "IVRNotifications_002";
pub const k_unMaxSettingsKeyLength: u32 = 128;
pub const k_pch_SteamVR_Section: &'static str = "steamvr";
pub const k_pch_SteamVR_RequireHmd_String: &'static str = "requireHmd";
pub const k_pch_SteamVR_ForcedDriverKey_String: &'static str = "forcedDriver";
pub const k_pch_SteamVR_ForcedHmdKey_String: &'static str = "forcedHmd";
pub const k_pch_SteamVR_DisplayDebug_Bool: &'static str = "displayDebug";
pub const k_pch_SteamVR_DebugProcessPipe_String: &'static str = "debugProcessPipe";
pub const k_pch_SteamVR_EnableDistortion_Bool: &'static str = "enableDistortion";
pub const k_pch_SteamVR_DisplayDebugX_Int32: &'static str = "displayDebugX";
pub const k_pch_SteamVR_DisplayDebugY_Int32: &'static str = "displayDebugY";
pub const k_pch_SteamVR_SendSystemButtonToAllApps_Bool: &'static str = "sendSystemButtonToAllApps";
pub const k_pch_SteamVR_LogLevel_Int32: &'static str = "loglevel";
pub const k_pch_SteamVR_IPD_Float: &'static str = "ipd";
pub const k_pch_SteamVR_Background_String: &'static str = "background";
pub const k_pch_SteamVR_GridColor_String: &'static str = "gridColor";
pub const k_pch_SteamVR_PlayAreaColor_String: &'static str = "playAreaColor";
pub const k_pch_SteamVR_ActivateMultipleDrivers_Bool: &'static str = "activateMultipleDrivers";
pub const k_pch_SteamVR_PowerOffOnExit_Bool: &'static str = "powerOffOnExit";
pub const k_pch_SteamVR_StandbyAppRunningTimeout_Float: &'static str = "standbyAppRunningTimeout";
pub const k_pch_SteamVR_StandbyNoAppTimeout_Float: &'static str = "standbyNoAppTimeout";
pub const k_pch_SteamVR_DirectMode_Bool: &'static str = "directMode";
pub const k_pch_SteamVR_DirectModeEdidVid_Int32: &'static str = "directModeEdidVid";
pub const k_pch_SteamVR_DirectModeEdidPid_Int32: &'static str = "directModeEdidPid";
pub const k_pch_SteamVR_UsingSpeakers_Bool: &'static str = "usingSpeakers";
pub const k_pch_SteamVR_SpeakersForwardYawOffsetDegrees_Float: &'static str = "speakersForwardYawOffsetDegrees";
pub const k_pch_SteamVR_BaseStationPowerManagement_Bool: &'static str = "basestationPowerManagement";
pub const k_pch_SteamVR_NeverKillProcesses_Bool: &'static str = "neverKillProcesses";
pub const k_pch_Lighthouse_Section: &'static str = "driver_lighthouse";
pub const k_pch_Lighthouse_DisableIMU_Bool: &'static str = "disableimu";
pub const k_pch_Lighthouse_UseDisambiguation_String: &'static str = "usedisambiguation";
pub const k_pch_Lighthouse_DisambiguationDebug_Int32: &'static str = "disambiguationdebug";
pub const k_pch_Lighthouse_PrimaryBasestation_Int32: &'static str = "primarybasestation";
pub const k_pch_Lighthouse_LighthouseName_String: &'static str = "lighthousename";
pub const k_pch_Lighthouse_MaxIncidenceAngleDegrees_Float: &'static str = "maxincidenceangledegrees";
pub const k_pch_Lighthouse_UseLighthouseDirect_Bool: &'static str = "uselighthousedirect";
pub const k_pch_Lighthouse_DBHistory_Bool: &'static str = "dbhistory";
pub const k_pch_Lighthouse_OriginOffsetX_Float: &'static str = "originoffsetx";
pub const k_pch_Lighthouse_OriginOffsetY_Float: &'static str = "originoffsety";
pub const k_pch_Lighthouse_OriginOffsetZ_Float: &'static str = "originoffsetz";
pub const k_pch_Lighthouse_HeadingOffset_Float: &'static str = "headingoffset";
pub const k_pch_Null_Section: &'static str = "driver_null";
pub const k_pch_Null_EnableNullDriver_Bool: &'static str = "enable";
pub const k_pch_Null_SerialNumber_String: &'static str = "serialNumber";
pub const k_pch_Null_ModelNumber_String: &'static str = "modelNumber";
pub const k_pch_Null_WindowX_Int32: &'static str = "windowX";
pub const k_pch_Null_WindowY_Int32: &'static str = "windowY";
pub const k_pch_Null_WindowWidth_Int32: &'static str = "windowWidth";
pub const k_pch_Null_WindowHeight_Int32: &'static str = "windowHeight";
pub const k_pch_Null_RenderWidth_Int32: &'static str = "renderWidth";
pub const k_pch_Null_RenderHeight_Int32: &'static str = "renderHeight";
pub const k_pch_Null_SecondsFromVsyncToPhotons_Float: &'static str = "secondsFromVsyncToPhotons";
pub const k_pch_Null_DisplayFrequency_Float: &'static str = "displayFrequency";
pub const k_pch_UserInterface_Section: &'static str = "userinterface";
pub const k_pch_UserInterface_StatusAlwaysOnTop_Bool: &'static str = "StatusAlwaysOnTop";
pub const k_pch_Notifications_Section: &'static str = "notifications";
pub const k_pch_Notifications_DoNotDisturb_Bool: &'static str = "DoNotDisturb";
pub const k_pch_Keyboard_Section: &'static str = "keyboard";
pub const k_pch_Keyboard_TutorialCompletions: &'static str = "TutorialCompletions";
pub const k_pch_Perf_Section: &'static str = "perfcheck";
pub const k_pch_Perf_HeuristicActive_Bool: &'static str = "heuristicActive";
pub const k_pch_Perf_NotifyInHMD_Bool: &'static str = "warnInHMD";
pub const k_pch_Perf_NotifyOnlyOnce_Bool: &'static str = "warnOnlyOnce";
pub const k_pch_Perf_AllowTimingStore_Bool: &'static str = "allowTimingStore";
pub const k_pch_Perf_SaveTimingsOnExit_Bool: &'static str = "saveTimingsOnExit";
pub const k_pch_Perf_TestData_Float: &'static str = "perfTestData";
pub const k_pch_Camera_Section: &'static str = "camera";
pub const IVRSettings_Version: &'static str = "IVRSettings_001";
pub const k_pch_audio_Section: &'static str = "audio";
pub const k_pch_audio_OnPlaybackDevice_String: &'static str = "onPlaybackDevice";
pub const k_pch_audio_OnRecordDevice_String: &'static str = "onRecordDevice";
pub const k_pch_audio_OffPlaybackDevice_String: &'static str = "offPlaybackDevice";
pub const k_pch_audio_OffRecordDevice_String: &'static str = "offRecordDevice";
pub const k_pch_audio_VIVEHDMIGain: &'static str = "viveHDMIGain";


#[repr(C)]
#[derive(Clone, Copy)]
pub struct HmdMatrix34 {
    m: [[f32; 4]; 3],
}

impl Default for HmdMatrix34 {
    fn default() -> HmdMatrix34 {
        unsafe { mem::zeroed() }
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct HmdMatrix44 {
    m: [[f32; 4]; 4],
}

impl Default for HmdMatrix44 {
    fn default() -> HmdMatrix44 {
        unsafe { mem::zeroed() }
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct HmdVector3 {
    v: [f32; 3],
}

impl Default for HmdVector3 {
    fn default() -> HmdVector3 {
        unsafe { mem::zeroed() }
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct HmdVector4 {
    v: [f32; 4],
}

impl Default for HmdVector4 {
    fn default() -> HmdVector4 {
        unsafe { mem::zeroed() }
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct HmdVector3d {
    v: [f64; 3],
}

impl Default for HmdVector3d {
    fn default() -> HmdVector3d {
        unsafe { mem::zeroed() }
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct HmdVector2 {
    v: [f32; 2],
}

impl Default for HmdVector2 {
    fn default() -> HmdVector2 {
        unsafe { mem::zeroed() }
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct HmdQuaternion {
    w: f64,
    x: f64,
    y: f64,
    z: f64,
}

impl Default for HmdQuaternion {
    fn default() -> HmdQuaternion {
        unsafe { mem::zeroed() }
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct HmdColor {
    r: f32,
    g: f32,
    b: f32,
    a: f32,
}

impl Default for HmdColor {
    fn default() -> HmdColor {
        unsafe { mem::zeroed() }
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct HmdQuad {
    vCorners: [HmdVector3; 4],
}

impl Default for HmdQuad {
    fn default() -> HmdQuad {
        unsafe { mem::zeroed() }
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct HmdRect2 {
    vTopLeft: HmdVector2,
    vBottomRight: HmdVector2,
}

impl Default for HmdRect2 {
    fn default() -> HmdRect2 {
        unsafe { mem::zeroed() }
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct DistortionCoordinates {
    rfRed: [f32; 2],
    rfGreen: [f32; 2],
    rfBlue: [f32; 2],
}

impl Default for DistortionCoordinates {
    fn default() -> DistortionCoordinates {
        unsafe { mem::zeroed() }
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct Texture {
    handle: *mut c_void,
    eType: EGraphicsAPIConvention,
    eColorSpace: EColorSpace,
}

impl Default for Texture {
    fn default() -> Texture {
        unsafe { mem::zeroed() }
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct TrackedDevicePose {
    mDeviceToAbsoluteTracking: HmdMatrix34,
    vVelocity: HmdVector3,
    vAngularVelocity: HmdVector3,
    eTrackingResult: ETrackingResult,
    bPoseIsValid: bool,
    bDeviceIsConnected: bool,
}

impl Default for TrackedDevicePose {
    fn default() -> TrackedDevicePose {
        unsafe { mem::zeroed() }
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct VRTextureBounds {
    uMin: f32,
    vMin: f32,
    uMax: f32,
    vMax: f32,
}

impl Default for VRTextureBounds {
    fn default() -> VRTextureBounds {
        unsafe { mem::zeroed() }
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct VREvent_Controller {
    button: u32,
}

impl Default for VREvent_Controller {
    fn default() -> VREvent_Controller {
        unsafe { mem::zeroed() }
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct VREvent_Mouse {
    x: f32,
    y: f32,
    button: u32,
}

impl Default for VREvent_Mouse {
    fn default() -> VREvent_Mouse {
        unsafe { mem::zeroed() }
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct VREvent_Scroll {
    xdelta: f32,
    ydelta: f32,
    repeatCount: u32,
}

impl Default for VREvent_Scroll {
    fn default() -> VREvent_Scroll {
        unsafe { mem::zeroed() }
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct VREvent_TouchPadMove {
    bFingerDown: bool,
    flSecondsFingerDown: f32,
    fValueXFirst: f32,
    fValueYFirst: f32,
    fValueXRaw: f32,
    fValueYRaw: f32,
}

impl Default for VREvent_TouchPadMove {
    fn default() -> VREvent_TouchPadMove {
        unsafe { mem::zeroed() }
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct VREvent_Notification {
    ulUserValue: u64,
    notificationId: u32,
}

impl Default for VREvent_Notification {
    fn default() -> VREvent_Notification {
        unsafe { mem::zeroed() }
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct VREvent_Process {
    pid: u32,
    oldPid: u32,
    bForced: bool,
}

impl Default for VREvent_Process {
    fn default() -> VREvent_Process {
        unsafe { mem::zeroed() }
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct VREvent_Overlay {
    overlayHandle: u64,
}

impl Default for VREvent_Overlay {
    fn default() -> VREvent_Overlay {
        unsafe { mem::zeroed() }
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct VREvent_Status {
    statusState: u32,
}

impl Default for VREvent_Status {
    fn default() -> VREvent_Status {
        unsafe { mem::zeroed() }
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct VREvent_Keyboard {
    cNewInput: [c_char; 8],
    uUserValue: u64,
}

impl Default for VREvent_Keyboard {
    fn default() -> VREvent_Keyboard {
        unsafe { mem::zeroed() }
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct VREvent_Ipd {
    ipdMeters: f32,
}

impl Default for VREvent_Ipd {
    fn default() -> VREvent_Ipd {
        unsafe { mem::zeroed() }
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct VREvent_Chaperone {
    m_nPreviousUniverse: u64,
    m_nCurrentUniverse: u64,
}

impl Default for VREvent_Chaperone {
    fn default() -> VREvent_Chaperone {
        unsafe { mem::zeroed() }
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct VREvent_Reserved {
    reserved0: u64,
    reserved1: u64,
}

impl Default for VREvent_Reserved {
    fn default() -> VREvent_Reserved {
        unsafe { mem::zeroed() }
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct VREvent_PerformanceTest {
    m_nFidelityLevel: u32,
}

impl Default for VREvent_PerformanceTest {
    fn default() -> VREvent_PerformanceTest {
        unsafe { mem::zeroed() }
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct VREvent_SeatedZeroPoseReset {
    bResetBySystemMenu: bool,
}

impl Default for VREvent_SeatedZeroPoseReset {
    fn default() -> VREvent_SeatedZeroPoseReset {
        unsafe { mem::zeroed() }
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct VREvent_Data {
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
}

impl Default for VREvent_Data {
    fn default() -> VREvent_Data {
        unsafe { mem::zeroed() }
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct VREvent {
    eventType: u32,
    trackedDeviceIndex: TrackedDeviceIndex,
    eventAgeSeconds: f32,
    data: VREvent_Data,
}

impl Default for VREvent {
    fn default() -> VREvent {
        unsafe { mem::zeroed() }
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct HiddenAreaMesh {
    pVertexData: *mut HmdVector2,
    unTriangleCount: u32,
}

impl Default for HiddenAreaMesh {
    fn default() -> HiddenAreaMesh {
        unsafe { mem::zeroed() }
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct VRControllerAxis {
    x: f32,
    y: f32,
}

impl Default for VRControllerAxis {
    fn default() -> VRControllerAxis {
        unsafe { mem::zeroed() }
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct VRControllerState001 {
    unPacketNum: u32,
    ulButtonPressed: u64,
    ulButtonTouched: u64,
    rAxis: [VRControllerAxis; 5],
}

impl Default for VRControllerState001 {
    fn default() -> VRControllerState001 {
        unsafe { mem::zeroed() }
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct Compositor_OverlaySettings {
    size: u32,
    curved: bool,
    antialias: bool,
    scale: f32,
    distance: f32,
    alpha: f32,
    uOffset: f32,
    vOffset: f32,
    uScale: f32,
    vScale: f32,
    gridDivs: f32,
    gridWidth: f32,
    gridScale: f32,
    transform: HmdMatrix44,
}

impl Default for Compositor_OverlaySettings {
    fn default() -> Compositor_OverlaySettings {
        unsafe { mem::zeroed() }
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct AppOverrideKeys {
    pchKey: *const c_char,
    pchValue: *const c_char,
}

impl Default for AppOverrideKeys {
    fn default() -> AppOverrideKeys {
        unsafe { mem::zeroed() }
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct Compositor_FrameTiming {
    m_nSize: u32,
    m_nFrameIndex: u32,
    m_nNumFramePresents: u32,
    m_nNumDroppedFrames: u32,
    m_flSystemTimeInSeconds: f64,
    m_flSceneRenderGpuMs: f32,
    m_flTotalRenderGpuMs: f32,
    m_flCompositorRenderGpuMs: f32,
    m_flCompositorRenderCpuMs: f32,
    m_flCompositorIdleCpuMs: f32,
    m_flClientFrameIntervalMs: f32,
    m_flPresentCallCpuMs: f32,
    m_flWaitForPresentCpuMs: f32,
    m_flSubmitFrameMs: f32,
    m_flWaitGetPosesCalledMs: f32,
    m_flNewPosesReadyMs: f32,
    m_flNewFrameReadyMs: f32,
    m_flCompositorUpdateStartMs: f32,
    m_flCompositorUpdateEndMs: f32,
    m_flCompositorRenderStartMs: f32,
    m_HmdPose: TrackedDevicePose,
    m_nFidelityLevel: i32,
}

impl Default for Compositor_FrameTiming {
    fn default() -> Compositor_FrameTiming {
        unsafe { mem::zeroed() }
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct VROverlayIntersectionParams {
    vSource: HmdVector3,
    vDirection: HmdVector3,
    eOrigin: ETrackingUniverseOrigin,
}

impl Default for VROverlayIntersectionParams {
    fn default() -> VROverlayIntersectionParams {
        unsafe { mem::zeroed() }
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct VROverlayIntersectionResults {
    vPoint: HmdVector3,
    vNormal: HmdVector3,
    vUVs: HmdVector2,
    fDistance: f32,
}

impl Default for VROverlayIntersectionResults {
    fn default() -> VROverlayIntersectionResults {
        unsafe { mem::zeroed() }
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct RenderModel_ComponentState {
    mTrackingToComponentRenderModel: HmdMatrix34,
    mTrackingToComponentLocal: HmdMatrix34,
    uProperties: VRComponentProperties,
}

impl Default for RenderModel_ComponentState {
    fn default() -> RenderModel_ComponentState {
        unsafe { mem::zeroed() }
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct RenderModel_Vertex {
    vPosition: HmdVector3,
    vNormal: HmdVector3,
    rfTextureCoord: [f32; 2],
}

impl Default for RenderModel_Vertex {
    fn default() -> RenderModel_Vertex {
        unsafe { mem::zeroed() }
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct RenderModel_TextureMap {
    unWidth: u16,
    unHeight: u16,
    rubTextureMapData: *const u8,
}

impl Default for RenderModel_TextureMap {
    fn default() -> RenderModel_TextureMap {
        unsafe { mem::zeroed() }
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct RenderModel {
    rVertexData: *mut RenderModel_Vertex,
    unVertexCount: u32,
    rIndexData: *const u16,
    unTriangleCount: u32,
    diffuseTextureId: TextureID,
}

impl Default for RenderModel {
    fn default() -> RenderModel {
        unsafe { mem::zeroed() }
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct RenderModel_ControllerMode_State {
    bScrollWheelVisible: bool,
}

impl Default for RenderModel_ControllerMode_State {
    fn default() -> RenderModel_ControllerMode_State {
        unsafe { mem::zeroed() }
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct NotificationBitmap {
    bytes: *mut c_void,
    width: i32,
    height: i32,
    depth: i32,
}

impl Default for NotificationBitmap {
    fn default() -> NotificationBitmap {
        unsafe { mem::zeroed() }
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct COpenVRContext {
    m_pVRSystem: *mut IVRSystem,
    m_pVRChaperone: *mut IVRChaperone,
    m_pVRChaperoneSetup: *mut IVRChaperoneSetup,
    m_pVRCompositor: *mut IVRCompositor,
    m_pVROverlay: *mut IVROverlay,
    m_pVRRenderModels: *mut IVRRenderModels,
    m_pVRExtendedDisplay: *mut IVRExtendedDisplay,
    m_pVRSettings: *mut IVRSettings,
    m_pVRApplications: *mut IVRApplications,
}

impl Default for COpenVRContext {
    fn default() -> COpenVRContext {
        unsafe { mem::zeroed() }
    }
}

pub struct IVRApplications {
    pub AddApplicationManifest: fn(pchApplicationManifestFullPath: *const c_char, bTemporary: bool) -> EVRApplicationError,
    pub RemoveApplicationManifest: fn(pchApplicationManifestFullPath: *const c_char) -> EVRApplicationError,
    pub IsApplicationInstalled: fn(pchAppKey: *const c_char) -> bool,
    pub GetApplicationCount: fn() -> u32,
    pub GetApplicationKeyByIndex: fn(unApplicationIndex: u32, pchAppKeyBuffer: *mut c_char, unAppKeyBufferLen: u32) -> EVRApplicationError,
    pub GetApplicationKeyByProcessId: fn(unProcessId: u32, pchAppKeyBuffer: *mut c_char, unAppKeyBufferLen: u32) -> EVRApplicationError,
    pub LaunchApplication: fn(pchAppKey: *const c_char) -> EVRApplicationError,
    pub LaunchTemplateApplication: fn(pchTemplateAppKey: *const c_char, pchNewAppKey: *const c_char, pKeys: *mut AppOverrideKeys, unKeys: u32) -> EVRApplicationError,
    pub LaunchDashboardOverlay: fn(pchAppKey: *const c_char) -> EVRApplicationError,
    pub CancelApplicationLaunch: fn(pchAppKey: *const c_char) -> bool,
    pub IdentifyApplication: fn(unProcessId: u32, pchAppKey: *const c_char) -> EVRApplicationError,
    pub GetApplicationProcessId: fn(pchAppKey: *const c_char) -> u32,
    pub GetApplicationsErrorNameFromEnum: fn(error: EVRApplicationError) -> *const c_char,
    pub GetApplicationPropertyString: fn(pchAppKey: *const c_char, eProperty: EVRApplicationProperty, pchPropertyValueBuffer: *mut c_char, unPropertyValueBufferLen: u32, peError: *mut EVRApplicationError) -> u32,
    pub GetApplicationPropertyBool: fn(pchAppKey: *const c_char, eProperty: EVRApplicationProperty, peError: *mut EVRApplicationError) -> bool,
    pub GetApplicationPropertyUint64: fn(pchAppKey: *const c_char, eProperty: EVRApplicationProperty, peError: *mut EVRApplicationError) -> u64,
    pub SetApplicationAutoLaunch: fn(pchAppKey: *const c_char, bAutoLaunch: bool) -> EVRApplicationError,
    pub GetApplicationAutoLaunch: fn(pchAppKey: *const c_char) -> bool,
    pub GetStartingApplication: fn(pchAppKeyBuffer: *mut c_char, unAppKeyBufferLen: u32) -> EVRApplicationError,
    pub GetTransitionState: fn() -> EVRApplicationTransitionState,
    pub PerformApplicationPrelaunchCheck: fn(pchAppKey: *const c_char) -> EVRApplicationError,
    pub GetApplicationsTransitionStateNameFromEnum: fn(state: EVRApplicationTransitionState) -> *const c_char,
    pub IsQuitUserPromptRequested: fn() -> bool,
    pub LaunchInternalProcess: fn(pchBinaryPath: *const c_char, pchArguments: *const c_char, pchWorkingDirectory: *const c_char) -> EVRApplicationError,
}

pub struct IVRChaperone {
    pub GetCalibrationState: fn() -> ChaperoneCalibrationState,
    pub GetPlayAreaSize: fn(pSizeX: *mut f32, pSizeZ: *mut f32) -> bool,
    pub GetPlayAreaRect: fn(rect: *mut HmdQuad) -> bool,
    pub ReloadInfo: fn(),
    pub SetSceneColor: fn(color: HmdColor),
    pub GetBoundsColor: fn(pOutputColorArray: *mut HmdColor, nNumOutputColors: c_int, flCollisionBoundsFadeDistance: f32, pOutputCameraColor: *mut HmdColor),
    pub AreBoundsVisible: fn() -> bool,
    pub ForceBoundsVisible: fn(bForce: bool),
}

pub struct IVRChaperoneSetup {
    pub CommitWorkingCopy: fn(configFile: EChaperoneConfigFile) -> bool,
    pub RevertWorkingCopy: fn(),
    pub GetWorkingPlayAreaSize: fn(pSizeX: *mut f32, pSizeZ: *mut f32) -> bool,
    pub GetWorkingPlayAreaRect: fn(rect: *mut HmdQuad) -> bool,
    pub GetWorkingCollisionBoundsInfo: fn(pQuadsBuffer: *mut HmdQuad, punQuadsCount: *mut u32) -> bool,
    pub GetLiveCollisionBoundsInfo: fn(pQuadsBuffer: *mut HmdQuad, punQuadsCount: *mut u32) -> bool,
    pub GetWorkingSeatedZeroPoseToRawTrackingPose: fn(pmatSeatedZeroPoseToRawTrackingPose: *mut HmdMatrix34) -> bool,
    pub GetWorkingStandingZeroPoseToRawTrackingPose: fn(pmatStandingZeroPoseToRawTrackingPose: *mut HmdMatrix34) -> bool,
    pub SetWorkingPlayAreaSize: fn(sizeX: f32, sizeZ: f32),
    pub SetWorkingCollisionBoundsInfo: fn(pQuadsBuffer: *mut HmdQuad, unQuadsCount: u32),
    pub SetWorkingSeatedZeroPoseToRawTrackingPose: fn(pMatSeatedZeroPoseToRawTrackingPose: *mut HmdMatrix34),
    pub SetWorkingStandingZeroPoseToRawTrackingPose: fn(pMatStandingZeroPoseToRawTrackingPose: *mut HmdMatrix34),
    pub ReloadFromDisk: fn(configFile: EChaperoneConfigFile),
    pub GetLiveSeatedZeroPoseToRawTrackingPose: fn(pmatSeatedZeroPoseToRawTrackingPose: *mut HmdMatrix34) -> bool,
    pub SetWorkingCollisionBoundsTagsInfo: fn(pTagsBuffer: *mut u8, unTagCount: u32),
    pub GetLiveCollisionBoundsTagsInfo: fn(pTagsBuffer: *mut u8, punTagCount: *mut u32) -> bool,
    pub SetWorkingPhysicalBoundsInfo: fn(pQuadsBuffer: *mut HmdQuad, unQuadsCount: u32) -> bool,
    pub GetLivePhysicalBoundsInfo: fn(pQuadsBuffer: *mut HmdQuad, punQuadsCount: *mut u32) -> bool,
    pub ExportLiveToBuffer: fn(pBuffer: *mut c_char, pnBufferLength: *mut u32) -> bool,
    pub ImportFromBufferToWorking: fn(pBuffer: *const c_char, nImportFlags: u32) -> bool,
}

pub struct IVRCompositor {
    pub SetTrackingSpace: fn(eOrigin: ETrackingUniverseOrigin),
    pub GetTrackingSpace: fn() -> ETrackingUniverseOrigin,
    pub WaitGetPoses: fn(pRenderPoseArray: *mut TrackedDevicePose, unRenderPoseArrayCount: u32, pGamePoseArray: *mut TrackedDevicePose, unGamePoseArrayCount: u32) -> EVRCompositorError,
    pub GetLastPoses: fn(pRenderPoseArray: *mut TrackedDevicePose, unRenderPoseArrayCount: u32, pGamePoseArray: *mut TrackedDevicePose, unGamePoseArrayCount: u32) -> EVRCompositorError,
    pub GetLastPoseForTrackedDeviceIndex: fn(unDeviceIndex: TrackedDeviceIndex, pOutputPose: *mut TrackedDevicePose, pOutputGamePose: *mut TrackedDevicePose) -> EVRCompositorError,
    pub Submit: fn(eEye: EVREye, pTexture: *mut Texture, pBounds: *mut VRTextureBounds, nSubmitFlags: EVRSubmitFlags) -> EVRCompositorError,
    pub ClearLastSubmittedFrame: fn(),
    pub PostPresentHandoff: fn(),
    pub GetFrameTiming: fn(pTiming: *mut Compositor_FrameTiming, unFramesAgo: u32) -> bool,
    pub GetFrameTimeRemaining: fn() -> f32,
    pub FadeToColor: fn(fSeconds: f32, fRed: f32, fGreen: f32, fBlue: f32, fAlpha: f32, bBackground: bool),
    pub FadeGrid: fn(fSeconds: f32, bFadeIn: bool),
    pub SetSkyboxOverride: fn(pTextures: *mut Texture, unTextureCount: u32) -> EVRCompositorError,
    pub ClearSkyboxOverride: fn(),
    pub CompositorBringToFront: fn(),
    pub CompositorGoToBack: fn(),
    pub CompositorQuit: fn(),
    pub IsFullscreen: fn() -> bool,
    pub GetCurrentSceneFocusProcess: fn() -> u32,
    pub GetLastFrameRenderer: fn() -> u32,
    pub CanRenderScene: fn() -> bool,
    pub ShowMirrorWindow: fn(),
    pub HideMirrorWindow: fn(),
    pub IsMirrorWindowVisible: fn() -> bool,
    pub CompositorDumpImages: fn(),
    pub ShouldAppRenderWithLowResources: fn() -> bool,
    pub ForceInterleavedReprojectionOn: fn(bOverride: bool),
}

pub struct IVRExtendedDisplay {
    pub GetWindowBounds: fn(pnX: *mut i32, pnY: *mut i32, pnWidth: *mut u32, pnHeight: *mut u32),
    pub GetEyeOutputViewport: fn(eEye: EVREye, pnX: *mut u32, pnY: *mut u32, pnWidth: *mut u32, pnHeight: *mut u32),
    pub GetDXGIOutputInfo: fn(pnAdapterIndex: *mut i32, pnAdapterOutputIndex: *mut i32),
}

pub struct IVRNotifications {
    pub CreateNotification: fn(ulOverlayHandle: VROverlayHandle, ulUserValue: u64, ty: EVRNotificationType, pchText: *const c_char, style: EVRNotificationStyle, pImage: *mut NotificationBitmap, pNotificationId: *mut VRNotificationId) -> EVRNotificationError,
    pub RemoveNotification: fn(notificationId: VRNotificationId) -> EVRNotificationError,
}

pub struct IVROverlay {
    pub FindOverlay: fn(pchOverlayKey: *const c_char, pOverlayHandle: *mut VROverlayHandle) -> EVROverlayError,
    pub CreateOverlay: fn(pchOverlayKey: *const c_char, pchOverlayFriendlyName: *const c_char, pOverlayHandle: *mut VROverlayHandle) -> EVROverlayError,
    pub DestroyOverlay: fn(ulOverlayHandle: VROverlayHandle) -> EVROverlayError,
    pub SetHighQualityOverlay: fn(ulOverlayHandle: VROverlayHandle) -> EVROverlayError,
    pub GetHighQualityOverlay: fn() -> VROverlayHandle,
    pub GetOverlayKey: fn(ulOverlayHandle: VROverlayHandle, pchValue: *mut c_char, unBufferSize: u32, pError: *mut EVROverlayError) -> u32,
    pub GetOverlayName: fn(ulOverlayHandle: VROverlayHandle, pchValue: *mut c_char, unBufferSize: u32, pError: *mut EVROverlayError) -> u32,
    pub GetOverlayImageData: fn(ulOverlayHandle: VROverlayHandle, pvBuffer: *mut c_void, unBufferSize: u32, punWidth: *mut u32, punHeight: *mut u32) -> EVROverlayError,
    pub GetOverlayErrorNameFromEnum: fn(error: EVROverlayError) -> *const c_char,
    pub SetOverlayRenderingPid: fn(ulOverlayHandle: VROverlayHandle, unPID: u32) -> EVROverlayError,
    pub GetOverlayRenderingPid: fn(ulOverlayHandle: VROverlayHandle) -> u32,
    pub SetOverlayFlag: fn(ulOverlayHandle: VROverlayHandle, eOverlayFlag: VROverlayFlags, bEnabled: bool) -> EVROverlayError,
    pub GetOverlayFlag: fn(ulOverlayHandle: VROverlayHandle, eOverlayFlag: VROverlayFlags, pbEnabled: *mut bool) -> EVROverlayError,
    pub SetOverlayColor: fn(ulOverlayHandle: VROverlayHandle, fRed: f32, fGreen: f32, fBlue: f32) -> EVROverlayError,
    pub GetOverlayColor: fn(ulOverlayHandle: VROverlayHandle, pfRed: *mut f32, pfGreen: *mut f32, pfBlue: *mut f32) -> EVROverlayError,
    pub SetOverlayAlpha: fn(ulOverlayHandle: VROverlayHandle, fAlpha: f32) -> EVROverlayError,
    pub GetOverlayAlpha: fn(ulOverlayHandle: VROverlayHandle, pfAlpha: *mut f32) -> EVROverlayError,
    pub SetOverlayWidthInMeters: fn(ulOverlayHandle: VROverlayHandle, fWidthInMeters: f32) -> EVROverlayError,
    pub GetOverlayWidthInMeters: fn(ulOverlayHandle: VROverlayHandle, pfWidthInMeters: *mut f32) -> EVROverlayError,
    pub SetOverlayAutoCurveDistanceRangeInMeters: fn(ulOverlayHandle: VROverlayHandle, fMinDistanceInMeters: f32, fMaxDistanceInMeters: f32) -> EVROverlayError,
    pub GetOverlayAutoCurveDistanceRangeInMeters: fn(ulOverlayHandle: VROverlayHandle, pfMinDistanceInMeters: *mut f32, pfMaxDistanceInMeters: *mut f32) -> EVROverlayError,
    pub SetOverlayTextureColorSpace: fn(ulOverlayHandle: VROverlayHandle, eTextureColorSpace: EColorSpace) -> EVROverlayError,
    pub GetOverlayTextureColorSpace: fn(ulOverlayHandle: VROverlayHandle, peTextureColorSpace: *mut EColorSpace) -> EVROverlayError,
    pub SetOverlayTextureBounds: fn(ulOverlayHandle: VROverlayHandle, pOverlayTextureBounds: *mut VRTextureBounds) -> EVROverlayError,
    pub GetOverlayTextureBounds: fn(ulOverlayHandle: VROverlayHandle, pOverlayTextureBounds: *mut VRTextureBounds) -> EVROverlayError,
    pub GetOverlayTransformType: fn(ulOverlayHandle: VROverlayHandle, peTransformType: *mut VROverlayTransformType) -> EVROverlayError,
    pub SetOverlayTransformAbsolute: fn(ulOverlayHandle: VROverlayHandle, eTrackingOrigin: ETrackingUniverseOrigin, pmatTrackingOriginToOverlayTransform: *mut HmdMatrix34) -> EVROverlayError,
    pub GetOverlayTransformAbsolute: fn(ulOverlayHandle: VROverlayHandle, peTrackingOrigin: *mut ETrackingUniverseOrigin, pmatTrackingOriginToOverlayTransform: *mut HmdMatrix34) -> EVROverlayError,
    pub SetOverlayTransformTrackedDeviceRelative: fn(ulOverlayHandle: VROverlayHandle, unTrackedDevice: TrackedDeviceIndex, pmatTrackedDeviceToOverlayTransform: *mut HmdMatrix34) -> EVROverlayError,
    pub GetOverlayTransformTrackedDeviceRelative: fn(ulOverlayHandle: VROverlayHandle, punTrackedDevice: *mut TrackedDeviceIndex, pmatTrackedDeviceToOverlayTransform: *mut HmdMatrix34) -> EVROverlayError,
    pub SetOverlayTransformTrackedDeviceComponent: fn(ulOverlayHandle: VROverlayHandle, unDeviceIndex: TrackedDeviceIndex, pchComponentName: *const c_char) -> EVROverlayError,
    pub GetOverlayTransformTrackedDeviceComponent: fn(ulOverlayHandle: VROverlayHandle, punDeviceIndex: *mut TrackedDeviceIndex, pchComponentName: *mut c_char, unComponentNameSize: u32) -> EVROverlayError,
    pub ShowOverlay: fn(ulOverlayHandle: VROverlayHandle) -> EVROverlayError,
    pub HideOverlay: fn(ulOverlayHandle: VROverlayHandle) -> EVROverlayError,
    pub IsOverlayVisible: fn(ulOverlayHandle: VROverlayHandle) -> bool,
    pub GetTransformForOverlayCoordinates: fn(ulOverlayHandle: VROverlayHandle, eTrackingOrigin: ETrackingUniverseOrigin, coordinatesInOverlay: HmdVector2, pmatTransform: *mut HmdMatrix34) -> EVROverlayError,
    pub PollNextOverlayEvent: fn(ulOverlayHandle: VROverlayHandle, pEvent: *mut VREvent, uncbVREvent: u32) -> bool,
    pub GetOverlayInputMethod: fn(ulOverlayHandle: VROverlayHandle, peInputMethod: *mut VROverlayInputMethod) -> EVROverlayError,
    pub SetOverlayInputMethod: fn(ulOverlayHandle: VROverlayHandle, eInputMethod: VROverlayInputMethod) -> EVROverlayError,
    pub GetOverlayMouseScale: fn(ulOverlayHandle: VROverlayHandle, pvecMouseScale: *mut HmdVector2) -> EVROverlayError,
    pub SetOverlayMouseScale: fn(ulOverlayHandle: VROverlayHandle, pvecMouseScale: *mut HmdVector2) -> EVROverlayError,
    pub ComputeOverlayIntersection: fn(ulOverlayHandle: VROverlayHandle, pParams: *mut VROverlayIntersectionParams, pResults: *mut VROverlayIntersectionResults) -> bool,
    pub HandleControllerOverlayInteractionAsMouse: fn(ulOverlayHandle: VROverlayHandle, unControllerDeviceIndex: TrackedDeviceIndex) -> bool,
    pub IsHoverTargetOverlay: fn(ulOverlayHandle: VROverlayHandle) -> bool,
    pub GetGamepadFocusOverlay: fn() -> VROverlayHandle,
    pub SetGamepadFocusOverlay: fn(ulNewFocusOverlay: VROverlayHandle) -> EVROverlayError,
    pub SetOverlayNeighbor: fn(eDirection: EOverlayDirection, ulFrom: VROverlayHandle, ulTo: VROverlayHandle) -> EVROverlayError,
    pub MoveGamepadFocusToNeighbor: fn(eDirection: EOverlayDirection, ulFrom: VROverlayHandle) -> EVROverlayError,
    pub SetOverlayTexture: fn(ulOverlayHandle: VROverlayHandle, pTexture: *mut Texture) -> EVROverlayError,
    pub ClearOverlayTexture: fn(ulOverlayHandle: VROverlayHandle) -> EVROverlayError,
    pub SetOverlayRaw: fn(ulOverlayHandle: VROverlayHandle, pvBuffer: *mut c_void, unWidth: u32, unHeight: u32, unDepth: u32) -> EVROverlayError,
    pub SetOverlayFromFile: fn(ulOverlayHandle: VROverlayHandle, pchFilePath: *const c_char) -> EVROverlayError,
    pub GetOverlayTexture: fn(ulOverlayHandle: VROverlayHandle, pNativeTextureHandle: *mut *mut c_void, pNativeTextureRef: *mut c_void, pWidth: *mut u32, pHeight: *mut u32, pNativeFormat: *mut u32, pAPI: *mut EGraphicsAPIConvention, pColorSpace: *mut EColorSpace) -> EVROverlayError,
    pub ReleaseNativeOverlayHandle: fn(ulOverlayHandle: VROverlayHandle, pNativeTextureHandle: *mut c_void) -> EVROverlayError,
    pub CreateDashboardOverlay: fn(pchOverlayKey: *const c_char, pchOverlayFriendlyName: *const c_char, pMainHandle: *mut VROverlayHandle, pThumbnailHandle: *mut VROverlayHandle) -> EVROverlayError,
    pub IsDashboardVisible: fn() -> bool,
    pub IsActiveDashboardOverlay: fn(ulOverlayHandle: VROverlayHandle) -> bool,
    pub SetDashboardOverlaySceneProcess: fn(ulOverlayHandle: VROverlayHandle, unProcessId: u32) -> EVROverlayError,
    pub GetDashboardOverlaySceneProcess: fn(ulOverlayHandle: VROverlayHandle, punProcessId: *mut u32) -> EVROverlayError,
    pub ShowDashboard: fn(pchOverlayToShow: *const c_char),
    pub GetPrimaryDashboardDevice: fn() -> TrackedDeviceIndex,
    pub ShowKeyboard: fn(eInputMode: EGamepadTextInputMode, eLineInputMode: EGamepadTextInputLineMode, pchDescription: *const c_char, unCharMax: u32, pchExistingText: *const c_char, bUseMinimalMode: bool, uUserValue: u64) -> EVROverlayError,
    pub ShowKeyboardForOverlay: fn(ulOverlayHandle: VROverlayHandle, eInputMode: EGamepadTextInputMode, eLineInputMode: EGamepadTextInputLineMode, pchDescription: *const c_char, unCharMax: u32, pchExistingText: *const c_char, bUseMinimalMode: bool, uUserValue: u64) -> EVROverlayError,
    pub GetKeyboardText: fn(pchText: *mut c_char, cchText: u32) -> u32,
    pub HideKeyboard: fn(),
    pub SetKeyboardTransformAbsolute: fn(eTrackingOrigin: ETrackingUniverseOrigin, pmatTrackingOriginToKeyboardTransform: *mut HmdMatrix34),
    pub SetKeyboardPositionForOverlay: fn(ulOverlayHandle: VROverlayHandle, avoidRect: HmdRect2),
}

pub struct IVRRenderModels {
    pub LoadRenderModel_Async: fn(pchRenderModelName: *const c_char, ppRenderModel: *mut *mut RenderModel) -> EVRRenderModelError,
    pub FreeRenderModel: fn(pRenderModel: *mut RenderModel),
    pub LoadTexture_Async: fn(textureId: TextureID, ppTexture: *mut *mut RenderModel_TextureMap) -> EVRRenderModelError,
    pub FreeTexture: fn(pTexture: *mut RenderModel_TextureMap),
    pub LoadTextureD3D11_Async: fn(textureId: TextureID, pD3D11Device: *mut c_void, ppD3D11Texture2D: *mut *mut c_void) -> EVRRenderModelError,
    pub LoadIntoTextureD3D11_Async: fn(textureId: TextureID, pDstTexture: *mut c_void) -> EVRRenderModelError,
    pub FreeTextureD3D11: fn(pD3D11Texture2D: *mut c_void),
    pub GetRenderModelName: fn(unRenderModelIndex: u32, pchRenderModelName: *mut c_char, unRenderModelNameLen: u32) -> u32,
    pub GetRenderModelCount: fn() -> u32,
    pub GetComponentCount: fn(pchRenderModelName: *const c_char) -> u32,
    pub GetComponentName: fn(pchRenderModelName: *const c_char, unComponentIndex: u32, pchComponentName: *mut c_char, unComponentNameLen: u32) -> u32,
    pub GetComponentButtonMask: fn(pchRenderModelName: *const c_char, pchComponentName: *const c_char) -> u64,
    pub GetComponentRenderModelName: fn(pchRenderModelName: *const c_char, pchComponentName: *const c_char, pchComponentRenderModelName: *mut c_char, unComponentRenderModelNameLen: u32) -> u32,
    pub GetComponentState: fn(pchRenderModelName: *const c_char, pchComponentName: *const c_char, pControllerState: *mut VRControllerState, pState: *mut RenderModel_ControllerMode_State, pComponentState: *mut RenderModel_ComponentState) -> bool,
    pub RenderModelHasComponent: fn(pchRenderModelName: *const c_char, pchComponentName: *const c_char) -> bool,
}

pub struct IVRSettings {
    pub GetSettingsErrorNameFromEnum: fn(eError: EVRSettingsError) -> *const c_char,
    pub Sync: fn(bForce: bool, peError: *mut EVRSettingsError) -> bool,
    pub GetBool: fn(pchSection: *const c_char, pchSettingsKey: *const c_char, bDefaultValue: bool, peError: *mut EVRSettingsError) -> bool,
    pub SetBool: fn(pchSection: *const c_char, pchSettingsKey: *const c_char, bValue: bool, peError: *mut EVRSettingsError),
    pub GetInt32: fn(pchSection: *const c_char, pchSettingsKey: *const c_char, nDefaultValue: i32, peError: *mut EVRSettingsError) -> i32,
    pub SetInt32: fn(pchSection: *const c_char, pchSettingsKey: *const c_char, nValue: i32, peError: *mut EVRSettingsError),
    pub GetFloat: fn(pchSection: *const c_char, pchSettingsKey: *const c_char, flDefaultValue: f32, peError: *mut EVRSettingsError) -> f32,
    pub SetFloat: fn(pchSection: *const c_char, pchSettingsKey: *const c_char, flValue: f32, peError: *mut EVRSettingsError),
    pub GetString: fn(pchSection: *const c_char, pchSettingsKey: *const c_char, pchValue: *mut c_char, unValueLen: u32, pchDefaultValue: *const c_char, peError: *mut EVRSettingsError),
    pub SetString: fn(pchSection: *const c_char, pchSettingsKey: *const c_char, pchValue: *const c_char, peError: *mut EVRSettingsError),
    pub RemoveSection: fn(pchSection: *const c_char, peError: *mut EVRSettingsError),
    pub RemoveKeyInSection: fn(pchSection: *const c_char, pchSettingsKey: *const c_char, peError: *mut EVRSettingsError),
}

pub struct IVRSystem {
    pub GetRecommendedRenderTargetSize: fn(pnWidth: *mut u32, pnHeight: *mut u32),
    pub GetProjectionMatrix: fn(eEye: EVREye, fNearZ: f32, fFarZ: f32, eProjType: EGraphicsAPIConvention) -> HmdMatrix44,
    pub GetProjectionRaw: fn(eEye: EVREye, pfLeft: *mut f32, pfRight: *mut f32, pfTop: *mut f32, pfBottom: *mut f32),
    pub ComputeDistortion: fn(eEye: EVREye, fU: f32, fV: f32) -> DistortionCoordinates,
    pub GetEyeToHeadTransform: fn(eEye: EVREye) -> HmdMatrix34,
    pub GetTimeSinceLastVsync: fn(pfSecondsSinceLastVsync: *mut f32, pulFrameCounter: *mut u64) -> bool,
    pub GetD3D9AdapterIndex: fn() -> i32,
    pub GetDXGIOutputInfo: fn(pnAdapterIndex: *mut i32),
    pub IsDisplayOnDesktop: fn() -> bool,
    pub SetDisplayVisibility: fn(bIsVisibleOnDesktop: bool) -> bool,
    pub GetDeviceToAbsoluteTrackingPose: fn(eOrigin: ETrackingUniverseOrigin, fPredictedSecondsToPhotonsFromNow: f32, pTrackedDevicePoseArray: *mut TrackedDevicePose, unTrackedDevicePoseArrayCount: u32),
    pub ResetSeatedZeroPose: fn(),
    pub GetSeatedZeroPoseToStandingAbsoluteTrackingPose: fn() -> HmdMatrix34,
    pub GetRawZeroPoseToStandingAbsoluteTrackingPose: fn() -> HmdMatrix34,
    pub GetSortedTrackedDeviceIndicesOfClass: fn(eTrackedDeviceClass: ETrackedDeviceClass, punTrackedDeviceIndexArray: *mut TrackedDeviceIndex, unTrackedDeviceIndexArrayCount: u32, unRelativeToTrackedDeviceIndex: TrackedDeviceIndex) -> u32,
    pub GetTrackedDeviceActivityLevel: fn(unDeviceId: TrackedDeviceIndex) -> EDeviceActivityLevel,
    pub ApplyTransform: fn(pOutputPose: *mut TrackedDevicePose, pTrackedDevicePose: *mut TrackedDevicePose, pTransform: *mut HmdMatrix34),
    pub GetTrackedDeviceIndexForControllerRole: fn(unDeviceType: ETrackedControllerRole) -> TrackedDeviceIndex,
    pub GetControllerRoleForTrackedDeviceIndex: fn(unDeviceIndex: TrackedDeviceIndex) -> ETrackedControllerRole,
    pub GetTrackedDeviceClass: fn(unDeviceIndex: TrackedDeviceIndex) -> ETrackedDeviceClass,
    pub IsTrackedDeviceConnected: fn(unDeviceIndex: TrackedDeviceIndex) -> bool,
    pub GetBoolTrackedDeviceProperty: fn(unDeviceIndex: TrackedDeviceIndex, prop: ETrackedDeviceProperty, pError: *mut ETrackedPropertyError) -> bool,
    pub GetFloatTrackedDeviceProperty: fn(unDeviceIndex: TrackedDeviceIndex, prop: ETrackedDeviceProperty, pError: *mut ETrackedPropertyError) -> f32,
    pub GetInt32TrackedDeviceProperty: fn(unDeviceIndex: TrackedDeviceIndex, prop: ETrackedDeviceProperty, pError: *mut ETrackedPropertyError) -> i32,
    pub GetUint64TrackedDeviceProperty: fn(unDeviceIndex: TrackedDeviceIndex, prop: ETrackedDeviceProperty, pError: *mut ETrackedPropertyError) -> u64,
    pub GetMatrix34TrackedDeviceProperty: fn(unDeviceIndex: TrackedDeviceIndex, prop: ETrackedDeviceProperty, pError: *mut ETrackedPropertyError) -> HmdMatrix34,
    pub GetStringTrackedDeviceProperty: fn(unDeviceIndex: TrackedDeviceIndex, prop: ETrackedDeviceProperty, pchValue: *mut c_char, unBufferSize: u32, pError: *mut ETrackedPropertyError) -> u32,
    pub GetPropErrorNameFromEnum: fn(error: ETrackedPropertyError) -> *const c_char,
    pub PollNextEvent: fn(pEvent: *mut VREvent, uncbVREvent: u32) -> bool,
    pub PollNextEventWithPose: fn(eOrigin: ETrackingUniverseOrigin, pEvent: *mut VREvent, uncbVREvent: u32, pTrackedDevicePose: *mut TrackedDevicePose) -> bool,
    pub GetEventTypeNameFromEnum: fn(eType: EVREventType) -> *const c_char,
    pub GetHiddenAreaMesh: fn(eEye: EVREye) -> HiddenAreaMesh,
    pub GetControllerState: fn(unControllerDeviceIndex: TrackedDeviceIndex, pControllerState: *mut VRControllerState) -> bool,
    pub GetControllerStateWithPose: fn(eOrigin: ETrackingUniverseOrigin, unControllerDeviceIndex: TrackedDeviceIndex, pControllerState: *mut VRControllerState, pTrackedDevicePose: *mut TrackedDevicePose) -> bool,
    pub TriggerHapticPulse: fn(unControllerDeviceIndex: TrackedDeviceIndex, unAxisId: u32, usDurationMicroSec: u16),
    pub GetButtonIdNameFromEnum: fn(eButtonId: EVRButtonId) -> *const c_char,
    pub GetControllerAxisTypeNameFromEnum: fn(eAxisType: EVRControllerAxisType) -> *const c_char,
    pub CaptureInputFocus: fn() -> bool,
    pub ReleaseInputFocus: fn(),
    pub IsInputFocusCapturedByAnotherProcess: fn() -> bool,
    pub DriverDebugRequest: fn(unDeviceIndex: TrackedDeviceIndex, pchRequest: *const c_char, pchResponseBuffer: *mut c_char, unResponseBufferSize: u32) -> u32,
    pub PerformFirmwareUpdate: fn(unDeviceIndex: TrackedDeviceIndex) -> EVRFirmwareError,
    pub AcknowledgeQuit_Exiting: fn(),
    pub AcknowledgeQuit_UserPrompt: fn(),
}

