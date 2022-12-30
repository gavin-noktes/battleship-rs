#[derive(Clone)]
pub struct Cell {
  // Whether the cell has been hit or not
  pub hit: bool,
  // Whether the cell contains a ship or not
  pub has_ship: bool,
}

impl Default for Cell {
  fn default() -> Self {
    Cell {
      hit: false,
      has_ship: false,
    }
  }
}
