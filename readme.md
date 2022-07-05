# snake-rs
A very simple TUI snake game written with [crossterm](https://github.com/crossterm-rs/crossterm)

## controls
Arrow keys - move, escape - stops game

## faq
### the game is too big
Get a smaller screen.
### the game panics when I die
The game relies on unsigned integer overflow for a small part of functionality (hitting the top/left borders). To stop the game without a panic when this happens, build with `--release`.
### I want more/less apples
Edit `src/snake.rs`, look for the `new()` method implemeneted by the snake, and change the `generate_apples()` function call to make however many apples you want. More apples means more lag, though.
