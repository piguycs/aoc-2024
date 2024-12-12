#![feature(let_chains)]

use common::Direction;

const INPUT: &str = "input0.txt";

type Data = common::SimpleData;

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
        for (newx, newy) in common::Direction::all_next_coords(x, y) {
            area_r(curr, data, newx, newy, visited, section);
        }
    }
}

fn get_sections(data: &Data) -> Vec<Vec<(usize, usize)>> {
    let mut visited_sections = vec![];
    let mut curr_section = vec![];

    let mut sections = vec![];

    for x in 0..data.width() {
        for y in 0..data.height() {
            if !visited_sections.contains(&(x, y)) {
                let c = data.at(x, y).unwrap();
                area_r(c, data, x, y, &mut vec![], &mut curr_section);
                sections.push(curr_section.clone());
                visited_sections.append(&mut curr_section);
            }
        }
    }

    sections
}

fn part1(sections: &Vec<Vec<(usize, usize)>>) {
    let mut p1 = 0;

    for section in sections {
        let area = section.len();
        let mut total = area * 4;

        for (x, y) in section {
            for (newx, newy) in common::Direction::all_next_coords(*x, *y) {
                if section.contains(&(newx, newy)) {
                    total -= 1;
                }
            }
        }

        p1 += total * area;
    }

    println!("part 1: {p1}");
}

fn count_corners(points: Vec<(usize, usize)>) -> usize {
    for (x, y) in points {
        let up = Direction::Up.next_coords(x, y);
        let d1 = {
            if let Some((x, y)) = Direction::Up.next_coords(x, y) {
                Direction::Right.next_coords(x, y)
            } else {
                None
            }
        };
        let right = Direction::Right.next_coords(x, y);
        let d2 = {
            if let Some((x, y)) = Direction::Right.next_coords(x, y) {
                Direction::Down.next_coords(x, y)
            } else {
                None
            }
        };
        let down = Direction::Down.next_coords(x, y);
        let d3 = {
            if let Some((x, y)) = Direction::Down.next_coords(x, y) {
                Direction::Left.next_coords(x, y)
            } else {
                None
            }
        };
        let left = Direction::Left.next_coords(x, y);
        let d4 = {
            if let Some((x, y)) = Direction::Left.next_coords(x, y) {
                Direction::Up.next_coords(x, y)
            } else {
                None
            }
        };

        #[allow(clippy::match_single_binding)]
        match (up, d1, right, d2, left, d3, down, d4) {
            _ => todo!(),
        };
    }

    0
}

#[allow(unused)]
#[allow(clippy::no_effect)]
fn part2(sections: &Vec<Vec<(usize, usize)>>) {
    let mut p2 = 0;

    for section in sections {
        let area = section.len();
        let sides = count_corners(section.to_vec());
        println!("{sides}");
        todo!();

        p2 += sides * area;
    }

    println!("part 2: {p2}");
}

fn main() {
    let input = common::get_input(12, INPUT);
    let data = Data { input };

    let sections = get_sections(&data);

    part1(&sections);
    part2(&sections);
}
