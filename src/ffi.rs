// This file is auto-generated

struct IVRApplications {
    ptr: *mut IVRApplications
}

impl IVRApplications {
    unsafe fn AddApplicationManifest(&self, pchApplicationManifestFullPath: *const c_char, bTemporary: bool) {
        AddApplicationManifest(self.ptr, pchApplicationManifestFullPath, bTemporary)
    }

    unsafe fn RemoveApplicationManifest(&self, pchApplicationManifestFullPath: *const c_char) {
        RemoveApplicationManifest(self.ptr, pchApplicationManifestFullPath)
    }

    unsafe fn IsApplicationInstalled(&self, pchAppKey: *const c_char) {
        IsApplicationInstalled(self.ptr, pchAppKey)
    }

    unsafe fn GetApplicationCount(&self, ) {
        GetApplicationCount(self.ptr, )
    }

    unsafe fn GetApplicationKeyByIndex(&self, unApplicationIndex: u32, pchAppKeyBuffer: *mut c_char, unAppKeyBufferLen: u32) {
        GetApplicationKeyByIndex(self.ptr, unApplicationIndex, pchAppKeyBuffer, unAppKeyBufferLen)
    }

    unsafe fn GetApplicationKeyByProcessId(&self, unProcessId: u32, pchAppKeyBuffer: *mut c_char, unAppKeyBufferLen: u32) {
        GetApplicationKeyByProcessId(self.ptr, unProcessId, pchAppKeyBuffer, unAppKeyBufferLen)
    }

    unsafe fn LaunchApplication(&self, pchAppKey: *const c_char) {
        LaunchApplication(self.ptr, pchAppKey)
    }

    unsafe fn LaunchTemplateApplication(&self, pchTemplateAppKey: *const c_char, pchNewAppKey: *const c_char, pKeys: *mut  AppOverrideKeys_t, unKeys: u32) {
        LaunchTemplateApplication(self.ptr, pchTemplateAppKey, pchNewAppKey, pKeys, unKeys)
    }

    unsafe fn LaunchDashboardOverlay(&self, pchAppKey: *const c_char) {
        LaunchDashboardOverlay(self.ptr, pchAppKey)
    }

    unsafe fn CancelApplicationLaunch(&self, pchAppKey: *const c_char) {
        CancelApplicationLaunch(self.ptr, pchAppKey)
    }

    unsafe fn IdentifyApplication(&self, unProcessId: u32, pchAppKey: *const c_char) {
        IdentifyApplication(self.ptr, unProcessId, pchAppKey)
    }

    unsafe fn GetApplicationProcessId(&self, pchAppKey: *const c_char) {
        GetApplicationProcessId(self.ptr, pchAppKey)
    }

    unsafe fn GetApplicationsErrorNameFromEnum(&self, error: EVRApplicationError) {
        GetApplicationsErrorNameFromEnum(self.ptr, error)
    }

    unsafe fn GetApplicationPropertyString(&self, pchAppKey: *const c_char, eProperty: EVRApplicationProperty, pchPropertyValueBuffer: *mut c_char, unPropertyValueBufferLen: u32, peError: *mut EVRApplicationError) {
        GetApplicationPropertyString(self.ptr, pchAppKey, eProperty, pchPropertyValueBuffer, unPropertyValueBufferLen, peError)
    }

    unsafe fn GetApplicationPropertyBool(&self, pchAppKey: *const c_char, eProperty: EVRApplicationProperty, peError: *mut EVRApplicationError) {
        GetApplicationPropertyBool(self.ptr, pchAppKey, eProperty, peError)
    }

    unsafe fn GetApplicationPropertyUint64(&self, pchAppKey: *const c_char, eProperty: EVRApplicationProperty, peError: *mut EVRApplicationError) {
        GetApplicationPropertyUint64(self.ptr, pchAppKey, eProperty, peError)
    }

    unsafe fn SetApplicationAutoLaunch(&self, pchAppKey: *const c_char, bAutoLaunch: bool) {
        SetApplicationAutoLaunch(self.ptr, pchAppKey, bAutoLaunch)
    }

    unsafe fn GetApplicationAutoLaunch(&self, pchAppKey: *const c_char) {
        GetApplicationAutoLaunch(self.ptr, pchAppKey)
    }

    unsafe fn GetStartingApplication(&self, pchAppKeyBuffer: *mut c_char, unAppKeyBufferLen: u32) {
        GetStartingApplication(self.ptr, pchAppKeyBuffer, unAppKeyBufferLen)
    }

    unsafe fn GetTransitionState(&self, ) {
        GetTransitionState(self.ptr, )
    }

    unsafe fn PerformApplicationPrelaunchCheck(&self, pchAppKey: *const c_char) {
        PerformApplicationPrelaunchCheck(self.ptr, pchAppKey)
    }

    unsafe fn GetApplicationsTransitionStateNameFromEnum(&self, state: EVRApplicationTransitionState) {
        GetApplicationsTransitionStateNameFromEnum(self.ptr, state)
    }

    unsafe fn IsQuitUserPromptRequested(&self, ) {
        IsQuitUserPromptRequested(self.ptr, )
    }

    unsafe fn LaunchInternalProcess(&self, pchBinaryPath: *const c_char, pchArguments: *const c_char, pchWorkingDirectory: *const c_char) {
        LaunchInternalProcess(self.ptr, pchBinaryPath, pchArguments, pchWorkingDirectory)
    }

}

struct IVRChaperone {
    ptr: *mut IVRChaperone
}

impl IVRChaperone {
    unsafe fn GetCalibrationState(&self, ) {
        GetCalibrationState(self.ptr, )
    }

    unsafe fn GetPlayAreaSize(&self, pSizeX: *mut f32, pSizeZ: *mut f32) {
        GetPlayAreaSize(self.ptr, pSizeX, pSizeZ)
    }

    unsafe fn GetPlayAreaRect(&self, rect: *mut HmdQuad_t) {
        GetPlayAreaRect(self.ptr, rect)
    }

    unsafe fn ReloadInfo(&self, ) {
        ReloadInfo(self.ptr, )
    }

    unsafe fn SetSceneColor(&self, color: HmdColor_t) {
        SetSceneColor(self.ptr, color)
    }

    unsafe fn GetBoundsColor(&self, pOutputColorArray: *mut HmdColor_t, nNumOutputColors: int, flCollisionBoundsFadeDistance: f32, pOutputCameraColor: *mut HmdColor_t) {
        GetBoundsColor(self.ptr, pOutputColorArray, nNumOutputColors, flCollisionBoundsFadeDistance, pOutputCameraColor)
    }

    unsafe fn AreBoundsVisible(&self, ) {
        AreBoundsVisible(self.ptr, )
    }

    unsafe fn ForceBoundsVisible(&self, bForce: bool) {
        ForceBoundsVisible(self.ptr, bForce)
    }

}

struct IVRChaperoneSetup {
    ptr: *mut IVRChaperoneSetup
}

impl IVRChaperoneSetup {
    unsafe fn CommitWorkingCopy(&self, configFile: EChaperoneConfigFile) {
        CommitWorkingCopy(self.ptr, configFile)
    }

    unsafe fn RevertWorkingCopy(&self, ) {
        RevertWorkingCopy(self.ptr, )
    }

    unsafe fn GetWorkingPlayAreaSize(&self, pSizeX: *mut f32, pSizeZ: *mut f32) {
        GetWorkingPlayAreaSize(self.ptr, pSizeX, pSizeZ)
    }

    unsafe fn GetWorkingPlayAreaRect(&self, rect: *mut HmdQuad_t) {
        GetWorkingPlayAreaRect(self.ptr, rect)
    }

    unsafe fn GetWorkingCollisionBoundsInfo(&self, pQuadsBuffer: *mut HmdQuad_t, punQuadsCount: *mut u32) {
        GetWorkingCollisionBoundsInfo(self.ptr, pQuadsBuffer, punQuadsCount)
    }

    unsafe fn GetLiveCollisionBoundsInfo(&self, pQuadsBuffer: *mut HmdQuad_t, punQuadsCount: *mut u32) {
        GetLiveCollisionBoundsInfo(self.ptr, pQuadsBuffer, punQuadsCount)
    }

    unsafe fn GetWorkingSeatedZeroPoseToRawTrackingPose(&self, pmatSeatedZeroPoseToRawTrackingPose: *mut HmdMatrix34_t) {
        GetWorkingSeatedZeroPoseToRawTrackingPose(self.ptr, pmatSeatedZeroPoseToRawTrackingPose)
    }

    unsafe fn GetWorkingStandingZeroPoseToRawTrackingPose(&self, pmatStandingZeroPoseToRawTrackingPose: *mut HmdMatrix34_t) {
        GetWorkingStandingZeroPoseToRawTrackingPose(self.ptr, pmatStandingZeroPoseToRawTrackingPose)
    }

    unsafe fn SetWorkingPlayAreaSize(&self, sizeX: f32, sizeZ: f32) {
        SetWorkingPlayAreaSize(self.ptr, sizeX, sizeZ)
    }

    unsafe fn SetWorkingCollisionBoundsInfo(&self, pQuadsBuffer: *mut HmdQuad_t, unQuadsCount: u32) {
        SetWorkingCollisionBoundsInfo(self.ptr, pQuadsBuffer, unQuadsCount)
    }

    unsafe fn SetWorkingSeatedZeroPoseToRawTrackingPose(&self, pMatSeatedZeroPoseToRawTrackingPose: *mut  HmdMatrix34_t) {
        SetWorkingSeatedZeroPoseToRawTrackingPose(self.ptr, pMatSeatedZeroPoseToRawTrackingPose)
    }

    unsafe fn SetWorkingStandingZeroPoseToRawTrackingPose(&self, pMatStandingZeroPoseToRawTrackingPose: *mut  HmdMatrix34_t) {
        SetWorkingStandingZeroPoseToRawTrackingPose(self.ptr, pMatStandingZeroPoseToRawTrackingPose)
    }

    unsafe fn ReloadFromDisk(&self, configFile: EChaperoneConfigFile) {
        ReloadFromDisk(self.ptr, configFile)
    }

    unsafe fn GetLiveSeatedZeroPoseToRawTrackingPose(&self, pmatSeatedZeroPoseToRawTrackingPose: *mut HmdMatrix34_t) {
        GetLiveSeatedZeroPoseToRawTrackingPose(self.ptr, pmatSeatedZeroPoseToRawTrackingPose)
    }

