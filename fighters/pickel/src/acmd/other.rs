
use super::*;

#[acmd_script( agent = "pickel", script = "sound_damageflyhi" , category = ACMD_SOUND , low_priority)]
unsafe fn damageflyhi_sound(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        if !StopModule::is_stop(fighter.module_accessor) {
            let play_vc = if DamageModule::reaction(boma, 0) < 100.0 {
                app::sv_math::rand(hash40("fighter"), 3)
            } else {
                0
            };
            if play_vc == 0 {PLAY_FLY_VOICE(fighter, Hash40::new("seq_pickel_rnd_futtobi01"), Hash40::new("seq_pickel_rnd_futtobi02"));}
        }
    }
    frame(lua_state, 1.1);
    if is_excute(fighter) {
        let play_vc = if DamageModule::reaction(boma, 0) < 100.0 {
            app::sv_math::rand(hash40("fighter"), 3)
        } else {
            0
        };
        if play_vc == 0 {PLAY_FLY_VOICE(fighter, Hash40::new("seq_pickel_rnd_futtobi01"), Hash40::new("seq_pickel_rnd_futtobi02"));}
    }
}

#[acmd_script( agent = "pickel", script = "sound_damageflylw" , category = ACMD_SOUND , low_priority)]
unsafe fn damageflylw_sound(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        if !StopModule::is_stop(fighter.module_accessor) {
            let play_vc = if DamageModule::reaction(boma, 0) < 100.0 {
                app::sv_math::rand(hash40("fighter"), 3)
            } else {
                0
            };
            if play_vc == 0 {PLAY_FLY_VOICE(fighter, Hash40::new("seq_pickel_rnd_futtobi01"), Hash40::new("seq_pickel_rnd_futtobi02"));}
        }
    }
    frame(lua_state, 1.1);
    if is_excute(fighter) {
        let play_vc = if DamageModule::reaction(boma, 0) < 100.0 {
            app::sv_math::rand(hash40("fighter"), 3)
        } else {
            0
        };
        if play_vc == 0 {PLAY_FLY_VOICE(fighter, Hash40::new("seq_pickel_rnd_futtobi01"), Hash40::new("seq_pickel_rnd_futtobi02"));}
    }
}

#[acmd_script( agent = "pickel", script = "sound_damageflyn" , category = ACMD_SOUND , low_priority)]
unsafe fn damageflyn_sound(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        if !StopModule::is_stop(fighter.module_accessor) {
            let play_vc = if DamageModule::reaction(boma, 0) < 100.0 {
                app::sv_math::rand(hash40("fighter"), 3)
            } else {
                0
            };
            if play_vc == 0 {PLAY_FLY_VOICE(fighter, Hash40::new("seq_pickel_rnd_futtobi01"), Hash40::new("seq_pickel_rnd_futtobi02"));}
        }
    }
    frame(lua_state, 1.1);
    if is_excute(fighter) {
        let play_vc = if DamageModule::reaction(boma, 0) < 100.0 {
            app::sv_math::rand(hash40("fighter"), 3)
        } else {
            0
        };
        if play_vc == 0 {PLAY_FLY_VOICE(fighter, Hash40::new("seq_pickel_rnd_futtobi01"), Hash40::new("seq_pickel_rnd_futtobi02"));}
    }
}

#[acmd_script( agent = "pickel", script = "sound_damageflyroll" , category = ACMD_SOUND , low_priority)]
unsafe fn damageflyroll_sound(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        if !StopModule::is_stop(fighter.module_accessor) {
            PLAY_FLY_VOICE(fighter, Hash40::new("seq_pickel_rnd_futtobi01"), Hash40::new("seq_pickel_rnd_futtobi02"));
        }
    }
    frame(lua_state, 1.1);
    if is_excute(fighter) {
        PLAY_FLY_VOICE(fighter, Hash40::new("seq_pickel_rnd_futtobi01"), Hash40::new("seq_pickel_rnd_futtobi02"));
    }
}

