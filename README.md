# Space Battles Game Server

The core game for Space Battles is run from here, the frontends will display what happened as well as send the commands from each player. This server will do all the logic and calculate the end results before sending back the new state of the game.

## To Do

### Creating a Game / Lobby

- [x] Player can create a game
- [x] Player can join a game
- [x] Player gets an SSE (server sent event) for game updates
- [x] Player can get who is in the lobby
- [x] Player gets event when another player joins the lobby
- [x] Player can change ship type in lobby
- [x] Player gets event when another player changes ship types
- [x] Player can change ship color in lobby
- [x] Player gets event when another player changes their color in the lobby
- [x] When the host quits the game, someone else becomes the host
- [x] When the last player quits the game, the game ends
- [x] Player can ready up
- [x] Players in lobby can see other players who have readied up
- [x] A player joining a game cancels all players ready up

### Playing the Game

- [x] Players in game cannot change ship type
- [x] Players in game cannot change their color
- [x] When the game starts, players are assigned random locations
- [x] Players can only see the location of other players
- [x] Players can update their speed +/- 1
  - [x] add max speed to ships
  - [x] add speed to players
  - [x] Ships route should return the maximum speed
  - [x] Create get my player route to get my player
  - [x] create route to increase speed
  - [x] Speed cannot be increased past maximum speed
  - [x] Speed can be decreased by one
  - [x] Speed cannot be decreased below 0
- [x] Each player can set commands for the current turn
  - [x] Create tables for turns and player commands
  - [x] When a game starts, create the first turn
  - [x] A player can submit a command for the current turn
  - [x] A player cannot submit multiple commands for the same turn
  - [x] When all players are ready, apply the command to the game for each player
  - [x] Mark current turn no longer active
  - [x] Create a new turn
  - [x] Mark all players as not ready
- [x] Players can update their speed
  - [x] When a player is submitting a command, they can include a speed increase or decrease
  - [x] Players cannot change speeds twice in one turn
- [ ] Ships have a max / minimum distance they can travel based on speed
- [ ] Ships have shields
- [ ] Ships have hull strength
- [ ] Players can set their destination
- [ ] Players can target a cell to fire at
- [ ] Players can ready up their turn
- [ ] As soon as the last player readied up their turn the turn ends and the turn simulates
- [ ] Results of the turn are kept in a log
- [ ] Players get access to the log every turn
- [ ] Game ends when only one player remains

### Game Over

## Polishing

### Game Start

- [ ] Players are not assigned locations next to other players
- [ ] Tweak game size