    unsafe fn SetWorkingCollisionBoundsTagsInfo(&self, pTagsBuffer: *mut u8, unTagCount: u32) {
        SetWorkingCollisionBoundsTagsInfo(self.ptr, pTagsBuffer, unTagCount)
    }

    unsafe fn GetLiveCollisionBoundsTagsInfo(&self, pTagsBuffer: *mut u8, punTagCount: *mut u32) {
        GetLiveCollisionBoundsTagsInfo(self.ptr, pTagsBuffer, punTagCount)
    }

    unsafe fn SetWorkingPhysicalBoundsInfo(&self, pQuadsBuffer: *mut HmdQuad_t, unQuadsCount: u32) {
        SetWorkingPhysicalBoundsInfo(self.ptr, pQuadsBuffer, unQuadsCount)
    }

    unsafe fn GetLivePhysicalBoundsInfo(&self, pQuadsBuffer: *mut HmdQuad_t, punQuadsCount: *mut u32) {
        GetLivePhysicalBoundsInfo(self.ptr, pQuadsBuffer, punQuadsCount)
    }

    unsafe fn ExportLiveToBuffer(&self, pBuffer: *mut c_char, pnBufferLength: *mut u32) {
        ExportLiveToBuffer(self.ptr, pBuffer, pnBufferLength)
    }

    unsafe fn ImportFromBufferToWorking(&self, pBuffer: *const c_char, nImportFlags: u32) {
        ImportFromBufferToWorking(self.ptr, pBuffer, nImportFlags)
    }

}

struct IVRCompositor {
    ptr: *mut IVRCompositor
}

impl IVRCompositor {
    unsafe fn SetTrackingSpace(&self, eOrigin: ETrackingUniverseOrigin) {
        SetTrackingSpace(self.ptr, eOrigin)
    }

    unsafe fn GetTrackingSpace(&self, ) {
        GetTrackingSpace(self.ptr, )
    }

    unsafe fn WaitGetPoses(&self, pRenderPoseArray: *mut TrackedDevicePose_t, unRenderPoseArrayCount: u32, pGamePoseArray: *mut TrackedDevicePose_t, unGamePoseArrayCount: u32) {
        WaitGetPoses(self.ptr, pRenderPoseArray, unRenderPoseArrayCount, pGamePoseArray, unGamePoseArrayCount)
    }

    unsafe fn GetLastPoses(&self, pRenderPoseArray: *mut TrackedDevicePose_t, unRenderPoseArrayCount: u32, pGamePoseArray: *mut TrackedDevicePose_t, unGamePoseArrayCount: u32) {
        GetLastPoses(self.ptr, pRenderPoseArray, unRenderPoseArrayCount, pGamePoseArray, unGamePoseArrayCount)
    }

    unsafe fn GetLastPoseForTrackedDeviceIndex(&self, unDeviceIndex: TrackedDeviceIndex_t, pOutputPose: *mut TrackedDevicePose_t, pOutputGamePose: *mut TrackedDevicePose_t) {
        GetLastPoseForTrackedDeviceIndex(self.ptr, unDeviceIndex, pOutputPose, pOutputGamePose)
    }

    unsafe fn Submit(&self, eEye: EVREye, pTexture: *mut  Texture_t, pBounds: *mut  VRTextureBounds_t, nSubmitFlags: EVRSubmitFlags) {
        Submit(self.ptr, eEye, pTexture, pBounds, nSubmitFlags)
    }

    unsafe fn ClearLastSubmittedFrame(&self, ) {
        ClearLastSubmittedFrame(self.ptr, )
    }

    unsafe fn PostPresentHandoff(&self, ) {
        PostPresentHandoff(self.ptr, )
    }

    unsafe fn GetFrameTiming(&self, pTiming: *mut Compositor_FrameTiming, unFramesAgo: u32) {
        GetFrameTiming(self.ptr, pTiming, unFramesAgo)
    }

    unsafe fn GetFrameTimeRemaining(&self, ) {
        GetFrameTimeRemaining(self.ptr, )
    }

    unsafe fn FadeToColor(&self, fSeconds: f32, fRed: f32, fGreen: f32, fBlue: f32, fAlpha: f32, bBackground: bool) {
        FadeToColor(self.ptr, fSeconds, fRed, fGreen, fBlue, fAlpha, bBackground)
    }

    unsafe fn FadeGrid(&self, fSeconds: f32, bFadeIn: bool) {
        FadeGrid(self.ptr, fSeconds, bFadeIn)
    }

    unsafe fn SetSkyboxOverride(&self, pTextures: *mut  Texture_t, unTextureCount: u32) {
        SetSkyboxOverride(self.ptr, pTextures, unTextureCount)
    }

    unsafe fn ClearSkyboxOverride(&self, ) {
        ClearSkyboxOverride(self.ptr, )
    }

    unsafe fn CompositorBringToFront(&self, ) {
        CompositorBringToFront(self.ptr, )
    }

    unsafe fn CompositorGoToBack(&self, ) {
        CompositorGoToBack(self.ptr, )
    }

    unsafe fn CompositorQuit(&self, ) {
        CompositorQuit(self.ptr, )
    }

    unsafe fn IsFullscreen(&self, ) {
        IsFullscreen(self.ptr, )
    }

    unsafe fn GetCurrentSceneFocusProcess(&self, ) {
        GetCurrentSceneFocusProcess(self.ptr, )
    }

    unsafe fn GetLastFrameRenderer(&self, ) {
        GetLastFrameRenderer(self.ptr, )
    }

    unsafe fn CanRenderScene(&self, ) {
        CanRenderScene(self.ptr, )
    }

    unsafe fn ShowMirrorWindow(&self, ) {
        ShowMirrorWindow(self.ptr, )
    }

    unsafe fn HideMirrorWindow(&self, ) {
        HideMirrorWindow(self.ptr, )
    }

    unsafe fn IsMirrorWindowVisible(&self, ) {
        IsMirrorWindowVisible(self.ptr, )
    }

    unsafe fn CompositorDumpImages(&self, ) {
        CompositorDumpImages(self.ptr, )
    }

    unsafe fn ShouldAppRenderWithLowResources(&self, ) {
        ShouldAppRenderWithLowResources(self.ptr, )
    }

    unsafe fn ForceInterleavedReprojectionOn(&self, bOverride: bool) {
        ForceInterleavedReprojectionOn(self.ptr, bOverride)
    }

}

struct IVRExtendedDisplay {
    ptr: *mut IVRExtendedDisplay
}

impl IVRExtendedDisplay {
    unsafe fn GetWindowBounds(&self, pnX: *mut i32, pnY: *mut i32, pnWidth: *mut u32, pnHeight: *mut u32) {
        GetWindowBounds(self.ptr, pnX, pnY, pnWidth, pnHeight)
    }

    unsafe fn GetEyeOutputViewport(&self, eEye: EVREye, pnX: *mut u32, pnY: *mut u32, pnWidth: *mut u32, pnHeight: *mut u32) {
        GetEyeOutputViewport(self.ptr, eEye, pnX, pnY, pnWidth, pnHeight)
    }

    unsafe fn GetDXGIOutputInfo(&self, pnAdapterIndex: *mut i32, pnAdapterOutputIndex: *mut i32) {
        GetDXGIOutputInfo(self.ptr, pnAdapterIndex, pnAdapterOutputIndex)
    }

}

struct IVRNotifications {
    ptr: *mut IVRNotifications
}

impl IVRNotifications {
    unsafe fn CreateNotification(&self, ulOverlayHandle: VROverlayHandle_t, ulUserValue: uint64_t, ty: EVRNotificationType, pchText: *const c_char, style: EVRNotificationStyle, pImage: *mut  NotificationBitmap_t, pNotificationId: *mut VRNotificationId) {
        CreateNotification(self.ptr, ulOverlayHandle, ulUserValue, ty, pchText, style, pImage, pNotificationId)
    }

    unsafe fn RemoveNotification(&self, notificationId: VRNotificationId) {
        RemoveNotification(self.ptr, notificationId)
    }

}

struct IVROverlay {
    ptr: *mut IVROverlay
}

impl IVROverlay {
    unsafe fn FindOverlay(&self, pchOverlayKey: *const c_char, pOverlayHandle: *mut VROverlayHandle_t) {
        FindOverlay(self.ptr, pchOverlayKey, pOverlayHandle)
    }

    unsafe fn CreateOverlay(&self, pchOverlayKey: *const c_char, pchOverlayFriendlyName: *const c_char, pOverlayHandle: *mut VROverlayHandle_t) {
        CreateOverlay(self.ptr, pchOverlayKey, pchOverlayFriendlyName, pOverlayHandle)
    }

    unsafe fn DestroyOverlay(&self, ulOverlayHandle: VROverlayHandle_t) {
        DestroyOverlay(self.ptr, ulOverlayHandle)
    }

    unsafe fn SetHighQualityOverlay(&self, ulOverlayHandle: VROverlayHandle_t) {
        SetHighQualityOverlay(self.ptr, ulOverlayHandle)
    }

    unsafe fn GetHighQualityOverlay(&self, ) {
        GetHighQualityOverlay(self.ptr, )
    }

    unsafe fn GetOverlayKey(&self, ulOverlayHandle: VROverlayHandle_t, pchValue: *mut c_char, unBufferSize: u32, pError: *mut EVROverlayError) {
        GetOverlayKey(self.ptr, ulOverlayHandle, pchValue, unBufferSize, pError)
    }

    unsafe fn GetOverlayName(&self, ulOverlayHandle: VROverlayHandle_t, pchValue: *mut c_char, unBufferSize: u32, pError: *mut EVROverlayError) {
        GetOverlayName(self.ptr, ulOverlayHandle, pchValue, unBufferSize, pError)
    }

    unsafe fn GetOverlayImageData(&self, ulOverlayHandle: VROverlayHandle_t, pvBuffer: *mut libc::c_void, unBufferSize: u32, punWidth: *mut u32, punHeight: *mut u32) {
        GetOverlayImageData(self.ptr, ulOverlayHandle, pvBuffer, unBufferSize, punWidth, punHeight)
    }

    unsafe fn GetOverlayErrorNameFromEnum(&self, error: EVROverlayError) {
        GetOverlayErrorNameFromEnum(self.ptr, error)
    }

    unsafe fn SetOverlayRenderingPid(&self, ulOverlayHandle: VROverlayHandle_t, unPID: u32) {
        SetOverlayRenderingPid(self.ptr, ulOverlayHandle, unPID)
    }

    unsafe fn GetOverlayRenderingPid(&self, ulOverlayHandle: VROverlayHandle_t) {
        GetOverlayRenderingPid(self.ptr, ulOverlayHandle)
    }

