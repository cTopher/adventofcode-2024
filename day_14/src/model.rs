use crate::tiles::{Position, Velocity};
use std::str::FromStr;

#[derive(Debug)]
pub struct RobotList<const W: u32, const H: u32> {
    robots: Vec<Robot<W, H>>,
}

#[derive(Debug)]
pub struct Robot<const W: u32, const H: u32> {
    pub position: Position<W, H>,
    pub velocity: Velocity,
}

impl<const W: u32, const H: u32> RobotList<W, H> {
    pub fn progress(&mut self, seconds: u32) {
        for robot in &mut self.robots {
            robot.progress(seconds);
        }
    }

    #[must_use]
    pub fn safety_factor(&self) -> u32 {
        let mut quadrants = [0; 4];
        for robot in &self.robots {
            if let Some(quadrant) = robot.position.quadrant() {
                quadrants[quadrant as usize] += 1;
            }
        }
        quadrants.iter().product()
    }
}

impl<const W: u32, const H: u32> Robot<W, H> {
    pub fn progress(&mut self, seconds: u32) {
        self.position += self.velocity * seconds;
    }
}

impl<const W: u32, const H: u32> FromStr for RobotList<W, H> {
    type Err = !;

    fn from_str(s: &str) -> Result<Self, !> {
        let robots = s
            .lines()
            .map(|line| {
                let Ok(robot) = line.parse();
                robot
            })
            .collect();
        Ok(Self { robots })
    }
}

impl<const W: u32, const H: u32> FromStr for Robot<W, H> {
    type Err = !;

    fn from_str(s: &str) -> Result<Self, !> {
        let (position, velocity) = s.split_once(' ').unwrap();
        let position = position.parse()?;
        let velocity = velocity.parse()?;
        Ok(Self { position, velocity })
    }
}
