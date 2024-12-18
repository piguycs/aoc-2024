use glam::IVec2;
use itertools::Itertools;
use pathfinding::prelude::*;

#[derive(Debug, Clone)]
pub struct Board {
    pub corruption: Vec<IVec2>,
    pub start: IVec2,
    pub end: IVec2,
    pub max: u32,
}

impl Board {
    pub fn parse(input: &str, size: u32, steps: u32) -> Board {
        let corruption = input
            .lines()
            .take(steps as usize)
            .map(|e| e.split(",").collect_vec())
            .map(|e| {
                let x: i32 = e[0].parse().expect("non digit detected");
                let y: i32 = e[1].parse().expect("non digit detected");

                IVec2::new(x, y)
            })
            .collect_vec();

        Board {
            corruption,
            start: IVec2::new(0, 0),
            end: IVec2::new(size as i32, size as i32),
            max: size,
        }
    }

    pub fn in_range(&self, pos: IVec2) -> bool {
        let max = self.max as i32;
        (0..=max).contains(&pos.x) && (0..=max).contains(&pos.y)
    }

    pub fn successor(&self, pos: IVec2) -> Vec<(IVec2, u32)> {
        let neighbours = [IVec2::X, IVec2::NEG_X, IVec2::Y, IVec2::NEG_Y]
            .iter()
            .filter_map(|e| {
                let neighbour = pos + e;

                if self.in_range(neighbour) {
                    Some((neighbour, 1))
                } else {
                    None
                }
            })
            .filter(|(e, _)| !self.corruption.contains(e))
            .collect_vec();

        neighbours
    }

    pub fn traverse(&self) -> Option<u32> {
        let path = dijkstra(&self.start, |&e| self.successor(e), |&e| e == self.end)?;
        Some(path.1)
    }
}