    unsafe fn SetOverlayFlag(&self, ulOverlayHandle: VROverlayHandle_t, eOverlayFlag: VROverlayFlags, bEnabled: bool) {
        SetOverlayFlag(self.ptr, ulOverlayHandle, eOverlayFlag, bEnabled)
    }

    unsafe fn GetOverlayFlag(&self, ulOverlayHandle: VROverlayHandle_t, eOverlayFlag: VROverlayFlags, pbEnabled: *mut bool) {
        GetOverlayFlag(self.ptr, ulOverlayHandle, eOverlayFlag, pbEnabled)
    }

    unsafe fn SetOverlayColor(&self, ulOverlayHandle: VROverlayHandle_t, fRed: f32, fGreen: f32, fBlue: f32) {
        SetOverlayColor(self.ptr, ulOverlayHandle, fRed, fGreen, fBlue)
    }

    unsafe fn GetOverlayColor(&self, ulOverlayHandle: VROverlayHandle_t, pfRed: *mut f32, pfGreen: *mut f32, pfBlue: *mut f32) {
        GetOverlayColor(self.ptr, ulOverlayHandle, pfRed, pfGreen, pfBlue)
    }

    unsafe fn SetOverlayAlpha(&self, ulOverlayHandle: VROverlayHandle_t, fAlpha: f32) {
        SetOverlayAlpha(self.ptr, ulOverlayHandle, fAlpha)
    }

    unsafe fn GetOverlayAlpha(&self, ulOverlayHandle: VROverlayHandle_t, pfAlpha: *mut f32) {
        GetOverlayAlpha(self.ptr, ulOverlayHandle, pfAlpha)
    }

    unsafe fn SetOverlayWidthInMeters(&self, ulOverlayHandle: VROverlayHandle_t, fWidthInMeters: f32) {
        SetOverlayWidthInMeters(self.ptr, ulOverlayHandle, fWidthInMeters)
    }

    unsafe fn GetOverlayWidthInMeters(&self, ulOverlayHandle: VROverlayHandle_t, pfWidthInMeters: *mut f32) {
        GetOverlayWidthInMeters(self.ptr, ulOverlayHandle, pfWidthInMeters)
    }

    unsafe fn SetOverlayAutoCurveDistanceRangeInMeters(&self, ulOverlayHandle: VROverlayHandle_t, fMinDistanceInMeters: f32, fMaxDistanceInMeters: f32) {
        SetOverlayAutoCurveDistanceRangeInMeters(self.ptr, ulOverlayHandle, fMinDistanceInMeters, fMaxDistanceInMeters)
    }

    unsafe fn GetOverlayAutoCurveDistanceRangeInMeters(&self, ulOverlayHandle: VROverlayHandle_t, pfMinDistanceInMeters: *mut f32, pfMaxDistanceInMeters: *mut f32) {
        GetOverlayAutoCurveDistanceRangeInMeters(self.ptr, ulOverlayHandle, pfMinDistanceInMeters, pfMaxDistanceInMeters)
    }

    unsafe fn SetOverlayTextureColorSpace(&self, ulOverlayHandle: VROverlayHandle_t, eTextureColorSpace: EColorSpace) {
        SetOverlayTextureColorSpace(self.ptr, ulOverlayHandle, eTextureColorSpace)
    }

    unsafe fn GetOverlayTextureColorSpace(&self, ulOverlayHandle: VROverlayHandle_t, peTextureColorSpace: *mut EColorSpace) {
        GetOverlayTextureColorSpace(self.ptr, ulOverlayHandle, peTextureColorSpace)
    }

    unsafe fn SetOverlayTextureBounds(&self, ulOverlayHandle: VROverlayHandle_t, pOverlayTextureBounds: *mut  VRTextureBounds_t) {
        SetOverlayTextureBounds(self.ptr, ulOverlayHandle, pOverlayTextureBounds)
    }

    unsafe fn GetOverlayTextureBounds(&self, ulOverlayHandle: VROverlayHandle_t, pOverlayTextureBounds: *mut VRTextureBounds_t) {
        GetOverlayTextureBounds(self.ptr, ulOverlayHandle, pOverlayTextureBounds)
    }

    unsafe fn GetOverlayTransformType(&self, ulOverlayHandle: VROverlayHandle_t, peTransformType: *mut VROverlayTransformType) {
        GetOverlayTransformType(self.ptr, ulOverlayHandle, peTransformType)
    }

    unsafe fn SetOverlayTransformAbsolute(&self, ulOverlayHandle: VROverlayHandle_t, eTrackingOrigin: ETrackingUniverseOrigin, pmatTrackingOriginToOverlayTransform: *mut  HmdMatrix34_t) {
        SetOverlayTransformAbsolute(self.ptr, ulOverlayHandle, eTrackingOrigin, pmatTrackingOriginToOverlayTransform)
    }

    unsafe fn GetOverlayTransformAbsolute(&self, ulOverlayHandle: VROverlayHandle_t, peTrackingOrigin: *mut ETrackingUniverseOrigin, pmatTrackingOriginToOverlayTransform: *mut HmdMatrix34_t) {
        GetOverlayTransformAbsolute(self.ptr, ulOverlayHandle, peTrackingOrigin, pmatTrackingOriginToOverlayTransform)
    }

    unsafe fn SetOverlayTransformTrackedDeviceRelative(&self, ulOverlayHandle: VROverlayHandle_t, unTrackedDevice: TrackedDeviceIndex_t, pmatTrackedDeviceToOverlayTransform: *mut  HmdMatrix34_t) {
        SetOverlayTransformTrackedDeviceRelative(self.ptr, ulOverlayHandle, unTrackedDevice, pmatTrackedDeviceToOverlayTransform)
    }

    unsafe fn GetOverlayTransformTrackedDeviceRelative(&self, ulOverlayHandle: VROverlayHandle_t, punTrackedDevice: *mut TrackedDeviceIndex_t, pmatTrackedDeviceToOverlayTransform: *mut HmdMatrix34_t) {
        GetOverlayTransformTrackedDeviceRelative(self.ptr, ulOverlayHandle, punTrackedDevice, pmatTrackedDeviceToOverlayTransform)
    }

    unsafe fn SetOverlayTransformTrackedDeviceComponent(&self, ulOverlayHandle: VROverlayHandle_t, unDeviceIndex: TrackedDeviceIndex_t, pchComponentName: *const c_char) {
        SetOverlayTransformTrackedDeviceComponent(self.ptr, ulOverlayHandle, unDeviceIndex, pchComponentName)
    }

    unsafe fn GetOverlayTransformTrackedDeviceComponent(&self, ulOverlayHandle: VROverlayHandle_t, punDeviceIndex: *mut TrackedDeviceIndex_t, pchComponentName: *mut c_char, unComponentNameSize: u32) {
        GetOverlayTransformTrackedDeviceComponent(self.ptr, ulOverlayHandle, punDeviceIndex, pchComponentName, unComponentNameSize)
    }

    unsafe fn ShowOverlay(&self, ulOverlayHandle: VROverlayHandle_t) {
        ShowOverlay(self.ptr, ulOverlayHandle)
    }

    unsafe fn HideOverlay(&self, ulOverlayHandle: VROverlayHandle_t) {
        HideOverlay(self.ptr, ulOverlayHandle)
    }

    unsafe fn IsOverlayVisible(&self, ulOverlayHandle: VROverlayHandle_t) {
        IsOverlayVisible(self.ptr, ulOverlayHandle)
    }

    unsafe fn GetTransformForOverlayCoordinates(&self, ulOverlayHandle: VROverlayHandle_t, eTrackingOrigin: ETrackingUniverseOrigin, coordinatesInOverlay: HmdVector2_t, pmatTransform: *mut HmdMatrix34_t) {
        GetTransformForOverlayCoordinates(self.ptr, ulOverlayHandle, eTrackingOrigin, coordinatesInOverlay, pmatTransform)
    }

    unsafe fn PollNextOverlayEvent(&self, ulOverlayHandle: VROverlayHandle_t, pEvent: *mut VREvent_t, uncbVREvent: u32) {
        PollNextOverlayEvent(self.ptr, ulOverlayHandle, pEvent, uncbVREvent)
    }

    unsafe fn GetOverlayInputMethod(&self, ulOverlayHandle: VROverlayHandle_t, peInputMethod: *mut VROverlayInputMethod) {
        GetOverlayInputMethod(self.ptr, ulOverlayHandle, peInputMethod)
    }

    unsafe fn SetOverlayInputMethod(&self, ulOverlayHandle: VROverlayHandle_t, eInputMethod: VROverlayInputMethod) {
        SetOverlayInputMethod(self.ptr, ulOverlayHandle, eInputMethod)
    }

    unsafe fn GetOverlayMouseScale(&self, ulOverlayHandle: VROverlayHandle_t, pvecMouseScale: *mut HmdVector2_t) {
        GetOverlayMouseScale(self.ptr, ulOverlayHandle, pvecMouseScale)
    }

    unsafe fn SetOverlayMouseScale(&self, ulOverlayHandle: VROverlayHandle_t, pvecMouseScale: *mut  HmdVector2_t) {
        SetOverlayMouseScale(self.ptr, ulOverlayHandle, pvecMouseScale)
    }

    unsafe fn ComputeOverlayIntersection(&self, ulOverlayHandle: VROverlayHandle_t, pParams: *mut  VROverlayIntersectionParams_t, pResults: *mut VROverlayIntersectionResults_t) {
        ComputeOverlayIntersection(self.ptr, ulOverlayHandle, pParams, pResults)
    }

    unsafe fn HandleControllerOverlayInteractionAsMouse(&self, ulOverlayHandle: VROverlayHandle_t, unControllerDeviceIndex: TrackedDeviceIndex_t) {
        HandleControllerOverlayInteractionAsMouse(self.ptr, ulOverlayHandle, unControllerDeviceIndex)
    }

    unsafe fn IsHoverTargetOverlay(&self, ulOverlayHandle: VROverlayHandle_t) {
        IsHoverTargetOverlay(self.ptr, ulOverlayHandle)
    }

    unsafe fn GetGamepadFocusOverlay(&self, ) {
        GetGamepadFocusOverlay(self.ptr, )
    }

    unsafe fn SetGamepadFocusOverlay(&self, ulNewFocusOverlay: VROverlayHandle_t) {
        SetGamepadFocusOverlay(self.ptr, ulNewFocusOverlay)
    }

