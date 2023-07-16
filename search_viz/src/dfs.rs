use std::{
  collections::{HashMap, HashSet},
  fmt::Display,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Mark {
  Cross,
  Circle,
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

  // TODO: BFS is much better here, because we can adjust score based on the depth of a level directly
  // thus => switch to minimax
  pub fn score_next_moves(&self, player_mark: Mark) -> HashMap<usize, f32> {
    let mut scores = HashMap::new();
    let mut is_first_turn = true;

    let mut stack = vec![(*self, 0, 1)];
    let mut seen = HashSet::new();

    while let Some((game, first_turn_pos, depth)) = stack.pop() {
      if !seen.contains(&game) {
        seen.insert(game);

        for (next_move_game, pos) in game.next_moves_with_pos() {
          // if it's the first turn the player makes on this branch, record the position for scoring;
          // if it's deeper in the tree than the first turn, we just preserve the first turn position
          let first_turn_pos = if is_first_turn { pos } else { first_turn_pos };

          match next_move_game.winning_mark() {
            None => stack.push((next_move_game, first_turn_pos, depth + 1)),
            Some(next_game_win_mark) => {
              if next_game_win_mark == player_mark {
                // TODO: make depth cliff more profound: neg inf or something for the depth==2 lose
                scores
                  .entry(first_turn_pos)
                  .and_modify(|score| *score += 1.0 * (1.0 / (depth as f32 * 10.0)))
                  .or_insert(1.0);
              } else {
                scores
                  .entry(first_turn_pos)
                  .and_modify(|score| *score -= 1.0 * (1.0 / (depth as f32 * 10.0)))
                  .or_insert(-1.0);
              }
            }
          }
        }

        is_first_turn = false;
      }
    }

    scores
  }

  pub fn select_next_move(&self, mark: Mark) -> Option<usize> {
    let scores = self.score_next_moves(mark);
    scores
      .iter()
      // TODO: unwrap
      .max_by(|(_pos1, score1), (_pos2, score2)| score1.partial_cmp(score2).unwrap())
      .map(|(pos, _score)| *pos)
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

    let scores = game.score_next_moves(Mark::Circle);
    dbg!(scores);

    let pos = game.select_next_move(Mark::Circle);
    game.do_move(pos.unwrap());
    println!("{}", game);

    game.do_move_row_col(0, 0);
    println!("{}", game);

    let scores = game.score_next_moves(Mark::Circle);
    dbg!(scores);
    let pos = game.select_next_move(Mark::Circle);
    game.do_move(pos.unwrap());
    println!("{}", game);
  }
}
