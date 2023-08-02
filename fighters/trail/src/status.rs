use super::*;
use globals::*;
// status script import

mod attack_air;
mod special_s;
mod special_lw;

pub fn install() {
    attack_air::install();
    special_s::install();
    special_lw::install();
}