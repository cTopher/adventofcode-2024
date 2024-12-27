use grid::{Grid, Position};

pub struct Warehouse {
    robot: Position,
    map: Grid<Tile>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    Empty,
    Wall,
    Box,
}
