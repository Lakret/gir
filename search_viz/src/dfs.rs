use std::{
  collections::{HashMap, HashSet},
  fmt::Display,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Mark {
  Cross,
  Circle,
}

impl Mark {
  fn opponent(self) -> Mark {
    match self {
      Mark::Cross => Mark::Circle,
      Mark::Circle => Mark::Cross,
    }
  }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
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

  pub fn next_moves_with_pos(&self) -> Vec<(Game, usize)> {
    let mut moves = vec![];

    for pos in 0..9 {
      if self.state[pos].is_none() {
        let mut new_game = self.clone();
        new_game.do_move(pos);

        moves.push((new_game, pos));
      }
    }

    moves
  }

  pub fn is_won(&self) -> bool {
    self.winning_mark().is_some()
  }

  pub fn is_draw(&self) -> bool {
    self.state.iter().all(|cell| cell.is_some()) && !self.is_won()
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

  pub fn select_next_move(&self) -> Option<usize> {
    self
      .next_moves_with_pos()
      .into_iter()
      .map(|(game, pos)| (pos, game.minimax(false, 0)))
      .max_by_key(|(_pos, score)| *score)
      .map(|(best_pos, _best_score)| best_pos)
  }

  pub fn minimax(&self, is_maximizing: bool, depth: i32) -> i32 {
    if self.is_won() {
      if is_maximizing {
        return -10 + depth;
      } else {
        return 10 - depth;
      }
    };

    if self.is_draw() {
      return 0;
    }

    if is_maximizing {
      // maximizing player
      let mut best_score = i32::MIN;
      for (next_move, _pos) in self.next_moves_with_pos() {
        best_score = best_score.max(next_move.minimax(false, depth + 1));
      }
      return best_score;
    } else {
      // minimizing player
      let mut best_score = i32::MAX;
      for (next_move, _pos) in self.next_moves_with_pos() {
        best_score = best_score.min(next_move.minimax(true, depth + 1));
      }
      return best_score;
    }
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
  fn select_next_moves_test() {
    let mut game = Game::default();
    // cross in the center
    game.do_move_row_col(1, 1);
    println!("{}", game);

    let pos = game.select_next_move();
    game.do_move(pos.unwrap());
    println!("{}", game);

    game.do_move_row_col(0, 1);
    println!("{}", game);

    let pos = game.select_next_move();
    game.do_move(pos.unwrap());
    println!("{}", game);

    game.do_move_row_col(2, 0);
    println!("{}", game);

    let pos = game.select_next_move();
    game.do_move(pos.unwrap());
    println!("{}", game);

    game.do_move_row_col(1, 2);
    println!("{}", game);

    let pos = game.select_next_move();
    game.do_move(pos.unwrap());
    println!("{}", game);
  }
}