#[acmd_script( agent = "pickel", script = "sound_damageflytop" , category = ACMD_SOUND , low_priority)]
unsafe fn damageflytop_sound(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        if !StopModule::is_stop(fighter.module_accessor) {
            let play_vc = if DamageModule::reaction(boma, 0) < 100.0 {
                app::sv_math::rand(hash40("fighter"), 3)
            } else {
                0
            };
            if play_vc == 0 {PLAY_FLY_VOICE(fighter, Hash40::new("seq_pickel_rnd_futtobi01"), Hash40::new("seq_pickel_rnd_futtobi02"));}
        }
    }
    frame(lua_state, 1.1);
    if is_excute(fighter) {
        let play_vc = if DamageModule::reaction(boma, 0) < 100.0 {
            app::sv_math::rand(hash40("fighter"), 3)
        } else {
            0
        };
        if play_vc == 0 {PLAY_FLY_VOICE(fighter, Hash40::new("seq_pickel_rnd_futtobi01"), Hash40::new("seq_pickel_rnd_futtobi02"));}
    }
}

#[acmd_script( agent = "pickel", script = "game_dash" , category = ACMD_GAME , low_priority)]
unsafe fn dash_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 1.4);
    }
	frame(lua_state, 11.0); // Effectively F15
    if is_excute(fighter) {
		FT_MOTION_RATE(fighter, 1.0);
		WorkModule::enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_DASH_TO_RUN);
    }
    
}

#[acmd_script( agent = "pickel", script = "sound_dash" , category = ACMD_SOUND , low_priority)]
unsafe fn dash_sound(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 4.0);
    if is_excute(fighter) {
        let dash_sfx_handle = SoundModule::play_se(fighter.module_accessor, Hash40::new("se_pickel_dash_start"), true, false, false, false, app::enSEType(0));
        SoundModule::set_se_vol(boma, dash_sfx_handle as i32, 0.5, 0);
    }
}

#[acmd_script( agent = "pickel", script = "game_turndash" , category = ACMD_GAME , low_priority)]
unsafe fn turn_dash_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 3.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 1.2);
		WorkModule::on_flag(boma, *FIGHTER_STATUS_DASH_FLAG_TURN_DASH);
    }
    frame(lua_state, 6.0);
    if is_excute(fighter) {
        //WorkModule::on_flag(boma, 60192);
		WorkModule::on_flag(boma, *FIGHTER_PICKEL_INSTANCE_WORK_ID_FLAG_BLEND_TURN);
    }
    frame(lua_state, 13.0); // Effectively F15
    if is_excute(fighter) {
		FT_MOTION_RATE(fighter, 1.0);
        WorkModule::enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_DASH_TO_RUN);
    }
    
}

