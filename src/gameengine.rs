use super::coordinate::{Coordinate,PossibleMoves, Positions};
use super::gamepiece::{GamePiece, Colors};
use super::gamepiece::Colors::{Black,White};

type BoardStatus = Vec<GamePiece>;

#[derive(Debug)]
struct GameEngine {
  move_count: usize,
  current_turn: Colors,
  board_status: BoardStatus,
}


impl GameEngine {
  pub fn new() -> GameEngine {
    GameEngine {
      move_count: 0,
      current_turn: Black,
      board_status: BoardStatus::new(),
    }
  }

  pub fn initialize_game_pieces(&mut self) {
      for i in 1..9 {
        let mut black_y_pos = 0;
        let mut white_y_pos = 0;

        if i%2 == 0 {
          black_y_pos = 2;
          white_y_pos = 8;
        } else {
          black_y_pos = 1;
          white_y_pos = 7;
        }

        let black_coordinate = Coordinate { x: i, y: black_y_pos };
        let gamepiece = GamePiece::new(Black, black_coordinate);
        self.board_status.push(gamepiece);

        let white_coordinate = Coordinate { x: i, y: white_y_pos };
        let gamepiece = GamePiece::new(White, white_coordinate);
        self.board_status.push(gamepiece);
      }
  }
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn it_should_create_new_engine_with_right_defaults() {
    let new_game_engine = GameEngine::new();
    assert_eq!(new_game_engine.current_turn, Black);
    assert_eq!(new_game_engine.move_count, 0);
    assert_eq!(new_game_engine.board_status, BoardStatus::new());
  }

 #[test]
  fn it_should_initialize_the_game_board() {
    let mut new_game_engine = GameEngine::new();
    new_game_engine.initialize_game_pieces();

    let expected: BoardStatus = vec![
      GamePiece { color: Black, crowned: false, position: Coordinate { x: 1, y: 1 } },
      GamePiece { color: White, crowned: false, position: Coordinate { x: 1, y: 7 } },
      GamePiece { color: Black, crowned: false, position: Coordinate { x: 2, y: 2 } },
      GamePiece { color: White, crowned: false, position: Coordinate { x: 2, y: 8 } },
      GamePiece { color: Black, crowned: false, position: Coordinate { x: 3, y: 1 } },
      GamePiece { color: White, crowned: false, position: Coordinate { x: 3, y: 7 } }, 
      GamePiece { color: Black, crowned: false, position: Coordinate { x: 4, y: 2 } },
      GamePiece { color: White, crowned: false, position: Coordinate { x: 4, y: 8 } },
      GamePiece { color: Black, crowned: false, position: Coordinate { x: 5, y: 1 } },
      GamePiece { color: White, crowned: false, position: Coordinate { x: 5, y: 7 } },
      GamePiece { color: Black, crowned: false, position: Coordinate { x: 6, y: 2 } }, 
      GamePiece { color: White, crowned: false, position: Coordinate { x: 6, y: 8 } }, 
      GamePiece { color: Black, crowned: false, position: Coordinate { x: 7, y: 1 } },
      GamePiece { color: White, crowned: false, position: Coordinate { x: 7, y: 7 } },
      GamePiece { color: Black, crowned: false, position: Coordinate { x: 8, y: 2 } },
      GamePiece { color: White, crowned: false, position: Coordinate { x: 8, y: 8 } }
    ];

    assert_eq!(new_game_engine.board_status, expected);
  }
}

