#![deny(unused_variables)]
#![deny(unused_imports)]
#![deny(unused_mut)]
#![deny(dead_code)]
#![warn(clippy::print_stdout)]
#![warn(clippy::print_stderr)]
#![warn(clippy::dbg_macro)]
#![warn(clippy::todo)]
#![warn(clippy::unimplemented)]

pub mod app_state;
pub mod order;
pub mod pricing;
pub mod promo;
pub mod routes;
pub mod store;
pub mod surge;
pub mod utils;
pub mod validators;