#[acmd_script( agent = "pickel_fire", script = "game_attacklw3" , category = ACMD_GAME , low_priority)]
unsafe fn pickel_fire_attack_lw3_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    let owner_module_accessor = &mut *sv_battle_object::module_accessor((WorkModule::get_int(boma, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
    if is_excute(fighter) {
        AttackModule::disable_tip(boma);
        VarModule::off_flag(owner_module_accessor.object(), vars::pickel::instance::IS_CURRENT_ATTACK_LW3_SOUL_FIRE);
        if VarModule::is_flag(owner_module_accessor.object(), vars::common::instance::IS_HEAVY_ATTACK){
            VarModule::on_flag(owner_module_accessor.object(), vars::pickel::instance::IS_CURRENT_ATTACK_LW3_SOUL_FIRE);
        }
        else{
            VarModule::off_flag(owner_module_accessor.object(), vars::pickel::instance::IS_CURRENT_ATTACK_LW3_SOUL_FIRE);
        }
        if VarModule::is_flag(owner_module_accessor.object(), vars::pickel::instance::IS_CURRENT_ATTACK_LW3_SOUL_FIRE) {
            //FT_MOTION_RATE(fighter, 1.25);
            ATTACK(fighter, 0, 0, Hash40::new("top"), 0.8, 366, 100, 40, 0, 3.2, 0.0, 2.8, 2.0, None, None, None, 0.7, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 8, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_aura"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_OBJECT);
            ATTACK(fighter, 1, 0, Hash40::new("top"), 0.8, 366, 100, 40, 0, 3.2, 0.0, 2.8, -2.0, None, None, None, 0.7, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 8, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_aura"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_OBJECT);
        }
        else{
            FT_MOTION_RATE(fighter, 0.75);
            ATTACK(fighter, 0, 0, Hash40::new("top"), 7.0, 70, 60, 0, 75, 3.2, 0.0, 2.8, 2.0, None, None, None, 1.75, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_OBJECT);
            ATTACK(fighter, 1, 0, Hash40::new("top"), 7.0, 70, 60, 0, 75, 3.2, 0.0, 2.8, -2.0, None, None, None, 1.75, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_OBJECT);
        }
        AttackModule::enable_safe_pos(boma);
    }
    frame(lua_state, 24.0);
    if is_excute(fighter) {
        if VarModule::is_flag(owner_module_accessor.object(), vars::pickel::instance::IS_CURRENT_ATTACK_LW3_SOUL_FIRE){
            FT_MOTION_RATE(fighter, 1.0);
            ATTACK(fighter, 0, 1, Hash40::new("top"), 6.4, 54, 116, 0, 42, 3.5, 0.0, 2.8, 2.0, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, -3, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_aura"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_OBJECT);
            ATTACK(fighter, 1, 1, Hash40::new("top"), 6.4, 54, 116, 0, 42, 3.5, 0.0, 2.8, -2.0, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, -3, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_aura"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_OBJECT);
        }
    }
    frame(lua_state, 28.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 45.0);
    if is_excute(fighter) {
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x199c462b5d));
    }
}

#[acmd_script( agent = "pickel", script = "game_escapeair" , category = ACMD_GAME , low_priority)]
unsafe fn escape_air_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    let escape_air_cancel_frame = WorkModule::get_param_float(boma, hash40("param_motion"), hash40("escape_air_cancel_frame"));

    frame(lua_state, 29.0);
    if is_excute(fighter) {
        KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_FALL);
    }
    frame(lua_state, escape_air_cancel_frame);
    if is_excute(fighter) {
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
    }
}

#[acmd_script( agent = "pickel", script = "game_escapeairslide" , category = ACMD_GAME , low_priority)]
unsafe fn escape_air_slide_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    
    frame(lua_state, 29.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ESCAPE_AIR_FLAG_SLIDE_ENABLE_CONTROL);
    }
    frame(lua_state, 39.0);
    if is_excute(fighter) {
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
    }
}

#[acmd_script( agent = "pickel_fire", script = "effect_attacklw3" , category = ACMD_EFFECT , low_priority)]
unsafe fn pickel_fire_attack_lw3_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    let owner_module_accessor = &mut *sv_battle_object::module_accessor((WorkModule::get_int(boma, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
    if is_excute(fighter) {
        if VarModule::is_flag(owner_module_accessor.object(), vars::pickel::instance::IS_CURRENT_ATTACK_LW3_SOUL_FIRE){
            EFFECT_FOLLOW(fighter, Hash40::new("pickel_fire_soot"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, false);
            EFFECT_FOLLOW(fighter, Hash40::new("pickel_fire"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, false);
            LAST_EFFECT_SET_COLOR(fighter, 0.29, 0.941, 0.957);
        }
        else {
            EFFECT_FOLLOW(fighter, Hash40::new("pickel_fire_soot"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, false);
            EFFECT_FOLLOW(fighter, Hash40::new("pickel_fire"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, false);
            LAST_EFFECT_SET_COLOR(fighter, 1.0, 0.568, 0.0);
        }
    }
    frame(lua_state, 30.0);
    if is_excute(fighter) {
        EFFECT_DETACH_KIND(fighter, Hash40::new("pickel_fire_soot"), -1);
    }
    frame(lua_state, 38.0);
    if is_excute(fighter) {
        EFFECT_DETACH_KIND(fighter, Hash40::new("pickel_fire"), -1);
    }
}

#[acmd_script( agent = "pickel", script = "game_appealsl", category = ACMD_GAME, low_priority )]
unsafe fn pickel_appealsl_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_PICKEL_INSTANCE_WORK_ID_FLAG_REQUEST_REMOVE_HAVE_CRAFT_WEAPON);
    }
    frame(lua_state, 90.0);
    if is_excute(fighter){
        if DamageModule::damage(boma, 0) > 2.0 {
            DamageModule::add_damage(boma, -2.0, 0);
        }
    }
}

#[acmd_script( agent = "pickel", script = "game_appealsr", category = ACMD_GAME, low_priority )]
unsafe fn pickel_appealsr_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_PICKEL_INSTANCE_WORK_ID_FLAG_REQUEST_REMOVE_HAVE_CRAFT_WEAPON);
    }
    frame(lua_state, 90.0);
    if is_excute(fighter){
        if DamageModule::damage(boma, 0) > 2.0 {
            DamageModule::add_damage(boma, -2.0, 0);
        }
    }
}


