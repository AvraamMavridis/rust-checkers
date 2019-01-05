use super::coordinate::{Coordinate,PossibleMoves};
use super::gamepiece::{GamePiece, Colors};

#[derive(Debug)]
struct GameEngine {
  move_count: usize,
  current_turn: Colors,
}


impl GameEngine {
  pub fn new() -> GameEngine {
    GameEngine {
      move_count: 0,
      current_turn: Colors::Black,
    }
  }
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn it_should_create_new_engine_with_right_defaults() {
    let new_game_engine = GameEngine::new();
    assert_eq!(new_game_engine.current_turn, Colors::Black);
    assert_eq!(new_game_engine.move_count, 0);
  }
}

