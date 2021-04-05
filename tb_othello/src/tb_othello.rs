use toybox_core;
use toybox_core::random;
use types::*;

use serde_json;
use rand::seq::SliceRandom;

use crate::types::{Othello, State, Player, FrameState};


impl Default for Othello {
    fn default() -> Self {
        let mut board = [0; 64];
        board[27] = 1;
        board[28] = 2;
        board[35] = 2;
        board[36] = 1;

        Othello {
            board,
            turn: Player::Black,
            reward_becomes: '0'
        }
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

        // NOT COMPLETE
    }


    /// Any state can create a vector of drawable objects to present itself.
    fn draw(&self) -> Vec<graphics::Drawable> {

        // NOT COMPLETE
    }


    /// Any state can serialize to JSON String.
    fn to_json(&self) -> String {
        serde_json::to_string(&self.state).expect("Should be no JSON Serialization Errors.")
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



impl FrameState {

    fn size(&self) -> (i32, i32) {
        let height = 8;
        let width = 8;
        (width, height)
    }

    fn from_config(config: &Othello) -> FrameState {


        FrameState {
            score: 0,
            game_over: false,
            reward_becomes: char_to_index[&config.reward_becomes],
            board,
            turn: Player::Black
        }
    }

    // NOT COMPLETE

}



impl toybox_core::Simulation for Othello {
    /// Seed simulation.
    fn reset_seed(&mut self, seed: u32) {
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
            AleAction::UPRIGHT,
            AleAction::UPLEFT,
            AleAction::DOWNRIGHT,
            AleAction::DOWNLEFT,
        ];
        actions.sort();
        actions
    }

    /// Return a tuple of game size in pixels, e.g., (100,100).
    fn game_size(&self) -> (i32, i32) {
        (8, 8)
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
