use crate::tiles::{Position, Velocity};
use std::fmt;
use std::str::FromStr;

#[derive(Debug)]
pub struct RobotList<const W: usize, const H: usize> {
    robots: Vec<Robot<W, H>>,
    pub seconds_elapsed: usize,
}

#[derive(Debug)]
pub struct Robot<const W: usize, const H: usize> {
    pub position: Position<W, H>,
    pub velocity: Velocity,
}

impl<const W: usize, const H: usize> RobotList<W, H> {
    pub fn progress_until_christmas_tree(&mut self) {
        while !self.is_christmas_tree() {
            self.progress(1);
        }
    }

    fn is_christmas_tree(&self) -> bool {
        self.robot_grid().iter().any(|line| {
            line.windows(10)
                .any(|window| window.iter().all(|&has_robot| has_robot))
        })
    }

    pub fn progress(&mut self, seconds: usize) {
        for robot in &mut self.robots {
            robot.progress(seconds);
        }
        self.seconds_elapsed += seconds;
    }

    #[must_use]
    pub fn safety_factor(&self) -> usize {
        let mut quadrants = [0; 4];
        for robot in &self.robots {
            if let Some(quadrant) = robot.position.quadrant() {
                quadrants[quadrant as usize] += 1;
            }
        }
        quadrants.iter().product()
    }

    fn robot_grid(&self) -> [[bool; W]; H] {
        let mut grid = [[false; W]; H];
        for robot in &self.robots {
            grid[robot.position.y][robot.position.x] = true;
        }
        grid
    }
}

impl<const W: usize, const H: usize> fmt::Display for RobotList<W, H> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for line in self.robot_grid() {
            for c in line {
                if c {
                    write!(f, "#")?;
                } else {
                    write!(f, " ")?;
                }
            }
            writeln!(f)?;
        }
        writeln!(f)
    }
}

impl<const W: usize, const H: usize> Robot<W, H> {
    pub fn progress(&mut self, seconds: usize) {
        self.position += self.velocity * seconds;
    }
}

impl<const W: usize, const H: usize> FromStr for RobotList<W, H> {
    type Err = !;

    fn from_str(s: &str) -> Result<Self, !> {
        let robots = s
            .lines()
            .map(|line| {
                let Ok(robot) = line.parse();
                robot
            })
            .collect();
        let seconds_elapsed = 0;
        Ok(Self {
            robots,
            seconds_elapsed,
        })
    }
}

impl<const W: usize, const H: usize> FromStr for Robot<W, H> {
    type Err = !;

    fn from_str(s: &str) -> Result<Self, !> {
        let (position, velocity) = s.split_once(' ').unwrap();
        let position = position.parse()?;
        let velocity = velocity.parse()?;
        Ok(Self { position, velocity })
    }
}
