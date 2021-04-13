use toybox_core::graphics::{Color, Drawable};
use toybox_core::{AleAction, Direction, Input, QueryError};

use serde_json;
use std::collections::HashMap;
use std::convert::TryInto;

use crate::types::{Othello, State, Player, FrameState, TileConfig};

impl TileConfig {
    fn floor() -> TileConfig {
        TileConfig {
            reward: 0,
            walkable: true,
            playable: true,
            terminal: false,
            color: Color::rgb(0, 255, 0),
        }
    }
    fn player1() -> TileConfig {
        TileConfig {
            reward: 1,
            walkable: true,
            playable: false,
            terminal: false,
            color: Color::black(),
        }
    }
    fn player2() -> TileConfig {
        TileConfig {
            reward: 1,
            walkable: true,
            playable: false,
            terminal: false,
            color: Color::white(),
        }
    }
    fn border() -> TileConfig {
        TileConfig {
            reward: 0,
            walkable: false,
            playable: false,
            terminal: false,
            color: Color::rgb(211, 211, 211),
        }
    }
}

impl Default for Othello {
    fn default() -> Self {

        let mut tiles = HashMap::new();
        tiles.insert('0', TileConfig::floor());
        tiles.insert('1', TileConfig::player1());
        tiles.insert('2', TileConfig::player2());
        tiles.insert('3', TileConfig::border());

        let mut board = vec![0; 64];
        let mut board_array : [i32; 64] = [0; 64];
        let mut count = 0;
        for x in &board {
            board_array[count] = *x;
            count += 1;
        }
        board_array[27] = 1;
        board_array[28] = 2;
        board_array[35] = 2;
        board_array[36] = 1;
        let mut count = 0;
        for item in &board_array{
            board[count] = *item;
            count += 1;
        }


        let grid = vec![
            "3333333333".to_owned(),
            "3000000003".to_owned(),
            "3000000003".to_owned(),
            "3000000003".to_owned(),
            "3000210003".to_owned(),
            "3000120003".to_owned(),
            "3000000003".to_owned(),
            "3000000003".to_owned(),
            "3000000003".to_owned(),
            "3333333333".to_owned(),
        ];

        Othello {
            player_color: Color::rgb(255, 0, 0),
            board,
            grid,
            tiles,
            turn: Player::Black,
            reward_becomes: '0',
            //THIS GOES COL, ROW
            player_start: (4,4),
            diagonal_support: false,
        }
    }
}




impl FrameState {

    fn size(&self) -> (i32, i32) {
        let height = self.grid.len() as i32;
        let width = self.grid[0].len() as i32;
        (width, height)
    }

    fn from_config(config: &Othello) -> FrameState {

        let mut tiles = Vec::new();
        let mut grid = Vec::new();

        let mut char_to_index = HashMap::new();
        for (ch, desc) in &config.tiles {
            let id = tiles.len();
            char_to_index.insert(ch, id);
            tiles.push(desc.clone());
        }
        for row in &config.grid {
            let mut grid_row = Vec::new();
            for ch in row.chars() {
                let tile_id = char_to_index[&ch];
                grid_row.push(tile_id);
            }
            grid.push(grid_row);
        }

        let mut board = Vec::new();
        for item in &config.board {
            board.push(*item);
        }

        // Need to initialize board, turn

        FrameState {
            game_over: false,
            step: 0,
            score: 0,
            reward_becomes: char_to_index[&config.reward_becomes],
            board,
            tiles,
            grid,
            turn: config.turn,
            player: config.player_start
        }
    }

    //Returns the type of a tile in the grid at a specific index (e.g (0,0) = 3)
    fn get_tile(&self, tx: i32, ty: i32) -> Option<&TileConfig> {
        let (w, h) = self.size();
        if tx < 0 || ty < 0 || tx >= w || ty >= h {
            return None;
        }
        let y = ty as usize;
        let x = tx as usize;
        let tile_id = self.grid[y][x];
        Some(&self.tiles[tile_id])
    }