#[acmd_script( agent = "pickel", script = "game_catch", category = ACMD_GAME, low_priority )]
unsafe fn pickel_catch_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_PICKEL_INSTANCE_WORK_ID_FLAG_REQUEST_REMOVE_HAVE_CRAFT_WEAPON);
        ArticleModule::generate_article(boma, *FIGHTER_PICKEL_GENERATE_ARTICLE_FISHINGROD, false, -1);
        ArticleModule::change_motion(boma, *FIGHTER_PICKEL_GENERATE_ARTICLE_FISHINGROD, Hash40::new("catch"), false, -1.0);
    }
    frame(lua_state, 12.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_PICKEL_STATUS_CATCH_FLAG_SHOOT);
        GrabModule::set_rebound(boma, true);
    }
    frame(lua_state, 13.0);
    if is_excute(fighter) {        
        ArticleModule::set_visibility(boma, *FIGHTER_PICKEL_GENERATE_ARTICLE_FISHINGROD, Hash40::new("rod"), Hash40::new("rod_cast"), smash::app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
        CATCH(fighter, 0, Hash40::new("throw"), 2.5, 0.0, -4.0, -1.0, Some(0.0), Some(-4.8), Some(-1.0), *FIGHTER_STATUS_KIND_CAPTURE_PULLED_PICKEL, *COLLISION_SITUATION_MASK_GA);
        CATCH(fighter, 1, Hash40::new("top"), 1.8, 0.0, 7.0, 3.2, Some(0.0), Some(7.0), Some(10.8), *FIGHTER_STATUS_KIND_CAPTURE_PULLED_PICKEL, *COLLISION_SITUATION_MASK_A);
        CATCH(fighter, 2, Hash40::new("top"), 3.6, 0.0, 7.0, 5.0, Some(0.0), Some(7.0), Some(9.0), *FIGHTER_STATUS_KIND_CAPTURE_PULLED_PICKEL, *COLLISION_SITUATION_MASK_G);
    }
    game_CaptureCutCommon(fighter);
    frame(lua_state, 17.0);
    if is_excute(fighter) {
        grab!(fighter, *MA_MSC_CMD_GRAB_CLEAR, 1);
        grab!(fighter, *MA_MSC_CMD_GRAB_CLEAR, 2);
    }
    frame(lua_state, 22.0);
    if is_excute(fighter) {
        CATCH(fighter, 0, Hash40::new("throw"), 2.5, 0.0, -5.0, -1.0, Some(0.0), Some(-4.0), Some(-1.0), *FIGHTER_STATUS_KIND_CAPTURE_PULLED_PICKEL, *COLLISION_SITUATION_MASK_GA);
    }
    frame(lua_state, 24.0);
    if is_excute(fighter) {
        CATCH(fighter, 0, Hash40::new("throw"), 2.5, 0.0, -3.0, -1.0, Some(0.0), Some(-0.5), Some(-1.0), *FIGHTER_STATUS_KIND_CAPTURE_PULLED_PICKEL, *COLLISION_SITUATION_MASK_GA);
    }
    frame(lua_state, 26.0);
    if is_excute(fighter) {
        CATCH(fighter, 0, Hash40::new("throw"), 2.5, 0.0, -3.0, -1.0, Some(0.0), Some(0.0), Some(-1.0), *FIGHTER_STATUS_KIND_CAPTURE_PULLED_PICKEL, *COLLISION_SITUATION_MASK_GA);
    }
    frame(lua_state, 28.0);
    if is_excute(fighter) {
        grab!(fighter, *MA_MSC_CMD_GRAB_CLEAR_ALL);
        WorkModule::on_flag(boma, *FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT);
        GrabModule::set_rebound(boma, false);
    }
    frame(lua_state, 38.0);
    if is_excute(fighter) {
        ArticleModule::set_flag(boma, *FIGHTER_PICKEL_GENERATE_ARTICLE_FISHINGROD, true, *WEAPON_PICKEL_FISHINGROD_INSTANCE_WORK_ID_FLAG_ENABLE_REWIND);
    }
    frame(lua_state, 45.0);
    FT_MOTION_RATE(fighter, 2.0);
}

