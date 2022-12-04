#![warn(clippy::pedantic)]
#![allow(dead_code)]

mod devices;
mod executioner;
mod interceptor;
mod rule_library;
mod units;
mod utils;

fn main() {
    interceptor::start();
}
