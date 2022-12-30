#[derive(Debug, PartialEq)]
pub enum Player {
  One,
  Two,
}

impl Into<String> for Player {
  fn into(self) -> String {
    self.into()
  }
}

impl ToString for Player {
  fn to_string(&self) -> String {
    match self {
      Player::One => "Player 1".into(),
      Player::Two => "Player 2".into(),
    }
  }
}

impl Default for Player {
  fn default() -> Self {
    Self::One
  }
}
