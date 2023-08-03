use super::*;

unsafe extern "C" fn trail_special_lw_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.is_situation(*SITUATION_KIND_AIR) {
        // Taken from vanilla down air
        if fighter.status_AttackAir_Main_common().get_bool() {
            return 1.into();
        }
        else {
            fighter.sub_air_check_superleaf_fall_slowly();
            let is_stopping = fighter.global_table[globals::IS_STOPPING].get_bool();
            if !is_stopping {
                fighter.sub_attack_air_inherit_jump_aerial_motion_uniq_process_exec_fix_pos();
            }
        }
        //
    }
    if fighter.is_situation(*SITUATION_KIND_GROUND) {
        fighter.change_status(FIGHTER_STATUS_KIND_LANDING.into(), false.into());
    }
    return 0.into();
}

#[status_script(agent = "trail", status = FIGHTER_STATUS_KIND_SPECIAL_LW, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn trail_special_lw_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.is_situation(*SITUATION_KIND_AIR) {
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
        // Taken from vanilla down air
        fighter.sub_attack_air_common(false.into());
        MotionModule::set_trans_move_speed_no_scale(fighter.module_accessor, true);
        //
    }
    if fighter.is_situation(*SITUATION_KIND_GROUND) {
        fighter.change_status(FIGHTER_STATUS_KIND_LANDING.into(), false.into());
    }
    fighter.main_shift(trail_special_lw_main_loop)
}

#[status_script(agent = "trail", status = FIGHTER_STATUS_KIND_SPECIAL_LW, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_PRE)]
unsafe fn trail_special_lw_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(
        fighter.module_accessor,
        app::SituationKind(*SITUATION_KIND_NONE),
        *FIGHTER_KINETIC_TYPE_UNIQ,
        *GROUND_CORRECT_KIND_KEEP as u32,
        app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE),
        true,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT,
        0
    );

    FighterStatusModuleImpl::set_fighter_status_data(
        fighter.module_accessor,
        false,
        *FIGHTER_TREADED_KIND_NO_REAC,
        false,
        false,
        false,
        *FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_LW as u64,
        *FIGHTER_STATUS_ATTR_START_TURN as u32,
        *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_N as u32,
        0
    );
    return 0.into();
}

pub fn install() {
    install_status_scripts!(
        trail_special_lw_pre, trail_special_lw_main
    );
}