    fn walkable(&self, tx: i32, ty: i32) -> bool {
        self.get_tile(tx, ty).map(|t| t.walkable).unwrap_or(false)
    }

    fn terminal(&mut self) -> bool{

        let mut possible = false;

        let mut count = 0;
        //let mut row = 0;
        //let mut col = 0;
        let mut row;
        let mut col;

        while count < 64 {
            row = &count / 8;
            col = &count - (row * 8);

            if self.check_move(col+1, row+1) {
                possible = true;
                break;
            }
            count += 1;
        }

        if possible{
            false
        } else {
            true
        }
    }

    fn walk_once(&mut self, dx: i32, dy: i32) {
        let (px, py) = self.player;
        let dest = (px + dx, py + dy);
        if self.walkable(dest.0, dest.1) {
            self.arrive(dest.0, dest.1)
        }
    }

    fn arrive(&mut self, x: i32, y: i32) {
        self.player = (x, y);

        // check terminal before "collect_reward" which removes the reward from the map.
        //if self.terminal(x, y) {
        //    self.game_over = true;
        //}

        // NEED TO CALL A REWARD FUNCTION AFTER A TOKEN IS PLACED, AND TOKENS ARE FLIPPED
        //self.collect_reward(x, y);
    }


    fn check_move(&mut self, x: i32, y: i32) -> bool {

        let mut valid: bool = false;
        let index: usize = (((y-1) * 8) + (x-1)).try_into().unwrap();

        //let mut token = 0;
        let token;
        //let mut oppo_token = 0;
        let oppo_token;

        if self.turn == Player::Black {
            token = 1;
            oppo_token = 2;
        } else {
            token = 2;
            oppo_token = 1;
        }

        let mut board_array : [i32; 64] = [0; 64];
        let mut count = 0;
        for x in &self.board {
            board_array[count] = *x;
            count += 1;
        }

        if board_array[index] == 0 {
            let mut legal: bool = false;
            let adjacent: [usize; 4] = [1, 7, 8, 9];
            for tile in &adjacent {
                let mut pos: bool = false;

                //if &index >= tile && &index - tile >= 0 {
                if &index >= tile{
                    if board_array[index - tile] != 0 && board_array[index - tile] != token {
                        legal = true;
                    }
                }

                if &index + tile < 64 {
                    if board_array[index + tile] != 0 && board_array[index + tile] != token {
                        legal = true;
                        pos = true;
                    }
                }

                if legal {

                    //let mut check = 0;
                    let mut check;

                    if pos {
                        if index + tile < 64{
                        check = index + tile;
                        } else {
                            continue;
                        }
                    } else {
                        if index > *tile {
                            check = index - tile;
                        } else {
                            continue;
                        }
                    }


                    //while 0 <= check && check < board_array.len() - 1 && board_array[check] == oppo_token {
                    while check < board_array.len() - 1 && board_array[check] == oppo_token {


                        if pos {
                            check += tile;
                        } else {
                            if check >= *tile {
                                check -= tile;
                            } else {
                                break;
                            }
                        }

                        if check % 8 == 7 || check % 8 == 0 {
                            if board_array[check] == token {
                                valid = true;
                            }
                            break;
                        }

                        if board_array[check] == token {
                            valid = true;
                            break;
                        }
                    }
                }
            }
        }

        let mut count = 0;
        for item in &board_array{
            self.board[count] = *item;
            count += 1;
        }
        valid
    }

