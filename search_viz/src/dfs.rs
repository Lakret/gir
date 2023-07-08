use std::fmt::Display;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Mark {
  Cross,
  Circle,
}

#[derive(Debug, Default, Clone, PartialEq)]
pub struct Game {
  state: [Option<Mark>; 9],
  circle_turn: bool,
}

impl Display for Game {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    for row in 0..3 {
      for col in 0..3 {
        let ch = match self.mark_at(row, col) {
          None => "口",
          Some(Mark::Cross) => "Ｘ",
          Some(Mark::Circle) => "⚪",
        };
        write!(f, "{} ", ch)?;
      }
      writeln!(f)?;
    }

    Ok(())
  }
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
  pub fn mark_at(&self, row: usize, col: usize) -> Option<Mark> {
    if let Some(pos) = row_col_to_pos(row, col) {
      return self.state[pos];
    }

    None
  }

  pub fn is_occupied(&self, row: usize, col: usize) -> bool {
    if let Some(pos) = row_col_to_pos(row, col) {
      return self.state[pos].is_some();
    }

    false
  }

  pub fn is_cross_turn(&self) -> bool {
    !self.circle_turn
  }

  pub fn is_circle_turn(&self) -> bool {
    self.circle_turn
  }

  pub fn do_move_row_col(&mut self, row: usize, col: usize) {
    let pos = row_col_to_pos(row, col).expect("invalid row or col");
    self.do_move(pos);
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

  pub fn is_won(&self) -> bool {
    self.winning_mark().is_some()
  }

  pub fn winning_mark(&self) -> Option<Mark> {
    for line in LINES {
      if let Some(mark) = self.state[line[0]] {
        if self.state[line[1]] == Some(mark) && self.state[line[2]] == Some(mark) {
          return Some(mark);
        }
      }
    }

    None
  }

  // computer always plays as circles
  pub fn select_next_move(&self) -> Option<usize> {
    // TODO: run dfs and figure out which turn to take
    todo!();
    None
  }
}

fn row_col_to_pos(row: usize, col: usize) -> Option<usize> {
  let pos = row * 3 + col;

  if pos < 9 {
    Some(pos)
  } else {
    None
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn next_moves_test() {
    // TODO: check next moves
    let mut game = Game::default();
    // cross in the center
    game.do_move_row_col(1, 1);
    game.do_move_row_col(0, 0);
    println!("{}", game);
  }
}