    unsafe fn SetOverlayNeighbor(&self, eDirection: EOverlayDirection, ulFrom: VROverlayHandle_t, ulTo: VROverlayHandle_t) {
        SetOverlayNeighbor(self.ptr, eDirection, ulFrom, ulTo)
    }

    unsafe fn MoveGamepadFocusToNeighbor(&self, eDirection: EOverlayDirection, ulFrom: VROverlayHandle_t) {
        MoveGamepadFocusToNeighbor(self.ptr, eDirection, ulFrom)
    }

    unsafe fn SetOverlayTexture(&self, ulOverlayHandle: VROverlayHandle_t, pTexture: *mut  Texture_t) {
        SetOverlayTexture(self.ptr, ulOverlayHandle, pTexture)
    }

    unsafe fn ClearOverlayTexture(&self, ulOverlayHandle: VROverlayHandle_t) {
        ClearOverlayTexture(self.ptr, ulOverlayHandle)
    }

    unsafe fn SetOverlayRaw(&self, ulOverlayHandle: VROverlayHandle_t, pvBuffer: *mut libc::c_void, unWidth: u32, unHeight: u32, unDepth: u32) {
        SetOverlayRaw(self.ptr, ulOverlayHandle, pvBuffer, unWidth, unHeight, unDepth)
    }

    unsafe fn SetOverlayFromFile(&self, ulOverlayHandle: VROverlayHandle_t, pchFilePath: *const c_char) {
        SetOverlayFromFile(self.ptr, ulOverlayHandle, pchFilePath)
    }

    unsafe fn GetOverlayTexture(&self, ulOverlayHandle: VROverlayHandle_t, pNativeTextureHandle: *mut *mut libc::c_void, pNativeTextureRef: *mut libc::c_void, pWidth: *mut u32, pHeight: *mut u32, pNativeFormat: *mut u32, pAPI: *mut EGraphicsAPIConvention, pColorSpace: *mut EColorSpace) {
        GetOverlayTexture(self.ptr, ulOverlayHandle, pNativeTextureHandle, pNativeTextureRef, pWidth, pHeight, pNativeFormat, pAPI, pColorSpace)
    }

    unsafe fn ReleaseNativeOverlayHandle(&self, ulOverlayHandle: VROverlayHandle_t, pNativeTextureHandle: *mut libc::c_void) {
        ReleaseNativeOverlayHandle(self.ptr, ulOverlayHandle, pNativeTextureHandle)
    }

    unsafe fn CreateDashboardOverlay(&self, pchOverlayKey: *const c_char, pchOverlayFriendlyName: *const c_char, pMainHandle: *mut VROverlayHandle_t, pThumbnailHandle: *mut VROverlayHandle_t) {
        CreateDashboardOverlay(self.ptr, pchOverlayKey, pchOverlayFriendlyName, pMainHandle, pThumbnailHandle)
    }

    unsafe fn IsDashboardVisible(&self, ) {
        IsDashboardVisible(self.ptr, )
    }

    unsafe fn IsActiveDashboardOverlay(&self, ulOverlayHandle: VROverlayHandle_t) {
        IsActiveDashboardOverlay(self.ptr, ulOverlayHandle)
    }

    unsafe fn SetDashboardOverlaySceneProcess(&self, ulOverlayHandle: VROverlayHandle_t, unProcessId: u32) {
        SetDashboardOverlaySceneProcess(self.ptr, ulOverlayHandle, unProcessId)
    }

    unsafe fn GetDashboardOverlaySceneProcess(&self, ulOverlayHandle: VROverlayHandle_t, punProcessId: *mut u32) {
        GetDashboardOverlaySceneProcess(self.ptr, ulOverlayHandle, punProcessId)
    }

    unsafe fn ShowDashboard(&self, pchOverlayToShow: *const c_char) {
        ShowDashboard(self.ptr, pchOverlayToShow)
    }

    unsafe fn GetPrimaryDashboardDevice(&self, ) {
        GetPrimaryDashboardDevice(self.ptr, )
    }

    unsafe fn ShowKeyboard(&self, eInputMode: EGamepadTextInputMode, eLineInputMode: EGamepadTextInputLineMode, pchDescription: *const c_char, unCharMax: u32, pchExistingText: *const c_char, bUseMinimalMode: bool, uUserValue: uint64_t) {
        ShowKeyboard(self.ptr, eInputMode, eLineInputMode, pchDescription, unCharMax, pchExistingText, bUseMinimalMode, uUserValue)
    }

    unsafe fn ShowKeyboardForOverlay(&self, ulOverlayHandle: VROverlayHandle_t, eInputMode: EGamepadTextInputMode, eLineInputMode: EGamepadTextInputLineMode, pchDescription: *const c_char, unCharMax: u32, pchExistingText: *const c_char, bUseMinimalMode: bool, uUserValue: uint64_t) {
        ShowKeyboardForOverlay(self.ptr, ulOverlayHandle, eInputMode, eLineInputMode, pchDescription, unCharMax, pchExistingText, bUseMinimalMode, uUserValue)
    }

    unsafe fn GetKeyboardText(&self, pchText: *mut c_char, cchText: u32) {
        GetKeyboardText(self.ptr, pchText, cchText)
    }

    unsafe fn HideKeyboard(&self, ) {
        HideKeyboard(self.ptr, )
    }

    unsafe fn SetKeyboardTransformAbsolute(&self, eTrackingOrigin: ETrackingUniverseOrigin, pmatTrackingOriginToKeyboardTransform: *mut  HmdMatrix34_t) {
        SetKeyboardTransformAbsolute(self.ptr, eTrackingOrigin, pmatTrackingOriginToKeyboardTransform)
    }

    unsafe fn SetKeyboardPositionForOverlay(&self, ulOverlayHandle: VROverlayHandle_t, avoidRect: HmdRect2_t) {
        SetKeyboardPositionForOverlay(self.ptr, ulOverlayHandle, avoidRect)
    }

}

struct IVRRenderModels {
    ptr: *mut IVRRenderModels
}

impl IVRRenderModels {
    unsafe fn LoadRenderModel_Async(&self, pchRenderModelName: *const c_char, ppRenderModel: *mut *mut RenderModel_t) {
        LoadRenderModel_Async(self.ptr, pchRenderModelName, ppRenderModel)
    }

    unsafe fn FreeRenderModel(&self, pRenderModel: *mut RenderModel_t) {
        FreeRenderModel(self.ptr, pRenderModel)
    }

    unsafe fn LoadTexture_Async(&self, textureId: TextureID_t, ppTexture: *mut *mut RenderModel_TextureMap_t) {
        LoadTexture_Async(self.ptr, textureId, ppTexture)
    }

    unsafe fn FreeTexture(&self, pTexture: *mut RenderModel_TextureMap_t) {
        FreeTexture(self.ptr, pTexture)
    }

    unsafe fn LoadTextureD3D11_Async(&self, textureId: TextureID_t, pD3D11Device: *mut libc::c_void, ppD3D11Texture2D: *mut *mut libc::c_void) {
        LoadTextureD3D11_Async(self.ptr, textureId, pD3D11Device, ppD3D11Texture2D)
    }

    unsafe fn LoadIntoTextureD3D11_Async(&self, textureId: TextureID_t, pDstTexture: *mut libc::c_void) {
        LoadIntoTextureD3D11_Async(self.ptr, textureId, pDstTexture)
    }

    unsafe fn FreeTextureD3D11(&self, pD3D11Texture2D: *mut libc::c_void) {
        FreeTextureD3D11(self.ptr, pD3D11Texture2D)
    }

    unsafe fn GetRenderModelName(&self, unRenderModelIndex: u32, pchRenderModelName: *mut c_char, unRenderModelNameLen: u32) {
        GetRenderModelName(self.ptr, unRenderModelIndex, pchRenderModelName, unRenderModelNameLen)
    }

    unsafe fn GetRenderModelCount(&self, ) {
        GetRenderModelCount(self.ptr, )
    }

    unsafe fn GetComponentCount(&self, pchRenderModelName: *const c_char) {
        GetComponentCount(self.ptr, pchRenderModelName)
    }

    unsafe fn GetComponentName(&self, pchRenderModelName: *const c_char, unComponentIndex: u32, pchComponentName: *mut c_char, unComponentNameLen: u32) {
        GetComponentName(self.ptr, pchRenderModelName, unComponentIndex, pchComponentName, unComponentNameLen)
    }

    unsafe fn GetComponentButtonMask(&self, pchRenderModelName: *const c_char, pchComponentName: *const c_char) {
        GetComponentButtonMask(self.ptr, pchRenderModelName, pchComponentName)
    }

    unsafe fn GetComponentRenderModelName(&self, pchRenderModelName: *const c_char, pchComponentName: *const c_char, pchComponentRenderModelName: *mut c_char, unComponentRenderModelNameLen: u32) {
        GetComponentRenderModelName(self.ptr, pchRenderModelName, pchComponentName, pchComponentRenderModelName, unComponentRenderModelNameLen)
    }

    unsafe fn GetComponentState(&self, pchRenderModelName: *const c_char, pchComponentName: *const c_char, pControllerState: *mut  VRControllerState_t, pState: *mut  RenderModel_ControllerMode_State_t, pComponentState: *mut RenderModel_ComponentState_t) {
        GetComponentState(self.ptr, pchRenderModelName, pchComponentName, pControllerState, pState, pComponentState)
    }

    unsafe fn RenderModelHasComponent(&self, pchRenderModelName: *const c_char, pchComponentName: *const c_char) {
        RenderModelHasComponent(self.ptr, pchRenderModelName, pchComponentName)
    }

}

struct IVRSettings {
    ptr: *mut IVRSettings
}

impl IVRSettings {
    unsafe fn GetSettingsErrorNameFromEnum(&self, eError: EVRSettingsError) {
        GetSettingsErrorNameFromEnum(self.ptr, eError)
    }

    unsafe fn Sync(&self, bForce: bool, peError: *mut EVRSettingsError) {
        Sync(self.ptr, bForce, peError)
    }

    unsafe fn GetBool(&self, pchSection: *const c_char, pchSettingsKey: *const c_char, bDefaultValue: bool, peError: *mut EVRSettingsError) {
        GetBool(self.ptr, pchSection, pchSettingsKey, bDefaultValue, peError)
    }

    unsafe fn SetBool(&self, pchSection: *const c_char, pchSettingsKey: *const c_char, bValue: bool, peError: *mut EVRSettingsError) {
        SetBool(self.ptr, pchSection, pchSettingsKey, bValue, peError)
    }

