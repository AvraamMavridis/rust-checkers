#[derive(Debug, Copy, Clone, PartialEq)]
pub enum PossibleMoves {
  Simple = 1,
  Jump = 2,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Coordinate {
  x: usize,
  y: usize,
}

pub type AllowedMoves = Vec<Coordinate>;

impl Coordinate {
  fn generate_possible_moves(&self, offset: PossibleMoves) -> AllowedMoves {
    let offset_value = offset as usize;

    vec![
      Coordinate{ x: self.x + offset_value, y: self.y + offset_value },
      Coordinate{ x: self.x - offset_value, y: self.y + offset_value },
      Coordinate{ x: self.x + offset_value, y: self.y - offset_value },
      Coordinate{ x: self.x - offset_value, y: self.y - offset_value },
    ]
  }

  fn filter_moves(allowed_moves: &AllowedMoves) -> AllowedMoves {
    allowed_moves
      .iter()
      .filter(|possible_move| possible_move.is_on_board())
      .cloned()
      .collect()
  }

  pub fn is_on_board(&self) -> bool {
    self.x <= 8 && self.y <= 8 && self.x > 0 && self.y > 0 
  }

  pub fn get_possible_moves(&self) -> AllowedMoves {
    let allowed_moves = self.generate_possible_moves(PossibleMoves::Simple);
    Coordinate::filter_moves(&allowed_moves)
  }

  pub fn get_possible_jumps(&self) -> AllowedMoves {
    let allowed_moves = self.generate_possible_moves(PossibleMoves::Jump);
    Coordinate::filter_moves(&allowed_moves)
  }
}


#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn it_should_calculate_the_possible_moves_for_pieces_not_in_the_edge(){
    let cord = Coordinate {
      x: 2,
      y: 2,
    };

    let expected_moves: AllowedMoves = vec![
      Coordinate { x: 3, y: 3 }, 
      Coordinate { x: 1, y: 3 }, 
      Coordinate { x: 3, y: 1 }, 
      Coordinate { x: 1, y: 1 }
    ];

    assert_eq!(cord.get_possible_moves(), expected_moves);
  }

  #[test]
  fn it_should_calculate_the_possible_moves_for_pieces_in_the_edge(){
    let cord = Coordinate {
      x: 1,
      y: 1,
    };

    let expected_moves: AllowedMoves = vec![Coordinate { x: 2, y: 2 }];

    assert_eq!(cord.get_possible_moves(), expected_moves);
  }

  #[test]
  fn it_should_calculate_the_possible_moves_for_pieces_in_the_edge_2(){
    let cord = Coordinate {
      x: 1,
      y: 7,
    };

    let expected_moves: AllowedMoves = vec![Coordinate { x: 2, y: 8 }, Coordinate { x: 2, y: 6 }];

    assert_eq!(cord.get_possible_moves(), expected_moves);
  }

  #[test]
  fn it_should_calculate_the_possible_jumps_for_pieces_not_in_the_edge(){
    let cord = Coordinate {
      x: 4,
      y: 4,
    };

    let expected_moves: AllowedMoves = vec![
      Coordinate { x: 6, y: 6 }, 
      Coordinate { x: 2, y: 6 }, 
      Coordinate { x: 6, y: 2 }, 
      Coordinate { x: 2, y: 2 }
    ];

    assert_eq!(cord.get_possible_jumps(), expected_moves);
  }

  #[test]
  fn it_should_calculate_the_possible_jumps_for_pieces_in_the_edge(){
    let cord = Coordinate {
      x: 2,
      y: 2,
    };

    let expected_moves: AllowedMoves = vec![
      Coordinate { x: 4, y: 4 }, 
    ];

    assert_eq!(cord.get_possible_jumps(), expected_moves);
  }
}
