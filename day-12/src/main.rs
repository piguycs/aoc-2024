#![feature(let_chains)]
#![allow(unused)]

const INPUT: &str = "input1.txt";

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

fn main() {
    let input = common::get_input(12, INPUT);
    let data = Data { input };

    let sections = get_sections(&data);

    part1(&sections);
}