    fn flip_tiles(&mut self) -> i32 {
        //let mut token = 0;
        let token;

        if self.turn == Player::Black {
            token = 1;
        } else {
            token = 2;
        }

        let (x, y) = self.player;
        let index: usize = (((y-1) * 8) + (x-1)).try_into().unwrap();

        let mut board_array : [i32; 64] = [0; 64];
        let mut count = 0;
        for x in &self.board {
            board_array[count] = *x;
            count += 1;
        }

        board_array[index] = token;
        let mut reward = 1;

        let adjacent: [usize; 4] = [1, 7, 8, 9];
        for tile in &adjacent {
            //let mut check_pos = 0;
            //let mut check_neg = 0;
            let mut check_pos;
            let mut check_neg;

            let mut pos_valid = true;
            let mut neg_valid = true;

            let mut pos_tiles: [usize; 8] = [0; 8];
            let mut neg_tiles: [usize; 8] = [0; 8];

            check_pos = index + tile;
            check_neg = index - tile;

            let mut count = 0;

            //while 0 <= check_pos && check_pos < (board_array.len() - 1) {
            while check_pos < (board_array.len() - 1) {
                pos_tiles[count] = check_pos.try_into().unwrap();
                //pos_tiles[count] = check_pos;
                if check_pos + tile < 64 {
                    check_pos += tile;
                }

                if check_pos % 8 == 7 || check_pos % 8 == 0 {
                    if board_array[check_pos] != token {
                        pos_valid = false;
                    }
                    break;
                }

                if board_array[check_pos] == token {
                    break;
                }

                if board_array[check_pos] == 0 {
                    pos_valid = false;
                    break;
                }

                count += 1;
            }

            count = 0;
            //while 0 <= check_neg && check_neg < board_array.len() - 1 {
            while check_neg < board_array.len() - 1 {
                neg_tiles[count] = check_neg.try_into().unwrap();
                //neg_tiles[count] = check_neg;
                //if check_neg - tile >= 0 {
                check_neg -= tile;
                //}

                if check_neg % 8 == 7 || check_neg % 8 == 0 {
                    if board_array[check_neg] != token {
                        neg_valid = false;
                    }
                    break;
                }

                if board_array[check_neg] == token {
                    break;
                }

                if board_array[check_neg] == 0 {
                    neg_valid = false;
                    break;
                }

                count += 1;
            }

            if neg_valid {
                for item in neg_tiles.iter() {
                    if item != &0 {
                        board_array[*item] = token;
                        reward += 1;
                    }
                }
            }
            if pos_valid {
                for item in pos_tiles.iter() {
                    if item != &0 {
                        board_array[*item] = token;
                        reward += 1;
                    }
                }
            }
        }

        let mut count = 0;
        for item in &board_array{
            self.board[count] = *item;
            count += 1;
        }
        reward
    }
}


impl toybox_core::Simulation for Othello {
    /// Seed simulation.
    fn reset_seed(&mut self, _seed: u32) {
        //No randomness
    }

    /// Generate a new State. This is in a Box<State> because it may be 1 of many unknown types as far as calling code is concerned.
    fn new_game(&mut self) -> Box<dyn toybox_core::State> {
        Box::new(State {
            frame: FrameState::from_config(&self),
            config: self.clone(),
        })
    }

    /// Legal action set:
    fn legal_action_set(&self) -> Vec<AleAction> {
        let mut actions = vec![
            AleAction::NOOP,
            AleAction::FIRE,
            AleAction::UP,
            AleAction::LEFT,
            AleAction::RIGHT,
            AleAction::DOWN,
            //AleAction::UPRIGHT,
            //AleAction::UPLEFT,
            //AleAction::DOWNRIGHT,
            //AleAction::DOWNLEFT,
        ];
        actions.sort();
        actions
    }

    /// Return a tuple of game size in pixels, e.g., (100,100).
    fn game_size(&self) -> (i32, i32) {
        let height = self.grid.len() as i32;
        let width = self.grid[0].len() as i32;
        (width, height)
    }

    fn new_state_from_json(
        &self,
        json_str: &str,
    ) -> Result<Box<dyn toybox_core::State>, serde_json::Error> {
        let state: State = serde_json::from_str(json_str)?;
        Ok(Box::new(state))
    }

    fn from_json(
        &self,
        json_str: &str,
    ) -> Result<Box<dyn toybox_core::Simulation>, serde_json::Error> {
        let config: Othello = serde_json::from_str(json_str)?;
        Ok(Box::new(config))
    }


