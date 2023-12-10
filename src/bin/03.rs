use std::ops::RangeInclusive;

use itertools::Itertools;

advent_of_code::solution!(3);

#[derive(Debug)]
struct Part {
    number: usize,
    lines: RangeInclusive<usize>,
    cols: RangeInclusive<usize>,
}

struct Gear {
    line: usize,
    position: usize,
}

fn is_symbol(c: char) -> bool {
    c != '.' && !c.is_digit(10)
}

fn numbers(schematic: &Vec<Vec<char>>, height: usize, width: usize) -> Vec<Part> {
    // Scan through looking for parts
    let numbers = schematic
        .iter()
        .enumerate()
        .map(|(index, line)| {
            let chars = line.iter().enumerate().collect::<Vec<(usize, &char)>>();

            let nums = chars
                .split(|(_, c)| !c.is_digit(10))
                .filter(|part| part.len() > 0)
                .map(|part| {
                    let num = part.iter().map(|(_, c)| c).join("");
                    Part {
                        number: num.parse::<usize>().unwrap(),
                        lines: index.saturating_sub(1)..=(height - 1).min(index + 1),
                        cols: part[0].0.saturating_sub(1)
                            ..=(width - 1).min(part[part.len() - 1].0 + 1),
                    }
                })
                .collect::<Vec<Part>>();

            nums
        })
        .flatten()
        .collect::<Vec<Part>>();

    // Now we have a list of potential parts, for each part check if it is adjacent to a symbol (non-digit and non-period)
    // Then sum the numbers of the adjacent parts

    numbers
}

fn gears(schematic: &Vec<Vec<char>>) -> Vec<Gear> {
    let gears = schematic
        .iter()
        .enumerate()
        .map(|(index, line)| {
            line.iter()
                .enumerate()
                .filter(|(_, c)| **c == '*')
                .map(move |(i, _)| Gear {
                    line: index,
                    position: i,
                })
        })
        .flatten()
        .collect_vec();
    gears
}

pub fn part_one(input: &str) -> Option<u32> {
    let schematic = input
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();
    let width = schematic[0].len();
    let height = schematic.len();

    let numbers = numbers(&schematic, height, width);

    let sum = numbers
        .iter()
        .filter(|Part { lines, cols, .. }| {
            schematic
                .iter()
                .enumerate()
                .filter(|(index, _)| lines.contains(index))
                .map(|(_, line)| {
                    line.iter()
                        .enumerate()
                        .filter(|(index, _)| cols.contains(index))
                        .filter(|(_, c)| is_symbol(**c))
                        .count()
                })
                .sum::<usize>()
                > 0
        })
        .map(|Part { number, .. }| number)
        .sum::<usize>() as u32;

    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let schematic = input
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();
    let width = schematic[0].len();
    let height = schematic.len();

    let numbers = numbers(&schematic, height, width);
    let potential_gears = gears(&schematic);

    // Now we need to find the gears that have two or more adjacent parts
    let sum = potential_gears
        .iter()
        .map(|Gear { line, position }| {
            numbers
                .iter()
                .filter(|Part { lines, cols, .. }| lines.contains(line) && cols.contains(position))
        })
        .filter(|g| g.clone().count() > 1)
        // Multiply the numbers of the parts
        .map(|g| g.map(|Part { number, .. }| number).product::<usize>())
        .sum::<usize>() as u32;

    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4361));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(467835));
    }
}
