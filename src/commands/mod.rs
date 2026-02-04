use crate::runtime::core_registry::register;

pub mod help;
use help::help_cmd;

pub mod clear;
use clear::clear_cmd;


pub fn init() {
    register("help", help_cmd);
    register("clear", clear_cmd);
}
