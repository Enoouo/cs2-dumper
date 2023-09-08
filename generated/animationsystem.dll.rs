#[allow(non_snake_case, non_upper_case_globals)]
pub mod AimMatrixOpFixedSettings_t {
    pub const m_attachment: usize = 0x0;
    pub const m_damping: usize = 0x80;
    pub const m_poseCacheHandles: usize = 0x90;
    pub const m_eBlendMode: usize = 0xb8;
    pub const m_fAngleIncrement: usize = 0xbc;
    pub const m_nSequenceMaxFrame: usize = 0xc0;
    pub const m_nBoneMaskIndex: usize = 0xc4;
    pub const m_bTargetIsPosition: usize = 0xc8;
}

#[allow(non_snake_case, non_upper_case_globals)]
pub mod AnimComponentID {
    pub const m_id: usize = 0x0;
}

#[allow(non_snake_case, non_upper_case_globals)]
pub mod AnimNodeID {
    pub const m_id: usize = 0x0;
}

#[allow(non_snake_case, non_upper_case_globals)]
pub mod AnimNodeOutputID {
    pub const m_id: usize = 0x0;
}

#[allow(non_snake_case, non_upper_case_globals)]
pub mod AnimParamID {
    pub const m_id: usize = 0x0;
}

#[allow(non_snake_case, non_upper_case_globals)]
pub mod AnimScriptHandle {
    pub const m_id: usize = 0x0;
}

#[allow(non_snake_case, non_upper_case_globals)]
pub mod AnimStateID {
    pub const m_id: usize = 0x0;
}

#[allow(non_snake_case, non_upper_case_globals)]
pub mod AnimTagID {
    pub const m_id: usize = 0x0;
}

#[allow(non_snake_case, non_upper_case_globals)]
pub mod AnimationDecodeDebugDumpElement_t {
    pub const m_nEntityIndex: usize = 0x0;
    pub const m_modelName: usize = 0x8;
    pub const m_poseParams: usize = 0x10;
    pub const m_decodeOps: usize = 0x28;
    pub const m_internalOps: usize = 0x40;
    pub const m_decodedAnims: usize = 0x58;
}

#[allow(non_snake_case, non_upper_case_globals)]
pub mod AnimationDecodeDebugDump_t {
    pub const m_processingType: usize = 0x0;
    pub const m_elems: usize = 0x8;
}

#[allow(non_snake_case, non_upper_case_globals)]
pub mod AnimationSnapshotBase_t {
    pub const m_flRealTime: usize = 0x0;
    pub const m_rootToWorld: usize = 0x10;
    pub const m_bBonesInWorldSpace: usize = 0x40;
    pub const m_boneSetupMask: usize = 0x48;
    pub const m_boneTransforms: usize = 0x60;
    pub const m_flexControllers: usize = 0x78;
    pub const m_SnapshotType: usize = 0x90;
    pub const m_bHasDecodeDump: usize = 0x94;
    pub const m_DecodeDump: usize = 0x98;
}

#[allow(non_snake_case, non_upper_case_globals)]
pub mod AnimationSnapshot_t {
    pub const m_nEntIndex: usize = 0x110;
    pub const m_modelName: usize = 0x118;
}

#[allow(non_snake_case, non_upper_case_globals)]
pub mod AttachmentHandle_t {
    pub const m_Value: usize = 0x0;
}

#[allow(non_snake_case, non_upper_case_globals)]
pub mod BlendItem_t {
    pub const m_tags: usize = 0x0;
    pub const m_pChild: usize = 0x18;
    pub const m_hSequence: usize = 0x28;
    pub const m_vPos: usize = 0x2c;
    pub const m_flDuration: usize = 0x34;
    pub const m_bUseCustomDuration: usize = 0x38;
}

#[allow(non_snake_case, non_upper_case_globals)]
pub mod BoneDemoCaptureSettings_t {
    pub const m_boneName: usize = 0x0;
}

#[allow(non_snake_case, non_upper_case_globals)]
pub mod CActionComponentUpdater {
    pub const m_actions: usize = 0x30;
}

#[allow(non_snake_case, non_upper_case_globals)]
pub mod CAddUpdateNode {
    pub const m_footMotionTiming: usize = 0x8c;
    pub const m_bApplyToFootMotion: usize = 0x90;
    pub const m_bApplyChannelsSeparately: usize = 0x91;
    pub const m_bUseModelSpace: usize = 0x92;
}

#[allow(non_snake_case, non_upper_case_globals)]
pub mod CAimConstraint {
    pub const m_qAimOffset: usize = 0x70;
    pub const m_nUpType: usize = 0x80;
}

#[allow(non_snake_case, non_upper_case_globals)]
pub mod CAimMatrixUpdateNode {
    pub const m_opFixedSettings: usize = 0x70;
    pub const m_target: usize = 0x148;
    pub const m_paramIndex: usize = 0x14c;
    pub const m_hSequence: usize = 0x150;
    pub const m_bResetChild: usize = 0x154;
    pub const m_bLockWhenWaning: usize = 0x155;
}

#[allow(non_snake_case, non_upper_case_globals)]
pub mod CAnimActivity {
    pub const m_name: usize = 0x0;
    pub const m_nActivity: usize = 0x10;
    pub const m_nFlags: usize = 0x14;
    pub const m_nWeight: usize = 0x18;
}

#[allow(non_snake_case, non_upper_case_globals)]
pub mod CAnimAttachment {
    pub const m_influenceRotations: usize = 0x0;
    pub const m_influenceOffsets: usize = 0x30;
    pub const m_influenceIndices: usize = 0x60;
    pub const m_influenceWeights: usize = 0x6c;
    pub const m_numInfluences: usize = 0x78;
}

#[allow(non_snake_case, non_upper_case_globals)]
pub mod CAnimBone {
    pub const m_name: usize = 0x0;
    pub const m_parent: usize = 0x10;
    pub const m_pos: usize = 0x14;
    pub const m_quat: usize = 0x20;
    pub const m_scale: usize = 0x30;
    pub const m_qAlignment: usize = 0x34;
    pub const m_flags: usize = 0x44;
}

#[allow(non_snake_case, non_upper_case_globals)]
pub mod CAnimBoneDifference {
    pub const m_name: usize = 0x0;
    pub const m_parent: usize = 0x10;
    pub const m_posError: usize = 0x20;
    pub const m_bHasRotation: usize = 0x2c;
    pub const m_bHasMovement: usize = 0x2d;
}

#[allow(non_snake_case, non_upper_case_globals)]
pub mod CAnimComponentUpdater {
    pub const m_name: usize = 0x18;
    pub const m_id: usize = 0x20;
    pub const m_networkMode: usize = 0x24;
    pub const m_bStartEnabled: usize = 0x28;
}

#[allow(non_snake_case, non_upper_case_globals)]
pub mod CAnimData {
    pub const m_name: usize = 0x10;
    pub const m_animArray: usize = 0x20;
    pub const m_decoderArray: usize = 0x38;
    pub const m_nMaxUniqueFrameIndex: usize = 0x50;
    pub const m_segmentArray: usize = 0x58;
}

#[allow(non_snake_case, non_upper_case_globals)]
pub mod CAnimDataChannelDesc {
    pub const m_szChannelClass: usize = 0x0;
    pub const m_szVariableName: usize = 0x10;
    pub const m_nFlags: usize = 0x20;
    pub const m_nType: usize = 0x24;
    pub const m_szGrouping: usize = 0x28;
    pub const m_szDescription: usize = 0x38;
    pub const m_szElementNameArray: usize = 0x48;
    pub const m_nElementIndexArray: usize = 0x60;
    pub const m_nElementMaskArray: usize = 0x78;
}

#[allow(non_snake_case, non_upper_case_globals)]
pub mod CAnimDecoder {
    pub const m_szName: usize = 0x0;
    pub const m_nVersion: usize = 0x10;
    pub const m_nType: usize = 0x14;
}

#[allow(non_snake_case, non_upper_case_globals)]
pub mod CAnimDemoCaptureSettings {
    pub const m_flMaxSplineErrorRotation: usize = 0x0;
    pub const m_flMaxSplineErrorTranslation: usize = 0x4;
    pub const m_flMaxSplineErrorScale: usize = 0x8;
    pub const m_flIkRotation_MaxSplineError: usize = 0xc;
    pub const m_flIkTranslation_MaxSplineError: usize = 0x10;
    pub const m_flMaxQuantizationErrorRotation: usize = 0x14;
    pub const m_flMaxQuantizationErrorTranslation: usize = 0x18;
    pub const m_flMaxQuantizationErrorScale: usize = 0x1c;
    pub const m_flIkRotation_MaxQuantizationError: usize = 0x20;
    pub const m_flIkTranslation_MaxQuantizationError: usize = 0x24;
    pub const m_baseSequence: usize = 0x28;
    pub const m_nBaseSequenceFrame: usize = 0x30;
    pub const m_boneSelectionMode: usize = 0x34;
    pub const m_bones: usize = 0x38;
    pub const m_ikChains: usize = 0x50;
}

#[allow(non_snake_case, non_upper_case_globals)]
pub mod CAnimDesc {
    pub const m_name: usize = 0x0;
    pub const m_flags: usize = 0x10;
    pub const fps: usize = 0x18;
    pub const m_Data: usize = 0x20;
    pub const m_movementArray: usize = 0xf8;
    pub const m_eventArray: usize = 0x110;
    pub const m_activityArray: usize = 0x128;
    pub const m_hierarchyArray: usize = 0x140;
    pub const framestalltime: usize = 0x158;
    pub const m_vecRootMin: usize = 0x15c;
    pub const m_vecRootMax: usize = 0x168;
    pub const m_vecBoneWorldMin: usize = 0x178;
    pub const m_vecBoneWorldMax: usize = 0x190;
    pub const m_sequenceParams: usize = 0x1a8;
}

#[allow(non_snake_case, non_upper_case_globals)]
pub mod CAnimDesc_Flag {
    pub const m_bLooping: usize = 0x0;
    pub const m_bAllZeros: usize = 0x1;
    pub const m_bHidden: usize = 0x2;
    pub const m_bDelta: usize = 0x3;
    pub const m_bLegacyWorldspace: usize = 0x4;
    pub const m_bModelDoc: usize = 0x5;
    pub const m_bImplicitSeqIgnoreDelta: usize = 0x6;
    pub const m_bAnimGraphAdditive: usize = 0x7;
}

#[allow(non_snake_case, non_upper_case_globals)]
pub mod CAnimEncodeDifference {
    pub const m_boneArray: usize = 0x0;
    pub const m_morphArray: usize = 0x18;
    pub const m_userArray: usize = 0x30;
    pub const m_bHasRotationBitArray: usize = 0x48;
    pub const m_bHasMovementBitArray: usize = 0x60;
    pub const m_bHasMorphBitArray: usize = 0x78;
    pub const m_bHasUserBitArray: usize = 0x90;
}

#[allow(non_snake_case, non_upper_case_globals)]
pub mod CAnimEncodedFrames {
    pub const m_fileName: usize = 0x0;
    pub const m_nFrames: usize = 0x10;
    pub const m_nFramesPerBlock: usize = 0x14;
    pub const m_frameblockArray: usize = 0x18;
    pub const m_usageDifferences: usize = 0x30;
}

#[allow(non_snake_case, non_upper_case_globals)]
pub mod CAnimEnum {
    pub const m_value: usize = 0x0;
}

#[allow(non_snake_case, non_upper_case_globals)]
pub mod CAnimEventDefinition {
    pub const m_nFrame: usize = 0x8;
    pub const m_flCycle: usize = 0xc;
    pub const m_EventData: usize = 0x10;
    pub const m_sLegacyOptions: usize = 0x20;
    pub const m_sEventName: usize = 0x30;
}

#[allow(non_snake_case, non_upper_case_globals)]
pub mod CAnimFoot {
    pub const m_name: usize = 0x0;
    pub const m_vBallOffset: usize = 0x8;
    pub const m_vHeelOffset: usize = 0x14;
    pub const m_ankleBoneIndex: usize = 0x20;
    pub const m_toeBoneIndex: usize = 0x24;
}

#[allow(non_snake_case, non_upper_case_globals)]
pub mod CAnimFrameBlockAnim {
    pub const m_nStartFrame: usize = 0x0;
    pub const m_nEndFrame: usize = 0x4;
    pub const m_segmentIndexArray: usize = 0x8;
}

#[allow(non_snake_case, non_upper_case_globals)]
pub mod CAnimFrameSegment {
    pub const m_nUniqueFrameIndex: usize = 0x0;
    pub const m_nLocalElementMasks: usize = 0x4;
    pub const m_nLocalChannel: usize = 0x8;
    pub const m_container: usize = 0x10;
}

#[allow(non_snake_case, non_upper_case_globals)]
pub mod CAnimGraphDebugReplay {
    pub const m_animGraphFileName: usize = 0x40;
    pub const m_frameList: usize = 0x48;
    pub const m_startIndex: usize = 0x60;
    pub const m_writeIndex: usize = 0x64;
    pub const m_frameCount: usize = 0x68;
}

#[allow(non_snake_case, non_upper_case_globals)]
pub mod CAnimGraphModelBinding {
    pub const m_modelName: usize = 0x8;
    pub const m_pSharedData: usize = 0x10;
}

#[allow(non_snake_case, non_upper_case_globals)]
pub mod CAnimGraphNetworkSettings {
    pub const m_bNetworkingEnabled: usize = 0x20;
}

#[allow(non_snake_case, non_upper_case_globals)]
pub mod CAnimGraphSettingsManager {
    pub const m_settingsGroups: usize = 0x18;
}

#[allow(non_snake_case, non_upper_case_globals)]
pub mod CAnimInputDamping {
    pub const m_speedFunction: usize = 0x8;
    pub const m_fSpeedScale: usize = 0xc;
}

#[allow(non_snake_case, non_upper_case_globals)]
pub mod CAnimKeyData {
    pub const m_name: usize = 0x0;
    pub const m_boneArray: usize = 0x10;
    pub const m_userArray: usize = 0x28;
    pub const m_morphArray: usize = 0x40;
    pub const m_nChannelElements: usize = 0x58;
    pub const m_dataChannelArray: usize = 0x60;
}

#[allow(non_snake_case, non_upper_case_globals)]
pub mod CAnimLocalHierarchy {
    pub const m_sBone: usize = 0x0;
    pub const m_sNewParent: usize = 0x10;
    pub const m_nStartFrame: usize = 0x20;
    pub const m_nPeakFrame: usize = 0x24;
    pub const m_nTailFrame: usize = 0x28;
    pub const m_nEndFrame: usize = 0x2c;
}

#[allow(non_snake_case, non_upper_case_globals)]
pub mod CAnimMorphDifference {
    pub const m_name: usize = 0x0;
}

