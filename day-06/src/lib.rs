use glam::{ivec2, IVec2};
use pathfinding::prelude::*;

#[derive(Debug, Clone)]
pub struct Board {
    pub guard: IVec2,
    pub guard_dir: IVec2,
    pub walls: Vec<IVec2>,
    pub width: usize,
    pub height: usize,
}

impl Board {
    pub fn parse(input: &str) -> Self {
        let width = input.lines().next().map_or(0, |line| line.len());
        let height = input.lines().count();

        let (guard, guard_dir, walls) = input
            .lines()
            .enumerate()
            .flat_map(|(y, line)| line.chars().enumerate().map(move |(x, c)| (x, y, c)))
            .fold(
                (None, None, vec![]),
                |(mut guard, mut guard_dir, mut walls), (x, y, c)| {
                    let pos = ivec2(x as i32, y as i32);
                    match c {
                        '#' => walls.push(pos),
                        '^' => {
                            guard = Some(pos);
                            guard_dir = Some(IVec2::NEG_Y);
                        }
                        'v' => {
                            guard = Some(pos);
                            guard_dir = Some(IVec2::Y);
                        }
                        '<' => {
                            guard = Some(pos);
                            guard_dir = Some(IVec2::NEG_X);
                        }
                        '>' => {
                            guard = Some(pos);
                            guard_dir = Some(IVec2::X);
                        }
                        '.' => (),
                        _ => unreachable!(),
                    }

                    (guard, guard_dir, walls)
                },
            );

        let guard = guard.expect("guard must be present in the input");
        let guard_dir = guard_dir.expect("guard must be present in the input");

        Self {
            guard,
            guard_dir,
            walls,
            width,
            height,
        }
    }

    pub fn traverse(&self) -> usize {
        let path = dijkstra(
            &(self.guard, self.guard_dir, Vec::<IVec2>::new()),
            |(pos, direction, visited)| {
                let mut neighbors = vec![];
                let mut new_visited = visited.clone();

                if !new_visited.contains(pos) {
                    new_visited.push(*pos);
                }

                let forward_pos = *pos + *direction;

                if self.walls.contains(&forward_pos) {
                    let new_direction = direction.perp();
                    neighbors.push(((*pos, new_direction, new_visited), 1));
                } else {
                    neighbors.push(((forward_pos, *direction, new_visited), 1));
                }

                neighbors
            },
            |&(pos, _, _)| {
                let width = self.width as i32;
                let height = self.height as i32;

                !(0..width).contains(&pos.x) || !(0..height).contains(&pos.y)
            },
        )
        .expect("path invalid");

        path.0.last().map(|e| e.2.len()).unwrap_or(0)
    }
}
