use glam::UVec2;
use itertools::Itertools;

#[derive(Debug, Clone)]
pub struct Board {
    pub corruption: Vec<UVec2>,
    pub start: UVec2,
    pub end: UVec2,
}

pub fn parse(input: &str, size: u32, steps: u32) -> Board {
    let corruption = input
        .lines()
        .take(steps as usize)
        .map(|e| e.split(",").collect_vec())
        .map(|e| {
            let x: u32 = e[0].parse().expect("non digit detected");
            let y: u32 = e[1].parse().expect("non digit detected");

            UVec2::new(x, y)
        })
        .collect_vec();

    Board {
        corruption,
        start: UVec2::new(0, 0),
        end: UVec2::new(size, size),
    }
}
