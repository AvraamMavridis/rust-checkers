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
  fn new(color: Colors) -> GamePiece {
    GamePiece {
      color,
      crowned: false
    }
  }

  fn crowned(gamePiece: GamePiece) -> GamePiece {
    GamePiece {
      color: gamePiece.color,
      crowned: true,
    }
  }
}



