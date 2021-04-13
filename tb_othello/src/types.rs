use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use toybox_core::graphics::Color;


#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct TileConfig {
    /// What reward (if any) is given or taken by passing this tile?
    pub reward: i32,
    /// Is this tile walkable by the agent?
    pub walkable: bool,
    /// Is this playable by an agent?
    pub playable: bool,
    /// Is this a terminal/goal tile?
    pub terminal: bool,
    /// What color should this tile be?
    pub color: Color,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Othello {
    pub player_color: Color,
    pub board: Vec<i32>,
    //pub board: [i32; 64],
    pub grid: Vec<String>,
    pub tiles: HashMap<char, TileConfig>,
    pub turn: Player,
    pub reward_becomes: char,
    pub player_start: (i32, i32),
    pub diagonal_support: bool,
}


#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct FrameState {
    pub game_over: bool,
    pub step: usize,
    pub score: i32,
    pub reward_becomes: usize,
    //pub board: [i32; 64],
    pub board: Vec<i32>,
    pub tiles: Vec<TileConfig>,
    pub grid: Vec<Vec<usize>>,
    pub turn: Player,
    pub player: (i32, i32)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct State {
    pub config: Othello,
    pub frame: FrameState,
}

#[derive(Debug, PartialEq, Copy, Clone, Serialize, Deserialize, JsonSchema)]
pub enum Player {
    Black,
    White,
}
