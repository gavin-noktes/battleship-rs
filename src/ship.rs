use crate::ship_kind::ShipKind;

pub struct Ship {
  // The type of ship (e.g. "destroyer", "cruiser", etc.)
  pub kind: ShipKind,
  // The cells that the ship occupies on the board
  pub positions: Vec<(usize, usize)>,
  // Whether the ship has been sunk or not
  pub sunk: bool,
}

impl Ship {
  pub fn size(&self) -> usize {
    self.kind.size()
  }
}

impl Default for Ship {
  fn default() -> Self {
    Ship {
      kind: ShipKind::Destroyer,
      positions: Vec::new(),
      sunk: false,
    }
  }
}
