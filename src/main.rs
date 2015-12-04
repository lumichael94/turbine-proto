pub mod data;
pub mod network;
pub mod util;
pub mod vm;
pub mod engine;

extern crate postgres;
extern crate rustc_serialize;
extern crate bincode;
extern crate rand;
extern crate crypto;
extern crate chrono;
extern crate regex;

use engine::turbo;

pub fn main() {
    turbo::init();
    turbo::command_loop();
    turbo::end();
}
