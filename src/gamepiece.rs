#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Colors {
  Black,
  White
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct GamePiece {
  color: Colors,
  crowned: bool,
}

impl GamePiece {
  pub fn new(color: Colors) -> GamePiece {
    GamePiece {
      color,
      crowned: false
    }
  }

  pub fn crowned(game_piece: GamePiece) -> GamePiece {
    GamePiece {
      color: game_piece.color,
      crowned: true,
    }
  }
}



