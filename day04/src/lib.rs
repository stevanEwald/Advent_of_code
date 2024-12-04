#[derive(Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
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
    const ALL: [Self; 8] = [
        Self::North,
        Self::NorthEast,
        Self::East,
        Self::SouthEast,
        Self::South,
        Self::SouthWest,
        Self::West,
        Self::NorthWest,
    ];
    #[allow(dead_code)]
    const CARDINALS: [Self; 4] =[
        Self::North,
        Self::South,
        Self::East,
        Self::West,
    ];
    const DIAGONALS: [Self; 4] = [
        Self::NorthEast,
        Self::NorthWest,
        Self::SouthWest,
        Self::SouthEast,
    ];
    fn offset(&self) -> (i32, i32) {
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
    fn oppposite(&self) -> Self {
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
}
struct WordSearchGrid {
    letters: Vec<Vec<char>>
}
impl WordSearchGrid {
    fn new(input: &str) -> Self {
        let letters = input
            .lines()
            .map(|row| row.chars().collect::<Vec<char>>())
            .collect::<Vec<Vec<char>>>();
        Self {letters}
    }
    fn get_letter<I: TryInto<usize>>(&self, y: I, x: I) -> Option<char> {
        let y = y.try_into().ok()?;
        let x = x.try_into().ok()?;
        return self.letters.get(y)?.get(x).copied()
    }
    fn position_word_count(&self, y: i32, x: i32, target_word: &str) -> i32 {
        let Some(fist_letter) = self.get_letter(y, x) else {return 0};
        let first_target_letter = target_word.chars().nth(0).unwrap();
        if fist_letter.to_ascii_lowercase() != first_target_letter.to_ascii_lowercase() {
            return 0;
        }
        let count = Direction::ALL
            .into_iter()
            .filter(|&direction| self.is_word_in_direction(y, x, target_word, direction))
            .count() as i32;
        return count
    }
    fn is_word_in_direction(&self, y: i32, x: i32, target_word: &str, direction: Direction) -> bool {
        let mut remaining_letters = target_word.chars();
        let mut target_letter = remaining_letters.next();
        let (y_offset, x_offset) = direction.offset();
        let mut new_y: i32 = y;
        let mut new_x: i32 = x;
        while let Some(target) = target_letter {
            let Some(new_letter) = self.get_letter(new_y, new_x) else {break};
            if new_letter.to_ascii_lowercase() == target.to_ascii_lowercase() {
                target_letter = remaining_letters.next();
            } else {
                break
            }
            new_y += y_offset;
            new_x += x_offset;
        }
        return target_letter.is_none()
    }
    fn total_word_count(&self, target_word: &str) -> i32 {
        let mut count = 0;
        for (y, row) in self.letters.iter().enumerate() {
            for x in 0..row.len() {
                count += self.position_word_count(y as i32, x as i32, target_word)
            }
        }
        return count;
    }

    fn is_crossed_word_center(&self, y: i32, x: i32, target_word: &str, directions: &[Direction]) -> bool {
        let Some(center_letter) = self.get_letter(y, x) else {return false};
        assert_eq!(target_word.len(), 3);
        let target_center_letter = target_word.chars().nth(1).unwrap();
        if center_letter != target_center_letter {
            return false
        }

        let mut count = 0;
        for direction in directions {
            let (offset_y, offset_x) = direction.offset();
            let new_y = y + offset_y;
            let new_x = x + offset_x;
            if self.is_word_in_direction(new_y, new_x, target_word, direction.oppposite()) {
                count += 1
            }
        }
        let needed_count = {
            let reversed_word: String = target_word.chars().rev().collect();
            if target_word == reversed_word {
                4
            } else {
                2
            }
        };
        return count >= needed_count;
    }
    fn total_crossed_word_count(&self, target_word: &str) -> i32 {
        let mut count = 0;
        for (y, row) in self.letters.iter().enumerate() {
            for x in 0..row.len() {
                if self.is_crossed_word_center(y as i32, x as i32, target_word, &Direction::DIAGONALS) {
                    count += 1
                }
            }
        }
        return count;
    }

}

pub fn part_1(input: &str) -> i32 {
    let grid = WordSearchGrid::new(input);
    let target_word = "XMAS";
    return grid.total_word_count(target_word);
}

pub fn part_2(input: &str) -> i32 {
    let grid = WordSearchGrid::new(input);
    let target_word = "MAS";
    return grid.total_crossed_word_count(target_word)
}