    unsafe fn GetInt32(&self, pchSection: *const c_char, pchSettingsKey: *const c_char, nDefaultValue: i32, peError: *mut EVRSettingsError) {
        GetInt32(self.ptr, pchSection, pchSettingsKey, nDefaultValue, peError)
    }

    unsafe fn SetInt32(&self, pchSection: *const c_char, pchSettingsKey: *const c_char, nValue: i32, peError: *mut EVRSettingsError) {
        SetInt32(self.ptr, pchSection, pchSettingsKey, nValue, peError)
    }

    unsafe fn GetFloat(&self, pchSection: *const c_char, pchSettingsKey: *const c_char, flDefaultValue: f32, peError: *mut EVRSettingsError) {
        GetFloat(self.ptr, pchSection, pchSettingsKey, flDefaultValue, peError)
    }

    unsafe fn SetFloat(&self, pchSection: *const c_char, pchSettingsKey: *const c_char, flValue: f32, peError: *mut EVRSettingsError) {
        SetFloat(self.ptr, pchSection, pchSettingsKey, flValue, peError)
    }

    unsafe fn GetString(&self, pchSection: *const c_char, pchSettingsKey: *const c_char, pchValue: *mut c_char, unValueLen: u32, pchDefaultValue: *const c_char, peError: *mut EVRSettingsError) {
        GetString(self.ptr, pchSection, pchSettingsKey, pchValue, unValueLen, pchDefaultValue, peError)
    }

    unsafe fn SetString(&self, pchSection: *const c_char, pchSettingsKey: *const c_char, pchValue: *const c_char, peError: *mut EVRSettingsError) {
        SetString(self.ptr, pchSection, pchSettingsKey, pchValue, peError)
    }

    unsafe fn RemoveSection(&self, pchSection: *const c_char, peError: *mut EVRSettingsError) {
        RemoveSection(self.ptr, pchSection, peError)
    }

    unsafe fn RemoveKeyInSection(&self, pchSection: *const c_char, pchSettingsKey: *const c_char, peError: *mut EVRSettingsError) {
        RemoveKeyInSection(self.ptr, pchSection, pchSettingsKey, peError)
    }

}

struct IVRSystem {
    ptr: *mut IVRSystem
}

impl IVRSystem {
    unsafe fn GetRecommendedRenderTargetSize(&self, pnWidth: *mut u32, pnHeight: *mut u32) {
        GetRecommendedRenderTargetSize(self.ptr, pnWidth, pnHeight)
    }

    unsafe fn GetProjectionMatrix(&self, eEye: EVREye, fNearZ: f32, fFarZ: f32, eProjType: EGraphicsAPIConvention) {
        GetProjectionMatrix(self.ptr, eEye, fNearZ, fFarZ, eProjType)
    }

    unsafe fn GetProjectionRaw(&self, eEye: EVREye, pfLeft: *mut f32, pfRight: *mut f32, pfTop: *mut f32, pfBottom: *mut f32) {
        GetProjectionRaw(self.ptr, eEye, pfLeft, pfRight, pfTop, pfBottom)
    }

    unsafe fn ComputeDistortion(&self, eEye: EVREye, fU: f32, fV: f32) {
        ComputeDistortion(self.ptr, eEye, fU, fV)
    }

    unsafe fn GetEyeToHeadTransform(&self, eEye: EVREye) {
        GetEyeToHeadTransform(self.ptr, eEye)
    }

    unsafe fn GetTimeSinceLastVsync(&self, pfSecondsSinceLastVsync: *mut f32, pulFrameCounter: *mut uint64_t) {
        GetTimeSinceLastVsync(self.ptr, pfSecondsSinceLastVsync, pulFrameCounter)
    }

    unsafe fn GetD3D9AdapterIndex(&self, ) {
        GetD3D9AdapterIndex(self.ptr, )
    }

    unsafe fn GetDXGIOutputInfo(&self, pnAdapterIndex: *mut i32) {
        GetDXGIOutputInfo(self.ptr, pnAdapterIndex)
    }

    unsafe fn IsDisplayOnDesktop(&self, ) {
        IsDisplayOnDesktop(self.ptr, )
    }

    unsafe fn SetDisplayVisibility(&self, bIsVisibleOnDesktop: bool) {
        SetDisplayVisibility(self.ptr, bIsVisibleOnDesktop)
    }

    unsafe fn GetDeviceToAbsoluteTrackingPose(&self, eOrigin: ETrackingUniverseOrigin, fPredictedSecondsToPhotonsFromNow: f32, pTrackedDevicePoseArray: *mut TrackedDevicePose_t, unTrackedDevicePoseArrayCount: u32) {
        GetDeviceToAbsoluteTrackingPose(self.ptr, eOrigin, fPredictedSecondsToPhotonsFromNow, pTrackedDevicePoseArray, unTrackedDevicePoseArrayCount)
    }

    unsafe fn ResetSeatedZeroPose(&self, ) {
        ResetSeatedZeroPose(self.ptr, )
    }

    unsafe fn GetSeatedZeroPoseToStandingAbsoluteTrackingPose(&self, ) {
        GetSeatedZeroPoseToStandingAbsoluteTrackingPose(self.ptr, )
    }

    unsafe fn GetRawZeroPoseToStandingAbsoluteTrackingPose(&self, ) {
        GetRawZeroPoseToStandingAbsoluteTrackingPose(self.ptr, )
    }

    unsafe fn GetSortedTrackedDeviceIndicesOfClass(&self, eTrackedDeviceClass: ETrackedDeviceClass, punTrackedDeviceIndexArray: *mut TrackedDeviceIndex_t, unTrackedDeviceIndexArrayCount: u32, unRelativeToTrackedDeviceIndex: TrackedDeviceIndex_t) {
        GetSortedTrackedDeviceIndicesOfClass(self.ptr, eTrackedDeviceClass, punTrackedDeviceIndexArray, unTrackedDeviceIndexArrayCount, unRelativeToTrackedDeviceIndex)
    }

    unsafe fn GetTrackedDeviceActivityLevel(&self, unDeviceId: TrackedDeviceIndex_t) {
        GetTrackedDeviceActivityLevel(self.ptr, unDeviceId)
    }

    unsafe fn ApplyTransform(&self, pOutputPose: *mut TrackedDevicePose_t, pTrackedDevicePose: *mut  TrackedDevicePose_t, pTransform: *mut  HmdMatrix34_t) {
        ApplyTransform(self.ptr, pOutputPose, pTrackedDevicePose, pTransform)
    }

    unsafe fn GetTrackedDeviceIndexForControllerRole(&self, unDeviceType: ETrackedControllerRole) {
        GetTrackedDeviceIndexForControllerRole(self.ptr, unDeviceType)
    }

    unsafe fn GetControllerRoleForTrackedDeviceIndex(&self, unDeviceIndex: TrackedDeviceIndex_t) {
        GetControllerRoleForTrackedDeviceIndex(self.ptr, unDeviceIndex)
    }

    unsafe fn GetTrackedDeviceClass(&self, unDeviceIndex: TrackedDeviceIndex_t) {
        GetTrackedDeviceClass(self.ptr, unDeviceIndex)
    }

    unsafe fn IsTrackedDeviceConnected(&self, unDeviceIndex: TrackedDeviceIndex_t) {
        IsTrackedDeviceConnected(self.ptr, unDeviceIndex)
    }

    unsafe fn GetBoolTrackedDeviceProperty(&self, unDeviceIndex: TrackedDeviceIndex_t, prop: ETrackedDeviceProperty, pError: *mut ETrackedPropertyError) {
        GetBoolTrackedDeviceProperty(self.ptr, unDeviceIndex, prop, pError)
    }

    unsafe fn GetFloatTrackedDeviceProperty(&self, unDeviceIndex: TrackedDeviceIndex_t, prop: ETrackedDeviceProperty, pError: *mut ETrackedPropertyError) {
        GetFloatTrackedDeviceProperty(self.ptr, unDeviceIndex, prop, pError)
    }

    unsafe fn GetInt32TrackedDeviceProperty(&self, unDeviceIndex: TrackedDeviceIndex_t, prop: ETrackedDeviceProperty, pError: *mut ETrackedPropertyError) {
        GetInt32TrackedDeviceProperty(self.ptr, unDeviceIndex, prop, pError)
    }

    unsafe fn GetUint64TrackedDeviceProperty(&self, unDeviceIndex: TrackedDeviceIndex_t, prop: ETrackedDeviceProperty, pError: *mut ETrackedPropertyError) {
        GetUint64TrackedDeviceProperty(self.ptr, unDeviceIndex, prop, pError)
    }

    unsafe fn GetMatrix34TrackedDeviceProperty(&self, unDeviceIndex: TrackedDeviceIndex_t, prop: ETrackedDeviceProperty, pError: *mut ETrackedPropertyError) {
        GetMatrix34TrackedDeviceProperty(self.ptr, unDeviceIndex, prop, pError)
    }

    unsafe fn GetStringTrackedDeviceProperty(&self, unDeviceIndex: TrackedDeviceIndex_t, prop: ETrackedDeviceProperty, pchValue: *mut c_char, unBufferSize: u32, pError: *mut ETrackedPropertyError) {
        GetStringTrackedDeviceProperty(self.ptr, unDeviceIndex, prop, pchValue, unBufferSize, pError)
    }

    unsafe fn GetPropErrorNameFromEnum(&self, error: ETrackedPropertyError) {
        GetPropErrorNameFromEnum(self.ptr, error)
    }

    unsafe fn PollNextEvent(&self, pEvent: *mut VREvent_t, uncbVREvent: u32) {
        PollNextEvent(self.ptr, pEvent, uncbVREvent)
    }

    unsafe fn PollNextEventWithPose(&self, eOrigin: ETrackingUniverseOrigin, pEvent: *mut VREvent_t, uncbVREvent: u32, pTrackedDevicePose: *mut TrackedDevicePose_t) {
        PollNextEventWithPose(self.ptr, eOrigin, pEvent, uncbVREvent, pTrackedDevicePose)
    }

    unsafe fn GetEventTypeNameFromEnum(&self, eType: EVREventType) {
        GetEventTypeNameFromEnum(self.ptr, eType)
    }

    unsafe fn GetHiddenAreaMesh(&self, eEye: EVREye) {
        GetHiddenAreaMesh(self.ptr, eEye)
    }

    unsafe fn GetControllerState(&self, unControllerDeviceIndex: TrackedDeviceIndex_t, pControllerState: *mut VRControllerState_t) {
        GetControllerState(self.ptr, unControllerDeviceIndex, pControllerState)
    }

