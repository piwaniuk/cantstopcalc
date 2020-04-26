# Calculator for Can't stop

"Can't stop" is a quite known board game. You can read more about it [here](https://en.wikipedia.org/wiki/Can't_Stop_(board_game)), or just search for it and play it online. The game uses interesting mechanics of a dice roll. Although the rules are relatively simple, some interesting properties emerge and it can be difficult to predict an outcome of an action. 

## How to run

The project was created in the Rust's Cargo packaging system, so the easiest way is to clone the repository, enter its directory and run:

    cargo run --

The output is probability of a failure. By default it is computer for an empty board, so the result would be 0. The input to the calculator is given as the command line arguments. Use `-b` to specify blocked (finished) paths and `-c` to specify chosen paths, for example following command will compute probability of failure when paths 6 and 8 are finished and the player has already picked paths 5, 7 and 9:

    cargo run -- -b 6 8 -c 5 7 9

In this case the output would be:

    Failing states count: 190
    Failing state probability: 0.147

## Final comments
This program was written as an exercise to learn the Rust programming language.
