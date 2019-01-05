use super::coordinate::{Coordinate,PossibleMoves, Positions};
use super::gamepiece::{GamePiece, Colors};
use super::gamepiece::Colors::{Black,White};
use std::num;

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

  pub fn move_game_piece(&mut self, from: Coordinate, to: Coordinate) {
    let piece = self.board_status.iter().find(|piece| piece.position == from);
    let to_piece = self.board_status.iter().find(|piece| piece.position == to);

    let mut piece = match piece {
        Some(p) => p,
        None => {
          panic!("Piece not found")
        },
    };

    if ((from.x as i32 - to.x as i32)).abs() > 1 && ((from.y as i32 - to.y as i32)).abs() > 1 {
      panic!("Piece can be moved only by 1 square at a time");
    }

    match to_piece {
        Some(_) => {
          panic!("Cannot be moved because there is another gamepiece in the position");
        },
        None => (),
    };

    piece.position = to;
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

  #[test]
  #[should_panic(expected = "Piece not found")]
  fn it_should_panic_when_try_to_move_a_piece_from_invalid_position() {
    let mut new_game_engine = GameEngine::new();
    new_game_engine.initialize_game_pieces();

    let from = Coordinate { x: 11 , y: 1 };
    let to = Coordinate { x: 5, y: 5 };
    
    new_game_engine.move_game_piece(from, to);
  }

  #[test]
  #[should_panic(expected = "Piece can be moved only by 1 square at a time")]
  fn it_should_panic_when_try_to_move_more_than_one() {
    let mut new_game_engine = GameEngine::new();
    new_game_engine.initialize_game_pieces();

    let from = Coordinate { x: 1 , y: 1 };
    let to = Coordinate { x: 5, y: 5 };
    
    new_game_engine.move_game_piece(from, to);
  }

  #[test]
  #[should_panic(expected = "Cannot be moved because there is another gamepiece in the position")]
  fn it_should_panic_when_try_to_move_on_other_piece() {
    let mut new_game_engine = GameEngine::new();
    new_game_engine.initialize_game_pieces();

    let from = Coordinate { x: 1 , y: 1 };
    let to = Coordinate { x: 2, y: 2 };
    
    new_game_engine.move_game_piece(from, to);
  }

  #[test]
  fn it_should_move_piece() {
    let mut new_game_engine = GameEngine::new();
    new_game_engine.initialize_game_pieces();

    let expected: BoardStatus = vec![
      GamePiece { color: Black, crowned: false, position: Coordinate { x: 1, y: 1 } },
      GamePiece { color: White, crowned: false, position: Coordinate { x: 1, y: 7 } },
      GamePiece { color: Black, crowned: false, position: Coordinate { x: 3, y: 3 } },
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

    let from = Coordinate { x: 2 , y: 2 };
    let to = Coordinate { x: 3, y: 3 };
    new_game_engine.move_game_piece(from, to);

    assert_eq!(new_game_engine.board_status, expected);
  }
}

