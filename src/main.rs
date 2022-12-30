mod cell;
mod game;
mod player;
mod ship;
mod ship_kind;

use std::vec;

use cell::Cell;
use game::Game;

fn main() {
  let mut game = Game {
    size: 25,
    player1_board: vec![vec![Cell::default(); 25]; 25],
    player2_board: vec![vec![Cell::default(); 25]; 25],
    ..Default::default()
  };

  game.play();
}
