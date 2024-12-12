#![feature(let_chains)]

use common::Direction;
use itertools::Itertools;

const INPUT: &str = "input1.txt";

type Data = common::SimpleData;

fn i2u(a: (isize, isize)) -> (usize, usize) {
    (a.0 as usize, a.1 as usize)
}

fn area_r(
    curr: char,
    data: &Data,
    x: usize,
    y: usize,
    visited: &mut Vec<(usize, usize)>,
    section: &mut Vec<(usize, usize)>,
) {
    if visited.contains(&(x, y)) {
        return;
    }
    visited.push((x, y));

    if let Some(c) = data.at(x, y)
        && c == curr
    {
        section.push((x, y));
        for (newx, newy) in Direction::all_next_coords(x, y) {
            area_r(curr, data, newx, newy, visited, section);
        }
    }
}

fn get_sections(data: &Data) -> Vec<(char, Vec<(usize, usize)>)> {
    let mut visited_sections = vec![];
    let mut curr_section = vec![];

    let mut sections = vec![];

    for x in 0..data.width() {
        for y in 0..data.height() {
            if !visited_sections.contains(&(x, y)) {
                let c = data.at(x, y).unwrap();
                area_r(c, data, x, y, &mut vec![], &mut curr_section);
                sections.push((c, curr_section.clone()));
                visited_sections.append(&mut curr_section);
            }
        }
    }

    sections
}

fn part1(sections: &Vec<(char, Vec<(usize, usize)>)>) {
    let mut p1 = 0;

    for (_, section) in sections {
        let area = section.len();
        let mut total = area * 4;

        for (x, y) in section {
            for (newx, newy) in Direction::all_next_coords(*x, *y) {
                if section.contains(&(newx, newy)) {
                    total -= 1;
                }
            }
        }

        p1 += total * area;
    }

    println!("part 1: {p1}");
}

fn part2(sections: &Vec<(char, Vec<(usize, usize)>)>) {
    const DIRECTIONS: [(isize, isize); 4] = [(0, -1), (1, 0), (0, 1), (-1, 0)];

    let mut total = 0;

    for (_, section) in sections {
        let area = section.len();

        let mut corner_count = 0;

        for &(x, y) in section {
            for ((dx1, dy1), (dx2, dy2)) in DIRECTIONS.iter().circular_tuple_windows() {
                // we are getting the coordinates of 2 neighbours and a diagonal
                // take for example the coordinates (3, 2), and deltas (0, -1), (1, 0)
                // neighbour_a = (3 + 0,     2 - 1    ) = (3, 1)
                // neighbour_b = (3 + 1,     2 + 0    ) = (4, 2)
                // diagonal    = (3 + 0 + 1, 2 - 1 + 0) = (4, 1)
                // the values (3, 1), (4, 2) and (4, 1) perfectly cover a corner
                let neighbour_a = (x as isize + dx1, y as isize + dy1);
                let neighbour_b = (x as isize + dx2, y as isize + dy2);
                let diagonal = (x as isize + dx1 + dx2, y as isize + dy1 + dy2);

                // i2u is a simple helper function to convert (isize, isize) -> (usize, usize)
                let neighbour_a = i2u(neighbour_a);
                let neighbour_b = i2u(neighbour_b);
                let diagonal = i2u(diagonal);

                // we check which of the neighbours/diagonals are inside the section
                let is_a_in = section.contains(&neighbour_a);
                let is_b_in = section.contains(&neighbour_b);
                let is_diagonal_in = section.contains(&diagonal);

                // now we are only iterating over points that are part of the section
                // if both neighbours are outside, then this means that we are at a turning
                // point. This is just a normal corner
                let is_corner = !is_a_in && !is_b_in;

                // this corner has one cell to the outside. Imagine a minesweeper tile with
                // one open spot and a 1 engraving on it. This is kind of like that.
                //
                // XXXXXX
                // XX1XXX
                // XXX
                // XXX
                let is_minesweeper_corner = is_a_in && is_b_in && !is_diagonal_in;

                if is_minesweeper_corner || is_corner {
                    corner_count += 1;
                }
            }
        }

        total += area * corner_count;
    }

    println!("part 2: {total}");
}

fn main() {
    let input = common::get_input(12, INPUT);
    let data = Data { input };

    let sections = get_sections(&data);

    part1(&sections);
    part2(&sections);
}