#[allow(non_snake_case, non_upper_case_globals)]
pub mod CAnimMotorUpdaterBase {
    pub const m_name: usize = 0x10;
    pub const m_bDefault: usize = 0x18;
}

#[allow(non_snake_case, non_upper_case_globals)]
pub mod CAnimMovement {
    pub const endframe: usize = 0x0;
    pub const motionflags: usize = 0x4;
    pub const v0: usize = 0x8;
    pub const v1: usize = 0xc;
    pub const angle: usize = 0x10;
    pub const vector: usize = 0x14;
    pub const position: usize = 0x20;
}

#[allow(non_snake_case, non_upper_case_globals)]
pub mod CAnimNodePath {
    pub const m_path: usize = 0x0;
    pub const m_nCount: usize = 0x2c;
}

#[allow(non_snake_case, non_upper_case_globals)]
pub mod CAnimParamHandle {
    pub const m_type: usize = 0x0;
    pub const m_index: usize = 0x1;
}

#[allow(non_snake_case, non_upper_case_globals)]
pub mod CAnimParamHandleMap {
    pub const m_list: usize = 0x0;
}

#[allow(non_snake_case, non_upper_case_globals)]
pub mod CAnimParameterBase {
    pub const m_name: usize = 0x18;
    pub const m_group: usize = 0x20;
    pub const m_id: usize = 0x28;
    pub const m_componentName: usize = 0x40;
    pub const m_bNetworkingRequested: usize = 0x4c;
    pub const m_bIsReferenced: usize = 0x4d;
}

#[allow(non_snake_case, non_upper_case_globals)]
pub mod CAnimParameterManagerUpdater {
    pub const m_parameters: usize = 0x18;
    pub const m_idToIndexMap: usize = 0x30;
    pub const m_nameToIndexMap: usize = 0x50;
    pub const m_indexToHandle: usize = 0x70;
    pub const m_autoResetParams: usize = 0x88;
    pub const m_autoResetMap: usize = 0xa0;
}

#[allow(non_snake_case, non_upper_case_globals)]
pub mod CAnimReplayFrame {
    pub const m_inputDataBlocks: usize = 0x10;
    pub const m_instanceData: usize = 0x28;
    pub const m_startingLocalToWorldTransform: usize = 0x40;
    pub const m_localToWorldTransform: usize = 0x60;
    pub const m_timeStamp: usize = 0x80;
}

#[allow(non_snake_case, non_upper_case_globals)]
pub mod CAnimScriptComponentUpdater {
    pub const m_hScript: usize = 0x30;
}

#[allow(non_snake_case, non_upper_case_globals)]
pub mod CAnimScriptManager {
    pub const m_scriptInfo: usize = 0x10;
}

#[allow(non_snake_case, non_upper_case_globals)]
pub mod CAnimSequenceParams {
    pub const m_flFadeInTime: usize = 0x0;
    pub const m_flFadeOutTime: usize = 0x4;
}

#[allow(non_snake_case, non_upper_case_globals)]
pub mod CAnimSkeleton {
    pub const m_localSpaceTransforms: usize = 0x10;
    pub const m_modelSpaceTransforms: usize = 0x28;
    pub const m_boneNames: usize = 0x40;
    pub const m_children: usize = 0x58;
    pub const m_parents: usize = 0x70;
    pub const m_feet: usize = 0x88;
    pub const m_morphNames: usize = 0xa0;
    pub const m_lodBoneCounts: usize = 0xb8;
}

#[allow(non_snake_case, non_upper_case_globals)]
pub mod CAnimStateMachineUpdater {
    pub const m_states: usize = 0x8;
    pub const m_transitions: usize = 0x20;
    pub const m_startStateIndex: usize = 0x50;
}

#[allow(non_snake_case, non_upper_case_globals)]
pub mod CAnimTagBase {
    pub const m_name: usize = 0x18;
    pub const m_group: usize = 0x20;
    pub const m_tagID: usize = 0x28;
    pub const m_bIsReferenced: usize = 0x2c;
}

#[allow(non_snake_case, non_upper_case_globals)]
pub mod CAnimTagManagerUpdater {
    pub const m_tags: usize = 0x18;
}

#[allow(non_snake_case, non_upper_case_globals)]
pub mod CAnimUpdateNodeBase {
    pub const m_nodePath: usize = 0x18;
    pub const m_name: usize = 0x48;
    pub const m_networkMode: usize = 0x50;
}

#[allow(non_snake_case, non_upper_case_globals)]
pub mod CAnimUpdateNodeRef {
    pub const m_nodeIndex: usize = 0x8;
}

#[allow(non_snake_case, non_upper_case_globals)]
pub mod CAnimUpdateSharedData {
    pub const m_nodes: usize = 0x10;
    pub const m_nodeIndexMap: usize = 0x28;
    pub const m_components: usize = 0x48;
    pub const m_pParamListUpdater: usize = 0x60;
    pub const m_pTagManagerUpdater: usize = 0x68;
    pub const m_scriptManager: usize = 0x70;
    pub const m_settings: usize = 0x78;
    pub const m_pStaticPoseCache: usize = 0xa8;
    pub const m_pSkeleton: usize = 0xb0;
    pub const m_rootNodePath: usize = 0xb8;
}

#[allow(non_snake_case, non_upper_case_globals)]
pub mod CAnimUser {
    pub const m_name: usize = 0x0;
    pub const m_nType: usize = 0x10;
}

#[allow(non_snake_case, non_upper_case_globals)]
pub mod CAnimUserDifference {
    pub const m_name: usize = 0x0;
    pub const m_nType: usize = 0x10;
}

#[allow(non_snake_case, non_upper_case_globals)]
pub mod CAnimationGraphVisualizerAxis {
    pub const m_xWsTransform: usize = 0x40;
    pub const m_flAxisSize: usize = 0x60;
}

#[allow(non_snake_case, non_upper_case_globals)]
pub mod CAnimationGraphVisualizerLine {
    pub const m_vWsPositionStart: usize = 0x40;
    pub const m_vWsPositionEnd: usize = 0x50;
    pub const m_Color: usize = 0x60;
}

#[allow(non_snake_case, non_upper_case_globals)]
pub mod CAnimationGraphVisualizerPie {
    pub const m_vWsCenter: usize = 0x40;
    pub const m_vWsStart: usize = 0x50;
    pub const m_vWsEnd: usize = 0x60;
    pub const m_Color: usize = 0x70;
}

#[allow(non_snake_case, non_upper_case_globals)]
pub mod CAnimationGraphVisualizerPrimitiveBase {
    pub const m_Type: usize = 0x8;
    pub const m_OwningAnimNodePaths: usize = 0xc;
    pub const m_nOwningAnimNodePathCount: usize = 0x38;
}

#[allow(non_snake_case, non_upper_case_globals)]
pub mod CAnimationGraphVisualizerSphere {
    pub const m_vWsPosition: usize = 0x40;
    pub const m_flRadius: usize = 0x50;
    pub const m_Color: usize = 0x54;
}

#[allow(non_snake_case, non_upper_case_globals)]
pub mod CAnimationGraphVisualizerText {
    pub const m_vWsPosition: usize = 0x40;
    pub const m_Color: usize = 0x50;
    pub const m_Text: usize = 0x58;
}

#[allow(non_snake_case, non_upper_case_globals)]
pub mod CAnimationGroup {
    pub const m_nFlags: usize = 0x10;
    pub const m_name: usize = 0x18;
    pub const m_localHAnimArray_Handle: usize = 0x60;
    pub const m_includedGroupArray_Handle: usize = 0x78;
    pub const m_directHSeqGroup_Handle: usize = 0x90;
    pub const m_decodeKey: usize = 0x98;
    pub const m_szScripts: usize = 0x110;
}

#[allow(non_snake_case, non_upper_case_globals)]
pub mod CAttachment {
    pub const m_name: usize = 0x0;
    pub const m_influenceNames: usize = 0x8;
    pub const m_vInfluenceRotations: usize = 0x20;
    pub const m_vInfluenceOffsets: usize = 0x50;
    pub const m_influenceWeights: usize = 0x74;
    pub const m_bInfluenceRootTransform: usize = 0x80;
    pub const m_nInfluences: usize = 0x83;
    pub const m_bIgnoreRotation: usize = 0x84;
}

#[allow(non_snake_case, non_upper_case_globals)]
pub mod CAudioAnimTag {
    pub const m_clipName: usize = 0x38;
    pub const m_attachmentName: usize = 0x40;
    pub const m_flVolume: usize = 0x48;
    pub const m_bStopWhenTagEnds: usize = 0x4c;
    pub const m_bStopWhenGraphEnds: usize = 0x4d;
    pub const m_bPlayOnServer: usize = 0x4e;
    pub const m_bPlayOnClient: usize = 0x4f;
}

#[allow(non_snake_case, non_upper_case_globals)]
pub mod CBaseConstraint {
    pub const m_name: usize = 0x28;
    pub const m_vUpVector: usize = 0x30;
    pub const m_slaves: usize = 0x40;
    pub const m_targets: usize = 0x58;
}

#[allow(non_snake_case, non_upper_case_globals)]
pub mod CBinaryUpdateNode {
    pub const m_pChild1: usize = 0x58;
    pub const m_pChild2: usize = 0x68;
    pub const m_timingBehavior: usize = 0x78;
    pub const m_flTimingBlend: usize = 0x7c;
    pub const m_bResetChild1: usize = 0x80;
    pub const m_bResetChild2: usize = 0x81;
}

#[allow(non_snake_case, non_upper_case_globals)]
pub mod CBlend2DUpdateNode {
    pub const m_items: usize = 0x60;
    pub const m_tags: usize = 0x78;
    pub const m_paramSpans: usize = 0x90;
    pub const m_nodeItemIndices: usize = 0xa8;
    pub const m_damping: usize = 0xc0;
    pub const m_blendSourceX: usize = 0xd0;
    pub const m_paramX: usize = 0xd4;
    pub const m_blendSourceY: usize = 0xd8;
    pub const m_paramY: usize = 0xdc;
    pub const m_eBlendMode: usize = 0xe0;
    pub const m_playbackSpeed: usize = 0xe4;
    pub const m_bLoop: usize = 0xe8;
    pub const m_bLockBlendOnReset: usize = 0xe9;
    pub const m_bLockWhenWaning: usize = 0xea;
    pub const m_bAnimEventsAndTagsOnMostWeightedOnly: usize = 0xeb;
}

#[allow(non_snake_case, non_upper_case_globals)]
pub mod CBlendCurve {
    pub const m_flControlPoint1: usize = 0x0;
    pub const m_flControlPoint2: usize = 0x4;
}

#[allow(non_snake_case, non_upper_case_globals)]
pub mod CBlendUpdateNode {
    pub const m_children: usize = 0x60;
    pub const m_sortedOrder: usize = 0x78;
    pub const m_targetValues: usize = 0x90;
    pub const m_blendValueSource: usize = 0xac;
    pub const m_paramIndex: usize = 0xb0;
    pub const m_damping: usize = 0xb8;
    pub const m_blendKeyType: usize = 0xc8;
    pub const m_bLockBlendOnReset: usize = 0xcc;
    pub const m_bSyncCycles: usize = 0xcd;
    pub const m_bLoop: usize = 0xce;
    pub const m_bLockWhenWaning: usize = 0xcf;
}

#[allow(non_snake_case, non_upper_case_globals)]
pub mod CBodyGroupAnimTag {
    pub const m_nPriority: usize = 0x38;
    pub const m_bodyGroupSettings: usize = 0x40;
}

#[allow(non_snake_case, non_upper_case_globals)]
pub mod CBodyGroupSetting {
    pub const m_BodyGroupName: usize = 0x0;
    pub const m_nBodyGroupOption: usize = 0x8;
}

#[allow(non_snake_case, non_upper_case_globals)]
pub mod CBoneConstraintDotToMorph {
    pub const m_sBoneName: usize = 0x28;
    pub const m_sTargetBoneName: usize = 0x30;
    pub const m_sMorphChannelName: usize = 0x38;
    pub const m_flRemap: usize = 0x40;
}

#[allow(non_snake_case, non_upper_case_globals)]
pub mod CBoneConstraintPoseSpaceBone {
    pub const m_inputList: usize = 0x70;
}

#[allow(non_snake_case, non_upper_case_globals)]
pub mod CBoneConstraintPoseSpaceBone_Input_t {
    pub const m_inputValue: usize = 0x0;
    pub const m_outputTransformList: usize = 0x10;
}

#[allow(non_snake_case, non_upper_case_globals)]
pub mod CBoneConstraintPoseSpaceMorph_Input_t {
    pub const m_inputValue: usize = 0x0;
    pub const m_outputWeightList: usize = 0x10;
}

#[allow(non_snake_case, non_upper_case_globals)]
pub mod CBoneMaskUpdateNode {
    pub const m_nWeightListIndex: usize = 0x8c;
    pub const m_flRootMotionBlend: usize = 0x90;
    pub const m_blendSpace: usize = 0x94;
    pub const m_footMotionTiming: usize = 0x98;
    pub const m_bUseBlendScale: usize = 0x9c;
    pub const m_blendValueSource: usize = 0xa0;
    pub const m_hBlendParameter: usize = 0xa4;
}

#[allow(non_snake_case, non_upper_case_globals)]
pub mod CBonePositionMetricEvaluator {
    pub const m_nBoneIndex: usize = 0x50;
}

#[allow(non_snake_case, non_upper_case_globals)]
pub mod CBoneVelocityMetricEvaluator {
    pub const m_nBoneIndex: usize = 0x50;
}

#[allow(non_snake_case, non_upper_case_globals)]
pub mod CBoolAnimParameter {
    pub const m_bDefaultValue: usize = 0x60;
}

#[allow(non_snake_case, non_upper_case_globals)]
pub mod CCPPScriptComponentUpdater {
    pub const m_scriptsToRun: usize = 0x30;
}

#[allow(non_snake_case, non_upper_case_globals)]
pub mod CCachedPose {
    pub const m_transforms: usize = 0x8;
    pub const m_morphWeights: usize = 0x20;
    pub const m_hSequence: usize = 0x38;
    pub const m_flCycle: usize = 0x3c;
}

#[allow(non_snake_case, non_upper_case_globals)]
pub mod CChoiceUpdateNode {
    pub const m_children: usize = 0x58;
    pub const m_weights: usize = 0x70;
    pub const m_blendTimes: usize = 0x88;
    pub const m_choiceMethod: usize = 0xa0;
    pub const m_choiceChangeMethod: usize = 0xa4;
    pub const m_blendMethod: usize = 0xa8;
    pub const m_blendTime: usize = 0xac;
    pub const m_bCrossFade: usize = 0xb0;
    pub const m_bResetChosen: usize = 0xb1;
    pub const m_bDontResetSameSelection: usize = 0xb2;
}

