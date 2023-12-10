advent_of_code::solution!(5);

use itertools::Itertools;

// Returns iterator of tuples of (usize, usize, usize)
fn table(table: &str) -> Vec<(usize, usize, usize)> {
    table
        .lines()
        .skip(1)
        .map(|s| -> (usize, usize, usize) {
            s
                .split_whitespace()
                .filter_map(|s| s.parse::<usize>().ok())
                .collect_tuple()
                .unwrap()
        }).collect_vec()
}


fn location_part_1(seed: usize, ranges: &Vec<Vec<(usize, usize, usize)>>) -> usize {
    let mut seed = seed;
    for table in ranges {
        let range = table.into_iter().find(|(_, src, len)| seed >= *src && seed <= *src + *len);
        if let Some((destination, source, _)) = range {
            seed = (destination + seed) - source;
        }
    }
    seed
}

pub fn part_one(input: &str) -> Option<usize> {
    let mut almanac = input.split("\n\n");
    let seeds = almanac
        .next()
        .unwrap()
        .split_ascii_whitespace()
        .filter_map(|s| s.parse::<usize>().ok());

    let ranges = almanac.map(table).collect_vec();

    let locations_min = seeds.map(|seed| location_part_1(seed, &ranges)).min();

    locations_min
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut almanac = input.split("\n\n");

    let seeds = almanac
        .next()
        .unwrap()
        .split_ascii_whitespace()
        .filter_map(|s| s.parse::<usize>().ok())
        .tuples::<(usize, usize)>()
        .collect_vec();

    let mut tables = almanac.map(table).collect_vec();
    tables.reverse();

    let location = (0..usize::MAX)
        .into_iter()
        .map(|possible_location| {
            let mut seed = possible_location.clone();
            for table in &tables {
                let range = table.into_iter().find(|(dest, _, len)| seed >= *dest && seed < *dest + *len);
                if let Some((destination, source, _)) = range {
                    seed = seed + source - destination;
                }
            }
            (possible_location, seed)
        })
        .find(|(_, seed)| seeds.iter().any(|(from, len)| seed >= &from && seed < &(from + len)))
        .map(|(loc, _)| loc);

    location

    // Before: 1809081164
    // After:
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(35));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(46));
    }
}