#[acmd_script( agent = "pickel", script = "game_catchdash", category = ACMD_GAME, low_priority )]
unsafe fn pickel_catchdash_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_PICKEL_INSTANCE_WORK_ID_FLAG_REQUEST_REMOVE_HAVE_CRAFT_WEAPON);
        ArticleModule::generate_article(boma, *FIGHTER_PICKEL_GENERATE_ARTICLE_FISHINGROD, false, -1);
        ArticleModule::change_motion(boma, *FIGHTER_PICKEL_GENERATE_ARTICLE_FISHINGROD, Hash40::new("catch_dash"), false, -1.0);
    }
    frame(lua_state, 15.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_PICKEL_STATUS_CATCH_FLAG_SHOOT);
        GrabModule::set_rebound(boma, true);
    }
    frame(lua_state, 16.0);
    if is_excute(fighter) {
        ArticleModule::set_visibility(boma, *FIGHTER_PICKEL_GENERATE_ARTICLE_FISHINGROD, Hash40::new("rod"), Hash40::new("rod_cast"), smash::app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
        CATCH(fighter, 0, Hash40::new("throw"), 2.5, 0.0, -5.0, -1.0, Some(0.0), Some(-4.8), Some(-1.0), *FIGHTER_STATUS_KIND_CAPTURE_PULLED_PICKEL, *COLLISION_SITUATION_MASK_GA);
        CATCH(fighter, 1, Hash40::new("top"), 1.8, 0.0, 7.0, 3.2, Some(0.0), Some(7.0), Some(10.8), *FIGHTER_STATUS_KIND_CAPTURE_PULLED_PICKEL, *COLLISION_SITUATION_MASK_A);
        CATCH(fighter, 2, Hash40::new("top"), 3.6, 0.0, 7.0, 5.0, Some(0.0), Some(7.0), Some(9.0), *FIGHTER_STATUS_KIND_CAPTURE_PULLED_PICKEL, *COLLISION_SITUATION_MASK_G);
    }
    game_CaptureCutCommon(fighter);
    frame(lua_state, 18.0);
    if is_excute(fighter) {
        grab!(fighter, *MA_MSC_CMD_GRAB_CLEAR, 1);
        grab!(fighter, *MA_MSC_CMD_GRAB_CLEAR, 2);
    }
    frame(lua_state, 25.0);
    if is_excute(fighter) {
        CATCH(fighter, 0, Hash40::new("throw"), 2.5, 0.0, -5.0, -1.0, Some(0.0), Some(-4.0), Some(-1.0), *FIGHTER_STATUS_KIND_CAPTURE_PULLED_PICKEL, *COLLISION_SITUATION_MASK_GA);
    }
    frame(lua_state, 27.0);
    if is_excute(fighter) {
        CATCH(fighter, 0, Hash40::new("throw"), 2.5, 0.0, -3.0, -1.0, Some(0.0), Some(-0.5), Some(-1.0), *FIGHTER_STATUS_KIND_CAPTURE_PULLED_PICKEL, *COLLISION_SITUATION_MASK_GA);
    }
    frame(lua_state, 29.0);
    if is_excute(fighter) {
        CATCH(fighter, 0, Hash40::new("throw"), 2.5, 0.0, -3.0, -1.0, Some(0.0), Some(0.0), Some(-1.0), *FIGHTER_STATUS_KIND_CAPTURE_PULLED_PICKEL, *COLLISION_SITUATION_MASK_GA);
    }
    frame(lua_state, 31.0);
    if is_excute(fighter) {
        grab!(fighter, *MA_MSC_CMD_GRAB_CLEAR_ALL);
        WorkModule::on_flag(boma, *FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT);
        GrabModule::set_rebound(boma, false);
    }
    frame(lua_state, 42.0);
    if is_excute(fighter) {
        ArticleModule::set_flag(boma, *FIGHTER_PICKEL_GENERATE_ARTICLE_FISHINGROD, true, *WEAPON_PICKEL_FISHINGROD_INSTANCE_WORK_ID_FLAG_ENABLE_REWIND);
    }
    frame(lua_state, 51.0);
    FT_MOTION_RATE(fighter, 2.0);
}

