# rust-nonogram
![Nonogram screenshot](assets/screenshots/game_screenshot.png)

## What is nonogram?
[Nonograms](https://en.wikipedia.org/wiki/Nonogram) (AKA Picross) are logic
puzzles that involve filling in cells on a grid. Each row and column has
a set of clues representing the 'runs' in that line, in order. For example,
`5 1 1 2` means a set of 5 filled squares, followed by two separate, singular
filled squares, and finally a set of 2 filled squares. The gaps between them
can be of any length, but there are no more filled squares than the clues
indicate.

# Usage
Run with `cargo run --release`

## Controls
| Action | Key |
| --- | --- |
| Toggle cell | Left Click |
| Toggle 'X' | Right Click |
