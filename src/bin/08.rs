use std::{
    collections::HashMap,
    iter::Cycle,
    str::Chars,
};

use itertools::{
    FoldWhile::{Continue, Done},
    Itertools
};
use rayon::iter::IntoParallelRefIterator;
use rayon::iter::ParallelIterator;

advent_of_code::solution!(8);

pub fn part_one(input: &str) -> Option<usize> {
    // Directions are characters, l = left, r = right
    let mut directions = input.lines().next().unwrap().chars().cycle();

    let map = input
        .lines()
        .skip(2)
        .map(|line| {
            let [node, left, right]: [&str; 3] = line
                .split(|c: char| !c.is_alphanumeric())
                .filter(|s| !s.is_empty())
                .collect::<Vec<&str>>()
                .try_into()
                .unwrap();
            [node, left, right]
        })
        .fold(HashMap::new(), |mut map, [node, left, right]| {
            map.insert(node, (left, right));
            map
        });

    let (_, steps) = directions
        .fold_while(("AAA", 0_usize), |(node, steps), direction| {
            let (left, right) = map.get(node).unwrap();

            let res = match node {
                "ZZZ" => Done((node, steps)),
                _ => match direction {
                    'L' => Continue((*left, steps + 1)),
                    'R' => Continue((*right, steps + 1)),
                    _ => Done((node, steps)),
                },
            };

            res
        })
        .into_inner();

    Some(steps)
}

struct Path<'a> {
    map: &'a HashMap<&'a str, (&'a str, &'a str)>,
    start_node: &'a str,
    steps: usize,
    directions: Cycle<Chars<'a>>,
}

impl<'a> Path<'a> {
    fn new(
        map: &'a HashMap<&'a str, (&'a str, &'a str)>,
        start_node: &'a str,
        directions: Chars<'a>,
    ) -> Self {
        Path {
            map,
            start_node,
            steps: 0,
            directions: directions.cycle(),
        }
    }
}

impl<'a> Iterator for Path<'a> {
    type Item = (&'a str, usize);

    fn next(&mut self) -> Option<Self::Item> {
        let direction = self.directions.next()?;

        let (left, right) = self.map.get(self.start_node).unwrap();

        match direction {
            'L' => {
                self.start_node = left;
                self.steps += 1;
                Some((self.start_node, self.steps))
            }
            'R' => {
                self.start_node = right;
                self.steps += 1;
                Some((self.start_node, self.steps))
            }
            _ => None,
        }
    }
}

fn greatest_common_divisor(a: usize, b: usize) -> usize {
    let mut a = a;
    let mut b = b;
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    a
}

fn least_common_multiple(numbers: &[usize]) -> usize {
    numbers
        .iter()
        .fold(1, |a, b| a * b / greatest_common_divisor(a, *b))
}

pub fn part_two(input: &str) -> Option<usize> {
    let directions = input.lines().next().unwrap().chars();

    let map = input
        .lines()
        .skip(2)
        .map(|line| {
            let [node, left, right]: [&str; 3] = line
                .split(|c: char| !c.is_alphanumeric())
                .filter(|s| !s.is_empty())
                .collect::<Vec<&str>>()
                .try_into()
                .unwrap();
            [node, left, right]
        })
        .fold(HashMap::new(), |mut map, [node, left, right]| {
            map.insert(node, (left, right));
            map
        });

    // This time we start simulatenously at all nodes ending with A, then navigate simulatenously according to the directions
    // When all nodes end with Z, we are done

    let start_nodes = map
        .keys()
        .filter(|node| node.ends_with("A"))
        .map(|node| *node)
        .collect_vec();

    // Find number of steps for each start node in parallel
    let steps: Vec<usize> = start_nodes
        .par_iter()
        .map(|start_node| {
            let path = Path::new(&map, start_node, directions.clone());
            path.filter(|(node, ..)| node.ends_with("Z"))
                .next()
                .unwrap()
                .1
        })
        .collect();

    // Find the minimum common multiple of all steps
    let steps = least_common_multiple(&steps);

    Some(steps)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(6));
    }
}
