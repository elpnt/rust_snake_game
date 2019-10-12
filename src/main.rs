use piston_window::Button::Keyboard;
use piston_window::Key;
use piston_window::*;
use rand::Rng;
use std::collections::VecDeque;

const N_WIDTH: u32 = 30;
const N_HEIGHT: u32 = 30;
const CELLSIZE: f64 = 20.0;
const TIMELIMIT: f64 = 0.08;
const START_X: u32 = 3;
const START_Y: u32 = 3;
const APPLE_X: u32 = 10;
const APPLE_Y: u32 = 10;

const COLOR_WALL: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
const COLOR_APPLE: [f32; 4] = [0.8, 0.2, 0.2, 1.0];
const COLOR_SNAKE: [f32; 4] = [0.7, 0.7, 0.7, 1.0];

#[derive(Clone, Copy, PartialEq)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

impl Direction {
    fn opposite(&self) -> Direction {
        match self {
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
        }
    }
}

#[derive(Clone, Copy)]
struct Position {
    x: u32,
    y: u32,
}

impl Position {
    fn new(x: u32, y: u32) -> Position {
        Position { x, y }
    }

    fn change_position(&mut self) {
        let mut rng = rand::thread_rng();
        self.x = rng.gen_range(1, N_WIDTH + 1);
        self.y = rng.gen_range(1, N_HEIGHT + 1);
    }
}

struct Snake {
    head: Position,
    body: VecDeque<Position>,
    direction: Direction,
    duration: f64,
}

impl Snake {
    fn new(x: u32, y: u32) -> Snake {
        Snake {
            head: Position { x, y },
            body: VecDeque::new(),
            direction: Direction::Right,
            duration: 0.0,
        }
    }

    fn keypress(&mut self, button: Button) {
        let next_dir = match button {
            Keyboard(Key::Left) => Direction::Left,
            Keyboard(Key::Right) => Direction::Right,
            Keyboard(Key::Up) => Direction::Up,
            Keyboard(Key::Down) => Direction::Down,
            _ => self.direction,
        };

        if next_dir != self.direction.opposite() {
            self.direction = next_dir;
        }
    }

    fn proceed(&mut self) {
        self.body.pop_back();
        self.body
            .push_front(Position::new(self.head.x, self.head.y));
        match self.direction {
            Direction::Left => {
                self.head.x -= 1;
            }
            Direction::Right => {
                self.head.x += 1;
            }
            Direction::Up => {
                self.head.y -= 1;
            }
            Direction::Down => {
                self.head.y += 1;
            }
        }
    }

    fn reach_apple(&mut self, apple: &mut Position) -> bool {
        self.head.x == apple.x && self.head.y == apple.y
    }

    fn add_tail(&mut self) {
        self.body.push_front(Position::new(self.head.x, self.head.y));
    }

    fn check_alive(&self) -> bool {
        // self-intersection
        for p in &self.body {
            if p.x == self.head.x && p.y == self.head.y {
                return false;
            }
        }

        // collision with walls
        if self.head.x == 0
            || self.head.x == N_WIDTH + 1
            || self.head.y == 0
            || self.head.y == N_HEIGHT + 1
        {
            return false;
        }

        return true;
    }

    fn next(&mut self, dt: f64, apple: &mut Position, window: &mut PistonWindow) {
        if self.reach_apple(apple) {
            println!("Score: {}", self.body.len());
            self.add_tail();
            apple.change_position();
        }

        self.duration += dt;
        if self.duration > TIMELIMIT {
            self.proceed();
            self.duration = 0.0;

            if !self.check_alive() {
                println!("Game Over!\nPress SPACE to restart / ESC to quit.");
                window.set_lazy(true);
            }
        }
    }

    fn restart(&mut self, apple: &mut Position) {
        self.head.x = START_X;
        self.head.y = START_Y;
        self.body = VecDeque::new();
        self.direction = Direction::Right;
        self.duration = 0.0;
        apple.x = APPLE_X;
        apple.y = APPLE_Y;
    }
}

fn draw_rect(color: [f32; 4], x: u32, y: u32, c: Context, g: &mut G2d) {
    rectangle(color,
              [x as f64 * CELLSIZE, y as f64 * CELLSIZE, CELLSIZE, CELLSIZE],
              c.transform, g)
}

fn main() {
    let mut snake = Snake::new(START_X, START_Y);
    let mut apple = Position::new(APPLE_X, APPLE_Y);

    let width = (N_WIDTH + 2) * CELLSIZE as u32;
    let height = (N_HEIGHT + 2) * CELLSIZE as u32;
    let mut window: PistonWindow = WindowSettings::new("Snake Game", (width, height))
        .exit_on_esc(true)
        .build()
        .unwrap_or_else(|e| panic!("Failed to build PistonWindow: {}", e));

    while let Some(e) = window.next() {

        if let Some(_) = e.render_args() {
            window.draw_2d(&e, |c, g, _| {
                // background
                clear([0.2, 0.2, 0.2, 1.0], g);
                // snake's body
                for p in &snake.body {
                    draw_rect(COLOR_SNAKE, p.x, p.y, c, g);
                }
                // snake's head
                draw_rect(COLOR_SNAKE, snake.head.x, snake.head.y, c, g);
                // apple
                draw_rect(COLOR_APPLE, apple.x, apple.y, c, g);
                // wall
                for i in 0..N_HEIGHT + 2 {
                    if i == 0 || i == N_HEIGHT + 1 {
                        for j in 0..N_WIDTH + 2 {
                            draw_rect(COLOR_WALL, j, i, c, g);
                        }
                    } else {
                        draw_rect(COLOR_WALL, 0, i, c, g);
                        draw_rect(COLOR_WALL, N_WIDTH+1, i, c, g);
                    }
                }
            });
        }

        // listen a key pressing event
        if let Some(button) = e.press_args() {
            snake.keypress(button);
        }

        // update field
        e.update(|u| {
            snake.next(u.dt, &mut apple, &mut window);
        });

        // Restart with pressing `space` key
        if window.get_event_settings().lazy {
            if let Some(button) = e.press_args() {
                if button == Keyboard(Key::Space) {
                    snake.restart(&mut apple);
                    window.set_lazy(false);
                }
            }
        }
    }
}
