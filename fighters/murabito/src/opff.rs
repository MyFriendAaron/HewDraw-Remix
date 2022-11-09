// opff import
utils::import_noreturn!(common::opff::fighter_common_opff);
use super::*;
use globals::*;

unsafe fn dspecial_cancels(boma: &mut BattleObjectModuleAccessor, status_kind: i32, situation_kind: i32, cat1: i32) {
    //PM-like down-b canceling
    if (status_kind == *FIGHTER_MURABITO_STATUS_KIND_SPECIAL_LW_WATER_AIR
        || status_kind == *FIGHTER_MURABITO_STATUS_KIND_SPECIAL_LW_WATER_DASH_B
        || status_kind == *FIGHTER_MURABITO_STATUS_KIND_SPECIAL_LW_WATER_DASH_F
        || status_kind == *FIGHTER_MURABITO_STATUS_KIND_SPECIAL_LW_WATER_JUMP
        || status_kind == *FIGHTER_MURABITO_STATUS_KIND_SPECIAL_LW_WATER_JUMP_SQUAT
        || status_kind == *FIGHTER_MURABITO_STATUS_KIND_SPECIAL_LW_WATER_LANDING
        || status_kind == *FIGHTER_MURABITO_STATUS_KIND_SPECIAL_LW_WATER_WAIT
        || status_kind == *FIGHTER_MURABITO_STATUS_KIND_SPECIAL_LW_WATER_WALK_B
        || status_kind == *FIGHTER_MURABITO_STATUS_KIND_SPECIAL_LW_WATER_WALK_BRAKE_B
        || status_kind == *FIGHTER_MURABITO_STATUS_KIND_SPECIAL_LW_WATER_WALK_BRAKE_F
        || status_kind == *FIGHTER_MURABITO_STATUS_KIND_SPECIAL_LW_WATER_WALK_F) {
            if ControlModule::check_button_trigger(boma, *CONTROL_PAD_BUTTON_GUARD) {
                if situation_kind == *SITUATION_KIND_AIR {
                    WorkModule::unable_transition_term_group(boma, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_ESCAPE);
                    ControlModule::clear_command_one(boma, *FIGHTER_PAD_COMMAND_CATEGORY1, *FIGHTER_PAD_CMD_CAT1_AIR_ESCAPE);
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_FALL, false);
                }
                if situation_kind == *SITUATION_KIND_GROUND {
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_WAIT, false);
                }
            }
    }
}
 
pub unsafe fn moveset(boma: &mut BattleObjectModuleAccessor, id: usize, cat: [i32 ; 4], status_kind: i32, situation_kind: i32, motion_kind: u64, stick_x: f32, stick_y: f32, facing: f32, frame: f32) {
    dspecial_cancels(boma, status_kind, situation_kind, cat[0]);
}

#[utils::macros::opff(FIGHTER_KIND_MURABITO )]
pub fn murabito_frame_wrapper(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    unsafe {
        common::opff::fighter_common_opff(fighter);
		murabito_frame(fighter);
    }
}

pub unsafe fn murabito_frame(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    if let Some(info) = FrameInfo::update_and_get(fighter) {
        moveset(&mut *info.boma, info.id, info.cat, info.status_kind, info.situation_kind, info.motion_kind.hash, info.stick_x, info.stick_y, info.facing, info.frame);
    }
}