#[allow(non_snake_case, non_upper_case_globals)]
pub mod CClothSettingsAnimTag {
    pub const m_flStiffness: usize = 0x38;
    pub const m_flEaseIn: usize = 0x3c;
    pub const m_flEaseOut: usize = 0x40;
    pub const m_nVertexSet: usize = 0x48;
}

#[allow(non_snake_case, non_upper_case_globals)]
pub mod CCompressorGroup {
    pub const m_nTotalElementCount: usize = 0x0;
    pub const m_szChannelClass: usize = 0x8;
    pub const m_szVariableName: usize = 0x20;
    pub const m_nType: usize = 0x38;
    pub const m_nFlags: usize = 0x50;
    pub const m_szGrouping: usize = 0x68;
    pub const m_nCompressorIndex: usize = 0x80;
    pub const m_szElementNames: usize = 0x98;
    pub const m_nElementUniqueID: usize = 0xb0;
    pub const m_nElementMask: usize = 0xc8;
    pub const m_vectorCompressor: usize = 0xf8;
    pub const m_quaternionCompressor: usize = 0x110;
    pub const m_intCompressor: usize = 0x128;
    pub const m_boolCompressor: usize = 0x140;
    pub const m_colorCompressor: usize = 0x158;
    pub const m_vector2DCompressor: usize = 0x170;
    pub const m_vector4DCompressor: usize = 0x188;
}

#[allow(non_snake_case, non_upper_case_globals)]
pub mod CConcreteAnimParameter {
    pub const m_previewButton: usize = 0x50;
    pub const m_eNetworkSetting: usize = 0x54;
    pub const m_bUseMostRecentValue: usize = 0x58;
    pub const m_bAutoReset: usize = 0x59;
    pub const m_bGameWritable: usize = 0x5a;
    pub const m_bGraphWritable: usize = 0x5b;
}

#[allow(non_snake_case, non_upper_case_globals)]
pub mod CConstraintSlave {
    pub const m_qBaseOrientation: usize = 0x0;
    pub const m_vBasePosition: usize = 0x10;
    pub const m_nBoneHash: usize = 0x1c;
    pub const m_flWeight: usize = 0x20;
    pub const m_sName: usize = 0x28;
}

#[allow(non_snake_case, non_upper_case_globals)]
pub mod CConstraintTarget {
    pub const m_qOffset: usize = 0x20;
    pub const m_vOffset: usize = 0x30;
    pub const m_nBoneHash: usize = 0x3c;
    pub const m_sName: usize = 0x40;
    pub const m_flWeight: usize = 0x48;
    pub const m_bIsAttachment: usize = 0x59;
}

#[allow(non_snake_case, non_upper_case_globals)]
pub mod CCycleBase {
    pub const m_flCycle: usize = 0x0;
}

#[allow(non_snake_case, non_upper_case_globals)]
pub mod CCycleControlClipUpdateNode {
    pub const m_tags: usize = 0x60;
    pub const m_hSequence: usize = 0x7c;
    pub const m_duration: usize = 0x80;
    pub const m_valueSource: usize = 0x84;
    pub const m_paramIndex: usize = 0x88;
}

#[allow(non_snake_case, non_upper_case_globals)]
pub mod CCycleControlUpdateNode {
    pub const m_valueSource: usize = 0x68;
    pub const m_paramIndex: usize = 0x6c;
}

#[allow(non_snake_case, non_upper_case_globals)]
pub mod CDampedPathAnimMotorUpdater {
    pub const m_flAnticipationTime: usize = 0x2c;
    pub const m_flMinSpeedScale: usize = 0x30;
    pub const m_hAnticipationPosParam: usize = 0x34;
    pub const m_hAnticipationHeadingParam: usize = 0x36;
    pub const m_flSpringConstant: usize = 0x38;
    pub const m_flMinSpringTension: usize = 0x3c;
    pub const m_flMaxSpringTension: usize = 0x40;
}

#[allow(non_snake_case, non_upper_case_globals)]
pub mod CDampedValueComponentUpdater {
    pub const m_items: usize = 0x30;
}

#[allow(non_snake_case, non_upper_case_globals)]
pub mod CDampedValueUpdateItem {
    pub const m_damping: usize = 0x0;
    pub const m_hParamIn: usize = 0x18;
    pub const m_hParamOut: usize = 0x1a;
}

#[allow(non_snake_case, non_upper_case_globals)]
pub mod CDemoSettingsComponentUpdater {
    pub const m_settings: usize = 0x30;
}

#[allow(non_snake_case, non_upper_case_globals)]
pub mod CDirectPlaybackTagData {
    pub const m_sequenceName: usize = 0x0;
    pub const m_tags: usize = 0x8;
}

#[allow(non_snake_case, non_upper_case_globals)]
pub mod CDirectPlaybackUpdateNode {
    pub const m_bFinishEarly: usize = 0x6c;
    pub const m_bResetOnFinish: usize = 0x6d;
    pub const m_allTags: usize = 0x70;
}

#[allow(non_snake_case, non_upper_case_globals)]
pub mod CDirectionalBlendUpdateNode {
    pub const m_hSequences: usize = 0x5c;
    pub const m_damping: usize = 0x80;
    pub const m_blendValueSource: usize = 0x90;
    pub const m_paramIndex: usize = 0x94;
    pub const m_playbackSpeed: usize = 0x98;
    pub const m_duration: usize = 0x9c;
    pub const m_bLoop: usize = 0xa0;
    pub const m_bLockBlendOnReset: usize = 0xa1;
}

#[allow(non_snake_case, non_upper_case_globals)]
pub mod CDistanceRemainingMetricEvaluator {
    pub const m_flMaxDistance: usize = 0x50;
    pub const m_flMinDistance: usize = 0x54;
    pub const m_flStartGoalFilterDistance: usize = 0x58;
    pub const m_flMaxGoalOvershootScale: usize = 0x5c;
    pub const m_bFilterFixedMinDistance: usize = 0x60;
    pub const m_bFilterGoalDistance: usize = 0x61;
    pub const m_bFilterGoalOvershoot: usize = 0x62;
}

#[allow(non_snake_case, non_upper_case_globals)]
pub mod CDrawCullingData {
    pub const m_vConeApex: usize = 0x0;
    pub const m_ConeAxis: usize = 0xc;
    pub const m_ConeCutoff: usize = 0xf;
}

#[allow(non_snake_case, non_upper_case_globals)]
pub mod CEmitTagActionUpdater {
    pub const m_nTagIndex: usize = 0x18;
    pub const m_bIsZeroDuration: usize = 0x1c;
}

#[allow(non_snake_case, non_upper_case_globals)]
pub mod CEnumAnimParameter {
    pub const m_defaultValue: usize = 0x68;
    pub const m_enumOptions: usize = 0x70;
}

#[allow(non_snake_case, non_upper_case_globals)]
pub mod CExpressionActionUpdater {
    pub const m_hParam: usize = 0x18;
    pub const m_eParamType: usize = 0x1a;
    pub const m_hScript: usize = 0x1c;
}

#[allow(non_snake_case, non_upper_case_globals)]
pub mod CFingerBone {
    pub const m_boneName: usize = 0x0;
    pub const m_hingeAxis: usize = 0x8;
    pub const m_vCapsulePos1: usize = 0x14;
    pub const m_vCapsulePos2: usize = 0x20;
    pub const m_flMinAngle: usize = 0x2c;
    pub const m_flMaxAngle: usize = 0x30;
    pub const m_flRadius: usize = 0x34;
}

#[allow(non_snake_case, non_upper_case_globals)]
pub mod CFingerChain {
    pub const m_targets: usize = 0x0;
    pub const m_bones: usize = 0x18;
    pub const m_name: usize = 0x30;
    pub const m_tipParentBoneName: usize = 0x38;
    pub const m_vTipOffset: usize = 0x40;
    pub const m_metacarpalBoneName: usize = 0x50;
    pub const m_vSplayHingeAxis: usize = 0x58;
    pub const m_flSplayMinAngle: usize = 0x64;
    pub const m_flSplayMaxAngle: usize = 0x68;
    pub const m_flFingerScaleRatio: usize = 0x6c;
}

#[allow(non_snake_case, non_upper_case_globals)]
pub mod CFingerSource {
    pub const m_nFingerIndex: usize = 0x0;
    pub const m_flFingerWeight: usize = 0x4;
}

#[allow(non_snake_case, non_upper_case_globals)]
pub mod CFlexController {
    pub const m_szName: usize = 0x0;
    pub const m_szType: usize = 0x8;
    pub const min: usize = 0x10;
    pub const max: usize = 0x14;
}

#[allow(non_snake_case, non_upper_case_globals)]
pub mod CFlexDesc {
    pub const m_szFacs: usize = 0x0;
}

#[allow(non_snake_case, non_upper_case_globals)]
pub mod CFlexOp {
    pub const m_OpCode: usize = 0x0;
    pub const m_Data: usize = 0x4;
}

#[allow(non_snake_case, non_upper_case_globals)]
pub mod CFlexRule {
    pub const m_nFlex: usize = 0x0;
    pub const m_FlexOps: usize = 0x8;
}

#[allow(non_snake_case, non_upper_case_globals)]
pub mod CFloatAnimParameter {
    pub const m_fDefaultValue: usize = 0x60;
    pub const m_fMinValue: usize = 0x64;
    pub const m_fMaxValue: usize = 0x68;
    pub const m_bInterpolate: usize = 0x6c;
}

#[allow(non_snake_case, non_upper_case_globals)]
pub mod CFollowAttachmentUpdateNode {
    pub const m_opFixedData: usize = 0x70;
}

#[allow(non_snake_case, non_upper_case_globals)]
pub mod CFollowPathUpdateNode {
    pub const m_flBlendOutTime: usize = 0x6c;
    pub const m_bBlockNonPathMovement: usize = 0x70;
    pub const m_bStopFeetAtGoal: usize = 0x71;
    pub const m_bScaleSpeed: usize = 0x72;
    pub const m_flScale: usize = 0x74;
    pub const m_flMinAngle: usize = 0x78;
    pub const m_flMaxAngle: usize = 0x7c;
    pub const m_flSpeedScaleBlending: usize = 0x80;
    pub const m_turnDamping: usize = 0x88;
    pub const m_facingTarget: usize = 0x98;
    pub const m_hParam: usize = 0x9c;
    pub const m_flTurnToFaceOffset: usize = 0xa0;
    pub const m_bTurnToFace: usize = 0xa4;
}

#[allow(non_snake_case, non_upper_case_globals)]
pub mod CFootAdjustmentUpdateNode {
    pub const m_clips: usize = 0x70;
    pub const m_hBasePoseCacheHandle: usize = 0x88;
    pub const m_facingTarget: usize = 0x8c;
    pub const m_flTurnTimeMin: usize = 0x90;
    pub const m_flTurnTimeMax: usize = 0x94;
    pub const m_flStepHeightMax: usize = 0x98;
    pub const m_flStepHeightMaxAngle: usize = 0x9c;
    pub const m_bResetChild: usize = 0xa0;
    pub const m_bAnimationDriven: usize = 0xa1;
}

#[allow(non_snake_case, non_upper_case_globals)]
pub mod CFootCycleDefinition {
    pub const m_vStancePositionMS: usize = 0x0;
    pub const m_vMidpointPositionMS: usize = 0xc;
    pub const m_flStanceDirectionMS: usize = 0x18;
    pub const m_vToStrideStartPos: usize = 0x1c;
    pub const m_stanceCycle: usize = 0x28;
    pub const m_footLiftCycle: usize = 0x2c;
    pub const m_footOffCycle: usize = 0x30;
    pub const m_footStrikeCycle: usize = 0x34;
    pub const m_footLandCycle: usize = 0x38;
}

#[allow(non_snake_case, non_upper_case_globals)]
pub mod CFootCycleMetricEvaluator {
    pub const m_footIndices: usize = 0x50;
}

#[allow(non_snake_case, non_upper_case_globals)]
pub mod CFootDefinition {
    pub const m_name: usize = 0x0;
    pub const m_ankleBoneName: usize = 0x8;
    pub const m_toeBoneName: usize = 0x10;
    pub const m_vBallOffset: usize = 0x18;
    pub const m_vHeelOffset: usize = 0x24;
    pub const m_flFootLength: usize = 0x30;
    pub const m_flBindPoseDirectionMS: usize = 0x34;
    pub const m_flTraceHeight: usize = 0x38;
    pub const m_flTraceRadius: usize = 0x3c;
}

#[allow(non_snake_case, non_upper_case_globals)]
pub mod CFootFallAnimTag {
    pub const m_foot: usize = 0x38;
}

#[allow(non_snake_case, non_upper_case_globals)]
pub mod CFootLockUpdateNode {
    pub const m_opFixedSettings: usize = 0x68;
    pub const m_footSettings: usize = 0xd0;
    pub const m_hipShiftDamping: usize = 0xe8;
    pub const m_rootHeightDamping: usize = 0xf8;
    pub const m_flStrideCurveScale: usize = 0x108;
    pub const m_flStrideCurveLimitScale: usize = 0x10c;
    pub const m_flStepHeightIncreaseScale: usize = 0x110;
    pub const m_flStepHeightDecreaseScale: usize = 0x114;
    pub const m_flHipShiftScale: usize = 0x118;
    pub const m_flBlendTime: usize = 0x11c;
    pub const m_flMaxRootHeightOffset: usize = 0x120;
    pub const m_flMinRootHeightOffset: usize = 0x124;
    pub const m_flTiltPlanePitchSpringStrength: usize = 0x128;
    pub const m_flTiltPlaneRollSpringStrength: usize = 0x12c;
    pub const m_bApplyFootRotationLimits: usize = 0x130;
    pub const m_bApplyHipShift: usize = 0x131;
    pub const m_bModulateStepHeight: usize = 0x132;
    pub const m_bResetChild: usize = 0x133;
    pub const m_bEnableVerticalCurvedPaths: usize = 0x134;
    pub const m_bEnableRootHeightDamping: usize = 0x135;
}

#[allow(non_snake_case, non_upper_case_globals)]
pub mod CFootMotion {
    pub const m_strides: usize = 0x0;
    pub const m_name: usize = 0x18;
    pub const m_bAdditive: usize = 0x20;
}

#[allow(non_snake_case, non_upper_case_globals)]
pub mod CFootPinningUpdateNode {
    pub const m_poseOpFixedData: usize = 0x70;
    pub const m_eTimingSource: usize = 0xa0;
    pub const m_params: usize = 0xa8;
    pub const m_bResetChild: usize = 0xc0;
}

#[allow(non_snake_case, non_upper_case_globals)]
pub mod CFootPositionMetricEvaluator {
    pub const m_footIndices: usize = 0x50;
    pub const m_bIgnoreSlope: usize = 0x68;
}