    fn to_json(&self) -> String {
        serde_json::to_string(self).expect("Othello should be JSON-serializable!")
    }


    /// Getter for JSON Schema for this game's state.
    fn schema_for_state(&self) -> String {
        let schema = schema_for!(FrameState);
        serde_json::to_string(&schema).expect("JSONSchema should be flawless.")
    }

    /// Getter for JSON Schema for this game's config.
    fn schema_for_config(&self) -> String {
        panic!("TODO: Othello characters as keys.")
    }

}


impl toybox_core::State for State {
    fn lives(&self) -> i32 {
        if self.frame.game_over {
            0
        } else {
            1
        }
    }

    /// Get the score from the game, i32 allows for negative scores.
    fn score(&self) -> i32 {
        self.frame.score
    }

    /// Get the level from the game.
    fn level(&self) -> i32 {
        0
    }

    /// To update internally to the next state, we pass buttons to internal logic.
    fn update_mut(&mut self, buttons: Input) {

        if buttons.is_empty() {

            return;
        }

        // Updates frame (by calling draw?)
        self.frame.step += 1;

        // This is pressing the spacebar, this should let you select
        // where you want to put your new token
        if buttons.button1 {

            println!("Move");

            // Check if you are able to place a token here
            let (x, y) = self.frame.player;

            if self.frame.check_move(x, y) {

                // Move is valid, now flip tiles
                self.frame.flip_tiles();

                // NEED TO CALL A REWARD FUNCTION AFTER A TOKEN IS PLACED, AND TOKENS ARE FLIPPED
                //self.collect_reward(x, y);

                let player1 = self.frame.terminal();

                // Change whose turn it is
                if self.frame.turn == Player::Black {
                    self.frame.turn = Player::White;
                } else {
                    self.frame.turn = Player::Black;
                }

                let player2 = self.frame.terminal();

                // if both player 1 and player 2 can't move, end game
                if player1 && player2{
                    self.frame.game_over = true;
                }

                // if just player2 can't move, change turn
                if player2 {
                    if self.frame.turn == Player::Black {
                        self.frame.turn = Player::White;
                    } else {
                        self.frame.turn = Player::Black;
                    }
                }

            } else {
                return;
            }

        }

        if let Some(dir) = Direction::from_input(buttons) {
                let (dx, dy) = dir.delta();
                self.frame.walk_once(dx, dy);
            }

    }


    /// Any state can create a vector of drawable objects to present itself.
    fn draw(&self) -> Vec<Drawable> {

        let mut output = Vec::new();
        output.push(Drawable::Clear(Color::black()));

        let (width, height) = self.frame.size();
        for y in 0..height {
            for x in 0..width {
                let tile = self.frame.get_tile(x, y).expect("Tile type should exist!");

                // THIS IS PROBABLY HOW YOU INCREASE THE TILE SIZE (originally, 1, 1)
                output.push(Drawable::rect(tile.color, x as i32, y as i32, 1, 1));
            }
        }
        output.push(Drawable::rect(
            self.config.player_color,
            self.frame.player.0,
            self.frame.player.1,
            1,
            1,
        ));

        output
    }


    /// Any state can serialize to JSON String.
    fn to_json(&self) -> String {
        serde_json::to_string(self).expect("Should be no JSON Serialization Errors.")
    }


    /// Copy this state to save it for later.
    fn copy(&self) -> Box<dyn toybox_core::State> {
        Box::new(self.clone())
    }


    /// Submit a query to this state object, returning a JSON String or error message.
    fn query_json(&self, query: &str, _args: &serde_json::Value) -> Result<String, QueryError> {
        Ok(match query {
            "xy" => {
                let (px, py) = self.frame.player;
                serde_json::to_string(&(px, py))?
            }
            "xyt" => {
                let (px, py) = self.frame.player;
                serde_json::to_string(&(px, py, self.frame.step))?
            }
            _ => Err(QueryError::NoSuchQuery)?,
        })
    }
}

