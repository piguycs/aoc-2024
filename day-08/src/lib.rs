use std::collections::HashMap;

use glam::{ivec2, IVec2};
use itertools::Itertools;

pub struct Antennas {
    width: i32,
    height: i32,
    antennas: HashMap<char, Vec<IVec2>>,
}

impl Antennas {
    pub fn parse(input: &str) -> Self {
        let antennas: HashMap<char, Vec<IVec2>> = input
            .lines()
            .enumerate()
            .flat_map(|(y, line)| {
                line.chars().enumerate().filter_map(move |(x, c)| {
                    if c.is_ascii_alphanumeric() {
                        Some((c, ivec2(x as i32, y as i32)))
                    } else {
                        None
                    }
                })
            })
            .fold(HashMap::new(), |mut acc, (c, pos)| {
                acc.entry(c).or_default().push(pos);
                acc
            });

        let width = input.lines().next().map(|e| e.chars().count()).unwrap() as i32;
        let height = input.lines().count() as i32;

        Self {
            antennas,
            width,
            height,
        }
    }

    pub fn num_antinodes(&self) -> usize {
        self.antennas
            .values()
            .flat_map(|positions| {
                positions
                    .iter()
                    .combinations(2)
                    .flat_map(|poss| {
                        let spacing = poss[0] - poss[1];
                        [poss[0] + spacing, poss[1] - spacing]
                    })
                    .filter(|&IVec2 { x, y }| {
                        (0..self.width).contains(&x) && (0..self.height).contains(&y)
                    })
            })
            .unique()
            .count()
    }

    pub fn num_all_antinodes(&self) -> usize {
        self.antennas
            .values()
            .flat_map(|positions| {
                positions
                    .iter()
                    .combinations(2)
                    .flat_map(|pair: Vec<&IVec2>| {
                        let pos1 = pair[0];
                        let pos2 = pair[1];

                        let row_diff = pos1.y - pos2.y;
                        let col_diff = pos1.x - pos2.x;

                        let forward = (1..).map_while(move |i| {
                            let new_x = pos1.x + i * col_diff;
                            let new_y = pos1.y + i * row_diff;
                            if (0..self.width).contains(&new_x) && (0..self.height).contains(&new_y)
                            {
                                Some(IVec2::new(new_x, new_y))
                            } else {
                                None
                            }
                        });

                        let reverse = (1..).map_while(move |i| {
                            let new_x = pos2.x - i * col_diff;
                            let new_y = pos2.y - i * row_diff;
                            if (0..self.width).contains(&new_x) && (0..self.height).contains(&new_y)
                            {
                                Some(IVec2::new(new_x, new_y))
                            } else {
                                None
                            }
                        });

                        let original_positions = vec![*pos1, *pos2];

                        original_positions.into_iter().chain(forward).chain(reverse)
                    })
            })
            .filter(|IVec2 { x, y }| (0..self.width).contains(x) && (0..self.height).contains(y))
            .unique()
            .count()
    }
}
