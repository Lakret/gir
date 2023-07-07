#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Mark {
  Cross,
  Circle,
}

#[derive(Debug, Default, Clone)]
pub struct Game {
  state: [Option<Mark>; 9],
  circle_turn: bool,
}

const LINES: [[usize; 3]; 8] = [
  // rows
  [0, 1, 2],
  [3, 4, 5],
  [6, 7, 8],
  // columns
  [0, 3, 6],
  [1, 4, 7],
  [2, 5, 8],
  // diagonals
  [0, 4, 8],
  [2, 4, 6],
];

impl Game {
  pub fn is_cross_turn(&self) -> bool {
    !self.circle_turn
  }

  pub fn is_circle_turn(&self) -> bool {
    self.circle_turn
  }

  pub fn do_move(&mut self, pos: usize) {
    assert!(pos < 9);

    let mark = if self.is_cross_turn() {
      Mark::Cross
    } else {
      Mark::Circle
    };

    self.state[pos] = Some(mark);
    self.circle_turn = !self.circle_turn;
  }

  pub fn next_moves(&self) -> Vec<Game> {
    let mut moves = vec![];

    for pos in 0..9 {
      if self.state[pos].is_none() {
        let mut new_game = self.clone();
        new_game.do_move(pos);

        moves.push(new_game);
      }
    }

    moves
  }

  pub fn is_won(&self) -> Option<Mark> {
    for line in LINES {
      if let Some(mark) = self.state[line[0]] {
        if self.state[line[1]] == Some(mark) && self.state[line[2]] == Some(mark) {
          return Some(mark);
        }
      }
    }

    None
  }
}