#[acmd_script( agent = "pickel", script = "game_catchturn", category = ACMD_GAME, low_priority )]
unsafe fn pickel_catchturn_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_PICKEL_INSTANCE_WORK_ID_FLAG_REQUEST_REMOVE_HAVE_CRAFT_WEAPON);
        ArticleModule::generate_article(boma, *FIGHTER_PICKEL_GENERATE_ARTICLE_FISHINGROD, false, -1);
        ArticleModule::change_motion(boma, *FIGHTER_PICKEL_GENERATE_ARTICLE_FISHINGROD, Hash40::new("catch_turn"), false, -1.0);
    }
    frame(lua_state, 16.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_PICKEL_STATUS_CATCH_FLAG_SHOOT);
        GrabModule::set_rebound(boma, true);
    }
    frame(lua_state, 17.0);
    if is_excute(fighter) {
        ArticleModule::set_visibility(boma, *FIGHTER_PICKEL_GENERATE_ARTICLE_FISHINGROD, Hash40::new("rod"), Hash40::new("rod_cast"), smash::app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
        CATCH(fighter, 0, Hash40::new("throw"), 2.5, 0.0, -5.0, 1.0, Some(0.0), Some(-4.8), Some(1.0), *FIGHTER_STATUS_KIND_CAPTURE_PULLED_PICKEL, *COLLISION_SITUATION_MASK_GA);
        CATCH(fighter, 1, Hash40::new("top"), 1.8, 0.0, 7.0, -3.2, Some(0.0), Some(7.0), Some(-10.8), *FIGHTER_STATUS_KIND_CAPTURE_PULLED_PICKEL, *COLLISION_SITUATION_MASK_A);
        CATCH(fighter, 2, Hash40::new("top"), 3.6, 0.0, 7.0, -5.0, Some(0.0), Some(7.0), Some(-9.0), *FIGHTER_STATUS_KIND_CAPTURE_PULLED_PICKEL, *COLLISION_SITUATION_MASK_G);
    }
    game_CaptureCutCommon(fighter);
    frame(lua_state, 19.0);
    if is_excute(fighter) {
        grab!(fighter, *MA_MSC_CMD_GRAB_CLEAR, 1);
        grab!(fighter, *MA_MSC_CMD_GRAB_CLEAR, 2);
    }
    frame(lua_state, 26.0);
    if is_excute(fighter) {
        CATCH(fighter, 0, Hash40::new("throw"), 2.5, 0.0, -5.0, 1.0, Some(0.0), Some(-4.0), Some(1.0), *FIGHTER_STATUS_KIND_CAPTURE_PULLED_PICKEL, *COLLISION_SITUATION_MASK_GA);
    }
    frame(lua_state, 28.0);
    if is_excute(fighter) {
        CATCH(fighter, 0, Hash40::new("throw"), 2.5, 0.0, -3.0, 1.0, Some(0.0), Some(-0.5), Some(1.0), *FIGHTER_STATUS_KIND_CAPTURE_PULLED_PICKEL, *COLLISION_SITUATION_MASK_GA);
    }
    frame(lua_state, 30.0);
    if is_excute(fighter) {
        CATCH(fighter, 0, Hash40::new("throw"), 2.5, 0.0, -3.0, -1.0, Some(0.0), Some(0.0), Some(-1.0), *FIGHTER_STATUS_KIND_CAPTURE_PULLED_PICKEL, *COLLISION_SITUATION_MASK_GA);
    }
    frame(lua_state, 32.0);
    if is_excute(fighter) {
        grab!(fighter, *MA_MSC_CMD_GRAB_CLEAR_ALL);
        WorkModule::on_flag(boma, *FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT);
        GrabModule::set_rebound(boma, false);
    }
    frame(lua_state, 42.0);
    if is_excute(fighter) {
        ArticleModule::set_flag(boma, *FIGHTER_PICKEL_GENERATE_ARTICLE_FISHINGROD, true, *WEAPON_PICKEL_FISHINGROD_INSTANCE_WORK_ID_FLAG_ENABLE_REWIND);
    }
    frame(lua_state, 52.0);
    FT_MOTION_RATE(fighter, 2.0);
}

