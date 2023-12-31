#[derive(Clone, Copy)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

impl Point {
    pub fn new(x: f64, y: f64) -> Point {
        Point { x, y }
    }

    pub fn length(&self) -> f64 {
        (self.x * self.x + self.y * self.y).sqrt()
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
    target_pos: Point,
    target_scale: f64,
    speed: f64,
}

impl Move {
    fn new(target_pos: Point, target_scale: f64, speed: f64) -> Move {
        Move {
            target_pos,
            target_scale,
            speed,
        }
    }

    fn get_target(&self) -> (Point, f64) {
        (self.target_pos, self.target_scale)
    }
}

pub struct Trajectory {
    moves: Vec<Move>,
    current_move: usize,
    dt: f64,
}

impl Trajectory {
    pub fn new(dt: f64) -> Trajectory {
        Trajectory {
            moves: Vec::new(),
            current_move: 0,
            dt,
        }
    }

    pub fn add_move(&mut self, x: f64, y: f64, scale: f64, speed: f64) {
        let new_pos = Point::new(x, y);
        self.moves.push(Move::new(new_pos, scale, speed));
    }

    pub fn step(&mut self) -> (Point, f64) {
        let current_move = &self.moves[self.current_move];
        let (new_position, new_scale) = current_move.get_target();
        self.current_move += 1;
        (new_position, new_scale)
    }

    pub fn smooth(&mut self, original_position: Point, original_scale: f64) {
        let mut steps_granular: Vec<Move> = Vec::new();
        let mut last_pos = original_position;
        let mut last_scale = original_scale;
        let mut last_speed = self.moves.first().unwrap().speed * original_scale;

        let mut step_remainder = 0.0;
        for move_ in self.moves.iter() {
            let target_pos = last_pos + move_.target_pos * last_scale;
            let target_scale = last_scale * move_.target_scale;
            let new_speed = move_.speed * target_scale;
            let dist = (target_pos - last_pos).length();
            let avg_speed = (last_speed + new_speed) / 2.0;
            let steps = dist / (avg_speed * self.dt) + step_remainder;
            if steps < 1.0 {
                step_remainder = steps.fract();
                last_speed = new_speed;
                continue;
            }

            for step in 0..steps.floor() as i32 {
                let cur_speed = last_speed + (new_speed - last_speed) * (step as f64 / steps);
                let cur_avg_speed = (last_speed + cur_speed) / 2.0;
                let ratio = cur_avg_speed * step as f64 / (avg_speed * steps);
                let new_pos = last_pos + (target_pos - last_pos) * ratio;
                let new_scale = last_scale + (target_scale - last_scale) * ratio;

                steps_granular.push(Move::new(new_pos, new_scale, 1.0));
                if steps_granular.len() > 50000 {
                    panic!("Too many steps!");
                }
            }
            step_remainder = steps.fract();
            last_pos = steps_granular.last().unwrap().target_pos;
            last_scale = steps_granular.last().unwrap().target_scale;
            last_speed = new_speed;
        }

        let steps_smoothed = apply_filter(steps_granular, original_position, original_scale);

        self.moves = steps_smoothed;
    }

    pub fn finished(&self) -> bool {
        self.current_move >= self.moves.len()
    }
}

fn apply_filter(
    steps_granular: Vec<Move>,
    original_position: Point,
    original_scale: f64,
) -> Vec<Move> {
    let mut steps_smoothed: Vec<Move> = Vec::new();
    let mut last_pos = original_position;
    let mut last_scale = original_scale;
    const A: f64 = 0.95;

    for move_ in steps_granular.iter() {
        last_pos = last_pos * A + move_.target_pos * (1.0 - A);
        last_scale = last_scale * A + move_.target_scale * (1.0 - A);
        steps_smoothed.push(Move::new(last_pos, last_scale, 1.0));
    }
    steps_smoothed
}