#[allow(non_snake_case, non_upper_case_globals)]
pub mod CFootStepTriggerUpdateNode {
    pub const m_triggers: usize = 0x68;
    pub const m_flTolerance: usize = 0x84;
}

#[allow(non_snake_case, non_upper_case_globals)]
pub mod CFootStride {
    pub const m_definition: usize = 0x0;
    pub const m_trajectories: usize = 0x40;
}

#[allow(non_snake_case, non_upper_case_globals)]
pub mod CFootTrajectories {
    pub const m_trajectories: usize = 0x0;
}

#[allow(non_snake_case, non_upper_case_globals)]
pub mod CFootTrajectory {
    pub const m_vOffset: usize = 0x0;
    pub const m_flRotationOffset: usize = 0xc;
    pub const m_flProgression: usize = 0x10;
}

#[allow(non_snake_case, non_upper_case_globals)]
pub mod CFootstepLandedAnimTag {
    pub const m_FootstepType: usize = 0x38;
    pub const m_OverrideSoundName: usize = 0x40;
    pub const m_DebugAnimSourceString: usize = 0x48;
    pub const m_BoneName: usize = 0x50;
}

#[allow(non_snake_case, non_upper_case_globals)]
pub mod CFutureFacingMetricEvaluator {
    pub const m_flDistance: usize = 0x50;
    pub const m_flTime: usize = 0x54;
}

#[allow(non_snake_case, non_upper_case_globals)]
pub mod CFutureVelocityMetricEvaluator {
    pub const m_flDistance: usize = 0x50;
    pub const m_flStoppingDistance: usize = 0x54;
    pub const m_flTargetSpeed: usize = 0x58;
    pub const m_eMode: usize = 0x5c;
}

#[allow(non_snake_case, non_upper_case_globals)]
pub mod CHitBox {
    pub const m_name: usize = 0x0;
    pub const m_sSurfaceProperty: usize = 0x8;
    pub const m_sBoneName: usize = 0x10;
    pub const m_vMinBounds: usize = 0x18;
    pub const m_vMaxBounds: usize = 0x24;
    pub const m_flShapeRadius: usize = 0x30;
    pub const m_nBoneNameHash: usize = 0x34;
    pub const m_nGroupId: usize = 0x38;
    pub const m_nShapeType: usize = 0x3c;
    pub const m_bTranslationOnly: usize = 0x3d;
    pub const m_CRC: usize = 0x40;
    pub const m_cRenderColor: usize = 0x44;
    pub const m_nHitBoxIndex: usize = 0x48;
}

#[allow(non_snake_case, non_upper_case_globals)]
pub mod CHitBoxSet {
    pub const m_name: usize = 0x0;
    pub const m_nNameHash: usize = 0x8;
    pub const m_HitBoxes: usize = 0x10;
    pub const m_SourceFilename: usize = 0x28;
}

#[allow(non_snake_case, non_upper_case_globals)]
pub mod CHitBoxSetList {
    pub const m_HitBoxSets: usize = 0x0;
}

#[allow(non_snake_case, non_upper_case_globals)]
pub mod CHitReactUpdateNode {
    pub const m_opFixedSettings: usize = 0x68;
    pub const m_triggerParam: usize = 0xb4;
    pub const m_hitBoneParam: usize = 0xb6;
    pub const m_hitOffsetParam: usize = 0xb8;
    pub const m_hitDirectionParam: usize = 0xba;
    pub const m_hitStrengthParam: usize = 0xbc;
    pub const m_flMinDelayBetweenHits: usize = 0xc0;
    pub const m_bResetChild: usize = 0xc4;
}

#[allow(non_snake_case, non_upper_case_globals)]
pub mod CIntAnimParameter {
    pub const m_defaultValue: usize = 0x60;
    pub const m_minValue: usize = 0x64;
    pub const m_maxValue: usize = 0x68;
}

#[allow(non_snake_case, non_upper_case_globals)]
pub mod CJiggleBoneUpdateNode {
    pub const m_opFixedData: usize = 0x68;
}

#[allow(non_snake_case, non_upper_case_globals)]
pub mod CJumpHelperUpdateNode {
    pub const m_hTargetParam: usize = 0xa8;
    pub const m_flOriginalJumpMovement: usize = 0xac;
    pub const m_flOriginalJumpDuration: usize = 0xb8;
    pub const m_flJumpStartCycle: usize = 0xbc;
    pub const m_flJumpEndCycle: usize = 0xc0;
    pub const m_eCorrectionMethod: usize = 0xc4;
    pub const m_bTranslationAxis: usize = 0xc8;
    pub const m_bScaleSpeed: usize = 0xcb;
}

#[allow(non_snake_case, non_upper_case_globals)]
pub mod CLODComponentUpdater {
    pub const m_nServerLOD: usize = 0x30;
}

#[allow(non_snake_case, non_upper_case_globals)]
pub mod CLeanMatrixUpdateNode {
    pub const m_frameCorners: usize = 0x5c;
    pub const m_poses: usize = 0x80;
    pub const m_damping: usize = 0xa8;
    pub const m_blendSource: usize = 0xb8;
    pub const m_paramIndex: usize = 0xbc;
    pub const m_verticalAxis: usize = 0xc0;
    pub const m_horizontalAxis: usize = 0xcc;
    pub const m_hSequence: usize = 0xd8;
    pub const m_flMaxValue: usize = 0xdc;
    pub const m_nSequenceMaxFrame: usize = 0xe0;
}

#[allow(non_snake_case, non_upper_case_globals)]
pub mod CLookAtUpdateNode {
    pub const m_opFixedSettings: usize = 0x70;
    pub const m_target: usize = 0x138;
    pub const m_paramIndex: usize = 0x13c;
    pub const m_weightParamIndex: usize = 0x13e;
    pub const m_bResetChild: usize = 0x140;
    pub const m_bLockWhenWaning: usize = 0x141;
}

#[allow(non_snake_case, non_upper_case_globals)]
pub mod CLookComponentUpdater {
    pub const m_hLookHeading: usize = 0x34;
    pub const m_hLookHeadingVelocity: usize = 0x36;
    pub const m_hLookPitch: usize = 0x38;
    pub const m_hLookDistance: usize = 0x3a;
    pub const m_hLookDirection: usize = 0x3c;
    pub const m_hLookTarget: usize = 0x3e;
    pub const m_hLookTargetWorldSpace: usize = 0x40;
    pub const m_bNetworkLookTarget: usize = 0x42;
}

#[allow(non_snake_case, non_upper_case_globals)]
pub mod CMaterialAttributeAnimTag {
    pub const m_AttributeName: usize = 0x38;
    pub const m_AttributeType: usize = 0x40;
    pub const m_flValue: usize = 0x44;
    pub const m_Color: usize = 0x48;
}

#[allow(non_snake_case, non_upper_case_globals)]
pub mod CMaterialDrawDescriptor {
    pub const m_nPrimitiveType: usize = 0x0;
    pub const m_nBaseVertex: usize = 0x4;
    pub const m_nVertexCount: usize = 0x8;
    pub const m_nStartIndex: usize = 0xc;
    pub const m_nIndexCount: usize = 0x10;
    pub const m_flUvDensity: usize = 0x14;
    pub const m_vTintColor: usize = 0x18;
    pub const m_flAlpha: usize = 0x24;
    pub const m_nFirstMeshlet: usize = 0x2c;
    pub const m_nNumMeshlets: usize = 0x30;
    pub const m_indexBuffer: usize = 0xb8;
    pub const m_material: usize = 0xe0;
}

#[allow(non_snake_case, non_upper_case_globals)]
pub mod CMeshletDescriptor {
    pub const m_PackedAABB: usize = 0x0;
    pub const m_CullingData: usize = 0x8;
}

#[allow(non_snake_case, non_upper_case_globals)]
pub mod CModelConfig {
    pub const m_ConfigName: usize = 0x0;
    pub const m_Elements: usize = 0x8;
    pub const m_bTopLevel: usize = 0x20;
}

#[allow(non_snake_case, non_upper_case_globals)]
pub mod CModelConfigElement {
    pub const m_ElementName: usize = 0x8;
    pub const m_NestedElements: usize = 0x10;
}

#[allow(non_snake_case, non_upper_case_globals)]
pub mod CModelConfigElement_AttachedModel {
    pub const m_InstanceName: usize = 0x48;
    pub const m_EntityClass: usize = 0x50;
    pub const m_hModel: usize = 0x58;
    pub const m_vOffset: usize = 0x60;
    pub const m_aAngOffset: usize = 0x6c;
    pub const m_AttachmentName: usize = 0x78;
    pub const m_LocalAttachmentOffsetName: usize = 0x80;
    pub const m_AttachmentType: usize = 0x88;
    pub const m_bBoneMergeFlex: usize = 0x8c;
    pub const m_bUserSpecifiedColor: usize = 0x8d;
    pub const m_bUserSpecifiedMaterialGroup: usize = 0x8e;
    pub const m_bAcceptParentMaterialDrivenDecals: usize = 0x8f;
    pub const m_BodygroupOnOtherModels: usize = 0x90;
    pub const m_MaterialGroupOnOtherModels: usize = 0x98;
}

#[allow(non_snake_case, non_upper_case_globals)]
pub mod CModelConfigElement_Command {
    pub const m_Command: usize = 0x48;
    pub const m_Args: usize = 0x50;
}

#[allow(non_snake_case, non_upper_case_globals)]
pub mod CModelConfigElement_RandomColor {
    pub const m_Gradient: usize = 0x48;
}

#[allow(non_snake_case, non_upper_case_globals)]
pub mod CModelConfigElement_RandomPick {
    pub const m_Choices: usize = 0x48;
    pub const m_ChoiceWeights: usize = 0x60;
}

#[allow(non_snake_case, non_upper_case_globals)]
pub mod CModelConfigElement_SetBodygroup {
    pub const m_GroupName: usize = 0x48;
    pub const m_nChoice: usize = 0x50;
}

#[allow(non_snake_case, non_upper_case_globals)]
pub mod CModelConfigElement_SetBodygroupOnAttachedModels {
    pub const m_GroupName: usize = 0x48;
    pub const m_nChoice: usize = 0x50;
}

#[allow(non_snake_case, non_upper_case_globals)]
pub mod CModelConfigElement_SetMaterialGroup {
    pub const m_MaterialGroupName: usize = 0x48;
}

#[allow(non_snake_case, non_upper_case_globals)]
pub mod CModelConfigElement_SetMaterialGroupOnAttachedModels {
    pub const m_MaterialGroupName: usize = 0x48;
}

#[allow(non_snake_case, non_upper_case_globals)]
pub mod CModelConfigElement_SetRenderColor {
    pub const m_Color: usize = 0x48;
}

#[allow(non_snake_case, non_upper_case_globals)]
pub mod CModelConfigElement_UserPick {
    pub const m_Choices: usize = 0x48;
}

#[allow(non_snake_case, non_upper_case_globals)]
pub mod CModelConfigList {
    pub const m_bHideMaterialGroupInTools: usize = 0x0;
    pub const m_bHideRenderColorInTools: usize = 0x1;
    pub const m_Configs: usize = 0x8;
}

#[allow(non_snake_case, non_upper_case_globals)]
pub mod CMoodVData {
    pub const m_sModelName: usize = 0x0;
    pub const m_nMoodType: usize = 0xe0;
    pub const m_animationLayers: usize = 0xe8;
}

#[allow(non_snake_case, non_upper_case_globals)]
pub mod CMorphBundleData {
    pub const m_flULeftSrc: usize = 0x0;
    pub const m_flVTopSrc: usize = 0x4;
    pub const m_offsets: usize = 0x8;
    pub const m_ranges: usize = 0x20;
}

#[allow(non_snake_case, non_upper_case_globals)]
pub mod CMorphConstraint {
    pub const m_sTargetMorph: usize = 0x70;
    pub const m_nSlaveChannel: usize = 0x78;
    pub const m_flMin: usize = 0x7c;
    pub const m_flMax: usize = 0x80;
}

#[allow(non_snake_case, non_upper_case_globals)]
pub mod CMorphData {
    pub const m_name: usize = 0x0;
    pub const m_morphRectDatas: usize = 0x8;
}

#[allow(non_snake_case, non_upper_case_globals)]
pub mod CMorphRectData {
    pub const m_nXLeftDst: usize = 0x0;
    pub const m_nYTopDst: usize = 0x2;
    pub const m_flUWidthSrc: usize = 0x4;
    pub const m_flVHeightSrc: usize = 0x8;
    pub const m_bundleDatas: usize = 0x10;
}

#[allow(non_snake_case, non_upper_case_globals)]
pub mod CMorphSetData {
    pub const m_nWidth: usize = 0x10;
    pub const m_nHeight: usize = 0x14;
    pub const m_bundleTypes: usize = 0x18;
    pub const m_morphDatas: usize = 0x30;
    pub const m_pTextureAtlas: usize = 0x48;
    pub const m_FlexDesc: usize = 0x50;
    pub const m_FlexControllers: usize = 0x68;
    pub const m_FlexRules: usize = 0x80;
}

#[allow(non_snake_case, non_upper_case_globals)]
pub mod CMotionDataSet {
    pub const m_groups: usize = 0x0;
    pub const m_nDimensionCount: usize = 0x18;
}

#[allow(non_snake_case, non_upper_case_globals)]
pub mod CMotionGraph {
    pub const m_paramSpans: usize = 0x10;
    pub const m_tags: usize = 0x28;
    pub const m_pRootNode: usize = 0x40;
    pub const m_nParameterCount: usize = 0x48;
    pub const m_nConfigStartIndex: usize = 0x4c;
    pub const m_nConfigCount: usize = 0x50;
    pub const m_bLoop: usize = 0x54;
}

#[allow(non_snake_case, non_upper_case_globals)]
pub mod CMotionGraphConfig {
    pub const m_paramValues: usize = 0x0;
    pub const m_flDuration: usize = 0x10;
    pub const m_nMotionIndex: usize = 0x14;
    pub const m_nSampleStart: usize = 0x18;
    pub const m_nSampleCount: usize = 0x1c;
}

#[allow(non_snake_case, non_upper_case_globals)]
pub mod CMotionGraphGroup {
    pub const m_searchDB: usize = 0x0;
    pub const m_motionGraphs: usize = 0xb8;
    pub const m_motionGraphConfigs: usize = 0xd0;
    pub const m_sampleToConfig: usize = 0xe8;
    pub const m_hIsActiveScript: usize = 0x100;
}

#[allow(non_snake_case, non_upper_case_globals)]
pub mod CMotionGraphUpdateNode {
    pub const m_pMotionGraph: usize = 0x58;
}

