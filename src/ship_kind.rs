#[derive(Debug, Clone, Copy)]
pub enum ShipKind {
  Carrier,
  Battleship,
  Cruiser,
  Submarine,
  Destroyer,
}

impl ShipKind {
  pub fn size(&self) -> usize {
    match self {
      ShipKind::Carrier => 5,
      ShipKind::Battleship => 4,
      ShipKind::Cruiser => 3,
      ShipKind::Submarine => 3,
      ShipKind::Destroyer => 2,
    }
  }
}

impl ToString for ShipKind {
  fn to_string(&self) -> String {
    match self {
      ShipKind::Carrier => "Carrier".into(),
      ShipKind::Battleship => "Battleship".into(),
      ShipKind::Cruiser => "Cruiser".into(),
      ShipKind::Submarine => "Submarine".into(),
      ShipKind::Destroyer => "Destroyer".into(),
    }
  }
}
