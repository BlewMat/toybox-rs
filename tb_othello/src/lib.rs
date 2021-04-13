extern crate serde;
extern crate serde_json;
extern crate toybox_core;
extern crate serde_derive;
extern crate lazy_static;
extern crate ordered_float;
extern crate rand;
#[macro_use]
extern crate schemars;

mod types;
mod othello;

pub use crate::types::{Othello, State, Player, FrameState, TileConfig};