    unsafe fn GetControllerStateWithPose(&self, eOrigin: ETrackingUniverseOrigin, unControllerDeviceIndex: TrackedDeviceIndex_t, pControllerState: *mut VRControllerState_t, pTrackedDevicePose: *mut TrackedDevicePose_t) {
        GetControllerStateWithPose(self.ptr, eOrigin, unControllerDeviceIndex, pControllerState, pTrackedDevicePose)
    }

    unsafe fn TriggerHapticPulse(&self, unControllerDeviceIndex: TrackedDeviceIndex_t, unAxisId: u32, usDurationMicroSec: u16) {
        TriggerHapticPulse(self.ptr, unControllerDeviceIndex, unAxisId, usDurationMicroSec)
    }

    unsafe fn GetButtonIdNameFromEnum(&self, eButtonId: EVRButtonId) {
        GetButtonIdNameFromEnum(self.ptr, eButtonId)
    }

    unsafe fn GetControllerAxisTypeNameFromEnum(&self, eAxisType: EVRControllerAxisType) {
        GetControllerAxisTypeNameFromEnum(self.ptr, eAxisType)
    }

    unsafe fn CaptureInputFocus(&self, ) {
        CaptureInputFocus(self.ptr, )
    }

    unsafe fn ReleaseInputFocus(&self, ) {
        ReleaseInputFocus(self.ptr, )
    }

    unsafe fn IsInputFocusCapturedByAnotherProcess(&self, ) {
        IsInputFocusCapturedByAnotherProcess(self.ptr, )
    }

    unsafe fn DriverDebugRequest(&self, unDeviceIndex: TrackedDeviceIndex_t, pchRequest: *const c_char, pchResponseBuffer: *mut c_char, unResponseBufferSize: u32) {
        DriverDebugRequest(self.ptr, unDeviceIndex, pchRequest, pchResponseBuffer, unResponseBufferSize)
    }

    unsafe fn PerformFirmwareUpdate(&self, unDeviceIndex: TrackedDeviceIndex_t) {
        PerformFirmwareUpdate(self.ptr, unDeviceIndex)
    }

    unsafe fn AcknowledgeQuit_Exiting(&self, ) {
        AcknowledgeQuit_Exiting(self.ptr, )
    }

    unsafe fn AcknowledgeQuit_UserPrompt(&self, ) {
        AcknowledgeQuit_UserPrompt(self.ptr, )
    }

}

enum EVREye {
    Eye_Left = 0,
    Eye_Right = 1,
}

enum EGraphicsAPIConvention {
    API_DirectX = 0,
    API_OpenGL = 1,
}

enum EColorSpace {
    ColorSpace_Auto = 0,
    ColorSpace_Gamma = 1,
    ColorSpace_Linear = 2,
}

enum ETrackingResult {
    TrackingResult_Uninitialized = 1,
    TrackingResult_Calibrating_InProgress = 100,
    TrackingResult_Calibrating_OutOfRange = 101,
    TrackingResult_Running_OK = 200,
    TrackingResult_Running_OutOfRange = 201,
}

enum ETrackedDeviceClass {
    TrackedDeviceClass_Invalid = 0,
    TrackedDeviceClass_HMD = 1,
    TrackedDeviceClass_Controller = 2,
    TrackedDeviceClass_TrackingReference = 4,
    TrackedDeviceClass_Other = 1000,
}

enum ETrackedControllerRole {
    TrackedControllerRole_Invalid = 0,
    TrackedControllerRole_LeftHand = 1,
    TrackedControllerRole_RightHand = 2,
}

enum ETrackingUniverseOrigin {
    TrackingUniverseSeated = 0,
    TrackingUniverseStanding = 1,
    TrackingUniverseRawAndUncalibrated = 2,
}

