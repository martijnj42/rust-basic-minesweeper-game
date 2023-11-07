# rust-basic-minesweeper-game
This is a basic Minesweeper game implemented in Rust using the sdl2 library.

## Player guide:

### How to Play (Standard Game):

1. Run the executable.
2. Click on the squares you want to open.
3. Press the "F" key to toggle between actions: open and flag while clicking a square.

### How to Play (Custom Game):

1. Find the executable using the terminal.
2. Run the game with the following command: "executable_path grid_size mines", where:
   - `executable_path` is the name of your executable.
   - `grid_size` is the length of one side of the square grid.
   - `mines` is the number of mines on the map.
   - Example: "Users/me/Desktop/minesweeper 4 2"

### Color Explanations:

- Unopened squares are Grey.
- Flagged squares are Cyan.
- Opened squares are color-coded based on the number of mines they border:
  - 0 = Yellow
  - 1 = Orange
  - 2 = Light Red
  - 3 = Light Purple
  - 4 = Purple
  - 5 = Dark Purple
  - 6 = Dark Green
  - 7 = Green
  - 8 = Light Green
- Revealed Mine squares are Black.

### Extra Options:

- Press the "H" key to let the computer help you. If a good option is available, it will take it; otherwise, it opens a random square.
- Press the "C" key to toggle between normal and developer mode.

Have fun playing Minesweeper!