#[allow(non_snake_case, non_upper_case_globals)]
pub mod CMotionMatchingUpdateNode {
    pub const m_dataSet: usize = 0x58;
    pub const m_metrics: usize = 0x78;
    pub const m_weights: usize = 0x90;
    pub const m_bSearchEveryTick: usize = 0xe0;
    pub const m_flSearchInterval: usize = 0xe4;
    pub const m_bSearchWhenClipEnds: usize = 0xe8;
    pub const m_bSearchWhenGoalChanges: usize = 0xe9;
    pub const m_blendCurve: usize = 0xec;
    pub const m_flSampleRate: usize = 0xf4;
    pub const m_flBlendTime: usize = 0xf8;
    pub const m_bLockClipWhenWaning: usize = 0xfc;
    pub const m_flSelectionThreshold: usize = 0x100;
    pub const m_flReselectionTimeWindow: usize = 0x104;
    pub const m_bEnableRotationCorrection: usize = 0x108;
    pub const m_bGoalAssist: usize = 0x109;
    pub const m_flGoalAssistDistance: usize = 0x10c;
    pub const m_flGoalAssistTolerance: usize = 0x110;
    pub const m_distanceScale_Damping: usize = 0x118;
    pub const m_flDistanceScale_OuterRadius: usize = 0x128;
    pub const m_flDistanceScale_InnerRadius: usize = 0x12c;
    pub const m_flDistanceScale_MaxScale: usize = 0x130;
    pub const m_flDistanceScale_MinScale: usize = 0x134;
    pub const m_bEnableDistanceScaling: usize = 0x138;
}

#[allow(non_snake_case, non_upper_case_globals)]
pub mod CMotionMetricEvaluator {
    pub const m_means: usize = 0x18;
    pub const m_standardDeviations: usize = 0x30;
    pub const m_flWeight: usize = 0x48;
    pub const m_nDimensionStartIndex: usize = 0x4c;
}

#[allow(non_snake_case, non_upper_case_globals)]
pub mod CMotionNode {
    pub const m_name: usize = 0x18;
    pub const m_id: usize = 0x20;
}

#[allow(non_snake_case, non_upper_case_globals)]
pub mod CMotionNodeBlend1D {
    pub const m_blendItems: usize = 0x28;
    pub const m_nParamIndex: usize = 0x40;
}

#[allow(non_snake_case, non_upper_case_globals)]
pub mod CMotionNodeSequence {
    pub const m_tags: usize = 0x28;
    pub const m_hSequence: usize = 0x40;
    pub const m_flPlaybackSpeed: usize = 0x44;
}

#[allow(non_snake_case, non_upper_case_globals)]
pub mod CMotionSearchDB {
    pub const m_rootNode: usize = 0x0;
    pub const m_residualQuantizer: usize = 0x80;
    pub const m_codeIndices: usize = 0xa0;
}

#[allow(non_snake_case, non_upper_case_globals)]
pub mod CMotionSearchNode {
    pub const m_children: usize = 0x0;
    pub const m_quantizer: usize = 0x18;
    pub const m_sampleCodes: usize = 0x38;
    pub const m_sampleIndices: usize = 0x50;
    pub const m_selectableSamples: usize = 0x68;
}

#[allow(non_snake_case, non_upper_case_globals)]
pub mod CMovementComponentUpdater {
    pub const m_movementModes: usize = 0x30;
    pub const m_motors: usize = 0x48;
    pub const m_facingDamping: usize = 0x60;
    pub const m_eDefaultFacingMode: usize = 0x70;
    pub const m_nDefaultMotorIndex: usize = 0x7c;
    pub const m_bMoveVarsDisabled: usize = 0x80;
    pub const m_bNetworkPath: usize = 0x81;
    pub const m_bNetworkFacing: usize = 0x82;
    pub const m_paramHandles: usize = 0x83;
}

#[allow(non_snake_case, non_upper_case_globals)]
pub mod CMovementMode {
    pub const m_name: usize = 0x0;
    pub const m_flSpeed: usize = 0x8;
}

#[allow(non_snake_case, non_upper_case_globals)]
pub mod CMoverUpdateNode {
    pub const m_damping: usize = 0x70;
    pub const m_facingTarget: usize = 0x80;
    pub const m_hMoveVecParam: usize = 0x84;
    pub const m_hMoveHeadingParam: usize = 0x86;
    pub const m_hTurnToFaceParam: usize = 0x88;
    pub const m_flTurnToFaceOffset: usize = 0x8c;
    pub const m_flTurnToFaceLimit: usize = 0x90;
    pub const m_bAdditive: usize = 0x94;
    pub const m_bApplyMovement: usize = 0x95;
    pub const m_bOrientMovement: usize = 0x96;
    pub const m_bApplyRotation: usize = 0x97;
    pub const m_bLimitOnly: usize = 0x98;
}

#[allow(non_snake_case, non_upper_case_globals)]
pub mod CParamSpanUpdater {
    pub const m_spans: usize = 0x0;
}

#[allow(non_snake_case, non_upper_case_globals)]
pub mod CParticleAnimTag {
    pub const m_hParticleSystem: usize = 0x38;
    pub const m_particleSystemName: usize = 0x40;
    pub const m_configName: usize = 0x48;
    pub const m_bDetachFromOwner: usize = 0x50;
    pub const m_bStopWhenTagEnds: usize = 0x51;
    pub const m_bTagEndStopIsInstant: usize = 0x52;
    pub const m_attachmentName: usize = 0x58;
    pub const m_attachmentType: usize = 0x60;
    pub const m_attachmentCP1Name: usize = 0x68;
    pub const m_attachmentCP1Type: usize = 0x70;
}

#[allow(non_snake_case, non_upper_case_globals)]
pub mod CPathAnimMotorUpdaterBase {
    pub const m_bLockToPath: usize = 0x20;
}

#[allow(non_snake_case, non_upper_case_globals)]
pub mod CPathHelperUpdateNode {
    pub const m_flStoppingRadius: usize = 0x68;
    pub const m_flStoppingSpeedScale: usize = 0x6c;
}

#[allow(non_snake_case, non_upper_case_globals)]
pub mod CPathMetricEvaluator {
    pub const m_pathTimeSamples: usize = 0x50;
    pub const m_flDistance: usize = 0x68;
    pub const m_bExtrapolateMovement: usize = 0x6c;
    pub const m_flMinExtrapolationSpeed: usize = 0x70;
}

#[allow(non_snake_case, non_upper_case_globals)]
pub mod CPhysSurfaceProperties {
    pub const m_name: usize = 0x0;
    pub const m_nameHash: usize = 0x8;
    pub const m_baseNameHash: usize = 0xc;
    pub const m_bHidden: usize = 0x18;
    pub const m_description: usize = 0x20;
    pub const m_physics: usize = 0x28;
    pub const m_audioSounds: usize = 0x48;
    pub const m_audioParams: usize = 0x88;
}

#[allow(non_snake_case, non_upper_case_globals)]
pub mod CPhysSurfacePropertiesAudio {
    pub const m_reflectivity: usize = 0x0;
    pub const m_hardnessFactor: usize = 0x4;
    pub const m_roughnessFactor: usize = 0x8;
    pub const m_roughThreshold: usize = 0xc;
    pub const m_hardThreshold: usize = 0x10;
    pub const m_hardVelocityThreshold: usize = 0x14;
    pub const m_flStaticImpactVolume: usize = 0x18;
    pub const m_flOcclusionFactor: usize = 0x1c;
}

#[allow(non_snake_case, non_upper_case_globals)]
pub mod CPhysSurfacePropertiesPhysics {
    pub const m_friction: usize = 0x0;
    pub const m_elasticity: usize = 0x4;
    pub const m_density: usize = 0x8;
    pub const m_thickness: usize = 0xc;
    pub const m_softContactFrequency: usize = 0x10;
    pub const m_softContactDampingRatio: usize = 0x14;
    pub const m_wheelDrag: usize = 0x18;
}

#[allow(non_snake_case, non_upper_case_globals)]
pub mod CPhysSurfacePropertiesSoundNames {
    pub const m_impactSoft: usize = 0x0;
    pub const m_impactHard: usize = 0x8;
    pub const m_scrapeSmooth: usize = 0x10;
    pub const m_scrapeRough: usize = 0x18;
    pub const m_bulletImpact: usize = 0x20;
    pub const m_rolling: usize = 0x28;
    pub const m_break: usize = 0x30;
    pub const m_strain: usize = 0x38;
}

#[allow(non_snake_case, non_upper_case_globals)]
pub mod CPlayerInputAnimMotorUpdater {
    pub const m_sampleTimes: usize = 0x20;
    pub const m_flSpringConstant: usize = 0x3c;
    pub const m_flAnticipationDistance: usize = 0x40;
    pub const m_hAnticipationPosParam: usize = 0x44;
    pub const m_hAnticipationHeadingParam: usize = 0x46;
    pub const m_bUseAcceleration: usize = 0x48;
}

#[allow(non_snake_case, non_upper_case_globals)]
pub mod CPoseHandle {
    pub const m_nIndex: usize = 0x0;
    pub const m_eType: usize = 0x2;
}

#[allow(non_snake_case, non_upper_case_globals)]
pub mod CProductQuantizer {
    pub const m_subQuantizers: usize = 0x0;
    pub const m_nDimensions: usize = 0x18;
}

#[allow(non_snake_case, non_upper_case_globals)]
pub mod CQuaternionAnimParameter {
    pub const m_defaultValue: usize = 0x60;
    pub const m_bInterpolate: usize = 0x70;
}

#[allow(non_snake_case, non_upper_case_globals)]
pub mod CRagdollAnimTag {
    pub const m_nPoseControl: usize = 0x38;
    pub const m_flFrequency: usize = 0x3c;
    pub const m_flDampingRatio: usize = 0x40;
    pub const m_flDecayDuration: usize = 0x44;
    pub const m_flDecayBias: usize = 0x48;
    pub const m_bDestroy: usize = 0x4c;
}

#[allow(non_snake_case, non_upper_case_globals)]
pub mod CRagdollComponentUpdater {
    pub const m_ragdollNodePaths: usize = 0x30;
    pub const m_boneIndices: usize = 0x48;
    pub const m_boneNames: usize = 0x60;
    pub const m_weightLists: usize = 0x78;
    pub const m_flSpringFrequencyMin: usize = 0x90;
    pub const m_flSpringFrequencyMax: usize = 0x94;
    pub const m_flMaxStretch: usize = 0x98;
}

#[allow(non_snake_case, non_upper_case_globals)]
pub mod CRagdollUpdateNode {
    pub const m_nWeightListIndex: usize = 0x68;
    pub const m_poseControlMethod: usize = 0x6c;
}

#[allow(non_snake_case, non_upper_case_globals)]
pub mod CRenderBufferBinding {
    pub const m_hBuffer: usize = 0x0;
    pub const m_nBindOffsetBytes: usize = 0x10;
}

#[allow(non_snake_case, non_upper_case_globals)]
pub mod CRenderMesh {
    pub const m_sceneObjects: usize = 0x10;
    pub const m_constraints: usize = 0xa0;
    pub const m_skeleton: usize = 0xb8;
}

#[allow(non_snake_case, non_upper_case_globals)]
pub mod CRenderSkeleton {
    pub const m_bones: usize = 0x0;
    pub const m_boneParents: usize = 0x30;
    pub const m_nBoneWeightCount: usize = 0x48;
}

#[allow(non_snake_case, non_upper_case_globals)]
pub mod CSceneObjectData {
    pub const m_vMinBounds: usize = 0x0;
    pub const m_vMaxBounds: usize = 0xc;
    pub const m_drawCalls: usize = 0x18;
    pub const m_drawBounds: usize = 0x30;
    pub const m_meshlets: usize = 0x48;
    pub const m_vTintColor: usize = 0x60;
}

#[allow(non_snake_case, non_upper_case_globals)]
pub mod CSelectorUpdateNode {
    pub const m_children: usize = 0x58;
    pub const m_tags: usize = 0x70;
    pub const m_blendCurve: usize = 0x8c;
    pub const m_flBlendTime: usize = 0x94;
    pub const m_hParameter: usize = 0x9c;
    pub const m_eTagBehavior: usize = 0xa0;
    pub const m_bResetOnChange: usize = 0xa4;
    pub const m_bSyncCyclesOnChange: usize = 0xa5;
}

#[allow(non_snake_case, non_upper_case_globals)]
pub mod CSeqAutoLayer {
    pub const m_nLocalReference: usize = 0x0;
    pub const m_nLocalPose: usize = 0x2;
    pub const m_flags: usize = 0x4;
    pub const m_start: usize = 0xc;
    pub const m_peak: usize = 0x10;
    pub const m_tail: usize = 0x14;
    pub const m_end: usize = 0x18;
}

#[allow(non_snake_case, non_upper_case_globals)]
pub mod CSeqAutoLayerFlag {
    pub const m_bPost: usize = 0x0;
    pub const m_bSpline: usize = 0x1;
    pub const m_bXFade: usize = 0x2;
    pub const m_bNoBlend: usize = 0x3;
    pub const m_bLocal: usize = 0x4;
    pub const m_bPose: usize = 0x5;
    pub const m_bFetchFrame: usize = 0x6;
    pub const m_bSubtract: usize = 0x7;
}

#[allow(non_snake_case, non_upper_case_globals)]
pub mod CSeqBoneMaskList {
    pub const m_sName: usize = 0x0;
    pub const m_nLocalBoneArray: usize = 0x10;
    pub const m_flBoneWeightArray: usize = 0x28;
    pub const m_flDefaultMorphCtrlWeight: usize = 0x40;
    pub const m_morphCtrlWeightArray: usize = 0x48;
}

#[allow(non_snake_case, non_upper_case_globals)]
pub mod CSeqCmdLayer {
    pub const m_cmd: usize = 0x0;
    pub const m_nLocalReference: usize = 0x2;
    pub const m_nLocalBonemask: usize = 0x4;
    pub const m_nDstResult: usize = 0x6;
    pub const m_nSrcResult: usize = 0x8;
    pub const m_bSpline: usize = 0xa;
    pub const m_flVar1: usize = 0xc;
    pub const m_flVar2: usize = 0x10;
    pub const m_nLineNumber: usize = 0x14;
}

#[allow(non_snake_case, non_upper_case_globals)]
pub mod CSeqCmdSeqDesc {
    pub const m_sName: usize = 0x0;
    pub const m_flags: usize = 0x10;
    pub const m_transition: usize = 0x1c;
    pub const m_nFrameRangeSequence: usize = 0x24;
    pub const m_nFrameCount: usize = 0x26;
    pub const m_flFPS: usize = 0x28;
    pub const m_nSubCycles: usize = 0x2c;
    pub const m_numLocalResults: usize = 0x2e;
    pub const m_cmdLayerArray: usize = 0x30;
    pub const m_eventArray: usize = 0x48;
    pub const m_activityArray: usize = 0x60;
    pub const m_poseSettingArray: usize = 0x78;
}

