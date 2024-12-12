use std::fs::read_to_string;

pub fn get_input(day: usize, input_file: &str) -> String {
    let root = format!("{}/..", env!("CARGO_MANIFEST_DIR"));
    let file_name = format!("{}/day-{:02}/{}", root, day, input_file);

    read_to_string(file_name).expect("Could not open given file")
}

#[derive(Debug, Clone, Copy)]
pub enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    pub const ALL: [Self; 4] = [Self::Up, Self::Right, Self::Down, Self::Left];

    pub fn next_coords(&self, x: usize, y: usize) -> Option<(usize, usize)> {
        match self {
            Self::Up => Some((x, y.checked_sub(1)?)),
            Self::Right => Some((x + 1, y)),
            Self::Down => Some((x, y + 1)),
            Self::Left => Some((x.checked_sub(1)?, y)),
        }
    }

    pub fn all_next_coords(x: usize, y: usize) -> Vec<(usize, usize)> {
        let mut ans = vec![];

        for dir in Self::ALL {
            if let Some((newx, newy)) = dir.next_coords(x, y) {
                ans.push((newx, newy));
            }
        }

        ans
    }
}

pub struct SimpleData {
    pub input: String,
}

impl SimpleData {
    pub fn at(&self, x: usize, y: usize) -> Option<char> {
        self.input
            .lines()
            .nth(y)
            .and_then(|line| line.chars().nth(x))
    }

    pub fn width(&self) -> usize {
        self.input
            .lines()
            .nth(0)
            .expect("invalid input")
            .chars()
            .count()
    }

    pub fn height(&self) -> usize {
        self.input.lines().count()
    }
}
