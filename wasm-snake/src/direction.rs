#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn opposite(self, other: Direction) -> bool {
        self == Direction::Up && other == Direction::Down
            || self == Direction::Right && other == Direction::Left
            || self == Direction::Down && other == Direction::Up
            || self == Direction::Left && other == Direction::Right
    }
}
