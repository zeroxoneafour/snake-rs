use crossterm::{
    terminal::{enable_raw_mode, disable_raw_mode, Clear, ClearType},
    event::{read, Event, KeyEvent, KeyCode},
    cursor::{MoveToColumn, Hide, Show},
    ExecutableCommand,
};
use std::{
    io::{Write, stdout},
    thread,
    time::Duration,
    sync::mpsc,
};

mod snake;
use snake::{Game, Directions};

fn main() {
    let mut game = Game::default();
    // the input runs in a separate thread and communicates via these channels
    let (input_tx, input_rx) = mpsc::channel();
    enable_raw_mode();
    thread::spawn(move || loop {
        match read().unwrap() {
            Event::Key(event) => input_tx.send(event).unwrap(),
            _ => (),
        }
    });
    println!("snake game: press any key to start");
    // this stuff is here to move the cursor to the beginning of the line, so the output is aligned correctly
    stdout().execute(MoveToColumn(0));
    stdout().execute(Hide);
    let mut input: KeyEvent;
    // make sure input has been initialized at least once
    loop {
        match input_rx.try_recv() {
            Ok(event) => {input = event; break},
            Err(_) => (),
        }
    }
    while game.is_alive() {
        // loop this to clear out everything but the last input
        loop {
            match input_rx.try_recv() {
                Ok(event) => input = event,
                Err(_) => break,
            }
        }
        match input.code {
            KeyCode::Left => game.step(Directions::Left),
            KeyCode::Right => game.step(Directions::Right),
            KeyCode::Up => game.step(Directions::Up),
            KeyCode::Down => game.step(Directions::Down),
            KeyCode::Esc => break,
            _ => game.step(Directions::Keep),
        }
        if game.is_alive() {
            write!(stdout(), "{}", game);
        }
        // flush to stdout because write! doesnt
        stdout().flush();
        thread::sleep(Duration::from_millis(500));
    }
    stdout().execute(Clear(ClearType::Purge));
    stdout().execute(Show);
    disable_raw_mode();
    println!("Your final score was {}", game.get_score());
}
