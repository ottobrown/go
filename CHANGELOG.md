# 0.4.1
- Revamp how endgames work:
    - Prevent moving when the game is over (When 2 passes have happened or a player has resigned)
    - Allow editing of the game's result from the `Game info` window.

# 0.4.0

## New features
- New tools to mark up the board!
    - Triangle
    - Circle
    - Square
    - Cross
    - Line
    - Arrow
    - Letter
    - Number
    - Custom label
- Prevent terminal from always opening when running on windows

## Ui changes
- New layout: Player info on top left and top right of the board
- Move editor `Frame` and `ScrollArea` to main `CentralPanel`

# 0.3.0

## New features
- The game is now stored as a tree, allowing for different variations
- Prevent continued play if the game has ended
- Count number of captured stones

## Bug fixes
- Prevent illegal moves being stored in the game

## UI changes
- Arrow buttons to navigate game tree
- Display result of game (ex. "Black won by resignation.")

# 0.2.1
- Fix star points
- Fix bug that prevented stones from being captured when using the Move tool

# 0.2.0

## UI changes
- Star points!
- Standardize capitalization of UI
- Replace board info `ComboBox` with a `Window`
- UI to create `Rules`
- Add horizontal and vertical scrolling to editor
- Use `Grid` layout for game builder UI
- Scale Board size based on egui item spacing

## Structure changes
- Add CHANGELOG.md
