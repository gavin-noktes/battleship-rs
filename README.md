# Battleship Game
A simple battleship game written 100% in rust.

## Gameplay
To start, please use `cargo run` or build the program and run the executable.
Player One will be asked to place their ships. The syntax to place a ship is as follows: "x_coordinate y_coordinate direction". The direction of the ship can be one of `u` `d` `l` or `r`.
Player Two will then be asked the same question.

After this, the gameplay loop will start. Try to hit ships with: "x_coordinate y_coordinate".
Good Luck!

## Todo (Probably won't go back to work on this, but if I do, these are the big Todos)
- [x] Create initial gameplay loop
- [x] Turn `ship_kind` into it's own Enum instead of using `String`
- [ ] Bot to play against
- [ ] Remove all uses of unwrap (Too lazy to do that right now)
- [ ] Make ship placement a little more interactive (You can see ships after you place them and are asked if thats where you want it)