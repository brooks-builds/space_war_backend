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
- [ ] Player can ready up
- [ ] Player gets event when another player readies up
- [ ] A player joining a game cancels all players ready up

### Playing the Game
