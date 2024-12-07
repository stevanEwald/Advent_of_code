#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum Direction {
    North,
    NorthEast,
    East,
    SouthEast,
    South,
    SouthWest,
    West,
    NorthWest,
}
impl Direction {
    pub const ALL: [Self; 8] = [
        Self::North,
        Self::NorthEast,
        Self::East,
        Self::SouthEast,
        Self::South,
        Self::SouthWest,
        Self::West,
        Self::NorthWest,
    ];

    pub const CARDINALS: [Self; 4] =[
        Self::North,
        Self::South,
        Self::East,
        Self::West,
    ];

    pub const DIAGONALS: [Self; 4] = [
        Self::NorthEast,
        Self::NorthWest,
        Self::SouthWest,
        Self::SouthEast,
    ];

    pub fn offset(&self) -> (i32, i32) {
        match self {
            Self::North => (-1, 0),
            Self::NorthEast => (-1, 1),
            Self::East => (0, 1),
            Self::SouthEast => (1, 1),
            Self::South => (1, 0),
            Self::SouthWest => (1, -1),
            Self::West => (0, -1),
            Self::NorthWest => (-1, -1),
        }
    }

    pub fn oppposite(&self) -> Self {
        match self {
            Self::North => Self::South,
            Self::NorthEast => Self::SouthWest,
            Self::East => Self::West,
            Self::SouthEast => Self::NorthWest,
            Self::South => Self::North,
            Self::SouthWest => Self::NorthEast,
            Self::West => Self::East,
            Self::NorthWest => Self::SouthEast,
        }
    }

    pub fn turn_right_90(&self) -> Self {
        match self {
            Self::North => Self::East,
            Self::NorthEast => Self::SouthWest,
            Self::East => Self::South,
            Self::SouthEast => Self::SouthWest,
            Self::South => Self::West,
            Self::SouthWest => Self::NorthWest,
            Self::West => Self::North,
            Self::NorthWest => Self::NorthEast,
        }
    }

    pub fn turn_left_90(&self) -> Self {
        match self {
            Self::North => Self::West,
            Self::NorthEast => Self::NorthWest,
            Self::East => Self::North,
            Self::SouthEast => Self::NorthEast,
            Self::South => Self::East,
            Self::SouthWest => Self::SouthEast,
            Self::West => Self::South,
            Self::NorthWest => Self::SouthWest,
        }
    }

    pub fn turn_right_45(&self) -> Self {
        match self {
            Self::North => Self::NorthEast,
            Self::NorthEast => Self::East,
            Self::East => Self::SouthEast,
            Self::SouthEast => Self::South,
            Self::South => Self::SouthWest,
            Self::SouthWest => Self::West,
            Self::West => Self::NorthWest,
            Self::NorthWest => Self::North,
        }
    }

    pub fn turn_left_45(&self) -> Self {
        match self {
            Self::North => Self::NorthWest,
            Self::NorthEast => Self::North,
            Self::East => Self::NorthEast,
            Self::SouthEast => Self::East,
            Self::South => Self::SouthEast,
            Self::SouthWest => Self::South,
            Self::West => Self::SouthWest,
            Self::NorthWest => Self::West,
        }
    }

}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum CardinalDirection {
    North,
    East,
    South,
    West,
}
impl CardinalDirection {
    pub const ALL: [Self; 4] = [
        Self::North,
        Self::East,
        Self::South,
        Self::West,
    ];

    pub fn offset(&self) -> (i32, i32) {
        match self {
            Self::North => (-1, 0),
            Self::East => (0, 1),
            Self::South => (1, 0),
            Self::West => (0, -1),
        }
    }

    pub fn oppposite(&self) -> Self {
        match self {
            Self::North => Self::South,
            Self::East => Self::West,
            Self::South => Self::North,
            Self::West => Self::East,
        }
    }

    pub fn turn_right(&self) -> Self {
        match self {
            Self::North => Self::East,
            Self::East => Self::South,
            Self::South => Self::West,
            Self::West => Self::North,
        }
    }

    pub fn turn_left(&self) -> Self {
        match self {
            Self::North => Self::West,
            Self::East => Self::North,
            Self::South => Self::East,
            Self::West => Self::South,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum DiagnalDirection {
    NorthEast,
    SouthEast,
    SouthWest,
    NorthWest,
}
impl DiagnalDirection {
    pub const ALL: [Self; 4] = [
        Self::NorthEast,
        Self::SouthEast,
        Self::SouthWest,
        Self::NorthWest,
    ];

    pub fn offset(&self) -> (i32, i32) {
        match self {
            Self::NorthEast => (-1, 1),
            Self::SouthEast => (1, 1),
            Self::SouthWest => (1, -1),
            Self::NorthWest => (-1, -1),
        }
    }

    pub fn oppposite(&self) -> Self {
        match self {
            Self::NorthEast => Self::SouthWest,
            Self::SouthEast => Self::NorthWest,
            Self::SouthWest => Self::NorthEast,
            Self::NorthWest => Self::SouthEast,
        }
    }

    pub fn turn_right(&self) -> Self {
        match self {
            Self::NorthEast => Self::SouthWest,
            Self::SouthEast => Self::SouthWest,
            Self::SouthWest => Self::NorthWest,
            Self::NorthWest => Self::NorthEast,
        }
    }

    pub fn turn_left(&self) -> Self {
        match self {
            Self::NorthEast => Self::NorthWest,
            Self::SouthEast => Self::NorthEast,
            Self::SouthWest => Self::SouthEast,
            Self::NorthWest => Self::SouthWest,
        }
    }

}