#[acmd_script( agent = "pickel_trolley", script = "game_specialsdriveemptypartial", category = ACMD_GAME, low_priority )]
unsafe fn pickel_trolley_drive_empty(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
        WorkModule::on_flag(boma, *WEAPON_PICKEL_TROLLEY_INSTANCE_WORK_ID_FLAG_NO_ATTACK_HIT_MOTION);
        ATTACK(fighter, 0, 0, Hash40::new("top"), 7.0, 60, 64, 0, 66, 1.0, 0.0, 5.0, 6.0, Some(0.0), Some(0.0), Some(6.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_SPEED, false, 0, 0.0, 60, true, false, false, false, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
        AttackModule::set_ignore_ground_shield(boma, 0, true);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 7.0, 60, 64, 0, 66, 1.0, 0.0, 5.0, -6.0, Some(0.0), Some(0.0), Some(-6.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_SPEED, false, 0, 0.0, 60, true, false, false, false, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
        AttackModule::set_ignore_ground_shield(boma, 1, true);
        ATTACK(fighter, 2, 0, Hash40::new("top"), 7.0, 60, 64, 0, 66, 1.0, 0.0, 0.0, 6.0, Some(0.0), Some(0.0), Some(-6.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_SPEED, false, 0, 0.0, 60, true, false, false, false, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
        AttackModule::set_ignore_ground_shield(boma, 2, true);
        ATTACK(fighter, 3, 0, Hash40::new("top"), 7.0, 60, 64, 0, 66, 3.5, 0.0, 3.0, 3.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_SPEED, false, 0, 0.0, 60, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_IIE, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
        ATTACK(fighter, 4, 0, Hash40::new("top"), 7.0, 60, 64, 0, 66, 3.5, 0.0, 3.0, 3.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_SPEED, false, f32::NAN, 0.0, 60, false, false, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_FIGHTER, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
        AttackModule::set_down_only(boma, 4, true);
    }
    wait(lua_state, 6.0);
    if is_excute(fighter) {
        ATTACK(fighter, 5, 0, Hash40::new("top"), 7.0, 60, 64, 0, 66, 1.5, 0.0, 3.0, 1.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_SPEED, false, f32::NAN, 0.0, 60, false, false, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_FIGHTER, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
        WorkModule::off_flag(boma, *WEAPON_PICKEL_TROLLEY_INSTANCE_WORK_ID_FLAG_NO_ATTACK_HIT_MOTION);
    }
}

#[acmd_script( agent = "pickel_forge", script = "game_fall", category = ACMD_GAME, low_priority )]
unsafe fn forge_fall_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        JostleModule::set_status(boma, false);
    }
    frame(lua_state, 8.0);
    if is_excute(fighter) {
        JostleModule::set_status(boma, true);
        WorkModule::on_flag(boma, *WEAPON_PICKEL_FORGE_INSTANCE_WORK_ID_FLAG_UPDATE_ATTACK);
    }
}

#[acmd_script( agent = "pickel_forge", script = "game_fallattackride", category = ACMD_GAME, low_priority )]
unsafe fn forge_fall_attack_ride_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    let owner_id = WorkModule::get_int(fighter.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER) as u32;
    let pickel = utils::util::get_battle_object_from_id(owner_id);
    let pickel_boma = &mut *(*pickel).module_accessor;
    
    let fall_distance = VarModule::get_float(pickel, vars::pickel::instance::FORGE_START_Y_POS) - PostureModule::pos_y(boma);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 10.0 + (fall_distance / 3.2), 70, (60.0 - (fall_distance / 2.5)), 0, 62, 6.4, 0.0, 4.6, 0.0, Some(0.0), Some(4.8), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 1, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_OBJECT);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 10.0 + (fall_distance / 3.2), 58, (60.0 - (fall_distance / 2.5)), 0, 62, 6.4, 0.0, 4.6, 0.0, Some(0.0), Some(4.8), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 1, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_OBJECT);
        AttackModule::set_attack_height_all(boma, AttackHeight(*ATTACK_HEIGHT_HIGH), false);
    }
}

#[acmd_script( agent = "pickel_forge", script = "game_fallattack", category = ACMD_GAME, low_priority )]
unsafe fn forge_fall_attack_game(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    let owner_id = WorkModule::get_int(agent.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER) as u32;
    let pickel = utils::util::get_battle_object_from_id(owner_id);
    let fall_distance = VarModule::get_float(pickel, vars::pickel::instance::FORGE_START_Y_POS) - PostureModule::pos_y(boma);
    if is_excute(agent) {
        if sv_battle_object::kind(owner_id) == *FIGHTER_KIND_PICKEL {
            let pickel = utils::util::get_battle_object_from_id(owner_id);
            let pickel_boma = &mut *(*pickel).module_accessor;
            if pickel_boma.is_motion_one_of(&[Hash40::new("attack_air_lw"),
            Hash40::new("attack_air_lw_2"),
            Hash40::new("attack_air_lw_fall"),]){
                //below hitbox shows for 1 frame if this isnt here lol
           }
            else{
                wait(lua_state, 2.0);
                ATTACK(agent, 0, 0, Hash40::new("top"), 5.0 + (fall_distance / 3.2) , 70, (60.0 - (fall_distance / 1.25)), 0, 62, 6.4, 0.0, 4.6, 0.0, Some(0.0), Some(4.8), Some(0.0), 0.7, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 1, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_OBJECT);
                ATTACK(agent, 1, 0, Hash40::new("top"), 5.0 + (fall_distance / 3.2) , 58, (60.0 - (fall_distance / 1.25)), 0, 62, 6.4, 0.0, 4.6, 0.0, Some(0.0), Some(4.8), Some(0.0), 0.7, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 1, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_OBJECT);
                AttackModule::set_attack_height_all(boma, AttackHeight(*ATTACK_HEIGHT_HIGH), false);
            }
        }
        else{
            ATTACK(agent, 0, 0, Hash40::new("top"), 5.0 + (fall_distance / 3.2) , 70, (60.0 - (fall_distance / 1.25)), 0, 62, 6.4, 0.0, 4.6, 0.0, Some(0.0), Some(4.8), Some(0.0), 0.7, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 1, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_OBJECT);
            ATTACK(agent, 1, 0, Hash40::new("top"), 5.0 + (fall_distance / 3.2) , 58, (60.0 - (fall_distance / 1.25)), 0, 62, 6.4, 0.0, 4.6, 0.0, Some(0.0), Some(4.8), Some(0.0), 0.7, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 1, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_OBJECT);
            AttackModule::set_attack_height_all(boma, AttackHeight(*ATTACK_HEIGHT_HIGH), false);
        }
    }
}

pub fn install() {
    install_acmd_scripts!(
        escape_air_game,
        escape_air_slide_game,
        dash_game,
        dash_sound,
        turn_dash_game,
        damageflyhi_sound,
        damageflylw_sound,
        damageflyn_sound,
        damageflyroll_sound,
        damageflytop_sound,
        pickel_appealsl_game,
        pickel_appealsr_game,
        pickel_catch_game,
        pickel_catchdash_game,
        pickel_catchturn_game,

        pickel_fire_attack_lw3_game,
        pickel_fire_attack_lw3_effect,
        pickel_trolley_drive_empty,
        forge_fall_game,
        forge_fall_attack_ride_game,
        forge_fall_attack_game,

    );
}

