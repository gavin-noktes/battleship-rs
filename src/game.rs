use std::io::stdin;

use crate::{cell::Cell, player::Player, ship::Ship, ship_kind::ShipKind};

pub struct Game {
  // The two players in the game
  pub player1: Player,
  pub player2: Player,
  // The size of the board (number of rows and columns)
  pub size: usize,
  // The boards for each player
  pub player1_board: Vec<Vec<Cell>>,
  pub player2_board: Vec<Vec<Cell>>,
  // The players' ships
  pub player1_ships: Vec<Ship>,
  pub player2_ships: Vec<Ship>,
}

impl Game {
  pub fn play(&mut self) {
    let mut current_player = &Player::One;
    let mut player_one_placing = true;
    let mut player_two_placing = true;

    loop {
      if player_one_placing || player_two_placing {
        self.place_ships(current_player);
        current_player = if current_player == &Player::One {
          player_one_placing = false;
          &Player::Two
        } else {
          player_two_placing = false;
          &Player::One
        };
        continue;
      }
      self.print_boards();

      println!("It is {}'s turn.", current_player.to_string());
      println!("Enter the coordinates of your shot (x y):");

      let (x, y, _) = Self::get_coordinates(false);

      let result = self.take_shot(current_player, x, y);
      if result.is_err() {
        println!("Error: {}", result.unwrap_err());
        continue;
      }

      println!("Shot successful!");

      if self.game_over() {
        println!("Game over!");
        break;
      }

      current_player = if current_player == &Player::One {
        &Player::Two
      } else {
        &Player::One
      };
    }
  }

  fn game_over(&self) -> bool {
    let player1_ships = self.player1_ships.iter().all(|s| s.sunk);
    let player2_ships = self.player2_ships.iter().all(|s| s.sunk);
    player1_ships || player2_ships
  }

  fn place_ships(&mut self, player: &Player) {
    Self::print_board(&self, player);

    let mut ships = vec![
      ShipKind::Carrier,
      ShipKind::Battleship,
      ShipKind::Cruiser,
      ShipKind::Submarine,
      ShipKind::Destroyer,
    ];

    loop {
      let ship = match ships.get(0) {
        Some(ship) => ship,
        None => break,
      };

      println!("Placing a {} ({} squares)", ship.to_string(), ship.size());
      println!("Enter the starting coordinates and direction (x y d):");

      let (x, y, d) = Self::get_coordinates(true);

      let positions = match d.as_str() {
        "u" => (0..ship.size()).map(|i| (x, y - i)).collect(),
        "d" => (0..ship.size()).map(|i| (x, y + i)).collect(),
        "l" => (0..ship.size()).map(|i| (x - i, y)).collect(),
        "r" => (0..ship.size()).map(|i| (x + i, y)).collect(),
        _ => {
          println!("Invalid direction");
          continue;
        }
      };

      let ship = Ship {
        kind: *ship,
        positions,
        sunk: false,
      };
      if let Err(e) = self.add_ship(player, ship) {
        println!("Error placing ship: {}", e);
        continue;
      }

      ships.remove(0);
      println!("Ship placed successfully!");
    }
  }

  fn get_coordinates(get_d: bool) -> (usize, usize, String) {
    loop {
      let mut input = String::new();
      stdin().read_line(&mut input).unwrap();
      let mut coordinates = input.split_whitespace();

      let x = match coordinates.next().and_then(|s| s.parse().ok()) {
        Some(x) => x,
        None => {
          println!("Invalid input");
          continue;
        }
      };

      let y = match coordinates.next().and_then(|s| s.parse().ok()) {
        Some(y) => y,
        None => {
          println!("Invalid input");
          continue;
        }
      };

      if get_d {
        let d = match coordinates.next() {
          Some(d) => d,
          None => {
            println!("Invalid input");
            continue;
          }
        };
        return (x, y, d.into());
      }

      return (x, y, "".into());
    }
  }

  pub fn add_ship(&mut self, player: &Player, ship: Ship) -> Result<(), String> {
    let board = match player {
      Player::One => &mut self.player1_board,
      Player::Two => &mut self.player2_board,
    };

    for (x, y) in &ship.positions {
      if x >= &self.size || y >= &self.size {
        return Err(String::from("Invalid coordinates"));
      }

      if board[*x][*y].has_ship {
        return Err(String::from("There is already a ship in that cell"));
      }

      board[*x][*y].has_ship = true;
    }

    match player {
      Player::One => self.player1_ships.push(ship),
      Player::Two => self.player2_ships.push(ship),
    };

    Ok(())
  }

  pub fn take_shot(&mut self, player: &Player, x: usize, y: usize) -> Result<(), String> {
    let (board, ships) = match player {
      Player::One => (&mut self.player2_board, &mut self.player2_ships),
      Player::Two => (&mut self.player1_board, &mut self.player1_ships),
    };

    if x >= self.size || y >= self.size {
      return Err(String::from("Invalid coordinates"));
    }

    let cell = &mut board[x][y];
    if cell.hit {
      return Err(String::from("Cell has already been hit"));
    }

    cell.hit = true;
    if cell.has_ship {
      ships
        .iter_mut()
        .find(|s| s.positions.contains(&(x, y)))
        .unwrap()
        .sunk = true;
    }

    Ok(())
  }

  fn print_boards(&self) {
    let padding = " ".repeat(self.size * 3);
    println!("Player 1:{}Player 2:", padding);
    print!("  ");
    for i in 0..self.size {
      print!("{:2} ", i);
    }
    print!("       ");
    for i in 0..self.size {
      print!("{:2} ", i);
    }
    println!();
    for i in 0..self.size {
      print!("{:2} ", i);
      for j in 0..self.size {
        let cell = &self.player1_board[i][j];
        if cell.hit {
          if cell.has_ship {
            print!("X  ");
          } else {
            print!("O  ");
          }
        } else {
          print!(".  ");
        }
      }
      print!("       ");
      for j in 0..self.size {
        let cell = &self.player2_board[i][j];
        if cell.hit {
          if cell.has_ship {
            print!("X  ");
          } else {
            print!("O  ");
          }
        } else {
          print!(".  ");
        }
      }
      println!();
    }
  }

  fn print_board(&self, player: &Player) {
    let board = match player {
      Player::One => &self.player1_board,
      Player::Two => &self.player2_board,
    };

    println!("  0 1 2 3 4 5 6 7 8 9");
    for (i, row) in board.iter().enumerate() {
      print!("{} ", i);
      for cell in row {
        if cell.hit {
          if cell.has_ship {
            print!("X ");
          } else {
            print!("O ");
          }
        } else {
          print!(". ");
        }
      }
      println!();
    }
  }
}
impl Default for Game {
  fn default() -> Self {
    Game {
      player1: Player::default(),
      player2: Player::default(),
      size: 10,
      player1_board: vec![vec![Cell::default(); 10]; 10],
      player2_board: vec![vec![Cell::default(); 10]; 10],
      player1_ships: vec![],
      player2_ships: vec![],
    }
  }
}
