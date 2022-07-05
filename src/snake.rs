// snake.rs - Main file containing the Game struct

// Terminal io
use std::fmt;

// other stuff
use std::vec::Vec;

pub enum Directions {
    Up,
    Down,
    Left,
    Right,
    Keep,
}

struct Snake {
    segments: Vec<(u16, u16)>,
    direction: Directions,
}

pub struct Game {
    score: u32,
    snake: Snake,
    apples: Vec<(u16,u16)>,
    // board size does not include borders
    board_size: (u16, u16),
    is_alive: bool,
}

impl Game {
    pub fn new(&mut self) {
        self.score = 0;
        self.snake = Snake{segments: vec![(self.board_size.0/2, self.board_size.1/2)], direction: Directions::Right};
        self.generate_apples(5);
        self.is_alive = true;
    }
    pub fn get_score(&self) -> u32 {
        self.score
    }
    pub fn is_alive(&self) -> bool {
        self.is_alive
    }
    pub fn step(&mut self, direction_input: Directions) {
        match direction_input {
            Directions::Keep => (),
            _ => self.snake.direction = direction_input,
        }
        let direction = &self.snake.direction;
        let mut old_location = self.snake.segments[0];
        match direction {
            Directions::Up => self.snake.segments[0] = (self.snake.segments[0].0, self.snake.segments[0].1-1),
            Directions::Down => self.snake.segments[0] = (self.snake.segments[0].0, self.snake.segments[0].1+1),
            Directions::Left => self.snake.segments[0] = (self.snake.segments[0].0-1, self.snake.segments[0].1),
            Directions::Right => self.snake.segments[0] = (self.snake.segments[0].0+1, self.snake.segments[0].1),
            Directions::Keep => panic!(),
        }
        for segment in &mut self.snake.segments[1..] {
            let tmp1 = old_location;
            old_location = *segment;
            *segment = tmp1;
        }
        if self.apples.contains(&self.snake.segments[0]) {
            self.score += 1;
            let snake_head = self.snake.segments[0];
            self.snake.segments.push(old_location);
            self.move_apple((&self.apples).into_iter().position(move |r| *r == snake_head).unwrap());
        }
        if self.snake.segments[1..].contains(&self.snake.segments[0]) {
            self.is_alive = false;
        }
        // integer overflow handles negatives
        if self.snake.segments[0].0 >= self.board_size.0 || self.snake.segments[0].1 >= self.board_size.1 {
            self.is_alive = false;
        }
    }
    fn generate_apples(&mut self, apples: usize) {
        self.apples = vec![(0u16, 0u16); apples];
        for i in 0..apples {
            let mut apple = ((rand::random::<u16>())%(self.board_size.0), (rand::random::<u16>())%(self.board_size.1));
            while self.apples.contains(&apple) {
                apple = ((rand::random::<u16>())%(self.board_size.0), (rand::random::<u16>())%(self.board_size.1));
            }
            self.apples[i] = apple;
        }
    }
    fn move_apple(&mut self, apple: usize) {
        let mut tmpapple = ((rand::random::<u16>())%(self.board_size.0), (rand::random::<u16>())%(self.board_size.1));
        while self.snake.segments.contains(&tmpapple) || self.apples.contains(&tmpapple) {
            tmpapple = ((rand::random::<u16>())%(self.board_size.0), (rand::random::<u16>())%(self.board_size.1));
        }
        self.apples[apple] = tmpapple;
    }
}

impl Default for Game {
    fn default() -> Game {
        let term_size = crossterm::terminal::size().unwrap();
        let mut game = Game{
            score: 0,
            snake: Snake{segments: Vec::new(), direction: Directions::Right},
            apples: Vec::new(),
            board_size: (term_size.0-2, term_size.1-2),
            is_alive: true,
        };
        game.new();
        game
    }
}

impl fmt::Display for Game {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let true_board_size = (self.board_size.0 + 2, self.board_size.1 + 2);
        let mut output_string = vec![' '; ((true_board_size.0)*true_board_size.1) as usize];
        // create top/bottom borders
        for i in 0..true_board_size.0 {
            output_string[i as usize] = 'X';
            output_string[(i+((true_board_size.0)*(true_board_size.1-1))) as usize] = 'X';
        }
        // create left/right borders
        for i in 1..true_board_size.1-1 {
            output_string[(i*true_board_size.0) as usize] = 'X';
            output_string[(((i+1)*true_board_size.0)-1) as usize] = 'X';
        }
        // for the apples and snake segments, heres how drawing them works - i.0 is the column offset, and i.1 is the row offset.
        // The cursor moves forward <row*columns_in_row> columns, which overlaps into <row> rows. The columns are then added.
        // The +1 is for the borders, as everything is pushed in by one because of the borders.
        // draw apples
        for i in &self.apples {
            output_string[(i.0+((i.1+1)*true_board_size.0)+1) as usize] = 'O';
        }
        // draw snake
        for i in &self.snake.segments {
            output_string[(i.0+((i.1+1)*true_board_size.0)+1) as usize] = 'S';
        }
        let final_string: String = output_string.into_iter().collect();
        write!(f, "{}", final_string)
    }
}
