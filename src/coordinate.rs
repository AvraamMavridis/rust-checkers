#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Coordinate {
  x: usize,
  y: usize,
}

pub type AllowedMoves = Vec<Coordinate>;

impl Coordinate {
  pub fn is_on_board(&self) -> bool {
    self.x <= 8 && self.y <= 8 && self.x > 0 && self.y > 0 
  }

  pub fn get_possible_moves(&self) -> AllowedMoves {
    let mut allowed_moves: AllowedMoves = vec![
      Coordinate{ x: self.x + 1, y: self.y + 1 },
      Coordinate{ x: self.x - 1, y: self.y + 1 },
      Coordinate{ x: self.x + 1, y: self.y - 1 },
      Coordinate{ x: self.x - 1, y: self.y - 1 },
    ];

    allowed_moves
      .iter()
      .filter(|possible_move| possible_move.is_on_board())
      .cloned()
      .collect()
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
}
