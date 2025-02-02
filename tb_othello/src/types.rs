use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use toybox_core::graphics::Color;


#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct TileConfig {
    pub reward: i32,
    pub walkable: bool,
    pub color: Color,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Othello {
    pub player_color: Color,
    pub board: Vec<i32>,
    pub grid: Vec<String>,
    pub tiles: HashMap<char, TileConfig>,
    pub turn: Player,
    pub player1_becomes: char,
    pub player2_becomes: char,
    pub player_start: (i32, i32),
    pub diagonal_support: bool,
    pub q_table: HashMap<(Vec<i32>, (i32, i32)), f32>,
}


#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct FrameState {
    pub game_over: bool,
    pub step: usize,
    pub score: i32,
    pub player1_becomes: usize,
    pub player2_becomes: usize,
    pub board: Vec<i32>,
    pub tiles: Vec<TileConfig>,
    pub grid: Vec<Vec<usize>>,
    pub turn: Player,
    pub player: (i32, i32),
    pub q_table: HashMap<(Vec<i32>, (i32, i32)), f32>,
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