enum ETrackedDeviceProperty {
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

enum ETrackedPropertyError {
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

enum EVRSubmitFlags {
    Submit_Default = 0,
    Submit_LensDistortionAlreadyApplied = 1,
    Submit_GlRenderBuffer = 2,
}

enum EVRState {
    VRState_Undefined = -1,
    VRState_Off = 0,
    VRState_Searching = 1,
    VRState_Searching_Alert = 2,
    VRState_Ready = 3,
    VRState_Ready_Alert = 4,
    VRState_NotReady = 5,
}

enum EVREventType {
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

enum EDeviceActivityLevel {
    k_EDeviceActivityLevel_Unknown = -1,
    k_EDeviceActivityLevel_Idle = 0,
    k_EDeviceActivityLevel_UserInteraction = 1,
    k_EDeviceActivityLevel_UserInteraction_Timeout = 2,
    k_EDeviceActivityLevel_Standby = 3,
}

enum EVRButtonId {
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
    k_EButton_SteamVR_Touchpad = 32,
    k_EButton_SteamVR_Trigger = 33,
    k_EButton_Dashboard_Back = 2,
    k_EButton_Max = 64,
}

enum EVRMouseButton {
    VRMouseButton_Left = 1,
    VRMouseButton_Right = 2,
    VRMouseButton_Middle = 4,
}

enum EVRControllerAxisType {
    k_eControllerAxis_None = 0,
    k_eControllerAxis_TrackPad = 1,
    k_eControllerAxis_Joystick = 2,
    k_eControllerAxis_Trigger = 3,
}

enum EVRControllerEventOutputType {
    ControllerEventOutput_OSEvents = 0,
    ControllerEventOutput_VREvents = 1,
}

enum ECollisionBoundsStyle {
    COLLISION_BOUNDS_STYLE_BEGINNER = 0,
    COLLISION_BOUNDS_STYLE_INTERMEDIATE = 1,
    COLLISION_BOUNDS_STYLE_SQUARES = 2,
    COLLISION_BOUNDS_STYLE_ADVANCED = 3,
    COLLISION_BOUNDS_STYLE_NONE = 4,
    COLLISION_BOUNDS_STYLE_COUNT = 5,
}

enum EVROverlayError {
    VROverlayError_None = 0,
    VROverlayError_UnknownOverlay = 10,
    VROverlayError_InvalidHandle = 11,
    VROverlayError_PermissionDenied = 12,
    VROverlayError_OverlayLimitExceeded = 13,
    VROverlayError_WrongVisibilityType = 14,
    VROverlayError_KeyTooLong = 15,
    VROverlayError_NameTooLong = 16,
    VROverlayError_KeyInUse = 17,
    VROverlayError_WrongTransformType = 18,
    VROverlayError_InvalidTrackedDevice = 19,
    VROverlayError_InvalidParameter = 20,
    VROverlayError_ThumbnailCantBeDestroyed = 21,
    VROverlayError_ArrayTooSmall = 22,
    VROverlayError_RequestFailed = 23,
    VROverlayError_InvalidTexture = 24,
    VROverlayError_UnableToLoadFile = 25,
    VROVerlayError_KeyboardAlreadyInUse = 26,
    VROverlayError_NoNeighbor = 27,
}

enum EVRApplicationType {
    VRApplication_Other = 0,
    VRApplication_Scene = 1,
    VRApplication_Overlay = 2,
    VRApplication_Background = 3,
    VRApplication_Utility = 4,
}

enum EVRFirmwareError {
    VRFirmwareError_None = 0,
    VRFirmwareError_Success = 1,
    VRFirmwareError_Fail = 2,
}

enum EVRNotificationError {
    VRNotificationError_OK = 0,
    VRNotificationError_InvalidNotificationId = 100,
    VRNotificationError_NotificationQueueFull = 101,
    VRNotificationError_InvalidOverlayHandle = 102,
}

enum EVRInitError {
    VRInitError_None = 0,
    VRInitError_Unknown = 1,
    VRInitError_Init_InstallationNotFound = 100,
    VRInitError_Init_InstallationCorrupt = 101,
    VRInitError_Init_VRClientDLLNotFound = 102,
    VRInitError_Init_FileNotFound = 103,
    VRInitError_Init_FactoryNotFound = 104,
    VRInitError_Init_InterfaceNotFound = 105,
    VRInitError_Init_InvalidInterface = 106,
    VRInitError_Init_UserConfigDirectoryInvalid = 107,
    VRInitError_Init_HmdNotFound = 108,
    VRInitError_Init_NotInitialized = 109,
    VRInitError_Init_PathRegistryNotFound = 110,
    VRInitError_Init_NoConfigPath = 111,
    VRInitError_Init_NoLogPath = 112,
    VRInitError_Init_PathRegistryNotWritable = 113,
    VRInitError_Init_AppInfoInitFailed = 114,
    VRInitError_Init_Retry = 115,
    VRInitError_Init_InitCanceledByUser = 116,
    VRInitError_Init_AnotherAppLaunching = 117,
    VRInitError_Init_SettingsInitFailed = 118,
    VRInitError_Init_ShuttingDown = 119,
    VRInitError_Init_TooManyObjects = 120,
    VRInitError_Init_NoServerForBackgroundApp = 121,
    VRInitError_Init_NotSupportedWithCompositor = 122,
    VRInitError_Init_NotAvailableToUtilityApps = 123,
    VRInitError_Driver_Failed = 200,
    VRInitError_Driver_Unknown = 201,
    VRInitError_Driver_HmdUnknown = 202,
    VRInitError_Driver_NotLoaded = 203,
    VRInitError_Driver_RuntimeOutOfDate = 204,
    VRInitError_Driver_HmdInUse = 205,
    VRInitError_Driver_NotCalibrated = 206,
    VRInitError_Driver_CalibrationInvalid = 207,
    VRInitError_Driver_HmdDisplayNotFound = 208,
    VRInitError_IPC_ServerInitFailed = 300,
    VRInitError_IPC_ConnectFailed = 301,
    VRInitError_IPC_SharedStateInitFailed = 302,
    VRInitError_IPC_CompositorInitFailed = 303,
    VRInitError_IPC_MutexInitFailed = 304,
    VRInitError_IPC_Failed = 305,
    VRInitError_Compositor_Failed = 400,
    VRInitError_Compositor_D3D11HardwareRequired = 401,
    VRInitError_VendorSpecific_UnableToConnectToOculusRuntime = 1000,
    VRInitError_VendorSpecific_HmdFound_CantOpenDevice = 1101,
    VRInitError_VendorSpecific_HmdFound_UnableToRequestConfigStart = 1102,
    VRInitError_VendorSpecific_HmdFound_NoStoredConfig = 1103,
    VRInitError_VendorSpecific_HmdFound_ConfigTooBig = 1104,
    VRInitError_VendorSpecific_HmdFound_ConfigTooSmall = 1105,
    VRInitError_VendorSpecific_HmdFound_UnableToInitZLib = 1106,
    VRInitError_VendorSpecific_HmdFound_CantReadFirmwareVersion = 1107,
    VRInitError_VendorSpecific_HmdFound_UnableToSendUserDataStart = 1108,
    VRInitError_VendorSpecific_HmdFound_UnableToGetUserDataStart = 1109,
    VRInitError_VendorSpecific_HmdFound_UnableToGetUserDataNext = 1110,
    VRInitError_VendorSpecific_HmdFound_UserDataAddressRange = 1111,
    VRInitError_VendorSpecific_HmdFound_UserDataError = 1112,
    VRInitError_VendorSpecific_HmdFound_ConfigFailedSanityCheck = 1113,
    VRInitError_Steam_SteamInstallationNotFound = 2000,
}

enum EVRApplicationError {
    VRApplicationError_None = 0,
    VRApplicationError_AppKeyAlreadyExists = 100,
    VRApplicationError_NoManifest = 101,
    VRApplicationError_NoApplication = 102,
    VRApplicationError_InvalidIndex = 103,
    VRApplicationError_UnknownApplication = 104,
    VRApplicationError_IPCFailed = 105,
    VRApplicationError_ApplicationAlreadyRunning = 106,
    VRApplicationError_InvalidManifest = 107,
    VRApplicationError_InvalidApplication = 108,
    VRApplicationError_LaunchFailed = 109,
    VRApplicationError_ApplicationAlreadyStarting = 110,
    VRApplicationError_LaunchInProgress = 111,
    VRApplicationError_OldApplicationQuitting = 112,
    VRApplicationError_TransitionAborted = 113,
    VRApplicationError_IsTemplate = 114,
    VRApplicationError_BufferTooSmall = 200,
    VRApplicationError_PropertyNotSet = 201,
    VRApplicationError_UnknownProperty = 202,
}

enum EVRApplicationProperty {
    VRApplicationProperty_Name_String = 0,
    VRApplicationProperty_LaunchType_String = 11,
    VRApplicationProperty_WorkingDirectory_String = 12,
    VRApplicationProperty_BinaryPath_String = 13,
    VRApplicationProperty_Arguments_String = 14,
    VRApplicationProperty_URL_String = 15,
    VRApplicationProperty_Description_String = 50,
    VRApplicationProperty_NewsURL_String = 51,
    VRApplicationProperty_ImagePath_String = 52,
    VRApplicationProperty_Source_String = 53,
    VRApplicationProperty_IsDashboardOverlay_Bool = 60,
    VRApplicationProperty_IsTemplate_Bool = 61,
    VRApplicationProperty_IsInstanced_Bool = 62,
    VRApplicationProperty_LastLaunchTime_Uint64 = 70,
}

enum EVRApplicationTransitionState {
    VRApplicationTransition_None = 0,
    VRApplicationTransition_OldAppQuitSent = 10,
    VRApplicationTransition_WaitingForExternalLaunch = 11,
    VRApplicationTransition_NewAppLaunched = 20,
}

enum ChaperoneCalibrationState {
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

enum EChaperoneConfigFile {
    EChaperoneConfigFile_Live = 1,
    EChaperoneConfigFile_Temp = 2,
}

enum EChaperoneImportFlags {
    EChaperoneImport_BoundsOnly = 1,
}

enum EVRCompositorError {
    VRCompositorError_None = 0,
    VRCompositorError_IncompatibleVersion = 100,
    VRCompositorError_DoNotHaveFocus = 101,
    VRCompositorError_InvalidTexture = 102,
    VRCompositorError_IsNotSceneApplication = 103,
    VRCompositorError_TextureIsOnWrongDevice = 104,
    VRCompositorError_TextureUsesUnsupportedFormat = 105,
    VRCompositorError_SharedTexturesNotSupported = 106,
    VRCompositorError_IndexOutOfRange = 107,
}

enum VROverlayInputMethod {
    VROverlayInputMethod_None = 0,
    VROverlayInputMethod_Mouse = 1,
}

enum VROverlayTransformType {
    VROverlayTransform_Absolute = 0,
    VROverlayTransform_TrackedDeviceRelative = 1,
    VROverlayTransform_SystemOverlay = 2,
    VROverlayTransform_TrackedComponent = 3,
}

enum VROverlayFlags {
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

enum EGamepadTextInputMode {
    k_EGamepadTextInputModeNormal = 0,
    k_EGamepadTextInputModePassword = 1,
    k_EGamepadTextInputModeSubmit = 2,
}

enum EGamepadTextInputLineMode {
    k_EGamepadTextInputLineModeSingleLine = 0,
    k_EGamepadTextInputLineModeMultipleLines = 1,
}

enum EOverlayDirection {
    OverlayDirection_Up = 0,
    OverlayDirection_Down = 1,
    OverlayDirection_Left = 2,
    OverlayDirection_Right = 3,
    OverlayDirection_Count = 4,
}

enum EVRRenderModelError {
    VRRenderModelError_None = 0,
    VRRenderModelError_Loading = 100,
    VRRenderModelError_NotSupported = 200,
    VRRenderModelError_InvalidArg = 300,
    VRRenderModelError_InvalidModel = 301,
    VRRenderModelError_NoShapes = 302,
    VRRenderModelError_MultipleShapes = 303,
    VRRenderModelError_TooManyIndices = 304,
    VRRenderModelError_MultipleTextures = 305,
    VRRenderModelError_InvalidTexture = 400,
}

enum EVRComponentProperty {
    VRComponentProperty_IsStatic = 1,
    VRComponentProperty_IsVisible = 2,
    VRComponentProperty_IsTouched = 4,
    VRComponentProperty_IsPressed = 8,
    VRComponentProperty_IsScrolled = 16,
}

enum EVRNotificationType {
    EVRNotificationType_Transient = 0,
    EVRNotificationType_Persistent = 1,
}

enum EVRNotificationStyle {
    EVRNotificationStyle_None = 0,
    EVRNotificationStyle_Application = 100,
    EVRNotificationStyle_Contact_Disabled = 200,
    EVRNotificationStyle_Contact_Enabled = 201,
    EVRNotificationStyle_Contact_Active = 202,
}

enum EVRSettingsError {
    VRSettingsError_None = 0,
    VRSettingsError_IPCFailed = 1,
    VRSettingsError_WriteFailed = 2,
    VRSettingsError_ReadFailed = 3,
}

const k_unTrackingStringSize:  uint32_t = 32;
const k_unMaxDriverDebugResponseSize:  uint32_t = 32768;
const k_unTrackedDeviceIndex_Hmd:  uint32_t = 0;
const k_unMaxTrackedDeviceCount:  uint32_t = 16;
const k_unTrackedDeviceIndexInvalid:  uint32_t = 4294967295;
const k_unMaxPropertyStringSize:  uint32_t = 32768;
const k_unControllerStateAxisCount:  uint32_t = 5;
const k_ulOverlayHandleInvalid:  VROverlayHandle_t = 0;
const IVRSystem_Version: *mut  char = IVRSystem_012;
const IVRExtendedDisplay_Version: *mut  char = IVRExtendedDisplay_001;
const k_unMaxApplicationKeyLength:  uint32_t = 128;
const IVRApplications_Version: *mut  char = IVRApplications_005;
const IVRChaperone_Version: *mut  char = IVRChaperone_003;
const IVRChaperoneSetup_Version: *mut  char = IVRChaperoneSetup_005;
const IVRCompositor_Version: *mut  char = IVRCompositor_013;
const k_unVROverlayMaxKeyLength:  uint32_t = 128;
const k_unVROverlayMaxNameLength:  uint32_t = 128;
const k_unMaxOverlayCount:  uint32_t = 32;
const IVROverlay_Version: *mut  char = IVROverlay_011;
const k_pch_Controller_Component_GDC2015: *mut  char = gdc2015;
const k_pch_Controller_Component_Base: *mut  char = base;
const k_pch_Controller_Component_Tip: *mut  char = tip;
const k_pch_Controller_Component_HandGrip: *mut  char = handgrip;
const k_pch_Controller_Component_Status: *mut  char = status;
const IVRRenderModels_Version: *mut  char = IVRRenderModels_005;
const k_unNotificationTextMaxSize:  uint32_t = 256;
const IVRNotifications_Version: *mut  char = IVRNotifications_002;
const k_unMaxSettingsKeyLength:  uint32_t = 128;
const k_pch_SteamVR_Section: *mut  char = steamvr;
const k_pch_SteamVR_RequireHmd_String: *mut  char = requireHmd;
const k_pch_SteamVR_ForcedDriverKey_String: *mut  char = forcedDriver;
const k_pch_SteamVR_ForcedHmdKey_String: *mut  char = forcedHmd;
const k_pch_SteamVR_DisplayDebug_Bool: *mut  char = displayDebug;
const k_pch_SteamVR_DebugProcessPipe_String: *mut  char = debugProcessPipe;
const k_pch_SteamVR_EnableDistortion_Bool: *mut  char = enableDistortion;
const k_pch_SteamVR_DisplayDebugX_Int32: *mut  char = displayDebugX;
const k_pch_SteamVR_DisplayDebugY_Int32: *mut  char = displayDebugY;
const k_pch_SteamVR_SendSystemButtonToAllApps_Bool: *mut  char = sendSystemButtonToAllApps;
const k_pch_SteamVR_LogLevel_Int32: *mut  char = loglevel;
const k_pch_SteamVR_IPD_Float: *mut  char = ipd;
const k_pch_SteamVR_Background_String: *mut  char = background;
const k_pch_SteamVR_GridColor_String: *mut  char = gridColor;
const k_pch_SteamVR_PlayAreaColor_String: *mut  char = playAreaColor;
const k_pch_SteamVR_ActivateMultipleDrivers_Bool: *mut  char = activateMultipleDrivers;
const k_pch_SteamVR_PowerOffOnExit_Bool: *mut  char = powerOffOnExit;
const k_pch_SteamVR_StandbyAppRunningTimeout_Float: *mut  char = standbyAppRunningTimeout;
const k_pch_SteamVR_StandbyNoAppTimeout_Float: *mut  char = standbyNoAppTimeout;
const k_pch_SteamVR_DirectMode_Bool: *mut  char = directMode;
const k_pch_SteamVR_DirectModeEdidVid_Int32: *mut  char = directModeEdidVid;
const k_pch_SteamVR_DirectModeEdidPid_Int32: *mut  char = directModeEdidPid;
const k_pch_SteamVR_UsingSpeakers_Bool: *mut  char = usingSpeakers;
const k_pch_SteamVR_SpeakersForwardYawOffsetDegrees_Float: *mut  char = speakersForwardYawOffsetDegrees;
const k_pch_SteamVR_BaseStationPowerManagement_Bool: *mut  char = basestationPowerManagement;
const k_pch_SteamVR_NeverKillProcesses_Bool: *mut  char = neverKillProcesses;
const k_pch_Lighthouse_Section: *mut  char = driver_lighthouse;
const k_pch_Lighthouse_DisableIMU_Bool: *mut  char = disableimu;
const k_pch_Lighthouse_UseDisambiguation_String: *mut  char = usedisambiguation;
const k_pch_Lighthouse_DisambiguationDebug_Int32: *mut  char = disambiguationdebug;
const k_pch_Lighthouse_PrimaryBasestation_Int32: *mut  char = primarybasestation;
const k_pch_Lighthouse_LighthouseName_String: *mut  char = lighthousename;
const k_pch_Lighthouse_MaxIncidenceAngleDegrees_Float: *mut  char = maxincidenceangledegrees;
const k_pch_Lighthouse_UseLighthouseDirect_Bool: *mut  char = uselighthousedirect;
const k_pch_Lighthouse_DBHistory_Bool: *mut  char = dbhistory;
const k_pch_Lighthouse_OriginOffsetX_Float: *mut  char = originoffsetx;
const k_pch_Lighthouse_OriginOffsetY_Float: *mut  char = originoffsety;
const k_pch_Lighthouse_OriginOffsetZ_Float: *mut  char = originoffsetz;
const k_pch_Lighthouse_HeadingOffset_Float: *mut  char = headingoffset;
const k_pch_Null_Section: *mut  char = driver_null;
const k_pch_Null_EnableNullDriver_Bool: *mut  char = enable;
const k_pch_Null_SerialNumber_String: *mut  char = serialNumber;
const k_pch_Null_ModelNumber_String: *mut  char = modelNumber;
const k_pch_Null_WindowX_Int32: *mut  char = windowX;
const k_pch_Null_WindowY_Int32: *mut  char = windowY;
const k_pch_Null_WindowWidth_Int32: *mut  char = windowWidth;
const k_pch_Null_WindowHeight_Int32: *mut  char = windowHeight;
const k_pch_Null_RenderWidth_Int32: *mut  char = renderWidth;
const k_pch_Null_RenderHeight_Int32: *mut  char = renderHeight;
const k_pch_Null_SecondsFromVsyncToPhotons_Float: *mut  char = secondsFromVsyncToPhotons;
const k_pch_Null_DisplayFrequency_Float: *mut  char = displayFrequency;
const k_pch_UserInterface_Section: *mut  char = userinterface;
const k_pch_UserInterface_StatusAlwaysOnTop_Bool: *mut  char = StatusAlwaysOnTop;
const k_pch_Notifications_Section: *mut  char = notifications;
const k_pch_Notifications_DoNotDisturb_Bool: *mut  char = DoNotDisturb;
const k_pch_Keyboard_Section: *mut  char = keyboard;
const k_pch_Keyboard_TutorialCompletions: *mut  char = TutorialCompletions;
const k_pch_Perf_Section: *mut  char = perfcheck;
const k_pch_Perf_HeuristicActive_Bool: *mut  char = heuristicActive;
const k_pch_Perf_NotifyInHMD_Bool: *mut  char = warnInHMD;
const k_pch_Perf_NotifyOnlyOnce_Bool: *mut  char = warnOnlyOnce;
const k_pch_Perf_AllowTimingStore_Bool: *mut  char = allowTimingStore;
const k_pch_Perf_SaveTimingsOnExit_Bool: *mut  char = saveTimingsOnExit;
const k_pch_Perf_TestData_Float: *mut  char = perfTestData;
const k_pch_Camera_Section: *mut  char = camera;
const IVRSettings_Version: *mut  char = IVRSettings_001;
const k_pch_audio_Section: *mut  char = audio;
const k_pch_audio_OnPlaybackDevice_String: *mut  char = onPlaybackDevice;
const k_pch_audio_OnRecordDevice_String: *mut  char = onRecordDevice;
const k_pch_audio_OffPlaybackDevice_String: *mut  char = offPlaybackDevice;
const k_pch_audio_OffRecordDevice_String: *mut  char = offRecordDevice;
const k_pch_audio_VIVEHDMIGain: *mut  char = viveHDMIGain;


struct HmdMatrix34_t {
    m: float [3][4],
}

struct HmdMatrix44_t {
    m: float [4][4],
}

struct HmdVector3_t {
    v: float [3],
}

struct HmdVector4_t {
    v: float [4],
}

struct HmdVector3d_t {
    v: double [3],
}

struct HmdVector2_t {
    v: float [2],
}

struct HmdQuaternion_t {
    w: f64,
    x: f64,
    y: f64,
    z: f64,
}

struct HmdColor_t {
    r: f32,
    g: f32,
    b: f32,
    a: f32,
}

struct HmdQuad_t {
    vCorners: HmdVector3_t [4],
}

struct HmdRect2_t {
    vTopLeft: HmdVector2_t,
    vBottomRight: HmdVector2_t,
}

struct DistortionCoordinates_t {
    rfRed: float [2],
    rfGreen: float [2],
    rfBlue: float [2],
}

struct Texture_t {
    handle: *mut libc::c_void,
    eType: enum EGraphicsAPIConvention,
    eColorSpace: enum EColorSpace,
}

struct TrackedDevicePose_t {
    mDeviceToAbsoluteTracking: HmdMatrix34_t,
    vVelocity: HmdVector3_t,
    vAngularVelocity: HmdVector3_t,
    eTrackingResult: enum ETrackingResult,
    bPoseIsValid: _Bool,
    bDeviceIsConnected: _Bool,
}

struct VRTextureBounds_t {
    uMin: f32,
    vMin: f32,
    uMax: f32,
    vMax: f32,
}

struct VREvent_Controller_t {
    button: u32,
}

struct VREvent_Mouse_t {
    x: f32,
    y: f32,
    button: u32,
}

struct VREvent_Scroll_t {
    xdelta: f32,
    ydelta: f32,
    repeatCount: u32,
}

struct VREvent_TouchPadMove_t {
    bFingerDown: _Bool,
    flSecondsFingerDown: f32,
    fValueXFirst: f32,
    fValueYFirst: f32,
    fValueXRaw: f32,
    fValueYRaw: f32,
}

struct VREvent_Notification_t {
    ulUserValue: uint64_t,
    notificationId: u32,
}

struct VREvent_Process_t {
    pid: u32,
    oldPid: u32,
    bForced: _Bool,
}

struct VREvent_Overlay_t {
    overlayHandle: uint64_t,
}

struct VREvent_Status_t {
    statusState: u32,
}

struct VREvent_Keyboard_t {
    cNewInput: char [8],
    uUserValue: uint64_t,
}

struct VREvent_Ipd_t {
    ipdMeters: f32,
}

struct VREvent_Chaperone_t {
    m_nPreviousUniverse: uint64_t,
    m_nCurrentUniverse: uint64_t,
}

struct VREvent_Reserved_t {
    reserved0: uint64_t,
    reserved1: uint64_t,
}

struct VREvent_PerformanceTest_t {
    m_nFidelityLevel: u32,
}

struct VREvent_SeatedZeroPoseReset_t {
    bResetBySystemMenu: _Bool,
}

struct (anonymous) {
    reserved: VREvent_Reserved_t,
    controller: VREvent_Controller_t,
    mouse: VREvent_Mouse_t,
    scroll: VREvent_Scroll_t,
    process: VREvent_Process_t,
    notification: VREvent_Notification_t,
    overlay: VREvent_Overlay_t,
    status: VREvent_Status_t,
    keyboard: VREvent_Keyboard_t,
    ipd: VREvent_Ipd_t,
    chaperone: VREvent_Chaperone_t,
    performanceTest: VREvent_PerformanceTest_t,
    touchPadMove: VREvent_TouchPadMove_t,
    seatedZeroPoseReset: VREvent_SeatedZeroPoseReset_t,
}

struct VREvent_t {
    eventType: u32,
    trackedDeviceIndex: TrackedDeviceIndex_t,
    eventAgeSeconds: f32,
    data: VREvent_Data_t,
}

struct HiddenAreaMesh_t {
    pVertexData: *mut  HmdVector2_t,
    unTriangleCount: u32,
}

struct VRControllerAxis_t {
    x: f32,
    y: f32,
}

struct VRControllerState001_t {
    unPacketNum: u32,
    ulButtonPressed: uint64_t,
    ulButtonTouched: uint64_t,
    rAxis: VRControllerAxis_t [5],
}

struct Compositor_OverlaySettings {
    size: u32,
    curved: _Bool,
    antialias: _Bool,
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
    transform: HmdMatrix44_t,
}

struct AppOverrideKeys_t {
    pchKey: *const c_char,
    pchValue: *const c_char,
}

struct Compositor_FrameTiming {
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
    m_HmdPose: TrackedDevicePose_t,
    m_nFidelityLevel: i32,
}

struct VROverlayIntersectionParams_t {
    vSource: HmdVector3_t,
    vDirection: HmdVector3_t,
    eOrigin: enum ETrackingUniverseOrigin,
}

struct VROverlayIntersectionResults_t {
    vPoint: HmdVector3_t,
    vNormal: HmdVector3_t,
    vUVs: HmdVector2_t,
    fDistance: f32,
}

struct RenderModel_ComponentState_t {
    mTrackingToComponentRenderModel: HmdMatrix34_t,
    mTrackingToComponentLocal: HmdMatrix34_t,
    uProperties: VRComponentProperties,
}

struct RenderModel_Vertex_t {
    vPosition: HmdVector3_t,
    vNormal: HmdVector3_t,
    rfTextureCoord: float [2],
}

struct RenderModel_TextureMap_t {
    unWidth: uint16_t,
    unHeight: uint16_t,
    rubTextureMapData: *const u8,
}

struct RenderModel_t {
    rVertexData: *mut  RenderModel_Vertex_t,
    unVertexCount: u32,
    rIndexData: *const u16,
    unTriangleCount: u32,
    diffuseTextureId: TextureID_t,
}

struct RenderModel_ControllerMode_State_t {
    bScrollWheelVisible: _Bool,
}

struct NotificationBitmap_t {
    bytes: *mut libc::c_void,
    width: i32,
    height: i32,
    depth: i32,
}

struct COpenVRContext {
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
