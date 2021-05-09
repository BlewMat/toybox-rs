use toybox_core::graphics::{Color, Drawable};
use toybox_core::{AleAction, Direction, Input, QueryError};

use serde_json;
use std::collections::HashMap;
use std::convert::TryInto;
use std::time::Duration;
use std::thread::sleep;
use rand::thread_rng;
use rand::Rng;

use crate::types::{Othello, State, Player, FrameState, TileConfig};

impl TileConfig {
    fn floor() -> TileConfig {
        TileConfig {
            reward: 0,
            walkable: true,
            color: Color::rgb(0, 255, 0),
        }
    }
    fn player1() -> TileConfig {
        TileConfig {
            reward: 1,
            walkable: true,
            color: Color::black(),
        }
    }
    fn player2() -> TileConfig {
        TileConfig {
            reward: 1,
            walkable: true,
            color: Color::white(),
        }
    }
    fn border() -> TileConfig {
        TileConfig {
            reward: 0,
            walkable: false,
            color: Color::rgb(211, 211, 211),
        }
    }
    fn dark_floor() -> TileConfig {
        TileConfig {
            reward: 0,
            walkable: true,
            color: Color::rgb(5, 195, 25),
        }
    }
    fn color_demo() -> TileConfig {
        TileConfig {
            reward: 0,
            walkable: true,
            color: Color::rgb(255, 0, 0),
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
        tiles.insert('4', TileConfig::dark_floor());
        tiles.insert('5', TileConfig::color_demo());

        let mut board = vec![0; 64];
        let mut board_array: [i32; 64] = [0; 64];
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
        for item in &board_array {
            board[count] = *item;
            count += 1;
        }

        let grid = vec![
            "3333333333".to_owned(),
            "3040404043".to_owned(),
            "3404040403".to_owned(),
            "3040404043".to_owned(),
            "3404120403".to_owned(),
            "3040214043".to_owned(),
            "3404040403".to_owned(),
            "3040404043".to_owned(),
            "3404040403".to_owned(),
            "3333333333".to_owned(),
        ];

        let q_table = HashMap::new();

        Othello {
            player_color: Color::rgb(255, 0, 0),
            board,
            grid,
            tiles,
            turn: Player::Black,
            player1_becomes: '1',
            player2_becomes: '2',
            //THIS GOES COL, ROW
            player_start: (1, 1),
            diagonal_support: false,
            q_table,
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

        FrameState {
            game_over: false,
            step: 0,
            score: 0,
            player2_becomes: char_to_index[&config.player2_becomes],
            player1_becomes: char_to_index[&config.player1_becomes],
            board,
            tiles,
            grid,
            turn: config.turn,
            player: config.player_start,
            q_table: config.q_table.clone(),
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

    fn terminal(&mut self) -> Vec<i32> {
        let mut count = 0;
        let mut row;
        let mut col;

        let mut possible_moves = Vec::new();

        while count < 64 {
            row = &count / 8;
            col = &count - (row * 8);

            if self.check_move(col + 1, row + 1) {
                possible_moves.push(col + 1);
                possible_moves.push(row + 1);
            }
            count += 1;
        }
        possible_moves
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
    }

    fn print_board(&self, board: [i32; 64]) {
        let mut count = 0;
        let mut row = 0;

        println!("    0 1 2 3 4 5 6 7");
        println!("   ----------------");

        for x in &board {
            if &count % 7 == 0 && &count > &0 {
                println!("{} ", x);
            } else {
                if &count == &0 {
                    print!("{} | ", row)
                }
                print!("{} ", x);
            }

            if &count == &7 {
                count = 0;
                row += 1;
            } else {
                count += 1;
            }
        }
    }


    fn check_move(&mut self, x: i32, y: i32) -> bool {
        let mut valid: bool = false;

        let index: usize = (((y - 1) * 8) + (x - 1)).try_into().unwrap();

        let token;
        let oppo_token;

        if self.turn == Player::Black {
            token = 1;
            oppo_token = 2;
        } else {
            token = 2;
            oppo_token = 1;
        }

        let mut board_array: [i32; 64] = [0; 64];
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

                if &index >= tile {
                    if board_array[index - tile] != 0 && board_array[index - tile] != token {
                        if (*tile == 1 as usize || *tile == 9 as usize) && index % 8 == 0 {
                            continue;
                        } else {
                            legal = true;
                        }
                    }
                }

                if &index + tile < 64 {
                    if board_array[index + tile] != 0 && board_array[index + tile] != token {
                        if (*tile == 1 as usize || *tile == 9 as usize) && index % 8 == 7 {
                            continue;
                        } else {
                            legal = true;
                            pos = true;
                        }
                    }
                }

                if legal {
                    let mut check;

                    if pos {
                        if index + tile < 64 {
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

                    while check < board_array.len() - 1 && board_array[check] == oppo_token {

                        if check % 8 == 7 || check % 8 == 0 {
                            if board_array[check] == token {
                                valid = true;
                            }
                            if *tile != 8 as usize{
                                break;
                            }
                            // Originally it was just break, no if statement
                            //if board_array[check] == 0 {
                            //    break;
                            //}
                            // now should keep going if tile == 8
                        }

                        if pos {
                            if check + tile < 64 {
                                check += tile;
                            } else {
                                break;
                            }
                        } else {
                            if check >= *tile {
                                check -= tile;
                            } else {
                                break;
                            }
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
        for item in &board_array {
            self.board[count] = *item;
            count += 1;
        }

        valid
    }

    fn flip_tiles(&mut self, x: i32, y: i32, print: bool) -> i32 {
        //let mut token = 0;
        let token;

        if self.turn == Player::Black {
            token = 1;
        } else {
            token = 2;
        }

        if self.turn == Player::Black {
            self.grid[y as usize][x as usize] = self.player1_becomes;
        } else {
            self.grid[y as usize][x as usize] = self.player2_becomes;
        }

        let index: usize = (((y - 1) * 8) + (x - 1)).try_into().unwrap();
        let mut reward;
        if index == 0 as usize || index == 7 as usize || index == 56 as usize ||
            index == 63 as usize {
            reward = 20;
        } else if index < 8 || index > 55 || index % 8 == 0 || index % 8 == 7 {
            reward = 5;
        } else if (index > 9 && index < 14) || (index > 49 && index < 54) || (index % 8 == 1 && index > 9 && index < 49) || (index % 8 == 6 && index > 14 && index < 54){
            reward = 0;
        } else {
            reward = 1;
        }

        let mut board_array: [i32; 64] = [0; 64];
        let mut count = 0;
        for x in &self.board {
            board_array[count] = *x;
            count += 1;
        }

        board_array[index] = token;

        let adjacent: [usize; 4] = [1, 7, 8, 9];
        for tile in &adjacent {
            let mut check_pos;
            let mut check_neg;

            let mut pos_valid = true;
            let mut neg_valid = true;

            let mut pos_tiles: [usize; 8] = [0; 8];
            let mut neg_tiles: [usize; 8] = [0; 8];

            check_pos = index + tile;
            check_neg = index - tile;

            let mut count = 0;

            while check_pos < (board_array.len() - 1) {

                if board_array[check_pos] == 0 || board_array[check_pos] == token {
                    if board_array[check_pos] == token {
                        break;
                    } else {
                        pos_valid = false;
                        break;
                    }
                } else {
                    pos_tiles[count] = check_pos.try_into().unwrap();
                }

                if check_pos % 8 == 7 || check_pos % 8 == 0 {
                    if *tile != 8 as usize{
                        if board_array[check_pos] != token {
                            pos_valid = false;
                        }
                        break;
                    } else {
                        if (check_pos == 0 || check_pos == 7 || check_pos == 56 || check_pos == 63) && board_array[check_pos] != token {
                            pos_valid = false;
                            break;
                        }
                    }
                }

                if check_pos + tile < 64 {
                    check_pos += tile;
                } else {
                    if board_array[check_pos] == token {
                        break;
                    } else {
                        pos_valid = false;
                        break;
                    }
                }

                count += 1;
            }

            count = 0;
            while check_neg < board_array.len() - 1 {

                if board_array[check_neg] == 0 || board_array[check_neg] == token {
                    if board_array[check_neg] == token {
                        break;
                    } else {
                        neg_valid = false;
                        break;
                    }
                } else {
                    neg_tiles[count] = check_neg.try_into().unwrap();
                }

                if check_neg % 8 == 7 || check_neg % 8 == 0 {
                    if *tile != 8 as usize{
                        if board_array[check_neg] != token {
                            neg_valid = false;
                        }
                        break;
                    } else {
                        if (check_neg == 0 || check_neg == 7 || check_neg == 56 || check_neg == 63) && board_array[check_neg] != token {
                            neg_valid = false;
                            break;
                        }
                    }
                }

                if check_neg >= *tile {
                    check_neg -= tile;
                } else {
                    if board_array[check_neg] == token {
                        break;
                    } else {
                        neg_valid = false;
                        break;
                    }
                }

                count += 1;
            }

            let mut x;
            let mut y;
            if neg_valid {
                for item in neg_tiles.iter() {
                    if item != &0 {
                        board_array[*item] = token;
                        reward += 1;

                        x = item / 8;
                        y = item - (x * 8);
                        x += 1;
                        y += 1;
                        if self.turn == Player::Black {
                            self.grid[x][y] = self.player1_becomes;
                        } else {
                            self.grid[x][y] = self.player2_becomes;
                        }
                    }
                }
            }
            if pos_valid {
                for item in pos_tiles.iter() {
                    if item != &0 {
                        board_array[*item] = token;
                        reward += 1;

                        x = item / 8;
                        y = item - (x * 8);
                        x += 1;
                        y += 1;
                        if self.turn == Player::Black {
                            self.grid[x][y] = self.player1_becomes;
                        } else {
                            self.grid[x][y] = self.player2_becomes;
                        }
                    }
                }
            }
        }

        let mut count = 0;
        for item in &board_array {
            self.board[count] = *item;
            count += 1;
        }

        if index == 1 || index == 8 as usize || index == 9 as usize ||
            index == 6 as usize || index == 14 as usize || index == 15 as usize ||
            index == 48 as usize || index == 49 as usize || index == 57 as usize ||
            index == 54 as usize || index == 55 as usize || index == 62 as usize {
            reward = 0;
        }

        if print {
            self.print_board(board_array);
            println!("reward: {}", reward);
        }
        reward
    }

    fn change_turn(&mut self) {
        let player1 = self.terminal();

        // Change whose turn it is
        if self.turn == Player::Black {
            self.turn = Player::White;
        } else {
            self.turn = Player::Black;
        }

        let player2 = self.terminal();

        // if both player 1 and player 2 can't move, end game
        if player1.len() == 0 && player2.len() == 0 {
            self.game_over = true;
            println!("Game over!");

            let mut black = 0;
            let mut white = 0;

            for tile in &self.board {
                if tile == &1 {
                    black += 1;
                } else if tile == &2 {
                    white += 1;
                }
            }

            println!("");
            if black > white {
                println!("Player 1 wins!");
                println!("Player 1 tokens = {}", black);
                println!("Player 2 tokens = {}", white);
            } else if black < white {
                println!("Player 2 wins!");
                println!("Player 2 tokens = {}", white);
                println!("Player 1 tokens = {}", black);
            } else {
                println!("It's a tie!");
                println!("Player 1 tokens = {}", black);
                println!("Player 2 tokens = {}", white);
            }

        }

        // if just player2 can't move, change turn
        if player2.len() == 0 {
            if self.turn == Player::Black {
                self.turn = Player::White;
            } else {
                self.turn = Player::Black;
            }
        }
    }


    fn get_greedy_move(&mut self, max: bool) -> (i32, i32) {

        // Epsilon close to 0 follows Q value more, and random value less
        let epsilon = 0.2;

        let moves = self.terminal();

        let mut rng = thread_rng();
        let random_num = rng.gen_range(0.0, 1.0);

        let mut x;
        let mut y;

        if random_num < epsilon && !max {
            //println!("Random Action!");
            //Select random action
            let mut rng = thread_rng();
            let random_move = rng.gen_range(0, moves.len());

            if random_move % 2 == 0 {
                x = moves[random_move];
                y = moves[random_move + 1];
            } else {
                x = moves[random_move - 1];
                y = moves[random_move];
            }

            //println!("x: {}", x);
            //println!("y: {}", y);
        } else {
            //Select greedy action
            let mut index = 0;
            let mut max_reward = 0.0;

            x = moves[0];
            y = moves[1];

            // checks if q_table has the state, if not, it adds the state
            while index < moves.len() {
                let result = self.q_table.entry((self.board.clone(), (moves[index], moves[index + 1]))).or_insert(1.0);

                if *result > max_reward {
                    max_reward = *result;
                    x = moves[index];
                    y = moves[index + 1];
                }
                index += 2;
            }
        }
        (x, y)
    }


    fn update_qtable(&mut self, x: i32, y: i32){

        // Alpha close to 0 makes Q-values change slower
        let alpha = 0.2;
        // Gamma close to 1 looks for high rewards in the long term
        let gamma = 0.8;

        let mut original_state;
        let mut reward;
        let mut moves;
        let mut x1 = x;
        let mut y1 = y;
        let mut length;

        while self.terminal().len() != 0 {

            original_state = self.board.clone();

            if self.check_move(x1, y1) {

                reward = self.flip_tiles(x1, y1, false);

                self.turn = Player::Black;

                length = self.terminal().len();
                if length != 0 {
                    // THIS IS THE OPPONENTS MOVE
                    moves = self.get_greedy_move(true);
                    self.flip_tiles(moves.0, moves.1, false);

                    self.turn = Player::White;
                    while self.terminal().len() == 0 {
                        self.turn = Player::Black;
                        if self.terminal().len() == 0 {
                            return;
                        }
                        moves = self.get_greedy_move(true);
                        self.flip_tiles(moves.0, moves.1, false);
                        self.turn = Player::White;
                    }
                }

                self.turn = Player::White;
                if self.terminal().len() == 0 && length == 0 {
                    return;
                } else if self.terminal().len() == 0{
                    while self.terminal().len() == 0 {
                        self.turn = Player::Black;
                        if self.terminal().len() == 0 {
                            return;
                        }
                        moves = self.get_greedy_move(true);
                        self.flip_tiles(moves.0, moves.1, false);
                        self.turn = Player::White;
                    }
                }
                // Getting the max q_table val for new state
                moves = self.get_greedy_move(true);

                let new_q = self.q_table.entry((self.board.clone(), (moves.0, moves.1))).or_insert(1.0);
                let next_move = *new_q;

                let original_q = self.q_table.entry((original_state.clone(), (x1, y1))).or_insert(1.0);
                //println!("{}", original_q);
                *original_q += alpha * (reward as f32 + (gamma * next_move) - *original_q);
                //println!("{}", self.q_table[&(original_state, (x1, y1))]);


                moves = self.get_greedy_move(false);
                x1 = moves.0;
                y1 = moves.1;


            } else {
                println!("Invalid move by Greedy Agent");
                return;
            }

        }
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

        self.frame.step += 1;

        // Greedy agent making a move
        if self.frame.turn == Player::White {

            sleep(Duration::from_millis(1000));

            let poss_moves = self.frame.terminal();

            let original_board = self.frame.board.clone();
            let original_grid = self.frame.grid.clone();

            if poss_moves.len() == 0 {
                self.frame.change_turn();
                return;
            }
            let moves = self.frame.get_greedy_move(false);
            let x = moves.0;
            let y = moves.1;

            self.frame.update_qtable(x, y);

            self.frame.board = original_board.clone();
            self.frame.grid = original_grid.clone();
            self.frame.turn = Player::White;

            if self.frame.check_move(x, y) {

                self.frame.flip_tiles(x, y, true);
                println!("possible moves: {}", poss_moves.len()/2);

                self.frame.change_turn();

            }

            //for (entry, result) in &self.frame.q_table {
            //    println!("{:?} has {}", entry, result);
            //}

        }

        // This is pressing the spacebar, this should let you select
        // where you want to put your new token
        if buttons.button1 {

            let moves = self.frame.terminal();

            // Check if you are able to place a token here
            let (x, y) = self.frame.player;

            if self.frame.check_move(x, y) {

                // Move is valid, now flip tiles
                let reward = self.frame.flip_tiles(x, y, true);
                println!("possible moves: {}", moves.len()/2);
                self.frame.score += reward;

                self.frame.change_turn();
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

