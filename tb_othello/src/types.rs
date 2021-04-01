#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Othello {
    pub board: [i32; 64],
    pub turn: Player,
    pub reward_becomes: char
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FrameState {
    pub score: i32,
    pub game_over: bool,
    pub reward_becomes: usize,
    pub board: [i32; 64],
    pub turn: Player
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct State {
    pub config: Othello,
    pub frame: FrameState,
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Player {
    Black,
    White,
}
