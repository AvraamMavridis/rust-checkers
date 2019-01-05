use super::coordinate::{Coordinate};


#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Colors {
  Black,
  White
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct GamePiece {
  pub color: Colors,
  pub crowned: bool,
  pub position: Coordinate,
}

impl GamePiece {
  pub fn new(color: Colors, position: Coordinate) -> GamePiece {
    GamePiece {
      color,
      crowned: false,
      position,
    }
  }

  pub fn crowned(game_piece: GamePiece) -> GamePiece {
    GamePiece {
      position: game_piece.position,
      color: game_piece.color,
      crowned: true,
    }
  }

  pub fn move_piece(&mut self, new_position: Coordinate) {
    self.position = new_position;
  }
}



