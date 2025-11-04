#[derive(PartialEq, Eq, Debug)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

pub struct Robot(i32, i32, Direction);

impl Robot {
    pub fn new(x: i32, y: i32, d: Direction) -> Self {
        Robot(x, y, d)
    }

    #[must_use]
    pub fn turn_right(self) -> Self {
        let mut new_direction = self.2;
        match new_direction {
            Direction::North => new_direction = Direction::East,
            Direction::South => new_direction = Direction::West,
            Direction::West => new_direction = Direction::North,
            Direction::East => new_direction = Direction::South,
        }
        Self::new(self.0, self.1, new_direction)
    }

    #[must_use]
    pub fn turn_left(self) -> Self {
        let mut new_direction = self.2;
        match new_direction {
            Direction::North => new_direction = Direction::West,
            Direction::South => new_direction = Direction::East,
            Direction::West => new_direction = Direction::South,
            Direction::East => new_direction = Direction::North,
        }
        Self::new(self.0, self.1, new_direction)
    }

    #[must_use]
    pub fn advance(self) -> Self {
        let mut robot = Self::new(self.0, self.1, self.2);
        match robot.2 {
            Direction::East => robot.0 = robot.0 + 1,
            Direction::West => robot.0 = robot.0 - 1,
            Direction::North => robot.1 = robot.1 + 1,
            Direction::South => robot.1 = robot.1 - 1,
        };
        robot
    }

    #[must_use]
    pub fn instructions(self, instructions: &str) -> Self {
        let mut new_robot = Robot::new(self.0, self.1, self.2);
        for c in instructions.chars() {
            match c.to_ascii_uppercase() {
                'A' => new_robot = new_robot.advance(),
                'R' => new_robot = new_robot.turn_right(),
                'L' => new_robot = new_robot.turn_left(), 
                _   => (),
            }
        }
    new_robot

        
    }

    pub fn position(&self) -> (i32, i32) {
        (self.0, self.1)
    }

    pub fn direction(&self) -> &Direction {
        &self.2
    }
}
