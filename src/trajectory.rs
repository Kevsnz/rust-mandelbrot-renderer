#[derive(Clone, Copy)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

impl Point {
    pub fn new(x: f64, y: f64) -> Point {
        Point { x, y }
    }
}

impl std::ops::Sub for Point {
    type Output = Point;
    fn sub(self, rhs: Point) -> Point {
        Point {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl std::ops::Add for Point {
    type Output = Point;
    fn add(self, rhs: Point) -> Point {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl std::ops::Mul<f64> for Point {
    type Output = Point;
    fn mul(self, rhs: f64) -> Point {
        Point {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

struct Move {
    target: Point,
    steps: i32,
}

impl Move {
    fn new(target: Point, steps: i32) -> Move {
        Move { target, steps }
    }

    fn step(&self, point: Point, current_step: i32) -> Point {
        let step = (self.target - point) * (1.0 / (self.steps - current_step) as f64);
        point + step
    }

    fn finished(&self, current_step: i32) -> bool {
        current_step >= self.steps
    }
}

pub struct Trajectory {
    moves: Vec<Move>,
    current_move: usize,
    current_step: i32,
}

impl Trajectory {
    pub fn new() -> Trajectory {
        Trajectory {
            moves: Vec::new(),
            current_move: 0,
            current_step: 0,
        }
    }

    pub fn add_move(&mut self, x: f64, y: f64, steps: i32) {
        self.moves.push(Move::new(Point::new(x, y), steps));
    }

    pub fn step(&mut self, current_position: Point) -> Point {
        let current_move = &self.moves[self.current_move];
        let new_position = current_move.step(current_position, self.current_step);
        if current_move.finished(self.current_step + 1) {
            self.current_move += 1;
            self.current_step = 0;
        } else {
            self.current_step += 1;
        }
        new_position
    }

    pub fn finished(&self) -> bool {
        self.current_move >= self.moves.len()
    }
}
