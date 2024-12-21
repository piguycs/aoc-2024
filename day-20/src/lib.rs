use glam::IVec2;
use pathfinding::prelude::*;

#[derive(Debug, Default)]
pub struct Board {
    pub walls: Vec<IVec2>,
    pub start: IVec2,
    pub end: IVec2,

    pub width: i32,
    pub height: i32,
}

impl Board {
    pub fn parse(input: &str) -> Self {
        let (walls, start, end) = input
            .lines()
            .enumerate()
            .flat_map(|(y, line)| {
                line.chars()
                    .enumerate()
                    .map(move |(x, c)| (IVec2::new(x as i32, y as i32), c))
            })
            .fold(
                (vec![], IVec2::ZERO, IVec2::ZERO),
                |(mut walls, mut start, mut end), (pos, c)| {
                    match c {
                        '#' => walls.push(pos),
                        'S' => start = pos,
                        'E' => end = pos,
                        _ => {}
                    }

                    (walls, start, end)
                },
            );

        let width = walls
            .iter()
            .map(|pos| pos.x)
            .max()
            .expect("malformed input");

        let height = walls
            .iter()
            .map(|pos| pos.y)
            .max()
            .expect("malformed input");

        Board {
            walls,
            start,
            end,
            width,
            height,
        }
    }

    pub fn successor(&self, pos: IVec2, walls: &[IVec2]) -> Vec<(IVec2, usize)> {
        [IVec2::NEG_X, IVec2::X, IVec2::NEG_Y, IVec2::Y]
            .iter()
            .filter_map(|e| {
                let next_block = pos + e;

                if !walls.contains(&next_block) {
                    Some((next_block, 1))
                } else {
                    None
                }
            })
            .collect()
    }

    pub fn traverse(&self) -> Vec<usize> {
        let (_path, initial_score) = dijkstra(
            &self.start,
            |&e| self.successor(e, &self.walls),
            |&e| e == self.end,
        )
        .expect("no valid path found");

        println!("{initial_score}");

        todo!()
    }
}
