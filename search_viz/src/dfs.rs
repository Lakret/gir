#[derive(Debug, Clone, Copy)]
pub enum Mark {
  Cross,
  Circle,
}

#[derive(Debug, Default, Clone)]
pub struct Game {
  state: [Option<Mark>; 9],
  circle_turn: bool,
}

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
  }

  pub fn next_moves(&self) -> impl Iterator<Item = Game> {
    todo!();
    vec![].into_iter()
  }

  pub fn is_won(&self) -> bool {
    todo!()
  }
}
