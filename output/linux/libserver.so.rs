// Generated using https://github.com/a2x/cs2-dumper
// 2024-03-29 21:16:02.890657436 UTC

#![allow(non_upper_case_globals, unused)]

pub mod cs2_dumper {
    pub mod schemas {
        // Module: libserver.so
        // Classes count: 231
        // Enums count: 0
        pub mod libserver {
            // Parent: None
            // Fields count: 1
            //
            // Metadata:
            // MPropertyCustomEditor
            pub mod CRangeFloat {
                pub const m_pValue: usize = 0x0; // float32[2]
            }
            // Parent: None
            // Fields count: 1
            //
            // Metadata:
            // MPropertyCustomEditor
            pub mod CRangeInt {
                pub const m_pValue: usize = 0x0; // int32[2]
            }
            // Parent: None
            // Fields count: 11
            //
            // Metadata:
            // NetworkVarNames: m_nameStringableIndex (int32)
            pub mod CEntityIdentity {
                pub const m_nameStringableIndex: usize = 0x14; // int32
                pub const m_name: usize = 0x18; // CUtlSymbolLarge
                pub const m_designerName: usize = 0x20; // CUtlSymbolLarge
                pub const m_flags: usize = 0x30; // uint32
                pub const m_worldGroupId: usize = 0x38; // WorldGroupId_t
                pub const m_fDataObjectTypes: usize = 0x3C; // uint32
                pub const m_PathIndex: usize = 0x40; // ChangeAccessorFieldPathIndex_t
                pub const m_pPrev: usize = 0x58; // CEntityIdentity*
                pub const m_pNext: usize = 0x60; // CEntityIdentity*
                pub const m_pPrevByClass: usize = 0x68; // CEntityIdentity*
                pub const m_pNextByClass: usize = 0x70; // CEntityIdentity*
            }
            // Parent: None
            // Fields count: 4
            //
            // Metadata:
            // NetworkVarNames: m_pEntity (CEntityIdentity*)
            // NetworkVarNames: m_CScriptComponent (CScriptComponent::Storage_t)
            pub mod CEntityInstance {
                pub const m_iszPrivateVScripts: usize = 0x8; // CUtlSymbolLarge
                pub const m_pEntity: usize = 0x10; // CEntityIdentity*
                pub const m_CScriptComponent: usize = 0x28; // CScriptComponent*
                pub const m_bVisibleinPVS: usize = 0x30; // bool
            }
            // Parent: None
            // Fields count: 2
            //
            // Metadata:
            // NetworkVarNames: m_sceneNode (CGameSceneNode)
            pub mod CBodyComponentPoint {
                pub const m_sceneNode: usize = 0x60; // CGameSceneNode
                pub const __m_pChainEntity: usize = 0x1C0; // CNetworkVarChainer
            }
            // Parent: None
            // Fields count: 2
            //
            // Metadata:
            // NetworkVarNames: m_skeletonInstance (CSkeletonInstance)
            pub mod CBodyComponentSkeletonInstance {
                pub const m_skeletonInstance: usize = 0x60; // CSkeletonInstance
                pub const __m_pChainEntity: usize = 0x450; // CNetworkVarChainer
            }
            // Parent: None
            // Fields count: 1
            //
            // Metadata:
            // NetworkVarNames: m_bvDisabledHitGroups (uint32)
            pub mod CHitboxComponent {
                pub const m_bvDisabledHitGroups: usize = 0x24; // uint32[1]
            }
            // Parent: None
            // Fields count: 67
            //
            // Metadata:
            // NetworkVarNames: m_Color (Color)
            // NetworkVarNames: m_SecondaryColor (Color)
            // NetworkVarNames: m_flBrightness (float)
            // NetworkVarNames: m_flBrightnessScale (float)
            // NetworkVarNames: m_flBrightnessMult (float)
            // NetworkVarNames: m_flRange (float)
            // NetworkVarNames: m_flFalloff (float)
            // NetworkVarNames: m_flAttenuation0 (float)
            // NetworkVarNames: m_flAttenuation1 (float)
            // NetworkVarNames: m_flAttenuation2 (float)
            // NetworkVarNames: m_flTheta (float)
            // NetworkVarNames: m_flPhi (float)
            // NetworkVarNames: m_hLightCookie (HRenderTextureStrong)
            // NetworkVarNames: m_nCascades (int)
            // NetworkVarNames: m_nCastShadows (int)
            // NetworkVarNames: m_nShadowWidth (int)
            // NetworkVarNames: m_nShadowHeight (int)
            // NetworkVarNames: m_bRenderDiffuse (bool)
            // NetworkVarNames: m_nRenderSpecular (int)
            // NetworkVarNames: m_bRenderTransmissive (bool)
            // NetworkVarNames: m_flOrthoLightWidth (float)
            // NetworkVarNames: m_flOrthoLightHeight (float)
            // NetworkVarNames: m_nStyle (int)
            // NetworkVarNames: m_Pattern (CUtlString)
            // NetworkVarNames: m_nCascadeRenderStaticObjects (int)
            // NetworkVarNames: m_flShadowCascadeCrossFade (float)
            // NetworkVarNames: m_flShadowCascadeDistanceFade (float)
            // NetworkVarNames: m_flShadowCascadeDistance0 (float)
            // NetworkVarNames: m_flShadowCascadeDistance1 (float)
            // NetworkVarNames: m_flShadowCascadeDistance2 (float)
            // NetworkVarNames: m_flShadowCascadeDistance3 (float)
            // NetworkVarNames: m_nShadowCascadeResolution0 (int)
            // NetworkVarNames: m_nShadowCascadeResolution1 (int)
            // NetworkVarNames: m_nShadowCascadeResolution2 (int)
            // NetworkVarNames: m_nShadowCascadeResolution3 (int)
            // NetworkVarNames: m_bUsesBakedShadowing (bool)
            // NetworkVarNames: m_nShadowPriority (int)
            // NetworkVarNames: m_nBakedShadowIndex (int)
            // NetworkVarNames: m_bRenderToCubemaps (bool)
            // NetworkVarNames: m_nDirectLight (int)
            // NetworkVarNames: m_nIndirectLight (int)
            // NetworkVarNames: m_flFadeMinDist (float)
            // NetworkVarNames: m_flFadeMaxDist (float)
            // NetworkVarNames: m_flShadowFadeMinDist (float)
            // NetworkVarNames: m_flShadowFadeMaxDist (float)
            // NetworkVarNames: m_bEnabled (bool)
            // NetworkVarNames: m_bFlicker (bool)
            // NetworkVarNames: m_bPrecomputedFieldsValid (bool)
            // NetworkVarNames: m_vPrecomputedBoundsMins (Vector)
            // NetworkVarNames: m_vPrecomputedBoundsMaxs (Vector)
            // NetworkVarNames: m_vPrecomputedOBBOrigin (Vector)
            // NetworkVarNames: m_vPrecomputedOBBAngles (QAngle)
            // NetworkVarNames: m_vPrecomputedOBBExtent (Vector)
            // NetworkVarNames: m_flPrecomputedMaxRange (float)
            // NetworkVarNames: m_nFogLightingMode (int)
            // NetworkVarNames: m_flFogContributionStength (float)
            // NetworkVarNames: m_flNearClipPlane (float)
            // NetworkVarNames: m_SkyColor (Color)
            // NetworkVarNames: m_flSkyIntensity (float)
            // NetworkVarNames: m_SkyAmbientBounce (Color)
            // NetworkVarNames: m_bUseSecondaryColor (bool)
            // NetworkVarNames: m_bMixedShadows (bool)
            // NetworkVarNames: m_flLightStyleStartTime (GameTime_t)
            // NetworkVarNames: m_flCapsuleLength (float)
            // NetworkVarNames: m_flMinRoughness (float)
            pub mod CLightComponent {
                pub const __m_pChainEntity: usize = 0x58; // CNetworkVarChainer
                pub const m_Color: usize = 0x95; // Color
                pub const m_SecondaryColor: usize = 0x99; // Color
                pub const m_flBrightness: usize = 0xA0; // float32
                pub const m_flBrightnessScale: usize = 0xA4; // float32
                pub const m_flBrightnessMult: usize = 0xA8; // float32
                pub const m_flRange: usize = 0xAC; // float32
                pub const m_flFalloff: usize = 0xB0; // float32
                pub const m_flAttenuation0: usize = 0xB4; // float32
                pub const m_flAttenuation1: usize = 0xB8; // float32
                pub const m_flAttenuation2: usize = 0xBC; // float32
                pub const m_flTheta: usize = 0xC0; // float32
                pub const m_flPhi: usize = 0xC4; // float32
                pub const m_hLightCookie: usize = 0xC8; // CStrongHandle<InfoForResourceTypeCTextureBase>
                pub const m_nCascades: usize = 0xD0; // int32
                pub const m_nCastShadows: usize = 0xD4; // int32
                pub const m_nShadowWidth: usize = 0xD8; // int32
                pub const m_nShadowHeight: usize = 0xDC; // int32
                pub const m_bRenderDiffuse: usize = 0xE0; // bool
                pub const m_nRenderSpecular: usize = 0xE4; // int32
                pub const m_bRenderTransmissive: usize = 0xE8; // bool
                pub const m_flOrthoLightWidth: usize = 0xEC; // float32
                pub const m_flOrthoLightHeight: usize = 0xF0; // float32
                pub const m_nStyle: usize = 0xF4; // int32
                pub const m_Pattern: usize = 0xF8; // CUtlString
                pub const m_nCascadeRenderStaticObjects: usize = 0x100; // int32
                pub const m_flShadowCascadeCrossFade: usize = 0x104; // float32
                pub const m_flShadowCascadeDistanceFade: usize = 0x108; // float32
                pub const m_flShadowCascadeDistance0: usize = 0x10C; // float32
                pub const m_flShadowCascadeDistance1: usize = 0x110; // float32
                pub const m_flShadowCascadeDistance2: usize = 0x114; // float32
                pub const m_flShadowCascadeDistance3: usize = 0x118; // float32
                pub const m_nShadowCascadeResolution0: usize = 0x11C; // int32
                pub const m_nShadowCascadeResolution1: usize = 0x120; // int32
                pub const m_nShadowCascadeResolution2: usize = 0x124; // int32
                pub const m_nShadowCascadeResolution3: usize = 0x128; // int32
                pub const m_bUsesBakedShadowing: usize = 0x12C; // bool
                pub const m_nShadowPriority: usize = 0x130; // int32
                pub const m_nBakedShadowIndex: usize = 0x134; // int32
                pub const m_bRenderToCubemaps: usize = 0x138; // bool
                pub const m_nDirectLight: usize = 0x13C; // int32
                pub const m_nIndirectLight: usize = 0x140; // int32
                pub const m_flFadeMinDist: usize = 0x144; // float32
                pub const m_flFadeMaxDist: usize = 0x148; // float32
                pub const m_flShadowFadeMinDist: usize = 0x14C; // float32
                pub const m_flShadowFadeMaxDist: usize = 0x150; // float32
                pub const m_bEnabled: usize = 0x154; // bool
                pub const m_bFlicker: usize = 0x155; // bool
                pub const m_bPrecomputedFieldsValid: usize = 0x156; // bool
                pub const m_vPrecomputedBoundsMins: usize = 0x158; // Vector
                pub const m_vPrecomputedBoundsMaxs: usize = 0x164; // Vector
                pub const m_vPrecomputedOBBOrigin: usize = 0x170; // Vector
                pub const m_vPrecomputedOBBAngles: usize = 0x17C; // QAngle
                pub const m_vPrecomputedOBBExtent: usize = 0x188; // Vector
                pub const m_flPrecomputedMaxRange: usize = 0x194; // float32
                pub const m_nFogLightingMode: usize = 0x198; // int32
                pub const m_flFogContributionStength: usize = 0x19C; // float32
                pub const m_flNearClipPlane: usize = 0x1A0; // float32
                pub const m_SkyColor: usize = 0x1A4; // Color
                pub const m_flSkyIntensity: usize = 0x1A8; // float32
                pub const m_SkyAmbientBounce: usize = 0x1AC; // Color
                pub const m_bUseSecondaryColor: usize = 0x1B0; // bool
                pub const m_bMixedShadows: usize = 0x1B1; // bool
                pub const m_flLightStyleStartTime: usize = 0x1B4; // GameTime_t
                pub const m_flCapsuleLength: usize = 0x1B8; // float32
                pub const m_flMinRoughness: usize = 0x1BC; // float32
                pub const m_bPvsModifyEntity: usize = 0x1D0; // bool
            }
            // Parent: None
            // Fields count: 1
            //
            // Metadata:
            // MPropertyCustomEditor
            pub mod CSkillFloat {
                pub const m_pValue: usize = 0x0; // float32[4]
            }
            // Parent: None
            // Fields count: 1
            //
            // Metadata:
            // MPropertyCustomEditor
            pub mod CSkillInt {
                pub const m_pValue: usize = 0x0; // int32[4]
            }
            // Parent: None
            // Fields count: 2
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod CSkillDamage {
                pub const m_flDamage: usize = 0x0; // CSkillFloat
                pub const m_flPhysicsForceDamage: usize = 0x10; // float32
            }
            // Parent: None
            // Fields count: 1
            //
            // Metadata:
            // MPropertyCustomEditor
            pub mod CRemapFloat {
                pub const m_pValue: usize = 0x0; // float32[4]
            }
            // Parent: None
            // Fields count: 0
            //
            // Metadata:
            // MPulseProvideFeatureTag
            // MPulseLibraryBindings
            pub mod CBasePlayerControllerAPI {
            }
            // Parent: None
            // Fields count: 3
            //
            // Metadata:
            // NetworkVarNames: nType (FixAngleSet_t)
            // NetworkVarNames: qAngle (QAngle)
            // NetworkVarNames: nIndex (uint32)
            pub mod ViewAngleServerChange_t {
                pub const nType: usize = 0x30; // FixAngleSet_t
                pub const qAngle: usize = 0x34; // QAngle
                pub const nIndex: usize = 0x40; // uint32
            }
            // Parent: None
            // Fields count: 12
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod CommandToolCommand_t {
                pub const m_bEnabled: usize = 0x0; // bool
                pub const m_bOpened: usize = 0x1; // bool
                pub const m_InternalId: usize = 0x4; // uint32
                pub const m_ShortName: usize = 0x8; // CUtlString
                pub const m_ExecMode: usize = 0x10; // CommandExecMode_t
                pub const m_SpawnGroup: usize = 0x18; // CUtlString
                pub const m_PeriodicExecDelay: usize = 0x20; // float32
                pub const m_SpecType: usize = 0x24; // CommandEntitySpecType_t
                pub const m_EntitySpec: usize = 0x28; // CUtlString
                pub const m_Commands: usize = 0x30; // CUtlString
                pub const m_SetDebugBits: usize = 0x38; // DebugOverlayBits_t
                pub const m_ClearDebugBits: usize = 0x40; // DebugOverlayBits_t
            }
            // Parent: None
            // Fields count: 0
            //
            // Metadata:
            // MPulseProvideFeatureTag
            // MPulseLibraryBindings
            pub mod CDynamicPropAPI {
            }
            // Parent: None
            // Fields count: 5
            //
            // Metadata:
            // NetworkVarNames: localSound (Vector)
            // NetworkVarNames: soundscapeIndex (int32)
            // NetworkVarNames: localBits (uint8)
            // NetworkVarNames: soundscapeEntityListIndex (int)
            // NetworkVarNames: soundEventHash (uint32)
            pub mod audioparams_t {
                pub const localSound: usize = 0x8; // Vector[8]
                pub const soundscapeIndex: usize = 0x68; // int32
                pub const localBits: usize = 0x6C; // uint8
                pub const soundscapeEntityListIndex: usize = 0x70; // int32
                pub const soundEventHash: usize = 0x74; // uint32
            }
            // Parent: None
            // Fields count: 12
            //
            // Metadata:
            // NetworkVarNames: m_vecCsViewPunchAngle (QAngle)
            // NetworkVarNames: m_nCsViewPunchAngleTick (GameTick_t)
            // NetworkVarNames: m_flCsViewPunchAngleTickRatio (float32)
            // NetworkVarNames: m_PlayerFog (fogplayerparams_t)
            // NetworkVarNames: m_hColorCorrectionCtrl (CHandle<CColorCorrection>)
            // NetworkVarNames: m_hViewEntity (CHandle<CBaseEntity>)
            // NetworkVarNames: m_hTonemapController (CHandle<CTonemapController2>)
            // NetworkVarNames: m_audio (audioparams_t)
            // NetworkVarNames: m_PostProcessingVolumes (CHandle<CPostProcessingVolume>)
            pub mod CPlayer_CameraServices {
                pub const m_vecCsViewPunchAngle: usize = 0x40; // QAngle
                pub const m_nCsViewPunchAngleTick: usize = 0x4C; // GameTick_t
                pub const m_flCsViewPunchAngleTickRatio: usize = 0x50; // float32
                pub const m_PlayerFog: usize = 0x58; // fogplayerparams_t
                pub const m_hColorCorrectionCtrl: usize = 0x98; // CHandle<CColorCorrection>
                pub const m_hViewEntity: usize = 0x9C; // CHandle<CBaseEntity>
                pub const m_hTonemapController: usize = 0xA0; // CHandle<CTonemapController2>
                pub const m_audio: usize = 0xA8; // audioparams_t
                pub const m_PostProcessingVolumes: usize = 0x120; // CNetworkUtlVectorBase<CHandle<CPostProcessingVolume>>
                pub const m_flOldPlayerZ: usize = 0x138; // float32
                pub const m_flOldPlayerViewOffsetZ: usize = 0x13C; // float32
                pub const m_hTriggerSoundscapeList: usize = 0x158; // CUtlVector<CHandle<CEnvSoundscapeTriggerable>>
            }
            // Parent: None
            // Fields count: 15
            //
            // Metadata:
            // NetworkVarNames: m_nToggleButtonDownMask (ButtonBitMask_t)
            // NetworkVarNames: m_flMaxspeed (float32)
            // NetworkVarNames: m_arrForceSubtickMoveWhen (float32)
            pub mod CPlayer_MovementServices {
                pub const m_nImpulse: usize = 0x40; // int32
                pub const m_nButtons: usize = 0x48; // CInButtonState
                pub const m_nQueuedButtonDownMask: usize = 0x68; // uint64
                pub const m_nQueuedButtonChangeMask: usize = 0x70; // uint64
                pub const m_nButtonDoublePressed: usize = 0x78; // uint64
                pub const m_pButtonPressedCmdNumber: usize = 0x80; // uint32[64]
                pub const m_nLastCommandNumberProcessed: usize = 0x180; // uint32
                pub const m_nToggleButtonDownMask: usize = 0x188; // uint64
                pub const m_flMaxspeed: usize = 0x198; // float32
                pub const m_arrForceSubtickMoveWhen: usize = 0x19C; // float32[4]
                pub const m_flForwardMove: usize = 0x1AC; // float32
                pub const m_flLeftMove: usize = 0x1B0; // float32
                pub const m_flUpMove: usize = 0x1B4; // float32
                pub const m_vecLastMovementImpulses: usize = 0x1B8; // Vector
                pub const m_vecOldViewAngles: usize = 0x1C4; // QAngle
            }
            // Parent: CPlayer_MovementServices
            // Fields count: 14
            //
            // Metadata:
            // NetworkVarNames: m_flFallVelocity (float32)
            // NetworkVarNames: m_bInCrouch (bool)
            // NetworkVarNames: m_nCrouchState (uint32)
            // NetworkVarNames: m_flCrouchTransitionStartTime (GameTime_t)
            // NetworkVarNames: m_bDucked (bool)
            // NetworkVarNames: m_bDucking (bool)
            // NetworkVarNames: m_bInDuckJump (bool)
            pub mod CPlayer_MovementServices_Humanoid {
                pub const m_flStepSoundTime: usize = 0x1D8; // float32
                pub const m_flFallVelocity: usize = 0x1DC; // float32
                pub const m_bInCrouch: usize = 0x1E0; // bool
                pub const m_nCrouchState: usize = 0x1E4; // uint32
                pub const m_flCrouchTransitionStartTime: usize = 0x1E8; // GameTime_t
                pub const m_bDucked: usize = 0x1EC; // bool
                pub const m_bDucking: usize = 0x1ED; // bool
                pub const m_bInDuckJump: usize = 0x1EE; // bool
                pub const m_groundNormal: usize = 0x1F0; // Vector
                pub const m_flSurfaceFriction: usize = 0x1FC; // float32
                pub const m_surfaceProps: usize = 0x200; // CUtlStringToken
                pub const m_nStepside: usize = 0x210; // int32
                pub const m_iTargetVolume: usize = 0x214; // int32
                pub const m_vecSmoothedVelocity: usize = 0x218; // Vector
            }
            // Parent: None
            // Fields count: 4
            //
            // Metadata:
            // NetworkVarNames: m_iObserverMode (uint8)
            // NetworkVarNames: m_hObserverTarget (CHandle<CBaseEntity>)
            pub mod CPlayer_ObserverServices {
                pub const m_iObserverMode: usize = 0x40; // uint8
                pub const m_hObserverTarget: usize = 0x44; // CHandle<CBaseEntity>
                pub const m_iObserverLastMode: usize = 0x48; // ObserverMode_t
                pub const m_bForcedObserverMode: usize = 0x4C; // bool
            }
            // Parent: None
            // Fields count: 5
            //
            // Metadata:
            // NetworkVarNames: m_hMyWeapons (CHandle<CBasePlayerWeapon>)
            // NetworkVarNames: m_hActiveWeapon (CHandle<CBasePlayerWeapon>)
            // NetworkVarNames: m_hLastWeapon (CHandle<CBasePlayerWeapon>)
            // NetworkVarNames: m_iAmmo (uint16)
            pub mod CPlayer_WeaponServices {
                pub const m_hMyWeapons: usize = 0x40; // CNetworkUtlVectorBase<CHandle<CBasePlayerWeapon>>
                pub const m_hActiveWeapon: usize = 0x58; // CHandle<CBasePlayerWeapon>
                pub const m_hLastWeapon: usize = 0x5C; // CHandle<CBasePlayerWeapon>
                pub const m_iAmmo: usize = 0x60; // uint16[32]
                pub const m_bPreventWeaponPickup: usize = 0xA0; // bool
            }
            // Parent: None
            // Fields count: 5
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod AmmoTypeInfo_t {
                pub const m_nMaxCarry: usize = 0x10; // int32
                pub const m_nSplashSize: usize = 0x1C; // CRangeInt
                pub const m_nFlags: usize = 0x24; // AmmoFlags_t
                pub const m_flMass: usize = 0x28; // float32
                pub const m_flSpeed: usize = 0x2C; // CRangeFloat
            }
            // Parent: CBodyComponentSkeletonInstance
            // Fields count: 2
            //
            // Metadata:
            // NetworkVarNames: m_animationController (CBaseAnimGraphController)
            pub mod CBodyComponentBaseAnimGraph {
                pub const m_animationController: usize = 0x478; // CBaseAnimGraphController
                pub const __m_pChainEntity: usize = 0x980; // CNetworkVarChainer
            }
            // Parent: None
            // Fields count: 2
            //
            // Metadata:
            // NetworkVarNames: m_ID (CUtlStringToken)
            // NetworkVarNames: m_Values (Vector4D)
            pub mod EntityRenderAttribute_t {
                pub const m_ID: usize = 0x30; // CUtlStringToken
                pub const m_Values: usize = 0x34; // Vector4D
            }
            // Parent: None
            // Fields count: 4
            //
            // Metadata:
            // NetworkVarNames: m_Handle (ModelConfigHandle_t)
            // NetworkVarNames: m_Name (string_t)
            // NetworkVarNames: m_AssociatedEntities (CHandle<CBaseModelEntity>)
            // NetworkVarNames: m_AssociatedEntityNames (string_t)
            pub mod ActiveModelConfig_t {
                pub const m_Handle: usize = 0x28; // ModelConfigHandle_t
                pub const m_Name: usize = 0x30; // CUtlSymbolLarge
                pub const m_AssociatedEntities: usize = 0x38; // CNetworkUtlVectorBase<CHandle<CBaseModelEntity>>
                pub const m_AssociatedEntityNames: usize = 0x50; // CNetworkUtlVectorBase<CUtlSymbolLarge>
            }
            // Parent: None
            // Fields count: 2
            //
            // Metadata:
            // NetworkVarNames: m_hOwner (CEntityHandle)
            // NetworkVarNames: m_name (CUtlStringToken)
            pub mod CGameSceneNodeHandle {
                pub const m_hOwner: usize = 0x8; // CEntityHandle
                pub const m_name: usize = 0xC; // CUtlStringToken
            }
            // Parent: None
            // Fields count: 32
            //
            // Metadata:
            // NetworkVarNames: m_hParent (CGameSceneNodeHandle)
            // NetworkVarNames: m_vecOrigin (CNetworkOriginCellCoordQuantizedVector)
            // NetworkVarNames: m_angRotation (QAngle)
            // NetworkVarNames: m_flScale (float)
            // NetworkVarNames: m_name (CUtlStringToken)
            // NetworkVarNames: m_hierarchyAttachName (CUtlStringToken)
            pub mod CGameSceneNode {
                pub const m_nodeToWorld: usize = 0x10; // CTransform
                pub const m_pOwner: usize = 0x30; // CEntityInstance*
                pub const m_pParent: usize = 0x38; // CGameSceneNode*
                pub const m_pChild: usize = 0x40; // CGameSceneNode*
                pub const m_pNextSibling: usize = 0x48; // CGameSceneNode*
                pub const m_hParent: usize = 0x70; // CGameSceneNodeHandle
                pub const m_vecOrigin: usize = 0x80; // CNetworkOriginCellCoordQuantizedVector
                pub const m_angRotation: usize = 0xB8; // QAngle
                pub const m_flScale: usize = 0xC4; // float32
                pub const m_vecAbsOrigin: usize = 0xC8; // Vector
                pub const m_angAbsRotation: usize = 0xD4; // QAngle
                pub const m_flAbsScale: usize = 0xE0; // float32
                pub const m_nParentAttachmentOrBone: usize = 0xE4; // int16
                pub const m_bDebugAbsOriginChanges: usize = 0xE6; // bool
                pub const m_bDormant: usize = 0xE7; // bool
                pub const m_bForceParentToBeNetworked: usize = 0xE8; // bool
                pub const m_bDirtyHierarchy: usize = 0x0; // bitfield:1
                pub const m_bDirtyBoneMergeInfo: usize = 0x0; // bitfield:1
                pub const m_bNetworkedPositionChanged: usize = 0x0; // bitfield:1
                pub const m_bNetworkedAnglesChanged: usize = 0x0; // bitfield:1
                pub const m_bNetworkedScaleChanged: usize = 0x0; // bitfield:1
                pub const m_bWillBeCallingPostDataUpdate: usize = 0x0; // bitfield:1
                pub const m_bBoneMergeFlex: usize = 0x0; // bitfield:1
                pub const m_nLatchAbsOrigin: usize = 0x0; // bitfield:2
                pub const m_bDirtyBoneMergeBoneToRoot: usize = 0x0; // bitfield:1
                pub const m_nHierarchicalDepth: usize = 0xEB; // uint8
                pub const m_nHierarchyType: usize = 0xEC; // uint8
                pub const m_nDoNotSetAnimTimeInInvalidatePhysicsCount: usize = 0xED; // uint8
                pub const m_name: usize = 0xF0; // CUtlStringToken
                pub const m_hierarchyAttachName: usize = 0x140; // CUtlStringToken
                pub const m_flZOffset: usize = 0x144; // float32
                pub const m_vRenderOrigin: usize = 0x148; // Vector
            }
            // Parent: None
            // Fields count: 8
            //
            // Metadata:
            // NetworkVarNames: m_hSequence (HSequence)
            // NetworkVarNames: m_flPrevCycle (float32)
            // NetworkVarNames: m_flCycle (float32)
            pub mod CNetworkedSequenceOperation {
                pub const m_hSequence: usize = 0x8; // HSequence
                pub const m_flPrevCycle: usize = 0xC; // float32
                pub const m_flCycle: usize = 0x10; // float32
                pub const m_flWeight: usize = 0x14; // CNetworkedQuantizedFloat
                pub const m_bSequenceChangeNetworked: usize = 0x1C; // bool
                pub const m_bDiscontinuity: usize = 0x1D; // bool
                pub const m_flPrevCycleFromDiscontinuity: usize = 0x20; // float32
                pub const m_flPrevCycleForAnimEventDetection: usize = 0x24; // float32
            }
            // Parent: None
            // Fields count: 7
            //
            // Metadata:
            // NetworkVarNames: m_hModel (HModelStrong)
            // NetworkVarNames: m_bClientClothCreationSuppressed (bool)
            // NetworkVarNames: m_MeshGroupMask (MeshGroupMask_t)
            // NetworkVarNames: m_nIdealMotionType (int8)
            pub mod CModelState {
                pub const m_hModel: usize = 0xA0; // CStrongHandle<InfoForResourceTypeCModel>
                pub const m_ModelName: usize = 0xA8; // CUtlSymbolLarge
                pub const m_bClientClothCreationSuppressed: usize = 0xE8; // bool
                pub const m_MeshGroupMask: usize = 0x180; // uint64
                pub const m_nIdealMotionType: usize = 0x212; // int8
                pub const m_nForceLOD: usize = 0x213; // int8
                pub const m_nClothUpdateFlags: usize = 0x214; // int8
            }
            // Parent: CGameSceneNode
            // Fields count: 8
            //
            // Metadata:
            // NetworkVarNames: m_modelState (CModelState)
            // NetworkVarNames: m_bIsAnimationEnabled (bool)
            // NetworkVarNames: m_bUseParentRenderBounds (bool)
            // NetworkVarNames: m_materialGroup (CUtlStringToken)
            // NetworkVarNames: m_nHitboxSet (uint8)
            pub mod CSkeletonInstance {
                pub const m_modelState: usize = 0x160; // CModelState
                pub const m_bIsAnimationEnabled: usize = 0x380; // bool
                pub const m_bUseParentRenderBounds: usize = 0x381; // bool
                pub const m_bDisableSolidCollisionsForHierarchy: usize = 0x382; // bool
                pub const m_bDirtyMotionType: usize = 0x0; // bitfield:1
                pub const m_bIsGeneratingLatchedParentSpaceState: usize = 0x0; // bitfield:1
                pub const m_materialGroup: usize = 0x384; // CUtlStringToken
                pub const m_nHitboxSet: usize = 0x388; // uint8
            }
            // Parent: None
            // Fields count: 2
            //
            // Metadata:
            // NetworkVarNames: m_timestamp (GameTime_t)
            // NetworkVarNames: m_nWorldGroupId (WorldGroupId_t)
            pub mod IntervalTimer {
                pub const m_timestamp: usize = 0x8; // GameTime_t
                pub const m_nWorldGroupId: usize = 0xC; // WorldGroupId_t
            }
            // Parent: None
            // Fields count: 4
            //
            // Metadata:
            // NetworkVarNames: m_duration (float32)
            // NetworkVarNames: m_timestamp (GameTime_t)
            // NetworkVarNames: m_timescale (float32)
            // NetworkVarNames: m_nWorldGroupId (WorldGroupId_t)
            pub mod CountdownTimer {
                pub const m_duration: usize = 0x8; // float32
                pub const m_timestamp: usize = 0xC; // GameTime_t
                pub const m_timescale: usize = 0x10; // float32
                pub const m_nWorldGroupId: usize = 0x14; // WorldGroupId_t
            }
            // Parent: None
            // Fields count: 3
            //
            // Metadata:
            // NetworkVarNames: m_duration (float32)
            // NetworkVarNames: m_timestamp (float32)
            // NetworkVarNames: m_timescale (float32)
            pub mod EngineCountdownTimer {
                pub const m_duration: usize = 0x8; // float32
                pub const m_timestamp: usize = 0xC; // float32
                pub const m_timescale: usize = 0x10; // float32
            }
            // Parent: IntervalTimer
            // Fields count: 7
            //
            // Metadata:
            // NetworkVarNames: m_flValues (float)
            // NetworkVarNames: m_nValueCounts (int)
            // NetworkVarNames: m_nBucketCount (int)
            // NetworkVarNames: m_flInterval (float)
            // NetworkVarNames: m_flFinalValue (float)
            // NetworkVarNames: m_nCompressionType (TimelineCompression_t)
            // NetworkVarNames: m_bStopped (bool)
            pub mod CTimeline {
                pub const m_flValues: usize = 0x10; // float32[64]
                pub const m_nValueCounts: usize = 0x110; // int32[64]
                pub const m_nBucketCount: usize = 0x210; // int32
                pub const m_flInterval: usize = 0x214; // float32
                pub const m_flFinalValue: usize = 0x218; // float32
                pub const m_nCompressionType: usize = 0x21C; // TimelineCompression_t
                pub const m_bStopped: usize = 0x220; // bool
            }
            // Parent: None
            // Fields count: 24
            //
            // Metadata:
            // NetworkVarNames: m_PredNetBoolVariables (uint32)
            // NetworkVarNames: m_PredNetByteVariables (byte)
            // NetworkVarNames: m_PredNetUInt16Variables (uint16)
            // NetworkVarNames: m_PredNetIntVariables (int32)
            // NetworkVarNames: m_PredNetUInt32Variables (uint32)
            // NetworkVarNames: m_PredNetUInt64Variables (uint64)
            // NetworkVarNames: m_PredNetFloatVariables (float)
            // NetworkVarNames: m_PredNetVectorVariables (Vector)
            // NetworkVarNames: m_PredNetQuaternionVariables (Quaternion)
            // NetworkVarNames: m_PredNetGlobalSymbolVariables (CGlobalSymbol)
            // NetworkVarNames: m_OwnerOnlyPredNetBoolVariables (uint32)
            // NetworkVarNames: m_OwnerOnlyPredNetByteVariables (byte)
            // NetworkVarNames: m_OwnerOnlyPredNetUInt16Variables (uint16)
            // NetworkVarNames: m_OwnerOnlyPredNetIntVariables (int32)
            // NetworkVarNames: m_OwnerOnlyPredNetUInt32Variables (uint32)
            // NetworkVarNames: m_OwnerOnlyPredNetUInt64Variables (uint64)
            // NetworkVarNames: m_OwnerOnlyPredNetFloatVariables (float)
            // NetworkVarNames: m_OwnerOnlyPredNetVectorVariables (Vector)
            // NetworkVarNames: m_OwnerOnlyPredNetQuaternionVariables (Quaternion)
            // NetworkVarNames: m_OwnerOnlyPredNetGlobalSymbolVariables (CGlobalSymbol)
            // NetworkVarNames: m_nBoolVariablesCount (int)
            // NetworkVarNames: m_nOwnerOnlyBoolVariablesCount (int)
            // NetworkVarNames: m_nRandomSeedOffset (int)
            // NetworkVarNames: m_flLastTeleportTime (float)
            pub mod CAnimGraphNetworkedVariables {
                pub const m_PredNetBoolVariables: usize = 0x8; // CNetworkUtlVectorBase<uint32>
                pub const m_PredNetByteVariables: usize = 0x20; // CNetworkUtlVectorBase<uint8>
                pub const m_PredNetUInt16Variables: usize = 0x38; // CNetworkUtlVectorBase<uint16>
                pub const m_PredNetIntVariables: usize = 0x50; // CNetworkUtlVectorBase<int32>
                pub const m_PredNetUInt32Variables: usize = 0x68; // CNetworkUtlVectorBase<uint32>
                pub const m_PredNetUInt64Variables: usize = 0x80; // CNetworkUtlVectorBase<uint64>
                pub const m_PredNetFloatVariables: usize = 0x98; // CNetworkUtlVectorBase<float32>
                pub const m_PredNetVectorVariables: usize = 0xB0; // CNetworkUtlVectorBase<Vector>
                pub const m_PredNetQuaternionVariables: usize = 0xC8; // CNetworkUtlVectorBase<Quaternion>
                pub const m_PredNetGlobalSymbolVariables: usize = 0xE0; // CNetworkUtlVectorBase<CGlobalSymbol>
                pub const m_OwnerOnlyPredNetBoolVariables: usize = 0xF8; // CNetworkUtlVectorBase<uint32>
                pub const m_OwnerOnlyPredNetByteVariables: usize = 0x110; // CNetworkUtlVectorBase<uint8>
                pub const m_OwnerOnlyPredNetUInt16Variables: usize = 0x128; // CNetworkUtlVectorBase<uint16>
                pub const m_OwnerOnlyPredNetIntVariables: usize = 0x140; // CNetworkUtlVectorBase<int32>
                pub const m_OwnerOnlyPredNetUInt32Variables: usize = 0x158; // CNetworkUtlVectorBase<uint32>
                pub const m_OwnerOnlyPredNetUInt64Variables: usize = 0x170; // CNetworkUtlVectorBase<uint64>
                pub const m_OwnerOnlyPredNetFloatVariables: usize = 0x188; // CNetworkUtlVectorBase<float32>
                pub const m_OwnerOnlyPredNetVectorVariables: usize = 0x1A0; // CNetworkUtlVectorBase<Vector>
                pub const m_OwnerOnlyPredNetQuaternionVariables: usize = 0x1B8; // CNetworkUtlVectorBase<Quaternion>
                pub const m_OwnerOnlyPredNetGlobalSymbolVariables: usize = 0x1D0; // CNetworkUtlVectorBase<CGlobalSymbol>
                pub const m_nBoolVariablesCount: usize = 0x1E8; // int32
                pub const m_nOwnerOnlyBoolVariablesCount: usize = 0x1EC; // int32
                pub const m_nRandomSeedOffset: usize = 0x1F0; // int32
                pub const m_flLastTeleportTime: usize = 0x1F4; // float32
            }
            // Parent: None
            // Fields count: 0
            //
            // Metadata:
            // MPropertyCustomFGDType
            pub mod CFootstepTableHandle {
            }
            // Parent: None
            // Fields count: 0
            //
            // Metadata:
            // MPulseProvideFeatureTag
            // MPulseLibraryBindings
            pub mod CBaseEntityAPI {
            }
            // Parent: None
            // Fields count: 16
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod CGameScriptedMoveData {
                pub const m_vAccumulatedRootMotion: usize = 0x0; // Vector
                pub const m_vDest: usize = 0xC; // Vector
                pub const m_vSrc: usize = 0x18; // Vector
                pub const m_angSrc: usize = 0x24; // QAngle
                pub const m_angDst: usize = 0x30; // QAngle
                pub const m_angCurrent: usize = 0x3C; // QAngle
                pub const m_flAngRate: usize = 0x48; // float32
                pub const m_flDuration: usize = 0x4C; // float32
                pub const m_flStartTime: usize = 0x50; // GameTime_t
                pub const m_bActive: usize = 0x54; // bool
                pub const m_bTeleportOnEnd: usize = 0x55; // bool
                pub const m_bIgnoreRotation: usize = 0x56; // bool
                pub const m_nType: usize = 0x58; // ScriptedMoveType_t
                pub const m_bSuccess: usize = 0x5C; // bool
                pub const m_nForcedCrouchState: usize = 0x60; // ForcedCrouchState_t
                pub const m_bIgnoreCollisions: usize = 0x64; // bool
            }
            // Parent: None
            // Fields count: 0
            //
            // Metadata:
            // MPulseProvideFeatureTag
            // MPulseLibraryBindings
            pub mod CTakeDamageInfoAPI {
            }
            // Parent: None
            // Fields count: 1
            //
            // Metadata:
            // MPulseInstanceDomainInfo
            // MPulseLibraryBindings
            // MPulseDomainOptInFeatureTag
            pub mod CPulseGraphInstance_ServerEntity {
                pub const m_pComponent: usize = 0xD8; // CPulseGraphComponentBase*
            }
            // Parent: None
            // Fields count: 0
            //
            // Metadata:
            // MPulseLibraryBindings
            // MPropertyFriendlyName
            // MPropertyDescription
            pub mod CPulseServerFuncs {
            }
            // Parent: CPulseCell_BaseFlow
            // Fields count: 1
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MCellForDomain
            // MPulseCellMethodBindings
            // MPropertyFriendlyName
            // MPropertyDescription
            pub mod CPulseCell_SoundEventStart {
                pub const m_Type: usize = 0x48; // SoundEventStartType_t
            }
            // Parent: None
            // Fields count: 0
            //
            // Metadata:
            // MPulseLibraryBindings
            // MPropertyFriendlyName
            // MPropertyDescription
            pub mod CPulseServerFuncs_Sounds {
            }
            // Parent: CPulseCell_BaseYieldingInflow
            // Fields count: 3
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod CPulseCell_Outflow_PlaySceneBase {
                pub const m_OnFinished: usize = 0x48; // CPulse_ResumePoint
                pub const m_OnCanceled: usize = 0x58; // CPulse_ResumePoint
                pub const m_Triggers: usize = 0x68; // CUtlVector<CPulse_OutflowConnection>
            }
            // Parent: None
            // Fields count: 9
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod PulseScriptedSequenceData_t {
                pub const m_nActorID: usize = 0x0; // int32
                pub const m_szPreIdleSequence: usize = 0x8; // CUtlString
                pub const m_szEntrySequence: usize = 0x10; // CUtlString
                pub const m_szSequence: usize = 0x18; // CUtlString
                pub const m_szExitSequence: usize = 0x20; // CUtlString
                pub const m_bLoopPreIdleSequence: usize = 0x28; // bool
                pub const m_bLoopActionSequence: usize = 0x29; // bool
                pub const m_bLoopPostIdleSequence: usize = 0x2A; // bool
                pub const m_bIgnoreLookAt: usize = 0x2B; // bool
            }
            // Parent: CEntityInstance
            // Fields count: 75
            //
            // Metadata:
            // MNetworkExcludeByName
            // MNetworkExcludeByName
            // MNetworkExcludeByUserGroup
            // MNetworkExcludeByUserGroup
            // MNetworkExcludeByUserGroup
            // MNetworkExcludeByName
            // MNetworkExcludeByName
            // MNetworkExcludeByName
            // NetworkVarNames: m_CBodyComponent (CBodyComponent::Storage_t)
            // NetworkVarNames: m_iHealth (int32)
            // NetworkVarNames: m_iMaxHealth (int32)
            // NetworkVarNames: m_lifeState (uint8)
            // NetworkVarNames: m_bTakesDamage (bool)
            // NetworkVarNames: m_nTakeDamageFlags (TakeDamageFlags_t)
            // NetworkVarNames: m_bIsPlatform (bool)
            // NetworkVarNames: m_MoveCollide (MoveCollide_t)
            // NetworkVarNames: m_MoveType (MoveType_t)
            // NetworkVarNames: m_nSubclassID (EntitySubclassID_t)
            // NetworkVarNames: m_flAnimTime (float32)
            // NetworkVarNames: m_flSimulationTime (float32)
            // NetworkVarNames: m_flCreateTime (GameTime_t)
            // NetworkVarNames: m_bClientSideRagdoll (bool)
            // NetworkVarNames: m_ubInterpolationFrame (uint8)
            // NetworkVarNames: m_iTeamNum (uint8)
            // NetworkVarNames: m_spawnflags (uint32)
            // NetworkVarNames: m_nNextThinkTick (GameTick_t)
            // NetworkVarNames: m_fFlags (uint32)
            // NetworkVarNames: m_vecVelocity (CNetworkVelocityVector)
            // NetworkVarNames: m_vecBaseVelocity (Vector)
            // NetworkVarNames: m_hEffectEntity (CHandle<CBaseEntity>)
            // NetworkVarNames: m_hOwnerEntity (CHandle<CBaseEntity>)
            // NetworkVarNames: m_fEffects (uint32)
            // NetworkVarNames: m_hGroundEntity (CHandle<CBaseEntity>)
            // NetworkVarNames: m_flFriction (float32)
            // NetworkVarNames: m_flElasticity (float32)
            // NetworkVarNames: m_flGravityScale (float32)
            // NetworkVarNames: m_flTimeScale (float32)
            // NetworkVarNames: m_flWaterLevel (float)
            // NetworkVarNames: m_bAnimatedEveryTick (bool)
            // NetworkVarNames: m_flNavIgnoreUntilTime (GameTime_t)
            pub mod CBaseEntity {
                pub const m_CBodyComponent: usize = 0x38; // CBodyComponent*
                pub const m_NetworkTransmitComponent: usize = 0x40; // CNetworkTransmitComponent
                pub const m_aThinkFunctions: usize = 0x4F0; // CUtlVector<thinkfunc_t>
                pub const m_iCurrentThinkContext: usize = 0x508; // int32
                pub const m_nLastThinkTick: usize = 0x50C; // GameTick_t
                pub const m_nDisableContextThinkStartTick: usize = 0x510; // GameTick_t
                pub const m_isSteadyState: usize = 0x520; // CBitVec<64>
                pub const m_lastNetworkChange: usize = 0x528; // float32
                pub const m_ResponseContexts: usize = 0x540; // CUtlVector<ResponseContext_t>
                pub const m_iszResponseContext: usize = 0x558; // CUtlSymbolLarge
                pub const m_iHealth: usize = 0x5A0; // int32
                pub const m_iMaxHealth: usize = 0x5A4; // int32
                pub const m_lifeState: usize = 0x5A8; // uint8
                pub const m_flDamageAccumulator: usize = 0x5AC; // float32
                pub const m_bTakesDamage: usize = 0x5B0; // bool
                pub const m_nTakeDamageFlags: usize = 0x5B4; // TakeDamageFlags_t
                pub const m_bIsPlatform: usize = 0x5B8; // bool
                pub const m_MoveCollide: usize = 0x5BA; // MoveCollide_t
                pub const m_MoveType: usize = 0x5BB; // MoveType_t
                pub const m_nActualMoveType: usize = 0x5BC; // MoveType_t
                pub const m_nWaterTouch: usize = 0x5BD; // uint8
                pub const m_nSlimeTouch: usize = 0x5BE; // uint8
                pub const m_bRestoreInHierarchy: usize = 0x5BF; // bool
                pub const m_target: usize = 0x5C0; // CUtlSymbolLarge
                pub const m_hDamageFilter: usize = 0x5C8; // CHandle<CBaseFilter>
                pub const m_iszDamageFilterName: usize = 0x5D0; // CUtlSymbolLarge
                pub const m_flMoveDoneTime: usize = 0x5D8; // float32
                pub const m_nSubclassID: usize = 0x5DC; // CUtlStringToken
                pub const m_flAnimTime: usize = 0x5E8; // float32
                pub const m_flSimulationTime: usize = 0x5EC; // float32
                pub const m_flCreateTime: usize = 0x5F0; // GameTime_t
                pub const m_bClientSideRagdoll: usize = 0x5F4; // bool
                pub const m_ubInterpolationFrame: usize = 0x5F5; // uint8
                pub const m_vPrevVPhysicsUpdatePos: usize = 0x5F8; // Vector
                pub const m_iTeamNum: usize = 0x604; // uint8
                pub const m_iGlobalname: usize = 0x608; // CUtlSymbolLarge
                pub const m_iSentToClients: usize = 0x610; // int32
                pub const m_flSpeed: usize = 0x614; // float32
                pub const m_sUniqueHammerID: usize = 0x618; // CUtlString
                pub const m_spawnflags: usize = 0x620; // uint32
                pub const m_nNextThinkTick: usize = 0x624; // GameTick_t
                pub const m_nSimulationTick: usize = 0x628; // int32
                pub const m_OnKilled: usize = 0x630; // CEntityIOOutput
                pub const m_fFlags: usize = 0x658; // uint32
                pub const m_vecAbsVelocity: usize = 0x65C; // Vector
                pub const m_vecVelocity: usize = 0x668; // CNetworkVelocityVector
                pub const m_vecBaseVelocity: usize = 0x698; // Vector
                pub const m_nPushEnumCount: usize = 0x6A4; // int32
                pub const m_pCollision: usize = 0x6A8; // CCollisionProperty*
                pub const m_hEffectEntity: usize = 0x6B0; // CHandle<CBaseEntity>
                pub const m_hOwnerEntity: usize = 0x6B4; // CHandle<CBaseEntity>
                pub const m_fEffects: usize = 0x6B8; // uint32
                pub const m_hGroundEntity: usize = 0x6BC; // CHandle<CBaseEntity>
                pub const m_flFriction: usize = 0x6C0; // float32
                pub const m_flElasticity: usize = 0x6C4; // float32
                pub const m_flGravityScale: usize = 0x6C8; // float32
                pub const m_flTimeScale: usize = 0x6CC; // float32
                pub const m_flWaterLevel: usize = 0x6D0; // float32
                pub const m_bAnimatedEveryTick: usize = 0x6D4; // bool
                pub const m_bDisableLowViolence: usize = 0x6D5; // bool
                pub const m_nWaterType: usize = 0x6D6; // uint8
                pub const m_iEFlags: usize = 0x6D8; // int32
                pub const m_OnUser1: usize = 0x6E0; // CEntityIOOutput
                pub const m_OnUser2: usize = 0x708; // CEntityIOOutput
                pub const m_OnUser3: usize = 0x730; // CEntityIOOutput
                pub const m_OnUser4: usize = 0x758; // CEntityIOOutput
                pub const m_iInitialTeamNum: usize = 0x780; // int32
                pub const m_flNavIgnoreUntilTime: usize = 0x784; // GameTime_t
                pub const m_vecAngVelocity: usize = 0x788; // QAngle
                pub const m_bNetworkQuantizeOriginAndAngles: usize = 0x794; // bool
                pub const m_bLagCompensate: usize = 0x795; // bool
                pub const m_flOverriddenFriction: usize = 0x798; // float32
                pub const m_pBlocker: usize = 0x79C; // CHandle<CBaseEntity>
                pub const m_flLocalTime: usize = 0x7A0; // float32
                pub const m_flVPhysicsUpdateLocalTime: usize = 0x7A4; // float32
            }
            // Parent: CBaseEntity
            // Fields count: 17
            //
            // Metadata:
            // MNetworkIncludeByUserGroup
            // NetworkVarNames: m_flFadeInDuration (float32)
            // NetworkVarNames: m_flFadeOutDuration (float32)
            // NetworkVarNames: m_flMaxWeight (float32)
            // NetworkVarNames: m_bEnabled (bool)
            // NetworkVarNames: m_bMaster (bool)
            // NetworkVarNames: m_bClientSide (bool)
            // NetworkVarNames: m_bExclusive (bool)
            // NetworkVarNames: m_MinFalloff (float32)
            // NetworkVarNames: m_MaxFalloff (float32)
            // NetworkVarNames: m_flCurWeight (float32)
            // NetworkVarNames: m_netlookupFilename (char)
            pub mod CColorCorrection {
                pub const m_flFadeInDuration: usize = 0x7A8; // float32
                pub const m_flFadeOutDuration: usize = 0x7AC; // float32
                pub const m_flStartFadeInWeight: usize = 0x7B0; // float32
                pub const m_flStartFadeOutWeight: usize = 0x7B4; // float32
                pub const m_flTimeStartFadeIn: usize = 0x7B8; // GameTime_t
                pub const m_flTimeStartFadeOut: usize = 0x7BC; // GameTime_t
                pub const m_flMaxWeight: usize = 0x7C0; // float32
                pub const m_bStartDisabled: usize = 0x7C4; // bool
                pub const m_bEnabled: usize = 0x7C5; // bool
                pub const m_bMaster: usize = 0x7C6; // bool
                pub const m_bClientSide: usize = 0x7C7; // bool
                pub const m_bExclusive: usize = 0x7C8; // bool
                pub const m_MinFalloff: usize = 0x7CC; // float32
                pub const m_MaxFalloff: usize = 0x7D0; // float32
                pub const m_flCurWeight: usize = 0x7D4; // float32
                pub const m_netlookupFilename: usize = 0x7D8; // char[512]
                pub const m_lookupFilename: usize = 0x9D8; // CUtlSymbolLarge
            }
            // Parent: CBaseEntity
            // Fields count: 11
            //
            // Metadata:
            // NetworkVarNames: m_hEntAttached (CHandle<CBaseEntity>)
            // NetworkVarNames: m_bCheapEffect (bool)
            pub mod CEntityFlame {
                pub const m_hEntAttached: usize = 0x7A8; // CHandle<CBaseEntity>
                pub const m_bCheapEffect: usize = 0x7AC; // bool
                pub const m_flSize: usize = 0x7B0; // float32
                pub const m_bUseHitboxes: usize = 0x7B4; // bool
                pub const m_iNumHitboxFires: usize = 0x7B8; // int32
                pub const m_flHitboxFireScale: usize = 0x7BC; // float32
                pub const m_flLifetime: usize = 0x7C0; // GameTime_t
                pub const m_hAttacker: usize = 0x7C4; // CHandle<CBaseEntity>
                pub const m_iDangerSound: usize = 0x7C8; // int32
                pub const m_flDirectDamagePerSecond: usize = 0x7CC; // float32
                pub const m_iCustomDamageType: usize = 0x7D0; // int32
            }
            // Parent: CBaseEntity
            // Fields count: 4
            //
            // Metadata:
            // NetworkVarNames: m_flScale (float32)
            // NetworkVarNames: m_flStartScale (float32)
            // NetworkVarNames: m_flScaleTime (float32)
            // NetworkVarNames: m_nFlags (uint32)
            pub mod CBaseFire {
                pub const m_flScale: usize = 0x7A8; // float32
                pub const m_flStartScale: usize = 0x7AC; // float32
                pub const m_flScaleTime: usize = 0x7B0; // float32
                pub const m_nFlags: usize = 0x7B4; // uint32
            }
            // Parent: CBaseFire
            // Fields count: 2
            //
            // Metadata:
            // NetworkVarNames: m_nFlameModelIndex (int32)
            // NetworkVarNames: m_nFlameFromAboveModelIndex (int32)
            pub mod CFireSmoke {
                pub const m_nFlameModelIndex: usize = 0x7B8; // int32
                pub const m_nFlameFromAboveModelIndex: usize = 0x7BC; // int32
            }
            // Parent: CBaseEntity
            // Fields count: 4
            //
            // Metadata:
            // NetworkVarNames: m_skyboxData (sky3dparams_t)
            // NetworkVarNames: m_skyboxSlotToken (CUtlStringToken)
            pub mod CSkyCamera {
                pub const m_skyboxData: usize = 0x7A8; // sky3dparams_t
                pub const m_skyboxSlotToken: usize = 0x838; // CUtlStringToken
                pub const m_bUseAngles: usize = 0x83C; // bool
                pub const m_pNext: usize = 0x840; // CSkyCamera*
            }
            // Parent: None
            // Fields count: 2
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod CNavLinkAnimgraphVar {
                pub const m_strAnimgraphVar: usize = 0x0; // CUtlString
                pub const m_unAlignmentDegrees: usize = 0x8; // uint32
            }
            // Parent: None
            // Fields count: 8
            //
            // Metadata:
            // NetworkVarNames: m_nInteractsAs (uint64)
            // NetworkVarNames: m_nInteractsWith (uint64)
            // NetworkVarNames: m_nInteractsExclude (uint64)
            // NetworkVarNames: m_nEntityId (uint32)
            // NetworkVarNames: m_nOwnerId (uint32)
            // NetworkVarNames: m_nHierarchyId (uint16)
            // NetworkVarNames: m_nCollisionGroup (uint8)
            // NetworkVarNames: m_nCollisionFunctionMask (uint8)
            pub mod VPhysicsCollisionAttribute_t {
                pub const m_nInteractsAs: usize = 0x8; // uint64
                pub const m_nInteractsWith: usize = 0x10; // uint64
                pub const m_nInteractsExclude: usize = 0x18; // uint64
                pub const m_nEntityId: usize = 0x20; // uint32
                pub const m_nOwnerId: usize = 0x24; // uint32
                pub const m_nHierarchyId: usize = 0x28; // uint16
                pub const m_nCollisionGroup: usize = 0x2A; // uint8
                pub const m_nCollisionFunctionMask: usize = 0x2B; // uint8
            }
            // Parent: None
            // Fields count: 17
            //
            // Metadata:
            // NetworkVarNames: m_collisionAttribute (VPhysicsCollisionAttribute_t)
            // NetworkVarNames: m_vecMins (Vector)
            // NetworkVarNames: m_vecMaxs (Vector)
            // NetworkVarNames: m_usSolidFlags (uint8)
            // NetworkVarNames: m_nSolidType (SolidType_t)
            // NetworkVarNames: m_triggerBloat (uint8)
            // NetworkVarNames: m_nSurroundType (SurroundingBoundsType_t)
            // NetworkVarNames: m_CollisionGroup (uint8)
            // NetworkVarNames: m_nEnablePhysics (uint8)
            // NetworkVarNames: m_vecSpecifiedSurroundingMins (Vector)
            // NetworkVarNames: m_vecSpecifiedSurroundingMaxs (Vector)
            // NetworkVarNames: m_vCapsuleCenter1 (Vector)
            // NetworkVarNames: m_vCapsuleCenter2 (Vector)
            // NetworkVarNames: m_flCapsuleRadius (float)
            pub mod CCollisionProperty {
                pub const m_collisionAttribute: usize = 0x10; // VPhysicsCollisionAttribute_t
                pub const m_vecMins: usize = 0x40; // Vector
                pub const m_vecMaxs: usize = 0x4C; // Vector
                pub const m_usSolidFlags: usize = 0x5A; // uint8
                pub const m_nSolidType: usize = 0x5B; // SolidType_t
                pub const m_triggerBloat: usize = 0x5C; // uint8
                pub const m_nSurroundType: usize = 0x5D; // SurroundingBoundsType_t
                pub const m_CollisionGroup: usize = 0x5E; // uint8
                pub const m_nEnablePhysics: usize = 0x5F; // uint8
                pub const m_flBoundingRadius: usize = 0x60; // float32
                pub const m_vecSpecifiedSurroundingMins: usize = 0x64; // Vector
                pub const m_vecSpecifiedSurroundingMaxs: usize = 0x70; // Vector
                pub const m_vecSurroundingMaxs: usize = 0x7C; // Vector
                pub const m_vecSurroundingMins: usize = 0x88; // Vector
                pub const m_vCapsuleCenter1: usize = 0x94; // Vector
                pub const m_vCapsuleCenter2: usize = 0xA0; // Vector
                pub const m_flCapsuleRadius: usize = 0xAC; // float32
            }
            // Parent: None
            // Fields count: 21
            //
            // Metadata:
            // NetworkVarNames: m_vOrigin (Vector)
            // NetworkVarNames: m_vStart (Vector)
            // NetworkVarNames: m_vNormal (Vector)
            // NetworkVarNames: m_vAngles (QAngle)
            // NetworkVarNames: m_hEntity (CEntityHandle)
            // NetworkVarNames: m_hOtherEntity (CEntityHandle)
            // NetworkVarNames: m_flScale (float32)
            // NetworkVarNames: m_flMagnitude (float32)
            // NetworkVarNames: m_flRadius (float32)
            // NetworkVarNames: m_nSurfaceProp (CUtlStringToken)
            // NetworkVarNames: m_nEffectIndex (HParticleSystemDefinition)
            // NetworkVarNames: m_nDamageType (uint32)
            // NetworkVarNames: m_nPenetrate (uint8)
            // NetworkVarNames: m_nMaterial (uint16)
            // NetworkVarNames: m_nHitBox (uint16)
            // NetworkVarNames: m_nColor (uint8)
            // NetworkVarNames: m_fFlags (uint8)
            // NetworkVarNames: m_nAttachmentIndex (AttachmentHandle_t)
            // NetworkVarNames: m_nAttachmentName (CUtlStringToken)
            // NetworkVarNames: m_iEffectName (uint16)
            // NetworkVarNames: m_nExplosionType (uint8)
            pub mod CEffectData {
                pub const m_vOrigin: usize = 0x8; // Vector
                pub const m_vStart: usize = 0x14; // Vector
                pub const m_vNormal: usize = 0x20; // Vector
                pub const m_vAngles: usize = 0x2C; // QAngle
                pub const m_hEntity: usize = 0x38; // CEntityHandle
                pub const m_hOtherEntity: usize = 0x3C; // CEntityHandle
                pub const m_flScale: usize = 0x40; // float32
                pub const m_flMagnitude: usize = 0x44; // float32
                pub const m_flRadius: usize = 0x48; // float32
                pub const m_nSurfaceProp: usize = 0x4C; // CUtlStringToken
                pub const m_nEffectIndex: usize = 0x50; // CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>
                pub const m_nDamageType: usize = 0x58; // uint32
                pub const m_nPenetrate: usize = 0x5C; // uint8
                pub const m_nMaterial: usize = 0x5E; // uint16
                pub const m_nHitBox: usize = 0x60; // uint16
                pub const m_nColor: usize = 0x62; // uint8
                pub const m_fFlags: usize = 0x63; // uint8
                pub const m_nAttachmentIndex: usize = 0x64; // AttachmentHandle_t
                pub const m_nAttachmentName: usize = 0x68; // CUtlStringToken
                pub const m_iEffectName: usize = 0x6C; // uint16
                pub const m_nExplosionType: usize = 0x6E; // uint8
            }
            // Parent: CBaseEntity
            // Fields count: 2
            //
            // Metadata:
            // NetworkVarNames: m_flFadeStartDist (float32)
            // NetworkVarNames: m_flFadeEndDist (float32)
            pub mod CEnvDetailController {
                pub const m_flFadeStartDist: usize = 0x7A8; // float32
                pub const m_flFadeEndDist: usize = 0x7AC; // float32
            }
            // Parent: None
            // Fields count: 31
            //
            // Metadata:
            // NetworkVarNames: m_flStartTime (GameTime_t)
            // NetworkVarNames: m_iWindSeed (uint32)
            // NetworkVarNames: m_iMinWind (uint16)
            // NetworkVarNames: m_iMaxWind (uint16)
            // NetworkVarNames: m_windRadius (int32)
            // NetworkVarNames: m_iMinGust (uint16)
            // NetworkVarNames: m_iMaxGust (uint16)
            // NetworkVarNames: m_flMinGustDelay (float32)
            // NetworkVarNames: m_flMaxGustDelay (float32)
            // NetworkVarNames: m_flGustDuration (float32)
            // NetworkVarNames: m_iGustDirChange (uint16)
            // NetworkVarNames: m_location (Vector)
            // NetworkVarNames: m_iInitialWindDir (uint16)
            // NetworkVarNames: m_flInitialWindSpeed (float32)
            pub mod CEnvWindShared {
                pub const m_flStartTime: usize = 0x8; // GameTime_t
                pub const m_iWindSeed: usize = 0xC; // uint32
                pub const m_iMinWind: usize = 0x10; // uint16
                pub const m_iMaxWind: usize = 0x12; // uint16
                pub const m_windRadius: usize = 0x14; // int32
                pub const m_iMinGust: usize = 0x18; // uint16
                pub const m_iMaxGust: usize = 0x1A; // uint16
                pub const m_flMinGustDelay: usize = 0x1C; // float32
                pub const m_flMaxGustDelay: usize = 0x20; // float32
                pub const m_flGustDuration: usize = 0x24; // float32
                pub const m_iGustDirChange: usize = 0x28; // uint16
                pub const m_location: usize = 0x2C; // Vector
                pub const m_iszGustSound: usize = 0x38; // int32
                pub const m_iWindDir: usize = 0x3C; // int32
                pub const m_flWindSpeed: usize = 0x40; // float32
                pub const m_currentWindVector: usize = 0x44; // Vector
                pub const m_CurrentSwayVector: usize = 0x50; // Vector
                pub const m_PrevSwayVector: usize = 0x5C; // Vector
                pub const m_iInitialWindDir: usize = 0x68; // uint16
                pub const m_flInitialWindSpeed: usize = 0x6C; // float32
                pub const m_OnGustStart: usize = 0x70; // CEntityIOOutput
                pub const m_OnGustEnd: usize = 0x98; // CEntityIOOutput
                pub const m_flVariationTime: usize = 0xC0; // GameTime_t
                pub const m_flSwayTime: usize = 0xC4; // GameTime_t
                pub const m_flSimTime: usize = 0xC8; // GameTime_t
                pub const m_flSwitchTime: usize = 0xCC; // GameTime_t
                pub const m_flAveWindSpeed: usize = 0xD0; // float32
                pub const m_bGusting: usize = 0xD4; // bool
                pub const m_flWindAngleVariation: usize = 0xD8; // float32
                pub const m_flWindSpeedVariation: usize = 0xDC; // float32
                pub const m_iEntIndex: usize = 0xE0; // CEntityIndex
            }
            // Parent: None
            // Fields count: 12
            //
            // Metadata:
            // NetworkVarNames: m_nModelID (int32)
            // NetworkVarNames: m_hMaterial (HMaterialStrong)
            // NetworkVarNames: m_solid (ShardSolid_t)
            // NetworkVarNames: m_ShatterPanelMode (ShatterPanelMode)
            // NetworkVarNames: m_vecPanelSize (Vector2D)
            // NetworkVarNames: m_vecStressPositionA (Vector2D)
            // NetworkVarNames: m_vecStressPositionB (Vector2D)
            // NetworkVarNames: m_vecPanelVertices (Vector2D)
            // NetworkVarNames: m_flGlassHalfThickness (float)
            // NetworkVarNames: m_bHasParent (bool)
            // NetworkVarNames: m_bParentFrozen (bool)
            // NetworkVarNames: m_SurfacePropStringToken (CUtlStringToken)
            pub mod shard_model_desc_t {
                pub const m_nModelID: usize = 0x8; // int32
                pub const m_hMaterial: usize = 0x10; // CStrongHandle<InfoForResourceTypeIMaterial2>
                pub const m_solid: usize = 0x18; // ShardSolid_t
                pub const m_ShatterPanelMode: usize = 0x19; // ShatterPanelMode
                pub const m_vecPanelSize: usize = 0x1C; // Vector2D
                pub const m_vecStressPositionA: usize = 0x24; // Vector2D
                pub const m_vecStressPositionB: usize = 0x2C; // Vector2D
                pub const m_vecPanelVertices: usize = 0x38; // CNetworkUtlVectorBase<Vector2D>
                pub const m_flGlassHalfThickness: usize = 0x50; // float32
                pub const m_bHasParent: usize = 0x54; // bool
                pub const m_bParentFrozen: usize = 0x55; // bool
                pub const m_SurfacePropStringToken: usize = 0x58; // CUtlStringToken
            }
            // Parent: None
            // Fields count: 11
            //
            // Metadata:
            // NetworkVarNames: m_iGlowType (int32)
            // NetworkVarNames: m_iGlowTeam (int32)
            // NetworkVarNames: m_nGlowRange (int32)
            // NetworkVarNames: m_nGlowRangeMin (int32)
            // NetworkVarNames: m_glowColorOverride (Color)
            // NetworkVarNames: m_bFlashing (bool)
            // NetworkVarNames: m_flGlowTime (float)
            // NetworkVarNames: m_flGlowStartTime (float)
            // NetworkVarNames: m_bEligibleForScreenHighlight (bool)
            pub mod CGlowProperty {
                pub const m_fGlowColor: usize = 0x8; // Vector
                pub const m_iGlowType: usize = 0x30; // int32
                pub const m_iGlowTeam: usize = 0x34; // int32
                pub const m_nGlowRange: usize = 0x38; // int32
                pub const m_nGlowRangeMin: usize = 0x3C; // int32
                pub const m_glowColorOverride: usize = 0x40; // Color
                pub const m_bFlashing: usize = 0x44; // bool
                pub const m_flGlowTime: usize = 0x48; // float32
                pub const m_flGlowStartTime: usize = 0x4C; // float32
                pub const m_bEligibleForScreenHighlight: usize = 0x50; // bool
                pub const m_bGlowing: usize = 0x51; // bool
            }
            // Parent: None
            // Fields count: 25
            //
            // Metadata:
            // NetworkVarNames: dirPrimary (Vector)
            // NetworkVarNames: colorPrimary (Color)
            // NetworkVarNames: colorSecondary (Color)
            // NetworkVarNames: colorPrimaryLerpTo (Color)
            // NetworkVarNames: colorSecondaryLerpTo (Color)
            // NetworkVarNames: start (float32)
            // NetworkVarNames: end (float32)
            // NetworkVarNames: farz (float32)
            // NetworkVarNames: maxdensity (float32)
            // NetworkVarNames: exponent (float32)
            // NetworkVarNames: HDRColorScale (float32)
            // NetworkVarNames: skyboxFogFactor (float32)
            // NetworkVarNames: skyboxFogFactorLerpTo (float32)
            // NetworkVarNames: startLerpTo (float32)
            // NetworkVarNames: endLerpTo (float32)
            // NetworkVarNames: maxdensityLerpTo (float32)
            // NetworkVarNames: lerptime (GameTime_t)
            // NetworkVarNames: duration (float32)
            // NetworkVarNames: blendtobackground (float32)
            // NetworkVarNames: scattering (float32)
            // NetworkVarNames: locallightscale (float32)
            // NetworkVarNames: enable (bool)
            // NetworkVarNames: blend (bool)
            // NetworkVarNames: m_bNoReflectionFog (bool)
            pub mod fogparams_t {
                pub const dirPrimary: usize = 0x8; // Vector
                pub const colorPrimary: usize = 0x14; // Color
                pub const colorSecondary: usize = 0x18; // Color
                pub const colorPrimaryLerpTo: usize = 0x1C; // Color
                pub const colorSecondaryLerpTo: usize = 0x20; // Color
                pub const start: usize = 0x24; // float32
                pub const end: usize = 0x28; // float32
                pub const farz: usize = 0x2C; // float32
                pub const maxdensity: usize = 0x30; // float32
                pub const exponent: usize = 0x34; // float32
                pub const HDRColorScale: usize = 0x38; // float32
                pub const skyboxFogFactor: usize = 0x3C; // float32
                pub const skyboxFogFactorLerpTo: usize = 0x40; // float32
                pub const startLerpTo: usize = 0x44; // float32
                pub const endLerpTo: usize = 0x48; // float32
                pub const maxdensityLerpTo: usize = 0x4C; // float32
                pub const lerptime: usize = 0x50; // GameTime_t
                pub const duration: usize = 0x54; // float32
                pub const blendtobackground: usize = 0x58; // float32
                pub const scattering: usize = 0x5C; // float32
                pub const locallightscale: usize = 0x60; // float32
                pub const enable: usize = 0x64; // bool
                pub const blend: usize = 0x65; // bool
                pub const m_bNoReflectionFog: usize = 0x66; // bool
                pub const m_bPadding: usize = 0x67; // bool
            }
            // Parent: None
            // Fields count: 14
            //
            // Metadata:
            // NetworkVarNames: m_hCtrl (CHandle<CFogController>)
            pub mod fogplayerparams_t {
                pub const m_hCtrl: usize = 0x8; // CHandle<CFogController>
                pub const m_flTransitionTime: usize = 0xC; // float32
                pub const m_OldColor: usize = 0x10; // Color
                pub const m_flOldStart: usize = 0x14; // float32
                pub const m_flOldEnd: usize = 0x18; // float32
                pub const m_flOldMaxDensity: usize = 0x1C; // float32
                pub const m_flOldHDRColorScale: usize = 0x20; // float32
                pub const m_flOldFarZ: usize = 0x24; // float32
                pub const m_NewColor: usize = 0x28; // Color
                pub const m_flNewStart: usize = 0x2C; // float32
                pub const m_flNewEnd: usize = 0x30; // float32
                pub const m_flNewMaxDensity: usize = 0x34; // float32
                pub const m_flNewHDRColorScale: usize = 0x38; // float32
                pub const m_flNewFarZ: usize = 0x3C; // float32
            }
            // Parent: None
            // Fields count: 6
            //
            // Metadata:
            // NetworkVarNames: scale (int16)
            // NetworkVarNames: origin (Vector)
            // NetworkVarNames: bClip3DSkyBoxNearToWorldFar (bool)
            // NetworkVarNames: flClip3DSkyBoxNearToWorldFarOffset (float32)
            // NetworkVarNames: fog (fogparams_t)
            // NetworkVarNames: m_nWorldGroupID (WorldGroupId_t)
            pub mod sky3dparams_t {
                pub const scale: usize = 0x8; // int16
                pub const origin: usize = 0xC; // Vector
                pub const bClip3DSkyBoxNearToWorldFar: usize = 0x18; // bool
                pub const flClip3DSkyBoxNearToWorldFarOffset: usize = 0x1C; // float32
                pub const fog: usize = 0x20; // fogparams_t
                pub const m_nWorldGroupID: usize = 0x88; // WorldGroupId_t
            }
            // Parent: None
            // Fields count: 3
            //
            // Metadata:
            // NetworkVarNames: m_Transforms (CTransform)
            // NetworkVarNames: m_hOwner (EHANDLE)
            pub mod PhysicsRagdollPose_t {
                pub const __m_pChainEntity: usize = 0x8; // CNetworkVarChainer
                pub const m_Transforms: usize = 0x30; // CNetworkUtlVectorBase<CTransform>
                pub const m_hOwner: usize = 0x48; // CHandle<CBaseEntity>
            }
            // Parent: CBaseEntity
            // Fields count: 9
            //
            // Metadata:
            // NetworkVarNames: m_iszStackName (string_t)
            // NetworkVarNames: m_iszOperatorName (string_t)
            // NetworkVarNames: m_iszOpvarName (string_t)
            // NetworkVarNames: m_iOpvarIndex (int)
            // NetworkVarNames: m_bUseAutoCompare (bool)
            pub mod CSoundOpvarSetPointBase {
                pub const m_bDisabled: usize = 0x7A8; // bool
                pub const m_hSource: usize = 0x7AC; // CEntityHandle
                pub const m_iszSourceEntityName: usize = 0x7B8; // CUtlSymbolLarge
                pub const m_vLastPosition: usize = 0x810; // Vector
                pub const m_iszStackName: usize = 0x820; // CUtlSymbolLarge
                pub const m_iszOperatorName: usize = 0x828; // CUtlSymbolLarge
                pub const m_iszOpvarName: usize = 0x830; // CUtlSymbolLarge
                pub const m_iOpvarIndex: usize = 0x838; // int32
                pub const m_bUseAutoCompare: usize = 0x83C; // bool
            }
            // Parent: None
            // Fields count: 2
            //
            // Metadata:
            // NetworkVarNames: m_Attributes (CEconItemAttribute)
            pub mod CAttributeList {
                pub const m_Attributes: usize = 0x8; // CUtlVectorEmbeddedNetworkVar<CEconItemAttribute>
                pub const m_pManager: usize = 0x58; // CAttributeManager*
            }
            // Parent: None
            // Fields count: 5
            //
            // Metadata:
            // NetworkVarNames: m_iAttributeDefinitionIndex (attrib_definition_index_t)
            // NetworkVarNames: m_flValue (float)
            // NetworkVarNames: m_flInitialValue (float)
            // NetworkVarNames: m_nRefundableCurrency (int)
            // NetworkVarNames: m_bSetBonus (bool)
            pub mod CEconItemAttribute {
                pub const m_iAttributeDefinitionIndex: usize = 0x30; // uint16
                pub const m_flValue: usize = 0x34; // float32
                pub const m_flInitialValue: usize = 0x38; // float32
                pub const m_nRefundableCurrency: usize = 0x3C; // int32
                pub const m_bSetBonus: usize = 0x40; // bool
            }
            // Parent: None
            // Fields count: 6
            //
            // Metadata:
            // NetworkVarNames: m_iReapplyProvisionParity (int)
            // NetworkVarNames: m_hOuter (EHANDLE)
            // NetworkVarNames: m_ProviderType (attributeprovidertypes_t)
            pub mod CAttributeManager {
                pub const m_Providers: usize = 0x8; // CUtlVector<CHandle<CBaseEntity>>
                pub const m_iReapplyProvisionParity: usize = 0x20; // int32
                pub const m_hOuter: usize = 0x24; // CHandle<CBaseEntity>
                pub const m_bPreventLoopback: usize = 0x28; // bool
                pub const m_ProviderType: usize = 0x2C; // attributeprovidertypes_t
                pub const m_CachedResults: usize = 0x30; // CUtlVector<CAttributeManager::cached_attribute_float_t>
            }
            // Parent: CAttributeManager
            // Fields count: 1
            //
            // Metadata:
            // MNetworkIncludeByName
            // MNetworkIncludeByName
            // MNetworkIncludeByName
            // MNetworkIncludeByName
            // NetworkVarNames: m_Item (CEconItemView)
            pub mod CAttributeContainer {
                pub const m_Item: usize = 0x50; // CEconItemView
            }
            // Parent: AmmoTypeInfo_t
            // Fields count: 2
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod GameAmmoTypeInfo_t {
                pub const m_nBuySize: usize = 0x34; // int32
                pub const m_nCost: usize = 0x38; // int32
            }
            // Parent: None
            // Fields count: 2
            //
            // Metadata:
            // NetworkVarNames: m_bSpotted (bool)
            // NetworkVarNames: m_bSpottedByMask (uint32)
            pub mod EntitySpottedState_t {
                pub const m_bSpotted: usize = 0x8; // bool
                pub const m_bSpottedByMask: usize = 0xC; // uint32[2]
            }
            // Parent: None
            // Fields count: 1
            //
            // Metadata:
            // NetworkVarNames: m_pGameRules (CCSGameRules*)
            pub mod CCSGameRulesProxy {
                pub const m_pGameRules: usize = 0x7A8; // CCSGameRules*
            }
            // Parent: None
            // Fields count: 215
            //
            // Metadata:
            // NetworkVarNames: m_bFreezePeriod (bool)
            // NetworkVarNames: m_bWarmupPeriod (bool)
            // NetworkVarNames: m_fWarmupPeriodEnd (GameTime_t)
            // NetworkVarNames: m_fWarmupPeriodStart (GameTime_t)
            // NetworkVarNames: m_nTotalPausedTicks (int)
            // NetworkVarNames: m_nPauseStartTick (int)
            // NetworkVarNames: m_bServerPaused (bool)
            // NetworkVarNames: m_bGamePaused (bool)
            // NetworkVarNames: m_bTerroristTimeOutActive (bool)
            // NetworkVarNames: m_bCTTimeOutActive (bool)
            // NetworkVarNames: m_flTerroristTimeOutRemaining (float)
            // NetworkVarNames: m_flCTTimeOutRemaining (float)
            // NetworkVarNames: m_nTerroristTimeOuts (int)
            // NetworkVarNames: m_nCTTimeOuts (int)
            // NetworkVarNames: m_bTechnicalTimeOut (bool)
            // NetworkVarNames: m_bMatchWaitingForResume (bool)
            // NetworkVarNames: m_iRoundTime (int)
            // NetworkVarNames: m_fMatchStartTime (float)
            // NetworkVarNames: m_fRoundStartTime (GameTime_t)
            // NetworkVarNames: m_flRestartRoundTime (GameTime_t)
            // NetworkVarNames: m_bGameRestart (bool)
            // NetworkVarNames: m_flGameStartTime (float)
            // NetworkVarNames: m_timeUntilNextPhaseStarts (float)
            // NetworkVarNames: m_gamePhase (int)
            // NetworkVarNames: m_totalRoundsPlayed (int)
            // NetworkVarNames: m_nRoundsPlayedThisPhase (int)
            // NetworkVarNames: m_nOvertimePlaying (int)
            // NetworkVarNames: m_iHostagesRemaining (int)
            // NetworkVarNames: m_bAnyHostageReached (bool)
            // NetworkVarNames: m_bMapHasBombTarget (bool)
            // NetworkVarNames: m_bMapHasRescueZone (bool)
            // NetworkVarNames: m_bMapHasBuyZone (bool)
            // NetworkVarNames: m_bIsQueuedMatchmaking (bool)
            // NetworkVarNames: m_nQueuedMatchmakingMode (int)
            // NetworkVarNames: m_bIsValveDS (bool)
            // NetworkVarNames: m_bLogoMap (bool)
            // NetworkVarNames: m_bPlayAllStepSoundsOnServer (bool)
            // NetworkVarNames: m_iSpectatorSlotCount (int)
            // NetworkVarNames: m_MatchDevice (int)
            // NetworkVarNames: m_bHasMatchStarted (bool)
            // NetworkVarNames: m_nNextMapInMapgroup (int)
            // NetworkVarNames: m_szTournamentEventName (char)
            // NetworkVarNames: m_szTournamentEventStage (char)
            // NetworkVarNames: m_szMatchStatTxt (char)
            // NetworkVarNames: m_szTournamentPredictionsTxt (char)
            // NetworkVarNames: m_nTournamentPredictionsPct (int)
            // NetworkVarNames: m_flCMMItemDropRevealStartTime (GameTime_t)
            // NetworkVarNames: m_flCMMItemDropRevealEndTime (GameTime_t)
            // NetworkVarNames: m_bIsDroppingItems (bool)
            // NetworkVarNames: m_bIsQuestEligible (bool)
            // NetworkVarNames: m_bIsHltvActive (bool)
            // NetworkVarNames: m_nGuardianModeWaveNumber (int)
            // NetworkVarNames: m_nGuardianModeSpecialKillsRemaining (int)
            // NetworkVarNames: m_nGuardianModeSpecialWeaponNeeded (int)
            // NetworkVarNames: m_numGlobalGiftsGiven (uint32)
            // NetworkVarNames: m_numGlobalGifters (uint32)
            // NetworkVarNames: m_numGlobalGiftsPeriodSeconds (uint32)
            // NetworkVarNames: m_arrFeaturedGiftersAccounts (uint32)
            // NetworkVarNames: m_arrFeaturedGiftersGifts (uint32)
            // NetworkVarNames: m_arrProhibitedItemIndices (uint16)
            // NetworkVarNames: m_arrTournamentActiveCasterAccounts (uint32)
            // NetworkVarNames: m_numBestOfMaps (int)
            // NetworkVarNames: m_nHalloweenMaskListSeed (int)
            // NetworkVarNames: m_bBombDropped (bool)
            // NetworkVarNames: m_bBombPlanted (bool)
            // NetworkVarNames: m_iRoundWinStatus (int)
            // NetworkVarNames: m_eRoundWinReason (int)
            // NetworkVarNames: m_bTCantBuy (bool)
            // NetworkVarNames: m_bCTCantBuy (bool)
            // NetworkVarNames: m_flGuardianBuyUntilTime (GameTime_t)
            // NetworkVarNames: m_iMatchStats_RoundResults (int)
            // NetworkVarNames: m_iMatchStats_PlayersAlive_CT (int)
            // NetworkVarNames: m_iMatchStats_PlayersAlive_T (int)
            // NetworkVarNames: m_TeamRespawnWaveTimes (float)
            // NetworkVarNames: m_flNextRespawnWave (GameTime_t)
            // NetworkVarNames: m_nServerQuestID (int)
            // NetworkVarNames: m_vMinimapMins (Vector)
            // NetworkVarNames: m_vMinimapMaxs (Vector)
            // NetworkVarNames: m_MinimapVerticalSectionHeights (float)
            // NetworkVarNames: m_nEndMatchMapGroupVoteTypes (int)
            // NetworkVarNames: m_nEndMatchMapGroupVoteOptions (int)
            // NetworkVarNames: m_nEndMatchMapVoteWinner (int)
            // NetworkVarNames: m_iNumConsecutiveCTLoses (int)
            // NetworkVarNames: m_iNumConsecutiveTerroristLoses (int)
            // NetworkVarNames: m_nMatchAbortedEarlyReason (int)
            // NetworkVarNames: m_pGameModeRules (CCSGameModeRules*)
            // NetworkVarNames: m_RetakeRules (CRetakeGameRules)
            // NetworkVarNames: m_nMatchEndCount (uint8)
            // NetworkVarNames: m_nTTeamIntroVariant (int)
            // NetworkVarNames: m_nCTTeamIntroVariant (int)
            // NetworkVarNames: m_bTeamIntroPeriod (bool)
            // NetworkVarNames: m_iRoundEndWinnerTeam (int)
            // NetworkVarNames: m_eRoundEndReason (int)
            // NetworkVarNames: m_bRoundEndShowTimerDefend (bool)
            // NetworkVarNames: m_iRoundEndTimerTime (int)
            // NetworkVarNames: m_sRoundEndFunFactToken (CUtlString)
            // NetworkVarNames: m_iRoundEndFunFactPlayerSlot (CPlayerSlot)
            // NetworkVarNames: m_iRoundEndFunFactData1 (int)
            // NetworkVarNames: m_iRoundEndFunFactData2 (int)
            // NetworkVarNames: m_iRoundEndFunFactData3 (int)
            // NetworkVarNames: m_sRoundEndMessage (CUtlString)
            // NetworkVarNames: m_iRoundEndPlayerCount (int)
            // NetworkVarNames: m_bRoundEndNoMusic (bool)
            // NetworkVarNames: m_iRoundEndLegacy (int)
            // NetworkVarNames: m_nRoundEndCount (uint8)
            // NetworkVarNames: m_iRoundStartRoundNumber (int)
            // NetworkVarNames: m_nRoundStartCount (uint8)
            pub mod CCSGameRules {
                pub const __m_pChainEntity: usize = 0x98; // CNetworkVarChainer
                pub const m_coopMissionManager: usize = 0xC0; // CHandle<CBaseEntity>
                pub const m_bFreezePeriod: usize = 0xC4; // bool
                pub const m_bWarmupPeriod: usize = 0xC5; // bool
                pub const m_fWarmupPeriodEnd: usize = 0xC8; // GameTime_t
                pub const m_fWarmupPeriodStart: usize = 0xCC; // GameTime_t
                pub const m_nTotalPausedTicks: usize = 0xD0; // int32
                pub const m_nPauseStartTick: usize = 0xD4; // int32
                pub const m_bServerPaused: usize = 0xD8; // bool
                pub const m_bGamePaused: usize = 0xD9; // bool
                pub const m_bTerroristTimeOutActive: usize = 0xDA; // bool
                pub const m_bCTTimeOutActive: usize = 0xDB; // bool
                pub const m_flTerroristTimeOutRemaining: usize = 0xDC; // float32
                pub const m_flCTTimeOutRemaining: usize = 0xE0; // float32
                pub const m_nTerroristTimeOuts: usize = 0xE4; // int32
                pub const m_nCTTimeOuts: usize = 0xE8; // int32
                pub const m_bTechnicalTimeOut: usize = 0xEC; // bool
                pub const m_bMatchWaitingForResume: usize = 0xED; // bool
                pub const m_iRoundTime: usize = 0xF0; // int32
                pub const m_fMatchStartTime: usize = 0xF4; // float32
                pub const m_fRoundStartTime: usize = 0xF8; // GameTime_t
                pub const m_flRestartRoundTime: usize = 0xFC; // GameTime_t
                pub const m_bGameRestart: usize = 0x100; // bool
                pub const m_flGameStartTime: usize = 0x104; // float32
                pub const m_timeUntilNextPhaseStarts: usize = 0x108; // float32
                pub const m_gamePhase: usize = 0x10C; // int32
                pub const m_totalRoundsPlayed: usize = 0x110; // int32
                pub const m_nRoundsPlayedThisPhase: usize = 0x114; // int32
                pub const m_nOvertimePlaying: usize = 0x118; // int32
                pub const m_iHostagesRemaining: usize = 0x11C; // int32
                pub const m_bAnyHostageReached: usize = 0x120; // bool
                pub const m_bMapHasBombTarget: usize = 0x121; // bool
                pub const m_bMapHasRescueZone: usize = 0x122; // bool
                pub const m_bMapHasBuyZone: usize = 0x123; // bool
                pub const m_bIsQueuedMatchmaking: usize = 0x124; // bool
                pub const m_nQueuedMatchmakingMode: usize = 0x128; // int32
                pub const m_bIsValveDS: usize = 0x12C; // bool
                pub const m_bLogoMap: usize = 0x12D; // bool
                pub const m_bPlayAllStepSoundsOnServer: usize = 0x12E; // bool
                pub const m_iSpectatorSlotCount: usize = 0x130; // int32
                pub const m_MatchDevice: usize = 0x134; // int32
                pub const m_bHasMatchStarted: usize = 0x138; // bool
                pub const m_nNextMapInMapgroup: usize = 0x13C; // int32
                pub const m_szTournamentEventName: usize = 0x140; // char[512]
                pub const m_szTournamentEventStage: usize = 0x340; // char[512]
                pub const m_szMatchStatTxt: usize = 0x540; // char[512]
                pub const m_szTournamentPredictionsTxt: usize = 0x740; // char[512]
                pub const m_nTournamentPredictionsPct: usize = 0x940; // int32
                pub const m_flCMMItemDropRevealStartTime: usize = 0x944; // GameTime_t
                pub const m_flCMMItemDropRevealEndTime: usize = 0x948; // GameTime_t
                pub const m_bIsDroppingItems: usize = 0x94C; // bool
                pub const m_bIsQuestEligible: usize = 0x94D; // bool
                pub const m_bIsHltvActive: usize = 0x94E; // bool
                pub const m_nGuardianModeWaveNumber: usize = 0x950; // int32
                pub const m_nGuardianModeSpecialKillsRemaining: usize = 0x954; // int32
                pub const m_nGuardianModeSpecialWeaponNeeded: usize = 0x958; // int32
                pub const m_nGuardianGrenadesToGiveBots: usize = 0x95C; // int32
                pub const m_nNumHeaviesToSpawn: usize = 0x960; // int32
                pub const m_numGlobalGiftsGiven: usize = 0x964; // uint32
                pub const m_numGlobalGifters: usize = 0x968; // uint32
                pub const m_numGlobalGiftsPeriodSeconds: usize = 0x96C; // uint32
                pub const m_arrFeaturedGiftersAccounts: usize = 0x970; // uint32[4]
                pub const m_arrFeaturedGiftersGifts: usize = 0x980; // uint32[4]
                pub const m_arrProhibitedItemIndices: usize = 0x990; // uint16[100]
                pub const m_arrTournamentActiveCasterAccounts: usize = 0xA58; // uint32[4]
                pub const m_numBestOfMaps: usize = 0xA68; // int32
                pub const m_nHalloweenMaskListSeed: usize = 0xA6C; // int32
                pub const m_bBombDropped: usize = 0xA70; // bool
                pub const m_bBombPlanted: usize = 0xA71; // bool
                pub const m_iRoundWinStatus: usize = 0xA74; // int32
                pub const m_eRoundWinReason: usize = 0xA78; // int32
                pub const m_bTCantBuy: usize = 0xA7C; // bool
                pub const m_bCTCantBuy: usize = 0xA7D; // bool
                pub const m_flGuardianBuyUntilTime: usize = 0xA80; // GameTime_t
                pub const m_iMatchStats_RoundResults: usize = 0xA84; // int32[30]
                pub const m_iMatchStats_PlayersAlive_CT: usize = 0xAFC; // int32[30]
                pub const m_iMatchStats_PlayersAlive_T: usize = 0xB74; // int32[30]
                pub const m_TeamRespawnWaveTimes: usize = 0xBEC; // float32[32]
                pub const m_flNextRespawnWave: usize = 0xC6C; // GameTime_t[32]
                pub const m_nServerQuestID: usize = 0xCEC; // int32
                pub const m_vMinimapMins: usize = 0xCF0; // Vector
                pub const m_vMinimapMaxs: usize = 0xCFC; // Vector
                pub const m_MinimapVerticalSectionHeights: usize = 0xD08; // float32[8]
                pub const m_bDontIncrementCoopWave: usize = 0xD28; // bool
                pub const m_bSpawnedTerrorHuntHeavy: usize = 0xD29; // bool
                pub const m_nEndMatchMapGroupVoteTypes: usize = 0xD2C; // int32[10]
                pub const m_nEndMatchMapGroupVoteOptions: usize = 0xD54; // int32[10]
                pub const m_nEndMatchMapVoteWinner: usize = 0xD7C; // int32
                pub const m_iNumConsecutiveCTLoses: usize = 0xD80; // int32
                pub const m_iNumConsecutiveTerroristLoses: usize = 0xD84; // int32
                pub const m_bHasHostageBeenTouched: usize = 0xDA0; // bool
                pub const m_flIntermissionStartTime: usize = 0xDA4; // GameTime_t
                pub const m_flIntermissionEndTime: usize = 0xDA8; // GameTime_t
                pub const m_bLevelInitialized: usize = 0xDAC; // bool
                pub const m_iTotalRoundsPlayed: usize = 0xDB0; // int32
                pub const m_iUnBalancedRounds: usize = 0xDB4; // int32
                pub const m_endMatchOnRoundReset: usize = 0xDB8; // bool
                pub const m_endMatchOnThink: usize = 0xDB9; // bool
                pub const m_iFreezeTime: usize = 0xDBC; // int32
                pub const m_iNumTerrorist: usize = 0xDC0; // int32
                pub const m_iNumCT: usize = 0xDC4; // int32
                pub const m_iNumSpawnableTerrorist: usize = 0xDC8; // int32
                pub const m_iNumSpawnableCT: usize = 0xDCC; // int32
                pub const m_arrSelectedHostageSpawnIndices: usize = 0xDD0; // CUtlVector<int32>
                pub const m_nSpawnPointsRandomSeed: usize = 0xDE8; // int32
                pub const m_bFirstConnected: usize = 0xDEC; // bool
                pub const m_bCompleteReset: usize = 0xDED; // bool
                pub const m_bPickNewTeamsOnReset: usize = 0xDEE; // bool
                pub const m_bScrambleTeamsOnRestart: usize = 0xDEF; // bool
                pub const m_bSwapTeamsOnRestart: usize = 0xDF0; // bool
                pub const m_nEndMatchTiedVotes: usize = 0xDF8; // CUtlVector<int32>
                pub const m_bNeedToAskPlayersForContinueVote: usize = 0xE14; // bool
                pub const m_numQueuedMatchmakingAccounts: usize = 0xE18; // uint32
                pub const m_pQueuedMatchmakingReservationString: usize = 0xE20; // char*
                pub const m_numTotalTournamentDrops: usize = 0xE28; // uint32
                pub const m_numSpectatorsCountMax: usize = 0xE2C; // uint32
                pub const m_numSpectatorsCountMaxTV: usize = 0xE30; // uint32
                pub const m_numSpectatorsCountMaxLnk: usize = 0xE34; // uint32
                pub const m_bForceTeamChangeSilent: usize = 0xE40; // bool
                pub const m_bLoadingRoundBackupData: usize = 0xE41; // bool
                pub const m_nMatchInfoShowType: usize = 0xE78; // int32
                pub const m_flMatchInfoDecidedTime: usize = 0xE7C; // float32
                pub const m_flCoopRespawnAndHealTime: usize = 0xE98; // float32
                pub const m_coopBonusCoinsFound: usize = 0xE9C; // int32
                pub const m_coopBonusPistolsOnly: usize = 0xEA0; // bool
                pub const m_coopPlayersInDeploymentZone: usize = 0xEA1; // bool
                pub const m_coopMissionDeadPlayerRespawnEnabled: usize = 0xEA2; // bool
                pub const mTeamDMLastWinningTeamNumber: usize = 0xEA4; // int32
                pub const mTeamDMLastThinkTime: usize = 0xEA8; // float32
                pub const m_flTeamDMLastAnnouncementTime: usize = 0xEAC; // float32
                pub const m_iAccountTerrorist: usize = 0xEB0; // int32
                pub const m_iAccountCT: usize = 0xEB4; // int32
                pub const m_iSpawnPointCount_Terrorist: usize = 0xEB8; // int32
                pub const m_iSpawnPointCount_CT: usize = 0xEBC; // int32
                pub const m_iMaxNumTerrorists: usize = 0xEC0; // int32
                pub const m_iMaxNumCTs: usize = 0xEC4; // int32
                pub const m_iLoserBonus: usize = 0xEC8; // int32
                pub const m_iLoserBonusMostRecentTeam: usize = 0xECC; // int32
                pub const m_tmNextPeriodicThink: usize = 0xED0; // float32
                pub const m_bVoiceWonMatchBragFired: usize = 0xED4; // bool
                pub const m_fWarmupNextChatNoticeTime: usize = 0xED8; // float32
                pub const m_iHostagesRescued: usize = 0xEE0; // int32
                pub const m_iHostagesTouched: usize = 0xEE4; // int32
                pub const m_flNextHostageAnnouncement: usize = 0xEE8; // float32
                pub const m_bNoTerroristsKilled: usize = 0xEEC; // bool
                pub const m_bNoCTsKilled: usize = 0xEED; // bool
                pub const m_bNoEnemiesKilled: usize = 0xEEE; // bool
                pub const m_bCanDonateWeapons: usize = 0xEEF; // bool
                pub const m_firstKillTime: usize = 0xEF4; // float32
                pub const m_firstBloodTime: usize = 0xEFC; // float32
                pub const m_hostageWasInjured: usize = 0xF18; // bool
                pub const m_hostageWasKilled: usize = 0xF19; // bool
                pub const m_bVoteCalled: usize = 0xF28; // bool
                pub const m_bServerVoteOnReset: usize = 0xF29; // bool
                pub const m_flVoteCheckThrottle: usize = 0xF2C; // float32
                pub const m_bBuyTimeEnded: usize = 0xF30; // bool
                pub const m_nLastFreezeEndBeep: usize = 0xF34; // int32
                pub const m_bTargetBombed: usize = 0xF38; // bool
                pub const m_bBombDefused: usize = 0xF39; // bool
                pub const m_bMapHasBombZone: usize = 0xF3A; // bool
                pub const m_vecMainCTSpawnPos: usize = 0xF58; // Vector
                pub const m_CTSpawnPointsMasterList: usize = 0xF68; // CUtlVector<SpawnPoint*>
                pub const m_TerroristSpawnPointsMasterList: usize = 0xF80; // CUtlVector<SpawnPoint*>
                pub const m_bRespawningAllRespawnablePlayers: usize = 0xF98; // bool
                pub const m_iNextCTSpawnPoint: usize = 0xF9C; // int32
                pub const m_flCTSpawnPointUsedTime: usize = 0xFA0; // float32
                pub const m_iNextTerroristSpawnPoint: usize = 0xFA4; // int32
                pub const m_flTerroristSpawnPointUsedTime: usize = 0xFA8; // float32
                pub const m_CTSpawnPoints: usize = 0xFB0; // CUtlVector<SpawnPoint*>
                pub const m_TerroristSpawnPoints: usize = 0xFC8; // CUtlVector<SpawnPoint*>
                pub const m_bIsUnreservedGameServer: usize = 0xFE0; // bool
                pub const m_fAutobalanceDisplayTime: usize = 0xFE4; // float32
                pub const m_bAllowWeaponSwitch: usize = 0x1250; // bool
                pub const m_bRoundTimeWarningTriggered: usize = 0x1251; // bool
                pub const m_phaseChangeAnnouncementTime: usize = 0x1254; // GameTime_t
                pub const m_fNextUpdateTeamClanNamesTime: usize = 0x1258; // float32
                pub const m_flLastThinkTime: usize = 0x125C; // GameTime_t
                pub const m_fAccumulatedRoundOffDamage: usize = 0x1260; // float32
                pub const m_nShorthandedBonusLastEvalRound: usize = 0x1264; // int32
                pub const m_nMatchAbortedEarlyReason: usize = 0x14E0; // int32
                pub const m_bHasTriggeredRoundStartMusic: usize = 0x14E4; // bool
                pub const m_bHasTriggeredCoopSpawnReset: usize = 0x14E5; // bool
                pub const m_bSwitchingTeamsAtRoundReset: usize = 0x14E6; // bool
                pub const m_pGameModeRules: usize = 0x1500; // CCSGameModeRules*
                pub const m_BtGlobalBlackboard: usize = 0x1508; // KeyValues3
                pub const m_hPlayerResource: usize = 0x1568; // CHandle<CBaseEntity>
                pub const m_RetakeRules: usize = 0x1570; // CRetakeGameRules
                pub const m_GuardianBotSkillLevelMax: usize = 0x1754; // int32
                pub const m_GuardianBotSkillLevelMin: usize = 0x1758; // int32
                pub const m_arrTeamUniqueKillWeaponsMatch: usize = 0x1760; // CUtlVector<int32>[4]
                pub const m_bTeamLastKillUsedUniqueWeaponMatch: usize = 0x17C0; // bool[4]
                pub const m_nMatchEndCount: usize = 0x17E8; // uint8
                pub const m_nTTeamIntroVariant: usize = 0x17EC; // int32
                pub const m_nCTTeamIntroVariant: usize = 0x17F0; // int32
                pub const m_bTeamIntroPeriod: usize = 0x17F4; // bool
                pub const m_fTeamIntroPeriodEnd: usize = 0x17F8; // GameTime_t
                pub const m_bPlayedTeamIntroVO: usize = 0x17FC; // bool
                pub const m_iRoundEndWinnerTeam: usize = 0x1800; // int32
                pub const m_eRoundEndReason: usize = 0x1804; // int32
                pub const m_bRoundEndShowTimerDefend: usize = 0x1808; // bool
                pub const m_iRoundEndTimerTime: usize = 0x180C; // int32
                pub const m_sRoundEndFunFactToken: usize = 0x1810; // CUtlString
                pub const m_iRoundEndFunFactPlayerSlot: usize = 0x1818; // CPlayerSlot
                pub const m_iRoundEndFunFactData1: usize = 0x181C; // int32
                pub const m_iRoundEndFunFactData2: usize = 0x1820; // int32
                pub const m_iRoundEndFunFactData3: usize = 0x1824; // int32
                pub const m_sRoundEndMessage: usize = 0x1828; // CUtlString
                pub const m_iRoundEndPlayerCount: usize = 0x1830; // int32
                pub const m_bRoundEndNoMusic: usize = 0x1834; // bool
                pub const m_iRoundEndLegacy: usize = 0x1838; // int32
                pub const m_nRoundEndCount: usize = 0x183C; // uint8
                pub const m_iRoundStartRoundNumber: usize = 0x1840; // int32
                pub const m_nRoundStartCount: usize = 0x1844; // uint8
                pub const m_flLastPerfSampleTime: usize = 0x5850; // float64
                pub const m_bSkipNextServerPerfSample: usize = 0x5858; // bool
            }
            // Parent: None
            // Fields count: 0
            //
            // Metadata:
            // MPulseProvideFeatureTag
            // MPulseLibraryBindings
            pub mod CCSTakeDamageInfoAPI {
            }
            // Parent: None
            // Fields count: 0
            //
            // Metadata:
            // MPulseInstanceDomainInfo
            // MPulseLibraryBindings
            // MPulseDomainOptInFeatureTag
            pub mod CCSGameModeScript {
            }
            // Parent: None
            // Fields count: 0
            //
            // Metadata:
            // MPulseLibraryBindings
            pub mod CCSGameModeScript_ConVars {
            }
            // Parent: None
            // Fields count: 1
            //
            // Metadata:
            // NetworkVarNames: m_WeaponSequence (CUtlString)
            pub mod CCSGameModeRules_ArmsRace {
                pub const m_WeaponSequence: usize = 0x38; // CNetworkUtlVectorBase<CUtlString>
            }
            // Parent: CCSGameModeScript
            // Fields count: 1
            //
            // Metadata:
            // MPulseInstanceDomainInfo
            // MPulseLibraryBindings
            // MPulseDomainOptInFeatureTag
            pub mod CCSArmsRaceScript {
                pub const m_pOuter: usize = 0xD8; // CCSGameModeRules_ArmsRace*
            }
            // Parent: None
            // Fields count: 3
            //
            // Metadata:
            // NetworkVarNames: m_flDMBonusStartTime (GameTime_t)
            // NetworkVarNames: m_flDMBonusTimeLength (float)
            // NetworkVarNames: m_nDMBonusWeaponLoadoutSlot (int16)
            pub mod CCSGameModeRules_Deathmatch {
                pub const m_flDMBonusStartTime: usize = 0x38; // GameTime_t
                pub const m_flDMBonusTimeLength: usize = 0x3C; // float32
                pub const m_nDMBonusWeaponLoadoutSlot: usize = 0x40; // int16
            }
            // Parent: CCSGameModeScript
            // Fields count: 1
            //
            // Metadata:
            // MPulseInstanceDomainInfo
            // MPulseLibraryBindings
            // MPulseDomainOptInFeatureTag
            pub mod CCSDeathmatchScript {
                pub const m_pOuter: usize = 0xD8; // CCSGameModeRules_Deathmatch*
            }
            // Parent: None
            // Fields count: 0
            //
            // Metadata:
            // MPulseLibraryBindings
            pub mod CCSDeathmatchScript_ConVars {
            }
            // Parent: None
            // Fields count: 5
            //
            // Metadata:
            // NetworkVarNames: m_nMatchSeed (int)
            // NetworkVarNames: m_bBlockersPresent (bool)
            // NetworkVarNames: m_bRoundInProgress (bool)
            // NetworkVarNames: m_iFirstSecondHalfRound (int)
            // NetworkVarNames: m_iBombSite (int)
            pub mod CRetakeGameRules {
                pub const m_nMatchSeed: usize = 0xF8; // int32
                pub const m_bBlockersPresent: usize = 0xFC; // bool
                pub const m_bRoundInProgress: usize = 0xFD; // bool
                pub const m_iFirstSecondHalfRound: usize = 0x100; // int32
                pub const m_iBombSite: usize = 0x104; // int32
            }
            // Parent: None
            // Fields count: 13
            //
            // Metadata:
            // NetworkVarNames: m_iKills (int)
            // NetworkVarNames: m_iDeaths (int)
            // NetworkVarNames: m_iAssists (int)
            // NetworkVarNames: m_iDamage (int)
            // NetworkVarNames: m_iEquipmentValue (int)
            // NetworkVarNames: m_iMoneySaved (int)
            // NetworkVarNames: m_iKillReward (int)
            // NetworkVarNames: m_iLiveTime (int)
            // NetworkVarNames: m_iHeadShotKills (int)
            // NetworkVarNames: m_iObjective (int)
            // NetworkVarNames: m_iCashEarned (int)
            // NetworkVarNames: m_iUtilityDamage (int)
            // NetworkVarNames: m_iEnemiesFlashed (int)
            pub mod CSPerRoundStats_t {
                pub const m_iKills: usize = 0x30; // int32
                pub const m_iDeaths: usize = 0x34; // int32
                pub const m_iAssists: usize = 0x38; // int32
                pub const m_iDamage: usize = 0x3C; // int32
                pub const m_iEquipmentValue: usize = 0x40; // int32
                pub const m_iMoneySaved: usize = 0x44; // int32
                pub const m_iKillReward: usize = 0x48; // int32
                pub const m_iLiveTime: usize = 0x4C; // int32
                pub const m_iHeadShotKills: usize = 0x50; // int32
                pub const m_iObjective: usize = 0x54; // int32
                pub const m_iCashEarned: usize = 0x58; // int32
                pub const m_iUtilityDamage: usize = 0x5C; // int32
                pub const m_iEnemiesFlashed: usize = 0x60; // int32
            }
            // Parent: CSPerRoundStats_t
            // Fields count: 21
            //
            // Metadata:
            // NetworkVarNames: m_iEnemy5Ks (int)
            // NetworkVarNames: m_iEnemy4Ks (int)
            // NetworkVarNames: m_iEnemy3Ks (int)
            // NetworkVarNames: m_iEnemyKnifeKills (int)
            // NetworkVarNames: m_iEnemyTaserKills (int)
            pub mod CSMatchStats_t {
                pub const m_iEnemy5Ks: usize = 0x64; // int32
                pub const m_iEnemy4Ks: usize = 0x68; // int32
                pub const m_iEnemy3Ks: usize = 0x6C; // int32
                pub const m_iEnemyKnifeKills: usize = 0x70; // int32
                pub const m_iEnemyTaserKills: usize = 0x74; // int32
                pub const m_iEnemy2Ks: usize = 0x78; // int32
                pub const m_iUtility_Count: usize = 0x7C; // int32
                pub const m_iUtility_Successes: usize = 0x80; // int32
                pub const m_iUtility_Enemies: usize = 0x84; // int32
                pub const m_iFlash_Count: usize = 0x88; // int32
                pub const m_iFlash_Successes: usize = 0x8C; // int32
                pub const m_nHealthPointsRemovedTotal: usize = 0x90; // int32
                pub const m_nHealthPointsDealtTotal: usize = 0x94; // int32
                pub const m_nShotsFiredTotal: usize = 0x98; // int32
                pub const m_nShotsOnTargetTotal: usize = 0x9C; // int32
                pub const m_i1v1Count: usize = 0xA0; // int32
                pub const m_i1v1Wins: usize = 0xA4; // int32
                pub const m_i1v2Count: usize = 0xA8; // int32
                pub const m_i1v2Wins: usize = 0xAC; // int32
                pub const m_iEntryCount: usize = 0xB0; // int32
                pub const m_iEntryWins: usize = 0xB4; // int32
            }
            // Parent: CBaseEntity
            // Fields count: 8
            //
            // Metadata:
            // NetworkVarNames: m_nVariant (int)
            // NetworkVarNames: m_nRandom (int)
            // NetworkVarNames: m_nOrdinal (int)
            // NetworkVarNames: m_sWeaponName (CUtlString)
            // NetworkVarNames: m_xuid (XUID)
            // NetworkVarNames: m_agentItem (CEconItemView)
            // NetworkVarNames: m_glovesItem (CEconItemView)
            // NetworkVarNames: m_weaponItem (CEconItemView)
            pub mod CCSGO_TeamPreviewCharacterPosition {
                pub const m_nVariant: usize = 0x7A8; // int32
                pub const m_nRandom: usize = 0x7AC; // int32
                pub const m_nOrdinal: usize = 0x7B0; // int32
                pub const m_sWeaponName: usize = 0x7B8; // CUtlString
                pub const m_xuid: usize = 0x7C0; // uint64
                pub const m_agentItem: usize = 0x7C8; // CEconItemView
                pub const m_glovesItem: usize = 0xA40; // CEconItemView
                pub const m_weaponItem: usize = 0xCB8; // CEconItemView
            }
            // Parent: CBaseEntity
            // Fields count: 5
            //
            // Metadata:
            // NetworkVarNames: m_hPlayer (CHandle<CCSPlayerPawn>)
            // NetworkVarNames: m_hPingedEntity (CHandle<CBaseEntity>)
            // NetworkVarNames: m_iType (int)
            // NetworkVarNames: m_bUrgent (bool)
            // NetworkVarNames: m_szPlaceName (char)
            pub mod CPlayerPing {
                pub const m_hPlayer: usize = 0x7B0; // CHandle<CCSPlayerPawn>
                pub const m_hPingedEntity: usize = 0x7B4; // CHandle<CBaseEntity>
                pub const m_iType: usize = 0x7B8; // int32
                pub const m_bUrgent: usize = 0x7BC; // bool
                pub const m_szPlaceName: usize = 0x7BD; // char[18]
            }
            // Parent: None
            // Fields count: 2
            //
            // Metadata:
            // NetworkVarNames: m_hPlayerPing (CHandle<CBaseEntity>)
            pub mod CCSPlayer_PingServices {
                pub const m_flPlayerPingTokens: usize = 0x40; // GameTime_t[5]
                pub const m_hPlayerPing: usize = 0x54; // CHandle<CBaseEntity>
            }
            // Parent: CBaseEntity
            // Fields count: 10
            //
            // Metadata:
            // NetworkVarNames: m_bHostageAlive (bool)
            // NetworkVarNames: m_isHostageFollowingSomeone (bool)
            // NetworkVarNames: m_iHostageEntityIDs (CEntityIndex)
            // NetworkVarNames: m_bombsiteCenterA (Vector)
            // NetworkVarNames: m_bombsiteCenterB (Vector)
            // NetworkVarNames: m_hostageRescueX (int)
            // NetworkVarNames: m_hostageRescueY (int)
            // NetworkVarNames: m_hostageRescueZ (int)
            // NetworkVarNames: m_bEndMatchNextMapAllVoted (bool)
            pub mod CCSPlayerResource {
                pub const m_bHostageAlive: usize = 0x7A8; // bool[12]
                pub const m_isHostageFollowingSomeone: usize = 0x7B4; // bool[12]
                pub const m_iHostageEntityIDs: usize = 0x7C0; // CEntityIndex[12]
                pub const m_bombsiteCenterA: usize = 0x7F0; // Vector
                pub const m_bombsiteCenterB: usize = 0x7FC; // Vector
                pub const m_hostageRescueX: usize = 0x808; // int32[4]
                pub const m_hostageRescueY: usize = 0x818; // int32[4]
                pub const m_hostageRescueZ: usize = 0x828; // int32[4]
                pub const m_bEndMatchNextMapAllVoted: usize = 0x838; // bool
                pub const m_foundGoalPositions: usize = 0x839; // bool
            }
            // Parent: None
            // Fields count: 0
            //
            // Metadata:
            // MPulseProvideFeatureTag
            // MPulseLibraryBindings
            pub mod CCSPlayerControllerAPI {
            }
            // Parent: CPlayer_CameraServices
            // Fields count: 7
            //
            // Metadata:
            // NetworkVarNames: m_iFOV (uint32)
            // NetworkVarNames: m_iFOVStart (uint32)
            // NetworkVarNames: m_flFOVTime (GameTime_t)
            // NetworkVarNames: m_flFOVRate (float32)
            // NetworkVarNames: m_hZoomOwner (CHandle<CBaseEntity>)
            pub mod CCSPlayerBase_CameraServices {
                pub const m_iFOV: usize = 0x170; // uint32
                pub const m_iFOVStart: usize = 0x174; // uint32
                pub const m_flFOVTime: usize = 0x178; // GameTime_t
                pub const m_flFOVRate: usize = 0x17C; // float32
                pub const m_hZoomOwner: usize = 0x180; // CHandle<CBaseEntity>
                pub const m_hTriggerFogList: usize = 0x188; // CUtlVector<CHandle<CBaseEntity>>
                pub const m_hLastFogTrigger: usize = 0x1A0; // CHandle<CBaseEntity>
            }
            // Parent: None
            // Fields count: 2
            //
            // Metadata:
            // NetworkVarNames: m_nItemDefIndex (uint16)
            // NetworkVarNames: m_nCount (uint16)
            pub mod WeaponPurchaseCount_t {
                pub const m_nItemDefIndex: usize = 0x30; // uint16
                pub const m_nCount: usize = 0x32; // uint16
            }
            // Parent: None
            // Fields count: 1
            //
            // Metadata:
            // NetworkVarNames: m_weaponPurchases (WeaponPurchaseCount_t)
            pub mod WeaponPurchaseTracker_t {
                pub const m_weaponPurchases: usize = 0x8; // CUtlVectorEmbeddedNetworkVar<WeaponPurchaseCount_t>
            }
            // Parent: None
            // Fields count: 4
            //
            // Metadata:
            // NetworkVarNames: m_bIsRescuing (bool)
            // NetworkVarNames: m_weaponPurchasesThisMatch (WeaponPurchaseTracker_t)
            // NetworkVarNames: m_weaponPurchasesThisRound (WeaponPurchaseTracker_t)
            pub mod CCSPlayer_ActionTrackingServices {
                pub const m_hLastWeaponBeforeC4AutoSwitch: usize = 0x208; // CHandle<CBasePlayerWeapon>
                pub const m_bIsRescuing: usize = 0x23C; // bool
                pub const m_weaponPurchasesThisMatch: usize = 0x240; // WeaponPurchaseTracker_t
                pub const m_weaponPurchasesThisRound: usize = 0x298; // WeaponPurchaseTracker_t
            }
            // Parent: None
            // Fields count: 1
            //
            // Metadata:
            // NetworkVarNames: m_totalHitsOnServer (int32)
            pub mod CCSPlayer_BulletServices {
                pub const m_totalHitsOnServer: usize = 0x40; // int32
            }
            // Parent: None
            // Fields count: 5
            //
            // Metadata:
            // NetworkVarNames: m_unDefIdx (item_definition_index_t)
            // NetworkVarNames: m_nCost (int)
            // NetworkVarNames: m_nPrevArmor (int)
            // NetworkVarNames: m_bPrevHelmet (bool)
            // NetworkVarNames: m_hItem (CEntityHandle)
            pub mod SellbackPurchaseEntry_t {
                pub const m_unDefIdx: usize = 0x30; // uint16
                pub const m_nCost: usize = 0x34; // int32
                pub const m_nPrevArmor: usize = 0x38; // int32
                pub const m_bPrevHelmet: usize = 0x3C; // bool
                pub const m_hItem: usize = 0x40; // CEntityHandle
            }
            // Parent: None
            // Fields count: 1
            //
            // Metadata:
            // NetworkVarNames: m_vecSellbackPurchaseEntries (SellbackPurchaseEntry_t)
            pub mod CCSPlayer_BuyServices {
                pub const m_vecSellbackPurchaseEntries: usize = 0xC8; // CUtlVectorEmbeddedNetworkVar<SellbackPurchaseEntry_t>
            }
            // Parent: None
            // Fields count: 2
            //
            // Metadata:
            // NetworkVarNames: m_hCarriedHostage (CHandle<CBaseEntity>)
            // NetworkVarNames: m_hCarriedHostageProp (CHandle<CBaseEntity>)
            pub mod CCSPlayer_HostageServices {
                pub const m_hCarriedHostage: usize = 0x40; // CHandle<CBaseEntity>
                pub const m_hCarriedHostageProp: usize = 0x44; // CHandle<CBaseEntity>
            }
            // Parent: None
            // Fields count: 3
            //
            // Metadata:
            // NetworkVarNames: m_bHasDefuser (bool)
            // NetworkVarNames: m_bHasHelmet (bool)
            // NetworkVarNames: m_bHasHeavyArmor (bool)
            pub mod CCSPlayer_ItemServices {
                pub const m_bHasDefuser: usize = 0x40; // bool
                pub const m_bHasHelmet: usize = 0x41; // bool
                pub const m_bHasHeavyArmor: usize = 0x42; // bool
            }
            // Parent: CPlayer_MovementServices_Humanoid
            // Fields count: 39
            //
            // Metadata:
            // NetworkVarNames: m_flMaxFallVelocity (float)
            // NetworkVarNames: m_vecLadderNormal (Vector)
            // NetworkVarNames: m_nLadderSurfacePropIndex (int)
            // NetworkVarNames: m_flDuckAmount (float)
            // NetworkVarNames: m_flDuckSpeed (float)
            // NetworkVarNames: m_bDuckOverride (bool)
            // NetworkVarNames: m_bDesiresDuck (bool)
            // NetworkVarNames: m_nDuckTimeMsecs (uint32)
            // NetworkVarNames: m_nDuckJumpTimeMsecs (uint32)
            // NetworkVarNames: m_nJumpTimeMsecs (uint32)
            // NetworkVarNames: m_flLastDuckTime (float)
            // NetworkVarNames: m_nGameCodeHasMovedPlayerAfterCommand (int)
            // NetworkVarNames: m_bOldJumpPressed (bool)
            // NetworkVarNames: m_flJumpUntil (float)
            // NetworkVarNames: m_flJumpVel (float)
            // NetworkVarNames: m_fStashGrenadeParameterWhen (GameTime_t)
            // NetworkVarNames: m_nButtonDownMaskPrev (ButtonBitMask_t)
            // NetworkVarNames: m_flOffsetTickCompleteTime (float)
            // NetworkVarNames: m_flOffsetTickStashedSpeed (float)
            // NetworkVarNames: m_flStamina (float)
            pub mod CCSPlayer_MovementServices {
                pub const m_flMaxFallVelocity: usize = 0x224; // float32
                pub const m_vecLadderNormal: usize = 0x228; // Vector
                pub const m_nLadderSurfacePropIndex: usize = 0x234; // int32
                pub const m_flDuckAmount: usize = 0x238; // float32
                pub const m_flDuckSpeed: usize = 0x23C; // float32
                pub const m_bDuckOverride: usize = 0x240; // bool
                pub const m_bDesiresDuck: usize = 0x241; // bool
                pub const m_flDuckOffset: usize = 0x244; // float32
                pub const m_nDuckTimeMsecs: usize = 0x248; // uint32
                pub const m_nDuckJumpTimeMsecs: usize = 0x24C; // uint32
                pub const m_nJumpTimeMsecs: usize = 0x250; // uint32
                pub const m_flLastDuckTime: usize = 0x254; // float32
                pub const m_vecLastPositionAtFullCrouchSpeed: usize = 0x260; // Vector2D
                pub const m_duckUntilOnGround: usize = 0x268; // bool
                pub const m_bHasWalkMovedSinceLastJump: usize = 0x269; // bool
                pub const m_bInStuckTest: usize = 0x26A; // bool
                pub const m_flStuckCheckTime: usize = 0x278; // float32[64][2]
                pub const m_nTraceCount: usize = 0x478; // int32
                pub const m_StuckLast: usize = 0x47C; // int32
                pub const m_bSpeedCropped: usize = 0x480; // bool
                pub const m_nOldWaterLevel: usize = 0x484; // int32
                pub const m_flWaterEntryTime: usize = 0x488; // float32
                pub const m_vecForward: usize = 0x48C; // Vector
                pub const m_vecLeft: usize = 0x498; // Vector
                pub const m_vecUp: usize = 0x4A4; // Vector
                pub const m_nGameCodeHasMovedPlayerAfterCommand: usize = 0x4B0; // int32
                pub const m_bMadeFootstepNoise: usize = 0x4B4; // bool
                pub const m_iFootsteps: usize = 0x4B8; // int32
                pub const m_bOldJumpPressed: usize = 0x4BC; // bool
                pub const m_flJumpPressedTime: usize = 0x4C0; // float32
                pub const m_flJumpUntil: usize = 0x4C4; // float32
                pub const m_flJumpVel: usize = 0x4C8; // float32
                pub const m_fStashGrenadeParameterWhen: usize = 0x4CC; // GameTime_t
                pub const m_nButtonDownMaskPrev: usize = 0x4D0; // uint64
                pub const m_flOffsetTickCompleteTime: usize = 0x4D8; // float32
                pub const m_flOffsetTickStashedSpeed: usize = 0x4DC; // float32
                pub const m_flStamina: usize = 0x4E0; // float32
                pub const m_flHeightAtJumpStart: usize = 0x4E4; // float32
                pub const m_flMaxJumpHeightThisJump: usize = 0x4E8; // float32
            }
            // Parent: None
            // Fields count: 1
            //
            // Metadata:
            // NetworkVarNames: m_hViewModel (CHandle<CBaseViewModel>)
            pub mod CCSPlayer_ViewModelServices {
                pub const m_hViewModel: usize = 0x40; // CHandle<CBaseViewModel>[3]
            }
            // Parent: CPlayer_WeaponServices
            // Fields count: 14
            //
            // Metadata:
            // NetworkVarNames: m_flNextAttack (GameTime_t)
            // NetworkVarNames: m_bIsLookingAtWeapon (bool)
            // NetworkVarNames: m_bIsHoldingLookAtWeapon (bool)
            pub mod CCSPlayer_WeaponServices {
                pub const m_flNextAttack: usize = 0xA4; // GameTime_t
                pub const m_bIsLookingAtWeapon: usize = 0xA8; // bool
                pub const m_bIsHoldingLookAtWeapon: usize = 0xA9; // bool
                pub const m_hSavedWeapon: usize = 0xAC; // CHandle<CBasePlayerWeapon>
                pub const m_nTimeToMelee: usize = 0xB0; // int32
                pub const m_nTimeToSecondary: usize = 0xB4; // int32
                pub const m_nTimeToPrimary: usize = 0xB8; // int32
                pub const m_nTimeToSniperRifle: usize = 0xBC; // int32
                pub const m_bIsBeingGivenItem: usize = 0xC0; // bool
                pub const m_bIsPickingUpItemWithUse: usize = 0xC1; // bool
                pub const m_bPickedUpWeapon: usize = 0xC2; // bool
                pub const m_bDisableAutoDeploy: usize = 0xC3; // bool
                pub const m_nOldShootPositionHistoryCount: usize = 0xC4; // uint32
                pub const m_nOldInputHistoryCount: usize = 0x460; // uint32
            }
            // Parent: None
            // Fields count: 5
            //
            // Metadata:
            // NetworkVarNames: m_perRoundStats (CSPerRoundStats_t)
            // NetworkVarNames: m_matchStats (CSMatchStats_t)
            // NetworkVarNames: m_iNumRoundKills (int)
            // NetworkVarNames: m_iNumRoundKillsHeadshots (int)
            // NetworkVarNames: m_unTotalRoundDamageDealt (uint32)
            pub mod CCSPlayerController_ActionTrackingServices {
                pub const m_perRoundStats: usize = 0x40; // CUtlVectorEmbeddedNetworkVar<CSPerRoundStats_t>
                pub const m_matchStats: usize = 0x90; // CSMatchStats_t
                pub const m_iNumRoundKills: usize = 0x148; // int32
                pub const m_iNumRoundKillsHeadshots: usize = 0x14C; // int32
                pub const m_unTotalRoundDamageDealt: usize = 0x150; // uint32
            }
            // Parent: None
            // Fields count: 14
            //
            // Metadata:
            // NetworkVarNames: m_PlayerDamager (CHandle<CCSPlayerPawnBase>)
            // NetworkVarNames: m_PlayerRecipient (CHandle<CCSPlayerPawnBase>)
            // NetworkVarNames: m_hPlayerControllerDamager (CHandle<CCSPlayerController>)
            // NetworkVarNames: m_hPlayerControllerRecipient (CHandle<CCSPlayerController>)
            // NetworkVarNames: m_szPlayerDamagerName (CUtlString)
            // NetworkVarNames: m_szPlayerRecipientName (CUtlString)
            // NetworkVarNames: m_DamagerXuid (uint64)
            // NetworkVarNames: m_RecipientXuid (uint64)
            // NetworkVarNames: m_iDamage (int)
            // NetworkVarNames: m_iActualHealthRemoved (int)
            // NetworkVarNames: m_iNumHits (int)
            // NetworkVarNames: m_iLastBulletUpdate (int)
            // NetworkVarNames: m_bIsOtherEnemy (bool)
            // NetworkVarNames: m_killType (EKillTypes_t)
            pub mod CDamageRecord {
                pub const m_PlayerDamager: usize = 0x28; // CHandle<CCSPlayerPawnBase>
                pub const m_PlayerRecipient: usize = 0x2C; // CHandle<CCSPlayerPawnBase>
                pub const m_hPlayerControllerDamager: usize = 0x30; // CHandle<CCSPlayerController>
                pub const m_hPlayerControllerRecipient: usize = 0x34; // CHandle<CCSPlayerController>
                pub const m_szPlayerDamagerName: usize = 0x38; // CUtlString
                pub const m_szPlayerRecipientName: usize = 0x40; // CUtlString
                pub const m_DamagerXuid: usize = 0x48; // uint64
                pub const m_RecipientXuid: usize = 0x50; // uint64
                pub const m_iDamage: usize = 0x58; // int32
                pub const m_iActualHealthRemoved: usize = 0x5C; // int32
                pub const m_iNumHits: usize = 0x60; // int32
                pub const m_iLastBulletUpdate: usize = 0x64; // int32
                pub const m_bIsOtherEnemy: usize = 0x68; // bool
                pub const m_killType: usize = 0x69; // EKillTypes_t
            }
            // Parent: None
            // Fields count: 2
            //
            // Metadata:
            // NetworkVarNames: m_nSendUpdate (int)
            // NetworkVarNames: m_DamageList (CDamageRecord)
            pub mod CCSPlayerController_DamageServices {
                pub const m_nSendUpdate: usize = 0x40; // int32
                pub const m_DamageList: usize = 0x48; // CUtlVectorEmbeddedNetworkVar<CDamageRecord>
            }
            // Parent: None
            // Fields count: 6
            //
            // Metadata:
            // NetworkVarNames: m_iAccount (int)
            // NetworkVarNames: m_iStartAccount (int)
            // NetworkVarNames: m_iTotalCashSpent (int)
            // NetworkVarNames: m_iCashSpentThisRound (int)
            pub mod CCSPlayerController_InGameMoneyServices {
                pub const m_bReceivesMoneyNextRound: usize = 0x40; // bool
                pub const m_iAccountMoneyEarnedForNextRound: usize = 0x44; // int32
                pub const m_iAccount: usize = 0x48; // int32
                pub const m_iStartAccount: usize = 0x4C; // int32
                pub const m_iTotalCashSpent: usize = 0x50; // int32
                pub const m_iCashSpentThisRound: usize = 0x54; // int32
            }
            // Parent: None
            // Fields count: 9
            //
            // Metadata:
            // NetworkVarNames: m_unMusicID (item_definition_index_t)
            // NetworkVarNames: m_rank (MedalRank_t)
            // NetworkVarNames: m_nPersonaDataPublicLevel (int)
            // NetworkVarNames: m_nPersonaDataPublicCommendsLeader (int)
            // NetworkVarNames: m_nPersonaDataPublicCommendsTeacher (int)
            // NetworkVarNames: m_nPersonaDataPublicCommendsFriendly (int)
            // NetworkVarNames: m_nPersonaDataXpTrailLevel (int)
            // NetworkVarNames: m_vecServerAuthoritativeWeaponSlots (ServerAuthoritativeWeaponSlot_t)
            pub mod CCSPlayerController_InventoryServices {
                pub const m_unMusicID: usize = 0x40; // uint16
                pub const m_rank: usize = 0x44; // MedalRank_t[6]
                pub const m_nPersonaDataPublicLevel: usize = 0x5C; // int32
                pub const m_nPersonaDataPublicCommendsLeader: usize = 0x60; // int32
                pub const m_nPersonaDataPublicCommendsTeacher: usize = 0x64; // int32
                pub const m_nPersonaDataPublicCommendsFriendly: usize = 0x68; // int32
                pub const m_nPersonaDataXpTrailLevel: usize = 0x6C; // int32
                pub const m_unEquippedPlayerSprayIDs: usize = 0xF48; // uint32[1]
                pub const m_vecServerAuthoritativeWeaponSlots: usize = 0xF50; // CUtlVectorEmbeddedNetworkVar<ServerAuthoritativeWeaponSlot_t>
            }
            // Parent: None
            // Fields count: 0
            //
            // Metadata:
            // MPulseProvideFeatureTag
            // MPulseLibraryBindings
            pub mod CCSWeaponBaseVDataAPI {
            }
            // Parent: None
            // Fields count: 0
            //
            // Metadata:
            // MPulseProvideFeatureTag
            // MPulseLibraryBindings
            pub mod CCSWeaponBaseAPI {
            }
            // Parent: None
            // Fields count: 1
            //
            // Metadata:
            // MPropertyCustomEditor
            pub mod CFiringModeFloat {
                pub const m_flValues: usize = 0x0; // float32[2]
            }
            // Parent: None
            // Fields count: 1
            //
            // Metadata:
            // MPropertyCustomEditor
            pub mod CFiringModeInt {
                pub const m_nValues: usize = 0x0; // int32[2]
            }
            // Parent: None
            // Fields count: 0
            //
            // Metadata:
            // MPulseProvideFeatureTag
            // MPulseLibraryBindings
            pub mod CCSObserverPawnAPI {
            }
            // Parent: None
            // Fields count: 0
            //
            // Metadata:
            // MPulseProvideFeatureTag
            // MPulseLibraryBindings
            pub mod CCSPlayerPawnAPI {
            }
            // Parent: CBaseEntity
            // Fields count: 14
            //
            // Metadata:
            // NetworkVarNames: m_iActiveIssueIndex (int)
            // NetworkVarNames: m_iOnlyTeamToVote (int)
            // NetworkVarNames: m_nVoteOptionCount (int)
            // NetworkVarNames: m_nPotentialVotes (int)
            // NetworkVarNames: m_bIsYesNoVote (bool)
            pub mod CVoteController {
                pub const m_iActiveIssueIndex: usize = 0x7A8; // int32
                pub const m_iOnlyTeamToVote: usize = 0x7AC; // int32
                pub const m_nVoteOptionCount: usize = 0x7B0; // int32[5]
                pub const m_nPotentialVotes: usize = 0x7C4; // int32
                pub const m_bIsYesNoVote: usize = 0x7C8; // bool
                pub const m_acceptingVotesTimer: usize = 0x7D0; // CountdownTimer
                pub const m_executeCommandTimer: usize = 0x7E8; // CountdownTimer
                pub const m_resetVoteTimer: usize = 0x800; // CountdownTimer
                pub const m_nVotesCast: usize = 0x818; // int32[64]
                pub const m_playerHoldingVote: usize = 0x918; // CPlayerSlot
                pub const m_playerOverrideForVote: usize = 0x91C; // CPlayerSlot
                pub const m_nHighestCountIndex: usize = 0x920; // int32
                pub const m_potentialIssues: usize = 0x928; // CUtlVector<CBaseIssue*>
                pub const m_VoteOptions: usize = 0x940; // CUtlVector<char*>
            }
            // Parent: CBaseEntity
            // Fields count: 24
            //
            // Metadata:
            // NetworkVarNames: m_nDraftType (int)
            // NetworkVarNames: m_nTeamWinningCoinToss (int)
            // NetworkVarNames: m_nTeamWithFirstChoice (int)
            // NetworkVarNames: m_nVoteMapIdsList (int)
            // NetworkVarNames: m_nAccountIDs (int)
            // NetworkVarNames: m_nMapId0 (int)
            // NetworkVarNames: m_nMapId1 (int)
            // NetworkVarNames: m_nMapId2 (int)
            // NetworkVarNames: m_nMapId3 (int)
            // NetworkVarNames: m_nMapId4 (int)
            // NetworkVarNames: m_nMapId5 (int)
            // NetworkVarNames: m_nStartingSide0 (int)
            // NetworkVarNames: m_nCurrentPhase (int)
            // NetworkVarNames: m_nPhaseStartTick (int)
            // NetworkVarNames: m_nPhaseDurationTicks (int)
            pub mod CMapVetoPickController {
                pub const m_bPlayedIntroVcd: usize = 0x7A8; // bool
                pub const m_bNeedToPlayFiveSecondsRemaining: usize = 0x7A9; // bool
                pub const m_dblPreMatchDraftSequenceTime: usize = 0x7C8; // float64
                pub const m_bPreMatchDraftStateChanged: usize = 0x7D0; // bool
                pub const m_nDraftType: usize = 0x7D4; // int32
                pub const m_nTeamWinningCoinToss: usize = 0x7D8; // int32
                pub const m_nTeamWithFirstChoice: usize = 0x7DC; // int32[64]
                pub const m_nVoteMapIdsList: usize = 0x8DC; // int32[7]
                pub const m_nAccountIDs: usize = 0x8F8; // int32[64]
                pub const m_nMapId0: usize = 0x9F8; // int32[64]
                pub const m_nMapId1: usize = 0xAF8; // int32[64]
                pub const m_nMapId2: usize = 0xBF8; // int32[64]
                pub const m_nMapId3: usize = 0xCF8; // int32[64]
                pub const m_nMapId4: usize = 0xDF8; // int32[64]
                pub const m_nMapId5: usize = 0xEF8; // int32[64]
                pub const m_nStartingSide0: usize = 0xFF8; // int32[64]
                pub const m_nCurrentPhase: usize = 0x10F8; // int32
                pub const m_nPhaseStartTick: usize = 0x10FC; // int32
                pub const m_nPhaseDurationTicks: usize = 0x1100; // int32
                pub const m_OnMapVetoed: usize = 0x1108; // CEntityOutputTemplate<CUtlSymbolLarge>
                pub const m_OnMapPicked: usize = 0x1130; // CEntityOutputTemplate<CUtlSymbolLarge>
                pub const m_OnSidesPicked: usize = 0x1158; // CEntityOutputTemplate<int32>
                pub const m_OnNewPhaseStarted: usize = 0x1180; // CEntityOutputTemplate<int32>
                pub const m_OnLevelTransition: usize = 0x11A8; // CEntityOutputTemplate<int32>
            }
            // Parent: None
            // Fields count: 0
            //
            // Metadata:
            // MPulseLibraryBindings
            // MPropertyFriendlyName
            // MPropertyDescription
            pub mod CCSPulseServerFuncs_Globals {
            }
            // Parent: CBaseEntity
            // Fields count: 24
            //
            // Metadata:
            // NetworkVarNames: m_Color (Color)
            // NetworkVarNames: m_flBrightness (float)
            // NetworkVarNames: m_hCubemapTexture (HRenderTextureStrong)
            // NetworkVarNames: m_bCustomCubemapTexture (bool)
            // NetworkVarNames: m_hLightProbeTexture (HRenderTextureStrong)
            // NetworkVarNames: m_hLightProbeDirectLightIndicesTexture (HRenderTextureStrong)
            // NetworkVarNames: m_hLightProbeDirectLightScalarsTexture (HRenderTextureStrong)
            // NetworkVarNames: m_hLightProbeDirectLightShadowsTexture (HRenderTextureStrong)
            // NetworkVarNames: m_vBoxMins (Vector)
            // NetworkVarNames: m_vBoxMaxs (Vector)
            // NetworkVarNames: m_bMoveable (bool)
            // NetworkVarNames: m_nHandshake (int)
            // NetworkVarNames: m_nEnvCubeMapArrayIndex (int)
            // NetworkVarNames: m_nPriority (int)
            // NetworkVarNames: m_bStartDisabled (bool)
            // NetworkVarNames: m_flEdgeFadeDist (float)
            // NetworkVarNames: m_vEdgeFadeDists (Vector)
            // NetworkVarNames: m_nLightProbeSizeX (int)
            // NetworkVarNames: m_nLightProbeSizeY (int)
            // NetworkVarNames: m_nLightProbeSizeZ (int)
            // NetworkVarNames: m_nLightProbeAtlasX (int)
            // NetworkVarNames: m_nLightProbeAtlasY (int)
            // NetworkVarNames: m_nLightProbeAtlasZ (int)
            // NetworkVarNames: m_bEnabled (bool)
            pub mod CEnvCombinedLightProbeVolume {
                pub const m_Color: usize = 0x1800; // Color
                pub const m_flBrightness: usize = 0x1804; // float32
                pub const m_hCubemapTexture: usize = 0x1808; // CStrongHandle<InfoForResourceTypeCTextureBase>
                pub const m_bCustomCubemapTexture: usize = 0x1810; // bool
                pub const m_hLightProbeTexture: usize = 0x1818; // CStrongHandle<InfoForResourceTypeCTextureBase>
                pub const m_hLightProbeDirectLightIndicesTexture: usize = 0x1820; // CStrongHandle<InfoForResourceTypeCTextureBase>
                pub const m_hLightProbeDirectLightScalarsTexture: usize = 0x1828; // CStrongHandle<InfoForResourceTypeCTextureBase>
                pub const m_hLightProbeDirectLightShadowsTexture: usize = 0x1830; // CStrongHandle<InfoForResourceTypeCTextureBase>
                pub const m_vBoxMins: usize = 0x1838; // Vector
                pub const m_vBoxMaxs: usize = 0x1844; // Vector
                pub const m_bMoveable: usize = 0x1850; // bool
                pub const m_nHandshake: usize = 0x1854; // int32
                pub const m_nEnvCubeMapArrayIndex: usize = 0x1858; // int32
                pub const m_nPriority: usize = 0x185C; // int32
                pub const m_bStartDisabled: usize = 0x1860; // bool
                pub const m_flEdgeFadeDist: usize = 0x1864; // float32
                pub const m_vEdgeFadeDists: usize = 0x1868; // Vector
                pub const m_nLightProbeSizeX: usize = 0x1874; // int32
                pub const m_nLightProbeSizeY: usize = 0x1878; // int32
                pub const m_nLightProbeSizeZ: usize = 0x187C; // int32
                pub const m_nLightProbeAtlasX: usize = 0x1880; // int32
                pub const m_nLightProbeAtlasY: usize = 0x1884; // int32
                pub const m_nLightProbeAtlasZ: usize = 0x1888; // int32
                pub const m_bEnabled: usize = 0x18A1; // bool
            }
            // Parent: CBaseEntity
            // Fields count: 18
            //
            // Metadata:
            // NetworkVarNames: m_hCubemapTexture (HRenderTextureStrong)
            // NetworkVarNames: m_bCustomCubemapTexture (bool)
            // NetworkVarNames: m_flInfluenceRadius (float)
            // NetworkVarNames: m_vBoxProjectMins (Vector)
            // NetworkVarNames: m_vBoxProjectMaxs (Vector)
            // NetworkVarNames: m_bMoveable (bool)
            // NetworkVarNames: m_nHandshake (int)
            // NetworkVarNames: m_nEnvCubeMapArrayIndex (int)
            // NetworkVarNames: m_nPriority (int)
            // NetworkVarNames: m_flEdgeFadeDist (float)
            // NetworkVarNames: m_vEdgeFadeDists (Vector)
            // NetworkVarNames: m_flDiffuseScale (float)
            // NetworkVarNames: m_bStartDisabled (bool)
            // NetworkVarNames: m_bDefaultEnvMap (bool)
            // NetworkVarNames: m_bDefaultSpecEnvMap (bool)
            // NetworkVarNames: m_bIndoorCubeMap (bool)
            // NetworkVarNames: m_bCopyDiffuseFromDefaultCubemap (bool)
            // NetworkVarNames: m_bEnabled (bool)
            pub mod CEnvCubemap {
                pub const m_hCubemapTexture: usize = 0x828; // CStrongHandle<InfoForResourceTypeCTextureBase>
                pub const m_bCustomCubemapTexture: usize = 0x830; // bool
                pub const m_flInfluenceRadius: usize = 0x834; // float32
                pub const m_vBoxProjectMins: usize = 0x838; // Vector
                pub const m_vBoxProjectMaxs: usize = 0x844; // Vector
                pub const m_bMoveable: usize = 0x850; // bool
                pub const m_nHandshake: usize = 0x854; // int32
                pub const m_nEnvCubeMapArrayIndex: usize = 0x858; // int32
                pub const m_nPriority: usize = 0x85C; // int32
                pub const m_flEdgeFadeDist: usize = 0x860; // float32
                pub const m_vEdgeFadeDists: usize = 0x864; // Vector
                pub const m_flDiffuseScale: usize = 0x870; // float32
                pub const m_bStartDisabled: usize = 0x874; // bool
                pub const m_bDefaultEnvMap: usize = 0x875; // bool
                pub const m_bDefaultSpecEnvMap: usize = 0x876; // bool
                pub const m_bIndoorCubeMap: usize = 0x877; // bool
                pub const m_bCopyDiffuseFromDefaultCubemap: usize = 0x878; // bool
                pub const m_bEnabled: usize = 0x888; // bool
            }
            // Parent: CBaseEntity
            // Fields count: 18
            //
            // Metadata:
            // NetworkVarNames: m_flEndDistance (float)
            // NetworkVarNames: m_flStartDistance (float)
            // NetworkVarNames: m_flFogFalloffExponent (float)
            // NetworkVarNames: m_bHeightFogEnabled (bool)
            // NetworkVarNames: m_flFogHeightWidth (float)
            // NetworkVarNames: m_flFogHeightEnd (float)
            // NetworkVarNames: m_flFogHeightStart (float)
            // NetworkVarNames: m_flFogHeightExponent (float)
            // NetworkVarNames: m_flLODBias (float)
            // NetworkVarNames: m_bActive (bool)
            // NetworkVarNames: m_bStartDisabled (bool)
            // NetworkVarNames: m_flFogMaxOpacity (float)
            // NetworkVarNames: m_nCubemapSourceType (int)
            // NetworkVarNames: m_hSkyMaterial (HMaterialStrong)
            // NetworkVarNames: m_iszSkyEntity (string_t)
            // NetworkVarNames: m_hFogCubemapTexture (HRenderTextureStrong)
            // NetworkVarNames: m_bHasHeightFogEnd (bool)
            pub mod CEnvCubemapFog {
                pub const m_flEndDistance: usize = 0x7A8; // float32
                pub const m_flStartDistance: usize = 0x7AC; // float32
                pub const m_flFogFalloffExponent: usize = 0x7B0; // float32
                pub const m_bHeightFogEnabled: usize = 0x7B4; // bool
                pub const m_flFogHeightWidth: usize = 0x7B8; // float32
                pub const m_flFogHeightEnd: usize = 0x7BC; // float32
                pub const m_flFogHeightStart: usize = 0x7C0; // float32
                pub const m_flFogHeightExponent: usize = 0x7C4; // float32
                pub const m_flLODBias: usize = 0x7C8; // float32
                pub const m_bActive: usize = 0x7CC; // bool
                pub const m_bStartDisabled: usize = 0x7CD; // bool
                pub const m_flFogMaxOpacity: usize = 0x7D0; // float32
                pub const m_nCubemapSourceType: usize = 0x7D4; // int32
                pub const m_hSkyMaterial: usize = 0x7D8; // CStrongHandle<InfoForResourceTypeIMaterial2>
                pub const m_iszSkyEntity: usize = 0x7E0; // CUtlSymbolLarge
                pub const m_hFogCubemapTexture: usize = 0x7E8; // CStrongHandle<InfoForResourceTypeCTextureBase>
                pub const m_bHasHeightFogEnd: usize = 0x7F0; // bool
                pub const m_bFirstTime: usize = 0x7F1; // bool
            }
            // Parent: CBaseEntity
            // Fields count: 16
            //
            // Metadata:
            // NetworkVarNames: m_hGradientFogTexture (HRenderTextureStrong)
            // NetworkVarNames: m_flFogStartDistance (float)
            // NetworkVarNames: m_flFogEndDistance (float)
            // NetworkVarNames: m_bHeightFogEnabled (bool)
            // NetworkVarNames: m_flFogStartHeight (float)
            // NetworkVarNames: m_flFogEndHeight (float)
            // NetworkVarNames: m_flFarZ (float)
            // NetworkVarNames: m_flFogMaxOpacity (float)
            // NetworkVarNames: m_flFogFalloffExponent (float)
            // NetworkVarNames: m_flFogVerticalExponent (float)
            // NetworkVarNames: m_fogColor (Color)
            // NetworkVarNames: m_flFogStrength (float)
            // NetworkVarNames: m_flFadeTime (float)
            // NetworkVarNames: m_bStartDisabled (bool)
            // NetworkVarNames: m_bIsEnabled (bool)
            pub mod CGradientFog {
                pub const m_hGradientFogTexture: usize = 0x7A8; // CStrongHandle<InfoForResourceTypeCTextureBase>
                pub const m_flFogStartDistance: usize = 0x7B0; // float32
                pub const m_flFogEndDistance: usize = 0x7B4; // float32
                pub const m_bHeightFogEnabled: usize = 0x7B8; // bool
                pub const m_flFogStartHeight: usize = 0x7BC; // float32
                pub const m_flFogEndHeight: usize = 0x7C0; // float32
                pub const m_flFarZ: usize = 0x7C4; // float32
                pub const m_flFogMaxOpacity: usize = 0x7C8; // float32
                pub const m_flFogFalloffExponent: usize = 0x7CC; // float32
                pub const m_flFogVerticalExponent: usize = 0x7D0; // float32
                pub const m_fogColor: usize = 0x7D4; // Color
                pub const m_flFogStrength: usize = 0x7D8; // float32
                pub const m_flFadeTime: usize = 0x7DC; // float32
                pub const m_bStartDisabled: usize = 0x7E0; // bool
                pub const m_bIsEnabled: usize = 0x7E1; // bool
                pub const m_bGradientFogNeedsTextures: usize = 0x7E2; // bool
            }
            // Parent: CBaseEntity
            // Fields count: 17
            //
            // Metadata:
            // NetworkVarNames: m_hLightProbeTexture (HRenderTextureStrong)
            // NetworkVarNames: m_hLightProbeDirectLightIndicesTexture (HRenderTextureStrong)
            // NetworkVarNames: m_hLightProbeDirectLightScalarsTexture (HRenderTextureStrong)
            // NetworkVarNames: m_hLightProbeDirectLightShadowsTexture (HRenderTextureStrong)
            // NetworkVarNames: m_vBoxMins (Vector)
            // NetworkVarNames: m_vBoxMaxs (Vector)
            // NetworkVarNames: m_bMoveable (bool)
            // NetworkVarNames: m_nHandshake (int)
            // NetworkVarNames: m_nPriority (int)
            // NetworkVarNames: m_bStartDisabled (bool)
            // NetworkVarNames: m_nLightProbeSizeX (int)
            // NetworkVarNames: m_nLightProbeSizeY (int)
            // NetworkVarNames: m_nLightProbeSizeZ (int)
            // NetworkVarNames: m_nLightProbeAtlasX (int)
            // NetworkVarNames: m_nLightProbeAtlasY (int)
            // NetworkVarNames: m_nLightProbeAtlasZ (int)
            // NetworkVarNames: m_bEnabled (bool)
            pub mod CEnvLightProbeVolume {
                pub const m_hLightProbeTexture: usize = 0x1780; // CStrongHandle<InfoForResourceTypeCTextureBase>
                pub const m_hLightProbeDirectLightIndicesTexture: usize = 0x1788; // CStrongHandle<InfoForResourceTypeCTextureBase>
                pub const m_hLightProbeDirectLightScalarsTexture: usize = 0x1790; // CStrongHandle<InfoForResourceTypeCTextureBase>
                pub const m_hLightProbeDirectLightShadowsTexture: usize = 0x1798; // CStrongHandle<InfoForResourceTypeCTextureBase>
                pub const m_vBoxMins: usize = 0x17A0; // Vector
                pub const m_vBoxMaxs: usize = 0x17AC; // Vector
                pub const m_bMoveable: usize = 0x17B8; // bool
                pub const m_nHandshake: usize = 0x17BC; // int32
                pub const m_nPriority: usize = 0x17C0; // int32
                pub const m_bStartDisabled: usize = 0x17C4; // bool
                pub const m_nLightProbeSizeX: usize = 0x17C8; // int32
                pub const m_nLightProbeSizeY: usize = 0x17CC; // int32
                pub const m_nLightProbeSizeZ: usize = 0x17D0; // int32
                pub const m_nLightProbeAtlasX: usize = 0x17D4; // int32
                pub const m_nLightProbeAtlasY: usize = 0x17D8; // int32
                pub const m_nLightProbeAtlasZ: usize = 0x17DC; // int32
                pub const m_bEnabled: usize = 0x17E9; // bool
            }
            // Parent: CBaseEntity
            // Fields count: 6
            //
            // Metadata:
            // NetworkVarNames: m_flVisibilityStrength (float)
            // NetworkVarNames: m_flFogDistanceMultiplier (float)
            // NetworkVarNames: m_flFogMaxDensityMultiplier (float)
            // NetworkVarNames: m_flFadeTime (float)
            // NetworkVarNames: m_bStartDisabled (bool)
            // NetworkVarNames: m_bIsEnabled (bool)
            pub mod CPlayerVisibility {
                pub const m_flVisibilityStrength: usize = 0x7A8; // float32
                pub const m_flFogDistanceMultiplier: usize = 0x7AC; // float32
                pub const m_flFogMaxDensityMultiplier: usize = 0x7B0; // float32
                pub const m_flFadeTime: usize = 0x7B4; // float32
                pub const m_bStartDisabled: usize = 0x7B8; // bool
                pub const m_bIsEnabled: usize = 0x7B9; // bool
            }
            // Parent: CBaseEntity
            // Fields count: 8
            //
            // Metadata:
            // NetworkVarNames: m_flAutoExposureMin (float)
            // NetworkVarNames: m_flAutoExposureMax (float)
            // NetworkVarNames: m_flTonemapPercentTarget (float)
            // NetworkVarNames: m_flTonemapPercentBrightPixels (float)
            // NetworkVarNames: m_flTonemapMinAvgLum (float)
            // NetworkVarNames: m_flExposureAdaptationSpeedUp (float)
            // NetworkVarNames: m_flExposureAdaptationSpeedDown (float)
            // NetworkVarNames: m_flTonemapEVSmoothingRange (float)
            pub mod CTonemapController2 {
                pub const m_flAutoExposureMin: usize = 0x7A8; // float32
                pub const m_flAutoExposureMax: usize = 0x7AC; // float32
                pub const m_flTonemapPercentTarget: usize = 0x7B0; // float32
                pub const m_flTonemapPercentBrightPixels: usize = 0x7B4; // float32
                pub const m_flTonemapMinAvgLum: usize = 0x7B8; // float32
                pub const m_flExposureAdaptationSpeedUp: usize = 0x7BC; // float32
                pub const m_flExposureAdaptationSpeedDown: usize = 0x7C0; // float32
                pub const m_flTonemapEVSmoothingRange: usize = 0x7C4; // float32
            }
            // Parent: CBaseEntity
            // Fields count: 28
            //
            // Metadata:
            // NetworkVarNames: m_flScattering (float)
            // NetworkVarNames: m_flAnisotropy (float)
            // NetworkVarNames: m_flFadeSpeed (float)
            // NetworkVarNames: m_flDrawDistance (float)
            // NetworkVarNames: m_flFadeInStart (float)
            // NetworkVarNames: m_flFadeInEnd (float)
            // NetworkVarNames: m_flIndirectStrength (float)
            // NetworkVarNames: m_nIndirectTextureDimX (int)
            // NetworkVarNames: m_nIndirectTextureDimY (int)
            // NetworkVarNames: m_nIndirectTextureDimZ (int)
            // NetworkVarNames: m_vBoxMins (Vector)
            // NetworkVarNames: m_vBoxMaxs (Vector)
            // NetworkVarNames: m_bActive (bool)
            // NetworkVarNames: m_flStartAnisoTime (GameTime_t)
            // NetworkVarNames: m_flStartScatterTime (GameTime_t)
            // NetworkVarNames: m_flStartDrawDistanceTime (GameTime_t)
            // NetworkVarNames: m_flStartAnisotropy (float)
            // NetworkVarNames: m_flStartScattering (float)
            // NetworkVarNames: m_flStartDrawDistance (float)
            // NetworkVarNames: m_flDefaultAnisotropy (float)
            // NetworkVarNames: m_flDefaultScattering (float)
            // NetworkVarNames: m_flDefaultDrawDistance (float)
            // NetworkVarNames: m_bStartDisabled (bool)
            // NetworkVarNames: m_bEnableIndirect (bool)
            // NetworkVarNames: m_bIsMaster (bool)
            // NetworkVarNames: m_hFogIndirectTexture (HRenderTextureStrong)
            // NetworkVarNames: m_nForceRefreshCount (int)
            pub mod CEnvVolumetricFogController {
                pub const m_flScattering: usize = 0x7A8; // float32
                pub const m_flAnisotropy: usize = 0x7AC; // float32
                pub const m_flFadeSpeed: usize = 0x7B0; // float32
                pub const m_flDrawDistance: usize = 0x7B4; // float32
                pub const m_flFadeInStart: usize = 0x7B8; // float32
                pub const m_flFadeInEnd: usize = 0x7BC; // float32
                pub const m_flIndirectStrength: usize = 0x7C0; // float32
                pub const m_nIndirectTextureDimX: usize = 0x7C4; // int32
                pub const m_nIndirectTextureDimY: usize = 0x7C8; // int32
                pub const m_nIndirectTextureDimZ: usize = 0x7CC; // int32
                pub const m_vBoxMins: usize = 0x7D0; // Vector
                pub const m_vBoxMaxs: usize = 0x7DC; // Vector
                pub const m_bActive: usize = 0x7E8; // bool
                pub const m_flStartAnisoTime: usize = 0x7EC; // GameTime_t
                pub const m_flStartScatterTime: usize = 0x7F0; // GameTime_t
                pub const m_flStartDrawDistanceTime: usize = 0x7F4; // GameTime_t
                pub const m_flStartAnisotropy: usize = 0x7F8; // float32
                pub const m_flStartScattering: usize = 0x7FC; // float32
                pub const m_flStartDrawDistance: usize = 0x800; // float32
                pub const m_flDefaultAnisotropy: usize = 0x804; // float32
                pub const m_flDefaultScattering: usize = 0x808; // float32
                pub const m_flDefaultDrawDistance: usize = 0x80C; // float32
                pub const m_bStartDisabled: usize = 0x810; // bool
                pub const m_bEnableIndirect: usize = 0x811; // bool
                pub const m_bIsMaster: usize = 0x812; // bool
                pub const m_hFogIndirectTexture: usize = 0x818; // CStrongHandle<InfoForResourceTypeCTextureBase>
                pub const m_nForceRefreshCount: usize = 0x820; // int32
                pub const m_bFirstTime: usize = 0x824; // bool
            }
            // Parent: CBaseEntity
            // Fields count: 7
            //
            // Metadata:
            // NetworkVarNames: m_bActive (bool)
            // NetworkVarNames: m_vBoxMins (Vector)
            // NetworkVarNames: m_vBoxMaxs (Vector)
            // NetworkVarNames: m_bStartDisabled (bool)
            // NetworkVarNames: m_flStrength (float)
            // NetworkVarNames: m_nFalloffShape (int)
            // NetworkVarNames: m_flFalloffExponent (float)
            pub mod CEnvVolumetricFogVolume {
                pub const m_bActive: usize = 0x7A8; // bool
                pub const m_vBoxMins: usize = 0x7AC; // Vector
                pub const m_vBoxMaxs: usize = 0x7B8; // Vector
                pub const m_bStartDisabled: usize = 0x7C4; // bool
                pub const m_flStrength: usize = 0x7C8; // float32
                pub const m_nFalloffShape: usize = 0x7CC; // int32
                pub const m_flFalloffExponent: usize = 0x7D0; // float32
            }
            // Parent: CBaseEntity
            // Fields count: 3
            //
            // Metadata:
            // NetworkVarNames: m_fog (fogparams_t)
            pub mod CFogController {
                pub const m_fog: usize = 0x7A8; // fogparams_t
                pub const m_bUseAngles: usize = 0x810; // bool
                pub const m_iChangedVariables: usize = 0x814; // int32
            }
            // Parent: CBaseEntity
            // Fields count: 3
            //
            // Metadata:
            // NetworkVarNames: m_nMode (int)
            // NetworkVarNames: m_vBoxSize (Vector)
            // NetworkVarNames: m_bEnabled (bool)
            pub mod CInfoVisibilityBox {
                pub const m_nMode: usize = 0x7AC; // int32
                pub const m_vBoxSize: usize = 0x7B0; // Vector
                pub const m_bEnabled: usize = 0x7BC; // bool
            }
            // Parent: CBaseEntity
            // Fields count: 7
            //
            // Metadata:
            // NetworkVarNames: m_worldName (string_t)
            // NetworkVarNames: m_layerName (string_t)
            // NetworkVarNames: m_bWorldLayerVisible (bool)
            // NetworkVarNames: m_bEntitiesSpawned (bool)
            pub mod CInfoWorldLayer {
                pub const m_pOutputOnEntitiesSpawned: usize = 0x7A8; // CEntityIOOutput
                pub const m_worldName: usize = 0x7D0; // CUtlSymbolLarge
                pub const m_layerName: usize = 0x7D8; // CUtlSymbolLarge
                pub const m_bWorldLayerVisible: usize = 0x7E0; // bool
                pub const m_bEntitiesSpawned: usize = 0x7E1; // bool
                pub const m_bCreateAsChildSpawnGroup: usize = 0x7E2; // bool
                pub const m_hLayerSpawnGroup: usize = 0x7E4; // uint32
            }
            // Parent: CBaseEntity
            // Fields count: 25
            //
            // Metadata:
            // NetworkVarNames: m_FOV (float)
            // NetworkVarNames: m_Resolution (float)
            // NetworkVarNames: m_bFogEnable (bool)
            // NetworkVarNames: m_FogColor (Color)
            // NetworkVarNames: m_flFogStart (float)
            // NetworkVarNames: m_flFogEnd (float)
            // NetworkVarNames: m_flFogMaxDensity (float)
            // NetworkVarNames: m_bActive (bool)
            // NetworkVarNames: m_bUseScreenAspectRatio (bool)
            // NetworkVarNames: m_flAspectRatio (float)
            // NetworkVarNames: m_bNoSky (bool)
            // NetworkVarNames: m_fBrightness (float)
            // NetworkVarNames: m_flZFar (float)
            // NetworkVarNames: m_flZNear (float)
            // NetworkVarNames: m_bCanHLTVUse (bool)
            // NetworkVarNames: m_bDofEnabled (bool)
            // NetworkVarNames: m_flDofNearBlurry (float)
            // NetworkVarNames: m_flDofNearCrisp (float)
            // NetworkVarNames: m_flDofFarCrisp (float)
            // NetworkVarNames: m_flDofFarBlurry (float)
            // NetworkVarNames: m_flDofTiltToGround (float)
            pub mod CPointCamera {
                pub const m_FOV: usize = 0x7A8; // float32
                pub const m_Resolution: usize = 0x7AC; // float32
                pub const m_bFogEnable: usize = 0x7B0; // bool
                pub const m_FogColor: usize = 0x7B1; // Color
                pub const m_flFogStart: usize = 0x7B8; // float32
                pub const m_flFogEnd: usize = 0x7BC; // float32
                pub const m_flFogMaxDensity: usize = 0x7C0; // float32
                pub const m_bActive: usize = 0x7C4; // bool
                pub const m_bUseScreenAspectRatio: usize = 0x7C5; // bool
                pub const m_flAspectRatio: usize = 0x7C8; // float32
                pub const m_bNoSky: usize = 0x7CC; // bool
                pub const m_fBrightness: usize = 0x7D0; // float32
                pub const m_flZFar: usize = 0x7D4; // float32
                pub const m_flZNear: usize = 0x7D8; // float32
                pub const m_bCanHLTVUse: usize = 0x7DC; // bool
                pub const m_bDofEnabled: usize = 0x7DD; // bool
                pub const m_flDofNearBlurry: usize = 0x7E0; // float32
                pub const m_flDofNearCrisp: usize = 0x7E4; // float32
                pub const m_flDofFarCrisp: usize = 0x7E8; // float32
                pub const m_flDofFarBlurry: usize = 0x7EC; // float32
                pub const m_flDofTiltToGround: usize = 0x7F0; // float32
                pub const m_TargetFOV: usize = 0x7F4; // float32
                pub const m_DegreesPerSecond: usize = 0x7F8; // float32
                pub const m_bIsOn: usize = 0x7FC; // bool
                pub const m_pNext: usize = 0x800; // CPointCamera*
            }
            // Parent: CBaseEntity
            // Fields count: 3
            //
            // Metadata:
            // NetworkVarNames: m_bDisabled (bool)
            // NetworkVarNames: m_iszSoundAreaType (string_t)
            // NetworkVarNames: m_vPos (Vector)
            pub mod CSoundAreaEntityBase {
                pub const m_bDisabled: usize = 0x7A8; // bool
                pub const m_iszSoundAreaType: usize = 0x7B0; // CUtlSymbolLarge
                pub const m_vPos: usize = 0x7B8; // Vector
            }
            // Parent: CSoundAreaEntityBase
            // Fields count: 1
            //
            // Metadata:
            // NetworkVarNames: m_flRadius (float)
            pub mod CSoundAreaEntitySphere {
                pub const m_flRadius: usize = 0x7C4; // float32
            }
            // Parent: CSoundAreaEntityBase
            // Fields count: 2
            //
            // Metadata:
            // NetworkVarNames: m_vMin (Vector)
            // NetworkVarNames: m_vMax (Vector)
            pub mod CSoundAreaEntityOrientedBox {
                pub const m_vMin: usize = 0x7C4; // Vector
                pub const m_vMax: usize = 0x7D0; // Vector
            }
            // Parent: CBaseEntity
            // Fields count: 4
            //
            // Metadata:
            // MNetworkIncludeByName
            // NetworkVarNames: m_aPlayerControllers (CHandle<CBasePlayerController>)
            // NetworkVarNames: m_aPlayers (CHandle<CBasePlayerPawn>)
            // NetworkVarNames: m_iScore (int32)
            // NetworkVarNames: m_szTeamname (char)
            pub mod CTeam {
                pub const m_aPlayerControllers: usize = 0x7A8; // CNetworkUtlVectorBase<CHandle<CBasePlayerController>>
                pub const m_aPlayers: usize = 0x7C0; // CNetworkUtlVectorBase<CHandle<CBasePlayerPawn>>
                pub const m_iScore: usize = 0x7D8; // int32
                pub const m_szTeamname: usize = 0x7DC; // char[129]
            }
            // Parent: CBaseEntity
            // Fields count: 25
            //
            // Metadata:
            // MNetworkIncludeByName
            // MNetworkIncludeByName
            // MNetworkIncludeByName
            // MNetworkIncludeByName
            // MNetworkIncludeByName
            // MNetworkIncludeByName
            // MNetworkUserGroupProxy
            // MNetworkUserGroupProxy
            // MNetworkIncludeByUserGroup
            // NetworkVarNames: m_nTickBase (uint32)
            // NetworkVarNames: m_hPawn (CHandle<CBasePlayerPawn>)
            // NetworkVarNames: m_iConnected (PlayerConnectedState)
            // NetworkVarNames: m_iszPlayerName (char)
            // NetworkVarNames: m_steamID (uint64)
            // NetworkVarNames: m_iDesiredFOV (uint32)
            pub mod CBasePlayerController {
                pub const m_nInButtonsWhichAreToggles: usize = 0x7B0; // uint64
                pub const m_nTickBase: usize = 0x7B8; // uint32
                pub const m_hPawn: usize = 0x7E8; // CHandle<CBasePlayerPawn>
                pub const m_nSplitScreenSlot: usize = 0x7EC; // CSplitScreenSlot
                pub const m_hSplitOwner: usize = 0x7F0; // CHandle<CBasePlayerController>
                pub const m_hSplitScreenPlayers: usize = 0x7F8; // CUtlVector<CHandle<CBasePlayerController>>
                pub const m_bIsHLTV: usize = 0x810; // bool
                pub const m_iConnected: usize = 0x814; // PlayerConnectedState
                pub const m_iszPlayerName: usize = 0x818; // char[128]
                pub const m_szNetworkIDString: usize = 0x898; // CUtlString
                pub const m_fLerpTime: usize = 0x8A0; // float32
                pub const m_bLagCompensation: usize = 0x8A4; // bool
                pub const m_bPredict: usize = 0x8A5; // bool
                pub const m_bAutoKickDisabled: usize = 0x8A6; // bool
                pub const m_bIsLowViolence: usize = 0x8A7; // bool
                pub const m_bGamePaused: usize = 0x8A8; // bool
                pub const m_nLastRealCommandNumberExecuted: usize = 0x934; // int32
                pub const m_nLastLateCommandExecuted: usize = 0x938; // int32
                pub const m_iIgnoreGlobalChat: usize = 0x950; // ChatIgnoreType_t
                pub const m_flLastPlayerTalkTime: usize = 0x954; // float32
                pub const m_flLastEntitySteadyState: usize = 0x958; // float32
                pub const m_nAvailableEntitySteadyState: usize = 0x95C; // int32
                pub const m_bHasAnySteadyStateEnts: usize = 0x960; // bool
                pub const m_steamID: usize = 0x970; // uint64
                pub const m_iDesiredFOV: usize = 0x978; // uint32
            }
            // Parent: None
            // Fields count: 14
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod CBasePlayerVData {
                pub const m_sModelName: usize = 0x28; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeCModel>>
                pub const m_flHeadDamageMultiplier: usize = 0x108; // CSkillFloat
                pub const m_flChestDamageMultiplier: usize = 0x118; // CSkillFloat
                pub const m_flStomachDamageMultiplier: usize = 0x128; // CSkillFloat
                pub const m_flArmDamageMultiplier: usize = 0x138; // CSkillFloat
                pub const m_flLegDamageMultiplier: usize = 0x148; // CSkillFloat
                pub const m_flHoldBreathTime: usize = 0x158; // float32
                pub const m_flDrowningDamageInterval: usize = 0x15C; // float32
                pub const m_nDrowningDamageInitial: usize = 0x160; // int32
                pub const m_nDrowningDamageMax: usize = 0x164; // int32
                pub const m_nWaterSpeed: usize = 0x168; // int32
                pub const m_flUseRange: usize = 0x16C; // float32
                pub const m_flUseAngleTolerance: usize = 0x170; // float32
                pub const m_flCrouchTime: usize = 0x174; // float32
            }
            // Parent: None
            // Fields count: 20
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod CBasePlayerWeaponVData {
                pub const m_szWorldModel: usize = 0x28; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeCModel>>
                pub const m_bBuiltRightHanded: usize = 0x108; // bool
                pub const m_bAllowFlipping: usize = 0x109; // bool
                pub const m_sMuzzleAttachment: usize = 0x110; // CUtlString
                pub const m_szMuzzleFlashParticle: usize = 0x118; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
                pub const m_iFlags: usize = 0x1F8; // ItemFlagTypes_t
                pub const m_nPrimaryAmmoType: usize = 0x1F9; // AmmoIndex_t
                pub const m_nSecondaryAmmoType: usize = 0x1FA; // AmmoIndex_t
                pub const m_iMaxClip1: usize = 0x1FC; // int32
                pub const m_iMaxClip2: usize = 0x200; // int32
                pub const m_iDefaultClip1: usize = 0x204; // int32
                pub const m_iDefaultClip2: usize = 0x208; // int32
                pub const m_iWeight: usize = 0x20C; // int32
                pub const m_bAutoSwitchTo: usize = 0x210; // bool
                pub const m_bAutoSwitchFrom: usize = 0x211; // bool
                pub const m_iRumbleEffect: usize = 0x214; // RumbleEffect_t
                pub const m_bLinkedCooldowns: usize = 0x218; // bool
                pub const m_aShootSounds: usize = 0x220; // CUtlMap<WeaponSound_t,CSoundEventName>
                pub const m_iSlot: usize = 0x240; // int32
                pub const m_iPosition: usize = 0x244; // int32
            }
            // Parent: None
            // Fields count: 13
            //
            // Metadata:
            // NetworkVarNames: m_animGraphNetworkedVars (CAnimGraphNetworkedVariables)
            // NetworkVarNames: m_hSequence (HSequence)
            // NetworkVarNames: m_flSeqStartTime (GameTime_t)
            // NetworkVarNames: m_flSeqFixedCycle (float)
            // NetworkVarNames: m_nAnimLoopMode (AnimLoopMode_t)
            pub mod CBaseAnimGraphController {
                pub const m_animGraphNetworkedVars: usize = 0x18; // CAnimGraphNetworkedVariables
                pub const m_bSequenceFinished: usize = 0x220; // bool
                pub const m_flSoundSyncTime: usize = 0x224; // float32
                pub const m_hSequence: usize = 0x228; // HSequence
                pub const m_flSeqStartTime: usize = 0x22C; // GameTime_t
                pub const m_flSeqFixedCycle: usize = 0x230; // float32
                pub const m_nAnimLoopMode: usize = 0x234; // AnimLoopMode_t
                pub const m_flPlaybackRate: usize = 0x238; // CNetworkedQuantizedFloat
                pub const m_nNotifyState: usize = 0x244; // SequenceFinishNotifyState_t
                pub const m_bNetworkedAnimationInputsChanged: usize = 0x246; // bool
                pub const m_bNetworkedSequenceChanged: usize = 0x247; // bool
                pub const m_bLastUpdateSkipped: usize = 0x248; // bool
                pub const m_flPrevAnimUpdateTime: usize = 0x24C; // GameTime_t
            }
            // Parent: CBaseEntity
            // Fields count: 25
            //
            // Metadata:
            // NetworkVarNames: m_CRenderComponent (CRenderComponent::Storage_t)
            // NetworkVarNames: m_CHitboxComponent (CHitboxComponent::Storage_t)
            // NetworkVarNames: m_nRenderMode (RenderMode_t)
            // NetworkVarNames: m_nRenderFX (RenderFx_t)
            // NetworkVarNames: m_clrRender (Color)
            // NetworkVarNames: m_vecRenderAttributes (EntityRenderAttribute_t)
            // NetworkVarNames: m_bRenderToCubemaps (bool)
            // NetworkVarNames: m_Collision (CCollisionProperty)
            // NetworkVarNames: m_Glow (CGlowProperty)
            // NetworkVarNames: m_flGlowBackfaceMult (float)
            // NetworkVarNames: m_fadeMinDist (float32)
            // NetworkVarNames: m_fadeMaxDist (float32)
            // NetworkVarNames: m_flFadeScale (float32)
            // NetworkVarNames: m_flShadowStrength (float32)
            // NetworkVarNames: m_nObjectCulling (uint8)
            // NetworkVarNames: m_nAddDecal (int)
            // NetworkVarNames: m_vDecalPosition (Vector)
            // NetworkVarNames: m_vDecalForwardAxis (Vector)
            // NetworkVarNames: m_flDecalHealBloodRate (float)
            // NetworkVarNames: m_flDecalHealHeightRate (float)
            // NetworkVarNames: m_ConfigEntitiesToPropagateMaterialDecalsTo (CHandle<CBaseModelEntity>)
            // NetworkVarNames: m_vecViewOffset (CNetworkViewOffsetVector)
            pub mod CBaseModelEntity {
                pub const m_CRenderComponent: usize = 0x7A8; // CRenderComponent*
                pub const m_CHitboxComponent: usize = 0x7B0; // CHitboxComponent
                pub const m_flDissolveStartTime: usize = 0x7D8; // GameTime_t
                pub const m_OnIgnite: usize = 0x7E0; // CEntityIOOutput
                pub const m_nRenderMode: usize = 0x808; // RenderMode_t
                pub const m_nRenderFX: usize = 0x809; // RenderFx_t
                pub const m_bAllowFadeInView: usize = 0x80A; // bool
                pub const m_clrRender: usize = 0x80B; // Color
                pub const m_vecRenderAttributes: usize = 0x810; // CUtlVectorEmbeddedNetworkVar<EntityRenderAttribute_t>
                pub const m_bRenderToCubemaps: usize = 0x860; // bool
                pub const m_Collision: usize = 0x868; // CCollisionProperty
                pub const m_Glow: usize = 0x918; // CGlowProperty
                pub const m_flGlowBackfaceMult: usize = 0x970; // float32
                pub const m_fadeMinDist: usize = 0x974; // float32
                pub const m_fadeMaxDist: usize = 0x978; // float32
                pub const m_flFadeScale: usize = 0x97C; // float32
                pub const m_flShadowStrength: usize = 0x980; // float32
                pub const m_nObjectCulling: usize = 0x984; // uint8
                pub const m_nAddDecal: usize = 0x988; // int32
                pub const m_vDecalPosition: usize = 0x98C; // Vector
                pub const m_vDecalForwardAxis: usize = 0x998; // Vector
                pub const m_flDecalHealBloodRate: usize = 0x9A4; // float32
                pub const m_flDecalHealHeightRate: usize = 0x9A8; // float32
                pub const m_ConfigEntitiesToPropagateMaterialDecalsTo: usize = 0x9B0; // CNetworkUtlVectorBase<CHandle<CBaseModelEntity>>
                pub const m_vecViewOffset: usize = 0x9C8; // CNetworkViewOffsetVector
            }
            // Parent: CBaseModelEntity
            // Fields count: 22
            //
            // Metadata:
            // MNetworkIncludeByUserGroup
            // MNetworkIncludeByName
            // MNetworkIncludeByName
            // MNetworkIncludeByName
            // MNetworkIncludeByName
            // MNetworkIncludeByName
            // NetworkVarNames: m_szSnapshotFileName (char)
            // NetworkVarNames: m_bActive (bool)
            // NetworkVarNames: m_bFrozen (bool)
            // NetworkVarNames: m_flFreezeTransitionDuration (float)
            // NetworkVarNames: m_nStopType (int)
            // NetworkVarNames: m_bAnimateDuringGameplayPause (bool)
            // NetworkVarNames: m_iEffectIndex (HParticleSystemDefinitionStrong)
            // NetworkVarNames: m_flStartTime (GameTime_t)
            // NetworkVarNames: m_flPreSimTime (float32)
            // NetworkVarNames: m_vServerControlPoints (Vector)
            // NetworkVarNames: m_iServerControlPointAssignments (uint8)
            // NetworkVarNames: m_hControlPointEnts (CHandle<CBaseEntity>)
            // NetworkVarNames: m_bNoSave (bool)
            // NetworkVarNames: m_bNoFreeze (bool)
            // NetworkVarNames: m_bNoRamp (bool)
            pub mod CParticleSystem {
                pub const m_szSnapshotFileName: usize = 0x9F8; // char[512]
                pub const m_bActive: usize = 0xBF8; // bool
                pub const m_bFrozen: usize = 0xBF9; // bool
                pub const m_flFreezeTransitionDuration: usize = 0xBFC; // float32
                pub const m_nStopType: usize = 0xC00; // int32
                pub const m_bAnimateDuringGameplayPause: usize = 0xC04; // bool
                pub const m_iEffectIndex: usize = 0xC08; // CStrongHandle<InfoForResourceTypeIParticleSystemDefinition>
                pub const m_flStartTime: usize = 0xC10; // GameTime_t
                pub const m_flPreSimTime: usize = 0xC14; // float32
                pub const m_vServerControlPoints: usize = 0xC18; // Vector[4]
                pub const m_iServerControlPointAssignments: usize = 0xC48; // uint8[4]
                pub const m_hControlPointEnts: usize = 0xC4C; // CHandle<CBaseEntity>[64]
                pub const m_bNoSave: usize = 0xD4C; // bool
                pub const m_bNoFreeze: usize = 0xD4D; // bool
                pub const m_bNoRamp: usize = 0xD4E; // bool
                pub const m_bStartActive: usize = 0xD4F; // bool
                pub const m_iszEffectName: usize = 0xD50; // CUtlSymbolLarge
                pub const m_iszControlPointNames: usize = 0xD58; // CUtlSymbolLarge[64]
                pub const m_nDataCP: usize = 0xF58; // int32
                pub const m_vecDataCPValue: usize = 0xF5C; // Vector
                pub const m_nTintCP: usize = 0xF68; // int32
                pub const m_clrTint: usize = 0xF6C; // Color
            }
            // Parent: CBaseEntity
            // Fields count: 16
            //
            // Metadata:
            // NetworkVarNames: m_flParticleSpacing (float)
            // NetworkVarNames: m_flSlack (float)
            // NetworkVarNames: m_flRadius (float)
            // NetworkVarNames: m_ColorTint (Color)
            // NetworkVarNames: m_nEffectState (int)
            // NetworkVarNames: m_iEffectIndex (HParticleSystemDefinitionStrong)
            // NetworkVarNames: m_PathNodes_Position (Vector)
            // NetworkVarNames: m_PathNodes_TangentIn (Vector)
            // NetworkVarNames: m_PathNodes_TangentOut (Vector)
            // NetworkVarNames: m_PathNodes_Color (Vector)
            // NetworkVarNames: m_PathNodes_PinEnabled (bool)
            // NetworkVarNames: m_PathNodes_RadiusScale (float)
            pub mod CPathParticleRope {
                pub const m_bStartActive: usize = 0x7A8; // bool
                pub const m_flMaxSimulationTime: usize = 0x7AC; // float32
                pub const m_iszEffectName: usize = 0x7B0; // CUtlSymbolLarge
                pub const m_PathNodes_Name: usize = 0x7B8; // CUtlVector<CUtlSymbolLarge>
                pub const m_flParticleSpacing: usize = 0x7D0; // float32
                pub const m_flSlack: usize = 0x7D4; // float32
                pub const m_flRadius: usize = 0x7D8; // float32
                pub const m_ColorTint: usize = 0x7DC; // Color
                pub const m_nEffectState: usize = 0x7E0; // int32
                pub const m_iEffectIndex: usize = 0x7E8; // CStrongHandle<InfoForResourceTypeIParticleSystemDefinition>
                pub const m_PathNodes_Position: usize = 0x7F0; // CNetworkUtlVectorBase<Vector>
                pub const m_PathNodes_TangentIn: usize = 0x808; // CNetworkUtlVectorBase<Vector>
                pub const m_PathNodes_TangentOut: usize = 0x820; // CNetworkUtlVectorBase<Vector>
                pub const m_PathNodes_Color: usize = 0x838; // CNetworkUtlVectorBase<Vector>
                pub const m_PathNodes_PinEnabled: usize = 0x850; // CNetworkUtlVectorBase<bool>
                pub const m_PathNodes_RadiusScale: usize = 0x868; // CNetworkUtlVectorBase<float32>
            }
            // Parent: CBaseModelEntity
            // Fields count: 9
            //
            // Metadata:
            // NetworkVarNames: m_Flags (uint8)
            // NetworkVarNames: m_LightStyle (uint8)
            // NetworkVarNames: m_Radius (float32)
            // NetworkVarNames: m_Exponent (int32)
            // NetworkVarNames: m_InnerAngle (float32)
            // NetworkVarNames: m_OuterAngle (float32)
            // NetworkVarNames: m_SpotRadius (float32)
            pub mod CDynamicLight {
                pub const m_ActualFlags: usize = 0x9F8; // uint8
                pub const m_Flags: usize = 0x9F9; // uint8
                pub const m_LightStyle: usize = 0x9FA; // uint8
                pub const m_On: usize = 0x9FB; // bool
                pub const m_Radius: usize = 0x9FC; // float32
                pub const m_Exponent: usize = 0xA00; // int32
                pub const m_InnerAngle: usize = 0xA04; // float32
                pub const m_OuterAngle: usize = 0xA08; // float32
                pub const m_SpotRadius: usize = 0xA0C; // float32
            }
            // Parent: CBaseEntity
            // Fields count: 1
            //
            // Metadata:
            // NetworkVarNames: m_EnvWindShared (CEnvWindShared)
            pub mod CEnvWind {
                pub const m_EnvWindShared: usize = 0x7A8; // CEnvWindShared
            }
            // Parent: None
            // Fields count: 5
            //
            // Metadata:
            // NetworkVarNames: m_iszOverlayNames (string_t)
            // NetworkVarNames: m_flOverlayTimes (float32)
            // NetworkVarNames: m_flStartTime (GameTime_t)
            // NetworkVarNames: m_iDesiredOverlay (int32)
            // NetworkVarNames: m_bIsActive (bool)
            pub mod CEnvScreenOverlay {
                pub const m_iszOverlayNames: usize = 0x7A8; // CUtlSymbolLarge[10]
                pub const m_flOverlayTimes: usize = 0x7F8; // float32[10]
                pub const m_flStartTime: usize = 0x820; // GameTime_t
                pub const m_iDesiredOverlay: usize = 0x824; // int32
                pub const m_bIsActive: usize = 0x828; // bool
            }
            // Parent: None
            // Fields count: 4
            //
            // Metadata:
            // NetworkVarNames: m_fadeColor (Color)
            pub mod CEnvFade {
                pub const m_fadeColor: usize = 0x7A8; // Color
                pub const m_Duration: usize = 0x7AC; // float32
                pub const m_HoldDuration: usize = 0x7B0; // float32
                pub const m_OnBeginFade: usize = 0x7B8; // CEntityIOOutput
            }
            // Parent: CBaseModelEntity
            // Fields count: 7
            //
            // Metadata:
            // MNetworkIncludeByUserGroup
            // MNetworkIncludeByName
            // MNetworkIncludeByName
            // MNetworkIncludeByName
            // MNetworkIncludeByName
            // NetworkVarNames: m_nHorizontalSize (uint32)
            // NetworkVarNames: m_nVerticalSize (uint32)
            // NetworkVarNames: m_nMinDist (uint32)
            // NetworkVarNames: m_nMaxDist (uint32)
            // NetworkVarNames: m_nOuterMaxDist (uint32)
            // NetworkVarNames: m_flGlowProxySize (float32)
            // NetworkVarNames: m_flHDRColorScale (float32)
            pub mod CLightGlow {
                pub const m_nHorizontalSize: usize = 0x9F8; // uint32
                pub const m_nVerticalSize: usize = 0x9FC; // uint32
                pub const m_nMinDist: usize = 0xA00; // uint32
                pub const m_nMaxDist: usize = 0xA04; // uint32
                pub const m_nOuterMaxDist: usize = 0xA08; // uint32
                pub const m_flGlowProxySize: usize = 0xA0C; // float32
                pub const m_flHDRColorScale: usize = 0xA10; // float32
            }
            // Parent: None
            // Fields count: 5
            //
            // Metadata:
            // NetworkVarNames: m_bEnabled (bool)
            pub mod CLogicGameEventListener {
                pub const m_OnEventFired: usize = 0x7B8; // CEntityIOOutput
                pub const m_iszGameEventName: usize = 0x7E0; // CUtlSymbolLarge
                pub const m_iszGameEventItem: usize = 0x7E8; // CUtlSymbolLarge
                pub const m_bEnabled: usize = 0x7F0; // bool
                pub const m_bStartDisabled: usize = 0x7F1; // bool
            }
            // Parent: CBaseEntity
            // Fields count: 44
            //
            // Metadata:
            // NetworkVarNames: m_bDisabled (bool)
            // NetworkVarNames: m_bUpdateOnClient (bool)
            // NetworkVarNames: m_nInputType (ValueRemapperInputType_t)
            // NetworkVarNames: m_hRemapLineStart (CHandle<CBaseEntity>)
            // NetworkVarNames: m_hRemapLineEnd (CHandle<CBaseEntity>)
            // NetworkVarNames: m_flMaximumChangePerSecond (float)
            // NetworkVarNames: m_flDisengageDistance (float)
            // NetworkVarNames: m_flEngageDistance (float)
            // NetworkVarNames: m_bRequiresUseKey (bool)
            // NetworkVarNames: m_nOutputType (ValueRemapperOutputType_t)
            // NetworkVarNames: m_hOutputEntities (CHandle<CBaseEntity>)
            // NetworkVarNames: m_nHapticsType (ValueRemapperHapticsType_t)
            // NetworkVarNames: m_nMomentumType (ValueRemapperMomentumType_t)
            // NetworkVarNames: m_flMomentumModifier (float)
            // NetworkVarNames: m_flSnapValue (float)
            // NetworkVarNames: m_nRatchetType (ValueRemapperRatchetType_t)
            // NetworkVarNames: m_flInputOffset (float)
            pub mod CPointValueRemapper {
                pub const m_bDisabled: usize = 0x7A8; // bool
                pub const m_bUpdateOnClient: usize = 0x7A9; // bool
                pub const m_nInputType: usize = 0x7AC; // ValueRemapperInputType_t
                pub const m_iszRemapLineStartName: usize = 0x7B0; // CUtlSymbolLarge
                pub const m_iszRemapLineEndName: usize = 0x7B8; // CUtlSymbolLarge
                pub const m_hRemapLineStart: usize = 0x7C0; // CHandle<CBaseEntity>
                pub const m_hRemapLineEnd: usize = 0x7C4; // CHandle<CBaseEntity>
                pub const m_flMaximumChangePerSecond: usize = 0x7C8; // float32
                pub const m_flDisengageDistance: usize = 0x7CC; // float32
                pub const m_flEngageDistance: usize = 0x7D0; // float32
                pub const m_bRequiresUseKey: usize = 0x7D4; // bool
                pub const m_nOutputType: usize = 0x7D8; // ValueRemapperOutputType_t
                pub const m_iszOutputEntityName: usize = 0x7E0; // CUtlSymbolLarge
                pub const m_iszOutputEntity2Name: usize = 0x7E8; // CUtlSymbolLarge
                pub const m_iszOutputEntity3Name: usize = 0x7F0; // CUtlSymbolLarge
                pub const m_iszOutputEntity4Name: usize = 0x7F8; // CUtlSymbolLarge
                pub const m_hOutputEntities: usize = 0x800; // CNetworkUtlVectorBase<CHandle<CBaseEntity>>
                pub const m_nHapticsType: usize = 0x818; // ValueRemapperHapticsType_t
                pub const m_nMomentumType: usize = 0x81C; // ValueRemapperMomentumType_t
                pub const m_flMomentumModifier: usize = 0x820; // float32
                pub const m_flSnapValue: usize = 0x824; // float32
                pub const m_flCurrentMomentum: usize = 0x828; // float32
                pub const m_nRatchetType: usize = 0x82C; // ValueRemapperRatchetType_t
                pub const m_flRatchetOffset: usize = 0x830; // float32
                pub const m_flInputOffset: usize = 0x834; // float32
                pub const m_bEngaged: usize = 0x838; // bool
                pub const m_bFirstUpdate: usize = 0x839; // bool
                pub const m_flPreviousValue: usize = 0x83C; // float32
                pub const m_flPreviousUpdateTickTime: usize = 0x840; // GameTime_t
                pub const m_vecPreviousTestPoint: usize = 0x844; // Vector
                pub const m_hUsingPlayer: usize = 0x850; // CHandle<CBasePlayerPawn>
                pub const m_flCustomOutputValue: usize = 0x854; // float32
                pub const m_iszSoundEngage: usize = 0x858; // CUtlSymbolLarge
                pub const m_iszSoundDisengage: usize = 0x860; // CUtlSymbolLarge
                pub const m_iszSoundReachedValueZero: usize = 0x868; // CUtlSymbolLarge
                pub const m_iszSoundReachedValueOne: usize = 0x870; // CUtlSymbolLarge
                pub const m_iszSoundMovingLoop: usize = 0x878; // CUtlSymbolLarge
                pub const m_Position: usize = 0x888; // CEntityOutputTemplate<float32>
                pub const m_PositionDelta: usize = 0x8B0; // CEntityOutputTemplate<float32>
                pub const m_OnReachedValueZero: usize = 0x8D8; // CEntityIOOutput
                pub const m_OnReachedValueOne: usize = 0x900; // CEntityIOOutput
                pub const m_OnReachedValueCustom: usize = 0x928; // CEntityIOOutput
                pub const m_OnEngage: usize = 0x950; // CEntityIOOutput
                pub const m_OnDisengage: usize = 0x978; // CEntityIOOutput
            }
            // Parent: None
            // Fields count: 11
            //
            // Metadata:
            // NetworkVarNames: m_messageText (char)
            // NetworkVarNames: m_FontName (char)
            // NetworkVarNames: m_bEnabled (bool)
            // NetworkVarNames: m_bFullbright (bool)
            // NetworkVarNames: m_flWorldUnitsPerPx (float)
            // NetworkVarNames: m_flFontSize (float)
            // NetworkVarNames: m_flDepthOffset (float)
            // NetworkVarNames: m_Color (Color)
            // NetworkVarNames: m_nJustifyHorizontal (PointWorldTextJustifyHorizontal_t)
            // NetworkVarNames: m_nJustifyVertical (PointWorldTextJustifyVertical_t)
            // NetworkVarNames: m_nReorientMode (PointWorldTextReorientMode_t)
            pub mod CPointWorldText {
                pub const m_messageText: usize = 0x9F8; // char[512]
                pub const m_FontName: usize = 0xBF8; // char[64]
                pub const m_bEnabled: usize = 0xC38; // bool
                pub const m_bFullbright: usize = 0xC39; // bool
                pub const m_flWorldUnitsPerPx: usize = 0xC3C; // float32
                pub const m_flFontSize: usize = 0xC40; // float32
                pub const m_flDepthOffset: usize = 0xC44; // float32
                pub const m_Color: usize = 0xC48; // Color
                pub const m_nJustifyHorizontal: usize = 0xC4C; // PointWorldTextJustifyHorizontal_t
                pub const m_nJustifyVertical: usize = 0xC50; // PointWorldTextJustifyVertical_t
                pub const m_nReorientMode: usize = 0xC54; // PointWorldTextReorientMode_t
            }
            // Parent: CBaseEntity
            // Fields count: 3
            //
            // Metadata:
            // NetworkVarNames: m_iCurrentMaxRagdollCount (int8)
            pub mod CRagdollManager {
                pub const m_iCurrentMaxRagdollCount: usize = 0x7A8; // int8
                pub const m_iMaxRagdollCount: usize = 0x7AC; // int32
                pub const m_bSaveImportant: usize = 0x7B0; // bool
            }
            // Parent: None
            // Fields count: 64
            //
            // Metadata:
            // NetworkVarNames: m_bIsPlayingBack (bool)
            // NetworkVarNames: m_bPaused (bool)
            // NetworkVarNames: m_bMultiplayer (bool)
            // NetworkVarNames: m_bAutogenerated (bool)
            // NetworkVarNames: m_flForceClientTime (float32)
            // NetworkVarNames: m_hActorList (CHandle<CBaseFlex>)
            // NetworkVarNames: m_nSceneStringIndex (uint16)
            pub mod CSceneEntity {
                pub const m_iszSceneFile: usize = 0x7B0; // CUtlSymbolLarge
                pub const m_iszResumeSceneFile: usize = 0x7B8; // CUtlSymbolLarge
                pub const m_iszTarget1: usize = 0x7C0; // CUtlSymbolLarge
                pub const m_iszTarget2: usize = 0x7C8; // CUtlSymbolLarge
                pub const m_iszTarget3: usize = 0x7D0; // CUtlSymbolLarge
                pub const m_iszTarget4: usize = 0x7D8; // CUtlSymbolLarge
                pub const m_iszTarget5: usize = 0x7E0; // CUtlSymbolLarge
                pub const m_iszTarget6: usize = 0x7E8; // CUtlSymbolLarge
                pub const m_iszTarget7: usize = 0x7F0; // CUtlSymbolLarge
                pub const m_iszTarget8: usize = 0x7F8; // CUtlSymbolLarge
                pub const m_hTarget1: usize = 0x800; // CHandle<CBaseEntity>
                pub const m_hTarget2: usize = 0x804; // CHandle<CBaseEntity>
                pub const m_hTarget3: usize = 0x808; // CHandle<CBaseEntity>
                pub const m_hTarget4: usize = 0x80C; // CHandle<CBaseEntity>
                pub const m_hTarget5: usize = 0x810; // CHandle<CBaseEntity>
                pub const m_hTarget6: usize = 0x814; // CHandle<CBaseEntity>
                pub const m_hTarget7: usize = 0x818; // CHandle<CBaseEntity>
                pub const m_hTarget8: usize = 0x81C; // CHandle<CBaseEntity>
                pub const m_bIsPlayingBack: usize = 0x820; // bool
                pub const m_bPaused: usize = 0x821; // bool
                pub const m_bMultiplayer: usize = 0x822; // bool
                pub const m_bAutogenerated: usize = 0x823; // bool
                pub const m_flForceClientTime: usize = 0x824; // float32
                pub const m_flCurrentTime: usize = 0x828; // float32
                pub const m_flFrameTime: usize = 0x82C; // float32
                pub const m_bCancelAtNextInterrupt: usize = 0x830; // bool
                pub const m_fPitch: usize = 0x834; // float32
                pub const m_bAutomated: usize = 0x838; // bool
                pub const m_nAutomatedAction: usize = 0x83C; // int32
                pub const m_flAutomationDelay: usize = 0x840; // float32
                pub const m_flAutomationTime: usize = 0x844; // float32
                pub const m_hWaitingForThisResumeScene: usize = 0x848; // CHandle<CBaseEntity>
                pub const m_bWaitingForResumeScene: usize = 0x84C; // bool
                pub const m_bPausedViaInput: usize = 0x84D; // bool
                pub const m_bPauseAtNextInterrupt: usize = 0x84E; // bool
                pub const m_bWaitingForActor: usize = 0x84F; // bool
                pub const m_bWaitingForInterrupt: usize = 0x850; // bool
                pub const m_bInterruptedActorsScenes: usize = 0x851; // bool
                pub const m_bBreakOnNonIdle: usize = 0x852; // bool
                pub const m_hActorList: usize = 0x858; // CNetworkUtlVectorBase<CHandle<CBaseFlex>>
                pub const m_hRemoveActorList: usize = 0x870; // CUtlVector<CHandle<CBaseEntity>>
                pub const m_nSceneFlushCounter: usize = 0x8B8; // int32
                pub const m_nSceneStringIndex: usize = 0x8BC; // uint16
                pub const m_OnStart: usize = 0x8C0; // CEntityIOOutput
                pub const m_OnCompletion: usize = 0x8E8; // CEntityIOOutput
                pub const m_OnCanceled: usize = 0x910; // CEntityIOOutput
                pub const m_OnPaused: usize = 0x938; // CEntityIOOutput
                pub const m_OnResumed: usize = 0x960; // CEntityIOOutput
                pub const m_OnTrigger: usize = 0x988; // CEntityIOOutput[16]
                pub const m_hInterruptScene: usize = 0xC98; // CHandle<CSceneEntity>
                pub const m_nInterruptCount: usize = 0xC9C; // int32
                pub const m_bSceneMissing: usize = 0xCA0; // bool
                pub const m_bInterrupted: usize = 0xCA1; // bool
                pub const m_bCompletedEarly: usize = 0xCA2; // bool
                pub const m_bInterruptSceneFinished: usize = 0xCA3; // bool
                pub const m_bRestoring: usize = 0xCA4; // bool
                pub const m_hNotifySceneCompletion: usize = 0xCA8; // CUtlVector<CHandle<CSceneEntity>>
                pub const m_hListManagers: usize = 0xCC0; // CUtlVector<CHandle<CSceneListManager>>
                pub const m_iszSoundName: usize = 0xCD8; // CUtlSymbolLarge
                pub const m_iszSequenceName: usize = 0xCE0; // CUtlSymbolLarge
                pub const m_hActor: usize = 0xCE8; // CHandle<CBaseFlex>
                pub const m_hActivator: usize = 0xCEC; // CHandle<CBaseEntity>
                pub const m_BusyActor: usize = 0xCF0; // int32
                pub const m_iPlayerDeathBehavior: usize = 0xCF4; // SceneOnPlayerDeath_t
            }
            // Parent: CBaseModelEntity
            // Fields count: 14
            //
            // Metadata:
            // MNetworkIncludeByName
            // NetworkVarNames: m_vDirection (Vector)
            // NetworkVarNames: m_clrOverlay (Color)
            // NetworkVarNames: m_iszEffectName (string_t)
            // NetworkVarNames: m_iszSSEffectName (string_t)
            // NetworkVarNames: m_bOn (bool)
            // NetworkVarNames: m_bmaxColor (bool)
            // NetworkVarNames: m_flSize (float32)
            // NetworkVarNames: m_flRotation (float32)
            // NetworkVarNames: m_flHazeScale (float32)
            // NetworkVarNames: m_flAlphaHaze (float32)
            // NetworkVarNames: m_flAlphaHdr (float32)
            // NetworkVarNames: m_flAlphaScale (float32)
            // NetworkVarNames: m_flHDRColorScale (float32)
            // NetworkVarNames: m_flFarZScale (float32)
            pub mod CSun {
                pub const m_vDirection: usize = 0x9F8; // Vector
                pub const m_clrOverlay: usize = 0xA04; // Color
                pub const m_iszEffectName: usize = 0xA08; // CUtlSymbolLarge
                pub const m_iszSSEffectName: usize = 0xA10; // CUtlSymbolLarge
                pub const m_bOn: usize = 0xA18; // bool
                pub const m_bmaxColor: usize = 0xA19; // bool
                pub const m_flSize: usize = 0xA1C; // float32
                pub const m_flRotation: usize = 0xA20; // float32
                pub const m_flHazeScale: usize = 0xA24; // float32
                pub const m_flAlphaHaze: usize = 0xA28; // float32
                pub const m_flAlphaHdr: usize = 0xA2C; // float32
                pub const m_flAlphaScale: usize = 0xA30; // float32
                pub const m_flHDRColorScale: usize = 0xA34; // float32
                pub const m_flFarZScale: usize = 0xA38; // float32
            }
            // Parent: CBaseEntity
            // Fields count: 2
            //
            // Metadata:
            // NetworkVarNames: m_Handle (CHandle<CBaseEntity>)
            // NetworkVarNames: m_bSendHandle (bool)
            pub mod CHandleTest {
                pub const m_Handle: usize = 0x7A8; // CHandle<CBaseEntity>
                pub const m_bSendHandle: usize = 0x7AC; // bool
            }
            // Parent: None
            // Fields count: 24
            //
            // Metadata:
            // NetworkVarNames: m_glowEntity (CHandle<CBaseModelEntity>)
            // NetworkVarNames: m_usable (bool)
            // NetworkVarNames: m_szDisplayText (string_t)
            pub mod CBaseButton {
                pub const m_angMoveEntitySpace: usize = 0xA78; // QAngle
                pub const m_fStayPushed: usize = 0xA84; // bool
                pub const m_fRotating: usize = 0xA85; // bool
                pub const m_ls: usize = 0xA88; // locksound_t
                pub const m_sUseSound: usize = 0xAA8; // CUtlSymbolLarge
                pub const m_sLockedSound: usize = 0xAB0; // CUtlSymbolLarge
                pub const m_sUnlockedSound: usize = 0xAB8; // CUtlSymbolLarge
                pub const m_bLocked: usize = 0xAC0; // bool
                pub const m_bDisabled: usize = 0xAC1; // bool
                pub const m_flUseLockedTime: usize = 0xAC4; // GameTime_t
                pub const m_bSolidBsp: usize = 0xAC8; // bool
                pub const m_OnDamaged: usize = 0xAD0; // CEntityIOOutput
                pub const m_OnPressed: usize = 0xAF8; // CEntityIOOutput
                pub const m_OnUseLocked: usize = 0xB20; // CEntityIOOutput
                pub const m_OnIn: usize = 0xB48; // CEntityIOOutput
                pub const m_OnOut: usize = 0xB70; // CEntityIOOutput
                pub const m_nState: usize = 0xB98; // int32
                pub const m_hConstraint: usize = 0xB9C; // CEntityHandle
                pub const m_hConstraintParent: usize = 0xBA0; // CEntityHandle
                pub const m_bForceNpcExclude: usize = 0xBA4; // bool
                pub const m_sGlowEntity: usize = 0xBA8; // CUtlSymbolLarge
                pub const m_glowEntity: usize = 0xBB0; // CHandle<CBaseModelEntity>
                pub const m_usable: usize = 0xBB4; // bool
                pub const m_szDisplayText: usize = 0xBB8; // CUtlSymbolLarge
            }
            // Parent: None
            // Fields count: 27
            //
            // Metadata:
            // NetworkVarNames: m_bIsUsable (bool)
            pub mod CBaseDoor {
                pub const m_angMoveEntitySpace: usize = 0xA84; // QAngle
                pub const m_vecMoveDirParentSpace: usize = 0xA90; // Vector
                pub const m_ls: usize = 0xAA0; // locksound_t
                pub const m_bForceClosed: usize = 0xAC0; // bool
                pub const m_bDoorGroup: usize = 0xAC1; // bool
                pub const m_bLocked: usize = 0xAC2; // bool
                pub const m_bIgnoreDebris: usize = 0xAC3; // bool
                pub const m_eSpawnPosition: usize = 0xAC4; // FuncDoorSpawnPos_t
                pub const m_flBlockDamage: usize = 0xAC8; // float32
                pub const m_NoiseMoving: usize = 0xAD0; // CUtlSymbolLarge
                pub const m_NoiseArrived: usize = 0xAD8; // CUtlSymbolLarge
                pub const m_NoiseMovingClosed: usize = 0xAE0; // CUtlSymbolLarge
                pub const m_NoiseArrivedClosed: usize = 0xAE8; // CUtlSymbolLarge
                pub const m_ChainTarget: usize = 0xAF0; // CUtlSymbolLarge
                pub const m_OnBlockedClosing: usize = 0xAF8; // CEntityIOOutput
                pub const m_OnBlockedOpening: usize = 0xB20; // CEntityIOOutput
                pub const m_OnUnblockedClosing: usize = 0xB48; // CEntityIOOutput
                pub const m_OnUnblockedOpening: usize = 0xB70; // CEntityIOOutput
                pub const m_OnFullyClosed: usize = 0xB98; // CEntityIOOutput
                pub const m_OnFullyOpen: usize = 0xBC0; // CEntityIOOutput
                pub const m_OnClose: usize = 0xBE8; // CEntityIOOutput
                pub const m_OnOpen: usize = 0xC10; // CEntityIOOutput
                pub const m_OnLockedUse: usize = 0xC38; // CEntityIOOutput
                pub const m_bLoopMoveSound: usize = 0xC60; // bool
                pub const m_bCreateNavObstacle: usize = 0xC78; // bool
                pub const m_isChaining: usize = 0xC79; // bool
                pub const m_bIsUsable: usize = 0xC7A; // bool
            }
            // Parent: CBaseModelEntity
            // Fields count: 10
            //
            // Metadata:
            // NetworkVarNames: m_flFadeInStart (float32)
            // NetworkVarNames: m_flFadeInLength (float32)
            // NetworkVarNames: m_flFadeOutModelStart (float32)
            // NetworkVarNames: m_flFadeOutModelLength (float32)
            // NetworkVarNames: m_flFadeOutStart (float32)
            // NetworkVarNames: m_flFadeOutLength (float32)
            // NetworkVarNames: m_flStartTime (GameTime_t)
            // NetworkVarNames: m_nDissolveType (EntityDisolveType_t)
            // NetworkVarNames: m_vDissolverOrigin (Vector)
            // NetworkVarNames: m_nMagnitude (uint32)
            pub mod CEntityDissolve {
                pub const m_flFadeInStart: usize = 0x9F8; // float32
                pub const m_flFadeInLength: usize = 0x9FC; // float32
                pub const m_flFadeOutModelStart: usize = 0xA00; // float32
                pub const m_flFadeOutModelLength: usize = 0xA04; // float32
                pub const m_flFadeOutStart: usize = 0xA08; // float32
                pub const m_flFadeOutLength: usize = 0xA0C; // float32
                pub const m_flStartTime: usize = 0xA10; // GameTime_t
                pub const m_nDissolveType: usize = 0xA14; // EntityDisolveType_t
                pub const m_vDissolverOrigin: usize = 0xA18; // Vector
                pub const m_nMagnitude: usize = 0xA24; // uint32
            }
            // Parent: None
            // Fields count: 30
            //
            // Metadata:
            // NetworkVarNames: m_hTargetEntity (CHandle<CBaseEntity>)
            // NetworkVarNames: m_bState (bool)
            // NetworkVarNames: m_bAlwaysUpdate (bool)
            // NetworkVarNames: m_flLightFOV (float32)
            // NetworkVarNames: m_bEnableShadows (bool)
            // NetworkVarNames: m_bSimpleProjection (bool)
            // NetworkVarNames: m_bLightOnlyTarget (bool)
            // NetworkVarNames: m_bLightWorld (bool)
            // NetworkVarNames: m_bCameraSpace (bool)
            // NetworkVarNames: m_flBrightnessScale (float32)
            // NetworkVarNames: m_LightColor (Color)
            // NetworkVarNames: m_flIntensity (float32)
            // NetworkVarNames: m_flLinearAttenuation (float32)
            // NetworkVarNames: m_flQuadraticAttenuation (float32)
            // NetworkVarNames: m_bVolumetric (bool)
            // NetworkVarNames: m_flNoiseStrength (float32)
            // NetworkVarNames: m_flFlashlightTime (float32)
            // NetworkVarNames: m_nNumPlanes (uint32)
            // NetworkVarNames: m_flPlaneOffset (float32)
            // NetworkVarNames: m_flVolumetricIntensity (float32)
            // NetworkVarNames: m_flColorTransitionTime (float32)
            // NetworkVarNames: m_flAmbient (float32)
            // NetworkVarNames: m_SpotlightTextureName (char)
            // NetworkVarNames: m_nSpotlightTextureFrame (int32)
            // NetworkVarNames: m_nShadowQuality (uint32)
            // NetworkVarNames: m_flNearZ (float32)
            // NetworkVarNames: m_flFarZ (float32)
            // NetworkVarNames: m_flProjectionSize (float32)
            // NetworkVarNames: m_flRotation (float32)
            // NetworkVarNames: m_bFlipHorizontal (bool)
            pub mod CEnvProjectedTexture {
                pub const m_hTargetEntity: usize = 0x9F8; // CHandle<CBaseEntity>
                pub const m_bState: usize = 0x9FC; // bool
                pub const m_bAlwaysUpdate: usize = 0x9FD; // bool
                pub const m_flLightFOV: usize = 0xA00; // float32
                pub const m_bEnableShadows: usize = 0xA04; // bool
                pub const m_bSimpleProjection: usize = 0xA05; // bool
                pub const m_bLightOnlyTarget: usize = 0xA06; // bool
                pub const m_bLightWorld: usize = 0xA07; // bool
                pub const m_bCameraSpace: usize = 0xA08; // bool
                pub const m_flBrightnessScale: usize = 0xA0C; // float32
                pub const m_LightColor: usize = 0xA10; // Color
                pub const m_flIntensity: usize = 0xA14; // float32
                pub const m_flLinearAttenuation: usize = 0xA18; // float32
                pub const m_flQuadraticAttenuation: usize = 0xA1C; // float32
                pub const m_bVolumetric: usize = 0xA20; // bool
                pub const m_flNoiseStrength: usize = 0xA24; // float32
                pub const m_flFlashlightTime: usize = 0xA28; // float32
                pub const m_nNumPlanes: usize = 0xA2C; // uint32
                pub const m_flPlaneOffset: usize = 0xA30; // float32
                pub const m_flVolumetricIntensity: usize = 0xA34; // float32
                pub const m_flColorTransitionTime: usize = 0xA38; // float32
                pub const m_flAmbient: usize = 0xA3C; // float32
                pub const m_SpotlightTextureName: usize = 0xA40; // char[512]
                pub const m_nSpotlightTextureFrame: usize = 0xC40; // int32
                pub const m_nShadowQuality: usize = 0xC44; // uint32
                pub const m_flNearZ: usize = 0xC48; // float32
                pub const m_flFarZ: usize = 0xC4C; // float32
                pub const m_flProjectionSize: usize = 0xC50; // float32
                pub const m_flRotation: usize = 0xC54; // float32
                pub const m_bFlipHorizontal: usize = 0xC58; // bool
            }
            // Parent: CBaseModelEntity
            // Fields count: 9
            //
            // Metadata:
            // NetworkVarNames: m_hDecalMaterial (HMaterialStrong)
            // NetworkVarNames: m_flWidth (float)
            // NetworkVarNames: m_flHeight (float)
            // NetworkVarNames: m_flDepth (float)
            // NetworkVarNames: m_nRenderOrder (uint32)
            // NetworkVarNames: m_bProjectOnWorld (bool)
            // NetworkVarNames: m_bProjectOnCharacters (bool)
            // NetworkVarNames: m_bProjectOnWater (bool)
            // NetworkVarNames: m_flDepthSortBias (float)
            pub mod CEnvDecal {
                pub const m_hDecalMaterial: usize = 0x9F8; // CStrongHandle<InfoForResourceTypeIMaterial2>
                pub const m_flWidth: usize = 0xA00; // float32
                pub const m_flHeight: usize = 0xA04; // float32
                pub const m_flDepth: usize = 0xA08; // float32
                pub const m_nRenderOrder: usize = 0xA0C; // uint32
                pub const m_bProjectOnWorld: usize = 0xA10; // bool
                pub const m_bProjectOnCharacters: usize = 0xA11; // bool
                pub const m_bProjectOnWater: usize = 0xA12; // bool
                pub const m_flDepthSortBias: usize = 0xA14; // float32
            }
            // Parent: None
            // Fields count: 13
            //
            // Metadata:
            // MNetworkIncludeByName
            // MNetworkIncludeByName
            // MNetworkOverride
            pub mod CFuncMoveLinear {
                pub const m_authoredPosition: usize = 0xA78; // MoveLinearAuthoredPos_t
                pub const m_angMoveEntitySpace: usize = 0xA7C; // QAngle
                pub const m_vecMoveDirParentSpace: usize = 0xA88; // Vector
                pub const m_soundStart: usize = 0xA98; // CUtlSymbolLarge
                pub const m_soundStop: usize = 0xAA0; // CUtlSymbolLarge
                pub const m_currentSound: usize = 0xAA8; // CUtlSymbolLarge
                pub const m_flBlockDamage: usize = 0xAB0; // float32
                pub const m_flStartPosition: usize = 0xAB4; // float32
                pub const m_flMoveDistance: usize = 0xAB8; // float32
                pub const m_OnFullyOpen: usize = 0xAC8; // CEntityIOOutput
                pub const m_OnFullyClosed: usize = 0xAF0; // CEntityIOOutput
                pub const m_bCreateMovableNavMesh: usize = 0xB18; // bool
                pub const m_bCreateNavObstacle: usize = 0xB19; // bool
            }
            // Parent: CBaseModelEntity
            // Fields count: 19
            //
            // Metadata:
            // MNetworkOverride
            // MNetworkOverride
            pub mod CFuncRotating {
                pub const m_OnStopped: usize = 0x9F8; // CEntityIOOutput
                pub const m_OnStarted: usize = 0xA20; // CEntityIOOutput
                pub const m_OnReachedStart: usize = 0xA48; // CEntityIOOutput
                pub const m_localRotationVector: usize = 0xA70; // RotationVector
                pub const m_flFanFriction: usize = 0xA7C; // float32
                pub const m_flAttenuation: usize = 0xA80; // float32
                pub const m_flVolume: usize = 0xA84; // float32
                pub const m_flTargetSpeed: usize = 0xA88; // float32
                pub const m_flMaxSpeed: usize = 0xA8C; // float32
                pub const m_flBlockDamage: usize = 0xA90; // float32
                pub const m_flTimeScale: usize = 0xA94; // float32
                pub const m_NoiseRunning: usize = 0xA98; // CUtlSymbolLarge
                pub const m_bReversed: usize = 0xAA0; // bool
                pub const m_bAccelDecel: usize = 0xAA1; // bool
                pub const m_prevLocalAngles: usize = 0xAAC; // QAngle
                pub const m_angStart: usize = 0xAB8; // QAngle
                pub const m_bStopAtStartPos: usize = 0xAC4; // bool
                pub const m_vecClientOrigin: usize = 0xAC8; // Vector
                pub const m_vecClientAngles: usize = 0xAD4; // QAngle
            }
            // Parent: CBaseModelEntity
            // Fields count: 21
            //
            // Metadata:
            // MNetworkIncludeByUserGroup
            // MNetworkIncludeByName
            // MNetworkIncludeByName
            // MNetworkIncludeByName
            // MNetworkIncludeByName
            // MNetworkIncludeByName
            // MNetworkIncludeByName
            // NetworkVarNames: m_RopeFlags (uint16)
            // NetworkVarNames: m_Slack (int16)
            // NetworkVarNames: m_Width (float32)
            // NetworkVarNames: m_TextureScale (float32)
            // NetworkVarNames: m_nSegments (uint8)
            // NetworkVarNames: m_bConstrainBetweenEndpoints (bool)
            // NetworkVarNames: m_iRopeMaterialModelIndex (HMaterialStrong)
            // NetworkVarNames: m_Subdiv (uint8)
            // NetworkVarNames: m_nChangeCount (uint8)
            // NetworkVarNames: m_RopeLength (int16)
            // NetworkVarNames: m_fLockedPoints (uint8)
            // NetworkVarNames: m_flScrollSpeed (float32)
            // NetworkVarNames: m_hStartPoint (CHandle<CBaseEntity>)
            // NetworkVarNames: m_hEndPoint (CHandle<CBaseEntity>)
            // NetworkVarNames: m_iStartAttachment (AttachmentHandle_t)
            // NetworkVarNames: m_iEndAttachment (AttachmentHandle_t)
            pub mod CRopeKeyframe {
                pub const m_RopeFlags: usize = 0xA00; // uint16
                pub const m_iNextLinkName: usize = 0xA08; // CUtlSymbolLarge
                pub const m_Slack: usize = 0xA10; // int16
                pub const m_Width: usize = 0xA14; // float32
                pub const m_TextureScale: usize = 0xA18; // float32
                pub const m_nSegments: usize = 0xA1C; // uint8
                pub const m_bConstrainBetweenEndpoints: usize = 0xA1D; // bool
                pub const m_strRopeMaterialModel: usize = 0xA20; // CUtlSymbolLarge
                pub const m_iRopeMaterialModelIndex: usize = 0xA28; // CStrongHandle<InfoForResourceTypeIMaterial2>
                pub const m_Subdiv: usize = 0xA30; // uint8
                pub const m_nChangeCount: usize = 0xA31; // uint8
                pub const m_RopeLength: usize = 0xA32; // int16
                pub const m_fLockedPoints: usize = 0xA34; // uint8
                pub const m_bCreatedFromMapFile: usize = 0xA35; // bool
                pub const m_flScrollSpeed: usize = 0xA38; // float32
                pub const m_bStartPointValid: usize = 0xA3C; // bool
                pub const m_bEndPointValid: usize = 0xA3D; // bool
                pub const m_hStartPoint: usize = 0xA40; // CHandle<CBaseEntity>
                pub const m_hEndPoint: usize = 0xA44; // CHandle<CBaseEntity>
                pub const m_iStartAttachment: usize = 0xA48; // AttachmentHandle_t
                pub const m_iEndAttachment: usize = 0xA49; // AttachmentHandle_t
            }
            // Parent: CBaseModelEntity
            // Fields count: 4
            //
            // Metadata:
            // NetworkVarNames: m_flLightScale (float32)
            // NetworkVarNames: m_Radius (float32)
            pub mod CSpotlightEnd {
                pub const m_flLightScale: usize = 0x9F8; // float32
                pub const m_Radius: usize = 0x9FC; // float32
                pub const m_vSpotlightDir: usize = 0xA00; // Vector
                pub const m_vSpotlightOrg: usize = 0xA0C; // Vector
            }
            // Parent: None
            // Fields count: 11
            //
            // Metadata:
            // MNetworkIncludeByName
            // NetworkVarNames: m_bDisabled (bool)
            // NetworkVarNames: m_bClientSidePredicted (bool)
            pub mod CBaseTrigger {
                pub const m_bDisabled: usize = 0xA78; // bool
                pub const m_iFilterName: usize = 0xA80; // CUtlSymbolLarge
                pub const m_hFilter: usize = 0xA88; // CHandle<CBaseFilter>
                pub const m_OnStartTouch: usize = 0xA90; // CEntityIOOutput
                pub const m_OnStartTouchAll: usize = 0xAB8; // CEntityIOOutput
                pub const m_OnEndTouch: usize = 0xAE0; // CEntityIOOutput
                pub const m_OnEndTouchAll: usize = 0xB08; // CEntityIOOutput
                pub const m_OnTouching: usize = 0xB30; // CEntityIOOutput
                pub const m_OnNotTouching: usize = 0xB58; // CEntityIOOutput
                pub const m_hTouchingEntities: usize = 0xB80; // CUtlVector<CHandle<CBaseEntity>>
                pub const m_bClientSidePredicted: usize = 0xB98; // bool
            }
            // Parent: CBaseModelEntity
            // Fields count: 24
            //
            // Metadata:
            // MNetworkIncludeByUserGroup
            // MNetworkIncludeByName
            // MNetworkIncludeByName
            // MNetworkIncludeByName
            // MNetworkIncludeByName
            // MNetworkIncludeByName
            // MNetworkOverride
            // NetworkVarNames: m_flFrameRate (float32)
            // NetworkVarNames: m_flHDRColorScale (float32)
            // NetworkVarNames: m_nNumBeamEnts (uint8)
            // NetworkVarNames: m_hBaseMaterial (HMaterialStrong)
            // NetworkVarNames: m_nHaloIndex (HMaterialStrong)
            // NetworkVarNames: m_nBeamType (BeamType_t)
            // NetworkVarNames: m_nBeamFlags (uint32)
            // NetworkVarNames: m_hAttachEntity (CHandle<CBaseEntity>)
            // NetworkVarNames: m_nAttachIndex (AttachmentHandle_t)
            // NetworkVarNames: m_fWidth (float32)
            // NetworkVarNames: m_fEndWidth (float32)
            // NetworkVarNames: m_fFadeLength (float32)
            // NetworkVarNames: m_fHaloScale (float32)
            // NetworkVarNames: m_fAmplitude (float32)
            // NetworkVarNames: m_fStartFrame (float32)
            // NetworkVarNames: m_fSpeed (float32)
            // NetworkVarNames: m_flFrame (float32)
            // NetworkVarNames: m_nClipStyle (BeamClipStyle_t)
            // NetworkVarNames: m_bTurnedOff (bool)
            // NetworkVarNames: m_vecEndPos (Vector)
            pub mod CBeam {
                pub const m_flFrameRate: usize = 0x9F8; // float32
                pub const m_flHDRColorScale: usize = 0x9FC; // float32
                pub const m_flFireTime: usize = 0xA00; // GameTime_t
                pub const m_flDamage: usize = 0xA04; // float32
                pub const m_nNumBeamEnts: usize = 0xA08; // uint8
                pub const m_hBaseMaterial: usize = 0xA10; // CStrongHandle<InfoForResourceTypeIMaterial2>
                pub const m_nHaloIndex: usize = 0xA18; // CStrongHandle<InfoForResourceTypeIMaterial2>
                pub const m_nBeamType: usize = 0xA20; // BeamType_t
                pub const m_nBeamFlags: usize = 0xA24; // uint32
                pub const m_hAttachEntity: usize = 0xA28; // CHandle<CBaseEntity>[10]
                pub const m_nAttachIndex: usize = 0xA50; // AttachmentHandle_t[10]
                pub const m_fWidth: usize = 0xA5C; // float32
                pub const m_fEndWidth: usize = 0xA60; // float32
                pub const m_fFadeLength: usize = 0xA64; // float32
                pub const m_fHaloScale: usize = 0xA68; // float32
                pub const m_fAmplitude: usize = 0xA6C; // float32
                pub const m_fStartFrame: usize = 0xA70; // float32
                pub const m_fSpeed: usize = 0xA74; // float32
                pub const m_flFrame: usize = 0xA78; // float32
                pub const m_nClipStyle: usize = 0xA7C; // BeamClipStyle_t
                pub const m_bTurnedOff: usize = 0xA80; // bool
                pub const m_vecEndPos: usize = 0xA84; // Vector
                pub const m_hEndEntity: usize = 0xA90; // CHandle<CBaseEntity>
                pub const m_nDissolveType: usize = 0xA94; // int32
            }
            // Parent: CBaseModelEntity
            // Fields count: 12
            //
            // Metadata:
            // NetworkVarNames: m_vecLadderDir (Vector)
            // NetworkVarNames: m_vecPlayerMountPositionTop (Vector)
            // NetworkVarNames: m_vecPlayerMountPositionBottom (Vector)
            // NetworkVarNames: m_flAutoRideSpeed (float)
            // NetworkVarNames: m_bFakeLadder (bool)
            pub mod CFuncLadder {
                pub const m_vecLadderDir: usize = 0x9F8; // Vector
                pub const m_Dismounts: usize = 0xA08; // CUtlVector<CHandle<CInfoLadderDismount>>
                pub const m_vecLocalTop: usize = 0xA20; // Vector
                pub const m_vecPlayerMountPositionTop: usize = 0xA2C; // Vector
                pub const m_vecPlayerMountPositionBottom: usize = 0xA38; // Vector
                pub const m_flAutoRideSpeed: usize = 0xA44; // float32
                pub const m_bDisabled: usize = 0xA48; // bool
                pub const m_bFakeLadder: usize = 0xA49; // bool
                pub const m_bHasSlack: usize = 0xA4A; // bool
                pub const m_surfacePropName: usize = 0xA50; // CUtlSymbolLarge
                pub const m_OnPlayerGotOnLadder: usize = 0xA58; // CEntityIOOutput
                pub const m_OnPlayerGotOffLadder: usize = 0xA80; // CEntityIOOutput
            }
            // Parent: None
            // Fields count: 7
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod CPrecipitationVData {
                pub const m_szParticlePrecipitationEffect: usize = 0x28; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
                pub const m_flInnerDistance: usize = 0x108; // float32
                pub const m_nAttachType: usize = 0x10C; // ParticleAttachment_t
                pub const m_bBatchSameVolumeType: usize = 0x110; // bool
                pub const m_nRTEnvCP: usize = 0x114; // int32
                pub const m_nRTEnvCPComponent: usize = 0x118; // int32
                pub const m_szModifier: usize = 0x120; // CUtlString
            }
            // Parent: CBaseModelEntity
            // Fields count: 23
            //
            // Metadata:
            // NetworkVarNames: m_hSpriteMaterial (HMaterialStrong)
            // NetworkVarNames: m_hAttachedToEntity (CHandle<CBaseEntity>)
            // NetworkVarNames: m_nAttachment (AttachmentHandle_t)
            // NetworkVarNames: m_flSpriteFramerate (float32)
            // NetworkVarNames: m_flFrame (float32)
            // NetworkVarNames: m_nBrightness (uint32)
            // NetworkVarNames: m_flBrightnessDuration (float32)
            // NetworkVarNames: m_flSpriteScale (float32)
            // NetworkVarNames: m_flScaleDuration (float32)
            // NetworkVarNames: m_bWorldSpaceScale (bool)
            // NetworkVarNames: m_flGlowProxySize (float32)
            // NetworkVarNames: m_flHDRColorScale (float32)
            pub mod CSprite {
                pub const m_hSpriteMaterial: usize = 0x9F8; // CStrongHandle<InfoForResourceTypeIMaterial2>
                pub const m_hAttachedToEntity: usize = 0xA00; // CHandle<CBaseEntity>
                pub const m_nAttachment: usize = 0xA04; // AttachmentHandle_t
                pub const m_flSpriteFramerate: usize = 0xA08; // float32
                pub const m_flFrame: usize = 0xA0C; // float32
                pub const m_flDieTime: usize = 0xA10; // GameTime_t
                pub const m_nBrightness: usize = 0xA20; // uint32
                pub const m_flBrightnessDuration: usize = 0xA24; // float32
                pub const m_flSpriteScale: usize = 0xA28; // float32
                pub const m_flScaleDuration: usize = 0xA2C; // float32
                pub const m_bWorldSpaceScale: usize = 0xA30; // bool
                pub const m_flGlowProxySize: usize = 0xA34; // float32
                pub const m_flHDRColorScale: usize = 0xA38; // float32
                pub const m_flLastTime: usize = 0xA3C; // GameTime_t
                pub const m_flMaxFrame: usize = 0xA40; // float32
                pub const m_flStartScale: usize = 0xA44; // float32
                pub const m_flDestScale: usize = 0xA48; // float32
                pub const m_flScaleTimeStart: usize = 0xA4C; // GameTime_t
                pub const m_nStartBrightness: usize = 0xA50; // int32
                pub const m_nDestBrightness: usize = 0xA54; // int32
                pub const m_flBrightnessTimeStart: usize = 0xA58; // GameTime_t
                pub const m_nSpriteWidth: usize = 0xA5C; // int32
                pub const m_nSpriteHeight: usize = 0xA60; // int32
            }
            // Parent: CBaseModelEntity
            // Fields count: 14
            //
            // Metadata:
            // NetworkVarNames: m_bEnabled (bool)
            // NetworkVarNames: m_DialogXMLName (string_t)
            // NetworkVarNames: m_PanelClassName (string_t)
            // NetworkVarNames: m_PanelID (string_t)
            pub mod CBaseClientUIEntity {
                pub const m_bEnabled: usize = 0x9F8; // bool
                pub const m_DialogXMLName: usize = 0xA00; // CUtlSymbolLarge
                pub const m_PanelClassName: usize = 0xA08; // CUtlSymbolLarge
                pub const m_PanelID: usize = 0xA10; // CUtlSymbolLarge
                pub const m_CustomOutput0: usize = 0xA18; // CEntityIOOutput
                pub const m_CustomOutput1: usize = 0xA40; // CEntityIOOutput
                pub const m_CustomOutput2: usize = 0xA68; // CEntityIOOutput
                pub const m_CustomOutput3: usize = 0xA90; // CEntityIOOutput
                pub const m_CustomOutput4: usize = 0xAB8; // CEntityIOOutput
                pub const m_CustomOutput5: usize = 0xAE0; // CEntityIOOutput
                pub const m_CustomOutput6: usize = 0xB08; // CEntityIOOutput
                pub const m_CustomOutput7: usize = 0xB30; // CEntityIOOutput
                pub const m_CustomOutput8: usize = 0xB58; // CEntityIOOutput
                pub const m_CustomOutput9: usize = 0xB80; // CEntityIOOutput
            }
            // Parent: CBaseClientUIEntity
            // Fields count: 2
            //
            // Metadata:
            // NetworkVarNames: m_hActivator (EHANDLE)
            pub mod CPointClientUIDialog {
                pub const m_hActivator: usize = 0xBA8; // CHandle<CBaseEntity>
                pub const m_bStartEnabled: usize = 0xBAC; // bool
            }
            // Parent: CBaseClientUIEntity
            // Fields count: 23
            //
            // Metadata:
            // NetworkVarNames: m_bIgnoreInput (bool)
            // NetworkVarNames: m_bLit (bool)
            // NetworkVarNames: m_bFollowPlayerAcrossTeleport (bool)
            // NetworkVarNames: m_flWidth (float)
            // NetworkVarNames: m_flHeight (float)
            // NetworkVarNames: m_flDPI (float)
            // NetworkVarNames: m_flInteractDistance (float)
            // NetworkVarNames: m_flDepthOffset (float)
            // NetworkVarNames: m_unOwnerContext (uint32)
            // NetworkVarNames: m_unHorizontalAlign (uint32)
            // NetworkVarNames: m_unVerticalAlign (uint32)
            // NetworkVarNames: m_unOrientation (uint32)
            // NetworkVarNames: m_bAllowInteractionFromAllSceneWorlds (bool)
            // NetworkVarNames: m_vecCSSClasses (string_t)
            // NetworkVarNames: m_bOpaque (bool)
            // NetworkVarNames: m_bNoDepth (bool)
            // NetworkVarNames: m_bRenderBackface (bool)
            // NetworkVarNames: m_bUseOffScreenIndicator (bool)
            // NetworkVarNames: m_bExcludeFromSaveGames (bool)
            // NetworkVarNames: m_bGrabbable (bool)
            // NetworkVarNames: m_bOnlyRenderToTexture (bool)
            // NetworkVarNames: m_bDisableMipGen (bool)
            // NetworkVarNames: m_nExplicitImageLayout (int32)
            pub mod CPointClientUIWorldPanel {
                pub const m_bIgnoreInput: usize = 0xBA8; // bool
                pub const m_bLit: usize = 0xBA9; // bool
                pub const m_bFollowPlayerAcrossTeleport: usize = 0xBAA; // bool
                pub const m_flWidth: usize = 0xBAC; // float32
                pub const m_flHeight: usize = 0xBB0; // float32
                pub const m_flDPI: usize = 0xBB4; // float32
                pub const m_flInteractDistance: usize = 0xBB8; // float32
                pub const m_flDepthOffset: usize = 0xBBC; // float32
                pub const m_unOwnerContext: usize = 0xBC0; // uint32
                pub const m_unHorizontalAlign: usize = 0xBC4; // uint32
                pub const m_unVerticalAlign: usize = 0xBC8; // uint32
                pub const m_unOrientation: usize = 0xBCC; // uint32
                pub const m_bAllowInteractionFromAllSceneWorlds: usize = 0xBD0; // bool
                pub const m_vecCSSClasses: usize = 0xBD8; // CNetworkUtlVectorBase<CUtlSymbolLarge>
                pub const m_bOpaque: usize = 0xBF0; // bool
                pub const m_bNoDepth: usize = 0xBF1; // bool
                pub const m_bRenderBackface: usize = 0xBF2; // bool
                pub const m_bUseOffScreenIndicator: usize = 0xBF3; // bool
                pub const m_bExcludeFromSaveGames: usize = 0xBF4; // bool
                pub const m_bGrabbable: usize = 0xBF5; // bool
                pub const m_bOnlyRenderToTexture: usize = 0xBF6; // bool
                pub const m_bDisableMipGen: usize = 0xBF7; // bool
                pub const m_nExplicitImageLayout: usize = 0xBF8; // int32
            }
            // Parent: CPointClientUIWorldPanel
            // Fields count: 1
            //
            // Metadata:
            // NetworkVarNames: m_messageText (char)
            pub mod CPointClientUIWorldTextPanel {
                pub const m_messageText: usize = 0xBFC; // char[512]
            }
            // Parent: None
            // Fields count: 10
            //
            // Metadata:
            // NetworkVarNames: m_bDisabled (bool)
            // NetworkVarNames: m_nResolutionX (int)
            // NetworkVarNames: m_nResolutionY (int)
            // NetworkVarNames: m_szLayoutFileName (string_t)
            // NetworkVarNames: m_RenderAttrName (string_t)
            // NetworkVarNames: m_TargetEntities (CHandle<CBaseModelEntity>)
            // NetworkVarNames: m_nTargetChangeCount (int)
            // NetworkVarNames: m_vecCSSClasses (string_t)
            pub mod CInfoOffscreenPanoramaTexture {
                pub const m_bDisabled: usize = 0x7A8; // bool
                pub const m_nResolutionX: usize = 0x7AC; // int32
                pub const m_nResolutionY: usize = 0x7B0; // int32
                pub const m_szLayoutFileName: usize = 0x7B8; // CUtlSymbolLarge
                pub const m_RenderAttrName: usize = 0x7C0; // CUtlSymbolLarge
                pub const m_TargetEntities: usize = 0x7C8; // CNetworkUtlVectorBase<CHandle<CBaseModelEntity>>
                pub const m_nTargetChangeCount: usize = 0x7E0; // int32
                pub const m_vecCSSClasses: usize = 0x7E8; // CNetworkUtlVectorBase<CUtlSymbolLarge>
                pub const m_szTargetsName: usize = 0x800; // CUtlSymbolLarge
                pub const m_AdditionalTargetEntities: usize = 0x808; // CUtlVector<CHandle<CBaseModelEntity>>
            }
            // Parent: None
            // Fields count: 13
            //
            // Metadata:
            // NetworkVarNames: m_iItemDefinitionIndex (item_definition_index_t)
            // NetworkVarNames: m_iEntityQuality (int)
            // NetworkVarNames: m_iEntityLevel (uint32)
            // NetworkVarNames: m_iItemIDHigh (uint32)
            // NetworkVarNames: m_iItemIDLow (uint32)
            // NetworkVarNames: m_iAccountID (uint32)
            // NetworkVarNames: m_iInventoryPosition (uint32)
            // NetworkVarNames: m_bInitialized (bool)
            // NetworkVarNames: m_AttributeList (CAttributeList)
            // NetworkVarNames: m_NetworkedDynamicAttributes (CAttributeList)
            // NetworkVarNames: m_szCustomName (char)
            pub mod CEconItemView {
                pub const m_iItemDefinitionIndex: usize = 0x38; // uint16
                pub const m_iEntityQuality: usize = 0x3C; // int32
                pub const m_iEntityLevel: usize = 0x40; // uint32
                pub const m_iItemID: usize = 0x48; // uint64
                pub const m_iItemIDHigh: usize = 0x50; // uint32
                pub const m_iItemIDLow: usize = 0x54; // uint32
                pub const m_iAccountID: usize = 0x58; // uint32
                pub const m_iInventoryPosition: usize = 0x5C; // uint32
                pub const m_bInitialized: usize = 0x68; // bool
                pub const m_AttributeList: usize = 0x70; // CAttributeList
                pub const m_NetworkedDynamicAttributes: usize = 0xD0; // CAttributeList
                pub const m_szCustomName: usize = 0x130; // char[161]
                pub const m_szCustomNameOverride: usize = 0x1D1; // char[161]
            }
            // Parent: CBaseTrigger
            // Fields count: 9
            //
            // Metadata:
            // NetworkVarNames: m_bBombPlantedHere (bool)
            pub mod CBombTarget {
                pub const m_OnBombExplode: usize = 0xBA0; // CEntityIOOutput
                pub const m_OnBombPlanted: usize = 0xBC8; // CEntityIOOutput
                pub const m_OnBombDefused: usize = 0xBF0; // CEntityIOOutput
                pub const m_bIsBombSiteB: usize = 0xC18; // bool
                pub const m_bIsHeistBombTarget: usize = 0xC19; // bool
                pub const m_bBombPlantedHere: usize = 0xC1A; // bool
                pub const m_szMountTarget: usize = 0xC20; // CUtlSymbolLarge
                pub const m_hInstructorHint: usize = 0xC28; // CHandle<CBaseEntity>
                pub const m_nBombSiteDesignation: usize = 0xC2C; // int32
            }
            // Parent: CBaseTrigger
            // Fields count: 2
            //
            // Metadata:
            // NetworkVarNames: m_flFluidDensity (float)
            pub mod CTriggerBuoyancy {
                pub const m_BuoyancyHelper: usize = 0xBA0; // CBuoyancyHelper
                pub const m_flFluidDensity: usize = 0xBC0; // float32
            }
            // Parent: CBasePlayerController
            // Fields count: 86
            //
            // Metadata:
            // MNetworkUserGroupProxy
            // MNetworkUserGroupProxy
            // MNetworkUserGroupProxy
            // NetworkVarNames: m_pInGameMoneyServices (CCSPlayerController_InGameMoneyServices*)
            // NetworkVarNames: m_pInventoryServices (CCSPlayerController_InventoryServices*)
            // NetworkVarNames: m_pActionTrackingServices (CCSPlayerController_ActionTrackingServices*)
            // NetworkVarNames: m_pDamageServices (CCSPlayerController_DamageServices*)
            // NetworkVarNames: m_iPing (uint32)
            // NetworkVarNames: m_bHasCommunicationAbuseMute (bool)
            // NetworkVarNames: m_szCrosshairCodes (string_t)
            // NetworkVarNames: m_iPendingTeamNum (uint8)
            // NetworkVarNames: m_flForceTeamTime (GameTime_t)
            // NetworkVarNames: m_iCompTeammateColor (int)
            // NetworkVarNames: m_bEverPlayedOnTeam (bool)
            // NetworkVarNames: m_szClan (string_t)
            // NetworkVarNames: m_iCoachingTeam (int)
            // NetworkVarNames: m_nPlayerDominated (uint64)
            // NetworkVarNames: m_nPlayerDominatingMe (uint64)
            // NetworkVarNames: m_iCompetitiveRanking (int)
            // NetworkVarNames: m_iCompetitiveWins (int)
            // NetworkVarNames: m_iCompetitiveRankType (int8)
            // NetworkVarNames: m_iCompetitiveRankingPredicted_Win (int)
            // NetworkVarNames: m_iCompetitiveRankingPredicted_Loss (int)
            // NetworkVarNames: m_iCompetitiveRankingPredicted_Tie (int)
            // NetworkVarNames: m_nEndMatchNextMapVote (int)
            // NetworkVarNames: m_unActiveQuestId (uint16)
            // NetworkVarNames: m_nQuestProgressReason (QuestProgress::Reason)
            // NetworkVarNames: m_unPlayerTvControlFlags (uint32)
            // NetworkVarNames: m_nDisconnectionTick (int)
            // NetworkVarNames: m_bControllingBot (bool)
            // NetworkVarNames: m_bHasControlledBotThisRound (bool)
            // NetworkVarNames: m_bCanControlObservedBot (bool)
            // NetworkVarNames: m_hPlayerPawn (CHandle<CCSPlayerPawn>)
            // NetworkVarNames: m_hObserverPawn (CHandle<CCSObserverPawn>)
            // NetworkVarNames: m_bPawnIsAlive (bool)
            // NetworkVarNames: m_iPawnHealth (uint32)
            // NetworkVarNames: m_iPawnArmor (int)
            // NetworkVarNames: m_bPawnHasDefuser (bool)
            // NetworkVarNames: m_bPawnHasHelmet (bool)
            // NetworkVarNames: m_nPawnCharacterDefIndex (item_definition_index_t)
            // NetworkVarNames: m_iPawnLifetimeStart (int)
            // NetworkVarNames: m_iPawnLifetimeEnd (int)
            // NetworkVarNames: m_iPawnBotDifficulty (int)
            // NetworkVarNames: m_hOriginalControllerOfCurrentPawn (CHandle<CCSPlayerController>)
            // NetworkVarNames: m_iScore (int32)
            // NetworkVarNames: m_vecKills (EKillTypes_t)
            // NetworkVarNames: m_bMvpNoMusic (bool)
            // NetworkVarNames: m_eMvpReason (int)
            // NetworkVarNames: m_iMusicKitID (int)
            // NetworkVarNames: m_iMusicKitMVPs (int)
            // NetworkVarNames: m_iMVPs (int)
            pub mod CCSPlayerController {
                pub const m_pInGameMoneyServices: usize = 0x9A8; // CCSPlayerController_InGameMoneyServices*
                pub const m_pInventoryServices: usize = 0x9B0; // CCSPlayerController_InventoryServices*
                pub const m_pActionTrackingServices: usize = 0x9B8; // CCSPlayerController_ActionTrackingServices*
                pub const m_pDamageServices: usize = 0x9C0; // CCSPlayerController_DamageServices*
                pub const m_iPing: usize = 0x9C8; // uint32
                pub const m_bHasCommunicationAbuseMute: usize = 0x9CC; // bool
                pub const m_szCrosshairCodes: usize = 0x9D0; // CUtlSymbolLarge
                pub const m_iPendingTeamNum: usize = 0x9D8; // uint8
                pub const m_flForceTeamTime: usize = 0x9DC; // GameTime_t
                pub const m_iCompTeammateColor: usize = 0x9E0; // int32
                pub const m_bEverPlayedOnTeam: usize = 0x9E4; // bool
                pub const m_bAttemptedToGetColor: usize = 0x9E5; // bool
                pub const m_iTeammatePreferredColor: usize = 0x9E8; // int32
                pub const m_bTeamChanged: usize = 0x9EC; // bool
                pub const m_bInSwitchTeam: usize = 0x9ED; // bool
                pub const m_bHasSeenJoinGame: usize = 0x9EE; // bool
                pub const m_bJustBecameSpectator: usize = 0x9EF; // bool
                pub const m_bSwitchTeamsOnNextRoundReset: usize = 0x9F0; // bool
                pub const m_bRemoveAllItemsOnNextRoundReset: usize = 0x9F1; // bool
                pub const m_szClan: usize = 0x9F8; // CUtlSymbolLarge
                pub const m_szClanName: usize = 0xA00; // char[32]
                pub const m_iCoachingTeam: usize = 0xA20; // int32
                pub const m_nPlayerDominated: usize = 0xA28; // uint64
                pub const m_nPlayerDominatingMe: usize = 0xA30; // uint64
                pub const m_iCompetitiveRanking: usize = 0xA38; // int32
                pub const m_iCompetitiveWins: usize = 0xA3C; // int32
                pub const m_iCompetitiveRankType: usize = 0xA40; // int8
                pub const m_iCompetitiveRankingPredicted_Win: usize = 0xA44; // int32
                pub const m_iCompetitiveRankingPredicted_Loss: usize = 0xA48; // int32
                pub const m_iCompetitiveRankingPredicted_Tie: usize = 0xA4C; // int32
                pub const m_nEndMatchNextMapVote: usize = 0xA50; // int32
                pub const m_unActiveQuestId: usize = 0xA54; // uint16
                pub const m_nQuestProgressReason: usize = 0xA58; // QuestProgress::Reason
                pub const m_unPlayerTvControlFlags: usize = 0xA5C; // uint32
                pub const m_iDraftIndex: usize = 0xA88; // int32
                pub const m_msQueuedModeDisconnectionTimestamp: usize = 0xA8C; // uint32
                pub const m_uiAbandonRecordedReason: usize = 0xA90; // uint32
                pub const m_bCannotBeKicked: usize = 0xA94; // bool
                pub const m_bEverFullyConnected: usize = 0xA95; // bool
                pub const m_bAbandonAllowsSurrender: usize = 0xA96; // bool
                pub const m_bAbandonOffersInstantSurrender: usize = 0xA97; // bool
                pub const m_bDisconnection1MinWarningPrinted: usize = 0xA98; // bool
                pub const m_bScoreReported: usize = 0xA99; // bool
                pub const m_nDisconnectionTick: usize = 0xA9C; // int32
                pub const m_bControllingBot: usize = 0xAA8; // bool
                pub const m_bHasControlledBotThisRound: usize = 0xAA9; // bool
                pub const m_bHasBeenControlledByPlayerThisRound: usize = 0xAAA; // bool
                pub const m_nBotsControlledThisRound: usize = 0xAAC; // int32
                pub const m_bCanControlObservedBot: usize = 0xAB0; // bool
                pub const m_hPlayerPawn: usize = 0xAB4; // CHandle<CCSPlayerPawn>
                pub const m_hObserverPawn: usize = 0xAB8; // CHandle<CCSObserverPawn>
                pub const m_DesiredObserverMode: usize = 0xABC; // int32
                pub const m_hDesiredObserverTarget: usize = 0xAC0; // CEntityHandle
                pub const m_bPawnIsAlive: usize = 0xAC4; // bool
                pub const m_iPawnHealth: usize = 0xAC8; // uint32
                pub const m_iPawnArmor: usize = 0xACC; // int32
                pub const m_bPawnHasDefuser: usize = 0xAD0; // bool
                pub const m_bPawnHasHelmet: usize = 0xAD1; // bool
                pub const m_nPawnCharacterDefIndex: usize = 0xAD2; // uint16
                pub const m_iPawnLifetimeStart: usize = 0xAD4; // int32
                pub const m_iPawnLifetimeEnd: usize = 0xAD8; // int32
                pub const m_iPawnBotDifficulty: usize = 0xADC; // int32
                pub const m_hOriginalControllerOfCurrentPawn: usize = 0xAE0; // CHandle<CCSPlayerController>
                pub const m_iScore: usize = 0xAE4; // int32
                pub const m_iRoundScore: usize = 0xAE8; // int32
                pub const m_iRoundsWon: usize = 0xAEC; // int32
                pub const m_vecKills: usize = 0xAF0; // CNetworkUtlVectorBase<EKillTypes_t>
                pub const m_bMvpNoMusic: usize = 0xB08; // bool
                pub const m_eMvpReason: usize = 0xB0C; // int32
                pub const m_iMusicKitID: usize = 0xB10; // int32
                pub const m_iMusicKitMVPs: usize = 0xB14; // int32
                pub const m_iMVPs: usize = 0xB18; // int32
                pub const m_nUpdateCounter: usize = 0xB1C; // int32
                pub const m_flSmoothedPing: usize = 0xB20; // float32
                pub const m_lastHeldVoteTimer: usize = 0xFBC8; // IntervalTimer
                pub const m_bShowHints: usize = 0xFBE0; // bool
                pub const m_iNextTimeCheck: usize = 0xFBE4; // int32
                pub const m_bJustDidTeamKill: usize = 0xFBE8; // bool
                pub const m_bPunishForTeamKill: usize = 0xFBE9; // bool
                pub const m_bGaveTeamDamageWarning: usize = 0xFBEA; // bool
                pub const m_bGaveTeamDamageWarningThisRound: usize = 0xFBEB; // bool
                pub const m_dblLastReceivedPacketPlatFloatTime: usize = 0xFBF0; // float64
                pub const m_LastTeamDamageWarningTime: usize = 0xFBF8; // GameTime_t
                pub const m_LastTimePlayerWasDisconnectedForPawnsRemove: usize = 0xFBFC; // GameTime_t
                pub const m_nSuspiciousHitCount: usize = 0xFC00; // uint32
                pub const m_nNonSuspiciousHitStreak: usize = 0xFC04; // uint32
            }
            // Parent: CBaseTrigger
            // Fields count: 2
            //
            // Metadata:
            // NetworkVarNames: m_source (string_t)
            // NetworkVarNames: m_destination (string_t)
            pub mod CFootstepControl {
                pub const m_source: usize = 0xBA0; // CUtlSymbolLarge
                pub const m_destination: usize = 0xBA8; // CUtlSymbolLarge
            }
            // Parent: CBasePlayerWeaponVData
            // Fields count: 92
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MPropertySuppressBaseClassField
            // MPropertySuppressBaseClassField
            pub mod CCSWeaponBaseVData {
                pub const m_WeaponType: usize = 0x248; // CSWeaponType
                pub const m_WeaponCategory: usize = 0x24C; // CSWeaponCategory
                pub const m_szViewModel: usize = 0x250; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeCModel>>
                pub const m_szPlayerModel: usize = 0x330; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeCModel>>
                pub const m_szWorldDroppedModel: usize = 0x410; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeCModel>>
                pub const m_szAimsightLensMaskModel: usize = 0x4F0; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeCModel>>
                pub const m_szMagazineModel: usize = 0x5D0; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeCModel>>
                pub const m_szHeatEffect: usize = 0x6B0; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
                pub const m_szEjectBrassEffect: usize = 0x790; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
                pub const m_szMuzzleFlashParticleAlt: usize = 0x870; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
                pub const m_szMuzzleFlashThirdPersonParticle: usize = 0x950; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
                pub const m_szMuzzleFlashThirdPersonParticleAlt: usize = 0xA30; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
                pub const m_szTracerParticle: usize = 0xB10; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
                pub const m_GearSlot: usize = 0xBF0; // gear_slot_t
                pub const m_GearSlotPosition: usize = 0xBF4; // int32
                pub const m_DefaultLoadoutSlot: usize = 0xBF8; // loadout_slot_t
                pub const m_sWrongTeamMsg: usize = 0xC00; // CUtlString
                pub const m_nPrice: usize = 0xC08; // int32
                pub const m_nKillAward: usize = 0xC0C; // int32
                pub const m_nPrimaryReserveAmmoMax: usize = 0xC10; // int32
                pub const m_nSecondaryReserveAmmoMax: usize = 0xC14; // int32
                pub const m_bMeleeWeapon: usize = 0xC18; // bool
                pub const m_bHasBurstMode: usize = 0xC19; // bool
                pub const m_bIsRevolver: usize = 0xC1A; // bool
                pub const m_bCannotShootUnderwater: usize = 0xC1B; // bool
                pub const m_szName: usize = 0xC20; // CGlobalSymbol
                pub const m_szAnimExtension: usize = 0xC28; // CUtlString
                pub const m_eSilencerType: usize = 0xC30; // CSWeaponSilencerType
                pub const m_nCrosshairMinDistance: usize = 0xC34; // int32
                pub const m_nCrosshairDeltaDistance: usize = 0xC38; // int32
                pub const m_bIsFullAuto: usize = 0xC3C; // bool
                pub const m_nNumBullets: usize = 0xC40; // int32
                pub const m_flCycleTime: usize = 0xC44; // CFiringModeFloat
                pub const m_flMaxSpeed: usize = 0xC4C; // CFiringModeFloat
                pub const m_flSpread: usize = 0xC54; // CFiringModeFloat
                pub const m_flInaccuracyCrouch: usize = 0xC5C; // CFiringModeFloat
                pub const m_flInaccuracyStand: usize = 0xC64; // CFiringModeFloat
                pub const m_flInaccuracyJump: usize = 0xC6C; // CFiringModeFloat
                pub const m_flInaccuracyLand: usize = 0xC74; // CFiringModeFloat
                pub const m_flInaccuracyLadder: usize = 0xC7C; // CFiringModeFloat
                pub const m_flInaccuracyFire: usize = 0xC84; // CFiringModeFloat
                pub const m_flInaccuracyMove: usize = 0xC8C; // CFiringModeFloat
                pub const m_flRecoilAngle: usize = 0xC94; // CFiringModeFloat
                pub const m_flRecoilAngleVariance: usize = 0xC9C; // CFiringModeFloat
                pub const m_flRecoilMagnitude: usize = 0xCA4; // CFiringModeFloat
                pub const m_flRecoilMagnitudeVariance: usize = 0xCAC; // CFiringModeFloat
                pub const m_nTracerFrequency: usize = 0xCB4; // CFiringModeInt
                pub const m_flInaccuracyJumpInitial: usize = 0xCBC; // float32
                pub const m_flInaccuracyJumpApex: usize = 0xCC0; // float32
                pub const m_flInaccuracyReload: usize = 0xCC4; // float32
                pub const m_nRecoilSeed: usize = 0xCC8; // int32
                pub const m_nSpreadSeed: usize = 0xCCC; // int32
                pub const m_flTimeToIdleAfterFire: usize = 0xCD0; // float32
                pub const m_flIdleInterval: usize = 0xCD4; // float32
                pub const m_flAttackMovespeedFactor: usize = 0xCD8; // float32
                pub const m_flHeatPerShot: usize = 0xCDC; // float32
                pub const m_flInaccuracyPitchShift: usize = 0xCE0; // float32
                pub const m_flInaccuracyAltSoundThreshold: usize = 0xCE4; // float32
                pub const m_flBotAudibleRange: usize = 0xCE8; // float32
                pub const m_szUseRadioSubtitle: usize = 0xCF0; // CUtlString
                pub const m_bUnzoomsAfterShot: usize = 0xCF8; // bool
                pub const m_bHideViewModelWhenZoomed: usize = 0xCF9; // bool
                pub const m_nZoomLevels: usize = 0xCFC; // int32
                pub const m_nZoomFOV1: usize = 0xD00; // int32
                pub const m_nZoomFOV2: usize = 0xD04; // int32
                pub const m_flZoomTime0: usize = 0xD08; // float32
                pub const m_flZoomTime1: usize = 0xD0C; // float32
                pub const m_flZoomTime2: usize = 0xD10; // float32
                pub const m_flIronSightPullUpSpeed: usize = 0xD14; // float32
                pub const m_flIronSightPutDownSpeed: usize = 0xD18; // float32
                pub const m_flIronSightFOV: usize = 0xD1C; // float32
                pub const m_flIronSightPivotForward: usize = 0xD20; // float32
                pub const m_flIronSightLooseness: usize = 0xD24; // float32
                pub const m_angPivotAngle: usize = 0xD28; // QAngle
                pub const m_vecIronSightEyePos: usize = 0xD34; // Vector
                pub const m_nDamage: usize = 0xD40; // int32
                pub const m_flHeadshotMultiplier: usize = 0xD44; // float32
                pub const m_flArmorRatio: usize = 0xD48; // float32
                pub const m_flPenetration: usize = 0xD4C; // float32
                pub const m_flRange: usize = 0xD50; // float32
                pub const m_flRangeModifier: usize = 0xD54; // float32
                pub const m_flFlinchVelocityModifierLarge: usize = 0xD58; // float32
                pub const m_flFlinchVelocityModifierSmall: usize = 0xD5C; // float32
                pub const m_flRecoveryTimeCrouch: usize = 0xD60; // float32
                pub const m_flRecoveryTimeStand: usize = 0xD64; // float32
                pub const m_flRecoveryTimeCrouchFinal: usize = 0xD68; // float32
                pub const m_flRecoveryTimeStandFinal: usize = 0xD6C; // float32
                pub const m_nRecoveryTransitionStartBullet: usize = 0xD70; // int32
                pub const m_nRecoveryTransitionEndBullet: usize = 0xD74; // int32
                pub const m_flThrowVelocity: usize = 0xD78; // float32
                pub const m_vSmokeColor: usize = 0xD7C; // Vector
                pub const m_szAnimClass: usize = 0xD88; // CGlobalSymbol
            }
            // Parent: CBaseModelEntity
            // Fields count: 9
            //
            // Metadata:
            // MNetworkIncludeByName
            // MNetworkOverride
            // NetworkVarNames: m_vecMoveDirEntitySpace (Vector)
            // NetworkVarNames: m_flTargetSpeed (float32)
            // NetworkVarNames: m_nTransitionStartTick (GameTick_t)
            // NetworkVarNames: m_nTransitionDurationTicks (int)
            // NetworkVarNames: m_flTransitionStartSpeed (float32)
            // NetworkVarNames: m_hConveyorModels (EHANDLE)
            pub mod CFuncConveyor {
                pub const m_szConveyorModels: usize = 0x9F8; // CUtlSymbolLarge
                pub const m_flTransitionDurationSeconds: usize = 0xA00; // float32
                pub const m_angMoveEntitySpace: usize = 0xA04; // QAngle
                pub const m_vecMoveDirEntitySpace: usize = 0xA10; // Vector
                pub const m_flTargetSpeed: usize = 0xA1C; // float32
                pub const m_nTransitionStartTick: usize = 0xA20; // GameTick_t
                pub const m_nTransitionDurationTicks: usize = 0xA24; // int32
                pub const m_flTransitionStartSpeed: usize = 0xA28; // float32
                pub const m_hConveyorModels: usize = 0xA30; // CNetworkUtlVectorBase<CHandle<CBaseEntity>>
            }
            // Parent: None
            // Fields count: 15
            //
            // Metadata:
            // NetworkVarNames: m_nUniqueID (int)
            // NetworkVarNames: m_unAccountID (uint32)
            // NetworkVarNames: m_unTraceID (uint32)
            // NetworkVarNames: m_rtGcTime (uint32)
            // NetworkVarNames: m_vecEndPos (Vector)
            // NetworkVarNames: m_vecStart (Vector)
            // NetworkVarNames: m_vecLeft (Vector)
            // NetworkVarNames: m_vecNormal (Vector)
            // NetworkVarNames: m_nPlayer (int)
            // NetworkVarNames: m_nEntity (int)
            // NetworkVarNames: m_nHitbox (int)
            // NetworkVarNames: m_flCreationTime (float)
            // NetworkVarNames: m_nTintID (int)
            // NetworkVarNames: m_nVersion (uint8)
            // NetworkVarNames: m_ubSignature (uint8)
            pub mod CPlayerSprayDecal {
                pub const m_nUniqueID: usize = 0x9F8; // int32
                pub const m_unAccountID: usize = 0x9FC; // uint32
                pub const m_unTraceID: usize = 0xA00; // uint32
                pub const m_rtGcTime: usize = 0xA04; // uint32
                pub const m_vecEndPos: usize = 0xA08; // Vector
                pub const m_vecStart: usize = 0xA14; // Vector
                pub const m_vecLeft: usize = 0xA20; // Vector
                pub const m_vecNormal: usize = 0xA2C; // Vector
                pub const m_nPlayer: usize = 0xA38; // int32
                pub const m_nEntity: usize = 0xA3C; // int32
                pub const m_nHitbox: usize = 0xA40; // int32
                pub const m_flCreationTime: usize = 0xA44; // float32
                pub const m_nTintID: usize = 0xA48; // int32
                pub const m_nVersion: usize = 0xA4C; // uint8
                pub const m_ubSignature: usize = 0xA4D; // uint8[128]
            }
            // Parent: CBaseModelEntity
            // Fields count: 25
            //
            // Metadata:
            // NetworkVarNames: m_firePositions (Vector)
            // NetworkVarNames: m_fireParentPositions (Vector)
            // NetworkVarNames: m_bFireIsBurning (bool)
            // NetworkVarNames: m_BurnNormal (Vector)
            // NetworkVarNames: m_fireCount (int)
            // NetworkVarNames: m_nInfernoType (int)
            // NetworkVarNames: m_nFireEffectTickBegin (int)
            // NetworkVarNames: m_nFireLifetime (float)
            // NetworkVarNames: m_bInPostEffectTime (bool)
            pub mod CInferno {
                pub const m_firePositions: usize = 0xA04; // Vector[64]
                pub const m_fireParentPositions: usize = 0xD04; // Vector[64]
                pub const m_bFireIsBurning: usize = 0x1004; // bool[64]
                pub const m_BurnNormal: usize = 0x1044; // Vector[64]
                pub const m_fireCount: usize = 0x1344; // int32
                pub const m_nInfernoType: usize = 0x1348; // int32
                pub const m_nFireEffectTickBegin: usize = 0x134C; // int32
                pub const m_nFireLifetime: usize = 0x1350; // float32
                pub const m_bInPostEffectTime: usize = 0x1354; // bool
                pub const m_nFiresExtinguishCount: usize = 0x1358; // int32
                pub const m_bWasCreatedInSmoke: usize = 0x135C; // bool
                pub const m_extent: usize = 0x1560; // Extent
                pub const m_damageTimer: usize = 0x1578; // CountdownTimer
                pub const m_damageRampTimer: usize = 0x1590; // CountdownTimer
                pub const m_splashVelocity: usize = 0x15A8; // Vector
                pub const m_InitialSplashVelocity: usize = 0x15B4; // Vector
                pub const m_startPos: usize = 0x15C0; // Vector
                pub const m_vecOriginalSpawnLocation: usize = 0x15CC; // Vector
                pub const m_activeTimer: usize = 0x15D8; // IntervalTimer
                pub const m_fireSpawnOffset: usize = 0x15E8; // int32
                pub const m_nMaxFlames: usize = 0x15EC; // int32
                pub const m_nSpreadCount: usize = 0x15F0; // int32
                pub const m_BookkeepingTimer: usize = 0x15F8; // CountdownTimer
                pub const m_NextSpreadTimer: usize = 0x1610; // CountdownTimer
                pub const m_nSourceItemDefIndex: usize = 0x1628; // uint16
            }
            // Parent: CBaseModelEntity
            // Fields count: 52
            //
            // Metadata:
            // NetworkVarNames: m_bEnabled (bool)
            // NetworkVarNames: m_nColorMode (int)
            // NetworkVarNames: m_Color (Color)
            // NetworkVarNames: m_flColorTemperature (float)
            // NetworkVarNames: m_flBrightness (float)
            // NetworkVarNames: m_flBrightnessScale (float)
            // NetworkVarNames: m_nDirectLight (int)
            // NetworkVarNames: m_nBakedShadowIndex (int)
            // NetworkVarNames: m_nLuminaireShape (int)
            // NetworkVarNames: m_flLuminaireSize (float)
            // NetworkVarNames: m_flLuminaireAnisotropy (float)
            // NetworkVarNames: m_LightStyleString (CUtlString)
            // NetworkVarNames: m_flLightStyleStartTime (GameTime_t)
            // NetworkVarNames: m_QueuedLightStyleStrings (CUtlString)
            // NetworkVarNames: m_LightStyleEvents (CUtlString)
            // NetworkVarNames: m_LightStyleTargets (CHandle<CBaseModelEntity>)
            // NetworkVarNames: m_hLightCookie (HRenderTextureStrong)
            // NetworkVarNames: m_flShape (float)
            // NetworkVarNames: m_flSoftX (float)
            // NetworkVarNames: m_flSoftY (float)
            // NetworkVarNames: m_flSkirt (float)
            // NetworkVarNames: m_flSkirtNear (float)
            // NetworkVarNames: m_vSizeParams (Vector)
            // NetworkVarNames: m_flRange (float)
            // NetworkVarNames: m_vShear (Vector)
            // NetworkVarNames: m_nBakeSpecularToCubemaps (int)
            // NetworkVarNames: m_vBakeSpecularToCubemapsSize (Vector)
            // NetworkVarNames: m_nCastShadows (int)
            // NetworkVarNames: m_nShadowMapSize (int)
            // NetworkVarNames: m_nShadowPriority (int)
            // NetworkVarNames: m_bContactShadow (bool)
            // NetworkVarNames: m_nBounceLight (int)
            // NetworkVarNames: m_flBounceScale (float)
            // NetworkVarNames: m_flMinRoughness (float)
            // NetworkVarNames: m_vAlternateColor (Vector)
            // NetworkVarNames: m_fAlternateColorBrightness (float)
            // NetworkVarNames: m_nFog (int)
            // NetworkVarNames: m_flFogStrength (float)
            // NetworkVarNames: m_nFogShadows (int)
            // NetworkVarNames: m_flFogScale (float)
            // NetworkVarNames: m_flFadeSizeStart (float)
            // NetworkVarNames: m_flFadeSizeEnd (float)
            // NetworkVarNames: m_flShadowFadeSizeStart (float)
            // NetworkVarNames: m_flShadowFadeSizeEnd (float)
            // NetworkVarNames: m_bPrecomputedFieldsValid (bool)
            // NetworkVarNames: m_vPrecomputedBoundsMins (Vector)
            // NetworkVarNames: m_vPrecomputedBoundsMaxs (Vector)
            // NetworkVarNames: m_vPrecomputedOBBOrigin (Vector)
            // NetworkVarNames: m_vPrecomputedOBBAngles (QAngle)
            // NetworkVarNames: m_vPrecomputedOBBExtent (Vector)
            pub mod CBarnLight {
                pub const m_bEnabled: usize = 0x9F8; // bool
                pub const m_nColorMode: usize = 0x9FC; // int32
                pub const m_Color: usize = 0xA00; // Color
                pub const m_flColorTemperature: usize = 0xA04; // float32
                pub const m_flBrightness: usize = 0xA08; // float32
                pub const m_flBrightnessScale: usize = 0xA0C; // float32
                pub const m_nDirectLight: usize = 0xA10; // int32
                pub const m_nBakedShadowIndex: usize = 0xA14; // int32
                pub const m_nLuminaireShape: usize = 0xA18; // int32
                pub const m_flLuminaireSize: usize = 0xA1C; // float32
                pub const m_flLuminaireAnisotropy: usize = 0xA20; // float32
                pub const m_LightStyleString: usize = 0xA28; // CUtlString
                pub const m_flLightStyleStartTime: usize = 0xA30; // GameTime_t
                pub const m_QueuedLightStyleStrings: usize = 0xA38; // CNetworkUtlVectorBase<CUtlString>
                pub const m_LightStyleEvents: usize = 0xA50; // CNetworkUtlVectorBase<CUtlString>
                pub const m_LightStyleTargets: usize = 0xA68; // CNetworkUtlVectorBase<CHandle<CBaseModelEntity>>
                pub const m_StyleEvent: usize = 0xA80; // CEntityIOOutput[4]
                pub const m_hLightCookie: usize = 0xB40; // CStrongHandle<InfoForResourceTypeCTextureBase>
                pub const m_flShape: usize = 0xB48; // float32
                pub const m_flSoftX: usize = 0xB4C; // float32
                pub const m_flSoftY: usize = 0xB50; // float32
                pub const m_flSkirt: usize = 0xB54; // float32
                pub const m_flSkirtNear: usize = 0xB58; // float32
                pub const m_vSizeParams: usize = 0xB5C; // Vector
                pub const m_flRange: usize = 0xB68; // float32
                pub const m_vShear: usize = 0xB6C; // Vector
                pub const m_nBakeSpecularToCubemaps: usize = 0xB78; // int32
                pub const m_vBakeSpecularToCubemapsSize: usize = 0xB7C; // Vector
                pub const m_nCastShadows: usize = 0xB88; // int32
                pub const m_nShadowMapSize: usize = 0xB8C; // int32
                pub const m_nShadowPriority: usize = 0xB90; // int32
                pub const m_bContactShadow: usize = 0xB94; // bool
                pub const m_nBounceLight: usize = 0xB98; // int32
                pub const m_flBounceScale: usize = 0xB9C; // float32
                pub const m_flMinRoughness: usize = 0xBA0; // float32
                pub const m_vAlternateColor: usize = 0xBA4; // Vector
                pub const m_fAlternateColorBrightness: usize = 0xBB0; // float32
                pub const m_nFog: usize = 0xBB4; // int32
                pub const m_flFogStrength: usize = 0xBB8; // float32
                pub const m_nFogShadows: usize = 0xBBC; // int32
                pub const m_flFogScale: usize = 0xBC0; // float32
                pub const m_flFadeSizeStart: usize = 0xBC4; // float32
                pub const m_flFadeSizeEnd: usize = 0xBC8; // float32
                pub const m_flShadowFadeSizeStart: usize = 0xBCC; // float32
                pub const m_flShadowFadeSizeEnd: usize = 0xBD0; // float32
                pub const m_bPrecomputedFieldsValid: usize = 0xBD4; // bool
                pub const m_vPrecomputedBoundsMins: usize = 0xBD8; // Vector
                pub const m_vPrecomputedBoundsMaxs: usize = 0xBE4; // Vector
                pub const m_vPrecomputedOBBOrigin: usize = 0xBF0; // Vector
                pub const m_vPrecomputedOBBAngles: usize = 0xBFC; // QAngle
                pub const m_vPrecomputedOBBExtent: usize = 0xC08; // Vector
                pub const m_bPvsModifyEntity: usize = 0xC14; // bool
            }
            // Parent: CBarnLight
            // Fields count: 1
            //
            // Metadata:
            // NetworkVarNames: m_bShowLight (bool)
            pub mod CRectLight {
                pub const m_bShowLight: usize = 0xC20; // bool
            }
            // Parent: CBarnLight
            // Fields count: 3
            //
            // Metadata:
            // NetworkVarNames: m_flInnerAngle (float)
            // NetworkVarNames: m_flOuterAngle (float)
            // NetworkVarNames: m_bShowLight (bool)
            pub mod COmniLight {
                pub const m_flInnerAngle: usize = 0xC20; // float32
                pub const m_flOuterAngle: usize = 0xC24; // float32
                pub const m_bShowLight: usize = 0xC28; // bool
            }
            // Parent: CTeam
            // Fields count: 14
            //
            // Metadata:
            // NetworkVarNames: m_bSurrendered (bool)
            // NetworkVarNames: m_szTeamMatchStat (char)
            // NetworkVarNames: m_numMapVictories (int)
            // NetworkVarNames: m_scoreFirstHalf (int32)
            // NetworkVarNames: m_scoreSecondHalf (int32)
            // NetworkVarNames: m_scoreOvertime (int32)
            // NetworkVarNames: m_szClanTeamname (char)
            // NetworkVarNames: m_iClanID (uint32)
            // NetworkVarNames: m_szTeamFlagImage (char)
            // NetworkVarNames: m_szTeamLogoImage (char)
            pub mod CCSTeam {
                pub const m_nLastRecievedShorthandedRoundBonus: usize = 0x860; // int32
                pub const m_nShorthandedRoundBonusStartRound: usize = 0x864; // int32
                pub const m_bSurrendered: usize = 0x868; // bool
                pub const m_szTeamMatchStat: usize = 0x869; // char[512]
                pub const m_numMapVictories: usize = 0xA6C; // int32
                pub const m_scoreFirstHalf: usize = 0xA70; // int32
                pub const m_scoreSecondHalf: usize = 0xA74; // int32
                pub const m_scoreOvertime: usize = 0xA78; // int32
                pub const m_szClanTeamname: usize = 0xA7C; // char[129]
                pub const m_iClanID: usize = 0xB00; // uint32
                pub const m_szTeamFlagImage: usize = 0xB04; // char[8]
                pub const m_szTeamLogoImage: usize = 0xB0C; // char[8]
                pub const m_flNextResourceTime: usize = 0xB14; // float32
                pub const m_iLastUpdateSentAt: usize = 0xB18; // int32
            }
            // Parent: CBaseModelEntity
            // Fields count: 12
            //
            // Metadata:
            // NetworkVarNames: m_hSkyMaterial (HMaterialStrong)
            // NetworkVarNames: m_hSkyMaterialLightingOnly (HMaterialStrong)
            // NetworkVarNames: m_bStartDisabled (bool)
            // NetworkVarNames: m_vTintColor (Color)
            // NetworkVarNames: m_vTintColorLightingOnly (Color)
            // NetworkVarNames: m_flBrightnessScale (float)
            // NetworkVarNames: m_nFogType (int)
            // NetworkVarNames: m_flFogMinStart (float)
            // NetworkVarNames: m_flFogMinEnd (float)
            // NetworkVarNames: m_flFogMaxStart (float)
            // NetworkVarNames: m_flFogMaxEnd (float)
            // NetworkVarNames: m_bEnabled (bool)
            pub mod CEnvSky {
                pub const m_hSkyMaterial: usize = 0x9F8; // CStrongHandle<InfoForResourceTypeIMaterial2>
                pub const m_hSkyMaterialLightingOnly: usize = 0xA00; // CStrongHandle<InfoForResourceTypeIMaterial2>
                pub const m_bStartDisabled: usize = 0xA08; // bool
                pub const m_vTintColor: usize = 0xA09; // Color
                pub const m_vTintColorLightingOnly: usize = 0xA0D; // Color
                pub const m_flBrightnessScale: usize = 0xA14; // float32
                pub const m_nFogType: usize = 0xA18; // int32
                pub const m_flFogMinStart: usize = 0xA1C; // float32
                pub const m_flFogMinEnd: usize = 0xA20; // float32
                pub const m_flFogMaxStart: usize = 0xA24; // float32
                pub const m_flFogMaxEnd: usize = 0xA28; // float32
                pub const m_bEnabled: usize = 0xA2C; // bool
            }
            // Parent: CBaseModelEntity
            // Fields count: 1
            //
            // Metadata:
            // NetworkVarNames: m_CLightComponent (CLightComponent::Storage_t)
            pub mod CLightEntity {
                pub const m_CLightComponent: usize = 0x9F8; // CLightComponent*
            }
            // Parent: CBaseTrigger
            // Fields count: 16
            //
            // Metadata:
            // NetworkVarNames: m_hPostSettings (HPostProcessingStrong)
            // NetworkVarNames: m_flFadeDuration (float)
            // NetworkVarNames: m_flMinLogExposure (float)
            // NetworkVarNames: m_flMaxLogExposure (float)
            // NetworkVarNames: m_flMinExposure (float)
            // NetworkVarNames: m_flMaxExposure (float)
            // NetworkVarNames: m_flExposureCompensation (float)
            // NetworkVarNames: m_flExposureFadeSpeedUp (float)
            // NetworkVarNames: m_flExposureFadeSpeedDown (float)
            // NetworkVarNames: m_flTonemapEVSmoothingRange (float)
            // NetworkVarNames: m_bMaster (bool)
            // NetworkVarNames: m_bExposureControl (bool)
            // NetworkVarNames: m_flRate (float)
            // NetworkVarNames: m_flTonemapPercentTarget (float)
            // NetworkVarNames: m_flTonemapPercentBrightPixels (float)
            // NetworkVarNames: m_flTonemapMinAvgLum (float)
            pub mod CPostProcessingVolume {
                pub const m_hPostSettings: usize = 0xBB0; // CStrongHandle<InfoForResourceTypeCPostProcessingResource>
                pub const m_flFadeDuration: usize = 0xBB8; // float32
                pub const m_flMinLogExposure: usize = 0xBBC; // float32
                pub const m_flMaxLogExposure: usize = 0xBC0; // float32
                pub const m_flMinExposure: usize = 0xBC4; // float32
                pub const m_flMaxExposure: usize = 0xBC8; // float32
                pub const m_flExposureCompensation: usize = 0xBCC; // float32
                pub const m_flExposureFadeSpeedUp: usize = 0xBD0; // float32
                pub const m_flExposureFadeSpeedDown: usize = 0xBD4; // float32
                pub const m_flTonemapEVSmoothingRange: usize = 0xBD8; // float32
                pub const m_bMaster: usize = 0xBDC; // bool
                pub const m_bExposureControl: usize = 0xBDD; // bool
                pub const m_flRate: usize = 0xBE0; // float32
                pub const m_flTonemapPercentTarget: usize = 0xBE4; // float32
                pub const m_flTonemapPercentBrightPixels: usize = 0xBE8; // float32
                pub const m_flTonemapMinAvgLum: usize = 0xBEC; // float32
            }
            // Parent: CParticleSystem
            // Fields count: 5
            //
            // Metadata:
            // NetworkVarNames: m_flAlphaScale (float32)
            // NetworkVarNames: m_flRadiusScale (float32)
            // NetworkVarNames: m_flSelfIllumScale (float32)
            // NetworkVarNames: m_ColorTint (Color)
            // NetworkVarNames: m_hTextureOverride (HRenderTextureStrong)
            pub mod CEnvParticleGlow {
                pub const m_flAlphaScale: usize = 0xF70; // float32
                pub const m_flRadiusScale: usize = 0xF74; // float32
                pub const m_flSelfIllumScale: usize = 0xF78; // float32
                pub const m_ColorTint: usize = 0xF7C; // Color
                pub const m_hTextureOverride: usize = 0xF80; // CStrongHandle<InfoForResourceTypeCTextureBase>
            }
            // Parent: CBaseModelEntity
            // Fields count: 8
            //
            // Metadata:
            // NetworkVarNames: m_bLoop (bool)
            // NetworkVarNames: m_flFPS (float)
            // NetworkVarNames: m_hPositionKeys (HRenderTextureStrong)
            // NetworkVarNames: m_hRotationKeys (HRenderTextureStrong)
            // NetworkVarNames: m_vAnimationBoundsMin (Vector)
            // NetworkVarNames: m_vAnimationBoundsMax (Vector)
            // NetworkVarNames: m_flStartTime (float)
            // NetworkVarNames: m_flStartFrame (float)
            pub mod CTextureBasedAnimatable {
                pub const m_bLoop: usize = 0x9F8; // bool
                pub const m_flFPS: usize = 0x9FC; // float32
                pub const m_hPositionKeys: usize = 0xA00; // CStrongHandle<InfoForResourceTypeCTextureBase>
                pub const m_hRotationKeys: usize = 0xA08; // CStrongHandle<InfoForResourceTypeCTextureBase>
                pub const m_vAnimationBoundsMin: usize = 0xA10; // Vector
                pub const m_vAnimationBoundsMax: usize = 0xA1C; // Vector
                pub const m_flStartTime: usize = 0xA28; // float32
                pub const m_flStartFrame: usize = 0xA2C; // float32
            }
            // Parent: CBaseModelEntity
            // Fields count: 10
            //
            // Metadata:
            // MNetworkIncludeByName
            // NetworkVarNames: m_bInitiallyPopulateInterpHistory (bool)
            // NetworkVarNames: m_bAnimGraphUpdateEnabled (bool)
            // NetworkVarNames: m_vecForce (Vector)
            // NetworkVarNames: m_nForceBone (int32)
            // NetworkVarNames: m_pRagdollPose (PhysicsRagdollPose_t*)
            // NetworkVarNames: m_bClientRagdoll (bool)
            pub mod CBaseAnimGraph {
                pub const m_bInitiallyPopulateInterpHistory: usize = 0xA70; // bool
                pub const m_pChoreoServices: usize = 0xA78; // IChoreoServices*
                pub const m_bAnimGraphUpdateEnabled: usize = 0xA80; // bool
                pub const m_flMaxSlopeDistance: usize = 0xA84; // float32
                pub const m_vLastSlopeCheckPos: usize = 0xA88; // Vector
                pub const m_bAnimationUpdateScheduled: usize = 0xA94; // bool
                pub const m_vecForce: usize = 0xA98; // Vector
                pub const m_nForceBone: usize = 0xAA4; // int32
                pub const m_pRagdollPose: usize = 0xAB8; // PhysicsRagdollPose_t*
                pub const m_bClientRagdoll: usize = 0xAC0; // bool
            }
            // Parent: None
            // Fields count: 34
            //
            // Metadata:
            // NetworkVarNames: m_noGhostCollision (bool)
            pub mod CBreakableProp {
                pub const m_OnBreak: usize = 0xC40; // CEntityIOOutput
                pub const m_OnHealthChanged: usize = 0xC68; // CEntityOutputTemplate<float32>
                pub const m_OnTakeDamage: usize = 0xC90; // CEntityIOOutput
                pub const m_impactEnergyScale: usize = 0xCB8; // float32
                pub const m_iMinHealthDmg: usize = 0xCBC; // int32
                pub const m_preferredCarryAngles: usize = 0xCC0; // QAngle
                pub const m_flPressureDelay: usize = 0xCCC; // float32
                pub const m_hBreaker: usize = 0xCD0; // CHandle<CBaseEntity>
                pub const m_PerformanceMode: usize = 0xCD4; // PerformanceMode_t
                pub const m_flDmgModBullet: usize = 0xCD8; // float32
                pub const m_flDmgModClub: usize = 0xCDC; // float32
                pub const m_flDmgModExplosive: usize = 0xCE0; // float32
                pub const m_flDmgModFire: usize = 0xCE4; // float32
                pub const m_iszPhysicsDamageTableName: usize = 0xCE8; // CUtlSymbolLarge
                pub const m_iszBasePropData: usize = 0xCF0; // CUtlSymbolLarge
                pub const m_iInteractions: usize = 0xCF8; // int32
                pub const m_flPreventDamageBeforeTime: usize = 0xCFC; // GameTime_t
                pub const m_bHasBreakPiecesOrCommands: usize = 0xD00; // bool
                pub const m_explodeDamage: usize = 0xD04; // float32
                pub const m_explodeRadius: usize = 0xD08; // float32
                pub const m_explosionDelay: usize = 0xD10; // float32
                pub const m_explosionBuildupSound: usize = 0xD18; // CUtlSymbolLarge
                pub const m_explosionCustomEffect: usize = 0xD20; // CUtlSymbolLarge
                pub const m_explosionCustomSound: usize = 0xD28; // CUtlSymbolLarge
                pub const m_explosionModifier: usize = 0xD30; // CUtlSymbolLarge
                pub const m_hPhysicsAttacker: usize = 0xD38; // CHandle<CBasePlayerPawn>
                pub const m_flLastPhysicsInfluenceTime: usize = 0xD3C; // GameTime_t
                pub const m_bOriginalBlockLOS: usize = 0xD40; // bool
                pub const m_flDefaultFadeScale: usize = 0xD44; // float32
                pub const m_hLastAttacker: usize = 0xD48; // CHandle<CBaseEntity>
                pub const m_hFlareEnt: usize = 0xD4C; // CHandle<CBaseEntity>
                pub const m_bUsePuntSound: usize = 0xD50; // bool
                pub const m_iszPuntSound: usize = 0xD58; // CUtlSymbolLarge
                pub const m_noGhostCollision: usize = 0xD60; // bool
            }
            // Parent: CBreakableProp
            // Fields count: 21
            //
            // Metadata:
            // NetworkVarNames: m_bUseHitboxesForRenderBox (bool)
            // NetworkVarNames: m_bUseAnimGraph (bool)
            pub mod CDynamicProp {
                pub const m_bCreateNavObstacle: usize = 0xD70; // bool
                pub const m_bUseHitboxesForRenderBox: usize = 0xD71; // bool
                pub const m_bUseAnimGraph: usize = 0xD72; // bool
                pub const m_pOutputAnimBegun: usize = 0xD78; // CEntityIOOutput
                pub const m_pOutputAnimOver: usize = 0xDA0; // CEntityIOOutput
                pub const m_pOutputAnimLoopCycleOver: usize = 0xDC8; // CEntityIOOutput
                pub const m_OnAnimReachedStart: usize = 0xDF0; // CEntityIOOutput
                pub const m_OnAnimReachedEnd: usize = 0xE18; // CEntityIOOutput
                pub const m_iszIdleAnim: usize = 0xE40; // CUtlSymbolLarge
                pub const m_nIdleAnimLoopMode: usize = 0xE48; // AnimLoopMode_t
                pub const m_bRandomizeCycle: usize = 0xE4C; // bool
                pub const m_bStartDisabled: usize = 0xE4D; // bool
                pub const m_bFiredStartEndOutput: usize = 0xE4E; // bool
                pub const m_bForceNpcExclude: usize = 0xE4F; // bool
                pub const m_bCreateNonSolid: usize = 0xE50; // bool
                pub const m_bIsOverrideProp: usize = 0xE51; // bool
                pub const m_iInitialGlowState: usize = 0xE54; // int32
                pub const m_nGlowRange: usize = 0xE58; // int32
                pub const m_nGlowRangeMin: usize = 0xE5C; // int32
                pub const m_glowColor: usize = 0xE60; // Color
                pub const m_nGlowTeam: usize = 0xE64; // int32
            }
            // Parent: CBaseTrigger
            // Fields count: 10
            //
            // Metadata:
            // NetworkVarNames: m_bEnabled (bool)
            // NetworkVarNames: m_MaxWeight (float32)
            // NetworkVarNames: m_FadeDuration (float32)
            // NetworkVarNames: m_Weight (float32)
            // NetworkVarNames: m_lookupFilename (char)
            pub mod CColorCorrectionVolume {
                pub const m_bEnabled: usize = 0xB99; // bool
                pub const m_MaxWeight: usize = 0xB9C; // float32
                pub const m_FadeDuration: usize = 0xBA0; // float32
                pub const m_bStartDisabled: usize = 0xBA4; // bool
                pub const m_Weight: usize = 0xBA8; // float32
                pub const m_lookupFilename: usize = 0xBAC; // char[512]
                pub const m_LastEnterWeight: usize = 0xDAC; // float32
                pub const m_LastEnterTime: usize = 0xDB0; // GameTime_t
                pub const m_LastExitWeight: usize = 0xDB4; // float32
                pub const m_LastExitTime: usize = 0xDB8; // GameTime_t
            }
            // Parent: CBaseAnimGraph
            // Fields count: 30
            //
            // Metadata:
            // NetworkVarNames: m_iszCommentaryFile (string_t)
            // NetworkVarNames: m_hViewPosition (CHandle<CBaseEntity>)
            // NetworkVarNames: m_bActive (bool)
            // NetworkVarNames: m_flStartTime (GameTime_t)
            // NetworkVarNames: m_flStartTimeInCommentary (float32)
            // NetworkVarNames: m_iszTitle (string_t)
            // NetworkVarNames: m_iszSpeakers (string_t)
            // NetworkVarNames: m_iNodeNumber (int)
            // NetworkVarNames: m_iNodeNumberMax (int)
            // NetworkVarNames: m_bListenedTo (bool)
            pub mod CPointCommentaryNode {
                pub const m_iszPreCommands: usize = 0xBF0; // CUtlSymbolLarge
                pub const m_iszPostCommands: usize = 0xBF8; // CUtlSymbolLarge
                pub const m_iszCommentaryFile: usize = 0xC00; // CUtlSymbolLarge
                pub const m_iszViewTarget: usize = 0xC08; // CUtlSymbolLarge
                pub const m_hViewTarget: usize = 0xC10; // CHandle<CBaseEntity>
                pub const m_hViewTargetAngles: usize = 0xC14; // CHandle<CBaseEntity>
                pub const m_iszViewPosition: usize = 0xC18; // CUtlSymbolLarge
                pub const m_hViewPosition: usize = 0xC20; // CHandle<CBaseEntity>
                pub const m_hViewPositionMover: usize = 0xC24; // CHandle<CBaseEntity>
                pub const m_bPreventMovement: usize = 0xC28; // bool
                pub const m_bUnderCrosshair: usize = 0xC29; // bool
                pub const m_bUnstoppable: usize = 0xC2A; // bool
                pub const m_flFinishedTime: usize = 0xC2C; // GameTime_t
                pub const m_vecFinishOrigin: usize = 0xC30; // Vector
                pub const m_vecOriginalAngles: usize = 0xC3C; // QAngle
                pub const m_vecFinishAngles: usize = 0xC48; // QAngle
                pub const m_bPreventChangesWhileMoving: usize = 0xC54; // bool
                pub const m_bDisabled: usize = 0xC55; // bool
                pub const m_vecTeleportOrigin: usize = 0xC58; // Vector
                pub const m_flAbortedPlaybackAt: usize = 0xC64; // GameTime_t
                pub const m_pOnCommentaryStarted: usize = 0xC68; // CEntityIOOutput
                pub const m_pOnCommentaryStopped: usize = 0xC90; // CEntityIOOutput
                pub const m_bActive: usize = 0xCB8; // bool
                pub const m_flStartTime: usize = 0xCBC; // GameTime_t
                pub const m_flStartTimeInCommentary: usize = 0xCC0; // float32
                pub const m_iszTitle: usize = 0xCC8; // CUtlSymbolLarge
                pub const m_iszSpeakers: usize = 0xCD0; // CUtlSymbolLarge
                pub const m_iNodeNumber: usize = 0xCD8; // int32
                pub const m_iNodeNumberMax: usize = 0xCDC; // int32
                pub const m_bListenedTo: usize = 0xCE0; // bool
            }
            // Parent: None
            // Fields count: 9
            //
            // Metadata:
            // NetworkVarNames: m_targetCamera (CUtlString)
            // NetworkVarNames: m_nResolutionEnum (int)
            // NetworkVarNames: m_bRenderShadows (bool)
            // NetworkVarNames: m_bUseUniqueColorTarget (bool)
            // NetworkVarNames: m_brushModelName (CUtlString)
            // NetworkVarNames: m_hTargetCamera (EHANDLE)
            // NetworkVarNames: m_bEnabled (bool)
            // NetworkVarNames: m_bDraw3DSkybox (bool)
            pub mod CFuncMonitor {
                pub const m_targetCamera: usize = 0xA18; // CUtlString
                pub const m_nResolutionEnum: usize = 0xA20; // int32
                pub const m_bRenderShadows: usize = 0xA24; // bool
                pub const m_bUseUniqueColorTarget: usize = 0xA25; // bool
                pub const m_brushModelName: usize = 0xA28; // CUtlString
                pub const m_hTargetCamera: usize = 0xA30; // CHandle<CBaseEntity>
                pub const m_bEnabled: usize = 0xA34; // bool
                pub const m_bDraw3DSkybox: usize = 0xA35; // bool
                pub const m_bStartEnabled: usize = 0xA36; // bool
            }
            // Parent: CBaseTrigger
            // Fields count: 3
            //
            // Metadata:
            // NetworkVarNames: m_strStartTouchEventName (CUtlString)
            // NetworkVarNames: m_strEndTouchEventName (CUtlString)
            // NetworkVarNames: m_strTriggerID (CUtlString)
            pub mod CTriggerGameEvent {
                pub const m_strStartTouchEventName: usize = 0xBA0; // CUtlString
                pub const m_strEndTouchEventName: usize = 0xBA8; // CUtlString
                pub const m_strTriggerID: usize = 0xBB0; // CUtlString
            }
            // Parent: None
            // Fields count: 4
            //
            // Metadata:
            // NetworkVarNames: m_EffectName (string_t)
            pub mod CFuncElectrifiedVolume {
                pub const m_EffectName: usize = 0xA18; // CUtlSymbolLarge
                pub const m_EffectInterpenetrateName: usize = 0xA20; // CUtlSymbolLarge
                pub const m_EffectZapName: usize = 0xA28; // CUtlSymbolLarge
                pub const m_iszEffectSource: usize = 0xA30; // CUtlSymbolLarge
            }
            // Parent: None
            // Fields count: 15
            //
            // Metadata:
            // NetworkVarNames: m_bTestOcclusion (bool)
            pub mod CTriggerLook {
                pub const m_hLookTarget: usize = 0xBC8; // CHandle<CBaseEntity>
                pub const m_flFieldOfView: usize = 0xBCC; // float32
                pub const m_flLookTime: usize = 0xBD0; // float32
                pub const m_flLookTimeTotal: usize = 0xBD4; // float32
                pub const m_flLookTimeLast: usize = 0xBD8; // GameTime_t
                pub const m_flTimeoutDuration: usize = 0xBDC; // float32
                pub const m_bTimeoutFired: usize = 0xBE0; // bool
                pub const m_bIsLooking: usize = 0xBE1; // bool
                pub const m_b2DFOV: usize = 0xBE2; // bool
                pub const m_bUseVelocity: usize = 0xBE3; // bool
                pub const m_hActivator: usize = 0xBE4; // CHandle<CBaseEntity>
                pub const m_bTestOcclusion: usize = 0xBE8; // bool
                pub const m_OnTimeout: usize = 0xBF0; // CEntityIOOutput
                pub const m_OnStartLook: usize = 0xC18; // CEntityIOOutput
                pub const m_OnEndLook: usize = 0xC40; // CEntityIOOutput
            }
            // Parent: CBaseTrigger
            // Fields count: 13
            //
            // Metadata:
            // NetworkVarNames: m_gravityScale (float)
            // NetworkVarNames: m_linearLimit (float)
            // NetworkVarNames: m_linearDamping (float)
            // NetworkVarNames: m_angularLimit (float)
            // NetworkVarNames: m_angularDamping (float)
            // NetworkVarNames: m_linearForce (float)
            // NetworkVarNames: m_flFrequency (float)
            // NetworkVarNames: m_flDampingRatio (float)
            // NetworkVarNames: m_vecLinearForcePointAt (Vector)
            // NetworkVarNames: m_bCollapseToForcePoint (bool)
            // NetworkVarNames: m_vecLinearForcePointAtWorld (Vector)
            // NetworkVarNames: m_vecLinearForceDirection (Vector)
            // NetworkVarNames: m_bConvertToDebrisWhenPossible (bool)
            pub mod CTriggerPhysics {
                pub const m_gravityScale: usize = 0xBB0; // float32
                pub const m_linearLimit: usize = 0xBB4; // float32
                pub const m_linearDamping: usize = 0xBB8; // float32
                pub const m_angularLimit: usize = 0xBBC; // float32
                pub const m_angularDamping: usize = 0xBC0; // float32
                pub const m_linearForce: usize = 0xBC4; // float32
                pub const m_flFrequency: usize = 0xBC8; // float32
                pub const m_flDampingRatio: usize = 0xBCC; // float32
                pub const m_vecLinearForcePointAt: usize = 0xBD0; // Vector
                pub const m_bCollapseToForcePoint: usize = 0xBDC; // bool
                pub const m_vecLinearForcePointAtWorld: usize = 0xBE0; // Vector
                pub const m_vecLinearForceDirection: usize = 0xBEC; // Vector
                pub const m_bConvertToDebrisWhenPossible: usize = 0xBF8; // bool
            }
            // Parent: CBaseAnimGraph
            // Fields count: 7
            //
            // Metadata:
            // NetworkVarNames: m_flexWeight (float32)
            // NetworkVarNames: m_vLookTargetPosition (Vector)
            // NetworkVarNames: m_blinktoggle (bool)
            pub mod CBaseFlex {
                pub const m_flexWeight: usize = 0xBF0; // CNetworkUtlVectorBase<float32>
                pub const m_vLookTargetPosition: usize = 0xC08; // Vector
                pub const m_blinktoggle: usize = 0xC14; // bool
                pub const m_flAllowResponsesEndTime: usize = 0xC68; // GameTime_t
                pub const m_flLastFlexAnimationTime: usize = 0xC6C; // GameTime_t
                pub const m_nNextSceneEventId: usize = 0xC70; // uint32
                pub const m_bUpdateLayerPriorities: usize = 0xC74; // bool
            }
            // Parent: CDynamicProp
            // Fields count: 37
            //
            // Metadata:
            // MNetworkExcludeByName
            // MNetworkExcludeByName
            // MNetworkExcludeByName
            // MNetworkExcludeByUserGroup
            // MNetworkExcludeByName
            // MNetworkExcludeByUserGroup
            // MNetworkIncludeByName
            // NetworkVarNames: m_eDoorState (DoorState_t)
            // NetworkVarNames: m_bLocked (bool)
            // NetworkVarNames: m_closedPosition (Vector)
            // NetworkVarNames: m_closedAngles (QAngle)
            // NetworkVarNames: m_hMaster (CHandle<CBasePropDoor>)
            pub mod CBasePropDoor {
                pub const m_flAutoReturnDelay: usize = 0xE74; // float32
                pub const m_hDoorList: usize = 0xE78; // CUtlVector<CHandle<CBasePropDoor>>
                pub const m_nHardwareType: usize = 0xE90; // int32
                pub const m_bNeedsHardware: usize = 0xE94; // bool
                pub const m_eDoorState: usize = 0xE98; // DoorState_t
                pub const m_bLocked: usize = 0xE9C; // bool
                pub const m_closedPosition: usize = 0xEA0; // Vector
                pub const m_closedAngles: usize = 0xEAC; // QAngle
                pub const m_hBlocker: usize = 0xEB8; // CHandle<CBaseEntity>
                pub const m_bFirstBlocked: usize = 0xEBC; // bool
                pub const m_ls: usize = 0xEC0; // locksound_t
                pub const m_bForceClosed: usize = 0xEE0; // bool
                pub const m_vecLatchWorldPosition: usize = 0xEE4; // Vector
                pub const m_hActivator: usize = 0xEF0; // CHandle<CBaseEntity>
                pub const m_SoundMoving: usize = 0xF00; // CUtlSymbolLarge
                pub const m_SoundOpen: usize = 0xF08; // CUtlSymbolLarge
                pub const m_SoundClose: usize = 0xF10; // CUtlSymbolLarge
                pub const m_SoundLock: usize = 0xF18; // CUtlSymbolLarge
                pub const m_SoundUnlock: usize = 0xF20; // CUtlSymbolLarge
                pub const m_SoundLatch: usize = 0xF28; // CUtlSymbolLarge
                pub const m_SoundPound: usize = 0xF30; // CUtlSymbolLarge
                pub const m_SoundJiggle: usize = 0xF38; // CUtlSymbolLarge
                pub const m_SoundLockedAnim: usize = 0xF40; // CUtlSymbolLarge
                pub const m_numCloseAttempts: usize = 0xF48; // int32
                pub const m_nPhysicsMaterial: usize = 0xF4C; // CUtlStringToken
                pub const m_SlaveName: usize = 0xF50; // CUtlSymbolLarge
                pub const m_hMaster: usize = 0xF58; // CHandle<CBasePropDoor>
                pub const m_OnBlockedClosing: usize = 0xF60; // CEntityIOOutput
                pub const m_OnBlockedOpening: usize = 0xF88; // CEntityIOOutput
                pub const m_OnUnblockedClosing: usize = 0xFB0; // CEntityIOOutput
                pub const m_OnUnblockedOpening: usize = 0xFD8; // CEntityIOOutput
                pub const m_OnFullyClosed: usize = 0x1000; // CEntityIOOutput
                pub const m_OnFullyOpen: usize = 0x1028; // CEntityIOOutput
                pub const m_OnClose: usize = 0x1050; // CEntityIOOutput
                pub const m_OnOpen: usize = 0x1078; // CEntityIOOutput
                pub const m_OnLockedUse: usize = 0x10A0; // CEntityIOOutput
                pub const m_OnAjarOpen: usize = 0x10C8; // CEntityIOOutput
            }
            // Parent: CBaseAnimGraph
            // Fields count: 24
            //
            // Metadata:
            // MNetworkIncludeByName
            // MNetworkIncludeByName
            // NetworkVarNames: m_x (float32)
            // NetworkVarNames: m_y (float32)
            // NetworkVarNames: m_z (float32)
            // NetworkVarNames: m_angle (float32)
            // NetworkVarNames: m_poolOrigin (Vector)
            // NetworkVarNames: m_waterLevel (float32)
            pub mod CFish {
                pub const m_pool: usize = 0xBF0; // CHandle<CFishPool>
                pub const m_id: usize = 0xBF4; // uint32
                pub const m_x: usize = 0xBF8; // float32
                pub const m_y: usize = 0xBFC; // float32
                pub const m_z: usize = 0xC00; // float32
                pub const m_angle: usize = 0xC04; // float32
                pub const m_angleChange: usize = 0xC08; // float32
                pub const m_forward: usize = 0xC0C; // Vector
                pub const m_perp: usize = 0xC18; // Vector
                pub const m_poolOrigin: usize = 0xC24; // Vector
                pub const m_waterLevel: usize = 0xC30; // float32
                pub const m_speed: usize = 0xC34; // float32
                pub const m_desiredSpeed: usize = 0xC38; // float32
                pub const m_calmSpeed: usize = 0xC3C; // float32
                pub const m_panicSpeed: usize = 0xC40; // float32
                pub const m_avoidRange: usize = 0xC44; // float32
                pub const m_turnTimer: usize = 0xC48; // CountdownTimer
                pub const m_turnClockwise: usize = 0xC60; // bool
                pub const m_goTimer: usize = 0xC68; // CountdownTimer
                pub const m_moveTimer: usize = 0xC80; // CountdownTimer
                pub const m_panicTimer: usize = 0xC98; // CountdownTimer
                pub const m_disperseTimer: usize = 0xCB0; // CountdownTimer
                pub const m_proximityTimer: usize = 0xCC8; // CountdownTimer
                pub const m_visible: usize = 0xCE0; // CUtlVector<CFish*>
            }
            // Parent: CBaseAnimGraph
            // Fields count: 28
            //
            // Metadata:
            // NetworkVarNames: m_ragPos (Vector)
            // NetworkVarNames: m_ragAngles (QAngle)
            // NetworkVarNames: m_hRagdollSource (EHANDLE)
            // NetworkVarNames: m_flBlendWeight (float32)
            pub mod CRagdollProp {
                pub const m_ragdoll: usize = 0xBF8; // ragdoll_t
                pub const m_bStartDisabled: usize = 0xC30; // bool
                pub const m_ragPos: usize = 0xC38; // CNetworkUtlVectorBase<Vector>
                pub const m_ragAngles: usize = 0xC50; // CNetworkUtlVectorBase<QAngle>
                pub const m_hRagdollSource: usize = 0xC68; // CHandle<CBaseEntity>
                pub const m_lastUpdateTickCount: usize = 0xC6C; // uint32
                pub const m_allAsleep: usize = 0xC70; // bool
                pub const m_bFirstCollisionAfterLaunch: usize = 0xC71; // bool
                pub const m_hDamageEntity: usize = 0xC74; // CHandle<CBaseEntity>
                pub const m_hKiller: usize = 0xC78; // CHandle<CBaseEntity>
                pub const m_hPhysicsAttacker: usize = 0xC7C; // CHandle<CBasePlayerPawn>
                pub const m_flLastPhysicsInfluenceTime: usize = 0xC80; // GameTime_t
                pub const m_flFadeOutStartTime: usize = 0xC84; // GameTime_t
                pub const m_flFadeTime: usize = 0xC88; // float32
                pub const m_vecLastOrigin: usize = 0xC8C; // Vector
                pub const m_flAwakeTime: usize = 0xC98; // GameTime_t
                pub const m_flLastOriginChangeTime: usize = 0xC9C; // GameTime_t
                pub const m_nBloodColor: usize = 0xCA0; // int32
                pub const m_strOriginClassName: usize = 0xCA8; // CUtlSymbolLarge
                pub const m_strSourceClassName: usize = 0xCB0; // CUtlSymbolLarge
                pub const m_bHasBeenPhysgunned: usize = 0xCB8; // bool
                pub const m_bShouldTeleportPhysics: usize = 0xCB9; // bool
                pub const m_flBlendWeight: usize = 0xCBC; // float32
                pub const m_flDefaultFadeScale: usize = 0xCC0; // float32
                pub const m_ragdollMins: usize = 0xCC8; // CUtlVector<Vector>
                pub const m_ragdollMaxs: usize = 0xCE0; // CUtlVector<Vector>
                pub const m_bShouldDeleteActivationRecord: usize = 0xCF8; // bool
                pub const m_bValidatePoweredRagdollPose: usize = 0xD58; // bool
            }
            // Parent: CBreakableProp
            // Fields count: 34
            //
            // Metadata:
            // MNetworkExcludeByName
            // MNetworkExcludeByName
            // MNetworkExcludeByName
            // MNetworkExcludeByUserGroup
            // MNetworkExcludeByName
            // MNetworkExcludeByUserGroup
            // MNetworkIncludeByName
            // NetworkVarNames: m_bAwake (bool)
            pub mod CPhysicsProp {
                pub const m_MotionEnabled: usize = 0xD70; // CEntityIOOutput
                pub const m_OnAwakened: usize = 0xD98; // CEntityIOOutput
                pub const m_OnAwake: usize = 0xDC0; // CEntityIOOutput
                pub const m_OnAsleep: usize = 0xDE8; // CEntityIOOutput
                pub const m_OnPlayerUse: usize = 0xE10; // CEntityIOOutput
                pub const m_OnPlayerPickup: usize = 0xE38; // CEntityIOOutput
                pub const m_OnOutOfWorld: usize = 0xE60; // CEntityIOOutput
                pub const m_massScale: usize = 0xE88; // float32
                pub const m_inertiaScale: usize = 0xE8C; // float32
                pub const m_buoyancyScale: usize = 0xE90; // float32
                pub const m_damageType: usize = 0xE94; // int32
                pub const m_damageToEnableMotion: usize = 0xE98; // int32
                pub const m_flForceToEnableMotion: usize = 0xE9C; // float32
                pub const m_bThrownByPlayer: usize = 0xEA0; // bool
                pub const m_bDroppedByPlayer: usize = 0xEA1; // bool
                pub const m_bTouchedByPlayer: usize = 0xEA2; // bool
                pub const m_bFirstCollisionAfterLaunch: usize = 0xEA3; // bool
                pub const m_iExploitableByPlayer: usize = 0xEA4; // int32
                pub const m_bHasBeenAwakened: usize = 0xEA8; // bool
                pub const m_bIsOverrideProp: usize = 0xEA9; // bool
                pub const m_fNextCheckDisableMotionContactsTime: usize = 0xEAC; // GameTime_t
                pub const m_iInitialGlowState: usize = 0xEB0; // int32
                pub const m_nGlowRange: usize = 0xEB4; // int32
                pub const m_nGlowRangeMin: usize = 0xEB8; // int32
                pub const m_glowColor: usize = 0xEBC; // Color
                pub const m_bForceNavIgnore: usize = 0xEC0; // bool
                pub const m_bNoNavmeshBlocker: usize = 0xEC1; // bool
                pub const m_bForceNpcExclude: usize = 0xEC2; // bool
                pub const m_bShouldAutoConvertBackFromDebris: usize = 0xEC3; // bool
                pub const m_bMuteImpactEffects: usize = 0xEC4; // bool
                pub const m_bAcceptDamageFromHeldObjects: usize = 0xECC; // bool
                pub const m_bEnableUseOutput: usize = 0xECD; // bool
                pub const m_bAwake: usize = 0xECE; // bool
                pub const m_nCollisionGroupOverride: usize = 0xED0; // int32
            }
            // Parent: CPhysicsProp
            // Fields count: 3
            //
            // Metadata:
            // MNetworkExcludeByName
            // NetworkVarNames: m_ShardDesc (shard_model_desc_t)
            pub mod CShatterGlassShardPhysics {
                pub const m_bDebris: usize = 0xED4; // bool
                pub const m_hParentShard: usize = 0xED8; // uint32
                pub const m_ShardDesc: usize = 0xEE0; // shard_model_desc_t
            }
            // Parent: CBaseFlex
            // Fields count: 9
            //
            // Metadata:
            // NetworkVarNames: m_AttributeManager (CAttributeContainer)
            // NetworkVarNames: m_OriginalOwnerXuidLow (uint32)
            // NetworkVarNames: m_OriginalOwnerXuidHigh (uint32)
            // NetworkVarNames: m_nFallbackPaintKit (int)
            // NetworkVarNames: m_nFallbackSeed (int)
            // NetworkVarNames: m_flFallbackWear (float)
            // NetworkVarNames: m_nFallbackStatTrak (int)
            pub mod CEconEntity {
                pub const m_AttributeManager: usize = 0xC90; // CAttributeContainer
                pub const m_OriginalOwnerXuidLow: usize = 0xF58; // uint32
                pub const m_OriginalOwnerXuidHigh: usize = 0xF5C; // uint32
                pub const m_nFallbackPaintKit: usize = 0xF60; // int32
                pub const m_nFallbackSeed: usize = 0xF64; // int32
                pub const m_flFallbackWear: usize = 0xF68; // float32
                pub const m_nFallbackStatTrak: usize = 0xF6C; // int32
                pub const m_hOldProvidee: usize = 0xF70; // CHandle<CBaseEntity>
                pub const m_iOldOwnerClass: usize = 0xF74; // int32
            }
            // Parent: CBaseFlex
            // Fields count: 14
            //
            // Metadata:
            // MNetworkIncludeByName
            // MNetworkIncludeByName
            // MNetworkExcludeByName
            // MNetworkExcludeByName
            // MNetworkExcludeByUserGroup
            // MNetworkExcludeByName
            // MNetworkExcludeByUserGroup
            // MNetworkExcludeByName
            // MNetworkExcludeByUserGroup
            // MNetworkExcludeByName
            // MNetworkExcludeByName
            // MNetworkExcludeByName
            // NetworkVarNames: m_bIsLive (bool)
            // NetworkVarNames: m_DmgRadius (float32)
            // NetworkVarNames: m_flDetonateTime (GameTime_t)
            // NetworkVarNames: m_flDamage (float32)
            // NetworkVarNames: m_hThrower (CHandle<CCSPlayerPawn>)
            pub mod CBaseGrenade {
                pub const m_OnPlayerPickup: usize = 0xC88; // CEntityIOOutput
                pub const m_OnExplode: usize = 0xCB0; // CEntityIOOutput
                pub const m_bHasWarnedAI: usize = 0xCD8; // bool
                pub const m_bIsSmokeGrenade: usize = 0xCD9; // bool
                pub const m_bIsLive: usize = 0xCDA; // bool
                pub const m_DmgRadius: usize = 0xCDC; // float32
                pub const m_flDetonateTime: usize = 0xCE0; // GameTime_t
                pub const m_flWarnAITime: usize = 0xCE4; // float32
                pub const m_flDamage: usize = 0xCE8; // float32
                pub const m_iszBounceSound: usize = 0xCF0; // CUtlSymbolLarge
                pub const m_ExplosionSound: usize = 0xCF8; // CUtlString
                pub const m_hThrower: usize = 0xD04; // CHandle<CCSPlayerPawn>
                pub const m_flNextAttack: usize = 0xD1C; // GameTime_t
                pub const m_hOriginalThrower: usize = 0xD20; // CHandle<CCSPlayerPawn>
            }
            // Parent: CBaseAnimGraph
            // Fields count: 11
            //
            // Metadata:
            // MNetworkIncludeByName
            // MNetworkIncludeByName
            // MNetworkIncludeByName
            // MNetworkIncludeByName
            // MNetworkIncludeByName
            // MNetworkIncludeByName
            // MNetworkIncludeByName
            // MNetworkIncludeByName
            // MNetworkIncludeByName
            // MNetworkIncludeByName
            // MNetworkIncludeByName
            // MNetworkIncludeByName
            // MNetworkExcludeByUserGroup
            // MNetworkOverride
            // MNetworkIncludeByName
            // NetworkVarNames: m_nViewModelIndex (uint32)
            // NetworkVarNames: m_nAnimationParity (uint32)
            // NetworkVarNames: m_flAnimationStartTime (float32)
            // NetworkVarNames: m_hWeapon (CHandle<CBasePlayerWeapon>)
            // NetworkVarNames: m_hControlPanel (CHandle<CBaseEntity>)
            pub mod CBaseViewModel {
                pub const m_vecLastFacing: usize = 0xBF8; // Vector
                pub const m_nViewModelIndex: usize = 0xC04; // uint32
                pub const m_nAnimationParity: usize = 0xC08; // uint32
                pub const m_flAnimationStartTime: usize = 0xC0C; // float32
                pub const m_hWeapon: usize = 0xC10; // CHandle<CBasePlayerWeapon>
                pub const m_sVMName: usize = 0xC18; // CUtlSymbolLarge
                pub const m_sAnimationPrefix: usize = 0xC20; // CUtlSymbolLarge
                pub const m_hOldLayerSequence: usize = 0xC28; // HSequence
                pub const m_oldLayer: usize = 0xC2C; // int32
                pub const m_oldLayerStartTime: usize = 0xC30; // float32
                pub const m_hControlPanel: usize = 0xC34; // CHandle<CBaseEntity>
            }
            // Parent: CBaseAnimGraph
            // Fields count: 26
            //
            // Metadata:
            // NetworkVarNames: m_bBombTicking (bool)
            // NetworkVarNames: m_flC4Blow (GameTime_t)
            // NetworkVarNames: m_nBombSite (int)
            // NetworkVarNames: m_nSourceSoundscapeHash (int)
            // NetworkVarNames: m_bCannotBeDefused (bool)
            // NetworkVarNames: m_entitySpottedState (EntitySpottedState_t)
            // NetworkVarNames: m_bHasExploded (bool)
            // NetworkVarNames: m_flTimerLength (float)
            // NetworkVarNames: m_bBeingDefused (bool)
            // NetworkVarNames: m_flDefuseLength (float)
            // NetworkVarNames: m_flDefuseCountDown (GameTime_t)
            // NetworkVarNames: m_bBombDefused (bool)
            // NetworkVarNames: m_hBombDefuser (CHandle<CCSPlayerPawn>)
            // NetworkVarNames: m_hControlPanel (CHandle<CBaseEntity>)
            pub mod CPlantedC4 {
                pub const m_bBombTicking: usize = 0xBF0; // bool
                pub const m_flC4Blow: usize = 0xBF4; // GameTime_t
                pub const m_nBombSite: usize = 0xBF8; // int32
                pub const m_nSourceSoundscapeHash: usize = 0xBFC; // int32
                pub const m_OnBombDefused: usize = 0xC00; // CEntityIOOutput
                pub const m_OnBombBeginDefuse: usize = 0xC28; // CEntityIOOutput
                pub const m_OnBombDefuseAborted: usize = 0xC50; // CEntityIOOutput
                pub const m_bCannotBeDefused: usize = 0xC78; // bool
                pub const m_entitySpottedState: usize = 0xC80; // EntitySpottedState_t
                pub const m_nSpotRules: usize = 0xC98; // int32
                pub const m_bTrainingPlacedByPlayer: usize = 0xC9C; // bool
                pub const m_bHasExploded: usize = 0xC9D; // bool
                pub const m_flTimerLength: usize = 0xCA0; // float32
                pub const m_bBeingDefused: usize = 0xCA4; // bool
                pub const m_fLastDefuseTime: usize = 0xCAC; // GameTime_t
                pub const m_flDefuseLength: usize = 0xCB4; // float32
                pub const m_flDefuseCountDown: usize = 0xCB8; // GameTime_t
                pub const m_bBombDefused: usize = 0xCBC; // bool
                pub const m_hBombDefuser: usize = 0xCC0; // CHandle<CCSPlayerPawn>
                pub const m_hControlPanel: usize = 0xCC4; // CHandle<CBaseEntity>
                pub const m_iProgressBarTime: usize = 0xCC8; // int32
                pub const m_bVoiceAlertFired: usize = 0xCCC; // bool
                pub const m_bVoiceAlertPlayed: usize = 0xCCD; // bool[4]
                pub const m_flNextBotBeepTime: usize = 0xCD4; // GameTime_t
                pub const m_angCatchUpToPlayerEye: usize = 0xCDC; // QAngle
                pub const m_flLastSpinDetectionTime: usize = 0xCE8; // GameTime_t
            }
            // Parent: CBaseGrenade
            // Fields count: 18
            //
            // Metadata:
            // NetworkVarNames: m_vInitialPosition (Vector)
            // NetworkVarNames: m_vInitialVelocity (Vector)
            // NetworkVarNames: m_nBounces (int)
            // NetworkVarNames: m_nExplodeEffectIndex (HParticleSystemDefinitionStrong)
            // NetworkVarNames: m_nExplodeEffectTickBegin (int)
            // NetworkVarNames: m_vecExplodeEffectOrigin (Vector)
            pub mod CBaseCSGrenadeProjectile {
                pub const m_vInitialPosition: usize = 0xD24; // Vector
                pub const m_vInitialVelocity: usize = 0xD30; // Vector
                pub const m_nBounces: usize = 0xD3C; // int32
                pub const m_nExplodeEffectIndex: usize = 0xD40; // CStrongHandle<InfoForResourceTypeIParticleSystemDefinition>
                pub const m_nExplodeEffectTickBegin: usize = 0xD48; // int32
                pub const m_vecExplodeEffectOrigin: usize = 0xD4C; // Vector
                pub const m_flSpawnTime: usize = 0xD58; // GameTime_t
                pub const m_unOGSExtraFlags: usize = 0xD5C; // uint8
                pub const m_bDetonationRecorded: usize = 0xD5D; // bool
                pub const m_flDetonateTime: usize = 0xD60; // GameTime_t
                pub const m_nItemIndex: usize = 0xD64; // uint16
                pub const m_vecOriginalSpawnLocation: usize = 0xD68; // Vector
                pub const m_flLastBounceSoundTime: usize = 0xD74; // GameTime_t
                pub const m_vecGrenadeSpin: usize = 0xD78; // RotationVector
                pub const m_vecLastHitSurfaceNormal: usize = 0xD84; // Vector
                pub const m_nTicksAtZeroVelocity: usize = 0xD90; // int32
                pub const m_bHasEverHitPlayer: usize = 0xD94; // bool
                pub const m_bClearFromPlayers: usize = 0xD95; // bool
            }
            // Parent: None
            // Fields count: 2
            //
            // Metadata:
            // NetworkVarNames: m_OwningPlayer (CHandle<CCSPlayerPawn>)
            // NetworkVarNames: m_KillingPlayer (CHandle<CCSPlayerPawn>)
            pub mod CItemDogtags {
                pub const m_OwningPlayer: usize = 0xCC8; // CHandle<CCSPlayerPawn>
                pub const m_KillingPlayer: usize = 0xCCC; // CHandle<CCSPlayerPawn>
            }
            // Parent: CDynamicProp
            // Fields count: 34
            //
            // Metadata:
            // NetworkVarNames: m_AttributeManager (CAttributeContainer)
            // NetworkVarNames: m_OriginalOwnerXuidLow (uint32)
            // NetworkVarNames: m_OriginalOwnerXuidHigh (uint32)
            // NetworkVarNames: m_jumpedThisFrame (bool)
            // NetworkVarNames: m_leader (CHandle<CCSPlayerPawn>)
            pub mod CChicken {
                pub const m_AttributeManager: usize = 0xE88; // CAttributeContainer
                pub const m_OriginalOwnerXuidLow: usize = 0x1150; // uint32
                pub const m_OriginalOwnerXuidHigh: usize = 0x1154; // uint32
                pub const m_updateTimer: usize = 0x1158; // CountdownTimer
                pub const m_stuckAnchor: usize = 0x1170; // Vector
                pub const m_stuckTimer: usize = 0x1180; // CountdownTimer
                pub const m_collisionStuckTimer: usize = 0x1198; // CountdownTimer
                pub const m_isOnGround: usize = 0x11B0; // bool
                pub const m_vFallVelocity: usize = 0x11B4; // Vector
                pub const m_activity: usize = 0x11C0; // ChickenActivity
                pub const m_activityTimer: usize = 0x11C8; // CountdownTimer
                pub const m_turnRate: usize = 0x11E0; // float32
                pub const m_fleeFrom: usize = 0x11E4; // CHandle<CBaseEntity>
                pub const m_moveRateThrottleTimer: usize = 0x11E8; // CountdownTimer
                pub const m_startleTimer: usize = 0x1200; // CountdownTimer
                pub const m_vocalizeTimer: usize = 0x1218; // CountdownTimer
                pub const m_flWhenZombified: usize = 0x1230; // GameTime_t
                pub const m_jumpedThisFrame: usize = 0x1234; // bool
                pub const m_leader: usize = 0x1238; // CHandle<CCSPlayerPawn>
                pub const m_reuseTimer: usize = 0x1240; // CountdownTimer
                pub const m_hasBeenUsed: usize = 0x1258; // bool
                pub const m_jumpTimer: usize = 0x1260; // CountdownTimer
                pub const m_flLastJumpTime: usize = 0x1278; // float32
                pub const m_bInJump: usize = 0x127C; // bool
                pub const m_isWaitingForLeader: usize = 0x127D; // bool
                pub const m_repathTimer: usize = 0x3288; // CountdownTimer
                pub const m_inhibitDoorTimer: usize = 0x32A0; // CountdownTimer
                pub const m_inhibitObstacleAvoidanceTimer: usize = 0x3330; // CountdownTimer
                pub const m_vecPathGoal: usize = 0x3350; // Vector
                pub const m_flActiveFollowStartTime: usize = 0x335C; // GameTime_t
                pub const m_followMinuteTimer: usize = 0x3360; // CountdownTimer
                pub const m_vecLastEggPoopPosition: usize = 0x3378; // Vector
                pub const m_vecEggsPooped: usize = 0x3388; // CUtlVector<CHandle<CBaseEntity>>
                pub const m_BlockDirectionTimer: usize = 0x33A8; // CountdownTimer
            }
            // Parent: CEconEntity
            // Fields count: 8
            //
            // Metadata:
            // MNetworkExcludeByName
            // MNetworkExcludeByName
            // MNetworkIncludeByName
            // MNetworkExcludeByUserGroup
            // MNetworkExcludeByUserGroup
            // MNetworkExcludeByUserGroup
            // MNetworkUserGroupProxy
            // NetworkVarNames: m_nNextPrimaryAttackTick (GameTick_t)
            // NetworkVarNames: m_flNextPrimaryAttackTickRatio (float32)
            // NetworkVarNames: m_nNextSecondaryAttackTick (GameTick_t)
            // NetworkVarNames: m_flNextSecondaryAttackTickRatio (float32)
            // NetworkVarNames: m_iClip1 (int32)
            // NetworkVarNames: m_iClip2 (int32)
            // NetworkVarNames: m_pReserveAmmo (int)
            pub mod CBasePlayerWeapon {
                pub const m_nNextPrimaryAttackTick: usize = 0xF78; // GameTick_t
                pub const m_flNextPrimaryAttackTickRatio: usize = 0xF7C; // float32
                pub const m_nNextSecondaryAttackTick: usize = 0xF80; // GameTick_t
                pub const m_flNextSecondaryAttackTickRatio: usize = 0xF84; // float32
                pub const m_iClip1: usize = 0xF88; // int32
                pub const m_iClip2: usize = 0xF8C; // int32
                pub const m_pReserveAmmo: usize = 0xF90; // int32[2]
                pub const m_OnPlayerUse: usize = 0xF98; // CEntityIOOutput
            }
            // Parent: CRagdollProp
            // Fields count: 6
            //
            // Metadata:
            // NetworkVarNames: m_boneIndexAttached (uint32)
            // NetworkVarNames: m_ragdollAttachedObjectIndex (uint32)
            // NetworkVarNames: m_attachmentPointBoneSpace (Vector)
            // NetworkVarNames: m_attachmentPointRagdollSpace (Vector)
            pub mod CRagdollPropAttached {
                pub const m_boneIndexAttached: usize = 0xD98; // uint32
                pub const m_ragdollAttachedObjectIndex: usize = 0xD9C; // uint32
                pub const m_attachmentPointBoneSpace: usize = 0xDA0; // Vector
                pub const m_attachmentPointRagdollSpace: usize = 0xDAC; // Vector
                pub const m_bShouldDetach: usize = 0xDB8; // bool
                pub const m_bShouldDeleteAttachedActivationRecord: usize = 0xDC8; // bool
            }
            // Parent: CBaseFlex
            // Fields count: 12
            //
            // Metadata:
            // MNetworkExcludeByUserGroup
            // NetworkVarNames: m_hMyWearables (CHandle<CEconWearable>)
            // NetworkVarNames: m_flFieldOfView (float)
            pub mod CBaseCombatCharacter {
                pub const m_bForceServerRagdoll: usize = 0xC80; // bool
                pub const m_hMyWearables: usize = 0xC88; // CNetworkUtlVectorBase<CHandle<CEconWearable>>
                pub const m_flFieldOfView: usize = 0xCA0; // float32
                pub const m_impactEnergyScale: usize = 0xCA4; // float32
                pub const m_LastHitGroup: usize = 0xCA8; // HitGroup_t
                pub const m_bApplyStressDamage: usize = 0xCAC; // bool
                pub const m_bloodColor: usize = 0xCB0; // int32
                pub const m_iDamageCount: usize = 0xCF8; // int32
                pub const m_pVecRelationships: usize = 0xD00; // CUtlVector<RelationshipOverride_t>*
                pub const m_strRelationships: usize = 0xD08; // CUtlSymbolLarge
                pub const m_eHull: usize = 0xD10; // Hull_t
                pub const m_nNavHullIdx: usize = 0xD14; // uint32
            }
            // Parent: CBaseCombatCharacter
            // Fields count: 25
            //
            // Metadata:
            // MNetworkUserGroupProxy
            // MNetworkUserGroupProxy
            // MNetworkExcludeByUserGroup
            // MNetworkIncludeByUserGroup
            // MNetworkIncludeByUserGroup
            // MNetworkIncludeByUserGroup
            // MNetworkIncludeByName
            // MNetworkOverride
            // MNetworkOverride
            // MNetworkOverride
            // MNetworkExcludeByName
            // MNetworkExcludeByName
            // MNetworkExcludeByName
            // MNetworkExcludeByName
            // MNetworkExcludeByName
            // MNetworkExcludeByName
            // MNetworkExcludeByName
            // MNetworkExcludeByName
            // NetworkVarNames: m_pWeaponServices (CPlayer_WeaponServices*)
            // NetworkVarNames: m_pItemServices (CPlayer_ItemServices*)
            // NetworkVarNames: m_pAutoaimServices (CPlayer_AutoaimServices*)
            // NetworkVarNames: m_pObserverServices (CPlayer_ObserverServices*)
            // NetworkVarNames: m_pWaterServices (CPlayer_WaterServices*)
            // NetworkVarNames: m_pUseServices (CPlayer_UseServices*)
            // NetworkVarNames: m_pFlashlightServices (CPlayer_FlashlightServices*)
            // NetworkVarNames: m_pCameraServices (CPlayer_CameraServices*)
            // NetworkVarNames: m_pMovementServices (CPlayer_MovementServices*)
            // MNetworkUserGroupProxy
            // NetworkVarNames: m_ServerViewAngleChanges (ViewAngleServerChange_t)
            // NetworkVarNames: m_iHideHUD (uint32)
            // NetworkVarNames: m_skybox3d (sky3dparams_t)
            // NetworkVarNames: m_flDeathTime (GameTime_t)
            // NetworkVarNames: m_hController (CHandle<CBasePlayerController>)
            pub mod CBasePlayerPawn {
                pub const m_pWeaponServices: usize = 0xD18; // CPlayer_WeaponServices*
                pub const m_pItemServices: usize = 0xD20; // CPlayer_ItemServices*
                pub const m_pAutoaimServices: usize = 0xD28; // CPlayer_AutoaimServices*
                pub const m_pObserverServices: usize = 0xD30; // CPlayer_ObserverServices*
                pub const m_pWaterServices: usize = 0xD38; // CPlayer_WaterServices*
                pub const m_pUseServices: usize = 0xD40; // CPlayer_UseServices*
                pub const m_pFlashlightServices: usize = 0xD48; // CPlayer_FlashlightServices*
                pub const m_pCameraServices: usize = 0xD50; // CPlayer_CameraServices*
                pub const m_pMovementServices: usize = 0xD58; // CPlayer_MovementServices*
                pub const m_ServerViewAngleChanges: usize = 0xD68; // CUtlVectorEmbeddedNetworkVar<ViewAngleServerChange_t>
                pub const m_nHighestGeneratedServerViewAngleChangeIndex: usize = 0xDB8; // uint32
                pub const v_angle: usize = 0xDBC; // QAngle
                pub const v_anglePrevious: usize = 0xDC8; // QAngle
                pub const m_iHideHUD: usize = 0xDD4; // uint32
                pub const m_skybox3d: usize = 0xDD8; // sky3dparams_t
                pub const m_fTimeLastHurt: usize = 0xE68; // GameTime_t
                pub const m_flDeathTime: usize = 0xE6C; // GameTime_t
                pub const m_fNextSuicideTime: usize = 0xE70; // GameTime_t
                pub const m_fInitHUD: usize = 0xE74; // bool
                pub const m_pExpresser: usize = 0xE78; // CAI_Expresser*
                pub const m_hController: usize = 0xE80; // CHandle<CBasePlayerController>
                pub const m_fHltvReplayDelay: usize = 0xE88; // float32
                pub const m_fHltvReplayEnd: usize = 0xE8C; // float32
                pub const m_iHltvReplayEntity: usize = 0xE90; // CEntityIndex
                pub const m_sndOpvarLatchData: usize = 0xE98; // CUtlVector<sndopvarlatchdata_t>
            }
            // Parent: None
            // Fields count: 3
            //
            // Metadata:
            // MNetworkOverride
            // NetworkVarNames: m_bShouldIgnoreOffsetAndAccuracy (bool)
            // NetworkVarNames: m_nWeaponParity (uint32)
            pub mod CCSGOViewModel {
                pub const m_bShouldIgnoreOffsetAndAccuracy: usize = 0xC38; // bool
                pub const m_nWeaponParity: usize = 0xC3C; // uint32
                pub const m_nOldWeaponParity: usize = 0xC40; // uint32
            }
            // Parent: CBasePlayerWeapon
            // Fields count: 62
            //
            // Metadata:
            // MNetworkExcludeByName
            // NetworkVarNames: m_flFireSequenceStartTime (float)
            // NetworkVarNames: m_nFireSequenceStartTimeChange (int)
            // NetworkVarNames: m_ePlayerFireEvent (PlayerAnimEvent_t)
            // NetworkVarNames: m_ePlayerFireEventAttackType (WeaponAttackType_t)
            // NetworkVarNames: m_iState (CSWeaponState_t)
            // NetworkVarNames: m_nViewModelIndex (uint32)
            // NetworkVarNames: m_flTimeWeaponIdle (GameTime_t)
            // NetworkVarNames: m_weaponMode (CSWeaponMode)
            // NetworkVarNames: m_fAccuracyPenalty (float)
            // NetworkVarNames: m_iRecoilIndex (int)
            // NetworkVarNames: m_flRecoilIndex (float)
            // NetworkVarNames: m_bBurstMode (bool)
            // NetworkVarNames: m_nPostponeFireReadyTicks (GameTick_t)
            // NetworkVarNames: m_flPostponeFireReadyFrac (float)
            // NetworkVarNames: m_bInReload (bool)
            // NetworkVarNames: m_bReloadVisuallyComplete (bool)
            // NetworkVarNames: m_flDroppedAtTime (GameTime_t)
            // NetworkVarNames: m_bIsHauledBack (bool)
            // NetworkVarNames: m_bSilencerOn (bool)
            // NetworkVarNames: m_flTimeSilencerSwitchComplete (GameTime_t)
            // NetworkVarNames: m_iOriginalTeamNumber (int)
            // NetworkVarNames: m_hPrevOwner (CHandle<CCSPlayerPawn>)
            // NetworkVarNames: m_nDropTick (GameTick_t)
            // NetworkVarNames: m_fLastShotTime (GameTime_t)
            // NetworkVarNames: m_iIronSightMode (int)
            // NetworkVarNames: m_iNumEmptyAttacks (int)
            pub mod CCSWeaponBase {
                pub const m_bRemoveable: usize = 0xFE8; // bool
                pub const m_flFireSequenceStartTime: usize = 0xFF0; // float32
                pub const m_nFireSequenceStartTimeChange: usize = 0xFF4; // int32
                pub const m_nFireSequenceStartTimeAck: usize = 0xFF8; // int32
                pub const m_ePlayerFireEvent: usize = 0xFFC; // PlayerAnimEvent_t
                pub const m_ePlayerFireEventAttackType: usize = 0x1000; // WeaponAttackType_t
                pub const m_seqIdle: usize = 0x1004; // HSequence
                pub const m_seqFirePrimary: usize = 0x1008; // HSequence
                pub const m_seqFireSecondary: usize = 0x100C; // HSequence
                pub const m_thirdPersonFireSequences: usize = 0x1010; // CUtlVector<HSequence>
                pub const m_hCurrentThirdPersonSequence: usize = 0x1028; // HSequence
                pub const m_nSilencerBoneIndex: usize = 0x102C; // int32
                pub const m_thirdPersonSequences: usize = 0x1030; // HSequence[7]
                pub const m_bPlayerAmmoStockOnPickup: usize = 0x1058; // bool
                pub const m_bRequireUseToTouch: usize = 0x1059; // bool
                pub const m_iState: usize = 0x105C; // CSWeaponState_t
                pub const m_flLastTimeInAir: usize = 0x1060; // GameTime_t
                pub const m_flLastDeployTime: usize = 0x1064; // GameTime_t
                pub const m_nLastEmptySoundCmdNum: usize = 0x1068; // int32
                pub const m_nViewModelIndex: usize = 0x106C; // uint32
                pub const m_bReloadsWithClips: usize = 0x1070; // bool
                pub const m_flTimeWeaponIdle: usize = 0x1090; // GameTime_t
                pub const m_bFireOnEmpty: usize = 0x1094; // bool
                pub const m_OnPlayerPickup: usize = 0x1098; // CEntityIOOutput
                pub const m_weaponMode: usize = 0x10C0; // CSWeaponMode
                pub const m_flTurningInaccuracyDelta: usize = 0x10C4; // float32
                pub const m_vecTurningInaccuracyEyeDirLast: usize = 0x10C8; // Vector
                pub const m_flTurningInaccuracy: usize = 0x10D4; // float32
                pub const m_fAccuracyPenalty: usize = 0x10D8; // float32
                pub const m_flLastAccuracyUpdateTime: usize = 0x10DC; // GameTime_t
                pub const m_fAccuracySmoothedForZoom: usize = 0x10E0; // float32
                pub const m_fScopeZoomEndTime: usize = 0x10E4; // GameTime_t
                pub const m_iRecoilIndex: usize = 0x10E8; // int32
                pub const m_flRecoilIndex: usize = 0x10EC; // float32
                pub const m_bBurstMode: usize = 0x10F0; // bool
                pub const m_nPostponeFireReadyTicks: usize = 0x10F4; // GameTick_t
                pub const m_flPostponeFireReadyFrac: usize = 0x10F8; // float32
                pub const m_bInReload: usize = 0x10FC; // bool
                pub const m_bReloadVisuallyComplete: usize = 0x10FD; // bool
                pub const m_flDroppedAtTime: usize = 0x1100; // GameTime_t
                pub const m_bIsHauledBack: usize = 0x1104; // bool
                pub const m_bSilencerOn: usize = 0x1105; // bool
                pub const m_flTimeSilencerSwitchComplete: usize = 0x1108; // GameTime_t
                pub const m_iOriginalTeamNumber: usize = 0x110C; // int32
                pub const m_flNextAttackRenderTimeOffset: usize = 0x1110; // float32
                pub const m_bCanBePickedUp: usize = 0x1128; // bool
                pub const m_bUseCanOverrideNextOwnerTouchTime: usize = 0x1129; // bool
                pub const m_nextOwnerTouchTime: usize = 0x112C; // GameTime_t
                pub const m_nextPrevOwnerTouchTime: usize = 0x1130; // GameTime_t
                pub const m_hPrevOwner: usize = 0x1134; // CHandle<CCSPlayerPawn>
                pub const m_nDropTick: usize = 0x1138; // GameTick_t
                pub const m_donated: usize = 0x115C; // bool
                pub const m_fLastShotTime: usize = 0x1160; // GameTime_t
                pub const m_bWasOwnedByCT: usize = 0x1164; // bool
                pub const m_bWasOwnedByTerrorist: usize = 0x1165; // bool
                pub const m_bFiredOutOfAmmoEvent: usize = 0x1166; // bool
                pub const m_numRemoveUnownedWeaponThink: usize = 0x1168; // int32
                pub const m_IronSightController: usize = 0x1170; // CIronSightController
                pub const m_iIronSightMode: usize = 0x1188; // int32
                pub const m_flLastLOSTraceFailureTime: usize = 0x118C; // GameTime_t
                pub const m_iNumEmptyAttacks: usize = 0x1190; // int32
                pub const m_flWatTickOffset: usize = 0x1194; // float32
            }
            // Parent: CCSWeaponBase
            // Fields count: 9
            //
            // Metadata:
            // NetworkVarNames: m_zoomLevel (int)
            // NetworkVarNames: m_iBurstShotsRemaining (int)
            // NetworkVarNames: m_bNeedsBoltAction (bool)
            pub mod CCSWeaponBaseGun {
                pub const m_zoomLevel: usize = 0x1198; // int32
                pub const m_iBurstShotsRemaining: usize = 0x119C; // int32
                pub const m_silencedModelIndex: usize = 0x11A8; // int32
                pub const m_inPrecache: usize = 0x11AC; // bool
                pub const m_bNeedsBoltAction: usize = 0x11AD; // bool
                pub const m_bSkillReloadAvailable: usize = 0x11AE; // bool
                pub const m_bSkillReloadLiftedReloadKey: usize = 0x11AF; // bool
                pub const m_bSkillBoltInterruptAvailable: usize = 0x11B0; // bool
                pub const m_bSkillBoltLiftedFireKey: usize = 0x11B1; // bool
            }
            // Parent: CCSWeaponBase
            // Fields count: 11
            //
            // Metadata:
            // NetworkVarNames: m_bStartedArming (bool)
            // NetworkVarNames: m_fArmedTime (GameTime_t)
            // NetworkVarNames: m_bBombPlacedAnimation (bool)
            // NetworkVarNames: m_bIsPlantingViaUse (bool)
            // NetworkVarNames: m_entitySpottedState (EntitySpottedState_t)
            pub mod CC4 {
                pub const m_vecLastValidPlayerHeldPosition: usize = 0x1198; // Vector
                pub const m_vecLastValidDroppedPosition: usize = 0x11A4; // Vector
                pub const m_bDoValidDroppedPositionCheck: usize = 0x11B0; // bool
                pub const m_bStartedArming: usize = 0x11B1; // bool
                pub const m_fArmedTime: usize = 0x11B4; // GameTime_t
                pub const m_bBombPlacedAnimation: usize = 0x11B8; // bool
                pub const m_bIsPlantingViaUse: usize = 0x11B9; // bool
                pub const m_entitySpottedState: usize = 0x11C0; // EntitySpottedState_t
                pub const m_nSpotRules: usize = 0x11D8; // int32
                pub const m_bPlayedArmingBeeps: usize = 0x11DC; // bool[7]
                pub const m_bBombPlanted: usize = 0x11E3; // bool
            }
            // Parent: CCSWeaponBaseGun
            // Fields count: 1
            //
            // Metadata:
            // NetworkVarNames: m_fFireTime (GameTime_t)
            pub mod CWeaponTaser {
                pub const m_fFireTime: usize = 0x11B4; // GameTime_t
            }
            // Parent: CCSWeaponBaseGun
            // Fields count: 3
            //
            // Metadata:
            // NetworkVarNames: m_flDisplayHealth (float)
            pub mod CWeaponShield {
                pub const m_flBulletDamageAbsorbed: usize = 0x11B4; // float32
                pub const m_flLastBulletHitSoundTime: usize = 0x11B8; // GameTime_t
                pub const m_flDisplayHealth: usize = 0x11BC; // float32
            }
            // Parent: CBaseCSGrenadeProjectile
            // Fields count: 4
            //
            // Metadata:
            // NetworkVarNames: m_bIsIncGrenade (bool)
            pub mod CMolotovProjectile {
                pub const m_bIsIncGrenade: usize = 0xD96; // bool
                pub const m_bDetonated: usize = 0xDA0; // bool
                pub const m_stillTimer: usize = 0xDA8; // IntervalTimer
                pub const m_bHasBouncedOffPlayer: usize = 0xE88; // bool
            }
            // Parent: CBaseCSGrenadeProjectile
            // Fields count: 4
            //
            // Metadata:
            // NetworkVarNames: m_nDecoyShotTick (int)
            pub mod CDecoyProjectile {
                pub const m_nDecoyShotTick: usize = 0xDA0; // int32
                pub const m_shotsRemaining: usize = 0xDA4; // int32
                pub const m_fExpireTime: usize = 0xDA8; // GameTime_t
                pub const m_decoyWeaponDefIndex: usize = 0xDB8; // uint16
            }
            // Parent: CBaseCSGrenadeProjectile
            // Fields count: 8
            //
            // Metadata:
            // NetworkVarNames: m_nSmokeEffectTickBegin (int)
            // NetworkVarNames: m_bDidSmokeEffect (bool)
            // NetworkVarNames: m_nRandomSeed (int)
            // NetworkVarNames: m_vSmokeColor (Vector)
            // NetworkVarNames: m_vSmokeDetonationPos (Vector)
            // NetworkVarNames: m_VoxelFrameData (CUtlVector<uint8>)
            pub mod CSmokeGrenadeProjectile {
                pub const m_nSmokeEffectTickBegin: usize = 0xDB0; // int32
                pub const m_bDidSmokeEffect: usize = 0xDB4; // bool
                pub const m_nRandomSeed: usize = 0xDB8; // int32
                pub const m_vSmokeColor: usize = 0xDBC; // Vector
                pub const m_vSmokeDetonationPos: usize = 0xDC8; // Vector
                pub const m_VoxelFrameData: usize = 0xDD8; // CUtlVector<uint8>
                pub const m_flLastBounce: usize = 0xDF0; // GameTime_t
                pub const m_fllastSimulationTime: usize = 0xDF4; // GameTime_t
            }
            // Parent: CCSWeaponBase
            // Fields count: 13
            //
            // Metadata:
            // NetworkVarNames: m_bRedraw (bool)
            // NetworkVarNames: m_bIsHeldByPlayer (bool)
            // NetworkVarNames: m_bPinPulled (bool)
            // NetworkVarNames: m_bJumpThrow (bool)
            // NetworkVarNames: m_bThrowAnimating (bool)
            // NetworkVarNames: m_fThrowTime (GameTime_t)
            // NetworkVarNames: m_flThrowStrength (float)
            // NetworkVarNames: m_flThrowStrengthApproach (float)
            // NetworkVarNames: m_fDropTime (GameTime_t)
            // NetworkVarNames: m_bJustPulledPin (bool)
            // NetworkVarNames: m_nNextHoldTick (GameTick_t)
            // NetworkVarNames: m_flNextHoldFrac (float)
            // NetworkVarNames: m_hSwitchToWeaponAfterThrow (CHandle<CCSWeaponBase>)
            pub mod CBaseCSGrenade {
                pub const m_bRedraw: usize = 0x1198; // bool
                pub const m_bIsHeldByPlayer: usize = 0x1199; // bool
                pub const m_bPinPulled: usize = 0x119A; // bool
                pub const m_bJumpThrow: usize = 0x119B; // bool
                pub const m_bThrowAnimating: usize = 0x119C; // bool
                pub const m_fThrowTime: usize = 0x11A0; // GameTime_t
                pub const m_flThrowStrength: usize = 0x11A4; // float32
                pub const m_flThrowStrengthApproach: usize = 0x11A8; // float32
                pub const m_fDropTime: usize = 0x11AC; // GameTime_t
                pub const m_bJustPulledPin: usize = 0x11B0; // bool
                pub const m_nNextHoldTick: usize = 0x11B4; // GameTick_t
                pub const m_flNextHoldFrac: usize = 0x11B8; // float32
                pub const m_hSwitchToWeaponAfterThrow: usize = 0x11BC; // CHandle<CCSWeaponBase>
            }
            // Parent: CCSWeaponBase
            // Fields count: 2
            //
            // Metadata:
            // NetworkVarNames: m_SequenceCompleteTimer (CountdownTimer)
            // NetworkVarNames: m_bRedraw (bool)
            pub mod CWeaponBaseItem {
                pub const m_SequenceCompleteTimer: usize = 0x1198; // CountdownTimer
                pub const m_bRedraw: usize = 0x11B0; // bool
            }
            // Parent: CCSWeaponBase
            // Fields count: 7
            //
            // Metadata:
            // NetworkVarNames: m_bPlayingUninterruptableAct (bool)
            // NetworkVarNames: m_nUninterruptableActivity (PlayerAnimEvent_t)
            pub mod CFists {
                pub const m_bPlayingUninterruptableAct: usize = 0x1198; // bool
                pub const m_nUninterruptableActivity: usize = 0x119C; // PlayerAnimEvent_t
                pub const m_bRestorePrevWep: usize = 0x11A0; // bool
                pub const m_hWeaponBeforePrevious: usize = 0x11A4; // CHandle<CBasePlayerWeapon>
                pub const m_hWeaponPrevious: usize = 0x11A8; // CHandle<CBasePlayerWeapon>
                pub const m_bDelayedHardPunchIncoming: usize = 0x11AC; // bool
                pub const m_bDestroyAfterTaunt: usize = 0x11AD; // bool
            }
            // Parent: CBasePlayerPawn
            // Fields count: 136
            //
            // Metadata:
            // NetworkVarNames: m_CTouchExpansionComponent (CTouchExpansionComponent::Storage_t)
            // MNetworkExcludeByName
            // MNetworkExcludeByName
            // MNetworkExcludeByUserGroup
            // MNetworkExcludeByName
            // MNetworkExcludeByName
            // NetworkVarNames: m_pPingServices (CCSPlayer_PingServices*)
            // NetworkVarNames: m_pViewModelServices (CPlayer_ViewModelServices*)
            // NetworkVarNames: m_hOriginalController (CHandle<CCSPlayerController>)
            // NetworkVarNames: m_entitySpottedState (EntitySpottedState_t)
            // NetworkVarNames: m_iPlayerState (CSPlayerState)
            // NetworkVarNames: m_bIsScoped (bool)
            // NetworkVarNames: m_bIsWalking (bool)
            // NetworkVarNames: m_bResumeZoom (bool)
            // NetworkVarNames: m_bIsDefusing (bool)
            // NetworkVarNames: m_bIsGrabbingHostage (bool)
            // NetworkVarNames: m_iBlockingUseActionInProgress (CSPlayerBlockingUseAction_t)
            // NetworkVarNames: m_fImmuneToGunGameDamageTime (GameTime_t)
            // NetworkVarNames: m_bGunGameImmunity (bool)
            // NetworkVarNames: m_GunGameImmunityColor (Color)
            // NetworkVarNames: m_fMolotovDamageTime (float)
            // NetworkVarNames: m_bHasMovedSinceSpawn (bool)
            // NetworkVarNames: m_bCanMoveDuringFreezePeriod (bool)
            // NetworkVarNames: m_flGuardianTooFarDistFrac (float)
            // NetworkVarNames: m_flDetectedByEnemySensorTime (GameTime_t)
            // NetworkVarNames: m_nHeavyAssaultSuitCooldownRemaining (int)
            // NetworkVarNames: m_flEmitSoundTime (GameTime_t)
            // NetworkVarNames: m_bHasNightVision (bool)
            // NetworkVarNames: m_bNightVisionOn (bool)
            // NetworkVarNames: m_bInNoDefuseArea (bool)
            // NetworkVarNames: m_bKilledByTaser (bool)
            // NetworkVarNames: m_iMoveState (int)
            // NetworkVarNames: m_nWhichBombZone (int)
            // NetworkVarNames: m_iDirection (int)
            // NetworkVarNames: m_iShotsFired (int)
            // NetworkVarNames: m_ArmorValue (int32)
            // NetworkVarNames: m_flVelocityModifier (float)
            // NetworkVarNames: m_flHitHeading (float)
            // NetworkVarNames: m_nHitBodyPart (int)
            // NetworkVarNames: m_flFlashDuration (float)
            // NetworkVarNames: m_flFlashMaxAlpha (float)
            // NetworkVarNames: m_flProgressBarStartTime (float)
            // NetworkVarNames: m_iProgressBarDuration (int)
            // NetworkVarNames: m_bWaitForNoAttack (bool)
            // NetworkVarNames: m_flLowerBodyYawTarget (float)
            // NetworkVarNames: m_bStrafing (bool)
            // NetworkVarNames: m_thirdPersonHeading (QAngle)
            // NetworkVarNames: m_flSlopeDropOffset (float)
            // NetworkVarNames: m_flSlopeDropHeight (float)
            // NetworkVarNames: m_vHeadConstraintOffset (Vector)
            // NetworkVarNames: m_angEyeAngles (QAngle)
            // NetworkVarNames: m_bHideTargetID (bool)
            // NetworkVarNames: m_bHud_MiniScoreHidden (bool)
            // NetworkVarNames: m_bHud_RadarHidden (bool)
            // NetworkVarNames: m_nLastKillerIndex (CEntityIndex)
            // NetworkVarNames: m_nLastConcurrentKilled (int)
            // NetworkVarNames: m_nDeathCamMusic (int)
            // NetworkVarNames: m_iAddonBits (int)
            // NetworkVarNames: m_iPrimaryAddon (int)
            // NetworkVarNames: m_iSecondaryAddon (int)
            // NetworkVarNames: m_vecPlayerPatchEconIndices (uint32)
            // NetworkVarNames: m_unCurrentEquipmentValue (uint16)
            // NetworkVarNames: m_unRoundStartEquipmentValue (uint16)
            // NetworkVarNames: m_unFreezetimeEndEquipmentValue (uint16)
            // NetworkVarNames: m_nSurvivalTeamNumber (int)
            // NetworkVarNames: m_bKilledByHeadshot (bool)
            pub mod CCSPlayerPawnBase {
                pub const m_CTouchExpansionComponent: usize = 0xEC8; // CTouchExpansionComponent
                pub const m_pPingServices: usize = 0xF18; // CCSPlayer_PingServices*
                pub const m_pViewModelServices: usize = 0xF20; // CPlayer_ViewModelServices*
                pub const m_iDisplayHistoryBits: usize = 0xF28; // uint32
                pub const m_flLastAttackedTeammate: usize = 0xF2C; // float32
                pub const m_hOriginalController: usize = 0xF30; // CHandle<CCSPlayerController>
                pub const m_blindUntilTime: usize = 0xF34; // GameTime_t
                pub const m_blindStartTime: usize = 0xF38; // GameTime_t
                pub const m_allowAutoFollowTime: usize = 0xF3C; // GameTime_t
                pub const m_entitySpottedState: usize = 0xF40; // EntitySpottedState_t
                pub const m_nSpotRules: usize = 0xF58; // int32
                pub const m_iPlayerState: usize = 0xF5C; // CSPlayerState
                pub const m_chickenIdleSoundTimer: usize = 0xF68; // CountdownTimer
                pub const m_chickenJumpSoundTimer: usize = 0xF80; // CountdownTimer
                pub const m_vecLastBookmarkedPosition: usize = 0x1038; // Vector
                pub const m_flLastDistanceTraveledNotice: usize = 0x1044; // float32
                pub const m_flAccumulatedDistanceTraveled: usize = 0x1048; // float32
                pub const m_flLastFriendlyFireDamageReductionRatio: usize = 0x104C; // float32
                pub const m_bRespawning: usize = 0x1050; // bool
                pub const m_nLastPickupPriority: usize = 0x1054; // int32
                pub const m_flLastPickupPriorityTime: usize = 0x1058; // float32
                pub const m_bIsScoped: usize = 0x105C; // bool
                pub const m_bIsWalking: usize = 0x105D; // bool
                pub const m_bResumeZoom: usize = 0x105E; // bool
                pub const m_bIsDefusing: usize = 0x105F; // bool
                pub const m_bIsGrabbingHostage: usize = 0x1060; // bool
                pub const m_iBlockingUseActionInProgress: usize = 0x1064; // CSPlayerBlockingUseAction_t
                pub const m_fImmuneToGunGameDamageTime: usize = 0x1068; // GameTime_t
                pub const m_bGunGameImmunity: usize = 0x106C; // bool
                pub const m_GunGameImmunityColor: usize = 0x106D; // Color
                pub const m_fMolotovDamageTime: usize = 0x1074; // float32
                pub const m_bHasMovedSinceSpawn: usize = 0x1078; // bool
                pub const m_bCanMoveDuringFreezePeriod: usize = 0x1079; // bool
                pub const m_flGuardianTooFarDistFrac: usize = 0x107C; // float32
                pub const m_flNextGuardianTooFarHurtTime: usize = 0x1080; // float32
                pub const m_flDetectedByEnemySensorTime: usize = 0x1084; // GameTime_t
                pub const m_flDealtDamageToEnemyMostRecentTimestamp: usize = 0x1088; // float32
                pub const m_flLastEquippedHelmetTime: usize = 0x108C; // GameTime_t
                pub const m_flLastEquippedArmorTime: usize = 0x1090; // GameTime_t
                pub const m_nHeavyAssaultSuitCooldownRemaining: usize = 0x1094; // int32
                pub const m_bResetArmorNextSpawn: usize = 0x1098; // bool
                pub const m_flLastBumpMineBumpTime: usize = 0x109C; // GameTime_t
                pub const m_flEmitSoundTime: usize = 0x10A0; // GameTime_t
                pub const m_iNumSpawns: usize = 0x10A4; // int32
                pub const m_iShouldHaveCash: usize = 0x10A8; // int32
                pub const m_flIdleTimeSinceLastAction: usize = 0x10B0; // float32
                pub const m_flNameChangeHistory: usize = 0x10B4; // float32[5]
                pub const m_fLastGivenDefuserTime: usize = 0x10C8; // float32
                pub const m_fLastGivenBombTime: usize = 0x10CC; // float32
                pub const m_bHasNightVision: usize = 0x10D0; // bool
                pub const m_bNightVisionOn: usize = 0x10D1; // bool
                pub const m_fNextRadarUpdateTime: usize = 0x10D4; // float32
                pub const m_flLastMoneyUpdateTime: usize = 0x10D8; // float32
                pub const m_MenuStringBuffer: usize = 0x10DC; // char[1024]
                pub const m_fIntroCamTime: usize = 0x14DC; // float32
                pub const m_nMyCollisionGroup: usize = 0x14E0; // int32
                pub const m_bInNoDefuseArea: usize = 0x14E4; // bool
                pub const m_bKilledByTaser: usize = 0x14E5; // bool
                pub const m_iMoveState: usize = 0x14E8; // int32
                pub const m_grenadeParameterStashTime: usize = 0x14EC; // GameTime_t
                pub const m_bGrenadeParametersStashed: usize = 0x14F0; // bool
                pub const m_angStashedShootAngles: usize = 0x14F4; // QAngle
                pub const m_vecStashedGrenadeThrowPosition: usize = 0x1500; // Vector
                pub const m_vecStashedVelocity: usize = 0x150C; // Vector
                pub const m_angShootAngleHistory: usize = 0x1518; // QAngle[2]
                pub const m_vecThrowPositionHistory: usize = 0x1530; // Vector[2]
                pub const m_vecVelocityHistory: usize = 0x1548; // Vector[2]
                pub const m_bDiedAirborne: usize = 0x1560; // bool
                pub const m_iBombSiteIndex: usize = 0x1564; // CEntityIndex
                pub const m_nWhichBombZone: usize = 0x1568; // int32
                pub const m_bInBombZoneTrigger: usize = 0x156C; // bool
                pub const m_bWasInBombZoneTrigger: usize = 0x156D; // bool
                pub const m_iDirection: usize = 0x1570; // int32
                pub const m_iShotsFired: usize = 0x1574; // int32
                pub const m_ArmorValue: usize = 0x1578; // int32
                pub const m_flFlinchStack: usize = 0x157C; // float32
                pub const m_flVelocityModifier: usize = 0x1580; // float32
                pub const m_flHitHeading: usize = 0x1584; // float32
                pub const m_nHitBodyPart: usize = 0x1588; // int32
                pub const m_iHostagesKilled: usize = 0x158C; // int32
                pub const m_vecTotalBulletForce: usize = 0x1590; // Vector
                pub const m_flFlashDuration: usize = 0x159C; // float32
                pub const m_flFlashMaxAlpha: usize = 0x15A0; // float32
                pub const m_flProgressBarStartTime: usize = 0x15A4; // float32
                pub const m_iProgressBarDuration: usize = 0x15A8; // int32
                pub const m_bWaitForNoAttack: usize = 0x15AC; // bool
                pub const m_flLowerBodyYawTarget: usize = 0x15B0; // float32
                pub const m_bStrafing: usize = 0x15B4; // bool
                pub const m_lastStandingPos: usize = 0x15B8; // Vector
                pub const m_ignoreLadderJumpTime: usize = 0x15C4; // float32
                pub const m_ladderSurpressionTimer: usize = 0x15C8; // CountdownTimer
                pub const m_lastLadderNormal: usize = 0x15E0; // Vector
                pub const m_lastLadderPos: usize = 0x15EC; // Vector
                pub const m_thirdPersonHeading: usize = 0x15F8; // QAngle
                pub const m_flSlopeDropOffset: usize = 0x1604; // float32
                pub const m_flSlopeDropHeight: usize = 0x1608; // float32
                pub const m_vHeadConstraintOffset: usize = 0x160C; // Vector
                pub const m_iLastWeaponFireUsercmd: usize = 0x1620; // int32
                pub const m_angEyeAngles: usize = 0x1624; // QAngle
                pub const m_bVCollisionInitted: usize = 0x1630; // bool
                pub const m_storedSpawnPosition: usize = 0x1634; // Vector
                pub const m_storedSpawnAngle: usize = 0x1640; // QAngle
                pub const m_bIsSpawning: usize = 0x164C; // bool
                pub const m_bHideTargetID: usize = 0x164D; // bool
                pub const m_nNumDangerZoneDamageHits: usize = 0x1650; // int32
                pub const m_bHud_MiniScoreHidden: usize = 0x1654; // bool
                pub const m_bHud_RadarHidden: usize = 0x1655; // bool
                pub const m_nLastKillerIndex: usize = 0x1658; // CEntityIndex
                pub const m_nLastConcurrentKilled: usize = 0x165C; // int32
                pub const m_nDeathCamMusic: usize = 0x1660; // int32
                pub const m_iAddonBits: usize = 0x1664; // int32
                pub const m_iPrimaryAddon: usize = 0x1668; // int32
                pub const m_iSecondaryAddon: usize = 0x166C; // int32
                pub const m_currentDeafnessFilter: usize = 0x1670; // CUtlStringToken
                pub const m_NumEnemiesKilledThisSpawn: usize = 0x1674; // int32
                pub const m_NumEnemiesKilledThisRound: usize = 0x1678; // int32
                pub const m_NumEnemiesAtRoundStart: usize = 0x167C; // int32
                pub const m_wasNotKilledNaturally: usize = 0x1680; // bool
                pub const m_vecPlayerPatchEconIndices: usize = 0x1684; // uint32[5]
                pub const m_iDeathFlags: usize = 0x1698; // int32
                pub const m_hPet: usize = 0x169C; // CHandle<CChicken>
                pub const m_unCurrentEquipmentValue: usize = 0x1868; // uint16
                pub const m_unRoundStartEquipmentValue: usize = 0x186A; // uint16
                pub const m_unFreezetimeEndEquipmentValue: usize = 0x186C; // uint16
                pub const m_nSurvivalTeamNumber: usize = 0x1870; // int32
                pub const m_bHasDeathInfo: usize = 0x1874; // bool
                pub const m_flDeathInfoTime: usize = 0x1878; // float32
                pub const m_vecDeathInfoOrigin: usize = 0x187C; // Vector
                pub const m_bKilledByHeadshot: usize = 0x1888; // bool
                pub const m_LastHitBox: usize = 0x188C; // int32
                pub const m_LastHealth: usize = 0x1890; // int32
                pub const m_flLastCollisionCeiling: usize = 0x1894; // float32
                pub const m_flLastCollisionCeilingChangeTime: usize = 0x1898; // float32
                pub const m_pBot: usize = 0x18A0; // CCSBot*
                pub const m_bBotAllowActive: usize = 0x18A8; // bool
                pub const m_bCommittingSuicideOnTeamChange: usize = 0x18A9; // bool
            }
            // Parent: CCSPlayerPawnBase
            // Fields count: 0
            //
            // Metadata:
            // MNetworkVarTypeOverride
            // MNetworkIncludeByName
            // MNetworkVarTypeOverride
            // MNetworkIncludeByName
            // MNetworkVarTypeOverride
            // MNetworkIncludeByName
            // MNetworkVarTypeOverride
            // MNetworkIncludeByName
            // MNetworkVarTypeOverride
            // MNetworkIncludeByName
            pub mod CCSObserverPawn {
            }
            // Parent: CCSPlayerPawnBase
            // Fields count: 49
            //
            // Metadata:
            // MNetworkVarTypeOverride
            // MNetworkIncludeByName
            // MNetworkVarTypeOverride
            // MNetworkIncludeByName
            // MNetworkVarTypeOverride
            // MNetworkIncludeByName
            // MNetworkVarTypeOverride
            // MNetworkIncludeByName
            // MNetworkVarTypeOverride
            // MNetworkIncludeByName
            // MNetworkVarTypeOverride
            // MNetworkIncludeByName
            // MNetworkVarTypeOverride
            // MNetworkIncludeByName
            // NetworkVarNames: m_pBulletServices (CCSPlayer_BulletServices*)
            // NetworkVarNames: m_pHostageServices (CCSPlayer_HostageServices*)
            // NetworkVarNames: m_pBuyServices (CCSPlayer_BuyServices*)
            // NetworkVarNames: m_pActionTrackingServices (CCSPlayer_ActionTrackingServices*)
            // NetworkVarNames: m_bHasFemaleVoice (bool)
            // NetworkVarNames: m_szLastPlaceName (char)
            // NetworkVarNames: m_bInBuyZone (bool)
            // NetworkVarNames: m_bInHostageRescueZone (bool)
            // NetworkVarNames: m_bInBombZone (bool)
            // NetworkVarNames: m_iRetakesOffering (int)
            // NetworkVarNames: m_iRetakesOfferingCard (int)
            // NetworkVarNames: m_bRetakesHasDefuseKit (bool)
            // NetworkVarNames: m_bRetakesMVPLastRound (bool)
            // NetworkVarNames: m_iRetakesMVPBoostItem (int)
            // NetworkVarNames: m_RetakesMVPBoostExtraUtility (loadout_slot_t)
            // NetworkVarNames: m_flHealthShotBoostExpirationTime (GameTime_t)
            // NetworkVarNames: m_aimPunchAngle (QAngle)
            // NetworkVarNames: m_aimPunchAngleVel (QAngle)
            // NetworkVarNames: m_aimPunchTickBase (int)
            // NetworkVarNames: m_aimPunchTickFraction (float)
            // NetworkVarNames: m_bIsBuyMenuOpen (bool)
            // NetworkVarNames: m_flTimeOfLastInjury (GameTime_t)
            // NetworkVarNames: m_flNextSprayDecalTime (GameTime_t)
            // NetworkVarNames: m_nRagdollDamageBone (int)
            // NetworkVarNames: m_vRagdollDamageForce (Vector)
            // NetworkVarNames: m_vRagdollDamagePosition (Vector)
            // NetworkVarNames: m_szRagdollDamageWeaponName (char)
            // NetworkVarNames: m_bRagdollDamageHeadshot (bool)
            // NetworkVarNames: m_vRagdollServerOrigin (Vector)
            // NetworkVarNames: m_EconGloves (CEconItemView)
            // NetworkVarNames: m_nEconGlovesChanged (uint8)
            // NetworkVarNames: m_qDeathEyeAngles (QAngle)
            pub mod CCSPlayerPawn {
                pub const m_pBulletServices: usize = 0x18B0; // CCSPlayer_BulletServices*
                pub const m_pHostageServices: usize = 0x18B8; // CCSPlayer_HostageServices*
                pub const m_pBuyServices: usize = 0x18C0; // CCSPlayer_BuyServices*
                pub const m_pActionTrackingServices: usize = 0x18C8; // CCSPlayer_ActionTrackingServices*
                pub const m_pRadioServices: usize = 0x18D0; // CCSPlayer_RadioServices*
                pub const m_pDamageReactServices: usize = 0x18D8; // CCSPlayer_DamageReactServices*
                pub const m_nCharacterDefIndex: usize = 0x18E0; // uint16
                pub const m_hPreviousModel: usize = 0x18E8; // CStrongHandle<InfoForResourceTypeCModel>
                pub const m_bHasFemaleVoice: usize = 0x18F0; // bool
                pub const m_strVOPrefix: usize = 0x18F8; // CUtlString
                pub const m_szLastPlaceName: usize = 0x1900; // char[18]
                pub const m_bInHostageResetZone: usize = 0x19C0; // bool
                pub const m_bInBuyZone: usize = 0x19C1; // bool
                pub const m_bWasInBuyZone: usize = 0x19C2; // bool
                pub const m_bInHostageRescueZone: usize = 0x19C3; // bool
                pub const m_bInBombZone: usize = 0x19C4; // bool
                pub const m_bWasInHostageRescueZone: usize = 0x19C5; // bool
                pub const m_iRetakesOffering: usize = 0x19C8; // int32
                pub const m_iRetakesOfferingCard: usize = 0x19CC; // int32
                pub const m_bRetakesHasDefuseKit: usize = 0x19D0; // bool
                pub const m_bRetakesMVPLastRound: usize = 0x19D1; // bool
                pub const m_iRetakesMVPBoostItem: usize = 0x19D4; // int32
                pub const m_RetakesMVPBoostExtraUtility: usize = 0x19D8; // loadout_slot_t
                pub const m_flHealthShotBoostExpirationTime: usize = 0x19DC; // GameTime_t
                pub const m_flLandseconds: usize = 0x19E0; // float32
                pub const m_aimPunchAngle: usize = 0x19E4; // QAngle
                pub const m_aimPunchAngleVel: usize = 0x19F0; // QAngle
                pub const m_aimPunchTickBase: usize = 0x19FC; // int32
                pub const m_aimPunchTickFraction: usize = 0x1A00; // float32
                pub const m_aimPunchCache: usize = 0x1A08; // CUtlVector<QAngle>
                pub const m_bIsBuyMenuOpen: usize = 0x1A20; // bool
                pub const m_xLastHeadBoneTransform: usize = 0x2060; // CTransform
                pub const m_bLastHeadBoneTransformIsValid: usize = 0x2080; // bool
                pub const m_lastLandTime: usize = 0x2084; // GameTime_t
                pub const m_bOnGroundLastTick: usize = 0x2088; // bool
                pub const m_iPlayerLocked: usize = 0x208C; // int32
                pub const m_flTimeOfLastInjury: usize = 0x2094; // GameTime_t
                pub const m_flNextSprayDecalTime: usize = 0x2098; // GameTime_t
                pub const m_bNextSprayDecalTimeExpedited: usize = 0x209C; // bool
                pub const m_nRagdollDamageBone: usize = 0x20A0; // int32
                pub const m_vRagdollDamageForce: usize = 0x20A4; // Vector
                pub const m_vRagdollDamagePosition: usize = 0x20B0; // Vector
                pub const m_szRagdollDamageWeaponName: usize = 0x20BC; // char[64]
                pub const m_bRagdollDamageHeadshot: usize = 0x20FC; // bool
                pub const m_vRagdollServerOrigin: usize = 0x2100; // Vector
                pub const m_EconGloves: usize = 0x2110; // CEconItemView
                pub const m_nEconGlovesChanged: usize = 0x2388; // uint8
                pub const m_qDeathEyeAngles: usize = 0x238C; // QAngle
                pub const m_bSkipOneHeadConstraintUpdate: usize = 0x2398; // bool
            }
            // Parent: None
            // Fields count: 39
            //
            // Metadata:
            // NetworkVarNames: m_entitySpottedState (EntitySpottedState_t)
            // MNetworkIncludeByName
            // MNetworkIncludeByName
            // MNetworkIncludeByName
            // MNetworkIncludeByName
            // MNetworkIncludeByName
            // NetworkVarNames: m_vel (Vector)
            // NetworkVarNames: m_isRescued (bool)
            // NetworkVarNames: m_jumpedThisFrame (bool)
            // NetworkVarNames: m_nHostageState (int)
            // NetworkVarNames: m_leader (CHandle<CBaseEntity>)
            // NetworkVarNames: m_reuseTimer (CountdownTimer)
            // NetworkVarNames: m_bHandsHaveBeenCut (bool)
            // NetworkVarNames: m_hHostageGrabber (CHandle<CCSPlayerPawn>)
            // NetworkVarNames: m_flRescueStartTime (GameTime_t)
            // NetworkVarNames: m_flGrabSuccessTime (GameTime_t)
            // NetworkVarNames: m_flDropStartTime (GameTime_t)
            pub mod CHostage {
                pub const m_OnHostageBeginGrab: usize = 0xD30; // CEntityIOOutput
                pub const m_OnFirstPickedUp: usize = 0xD58; // CEntityIOOutput
                pub const m_OnDroppedNotRescued: usize = 0xD80; // CEntityIOOutput
                pub const m_OnRescued: usize = 0xDA8; // CEntityIOOutput
                pub const m_entitySpottedState: usize = 0xDD0; // EntitySpottedState_t
                pub const m_nSpotRules: usize = 0xDE8; // int32
                pub const m_uiHostageSpawnExclusionGroupMask: usize = 0xDEC; // uint32
                pub const m_nHostageSpawnRandomFactor: usize = 0xDF0; // uint32
                pub const m_bRemove: usize = 0xDF4; // bool
                pub const m_vel: usize = 0xDF8; // Vector
                pub const m_isRescued: usize = 0xE04; // bool
                pub const m_jumpedThisFrame: usize = 0xE05; // bool
                pub const m_nHostageState: usize = 0xE08; // int32
                pub const m_leader: usize = 0xE0C; // CHandle<CBaseEntity>
                pub const m_lastLeader: usize = 0xE10; // CHandle<CCSPlayerPawnBase>
                pub const m_reuseTimer: usize = 0xE18; // CountdownTimer
                pub const m_hasBeenUsed: usize = 0xE30; // bool
                pub const m_accel: usize = 0xE34; // Vector
                pub const m_isRunning: usize = 0xE40; // bool
                pub const m_isCrouching: usize = 0xE41; // bool
                pub const m_jumpTimer: usize = 0xE48; // CountdownTimer
                pub const m_isWaitingForLeader: usize = 0xE60; // bool
                pub const m_repathTimer: usize = 0x2E70; // CountdownTimer
                pub const m_inhibitDoorTimer: usize = 0x2E88; // CountdownTimer
                pub const m_inhibitObstacleAvoidanceTimer: usize = 0x2F18; // CountdownTimer
                pub const m_wiggleTimer: usize = 0x2F38; // CountdownTimer
                pub const m_isAdjusted: usize = 0x2F54; // bool
                pub const m_bHandsHaveBeenCut: usize = 0x2F55; // bool
                pub const m_hHostageGrabber: usize = 0x2F58; // CHandle<CCSPlayerPawn>
                pub const m_fLastGrabTime: usize = 0x2F5C; // GameTime_t
                pub const m_vecPositionWhenStartedDroppingToGround: usize = 0x2F60; // Vector
                pub const m_vecGrabbedPos: usize = 0x2F6C; // Vector
                pub const m_flRescueStartTime: usize = 0x2F78; // GameTime_t
                pub const m_flGrabSuccessTime: usize = 0x2F7C; // GameTime_t
                pub const m_flDropStartTime: usize = 0x2F80; // GameTime_t
                pub const m_nApproachRewardPayouts: usize = 0x2F84; // int32
                pub const m_nPickupEventCount: usize = 0x2F88; // int32
                pub const m_vecSpawnGroundPos: usize = 0x2F8C; // Vector
                pub const m_vecHostageResetPosition: usize = 0x2FAC; // Vector
            }
        }
    }
}