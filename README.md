# Usage

Clone the repo and run
```
cargo run
```
to launch the game.

## Rules
The rules are the basic four in a row rules:
The player who gets four stones in a row first, wins.

## Screenshot
The game is completely cli based.
The board is drawn in the cli and every player selects in which column he or she wants to put the next stone.

```
-----------------------------
|   |   |   |   |   |   |   |
-----------------------------
|   |   |   | o |   |   |   |
-----------------------------
|   |   |   | x |   |   |   |
-----------------------------
|   |   | o | x | x |   |   |
-----------------------------
|   |   | o | x | x | x | o |
-----------------------------
| o |   | x | o | o | o | x |
-----------------------------
  1   2   3   4   5   6   7 

Player 2 wins!
```