#[allow(non_snake_case, non_upper_case_globals)]
pub mod CSeqIKLock {
    pub const m_flPosWeight: usize = 0x0;
    pub const m_flAngleWeight: usize = 0x4;
    pub const m_nLocalBone: usize = 0x8;
    pub const m_bBonesOrientedAlongPositiveX: usize = 0xa;
}

#[allow(non_snake_case, non_upper_case_globals)]
pub mod CSeqMultiFetch {
    pub const m_flags: usize = 0x0;
    pub const m_localReferenceArray: usize = 0x8;
    pub const m_nGroupSize: usize = 0x20;
    pub const m_nLocalPose: usize = 0x28;
    pub const m_poseKeyArray0: usize = 0x30;
    pub const m_poseKeyArray1: usize = 0x48;
    pub const m_nLocalCyclePoseParameter: usize = 0x60;
    pub const m_bCalculatePoseParameters: usize = 0x64;
}

#[allow(non_snake_case, non_upper_case_globals)]
pub mod CSeqMultiFetchFlag {
    pub const m_bRealtime: usize = 0x0;
    pub const m_bCylepose: usize = 0x1;
    pub const m_b0D: usize = 0x2;
    pub const m_b1D: usize = 0x3;
    pub const m_b2D: usize = 0x4;
    pub const m_b2D_TRI: usize = 0x5;
}

#[allow(non_snake_case, non_upper_case_globals)]
pub mod CSeqPoseParamDesc {
    pub const m_sName: usize = 0x0;
    pub const m_flStart: usize = 0x10;
    pub const m_flEnd: usize = 0x14;
    pub const m_flLoop: usize = 0x18;
    pub const m_bLooping: usize = 0x1c;
}

#[allow(non_snake_case, non_upper_case_globals)]
pub mod CSeqPoseSetting {
    pub const m_sPoseParameter: usize = 0x0;
    pub const m_sAttachment: usize = 0x10;
    pub const m_sReferenceSequence: usize = 0x20;
    pub const m_flValue: usize = 0x30;
    pub const m_bX: usize = 0x34;
    pub const m_bY: usize = 0x35;
    pub const m_bZ: usize = 0x36;
    pub const m_eType: usize = 0x38;
}

#[allow(non_snake_case, non_upper_case_globals)]
pub mod CSeqS1SeqDesc {
    pub const m_sName: usize = 0x0;
    pub const m_flags: usize = 0x10;
    pub const m_fetch: usize = 0x20;
    pub const m_nLocalWeightlist: usize = 0x88;
    pub const m_autoLayerArray: usize = 0x90;
    pub const m_IKLockArray: usize = 0xa8;
    pub const m_transition: usize = 0xc0;
    pub const m_SequenceKeys: usize = 0xc8;
    pub const m_LegacyKeyValueText: usize = 0xd8;
    pub const m_activityArray: usize = 0xe8;
    pub const m_footMotion: usize = 0x100;
}

#[allow(non_snake_case, non_upper_case_globals)]
pub mod CSeqScaleSet {
    pub const m_sName: usize = 0x0;
    pub const m_bRootOffset: usize = 0x10;
    pub const m_vRootOffset: usize = 0x14;
    pub const m_nLocalBoneArray: usize = 0x20;
    pub const m_flBoneScaleArray: usize = 0x38;
}

#[allow(non_snake_case, non_upper_case_globals)]
pub mod CSeqSeqDescFlag {
    pub const m_bLooping: usize = 0x0;
    pub const m_bSnap: usize = 0x1;
    pub const m_bAutoplay: usize = 0x2;
    pub const m_bPost: usize = 0x3;
    pub const m_bHidden: usize = 0x4;
    pub const m_bMulti: usize = 0x5;
    pub const m_bLegacyDelta: usize = 0x6;
    pub const m_bLegacyWorldspace: usize = 0x7;
    pub const m_bLegacyCyclepose: usize = 0x8;
    pub const m_bLegacyRealtime: usize = 0x9;
    pub const m_bModelDoc: usize = 0xa;
}

#[allow(non_snake_case, non_upper_case_globals)]
pub mod CSeqSynthAnimDesc {
    pub const m_sName: usize = 0x0;
    pub const m_flags: usize = 0x10;
    pub const m_transition: usize = 0x1c;
    pub const m_nLocalBaseReference: usize = 0x24;
    pub const m_nLocalBoneMask: usize = 0x26;
    pub const m_activityArray: usize = 0x28;
}

#[allow(non_snake_case, non_upper_case_globals)]
pub mod CSeqTransition {
    pub const m_flFadeInTime: usize = 0x0;
    pub const m_flFadeOutTime: usize = 0x4;
}

#[allow(non_snake_case, non_upper_case_globals)]
pub mod CSequenceFinishedAnimTag {
    pub const m_sequenceName: usize = 0x38;
}

#[allow(non_snake_case, non_upper_case_globals)]
pub mod CSequenceGroupData {
    pub const m_sName: usize = 0x10;
    pub const m_nFlags: usize = 0x20;
    pub const m_localSequenceNameArray: usize = 0x28;
    pub const m_localS1SeqDescArray: usize = 0x40;
    pub const m_localMultiSeqDescArray: usize = 0x58;
    pub const m_localSynthAnimDescArray: usize = 0x70;
    pub const m_localCmdSeqDescArray: usize = 0x88;
    pub const m_localBoneMaskArray: usize = 0xa0;
    pub const m_localScaleSetArray: usize = 0xb8;
    pub const m_localBoneNameArray: usize = 0xd0;
    pub const m_localNodeName: usize = 0xe8;
    pub const m_localPoseParamArray: usize = 0xf8;
    pub const m_keyValues: usize = 0x110;
    pub const m_localIKAutoplayLockArray: usize = 0x120;
}

#[allow(non_snake_case, non_upper_case_globals)]
pub mod CSequenceUpdateNode {
    pub const m_paramSpans: usize = 0x60;
    pub const m_tags: usize = 0x78;
    pub const m_hSequence: usize = 0x94;
    pub const m_playbackSpeed: usize = 0x98;
    pub const m_duration: usize = 0x9c;
    pub const m_bLoop: usize = 0xa0;
}

#[allow(non_snake_case, non_upper_case_globals)]
pub mod CSetFacingUpdateNode {
    pub const m_facingMode: usize = 0x68;
    pub const m_bResetChild: usize = 0x6c;
}

#[allow(non_snake_case, non_upper_case_globals)]
pub mod CSetParameterActionUpdater {
    pub const m_hParam: usize = 0x18;
    pub const m_value: usize = 0x1a;
}

#[allow(non_snake_case, non_upper_case_globals)]
pub mod CSingleFrameUpdateNode {
    pub const m_actions: usize = 0x58;
    pub const m_hPoseCacheHandle: usize = 0x70;
    pub const m_hSequence: usize = 0x74;
    pub const m_flCycle: usize = 0x78;
}

#[allow(non_snake_case, non_upper_case_globals)]
pub mod CSkeletalInputUpdateNode {
    pub const m_fixedOpData: usize = 0x58;
}

#[allow(non_snake_case, non_upper_case_globals)]
pub mod CSlopeComponentUpdater {
    pub const m_flTraceDistance: usize = 0x34;
    pub const m_hSlopeAngle: usize = 0x38;
    pub const m_hSlopeAngleFront: usize = 0x3a;
    pub const m_hSlopeAngleSide: usize = 0x3c;
    pub const m_hSlopeHeading: usize = 0x3e;
    pub const m_hSlopeNormal: usize = 0x40;
    pub const m_hSlopeNormal_WorldSpace: usize = 0x42;
}

#[allow(non_snake_case, non_upper_case_globals)]
pub mod CSlowDownOnSlopesUpdateNode {
    pub const m_flSlowDownStrength: usize = 0x68;
}

#[allow(non_snake_case, non_upper_case_globals)]
pub mod CSolveIKChainUpdateNode {
    pub const m_targetHandles: usize = 0x68;
    pub const m_opFixedData: usize = 0x80;
}

#[allow(non_snake_case, non_upper_case_globals)]
pub mod CSolveIKTargetHandle_t {
    pub const m_positionHandle: usize = 0x0;
    pub const m_orientationHandle: usize = 0x2;
}

#[allow(non_snake_case, non_upper_case_globals)]
pub mod CSpeedScaleUpdateNode {
    pub const m_paramIndex: usize = 0x68;
}

#[allow(non_snake_case, non_upper_case_globals)]
pub mod CStanceOverrideUpdateNode {
    pub const m_footStanceInfo: usize = 0x68;
    pub const m_pStanceSourceNode: usize = 0x80;
    pub const m_hParameter: usize = 0x90;
    pub const m_eMode: usize = 0x94;
}

#[allow(non_snake_case, non_upper_case_globals)]
pub mod CStanceScaleUpdateNode {
    pub const m_hParam: usize = 0x68;
}

#[allow(non_snake_case, non_upper_case_globals)]
pub mod CStateActionUpdater {
    pub const m_pAction: usize = 0x0;
    pub const m_eBehavior: usize = 0x8;
}

#[allow(non_snake_case, non_upper_case_globals)]
pub mod CStateMachineComponentUpdater {
    pub const m_stateMachine: usize = 0x30;
}

#[allow(non_snake_case, non_upper_case_globals)]
pub mod CStateMachineUpdateNode {
    pub const m_stateMachine: usize = 0x68;
    pub const m_stateData: usize = 0xc0;
    pub const m_transitionData: usize = 0xd8;
    pub const m_bBlockWaningTags: usize = 0xf4;
    pub const m_bLockStateWhenWaning: usize = 0xf5;
}

#[allow(non_snake_case, non_upper_case_globals)]
pub mod CStateNodeStateData {
    pub const m_pChild: usize = 0x0;
    pub const m_bExclusiveRootMotion: usize = 0x0;
}

#[allow(non_snake_case, non_upper_case_globals)]
pub mod CStateNodeTransitionData {
    pub const m_curve: usize = 0x0;
    pub const m_blendDuration: usize = 0x8;
    pub const m_resetCycleValue: usize = 0x10;
    pub const m_bReset: usize = 0x0;
    pub const m_resetCycleOption: usize = 0x0;
}

#[allow(non_snake_case, non_upper_case_globals)]
pub mod CStateUpdateData {
    pub const m_name: usize = 0x0;
    pub const m_hScript: usize = 0x8;
    pub const m_transitionIndices: usize = 0x10;
    pub const m_actions: usize = 0x28;
    pub const m_stateID: usize = 0x40;
    pub const m_bIsStartState: usize = 0x0;
    pub const m_bIsEndState: usize = 0x0;
    pub const m_bIsPassthrough: usize = 0x0;
}

#[allow(non_snake_case, non_upper_case_globals)]
pub mod CStaticPoseCache {
    pub const m_poses: usize = 0x10;
    pub const m_nBoneCount: usize = 0x28;
    pub const m_nMorphCount: usize = 0x2c;
}

#[allow(non_snake_case, non_upper_case_globals)]
pub mod CStepsRemainingMetricEvaluator {
    pub const m_footIndices: usize = 0x50;
    pub const m_flMinStepsRemaining: usize = 0x68;
}

#[allow(non_snake_case, non_upper_case_globals)]
pub mod CStopAtGoalUpdateNode {
    pub const m_flOuterRadius: usize = 0x6c;
    pub const m_flInnerRadius: usize = 0x70;
    pub const m_flMaxScale: usize = 0x74;
    pub const m_flMinScale: usize = 0x78;
    pub const m_damping: usize = 0x80;
}

#[allow(non_snake_case, non_upper_case_globals)]
pub mod CSubtractUpdateNode {
    pub const m_footMotionTiming: usize = 0x8c;
    pub const m_bApplyToFootMotion: usize = 0x90;
    pub const m_bApplyChannelsSeparately: usize = 0x91;
    pub const m_bUseModelSpace: usize = 0x92;
}

#[allow(non_snake_case, non_upper_case_globals)]
pub mod CTiltTwistConstraint {
    pub const m_nTargetAxis: usize = 0x70;
    pub const m_nSlaveAxis: usize = 0x74;
}

#[allow(non_snake_case, non_upper_case_globals)]
pub mod CTimeRemainingMetricEvaluator {
    pub const m_bMatchByTimeRemaining: usize = 0x50;
    pub const m_flMaxTimeRemaining: usize = 0x54;
    pub const m_bFilterByTimeRemaining: usize = 0x58;
    pub const m_flMinTimeRemaining: usize = 0x5c;
}

#[allow(non_snake_case, non_upper_case_globals)]
pub mod CToggleComponentActionUpdater {
    pub const m_componentID: usize = 0x18;
    pub const m_bSetEnabled: usize = 0x1c;
}

#[allow(non_snake_case, non_upper_case_globals)]
pub mod CTransitionUpdateData {
    pub const m_srcStateIndex: usize = 0x0;
    pub const m_destStateIndex: usize = 0x1;
    pub const m_bDisabled: usize = 0x0;
}

#[allow(non_snake_case, non_upper_case_globals)]
pub mod CTurnHelperUpdateNode {
    pub const m_facingTarget: usize = 0x6c;
    pub const m_turnStartTimeOffset: usize = 0x70;
    pub const m_turnDuration: usize = 0x74;
    pub const m_bMatchChildDuration: usize = 0x78;
    pub const m_manualTurnOffset: usize = 0x7c;
    pub const m_bUseManualTurnOffset: usize = 0x80;
}

#[allow(non_snake_case, non_upper_case_globals)]
pub mod CTwistConstraint {
    pub const m_bInverse: usize = 0x70;
    pub const m_qParentBindRotation: usize = 0x80;
    pub const m_qChildBindRotation: usize = 0x90;
}

#[allow(non_snake_case, non_upper_case_globals)]
pub mod CTwoBoneIKUpdateNode {
    pub const m_opFixedData: usize = 0x70;
}

#[allow(non_snake_case, non_upper_case_globals)]
pub mod CUnaryUpdateNode {
    pub const m_pChildNode: usize = 0x58;
}

#[allow(non_snake_case, non_upper_case_globals)]
pub mod CVPhysXSurfacePropertiesList {
    pub const m_surfacePropertiesList: usize = 0x0;
}

#[allow(non_snake_case, non_upper_case_globals)]
pub mod CVRInputComponentUpdater {
    pub const m_FingerCurl_Thumb: usize = 0x34;
    pub const m_FingerCurl_Index: usize = 0x36;
    pub const m_FingerCurl_Middle: usize = 0x38;
    pub const m_FingerCurl_Ring: usize = 0x3a;
    pub const m_FingerCurl_Pinky: usize = 0x3c;
    pub const m_FingerSplay_Thumb_Index: usize = 0x3e;
    pub const m_FingerSplay_Index_Middle: usize = 0x40;
    pub const m_FingerSplay_Middle_Ring: usize = 0x42;
    pub const m_FingerSplay_Ring_Pinky: usize = 0x44;
}

