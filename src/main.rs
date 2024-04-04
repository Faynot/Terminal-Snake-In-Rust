use std::collections::LinkedList;
use std::io::{self, Read};
use std::thread;
use std::time::{Duration, Instant};

const WIDTH: usize = 20;
const HEIGHT: usize = 10;

#[derive(PartialEq, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn opposite(&self) -> Direction {
        match *self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }
}

struct Snake {
    body: LinkedList<(usize, usize)>,
    direction: Direction,
    next_direction: Option<Direction>,
}

impl Snake {
    fn new() -> Snake {
        let mut body = LinkedList::new();
        body.push_back((2, 2));
        body.push_back((2, 3));
        Snake {
            body,
            direction: Direction::Right,
            next_direction: None,
        }
    }

    fn move_forward(&mut self) {
        let (mut x, mut y) = *self.body.front().expect("Snake has no body");
        match self.next_direction {
            Some(dir) => {
                self.direction = dir;
                self.next_direction = None;
            }
            None => {}
        }
        match self.direction {
            Direction::Up => {
                if y == 0 {
                    y = HEIGHT - 1;
                } else {
                    y -= 1;
                }
            }
            Direction::Down => {
                y = (y + 1) % HEIGHT;
            }
            Direction::Left => {
                if x == 0 {
                    x = WIDTH - 1;
                } else {
                    x -= 1;
                }
            }
            Direction::Right => {
                x = (x + 1) % WIDTH;
            }
        }
        // Check if snake eats food
        if self.body.front().unwrap() == &(x, y) {
            self.body.push_front((x, y));
        } else {
            self.body.pop_back();
            self.body.push_front((x, y));
        }
    }

    fn change_direction(&mut self, dir: Direction) {
        if self.direction != dir.opposite() {
            self.next_direction = Some(dir);
        }
    }

    fn check_collision(&self) -> bool {
        let (head_x, head_y) = *self.body.front().expect("Snake has no body");
        if self
            .body
            .iter()
            .skip(1)
            .any(|&(x, y)| x == head_x && y == head_y)
        {
            return true; // Collides with itself
        }
        false
    }

    fn eat(&mut self, food: (usize, usize)) {
        self.body.push_back(food);
    }
}

struct Food {
    position: (usize, usize),
}

impl Food {
    fn new() -> Food {
        Food {
            position: (5, 5), // Initialize food position
        }
    }

    fn spawn(&mut self) {
        // Respawn food at random position
        self.position = (rand::random::<usize>() % WIDTH, rand::random::<usize>() % HEIGHT);
    }
}

fn main() {
    let mut snake = Snake::new();
    let mut food = Food::new();
    let mut last_update = Instant::now();

    loop {
        if last_update.elapsed() >= Duration::from_millis(100) {
            // Clear the screen
            print!("{}[2J", 27 as char);

            // Draw the snake
            for y in 0..HEIGHT {
                for x in 0..WIDTH {
                    if snake.body.contains(&(x, y)) {
                        print!("■");
                    } else if (x, y) == food.position {
                        print!("●");
                    } else {
                        print!(" ");
                    }
                }
                println!();
            }

            // Update the game state
            snake.move_forward();

            // Check for collisions
            if snake.check_collision() {
                println!("Game over!");
                break;
            }

            // Check if snake eats food
            if snake.body.front().unwrap() == &food.position {
                snake.eat(food.position);
                food.spawn();
            }

            // Check for user input
            if let Ok(key) = read_key() {
                match key {
                    'w' => snake.change_direction(Direction::Up),
                    's' => snake.change_direction(Direction::Down),
                    'a' => snake.change_direction(Direction::Left),
                    'd' => snake.change_direction(Direction::Right),
                    _ => {}
                }
            }

            last_update = Instant::now();
        }

        thread::sleep(Duration::from_millis(50));
    }
}

fn read_key() -> io::Result<char> {
    let mut buffer = [0; 1];
    io::stdin().read_exact(&mut buffer)?;
    Ok(buffer[0] as char)
}
