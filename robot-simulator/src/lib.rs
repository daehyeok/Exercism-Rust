// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

#[derive(PartialEq, Debug, Copy, Clone)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

#[derive(Copy, Clone)]
pub struct Robot {
    x: i32,
    y: i32,
    d: Direction,
}

impl Robot {
    pub fn new(x: i32, y: i32, d: Direction) -> Self {
        Robot { x: x, y: y, d: d }
    }

    pub fn turn_right(mut self) -> Self {
        self.d = match self.d {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        };
        self
    }

    pub fn turn_left(mut self) -> Self {
        self.d = match self.d {
            Direction::North => Direction::West,
            Direction::East => Direction::North,
            Direction::South => Direction::East,
            Direction::West => Direction::South,
        };
        self
    }

    pub fn advance(mut self) -> Self {
        let delta = match self.d {
            Direction::North => (0, 1),
            Direction::East => (1, 0),
            Direction::South => (0, -1),
            Direction::West => (-1, 0),
        };
        self.x = delta.0;
        self.y = delta.1;
        self
    }

    pub fn instructions(self, instructions: &str) -> Self {
        instructions.chars().for_each(|ch| {
            match ch {
                'R' => {
                    self.turn_right();
                }
                'L' => {
                    self.turn_left();
                }
                'A' => {
                    self.advance();
                }
                _ => {}
            };
        });
        self
    }

    pub fn position(&self) -> (i32, i32) {
        (self.x, self.y)
    }

    pub fn direction(&self) -> &Direction {
        &self.d
    }
}