#[allow(non_snake_case, non_upper_case_globals)]
pub mod CVectorAnimParameter {
    pub const m_defaultValue: usize = 0x60;
    pub const m_bInterpolate: usize = 0x6c;
}

#[allow(non_snake_case, non_upper_case_globals)]
pub mod CVectorQuantizer {
    pub const m_centroidVectors: usize = 0x0;
    pub const m_nCentroids: usize = 0x18;
    pub const m_nDimensions: usize = 0x1c;
}

#[allow(non_snake_case, non_upper_case_globals)]
pub mod CVirtualAnimParameter {
    pub const m_expressionString: usize = 0x50;
    pub const m_eParamType: usize = 0x58;
}

#[allow(non_snake_case, non_upper_case_globals)]
pub mod CVrSkeletalInputSettings {
    pub const m_wristBones: usize = 0x0;
    pub const m_fingers: usize = 0x18;
    pub const m_name: usize = 0x30;
    pub const m_outerKnuckle1: usize = 0x38;
    pub const m_outerKnuckle2: usize = 0x40;
    pub const m_eHand: usize = 0x48;
}

#[allow(non_snake_case, non_upper_case_globals)]
pub mod CWayPointHelperUpdateNode {
    pub const m_flStartCycle: usize = 0x6c;
    pub const m_flEndCycle: usize = 0x70;
    pub const m_bOnlyGoals: usize = 0x74;
    pub const m_bPreventOvershoot: usize = 0x75;
    pub const m_bPreventUndershoot: usize = 0x76;
}

#[allow(non_snake_case, non_upper_case_globals)]
pub mod CWristBone {
    pub const m_name: usize = 0x0;
    pub const m_vForwardLS: usize = 0x8;
    pub const m_vUpLS: usize = 0x14;
    pub const m_vOffset: usize = 0x20;
}

#[allow(non_snake_case, non_upper_case_globals)]
pub mod ChainToSolveData_t {
    pub const m_nChainIndex: usize = 0x0;
    pub const m_SolverSettings: usize = 0x4;
    pub const m_TargetSettings: usize = 0x10;
    pub const m_DebugSetting: usize = 0x38;
    pub const m_flDebugNormalizedValue: usize = 0x3c;
    pub const m_vDebugOffset: usize = 0x40;
}

#[allow(non_snake_case, non_upper_case_globals)]
pub mod ConfigIndex {
    pub const m_nGroup: usize = 0x0;
    pub const m_nConfig: usize = 0x2;
}

#[allow(non_snake_case, non_upper_case_globals)]
pub mod FingerBone_t {
    pub const m_boneIndex: usize = 0x0;
    pub const m_hingeAxis: usize = 0x4;
    pub const m_vCapsulePos1: usize = 0x10;
    pub const m_vCapsulePos2: usize = 0x1c;
    pub const m_flMinAngle: usize = 0x28;
    pub const m_flMaxAngle: usize = 0x2c;
    pub const m_flRadius: usize = 0x30;
}

#[allow(non_snake_case, non_upper_case_globals)]
pub mod FingerChain_t {
    pub const m_targets: usize = 0x0;
    pub const m_bones: usize = 0x18;
    pub const m_vTipOffset: usize = 0x30;
    pub const m_vSplayHingeAxis: usize = 0x3c;
    pub const m_tipParentBoneIndex: usize = 0x48;
    pub const m_metacarpalBoneIndex: usize = 0x4c;
    pub const m_flSplayMinAngle: usize = 0x50;
    pub const m_flSplayMaxAngle: usize = 0x54;
    pub const m_flFingerScaleRatio: usize = 0x58;
}

#[allow(non_snake_case, non_upper_case_globals)]
pub mod FingerSource_t {
    pub const m_nFingerIndex: usize = 0x0;
    pub const m_flFingerWeight: usize = 0x4;
}

#[allow(non_snake_case, non_upper_case_globals)]
pub mod FollowAttachmentSettings_t {
    pub const m_attachment: usize = 0x0;
    pub const m_boneIndex: usize = 0x80;
    pub const m_bMatchTranslation: usize = 0x84;
    pub const m_bMatchRotation: usize = 0x85;
}

#[allow(non_snake_case, non_upper_case_globals)]
pub mod FootFixedData_t {
    pub const m_vToeOffset: usize = 0x0;
    pub const m_vHeelOffset: usize = 0x10;
    pub const m_nTargetBoneIndex: usize = 0x20;
    pub const m_nAnkleBoneIndex: usize = 0x24;
    pub const m_nIKAnchorBoneIndex: usize = 0x28;
    pub const m_ikChainIndex: usize = 0x2c;
    pub const m_flMaxIKLength: usize = 0x30;
    pub const m_nFootIndex: usize = 0x34;
    pub const m_nTagIndex: usize = 0x38;
    pub const m_flMaxRotationLeft: usize = 0x3c;
    pub const m_flMaxRotationRight: usize = 0x40;
}

#[allow(non_snake_case, non_upper_case_globals)]
pub mod FootFixedSettings {
    pub const m_traceSettings: usize = 0x0;
    pub const m_vFootBaseBindPosePositionMS: usize = 0x10;
    pub const m_flFootBaseLength: usize = 0x20;
    pub const m_flMaxRotationLeft: usize = 0x24;
    pub const m_flMaxRotationRight: usize = 0x28;
    pub const m_footstepLandedTagIndex: usize = 0x2c;
    pub const m_bEnableTracing: usize = 0x30;
    pub const m_flTraceAngleBlend: usize = 0x34;
    pub const m_nDisableTagIndex: usize = 0x38;
    pub const m_nFootIndex: usize = 0x3c;
}

#[allow(non_snake_case, non_upper_case_globals)]
pub mod FootLockPoseOpFixedSettings {
    pub const m_footInfo: usize = 0x0;
    pub const m_hipDampingSettings: usize = 0x18;
    pub const m_nHipBoneIndex: usize = 0x28;
    pub const m_ikSolverType: usize = 0x2c;
    pub const m_bApplyTilt: usize = 0x30;
    pub const m_bApplyHipDrop: usize = 0x31;
    pub const m_bAlwaysUseFallbackHinge: usize = 0x32;
    pub const m_bApplyFootRotationLimits: usize = 0x33;
    pub const m_bApplyLegTwistLimits: usize = 0x34;
    pub const m_flMaxFootHeight: usize = 0x38;
    pub const m_flExtensionScale: usize = 0x3c;
    pub const m_flMaxLegTwist: usize = 0x40;
    pub const m_bEnableLockBreaking: usize = 0x44;
    pub const m_flLockBreakTolerance: usize = 0x48;
    pub const m_flLockBlendTime: usize = 0x4c;
    pub const m_bEnableStretching: usize = 0x50;
    pub const m_flMaxStretchAmount: usize = 0x54;
    pub const m_flStretchExtensionScale: usize = 0x58;
}

#[allow(non_snake_case, non_upper_case_globals)]
pub mod FootPinningPoseOpFixedData_t {
    pub const m_footInfo: usize = 0x0;
    pub const m_flBlendTime: usize = 0x18;
    pub const m_flLockBreakDistance: usize = 0x1c;
    pub const m_flMaxLegTwist: usize = 0x20;
    pub const m_nHipBoneIndex: usize = 0x24;
    pub const m_bApplyLegTwistLimits: usize = 0x28;
    pub const m_bApplyFootRotationLimits: usize = 0x29;
}

#[allow(non_snake_case, non_upper_case_globals)]
pub mod FootStepTrigger {
    pub const m_tags: usize = 0x0;
    pub const m_nFootIndex: usize = 0x18;
    pub const m_triggerPhase: usize = 0x1c;
}

#[allow(non_snake_case, non_upper_case_globals)]
pub mod HSequence {
    pub const m_Value: usize = 0x0;
}

#[allow(non_snake_case, non_upper_case_globals)]
pub mod HitReactFixedSettings_t {
    pub const m_nWeightListIndex: usize = 0x0;
    pub const m_nEffectedBoneCount: usize = 0x4;
    pub const m_flMaxImpactForce: usize = 0x8;
    pub const m_flMinImpactForce: usize = 0xc;
    pub const m_flWhipImpactScale: usize = 0x10;
    pub const m_flCounterRotationScale: usize = 0x14;
    pub const m_flDistanceFadeScale: usize = 0x18;
    pub const m_flPropagationScale: usize = 0x1c;
    pub const m_flWhipDelay: usize = 0x20;
    pub const m_flSpringStrength: usize = 0x24;
    pub const m_flWhipSpringStrength: usize = 0x28;
    pub const m_flMaxAngleRadians: usize = 0x2c;
    pub const m_nHipBoneIndex: usize = 0x30;
    pub const m_flHipBoneTranslationScale: usize = 0x34;
    pub const m_flHipDipSpringStrength: usize = 0x38;
    pub const m_flHipDipImpactScale: usize = 0x3c;
    pub const m_flHipDipDelay: usize = 0x40;
}

#[allow(non_snake_case, non_upper_case_globals)]
pub mod IKBoneNameAndIndex_t {
    pub const m_Name: usize = 0x0;
}

#[allow(non_snake_case, non_upper_case_globals)]
pub mod IKDemoCaptureSettings_t {
    pub const m_parentBoneName: usize = 0x0;
    pub const m_eMode: usize = 0x8;
    pub const m_ikChainName: usize = 0x10;
    pub const m_oneBoneStart: usize = 0x18;
    pub const m_oneBoneEnd: usize = 0x20;
}

#[allow(non_snake_case, non_upper_case_globals)]
pub mod IKSolverSettings_t {
    pub const m_SolverType: usize = 0x0;
    pub const m_nNumIterations: usize = 0x4;
}

#[allow(non_snake_case, non_upper_case_globals)]
pub mod IKTargetSettings_t {
    pub const m_TargetSource: usize = 0x0;
    pub const m_Bone: usize = 0x8;
    pub const m_AnimgraphParameterNamePosition: usize = 0x18;
    pub const m_AnimgraphParameterNameOrientation: usize = 0x1c;
    pub const m_TargetCoordSystem: usize = 0x20;
}

#[allow(non_snake_case, non_upper_case_globals)]
pub mod JiggleBoneSettingsList_t {
    pub const m_boneSettings: usize = 0x0;
}

#[allow(non_snake_case, non_upper_case_globals)]
pub mod JiggleBoneSettings_t {
    pub const m_nBoneIndex: usize = 0x0;
    pub const m_flSpringStrength: usize = 0x4;
    pub const m_flMaxTimeStep: usize = 0x8;
    pub const m_flDamping: usize = 0xc;
    pub const m_vBoundsMaxLS: usize = 0x10;
    pub const m_vBoundsMinLS: usize = 0x1c;
    pub const m_eSimSpace: usize = 0x28;
}

#[allow(non_snake_case, non_upper_case_globals)]
pub mod LookAtBone_t {
    pub const m_index: usize = 0x0;
    pub const m_weight: usize = 0x4;
}

#[allow(non_snake_case, non_upper_case_globals)]
pub mod LookAtOpFixedSettings_t {
    pub const m_attachment: usize = 0x0;
    pub const m_damping: usize = 0x80;
    pub const m_bones: usize = 0x90;
    pub const m_flYawLimit: usize = 0xa8;
    pub const m_flPitchLimit: usize = 0xac;
    pub const m_flHysteresisInnerAngle: usize = 0xb0;
    pub const m_flHysteresisOuterAngle: usize = 0xb4;
    pub const m_bRotateYawForward: usize = 0xb8;
    pub const m_bMaintainUpDirection: usize = 0xb9;
    pub const m_bTargetIsPosition: usize = 0xba;
    pub const m_bUseHysteresis: usize = 0xbb;
}

#[allow(non_snake_case, non_upper_case_globals)]
pub mod MaterialGroup_t {
    pub const m_name: usize = 0x0;
    pub const m_materials: usize = 0x8;
}

#[allow(non_snake_case, non_upper_case_globals)]
pub mod ModelBoneFlexDriverControl_t {
    pub const m_nBoneComponent: usize = 0x0;
    pub const m_flexController: usize = 0x8;
    pub const m_flexControllerToken: usize = 0x10;
    pub const m_flMin: usize = 0x14;
    pub const m_flMax: usize = 0x18;
}

#[allow(non_snake_case, non_upper_case_globals)]
pub mod ModelBoneFlexDriver_t {
    pub const m_boneName: usize = 0x0;
    pub const m_boneNameToken: usize = 0x8;
    pub const m_controls: usize = 0x10;
}

#[allow(non_snake_case, non_upper_case_globals)]
pub mod ModelSkeletonData_t {
    pub const m_boneName: usize = 0x0;
    pub const m_nParent: usize = 0x18;
    pub const m_boneSphere: usize = 0x30;
    pub const m_nFlag: usize = 0x48;
    pub const m_bonePosParent: usize = 0x60;
    pub const m_boneRotParent: usize = 0x78;
    pub const m_boneScaleParent: usize = 0x90;
}

#[allow(non_snake_case, non_upper_case_globals)]
pub mod MoodAnimationLayer_t {
    pub const m_sName: usize = 0x0;
    pub const m_bActiveListening: usize = 0x8;
    pub const m_bActiveTalking: usize = 0x9;
    pub const m_layerAnimations: usize = 0x10;
    pub const m_flIntensity: usize = 0x28;
    pub const m_flDurationScale: usize = 0x30;
    pub const m_bScaleWithInts: usize = 0x38;
    pub const m_flNextStart: usize = 0x3c;
    pub const m_flStartOffset: usize = 0x44;
    pub const m_flEndOffset: usize = 0x4c;
    pub const m_flFadeIn: usize = 0x54;
    pub const m_flFadeOut: usize = 0x58;
}

#[allow(non_snake_case, non_upper_case_globals)]
pub mod MoodAnimation_t {
    pub const m_sName: usize = 0x0;
    pub const m_flWeight: usize = 0x8;
}

#[allow(non_snake_case, non_upper_case_globals)]
pub mod MotionBlendItem {
    pub const m_pChild: usize = 0x0;
    pub const m_flKeyValue: usize = 0x8;
}

#[allow(non_snake_case, non_upper_case_globals)]
pub mod MotionDBIndex {
    pub const m_nIndex: usize = 0x0;
}

#[allow(non_snake_case, non_upper_case_globals)]
pub mod MotionIndex {
    pub const m_nGroup: usize = 0x0;
    pub const m_nMotion: usize = 0x2;
}

#[allow(non_snake_case, non_upper_case_globals)]
pub mod ParamSpanSample_t {
    pub const m_value: usize = 0x0;
    pub const m_flCycle: usize = 0x14;
}

