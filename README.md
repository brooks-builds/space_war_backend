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

- [ ] Players in game cannot change ship type
- [ ] Players in game cannot change their color
- [ ] Players can see their own stats
- [ ] Players can only see the location of other players
- [ ] Players can update their speed +/- 1
- [ ] Ships have a max speed
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