#[allow(non_snake_case, non_upper_case_globals)]
pub mod ParamSpan_t {
    pub const m_samples: usize = 0x0;
    pub const m_hParam: usize = 0x18;
    pub const m_eParamType: usize = 0x1a;
    pub const m_flStartCycle: usize = 0x1c;
    pub const m_flEndCycle: usize = 0x20;
}

#[allow(non_snake_case, non_upper_case_globals)]
pub mod PermModelDataAnimatedMaterialAttribute_t {
    pub const m_AttributeName: usize = 0x0;
    pub const m_nNumChannels: usize = 0x8;
}

#[allow(non_snake_case, non_upper_case_globals)]
pub mod PermModelData_t {
    pub const m_name: usize = 0x0;
    pub const m_modelInfo: usize = 0x8;
    pub const m_ExtParts: usize = 0x60;
    pub const m_refMeshes: usize = 0x78;
    pub const m_refMeshGroupMasks: usize = 0x90;
    pub const m_refPhysGroupMasks: usize = 0xa8;
    pub const m_refLODGroupMasks: usize = 0xc0;
    pub const m_lodGroupSwitchDistances: usize = 0xd8;
    pub const m_refPhysicsData: usize = 0xf0;
    pub const m_refPhysicsHitboxData: usize = 0x108;
    pub const m_refAnimGroups: usize = 0x120;
    pub const m_refSequenceGroups: usize = 0x138;
    pub const m_meshGroups: usize = 0x150;
    pub const m_materialGroups: usize = 0x168;
    pub const m_nDefaultMeshGroupMask: usize = 0x180;
    pub const m_modelSkeleton: usize = 0x188;
    pub const m_remappingTable: usize = 0x230;
    pub const m_remappingTableStarts: usize = 0x248;
    pub const m_boneFlexDrivers: usize = 0x260;
    pub const m_pModelConfigList: usize = 0x278;
    pub const m_BodyGroupsHiddenInTools: usize = 0x280;
    pub const m_refAnimIncludeModels: usize = 0x298;
    pub const m_AnimatedMaterialAttributes: usize = 0x2b0;
}

#[allow(non_snake_case, non_upper_case_globals)]
pub mod PermModelExtPart_t {
    pub const m_Transform: usize = 0x0;
    pub const m_Name: usize = 0x20;
    pub const m_nParent: usize = 0x28;
    pub const m_refModel: usize = 0x30;
}

#[allow(non_snake_case, non_upper_case_globals)]
pub mod PermModelInfo_t {
    pub const m_nFlags: usize = 0x0;
    pub const m_vHullMin: usize = 0x4;
    pub const m_vHullMax: usize = 0x10;
    pub const m_vViewMin: usize = 0x1c;
    pub const m_vViewMax: usize = 0x28;
    pub const m_flMass: usize = 0x34;
    pub const m_vEyePosition: usize = 0x38;
    pub const m_flMaxEyeDeflection: usize = 0x44;
    pub const m_sSurfaceProperty: usize = 0x48;
    pub const m_keyValueText: usize = 0x50;
}

#[allow(non_snake_case, non_upper_case_globals)]
pub mod PhysSoftbodyDesc_t {
    pub const m_ParticleBoneHash: usize = 0x0;
    pub const m_Particles: usize = 0x18;
    pub const m_Springs: usize = 0x30;
    pub const m_Capsules: usize = 0x48;
    pub const m_InitPose: usize = 0x60;
    pub const m_ParticleBoneName: usize = 0x78;
}

#[allow(non_snake_case, non_upper_case_globals)]
pub mod RenderSkeletonBone_t {
    pub const m_boneName: usize = 0x0;
    pub const m_parentName: usize = 0x8;
    pub const m_invBindPose: usize = 0x10;
    pub const m_bbox: usize = 0x40;
    pub const m_flSphereRadius: usize = 0x58;
}

#[allow(non_snake_case, non_upper_case_globals)]
pub mod SampleCode {
    pub const m_subCode: usize = 0x0;
}

#[allow(non_snake_case, non_upper_case_globals)]
pub mod ScriptInfo_t {
    pub const m_code: usize = 0x0;
    pub const m_paramsModified: usize = 0x8;
    pub const m_proxyReadParams: usize = 0x20;
    pub const m_proxyWriteParams: usize = 0x38;
    pub const m_eScriptType: usize = 0x50;
}

#[allow(non_snake_case, non_upper_case_globals)]
pub mod SkeletalInputOpFixedSettings_t {
    pub const m_wristBones: usize = 0x0;
    pub const m_fingers: usize = 0x18;
    pub const m_outerKnuckle1: usize = 0x30;
    pub const m_outerKnuckle2: usize = 0x34;
    pub const m_eHand: usize = 0x38;
    pub const m_eMotionRange: usize = 0x3c;
    pub const m_eTransformSource: usize = 0x40;
    pub const m_bEnableIK: usize = 0x44;
    pub const m_bEnableCollision: usize = 0x45;
}

#[allow(non_snake_case, non_upper_case_globals)]
pub mod SkeletonBoneBounds_t {
    pub const m_vecCenter: usize = 0x0;
    pub const m_vecSize: usize = 0xc;
}

#[allow(non_snake_case, non_upper_case_globals)]
pub mod SolveIKChainPoseOpFixedSettings_t {
    pub const m_ChainsToSolveData: usize = 0x0;
    pub const m_bMatchTargetOrientation: usize = 0x18;
}

#[allow(non_snake_case, non_upper_case_globals)]
pub mod StanceInfo_t {
    pub const m_vPosition: usize = 0x0;
    pub const m_flDirection: usize = 0xc;
}

#[allow(non_snake_case, non_upper_case_globals)]
pub mod TagSpan_t {
    pub const m_tagIndex: usize = 0x0;
    pub const m_startCycle: usize = 0x4;
    pub const m_endCycle: usize = 0x8;
}

#[allow(non_snake_case, non_upper_case_globals)]
pub mod TraceSettings_t {
    pub const m_flTraceHeight: usize = 0x0;
    pub const m_flTraceRadius: usize = 0x4;
}

#[allow(non_snake_case, non_upper_case_globals)]
pub mod TwoBoneIKSettings_t {
    pub const m_endEffectorType: usize = 0x0;
    pub const m_endEffectorAttachment: usize = 0x10;
    pub const m_targetType: usize = 0x90;
    pub const m_targetAttachment: usize = 0xa0;
    pub const m_targetBoneIndex: usize = 0x120;
    pub const m_hPositionParam: usize = 0x124;
    pub const m_hRotationParam: usize = 0x126;
    pub const m_bAlwaysUseFallbackHinge: usize = 0x128;
    pub const m_vLsFallbackHingeAxis: usize = 0x130;
    pub const m_nFixedBoneIndex: usize = 0x140;
    pub const m_nMiddleBoneIndex: usize = 0x144;
    pub const m_nEndBoneIndex: usize = 0x148;
    pub const m_bMatchTargetOrientation: usize = 0x14c;
    pub const m_bConstrainTwist: usize = 0x14d;
    pub const m_flMaxTwist: usize = 0x150;
}

#[allow(non_snake_case, non_upper_case_globals)]
pub mod VPhysXAggregateData_t {
    pub const m_nFlags: usize = 0x0;
    pub const m_nRefCounter: usize = 0x2;
    pub const m_bonesHash: usize = 0x8;
    pub const m_boneNames: usize = 0x20;
    pub const m_indexNames: usize = 0x38;
    pub const m_indexHash: usize = 0x50;
    pub const m_bindPose: usize = 0x68;
    pub const m_parts: usize = 0x80;
    pub const m_constraints2: usize = 0x98;
    pub const m_joints: usize = 0xb0;
    pub const m_pFeModel: usize = 0xc8;
    pub const m_boneParents: usize = 0xd0;
    pub const m_surfacePropertyHashes: usize = 0xe8;
    pub const m_collisionAttributes: usize = 0x100;
    pub const m_debugPartNames: usize = 0x118;
    pub const m_embeddedKeyvalues: usize = 0x130;
}

#[allow(non_snake_case, non_upper_case_globals)]
pub mod VPhysXBodyPart_t {
    pub const m_nFlags: usize = 0x0;
    pub const m_flMass: usize = 0x4;
    pub const m_rnShape: usize = 0x8;
    pub const m_nCollisionAttributeIndex: usize = 0x80;
    pub const m_nReserved: usize = 0x82;
    pub const m_flInertiaScale: usize = 0x84;
    pub const m_flLinearDamping: usize = 0x88;
    pub const m_flAngularDamping: usize = 0x8c;
    pub const m_bOverrideMassCenter: usize = 0x90;
    pub const m_vMassCenterOverride: usize = 0x94;
}

#[allow(non_snake_case, non_upper_case_globals)]
pub mod VPhysXCollisionAttributes_t {
    pub const m_CollisionGroup: usize = 0x0;
    pub const m_InteractAs: usize = 0x8;
    pub const m_InteractWith: usize = 0x20;
    pub const m_InteractExclude: usize = 0x38;
    pub const m_CollisionGroupString: usize = 0x50;
    pub const m_InteractAsStrings: usize = 0x58;
    pub const m_InteractWithStrings: usize = 0x70;
    pub const m_InteractExcludeStrings: usize = 0x88;
}

#[allow(non_snake_case, non_upper_case_globals)]
pub mod VPhysXConstraint2_t {
    pub const m_nFlags: usize = 0x0;
    pub const m_nParent: usize = 0x4;
    pub const m_nChild: usize = 0x6;
    pub const m_params: usize = 0x8;
}

#[allow(non_snake_case, non_upper_case_globals)]
pub mod VPhysXConstraintParams_t {
    pub const m_nType: usize = 0x0;
    pub const m_nTranslateMotion: usize = 0x1;
    pub const m_nRotateMotion: usize = 0x2;
    pub const m_nFlags: usize = 0x3;
    pub const m_anchor: usize = 0x4;
    pub const m_axes: usize = 0x1c;
    pub const m_maxForce: usize = 0x3c;
    pub const m_maxTorque: usize = 0x40;
    pub const m_linearLimitValue: usize = 0x44;
    pub const m_linearLimitRestitution: usize = 0x48;
    pub const m_linearLimitSpring: usize = 0x4c;
    pub const m_linearLimitDamping: usize = 0x50;
    pub const m_twistLowLimitValue: usize = 0x54;
    pub const m_twistLowLimitRestitution: usize = 0x58;
    pub const m_twistLowLimitSpring: usize = 0x5c;
    pub const m_twistLowLimitDamping: usize = 0x60;
    pub const m_twistHighLimitValue: usize = 0x64;
    pub const m_twistHighLimitRestitution: usize = 0x68;
    pub const m_twistHighLimitSpring: usize = 0x6c;
    pub const m_twistHighLimitDamping: usize = 0x70;
    pub const m_swing1LimitValue: usize = 0x74;
    pub const m_swing1LimitRestitution: usize = 0x78;
    pub const m_swing1LimitSpring: usize = 0x7c;
    pub const m_swing1LimitDamping: usize = 0x80;
    pub const m_swing2LimitValue: usize = 0x84;
    pub const m_swing2LimitRestitution: usize = 0x88;
    pub const m_swing2LimitSpring: usize = 0x8c;
    pub const m_swing2LimitDamping: usize = 0x90;
    pub const m_goalPosition: usize = 0x94;
    pub const m_goalOrientation: usize = 0xa0;
    pub const m_goalAngularVelocity: usize = 0xb0;
    pub const m_driveSpringX: usize = 0xbc;
    pub const m_driveSpringY: usize = 0xc0;
    pub const m_driveSpringZ: usize = 0xc4;
    pub const m_driveDampingX: usize = 0xc8;
    pub const m_driveDampingY: usize = 0xcc;
    pub const m_driveDampingZ: usize = 0xd0;
    pub const m_driveSpringTwist: usize = 0xd4;
    pub const m_driveSpringSwing: usize = 0xd8;
    pub const m_driveSpringSlerp: usize = 0xdc;
    pub const m_driveDampingTwist: usize = 0xe0;
    pub const m_driveDampingSwing: usize = 0xe4;
    pub const m_driveDampingSlerp: usize = 0xe8;
    pub const m_solverIterationCount: usize = 0xec;
    pub const m_projectionLinearTolerance: usize = 0xf0;
    pub const m_projectionAngularTolerance: usize = 0xf4;
}

#[allow(non_snake_case, non_upper_case_globals)]
pub mod VPhysXJoint_t {
    pub const m_nType: usize = 0x0;
    pub const m_nBody1: usize = 0x2;
    pub const m_nBody2: usize = 0x4;
    pub const m_nFlags: usize = 0x6;
    pub const m_Frame1: usize = 0x10;
    pub const m_Frame2: usize = 0x30;
    pub const m_bEnableCollision: usize = 0x50;
    pub const m_bEnableLinearLimit: usize = 0x51;
    pub const m_LinearLimit: usize = 0x54;
    pub const m_bEnableLinearMotor: usize = 0x5c;
    pub const m_vLinearTargetVelocity: usize = 0x60;
    pub const m_flMaxForce: usize = 0x6c;
    pub const m_bEnableSwingLimit: usize = 0x70;
    pub const m_SwingLimit: usize = 0x74;
    pub const m_bEnableTwistLimit: usize = 0x7c;
    pub const m_TwistLimit: usize = 0x80;
    pub const m_bEnableAngularMotor: usize = 0x88;
    pub const m_vAngularTargetVelocity: usize = 0x8c;
    pub const m_flMaxTorque: usize = 0x98;
    pub const m_flLinearFrequency: usize = 0x9c;
    pub const m_flLinearDampingRatio: usize = 0xa0;
    pub const m_flAngularFrequency: usize = 0xa4;
    pub const m_flAngularDampingRatio: usize = 0xa8;
    pub const m_flFriction: usize = 0xac;
}

#[allow(non_snake_case, non_upper_case_globals)]
pub mod VPhysXRange_t {
    pub const m_flMin: usize = 0x0;
    pub const m_flMax: usize = 0x4;
}

#[allow(non_snake_case, non_upper_case_globals)]
pub mod VPhysics2ShapeDef_t {
    pub const m_spheres: usize = 0x0;
    pub const m_capsules: usize = 0x18;
    pub const m_hulls: usize = 0x30;
    pub const m_meshes: usize = 0x48;
    pub const m_CollisionAttributeIndices: usize = 0x60;
}

#[allow(non_snake_case, non_upper_case_globals)]
pub mod WeightList {
    pub const m_name: usize = 0x0;
    pub const m_weights: usize = 0x8;
}

#[allow(non_snake_case, non_upper_case_globals)]
pub mod WristBone_t {
    pub const m_xOffsetTransformMS: usize = 0x0;
    pub const m_boneIndex: usize = 0x20;
}